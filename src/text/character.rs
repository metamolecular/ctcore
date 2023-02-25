use super::{Eol, Printable};

#[derive(Debug, PartialEq)]
pub enum Character {
    Eol(Eol),
    Printable(Printable),
    Unprintable(u8),
}

impl Character {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0x00..=0x09 => Self::Unprintable(byte),
            0x0a => Self::Eol(Eol::Lf),
            0x0b..=0x0c => Self::Unprintable(byte),
            0x0d => Self::Eol(Eol::Cr),
            0x0e..=0x1d => Self::Unprintable(byte),
            0x1e => Self::Eol(Eol::Rs),
            0x1f..=0x1f => Self::Unprintable(byte),
            byte => match Printable::from_byte(byte) {
                Some(printable) => Self::Printable(printable),
                None => Self::Unprintable(byte),
            },
        }
    }

    pub fn is_cr(&self) -> bool {
        match self {
            Self::Eol(eol) => eol == &Eol::Cr,
            _ => false,
        }
    }

    pub fn is_rs(&self) -> bool {
        match self {
            Self::Eol(eol) => eol == &Eol::Rs,
            _ => false,
        }
    }

    pub fn is_lf(&self) -> bool {
        match self {
            Self::Eol(_) => true,
            _ => false,
        }
    }
}
