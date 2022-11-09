use super::{Digit, FixedNatural, Natural, NonZeroDigit};

#[derive(Debug, PartialEq, Clone)]
pub struct FortranInt<const I: usize> {
    kind: Kind<I>,
}

#[derive(Debug, PartialEq, Clone)]
enum Kind<const I: usize> {
    Positive(FixedNatural<I>),
    Negative(FixedNatural<I>),
    Zero,
}

impl<const I: usize> FortranInt<I> {
    pub fn positive(natural: FixedNatural<I>) -> Option<Self> {
        Some(FortranInt {
            kind: Kind::Positive(natural),
        })
    }

    pub fn negative(natural: FixedNatural<I>) -> Option<Self> {
        if I < 2 {
            None
        } else {
            Some(FortranInt {
                kind: Kind::Negative(natural),
            })
        }
    }

    pub fn zero() -> Self {
        Self { kind: Kind::Zero }
    }

    pub fn from_int(int: i32) -> Option<Self> {
        if int == 0 {
            return Some(Self::zero());
        }

        let natural = FixedNatural::from_int(int.abs() as u32)?;

        if int > 0 {
            Self::positive(natural)
        } else {
            Self::negative(natural)
        }
    }

    pub fn from_digit(digit: Digit) -> Option<Self> {
        if I == 0 {
            return None;
        }

        match digit.to_non_zero() {
            Some(non_zero) => {
                Self::positive(FixedNatural::from_digit(non_zero)?)
            }
            None => Some(Self::zero()),
        }
    }

    pub fn from_postitive_parts(
        head: NonZeroDigit,
        tail: Vec<Digit>,
    ) -> Option<Self> {
        Some(Self {
            kind: Kind::Positive(FixedNatural::new(Natural { head, tail })?),
        })
    }

    pub fn from_negative_parts(
        head: NonZeroDigit,
        tail: Vec<Digit>,
    ) -> Option<Self> {
        if I < 2 {
            None
        } else {
            Some(Self {
                kind: Kind::Negative(FixedNatural::new(Natural {
                    head,
                    tail,
                })?),
            })
        }
    }
}

#[cfg(test)]
mod positive {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn nonzero() {
        assert_eq!(
            FortranInt::<1>::positive(FixedNatural::from_int(1).unwrap()),
            Some(FortranInt {
                kind: Kind::Positive(FixedNatural::from_int(1).unwrap())
            })
        )
    }
}

#[cfg(test)]
mod negative {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn width_one() {
        assert_eq!(
            FortranInt::<1>::negative(FixedNatural::from_int(1).unwrap()),
            None
        )
    }

    #[test]
    fn at_flow() {
        assert_eq!(
            FortranInt::<3>::negative(FixedNatural::from_int(42).unwrap()),
            Some(FortranInt {
                kind: Kind::Negative(FixedNatural::from_int(42).unwrap())
            })
        )
    }
}

#[cfg(test)]
mod from_int {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn positive_overflow() {
        assert_eq!(FortranInt::<3>::from_int(12345), None)
    }

    #[test]
    fn negative_overflow() {
        assert_eq!(FortranInt::<3>::from_int(-1234), None)
    }

    #[test]
    fn zero() {
        assert_eq!(
            FortranInt::<3>::from_int(0),
            Some(FortranInt { kind: Kind::Zero })
        )
    }

    #[test]
    fn negative_undeflow() {
        assert_eq!(
            FortranInt::<4>::from_int(-12),
            Some(FortranInt {
                kind: Kind::Negative(FixedNatural::from_int(12).unwrap())
            })
        )
    }

    #[test]
    fn negative_to_fit() {
        assert_eq!(
            FortranInt::<4>::from_int(-123),
            Some(FortranInt {
                kind: Kind::Negative(FixedNatural::from_int(123).unwrap())
            })
        )
    }

    #[test]
    fn positive_underflow() {
        assert_eq!(
            FortranInt::<4>::from_int(123),
            Some(FortranInt {
                kind: Kind::Positive(FixedNatural::from_int(123).unwrap())
            })
        )
    }

    #[test]
    fn positive_to_fit() {
        assert_eq!(
            FortranInt::<4>::from_int(1234),
            Some(FortranInt {
                kind: Kind::Positive(FixedNatural::from_int(1234).unwrap())
            })
        )
    }
}
