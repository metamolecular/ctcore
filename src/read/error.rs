use crate::text::Printable;

#[derive(Debug, PartialEq)]
pub enum Error {
    Character(usize, usize, Vec<Printable>),
    Eof(usize),
    Eol(usize),
    Overflow(usize, usize),
    Unprintable(usize, usize, u8),
}
