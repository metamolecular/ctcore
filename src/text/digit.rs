use super::{NonZeroDigit, Printable};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Digit {
    Zero,
    NonZero(NonZeroDigit),
}

impl Digit {
    pub fn d0() -> Self {
        Self::Zero
    }

    pub fn d1() -> Self {
        Self::NonZero(NonZeroDigit::D1)
    }

    pub fn d2() -> Self {
        Self::NonZero(NonZeroDigit::D2)
    }

    pub fn d3() -> Self {
        Self::NonZero(NonZeroDigit::D3)
    }

    pub fn d4() -> Self {
        Self::NonZero(NonZeroDigit::D4)
    }

    pub fn d5() -> Self {
        Self::NonZero(NonZeroDigit::D5)
    }

    pub fn d6() -> Self {
        Self::NonZero(NonZeroDigit::D6)
    }

    pub fn d7() -> Self {
        Self::NonZero(NonZeroDigit::D7)
    }

    pub fn d8() -> Self {
        Self::NonZero(NonZeroDigit::D8)
    }

    pub fn d9() -> Self {
        Self::NonZero(NonZeroDigit::D9)
    }

    pub fn from_printable(printable: Printable) -> Option<Self> {
        match printable {
            Printable::D0 => Some(Self::Zero),
            Printable::D1 => Some(Self::NonZero(NonZeroDigit::D1)),
            Printable::D2 => Some(Self::NonZero(NonZeroDigit::D2)),
            Printable::D3 => Some(Self::NonZero(NonZeroDigit::D3)),
            Printable::D4 => Some(Self::NonZero(NonZeroDigit::D4)),
            Printable::D5 => Some(Self::NonZero(NonZeroDigit::D5)),
            Printable::D6 => Some(Self::NonZero(NonZeroDigit::D6)),
            Printable::D7 => Some(Self::NonZero(NonZeroDigit::D7)),
            Printable::D8 => Some(Self::NonZero(NonZeroDigit::D8)),
            Printable::D9 => Some(Self::NonZero(NonZeroDigit::D9)),
            _ => None,
        }
    }

    pub fn split(mut int: u32) -> Vec<Self> {
        if int == 0 {
            return vec![Digit::Zero];
        }

        let mut digits = Vec::new();

        while int > 0 {
            digits.push(match int % 10 {
                0 => Digit::Zero,
                1 => Digit::NonZero(NonZeroDigit::D1),
                2 => Digit::NonZero(NonZeroDigit::D2),
                3 => Digit::NonZero(NonZeroDigit::D3),
                4 => Digit::NonZero(NonZeroDigit::D4),
                5 => Digit::NonZero(NonZeroDigit::D5),
                6 => Digit::NonZero(NonZeroDigit::D6),
                7 => Digit::NonZero(NonZeroDigit::D7),
                8 => Digit::NonZero(NonZeroDigit::D8),
                9 => Digit::NonZero(NonZeroDigit::D9),
                _ => unreachable!(),
            });

            int /= 10;
        }

        digits.reverse();

        digits
    }
}

#[cfg(test)]
mod split {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn zero() {
        assert_eq!(Digit::split(0), vec![Digit::Zero])
    }

    #[test]
    fn one() {
        assert_eq!(Digit::split(1), vec![Digit::NonZero(NonZeroDigit::D1)])
    }

    #[test]
    fn forty_two() {
        assert_eq!(Digit::split(42), vec![Digit::d4(), Digit::d2()])
    }
}
