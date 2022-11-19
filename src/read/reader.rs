use std::iter::Peekable;

use crate::text::{
    Character, Count, Digit, FixedCount, FortranFloat, FortranInt, Natural,
    NonZeroDigit, Sequence,
};

use super::Error;

pub struct Reader<'a> {
    iter: Peekable<&'a mut dyn Iterator<Item = u8>>,
    pub row: usize,
    pub column: usize,
}

impl<'a> Reader<'a> {
    pub fn new(iter: &'a mut dyn Iterator<Item = u8>) -> Self {
        Self {
            iter: iter.peekable(),
            row: 0,
            column: 0,
        }
    }

    pub fn line<const L: usize>(
        &mut self,
    ) -> Result<Option<Sequence<L>>, Error> {
        let chars = self.some(L)?;

        self.line_sequence(chars)
    }

    pub fn line_with_blacklist<const L: usize>(
        &mut self,
        items: &'a [&'a [Character]],
    ) -> Result<Option<Sequence<L>>, Error> {
        let mut blacklist = Blacklist::new(items);
        let mut result = Vec::new();

        for _ in 0..L {
            let decoded = match self.may_decode()? {
                Some(decoded) => decoded,
                None => break,
            };

            if let Some(hit) = blacklist.test(&decoded) {
                let start = self.column - hit.len();
                let stop = self.column;

                return Err(Error::Blacklist(self.row, start..stop));
            }

            result.push(decoded)
        }

        self.line_sequence(result)
    }

    pub fn sequence<const L: usize>(
        &mut self,
    ) -> Result<Option<Sequence<L>>, Error> {
        println!("sequence");
        let chars = match self.all_or_none(L)? {
            Some(chars) => chars,
            None => {
                println!("NONE");
                return Ok(None);
            }
        };

        println!("chars {:?}", chars);

        Ok(Some(Sequence::new(chars).expect("sequence")))
    }

    pub fn fortran_int<const I: usize>(
        &mut self,
    ) -> Result<Option<FortranInt<I>>, Error> {
        let padding = self.count::<CodePoint<SP>>(I)?;

        if padding == I || (padding == 0 && !self.line_continues()) {
            return Ok(None);
        } else if padding == I - 1 {
            return Ok(FortranInt::from_digit(self.must_decode::<Digit>()?));
        }

        if self.may_decode::<CodePoint<MI>>()?.is_some() {
            Ok(FortranInt::from_negative_parts(
                self.must_decode::<NonZeroDigit>()?,
                self.all::<Digit>(I - padding - 2)?,
            ))
        } else {
            Ok(FortranInt::from_postitive_parts(
                self.must_decode::<NonZeroDigit>()?,
                self.all::<Digit>(I - padding - 1)?,
            ))
        }
    }

    pub fn fortran_float<const I: usize, const F: usize>(
        &mut self,
    ) -> Result<Option<FortranFloat<I, F>>, Error> {
        let start = self.column;
        let integer_part = match self.fortran_int::<I>()? {
            Some(integer_part) => integer_part,
            None => {
                if self.column == start {
                    return Ok(None);
                } else {
                    self.all::<CodePoint<SP>>(F + 1)?;

                    return Ok(None);
                }
            }
        };

        self.must_decode::<CodePoint<PO>>()?;

        Ok(Some(
            FortranFloat::new(integer_part, self.all::<Digit>(F)?)
                .expect("fortran float"),
        ))
    }

    pub fn fixed_count<const I: usize>(
        &mut self,
    ) -> Result<Option<FixedCount<I>>, Error> {
        let leading_space = self.count::<CodePoint<SP>>(I)?;
        let digits = self.some::<Digit>(I - leading_space)?;

        self.all::<CodePoint<SP>>(I - leading_space - digits.len())?;

        if digits.is_empty() {
            return Ok(None);
        } else {
            Ok(FixedCount::from_digits(digits))
        }
    }

    pub fn newline(&mut self) -> Result<(), Error> {
        if self.iter.next_if_eq(&CR).is_some() {
            self.iter.next_if_eq(&LF);
        } else if self.iter.next_if_eq(&LF).is_some() {
            self.iter.next_if_eq(&CR);
        } else if self.iter.next_if_eq(&RS).is_none() {
            return Err(Error::Continuation(self.row, self.column));
        }

        self.column = 0;
        self.row += 1;

        Ok(())
    }

    pub fn space(&mut self) -> Result<(), Error> {
        self.must_decode::<CodePoint<SP>>()?;

        Ok(())
    }

    pub fn variable_count(&mut self) -> Result<Count, Error> {
        let first = self.must_decode::<Digit>()?;

        if first.is_zero() {
            while self.may_decode::<CodePoint<ZE>>()?.is_some() {}
        }

        let head = match first.to_non_zero() {
            Some(non_zero) => non_zero,
            None => match self.may_decode::<NonZeroDigit>()? {
                Some(non_zero) => non_zero,
                None => return Ok(Count::Zero),
            },
        };
        let mut tail = Vec::new();

        while let Some(digit) = self.may_decode::<Digit>()? {
            tail.push(digit)
        }

        Ok(Count::Positive(Natural { head, tail }))
    }

    fn must_decode<D: Decode>(&mut self) -> Result<D, Error> {
        match self.iter.peek() {
            Some(&byte) => match byte {
                0x00..=0x1f | 0x7f => match byte {
                    CR | LF | RS => Err(Error::EndOfLine(self.row)),
                    _ => Err(Error::Unprintable(self.row, self.column)),
                },
                0x20..=0x7e => match D::decode(byte) {
                    Some(decoded) => {
                        self.column += 1;

                        self.iter.next();

                        Ok(decoded)
                    }
                    None => Err(Error::Character(
                        self.row,
                        self.column,
                        D::expecting(),
                    )),
                },
                0x80.. => Err(Error::CodePoint(self.row, self.column)),
            },
            None => Err(Error::EndOfFile(self.row)),
        }
    }

    fn may_decode<D: Decode>(&mut self) -> Result<Option<D>, Error> {
        match self.iter.peek() {
            Some(&byte) => match byte {
                0x00..=0x1f | 0x7f => match byte {
                    CR | LF | RS => Ok(None),
                    _ => Err(Error::Unprintable(self.row, self.column)),
                },
                0x20..=0x7e => match D::decode(byte) {
                    Some(decoded) => {
                        self.column += 1;

                        self.iter.next();

                        Ok(Some(decoded))
                    }
                    None => Ok(None),
                },
                0x80.. => Err(Error::CodePoint(self.row, self.column)),
            },
            None => Ok(None),
        }
    }

    fn line_continues(&mut self) -> bool {
        match self.iter.peek() {
            Some(&CR | &LF | &RS) | None => false,
            _ => true,
        }
    }

    fn all<D: Decode>(&mut self, limit: usize) -> Result<Vec<D>, Error> {
        let mut result = Vec::new();

        for _ in 0..limit {
            result.push(self.must_decode()?)
        }

        Ok(result)
    }

    fn all_or_none<D: Decode>(
        &mut self,
        limit: usize,
    ) -> Result<Option<Vec<D>>, Error> {
        if limit == 0 {
            return Ok(None);
        }

        let mut result = vec![match self.may_decode()? {
            Some(decoded) => decoded,
            None => return Ok(None),
        }];

        for _ in 1..limit {
            result.push(self.must_decode()?)
        }

        Ok(Some(result))
    }

    fn some<D: Decode>(&mut self, limit: usize) -> Result<Vec<D>, Error> {
        let mut result = Vec::new();

        for _ in 0..limit {
            match self.may_decode()? {
                Some(decoded) => result.push(decoded),
                None => break,
            }
        }

        Ok(result)
    }

    fn count<D: Decode>(&mut self, limit: usize) -> Result<usize, Error> {
        let mut result = 0;

        for _ in 0..limit {
            match self.may_decode::<D>()? {
                Some(_) => result += 1,
                None => break,
            }
        }

        Ok(result)
    }

    fn line_sequence<const L: usize>(
        &mut self,
        items: Vec<Character>,
    ) -> Result<Option<Sequence<L>>, Error> {
        if self.line_continues() {
            Err(Error::Overflow(self.row, self.column))
        } else if items.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Sequence::<L>::new(items).expect("sequence")))
        }
    }
}

