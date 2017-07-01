// Copyright © 2017 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

// Anagram generator

use std::env::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod hist;
mod ascii;

use ::hist::*;

const DICT: &'static str = "dict.txt";

struct Entry {
    word: String,
    whist: Hist
}

type Dict = Vec<Entry>;
type WordList = Vec<String>;

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
    let f = File::open(DICT).expect("cannot open dictionary");
    let r = BufReader::new(&f);
    for l in r.lines() {
        let w = l.expect("cannot read word from dictionary");
	if w.len() <= 1 {
	    continue;
        }
        let e = Entry {
            whist: Hist::hist_string(&w),
            word: w
        };
        dict.push(e);
    };
    for x in extra.iter() {
        let e = Entry {
	    word: String::from(*x),
	    whist: Hist::hist_string(x)
	};
        dict.push(e);
    }
    let len_order = |a: &Entry, b: &Entry| {
	a.word.len().cmp(&b.word.len())
    };
    dict.sort_by(len_order);
    dict
}

fn anagram(dict: &Dict, remaining: &mut Hist,
           start: usize, sofar: &mut WordList) {
    if remaining.hist_empty() {
        print!("{}", sofar[0]);
        for i in 1..sofar.len() {
            print!(" {}", sofar[i]);
        }
        println!();
        return;
    };
    for i in start..dict.len() {
        if remaining.hist_subset(&dict[i].whist) {
            remaining.hist_subtract(&dict[i].whist);
            sofar.push(dict[i].word.clone());
            anagram(dict, remaining, i, sofar);
            let _ = sofar.pop();
            remaining.hist_add(&dict[i].whist);
        }
    }
}

fn main() {
    let target = args().nth(1).expect("usage: anagram <target>");
    let mut th = Hist::hist_string(&target);
    let dict = load_dict();
    let mut sofar = Vec::new();
    anagram(&dict, &mut th, 0, &mut sofar);
}
