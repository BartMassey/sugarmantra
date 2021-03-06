// Copyright © 2017 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

//! Histogram support for sugarmantra.

use multiset::HashMultiSet;

/// A histogram is represented as a multiset or "bag" of
/// characters.
pub type Histogram = HashMultiSet<char>;

/// Calculate the histogram of the given word.
pub fn word_histogram(word: &str) -> Option<Histogram> {
    let mut histogram = Histogram::new();
    for c in word.chars() {
        if !c.is_alphabetic() {
            return None;
        }
        // It may be the case that a single uppercase
        // char translates to several lowercase chars.
        for c_lowercase in c.to_lowercase() {
            histogram.insert(c_lowercase);
        }
    }
    Some(histogram)
}

#[test]
fn test_histogram_basic() {
    assert_eq!(word_histogram("aha!"), None);
    assert_eq!(word_histogram("").unwrap(), HashMultiSet::new());
    let aha = vec!['a', 'h', 'a'].into_iter().collect();
    assert_eq!(word_histogram("aha").unwrap(), aha);
}