trait Decode: Sized {
    fn decode(byte: u8) -> Option<Self>;
    fn expecting() -> Vec<Character>;
}

impl Decode for Character {
    fn decode(int: u8) -> Option<Self> {
        Character::from_byte(int)
    }

    fn expecting() -> Vec<Character> {
        Character::options()
    }
}

impl Decode for Digit {
    fn decode(byte: u8) -> Option<Self> {
        Digit::from_byte(byte)
    }

    fn expecting() -> Vec<Character> {
        vec![
            Character::D0,
            Character::D1,
            Character::D2,
            Character::D3,
            Character::D4,
            Character::D5,
            Character::D6,
            Character::D7,
            Character::D8,
            Character::D9,
        ]
    }
}

impl Decode for NonZeroDigit {
    fn decode(byte: u8) -> Option<Self> {
        NonZeroDigit::from_byte(byte)
    }

    fn expecting() -> Vec<Character> {
        vec![
            Character::D1,
            Character::D2,
            Character::D3,
            Character::D4,
            Character::D5,
            Character::D6,
            Character::D7,
            Character::D8,
            Character::D9,
        ]
    }
}

struct CodePoint<const P: u8> {}

impl<const P: u8> Decode for CodePoint<P> {
    fn decode(byte: u8) -> Option<Self> {
        if byte == P {
            Some(Self {})
        } else {
            None
        }
    }

