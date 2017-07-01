// Copyright © 2017 Bart Massey

// Histograms of alphabetic ASCII characters.

use ::ascii::*;

pub struct Hist {
    pub hist: Box<[usize; 26]>
}

impl Hist {

    pub fn hist_string(w: &str) -> Hist {
	let mut h = [0; 26];
	for c in w.chars() {
	    if !is_alpha(c)  {
		continue;
            };
            let i = (ord(to_lower(c)) - ord('a')) as usize;
	    h[i] += 1;
	};
	Hist{hist: Box::new(h)}
    }


    pub fn hist_empty(&self) -> bool {
	for b in self.hist.iter() {
	    if *b > 0 {
		return false;
            };
        };
	true
    }

    pub fn hist_subset(&self, ss: &Hist) -> bool {
	for i in 0..self.hist.len() {
	    if self.hist[i] < ss.hist[i] {
		return false;
            };
        };
	true
    }

    pub fn hist_subtract(&mut self, t: &Hist) {
	for i in 0..self.hist.len() {
	    self.hist[i] -= t.hist[i];
        };
    }

    pub fn hist_add(&mut self, t: &Hist) {
	for i in 0..self.hist.len() {
	    self.hist[i] += t.hist[i];
        };
    }

}

