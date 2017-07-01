// Copyright © 2017 Bart Massey

// Histograms of alphabetic ASCII characters.

use ascii::*;

struct Hist {
    hist: usize[26]
}

impl Hist {

    pub fn hist_string(w: &str) -> Hist {
	let mut h = [26; 0usize];
	for c in w {
	    if !is_alpha(c)  {
		continue;
            };
            let i = (ord(to_lower(c)) - ord('a')) as usize;
	    h[i] += 1;
	};
	h
    }


    pub fn hist_empty(&self) -> bool {
	for b in self {
	    if b > 0 {
		return false;
            };
        };
	true
    }

    pub fn hist_subset(&self, ss: &Hist ) -> bool {
	for i in 0..self.len() {
	    if self[i] < ss[i] {
		return false;
            };
        };
	true
    }

    pub fn hist_subtract(&mut self, &hist t) {
	for i in 0..self.len() {
	    self[i] -= t[i];
        };
    }

    pub fn hist_add(&mut self, &hist t) {
	for i in 0..self.len() {
	    self[i] += t[i];
        };
    }

}