    fn expecting() -> Vec<Character> {
        vec![Character::from_byte(P).expect("character")]
    }
}

struct Blacklist<'a> {
    items: Vec<(usize, &'a [Character])>,
}

impl<'a> Blacklist<'a> {
    pub fn new(items: &'a [&'a [Character]]) -> Self {
        Self {
            items: items.iter().map(|i| (0, *i)).collect::<Vec<_>>(),
        }
    }

    pub fn test(&mut self, test: &Character) -> Option<&'a [Character]> {
        for (index, item) in self.items.iter_mut() {
            if &item[*index] == test {
                if *index + 1 == item.len() {
                    return Some(item);
                } else {
                    *index += 1
                }
            } else {
                *index = 0
            }
        }

        None
    }
}

const LF: u8 = 0x0a;
const CR: u8 = 0x0d;
const RS: u8 = 0x1e;
const SP: u8 = 0x20;
const MI: u8 = 0x2d;
const PO: u8 = 0x2e;
const ZE: u8 = 0x30;

#[cfg(test)]
mod blacklist {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn one_item_hit_not_found() {
        let items = [[Character::A].as_slice()];
        let mut blacklist = Blacklist::new(&items);

        assert_eq!(blacklist.test(&Character::X), None)
    }

    #[test]
    fn one_item_hit_found() {
        let items = [[Character::X].as_slice()];
        let mut blacklist = Blacklist::new(&items);

        assert_eq!(blacklist.test(&Character::X), Some(items[0]))
    }

    #[test]
    fn one_item_after_fail_hit_found() {
        let items = [
            [Character::A, Character::B].as_slice(),
            [Character::X, Character::Y].as_slice(),
        ];
        let mut blacklist = Blacklist::new(&items);

        assert_eq!(blacklist.test(&Character::X), None);
        assert_eq!(blacklist.test(&Character::A), None);
        assert_eq!(blacklist.test(&Character::X), None);
        assert_eq!(blacklist.test(&Character::Y), Some(items[1]))
    }

    #[test]
    fn two_items_hit_found() {
        let items = [[Character::A].as_slice(), [Character::B].as_slice()];
        let mut blacklist = Blacklist::new(&items);

        assert_eq!(blacklist.test(&Character::B), Some(items[1]))
    }
}

