// Copyright © 2017 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

//! Anagram generator.

mod histogram;
mod dictionary;
    
use histogram::*;
use dictionary::*;

use std::env::args;
use std::process::exit;

extern crate multiset;


/// Construct anagrams from the suffix of `dict` starting at
/// `start`. The `remaining` characters are available for
/// histogramming. Anagram words are pushed onto `sofar` as
/// they are constructed. If construction is complete,
/// the result is displayed.
fn anagram<'a>(dict: &'a Vec<Entry>, remaining: &Histogram,
           start: usize, sofar: &mut Vec<&'a str>) {
    // Base case: An anagram has been completely
    // constructed. Display it and return.
    if remaining.total_elements() == 0 {
        print!("{}", sofar[0]);
        for i in 1..sofar.len() {
            print!(" {}", sofar[i]);
        }
        println!();
        return;
    };
    // Recursive case: For each entry from start to end of
    // the dictionary, if that entry can be used to extend
    // the anagram do so, then recursively try all
    // completions from that point. Use do-undo on `sofar`
    // to avoid cloning.
    for i in start..dict.len() {
        if dict[i].whist.is_submultiset(remaining)  {
            let now_remaining = remaining - &dict[i].whist;
            sofar.push(&dict[i].word);
            anagram(dict, &now_remaining, i, sofar);
            let _ = sofar.pop();
        }
    }
}

/// Run the program.
fn main() {
    let mut target_hist = Histogram::new();
    for word in args().skip(1) {
        target_hist += match word_histogram(&word) {
            Some(hist) => hist,
            None => {
                eprintln!("target words contain invalid characters");
                exit(1);
            },
        }
    };
    let dict = load_dictionary(&target_hist);
    let mut sofar = Vec::new();
    anagram(&dict, &mut target_hist, 0, &mut sofar);
}
