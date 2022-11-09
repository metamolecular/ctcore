use std::ops::Range;

use crate::text::Character;

#[derive(Debug, PartialEq)]
pub enum Error {
    Blacklist(usize, Range<usize>),
    Character(usize, usize, Vec<Character>),
    CodePoint(usize, usize),
    Continuation(usize, usize),
    EndOfFile(usize),
    EndOfLine(usize),
    Overflow(usize, usize),
    Unprintable(usize, usize),
}