#[cfg(test)]
mod may_decode {
    use crate::text::Digit;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn bad_code_point() {
        let mut bytes = b"\xff".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.may_decode::<Character>(),
            Err(Error::CodePoint(0, 0))
        );
        assert_eq!(reader.column, 0)
    }

    #[test]
    fn unprintable() {
        let mut bytes = b"\x00".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.may_decode::<Character>(),
            Err(Error::Unprintable(0, 0))
        );
        assert_eq!(reader.column, 0)
    }

    #[test]
    fn eof() {
        let mut bytes = b"".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.may_decode::<Character>(), Ok(None));
        assert_eq!(reader.column, 0)
    }

    #[test]
    fn eol() {
        let mut bytes = b"\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.may_decode::<Character>(), Ok(None));
        assert_eq!(reader.column, 0)
    }

    #[test]
    fn undecodable() {
        let mut bytes = b"A".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.may_decode::<Digit>(), Ok(None));
        assert_eq!(reader.column, 0)
    }

    #[test]
    fn decodable() {
        let mut bytes = b"7".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.may_decode::<Digit>(), Ok(Some(Digit::d7())));
        assert_eq!(reader.column, 1)
    }
}

#[cfg(test)]
mod must_decode {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn bad_code_point() {
        let mut bytes = b"\xff".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.must_decode::<Character>(),
            Err(Error::CodePoint(0, 0))
        );
        assert_eq!(reader.column, 0)
    }

    #[test]
    fn unprintable() {
        let mut bytes = b"\x00".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.must_decode::<Character>(),
            Err(Error::Unprintable(0, 0))
        );
        assert_eq!(reader.column, 0)
    }

    #[test]
    fn eof() {
        let mut bytes = b"".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.must_decode::<Character>(), Err(Error::EndOfFile(0)));
        assert_eq!(reader.column, 0)
    }

    #[test]
    fn eol() {
        let mut bytes = b"\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.must_decode::<Character>(), Err(Error::EndOfLine(0)));
        assert_eq!(reader.column, 0)
    }

    #[test]
    fn undecodable() {
        let mut bytes = b"A".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.must_decode::<Digit>(),
            Err(Error::Character(0, 0, Digit::expecting()))
        );
        assert_eq!(reader.column, 0)
    }

    #[test]
    fn decodable() {
        let mut bytes = b"7".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.must_decode::<Digit>(), Ok(Digit::d7()));
        assert_eq!(reader.column, 1)
    }
}

#[cfg(test)]
mod line {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn invalid_byte_middle() {
        let mut bytes = b"abc\xffdef".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.line::<80>(), Err(Error::CodePoint(0, 3)))
    }

    #[test]
    fn newline_middle() {
        let mut bytes = b"abc\ndef".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.line::<80>(),
            Ok(Some(Sequence::from_str("abc").unwrap()))
        )
    }

    #[test]
    fn overflow() {
        let mut bytes = b"abcdef".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.line::<5>(), Err(Error::Overflow(0, 5)))
    }

    #[test]
    fn eof() {
        let mut bytes = "".bytes();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.line::<80>(), Ok(None))
    }

    #[test]
    fn underflow() {
        let mut bytes = "abcd".bytes();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.line::<80>(),
            Ok(Some(Sequence::from_str("abcd").unwrap()))
        )
    }

    #[test]
    fn at_flow() {
        let mut bytes = "abcd\n".bytes();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.line::<4>(),
            Ok(Some(Sequence::from_str("abcd").unwrap()))
        )
    }
}

#[cfg(test)]
mod line_with_blacklist {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn invalid_byte_middle() {
        let mut bytes = b"abc\xffdef".iter().cloned();
        let mut reader = Reader::new(&mut bytes);
        let blacklist = [];

        assert_eq!(
            reader.line_with_blacklist::<80>(&blacklist),
            Err(Error::CodePoint(0, 3))
        )
    }

    #[test]
    fn newline_middle() {
        let mut bytes = b"abc\ndef".iter().cloned();
        let mut reader = Reader::new(&mut bytes);
        let blacklist = [];

        assert_eq!(
            reader.line_with_blacklist::<80>(&blacklist),
            Ok(Some(Sequence::from_str("abc").unwrap()))
        )
    }

    #[test]
    fn overflow() {
        let mut bytes = b"abcdef".iter().cloned();
        let mut reader = Reader::new(&mut bytes);
        let blacklist = [];

        assert_eq!(
            reader.line_with_blacklist::<5>(&blacklist),
            Err(Error::Overflow(0, 5))
        )
    }

