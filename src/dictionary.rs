// Copyright © 2017 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.

//! Dictionary support for sugarmantra.

use crate::histogram::*;

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::*;

#[derive(Debug)]
pub enum DictionaryError {
    NotFound,
    ReadFailed(io::Error),
    BadStem(String),
}

impl fmt::Display for DictionaryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DictionaryError::NotFound => {
                write!(f, "could not find a dictionary")
            }
            DictionaryError::ReadFailed(e) => {
                write!(f, "could not read from dictionary: {}", e)
            }
            DictionaryError::BadStem(s) => {
                write!(f, "illegal stem {}", s)
            }
        }
    }
}

impl Error for DictionaryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DictionaryError::NotFound => None,
            DictionaryError::ReadFailed(e) => Some(e),
            DictionaryError::BadStem(_) => None,
        }
    }
}

/// Read the word list from some dictionary.
fn open_system_dict() -> Result<File, DictionaryError> {
    for file in ["freq.txt", "scowl.txt", "eowl.txt", "words"].iter() {
        for dir in [".", "/usr/local/share/dict", "/usr/share/dict"].iter() {
            let mut path = PathBuf::from(dir);
            path.push(file);
            if let Ok(f) = File::open(path) {
                return Ok(f);
            }
        }
    }
    Err(DictionaryError::NotFound)
}

/// A dictionary entry maps a word to its corresponding
/// histogram.
pub struct Entry {
    pub word: String,
    pub whist: Histogram,
}

/// Parse the words in the dictionary into histograms.  Do
/// some pruning along the way for efficiency: the `target`
/// argument restricts the dictionary to words that can be
/// made from the target letters.  Augment the dictionary
/// with common stems that can be used to help construct
/// words.
pub fn load_dictionary<P>(
    path: Option<P>,
    limit: Option<usize>,
    stems: String,
    target: &Histogram,
) -> Result<Vec<Entry>, DictionaryError>
    where P: AsRef<Path>
{
    // Load in the dictionary.
    let mut dict: Vec<Entry> = Vec::new();
    let f = if let Some(path) = path {
        File::open(path).map_err(|_| DictionaryError::NotFound)?
    } else {
        open_system_dict()?
    };
    let r = BufReader::new(&f);
    for line in r.lines() {
        let word = line.map_err(DictionaryError::ReadFailed)?;
        if word.len() <= 1 {
            continue;
        }
        if let Some(limit) = limit {
            if word.len() > limit {
                continue;
            }
        }
        if let Some(whist) = word_histogram(&word) {
            if whist.is_submultiset(target) {
                let e = Entry { whist, word };
                dict.push(e);
            }
        }
    }
    // Add the stems.
    for word in stems.split(',').map(str::to_owned) {
        let whist = word_histogram(&word).ok_or_else(|| DictionaryError::BadStem(word.clone()))?;
        let e = Entry { word, whist };
        dict.push(e);
    }
    // Sort in order of increasing length.
    let len_order = |a: &Entry, b: &Entry| b.word.len().cmp(&a.word.len());
    dict.sort_by(len_order);
    Ok(dict)
}
