use super::{Digit, FixedNatural, Natural};

#[derive(Debug, PartialEq, Clone)]
pub enum FixedCount<const I: usize> {
    Positive(FixedNatural<I>),
    Zero,
}

impl<const I: usize> FixedCount<I> {
    pub fn from_int(int: u32) -> Option<Self> {
        if int == 0 {
            return Some(Self::Zero);
        } else {
            Some(Self::Positive(FixedNatural::from_int(int)?))
        }
    }

    pub fn from_digits(digits: Vec<Digit>) -> Option<Self> {
        if digits.is_empty() {
            return None;
        }

        let natural = match Natural::from_digits(digits) {
            Some(natural) => natural,
            None => return Some(Self::Zero),
        };

        match FixedNatural::new(natural) {
            Some(fixed_natural) => Some(Self::Positive(fixed_natural)),
            None => None,
        }
    }

    pub fn zero() -> Self {
        Self::Zero
    }
}

#[cfg(test)]
mod from_int {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn zero() {
        assert_eq!(FixedCount::<0>::from_int(0), Some(FixedCount::Zero))
    }

    #[test]
    fn nonzero() {
        assert_eq!(
            FixedCount::<1>::from_int(1),
            Some(FixedCount::Positive(FixedNatural::from_int(1).unwrap()))
        )
    }
}

#[cfg(test)]
mod from_digits {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn i_zero_empty() {
        let digits = vec![];

        assert_eq!(FixedCount::<0>::from_digits(digits), None)
    }

    #[test]
    fn i_non_zero_empty() {
        let digits = vec![];

        assert_eq!(FixedCount::<1>::from_digits(digits), None)
    }

    #[test]
    fn i_zero_one_zero() {
        let digits = vec![Digit::d0()];

        assert_eq!(FixedCount::<0>::from_digits(digits), Some(FixedCount::Zero))
    }

    #[test]
    fn i_zero_one_non_zero() {
        let digits = vec![Digit::d1()];

        assert_eq!(FixedCount::<0>::from_digits(digits), None)
    }

    #[test]
    fn i_non_zero_one_non_zero() {
        let digits = vec![Digit::d4(), Digit::d2()];

        assert_eq!(
            FixedCount::<1>::from_digits(digits),
            FixedCount::from_int(42)
        )
    }
}