    #[test]
    fn blacklisted() {
        let mut bytes = b"abc$$$$def".iter().cloned();
        let mut reader = Reader::new(&mut bytes);
        let blacklist = [[
            Character::Dollar,
            Character::Dollar,
            Character::Dollar,
            Character::Dollar,
        ]
        .as_ref()];

        assert_eq!(
            reader.line_with_blacklist::<80>(&blacklist),
            Err(Error::Blacklist(0, 3..7))
        )
    }

    #[test]
    fn eof() {
        let mut bytes = b"".iter().cloned();
        let mut reader = Reader::new(&mut bytes);
        let blacklist = [];

        assert_eq!(reader.line_with_blacklist::<5>(&blacklist), Ok(None))
    }

    #[test]
    fn underflow() {
        let mut bytes = b"abc".iter().cloned();
        let mut reader = Reader::new(&mut bytes);
        let blacklist = [];

        assert_eq!(
            reader.line_with_blacklist::<5>(&blacklist),
            Ok(Some(Sequence::from_str("abc").unwrap()))
        )
    }

    #[test]
    fn at_flow() {
        let mut bytes = b"abcde".iter().cloned();
        let mut reader = Reader::new(&mut bytes);
        let blacklist = [];

        assert_eq!(
            reader.line_with_blacklist::<5>(&blacklist),
            Ok(Some(Sequence::from_str("abcde").unwrap()))
        )
    }
}

#[cfg(test)]
mod newline {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn more() {
        let mut bytes = b"abc".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.newline(), Err(Error::Continuation(0, 0)))
    }

    #[test]
    fn lf_eol() {
        let mut bytes = b"\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.newline(), Ok(()));
        assert_eq!(reader.column, 0);
        assert_eq!(reader.row, 1);
        assert_eq!(bytes.next(), None)
    }

    #[test]
    fn lf_cr_eol() {
        let mut bytes = b"\n\r".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.newline(), Ok(()));
        assert_eq!(reader.column, 0);
        assert_eq!(reader.row, 1);
        assert_eq!(bytes.next(), None)
    }

    #[test]
    fn lf_cr_printable() {
        let mut bytes = b"\n\rx".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.newline(), Ok(()));
        assert_eq!(reader.column, 0);
        assert_eq!(reader.row, 1);
    }

    #[test]
    fn cr_eol() {
        let mut bytes = b"\r".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.newline(), Ok(()));
        assert_eq!(reader.column, 0);
        assert_eq!(reader.row, 1);
    }

    #[test]
    fn cr_printable() {
        let mut bytes = b"\rx".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.newline(), Ok(()));
        assert_eq!(reader.column, 0);
        assert_eq!(reader.row, 1)
    }

    #[test]
    fn cr_lf_eol() {
        let mut bytes = b"\r\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.newline(), Ok(()));
        assert_eq!(reader.column, 0);
        assert_eq!(reader.row, 1);
    }

    #[test]
    fn cr_lf_printable() {
        let mut bytes = b"\r\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.newline(), Ok(()));
        assert_eq!(reader.column, 0);
        assert_eq!(reader.row, 1);
    }

    #[test]
    fn rs_eol() {
        let mut bytes = b"\x1e".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.newline(), Ok(()));
        assert_eq!(reader.column, 0);
        assert_eq!(reader.row, 1);
    }
}

#[cfg(test)]
mod variable_count {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn eof() {
        let mut bytes = b"".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.variable_count(), Err(Error::EndOfFile(0)))
    }

    #[test]
    fn eol() {
        let mut bytes = b"\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.variable_count(), Err(Error::EndOfLine(0)))
    }

    #[test]
    fn alpha() {
        let mut bytes = b"x".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.variable_count(),
            Err(Error::Character(0, 0, Digit::expecting()))
        )
    }

    #[test]
    fn zero_eof() {
        let mut bytes = b"0".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.variable_count(), Ok(Count::Zero))
    }

    #[test]
    fn zero_zero_eof() {
        let mut bytes = b"00".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.variable_count(), Ok(Count::Zero));
        assert_eq!(reader.column, 2)
    }

    #[test]
    fn zero_non_zero_eof() {
        let mut bytes = b"07".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.variable_count(), Ok(Count::from_int(7)));
        assert_eq!(reader.column, 2)
    }

    #[test]
    fn non_zero_eof() {
        let mut bytes = b"1".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.variable_count(), Ok(Count::from_int(1)))
    }

    #[test]
    fn non_zero_non_zero_eof() {
        let mut bytes = b"42".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.variable_count(), Ok(Count::from_int(42)))
    }

    #[test]
    fn non_zero_non_zero_space() {
        let mut bytes = b"42 ".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.variable_count(), Ok(Count::from_int(42)));
        assert_eq!(reader.column, 2)
    }
}

