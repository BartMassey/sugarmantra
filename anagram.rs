// Copyright © 2017 Bart Massey

// Anagram generator

use std::env::args;

use hist;

const DICT: &str = "dict.txt";

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
	"ing",
	"ly"
    ];
    let f = open(DICT).expect("cannot open dictionary");
    for l in f.lines() {
        let w = l.expect("cannot read word from dictionary");
	if len(w) == 0 {
	    continue;
        }
        let e = Entry {
            whist: Hist::hist_string(w),
            word: w
        };
        dict.push(e);
    };
    for x in extra {
        let e = Entry {
	    word: extra[i],
	    whist: Hist::hist_string(extra[i])
	};
        dict.push(e);
    }
    let len_order = |a, b| {
	a.word.len().cmp(b.word.len())
    };
    dict.sort_by(len_order);
    dict
}

fn anagram(dict: Dict, remaining: mut Hist) {
    fn do_anagram(start: usize, sofar: mut WordList) {
	if remaining.hist_empty() {
	    print!("%s", sofar[0]);
            for i in 1..sofar.len() {
		print!(" %s", sofar[i]);
	    }
            println!();
	    return;
	};
	for i in start..dict.len() {
	    if remaining.hist_subset(&dict[i].whist) {
		remaining.hist_subtract(&dict[i].whist);
                sofar.push(dict[i].word);
		do_anagram(i, sofar);
                let _ = sofar.pop();
		remaining.hist_add(&dict[i].whist);
	    }
	}
    };
    do_anagram(0, Vec::new());
}

fn main() {
    let target = args.nth(1);
    let mut th = Hist::hist_string(target);
    let dict = load_dict();
    anagram(dict, &mut th);
}
