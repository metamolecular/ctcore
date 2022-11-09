use super::{Digit, NonZeroDigit};

#[derive(Debug, PartialEq, Clone)]
pub struct Natural {
    pub head: NonZeroDigit,
    pub tail: Vec<Digit>,
}

impl Natural {
    pub fn new(head: NonZeroDigit, tail: Vec<Digit>) -> Self {
        Self { head, tail }
    }

    pub fn from_digit(head: NonZeroDigit) -> Self {
        Self { head, tail: vec![] }
    }

    pub fn from_digits(mut digits: Vec<Digit>) -> Option<Self> {
        while !digits.is_empty() {
            let digit = digits.remove(0);

            match digit.to_non_zero() {
                Some(non_zero) => return Some(Self::new(non_zero, digits)),
                None => (),
            }
        }

        None
    }

    pub fn width(&self) -> usize {
        self.tail.len() + 1
    }
}

#[cfg(test)]
mod from_digits {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn empty() {
        let digits = vec![];

        assert_eq!(Natural::from_digits(digits), None)
    }

    #[test]
    fn one_zero() {
        let digits = vec![Digit::d0()];

        assert_eq!(Natural::from_digits(digits), None)
    }

    #[test]
    fn three_zeros() {
        let digits = vec![Digit::d0(), Digit::d0(), Digit::d0()];

        assert_eq!(Natural::from_digits(digits), None)
    }

    #[test]
    fn leading_zero() {
        let digits = vec![Digit::d0(), Digit::d4(), Digit::d2()];

        assert_eq!(
            Natural::from_digits(digits),
            Some(Natural {
                head: NonZeroDigit::D4,
                tail: vec![Digit::d2()]
            })
        )
    }

    #[test]
    fn leading_non_zero() {
        let digits = vec![Digit::d4(), Digit::d2()];

        assert_eq!(
            Natural::from_digits(digits),
            Some(Natural {
                head: NonZeroDigit::D4,
                tail: vec![Digit::d2()]
            })
        )
    }
}
