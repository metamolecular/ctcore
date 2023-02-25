use crate::text::Printable;

#[derive(Debug, PartialEq)]
pub enum Error {
    Character(Vec<Printable>),
}

impl Error {
    pub fn digit() -> Self {
        Self::Character(Printable::digits())
    }

    pub fn non_zero_digit() -> Self {
        Self::Character(Printable::non_zero_digits())
    }

    pub fn integer_leading() -> Self {
        Self::Character(vec![])
    }

    pub fn flag() -> Self {
        Self::Character(vec![Printable::D0, Printable::D1])
    }

    pub fn space() -> Self {
        Self::Character(vec![Printable::D0])
    }

    pub fn two_or_three() -> Self {
        Self::Character(vec![Printable::D2, Printable::D3])
    }

    pub fn zero() -> Self {
        Self::Character(vec![Printable::D0])
    }

    pub fn v() -> Self {
        Self::Character(vec![Printable::V])
    }
}
