// Copyright © 2017 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

// Anagram generator

use std::env::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::*;

extern crate multiset;

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

type Hist = multiset::HashMultiSet<char>;

struct Entry {
    word: String,
    whist: Hist
}

type Dict = Vec<Entry>;
type WordList = Vec<String>;

fn hist_string(w: &str) -> Option<Hist> {
    let mut h = Hist::new();
    for c in w.chars() {
        if !c.is_alphabetic() {
            return None
        };
        let ci =
            if c.is_uppercase() {
                let mut clc = c.to_lowercase();
                let cl = clc.next().expect("character has no lowercase codes");
                if clc.count() != 0 {
                    return None
                };
                cl
            } else {
                c
            };
        h.insert(ci);
    };
    Some(h)
}

fn load_dict() -> Dict {
    let mut dict: Dict = Vec::new();
    let extra = [
	"s",
	"ed",
	"er",
	"ing",
	"ly",
        "i",
        "a"
    ];
    let f = open_dict();
    let r = BufReader::new(&f);
    for l in r.lines() {
        let w = l.expect("cannot read word from dictionary");
	if w.len() <= 1 {
	    continue;
        };
        if let Some(h) = hist_string(&w) {
            let e = Entry {
                whist: h,
                word: w
            };
            dict.push(e);
        };
    };
    for x in extra.iter() {
        if let Some(h) = hist_string(x) {
            let e = Entry {
	        word: String::from(*x),
	        whist: h
	    };
            dict.push(e);
        } else {
            panic!("mysterious extra entry");
        };
    };
    let len_order = |a: &Entry, b: &Entry| {
	a.word.len().cmp(&b.word.len())
    };
    dict.sort_by(len_order);
    dict
}

fn anagram(dict: &Dict, remaining: &mut Hist,
           start: usize, sofar: &mut WordList) {
    if remaining.total_elements() == 0 {
        print!("{}", sofar[0]);
        for i in 1..sofar.len() {
            print!(" {}", sofar[i]);
        }
        println!();
        return;
    };
    for i in start..dict.len() {
        if *remaining >= dict[i].whist {
            *remaining -= dict[i].whist.clone();
            sofar.push(dict[i].word.clone());
            anagram(dict, remaining, i, sofar);
            let _ = sofar.pop();
            *remaining += dict[i].whist.clone();
        }
    }
}

fn main() {
    let target = args().nth(1).expect("usage: anagram <target>");
    let mut th = match hist_string(&target) {
        Some(h) => h,
        None => panic!("invalid target")
    };
    let dict = load_dict();
    let mut sofar = Vec::new();
    anagram(&dict, &mut th, 0, &mut sofar);
}
