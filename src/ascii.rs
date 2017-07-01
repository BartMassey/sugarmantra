// Copyright © 2017 Bart Massey

// Provide basic conversions between characters and Unicode
// scalars.

use std::char::*;

pub fn ord(c: char) -> u32 {
    let mut b = [0; 4];
    let _ = c.encode_utf8(&mut b);
    let mut result = 0;
    for i in 0..4 {
        result |= b[4-i] as u32;
    };
    result
}

pub fn chr(c: u32) -> char {
    from_u32(c).expect("chr: illegal unicode character code")
}

pub fn is_ascii(c: char) -> bool {
    ord(c) <= 0x7f
}

pub fn is_lower(c: char) -> bool {
    let x = ord(c);
    x >= ord('a') && x <= ord('z')
}

pub fn is_upper(c: char) -> bool {
    let x = ord(c);
    x >= ord('A') && x <= ord('Z')
}

pub fn is_alpha(c: char) -> bool {
    is_lower(c) || is_upper(c)
}

pub fn to_lower(c: char) -> char {
    if !is_upper(c) {
        return c;
    };
    chr(ord(c) - ord('A') + ord('a'))
}

pub fn to_upper(c: char) -> char {
    if !is_lower(c) {
        return c;
    };
    chr(ord(c) - ord('a') + ord('A'))
}