#[cfg(test)]
mod sequence {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn eof() {
        let mut bytes = b"".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.sequence::<8>(), Ok(None))
    }

    #[test]
    fn eol() {
        let mut bytes = b"\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.sequence::<8>(), Ok(None))
    }

    #[test]
    fn underflow_eof() {
        let mut bytes = b"abc".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.sequence::<8>(), Err(Error::EndOfFile(0)))
    }

    #[test]
    fn underflow_cr() {
        let mut bytes = b"abc\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.sequence::<8>(), Err(Error::EndOfLine(0)))
    }

    #[test]
    fn at_capacity_eof() {
        let mut bytes = b"abcde".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.sequence::<5>(),
            Ok(Some(Sequence::from_str("abcde").unwrap()))
        )
    }

    #[test]
    fn at_capacity_cr() {
        let mut bytes = b"abcde\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.sequence::<5>(),
            Ok(Some(Sequence::from_str("abcde").unwrap()))
        )
    }
}

#[cfg(test)]
mod fortran_int {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn eof() {
        let mut bytes = b"".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fortran_int::<4>(), Ok(None))
    }

    #[test]
    fn eol() {
        let mut bytes = b"\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fortran_int::<4>(), Ok(None))
    }

    #[test]
    fn underflow_spaces() {
        let mut bytes = b"   ".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fortran_int::<4>(), Err(Error::EndOfFile(0)))
    }

    #[test]
    fn spaces_minus_eof() {
        let mut bytes = b"  -".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fortran_int::<4>(), Err(Error::EndOfFile(0)))
    }

    #[test]
    fn spaces_minus_space() {
        let mut bytes = b"  - ".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.fortran_int::<4>(),
            Err(Error::Character(0, 3, NonZeroDigit::expecting()))
        )
    }

    #[test]
    fn minus_space() {
        let mut bytes = b"- ".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.fortran_int::<4>(),
            Err(Error::Character(0, 1, NonZeroDigit::expecting()))
        )
    }

    #[test]
    fn digit_space() {
        let mut bytes = b"1 ".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.fortran_int::<4>(),
            Err(Error::Character(0, 1, Digit::expecting()))
        )
    }

    #[test]
    fn spaces_minus_digit_eof() {
        let mut bytes = b"  -1".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fortran_int::<5>(), Err(Error::EndOfFile(0)))
    }

    #[test]
    fn padded_negative_zero() {
        let mut bytes = b"  -0".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.fortran_int::<4>(),
            Err(Error::Character(0, 3, NonZeroDigit::expecting()))
        )
    }

    #[test]
    fn all_spaces() {
        let mut bytes = b"    ".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fortran_int::<4>(), Ok(None))
    }

    #[test]
    fn padded_zero() {
        let mut bytes = b"   0".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.fortran_int::<4>(),
            Ok(Some(FortranInt::from_int(0).unwrap()))
        )
    }

    #[test]
    fn padded_non_zero() {
        let mut bytes = b"   1".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.fortran_int::<4>(),
            Ok(Some(FortranInt::from_int(1).unwrap()))
        )
    }

    #[test]

    fn padded_negative() {
        let mut bytes = b"  -1".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.fortran_int::<4>(),
            Ok(Some(FortranInt::from_int(-1).unwrap()))
        )
    }

    #[test]
    fn all_digits() {
        let mut bytes = b"1234".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.fortran_int::<4>(),
            Ok(Some(FortranInt::from_int(1234).unwrap()))
        )
    }

    #[test]
    fn all_digits_more() {
        let mut bytes = b"12345".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.fortran_int::<4>(),
            Ok(Some(FortranInt::from_int(1234).unwrap()))
        )
    }

    #[test]
    fn all_digits_negative() {
        let mut bytes = b"-123".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.fortran_int::<4>(),
            Ok(Some(FortranInt::from_int(-123).unwrap()))
        )
    }
}

