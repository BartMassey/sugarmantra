// Copyright © 2017 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

//! Anagram generator.

mod histogram;
mod dictionary;
    
use histogram::*;
use dictionary::*;

use std::env::*;

extern crate multiset;


/// Construct anagrams from the suffix of `dict` starting at
/// `start`. The `remaining` characters are available for
/// histogramming. Anagrams are pushed onto `sofar` as they
/// are constructed. If construction is complete, display
/// the result.
fn anagram(dict: &Vec<Entry>, remaining: &mut Histogram,
           start: usize, sofar: &mut Vec<String>) {
    // Base case: All possible anagrams have been
    // constructed. Display and return them.
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
    // completions from that point. Use do-undo to ensure
    // that remaining is still valid.
    for i in start..dict.len() {
        if dict[i].whist.is_submultiset(remaining)  {
            // XXX The dubious cloning here is due to
            // unfortunate decisions in this code and also
            // in the `MultiSet` crate.  Both should be
            // fixed.
            *remaining -= dict[i].whist.clone();
            sofar.push(dict[i].word.clone());
            anagram(dict, remaining, i, sofar);
            let _ = sofar.pop();
            *remaining += dict[i].whist.clone();
        }
    }
}

/// Run the program.
fn main() {
    let target = args().nth(1).expect("usage: anagram <target>");
    let mut th = match word_histogram(&target) {
        Some(h) => h,
        None => panic!("invalid target")
    };
    let dict = load_dictionary(&th);
    let mut sofar = Vec::new();
    anagram(&dict, &mut th, 0, &mut sofar);
}
