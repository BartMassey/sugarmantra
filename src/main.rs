// Copyright © 2017 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

//! Anagram generator.

/// Stems are extra fragments used to allow more anagrams
/// to be made by the user by giving the opportunity to glue
/// them to words. A stemmed dictionary would be better than
/// this plan; also, this is very English-specific.
const STEMS:&[&str] = &[
    "s",
    "ed",
    "er",
    "ing",
    "ly",
    "i",
    "a",
];

use std::env::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::*;

extern crate multiset;

/// Read the word list from some dictionary.
fn open_dict() -> File {
    for file in ["scowl.txt", "eowl.txt", "words"].iter() {
        for dir in ["/usr/share/dict", "/usr/local/share/dict"].iter() {
            let mut path = PathBuf::from(dir);
            path.push(file);
            if let Ok(f) = File::open(path) {
                return f;
            };
        };
    };
    panic!("could not find a dictionary");
}

/// A histogram is represented as a multiset or "bag" of
/// characters.
type Histogram = multiset::HashMultiSet<char>;

/// A dictionary entry maps a word to its corresponding
/// histogram.
struct Entry {
    word: String,
    whist: Histogram
}

/// Calculate the histogram of the given word.
fn word_histogram(word: &str) -> Option<Histogram> {
    let mut histogram = Histogram::new();
    for c in word.chars() {
        if !c.is_alphabetic() {
            return None
        };
        // It may be the case that a single uppercase
        // char translates to several lowercase chars.
        for c_lowercase in c.to_lowercase() {
            histogram.insert(c_lowercase);
        }
    };
    Some(histogram)
}

/// Parse the words in the dictionary into histograms.  Do
/// some pruning along the way for efficiency.  Augment the
/// dictionary with common stems that can be used to help
/// construct words.
fn load_dictionary(target: &Histogram) -> Vec<Entry> {
    // Load in the dictionary.
    let mut dict: Vec<Entry> = Vec::new();
    let f = open_dict();
    let r = BufReader::new(&f);
    for line in r.lines() {
        let word = line.expect("cannot read word from dictionary");
	if word.len() <= 1 {
	    continue;
        };
        if let Some(whist) = word_histogram(&word) {
            if whist.is_submultiset(target) {
                let e = Entry {
                    whist,
                    word,
                };
                dict.push(e);
            };
        };
    };
    // Add the stems.
    for stem in STEMS.iter() {
        if let Some(whist) = word_histogram(stem) {
            let e = Entry {
	        word: String::from(*stem),
	        whist,
	    };
            dict.push(e);
        } else {
            panic!("mysterious extra entry");
        };
    };
    // Sort in order of increasing length.
    let len_order = |a: &Entry, b: &Entry| {
	a.word.len().cmp(&b.word.len())
    };
    dict.sort_by(len_order);
    dict
}

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