#[cfg(test)]
mod fortran_float {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn eof() {
        let mut bytes = b"".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fortran_float::<4, 4>(), Ok(None))
    }

    #[test]
    fn eol() {
        let mut bytes = b"\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fortran_float::<4, 4>(), Ok(None))
    }

    #[test]
    fn underflow_spaces() {
        let mut bytes = b"   ".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fortran_float::<4, 2>(), Err(Error::EndOfFile(0)))
    }

    #[test]
    fn spaces_digit_eof() {
        let mut bytes = b"   1".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fortran_float::<4, 2>(), Err(Error::EndOfFile(0)))
    }

    #[test]
    fn digit_dot_eof() {
        let mut bytes = b"   1.".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fortran_float::<4, 2>(), Err(Error::EndOfFile(0)))
    }

    #[test]
    fn spaces_dot_eof() {
        let mut bytes = b"    .".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.fortran_float::<4, 2>(),
            Err(Error::Character(0, 4, vec![Character::Space]))
        )
    }

    #[test]
    fn digit_dot_digiteof() {
        let mut bytes = b"   1.4".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fortran_float::<4, 2>(), Err(Error::EndOfFile(0)))
    }

    #[test]
    fn all_spaces() {
        let mut bytes = b"       ".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fortran_float::<4, 2>(), Ok(None))
    }

    #[test]
    fn positive_padded_left() {
        let mut bytes = b"   1.42".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.fortran_float::<4, 2>(),
            Ok(Some(FortranFloat::from_float(1.42).unwrap()))
        )
    }

    #[test]
    fn negative_padded_left() {
        let mut bytes = b"  -1.42".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.fortran_float::<4, 2>(),
            Ok(Some(FortranFloat::from_float(-1.42).unwrap()))
        )
    }

    #[test]
    fn positive_full() {
        let mut bytes = b"4321.42".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.fortran_float::<4, 2>(),
            Ok(Some(FortranFloat::from_float(4321.42).unwrap()))
        )
    }

    #[test]
    fn negative_full() {
        let mut bytes = b"-321.42".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.fortran_float::<4, 2>(),
            Ok(Some(FortranFloat::from_float(-321.42).unwrap()))
        )
    }
}

#[cfg(test)]
mod fixed_count {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn i_zero_eof() {
        let mut bytes = b"".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fixed_count::<0>(), Ok(None))
    }

    #[test]
    fn i_zero_eol() {
        let mut bytes = b"\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fixed_count::<0>(), Ok(None))
    }

    #[test]
    fn i_non_zero_eof() {
        let mut bytes = b"\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fixed_count::<1>(), Err(Error::EndOfLine(0)))
    }

    #[test]
    fn underflow_padded() {
        let mut bytes = b"  ".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fixed_count::<3>(), Err(Error::EndOfFile(0)))
    }

    #[test]
    fn padded_zero_underflow() {
        let mut bytes = b" 0".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fixed_count::<3>(), Err(Error::EndOfFile(0)))
    }

    #[test]
    fn space_digit_eof() {
        let mut bytes = b" 1".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fixed_count::<3>(), Err(Error::EndOfFile(0)))
    }

    #[test]
    fn digit_eof() {
        let mut bytes = b"1".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fixed_count::<3>(), Err(Error::EndOfFile(0)))
    }

    #[test]
    fn all_spaces() {
        let mut bytes = b"   ".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fixed_count::<3>(), Ok(None))
    }

    #[test]
    fn all_zeroes() {
        let mut bytes = b"000".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fixed_count::<3>(), Ok(FixedCount::from_int(0)))
    }

    #[test]
    fn digit_spaces() {
        let mut bytes = b"1  ".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fixed_count::<3>(), Ok(FixedCount::from_int(1)))
    }

    #[test]
    fn space_zero_space() {
        let mut bytes = b" 0 ".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fixed_count::<3>(), Ok(Some(FixedCount::Zero)))
    }

    #[test]
    fn space_zero_digit() {
        let mut bytes = b" 01".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fixed_count::<3>(), Ok(FixedCount::from_int(1)))
    }

    #[test]
    fn digit_padded_left() {
        let mut bytes = b"  1".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.fixed_count::<3>(),
            Ok(Some(FixedCount::from_int(1).unwrap()))
        )
    }

    #[test]
    fn digit_padded_right() {
        let mut bytes = b"1  ".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.fixed_count::<3>(), Ok(FixedCount::from_int(1)))
    }
}

