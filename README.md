# sugarmantra
Copyright © 2017 Bart Massey

This is a Rust anagram solver. The letters to be anagrammed
are given as the first argument: the output is a list of
anagrams.

This is a fairly direct translation of an anagram solver I
originally wrote in [Nickle](http://nickle.org) a million
years ago. It is much, much faster in Rust.

The basic strategy is to work in the space of histograms,
using a do-undo depth-first search over the supplied
dictionary. The current algorithm does not produce
algorithms with duplicate words, and outputs the words in
shortest-to-longest order with alphabetic suborder.

The program expects to find a dictionary named `dict.txt` in
its working directory. You can get such a dictionary from my
[wordlists](http://github.com/BartMassey/wordlists) or just
use `/usr/dict/words` or whatever. One-letter "words" are
excluded from the dictionary. "i", "a", "s" and some other
common word-parts are included in the word list to allow
more anagrams.

The program will kind of work with general Unicode, but
really has only been tested with ASCII. Also, the dictionary
supplements are specific to English.

This program is licensed under the "MIT License". Please see
the file `COPYING` in the source distribution of this software
for license terms.
