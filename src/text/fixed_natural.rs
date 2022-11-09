use super::{Digit, Natural, NonZeroDigit};

#[derive(Debug, PartialEq, Clone)]
pub struct FixedNatural<const I: usize>(Natural);

impl<const I: usize> FixedNatural<I> {
    pub fn new(natural: Natural) -> Option<Self> {
        if natural.width() > I {
            None
        } else {
            Some(Self(natural))
        }
    }

    pub fn from_digit(head: NonZeroDigit) -> Option<Self> {
        if I == 0 {
            None
        } else {
            Some(Self(Natural::new(head, vec![])))
        }
    }

    pub fn from_int(int: u32) -> Option<Self> {
        let mut digits = digits(int);

        if digits.is_empty() {
            return None;
        }

        Self::new(Natural::new(digits.remove(0).to_non_zero()?, digits))
    }
}

pub fn digits(mut int: u32) -> Vec<Digit> {
    let mut result = Vec::new();

    while int > 0 {
        result.push(Digit::from_int(int % 10).unwrap());

        int /= 10;
    }

    result.reverse();

    result
}

#[cfg(test)]
mod unsigned_digits {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn zero() {
        assert_eq!(digits(0), vec![])
    }

    #[test]
    fn one() {
        assert_eq!(digits(1), vec![Digit::d1()])
    }

    #[test]
    fn ten() {
        assert_eq!(digits(10), vec![Digit::d1(), Digit::d0()])
    }

    #[test]
    fn one_two_three_four() {
        assert_eq!(
            digits(1234),
            vec![Digit::d1(), Digit::d2(), Digit::d3(), Digit::d4()]
        )
    }
}

#[cfg(test)]
mod new {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn width_zero() {
        assert_eq!(
            FixedNatural::<0>::new(Natural::new(NonZeroDigit::D1, vec![])),
            None
        )
    }

    #[test]
    fn overflow() {
        assert_eq!(
            FixedNatural::<4>::new(Natural::new(
                NonZeroDigit::D1,
                vec![Digit::d0(), Digit::d1(), Digit::d2(), Digit::d3()]
            )),
            None
        )
    }

    #[test]
    fn underflow() {
        assert_eq!(
            FixedNatural::<4>::new(Natural::new(
                NonZeroDigit::D1,
                vec![Digit::d0(), Digit::d1()]
            )),
            Some(FixedNatural(Natural {
                head: NonZeroDigit::D1,
                tail: vec![Digit::d0(), Digit::d1()]
            }))
        )
    }

    #[test]
    fn at_flow() {
        assert_eq!(
            FixedNatural::<4>::new(Natural::new(
                NonZeroDigit::D1,
                vec![Digit::d2(), Digit::d3(), Digit::d4()]
            )),
            Some(FixedNatural(Natural {
                head: NonZeroDigit::D1,
                tail: vec![Digit::d2(), Digit::d3(), Digit::d4()]
            }))
        )
    }
}

#[cfg(test)]
mod from_int {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn overflow() {
        assert_eq!(FixedNatural::<3>::from_int(12345), None)
    }

    #[test]
    fn zero() {
        assert_eq!(FixedNatural::<3>::from_int(0), None)
    }

    #[test]
    fn underflow() {
        assert_eq!(
            FixedNatural::<4>::from_int(123),
            Some(FixedNatural(Natural::new(
                NonZeroDigit::D1,
                vec![Digit::d2(), Digit::d3()]
            )))
        )
    }

    #[test]
    fn at_flow() {
        assert_eq!(
            FixedNatural::<4>::from_int(1234),
            Some(FixedNatural(Natural::new(
                NonZeroDigit::D1,
                vec![Digit::d2(), Digit::d3(), Digit::d4()]
            )))
        )
    }
}