#[cfg(test)]
mod all {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn limit_0_eof() {
        let mut bytes = b"".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.all::<Digit>(0), Ok(vec![]))
    }

    #[test]
    fn limit_0_eol() {
        let mut bytes = b"\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.all::<Digit>(0), Ok(vec![]))
    }

    #[test]
    fn limit_1_eof() {
        let mut bytes = b"".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.all::<Digit>(1), Err(Error::EndOfFile(0)))
    }

    #[test]
    fn limit_1_eol() {
        let mut bytes = b"\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.all::<Digit>(1), Err(Error::EndOfLine(0)))
    }

    #[test]
    fn limit_1_bad_character() {
        let mut bytes = b"a".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.all::<Digit>(1),
            Err(Error::Character(0, 0, Digit::expecting()))
        )
    }

    #[test]
    fn limit_1_all() {
        let mut bytes = b"42".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.all::<Digit>(1), Ok(vec![Digit::d4()]))
    }
}

#[cfg(test)]
mod all_or_none {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn limit_0_eof() {
        let mut bytes = b"".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.all_or_none::<Digit>(0), Ok(None))
    }

    #[test]
    fn limit_0_eol() {
        let mut bytes = b"\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.all_or_none::<Digit>(0), Ok(None))
    }

    #[test]
    fn limit_1_eof() {
        let mut bytes = b"".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.all_or_none::<Digit>(1), Ok(None))
    }

    #[test]
    fn limit_1_eol() {
        let mut bytes = b"\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.all_or_none::<Digit>(1), Ok(None))
    }

    #[test]
    fn limit_1_bad_character() {
        let mut bytes = b"a".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.all_or_none::<Digit>(1), Ok(None))
    }

    #[test]
    fn limit_2_good_then_bad_character() {
        let mut bytes = b"1a".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.all_or_none::<Digit>(2),
            Err(Error::Character(0, 1, Digit::expecting()))
        )
    }

    #[test]
    fn limit_1_all() {
        let mut bytes = b"42".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.all_or_none::<Digit>(1), Ok(Some(vec![Digit::d4()])))
    }

    #[test]
    fn limit_2_all() {
        let mut bytes = b"42".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(
            reader.all_or_none::<Digit>(2),
            Ok(Some(vec![Digit::d4(), Digit::d2()]))
        )
    }
}

#[cfg(test)]
mod some {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn limit_0_eof() {
        let mut bytes = b"".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.some::<Digit>(0), Ok(vec![]))
    }

    #[test]
    fn limit_0_eol() {
        let mut bytes = b"\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.some::<Digit>(0), Ok(vec![]))
    }

    #[test]
    fn limit_1_eof() {
        let mut bytes = b"".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.some::<Digit>(1), Ok(vec![]))
    }

    #[test]
    fn limit_1_eol() {
        let mut bytes = b"\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.some::<Digit>(1), Ok(vec![]))
    }

    #[test]
    fn limit_1_bad_character() {
        let mut bytes = b"a".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.some::<Digit>(1), Ok(vec![]))
    }

    #[test]
    fn limit_2_good_then_bad_character() {
        let mut bytes = b"1a".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.some::<Digit>(2), Ok(vec![Digit::d1()]))
    }

    #[test]
    fn limit_2_all() {
        let mut bytes = b"42".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.some::<Digit>(2), Ok(vec![Digit::d4(), Digit::d2()]))
    }
}

#[cfg(test)]
mod line_continues {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn eof() {
        let mut bytes = b"".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.line_continues(), false)
    }

    #[test]
    fn eol() {
        let mut bytes = b"\n".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.line_continues(), false)
    }

    #[test]
    fn any() {
        let mut bytes = b"x".iter().cloned();
        let mut reader = Reader::new(&mut bytes);

        assert_eq!(reader.line_continues(), true)
    }
}
