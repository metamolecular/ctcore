use crate::text::{Digit, NonZeroDigit};

#[derive(Debug, PartialEq)]
pub struct Natural {
    head: NonZeroDigit,
    tail: Vec<Digit>,
}

impl Natural {
    pub fn new(head: NonZeroDigit) -> Self {
        Self {
            head,
            tail: Vec::new(),
        }
    }

    pub fn from_int(int: u32) -> Option<Self> {
        Self::from_digits(Digit::split(int))
    }

    pub fn from_digits(digits: Vec<Digit>) -> Option<Self> {
        let mut digits = digits.into_iter();
        let mut result = match digits.next()? {
            Digit::Zero => return None,
            Digit::NonZero(non_zero) => Self::new(non_zero),
        };

        for digit in digits {
            result.push(digit)
        }

        Some(result)
    }

    pub fn push(&mut self, digit: Digit) {
        self.tail.push(digit)
    }

    pub fn len(&self) -> usize {
        self.tail.len() + 1
    }
}

#[cfg(test)]
mod from_digits {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty() {
        assert_eq!(Natural::from_digits(vec![]), None)
    }

    #[test]
    fn leading_zero() {
        assert_eq!(Natural::from_digits(vec![Digit::d0(), Digit::d1()]), None)
    }

    #[test]
    fn leading_non_zero() {
        assert_eq!(
            Natural::from_digits(vec![Digit::d4(), Digit::d2()]),
            Some(Natural {
                head: NonZeroDigit::D4,
                tail: vec![Digit::d2()]
            })
        )
    }
}

#[cfg(test)]
mod from_int {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn zero() {
        assert_eq!(Natural::from_int(0), None)
    }

    #[test]
    fn one_digit() {
        assert_eq!(
            Natural::from_int(4),
            Some(Natural {
                head: NonZeroDigit::D4,
                tail: vec![]
            })
        )
    }

    #[test]
    fn two_digits() {
        assert_eq!(
            Natural::from_int(42),
            Some(Natural {
                head: NonZeroDigit::D4,
                tail: vec![Digit::d2()]
            })
        )
    }
}
