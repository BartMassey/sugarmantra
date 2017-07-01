# anagram-rust
Copyright © 2017 Bart Massey

This is a fairly direct translation of an anagram solver I
originally wrote in [Nickle](http://nickle.org) a million
years ago. It is much faster in Rust.

The basic strategy is to work in the space of histograms,
using a do-undo depth-first search over the supplied
dictionary.

The program expects to find a dictionary named `dict.txt` in
its working directory. You can get such a dictionary from my
[wordlists](http://github.com/BartMassey/wordlists) or just
use `/usr/dict/words` or whatever.

The current output is kind of garbage: it allows stray
single letters for completeness. More sophisticated
heuristics for what actually constitutes an anagram would be
better.
