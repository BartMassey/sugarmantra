// Copyright © 2017 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.



#![allow(clippy::uninlined_format_args)]

//! Anagram generator.

mod dictionary;
mod histogram;

use dictionary::*;
use histogram::*;

use std::path::PathBuf;
use std::process::exit;

use clap::Parser;

// Stems are extra fragments used to allow more anagrams to
// be made by the user by giving the opportunity to glue
// them to words. A stemmed dictionary would be better than
// this plan; also, the default stems are very
// English-specific.

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// Find anagrams of words presented as arguments.
struct Args {
    #[arg(short, long)]
    /// Path to dictionary. [default: some system dictionary.]
    dict: Option<PathBuf>,
    #[arg(short, long)]
    /// Limit on generated word length in characters.
    limit: Option<usize>,
    #[arg(short, long, default_value="s,ed,er,ing,ly,i,a")]
    /// Stems treated as words, comma-separated with no spaces.
    stems: String,
    /// Words to anagram.
    #[arg(required(true))]
    words: Vec<String>,
}

extern crate multiset;

/// Construct anagrams from the suffix of `dict` starting at
/// `start`. The `remaining` characters are available for
/// histogramming. Anagram words are pushed onto `sofar` as
/// they are constructed. If construction is complete,
/// the result is displayed.
fn anagram<'a>(dict: &'a [Entry], remaining: &Histogram, start: usize, sofar: &mut Vec<&'a str>) {
    // Base case: An anagram has been completely
    // constructed. Display it and return.
    if remaining.total_elements() == 0 {
        print!("{}", sofar[0]);
        for w in &sofar[1..] {
            print!(" {}", *w);
        }
        println!();
        return;
    }
    // Recursive case: For each entry from start to end of
    // the dictionary, if that entry can be used to extend
    // the anagram do so, then recursively try all
    // completions from that point. Use do-undo on `sofar`
    // to avoid cloning.
    for i in start..dict.len() {
        if dict[i].whist.is_submultiset(remaining) {
            let now_remaining = remaining - &dict[i].whist;
            sofar.push(&dict[i].word);
            anagram(dict, &now_remaining, i, sofar);
            let _ = sofar.pop();
        }
    }
}

/// Run the program.
fn main() {
    let args = Args::parse();

    let mut target_hist = Histogram::new();
    for word in args.words {
        target_hist += match word_histogram(&word) {
            Some(hist) => hist,
            None => {
                eprintln!("target words contain invalid characters");
                exit(1);
            }
        }
    }
    let dict = load_dictionary(args.dict, args.limit, args.stems, &target_hist).unwrap_or_else(|e| {
        eprintln!("{}", e);
        exit(1);
    });
    let mut sofar = Vec::new();
    anagram(&dict, &target_hist, 0, &mut sofar);
}
