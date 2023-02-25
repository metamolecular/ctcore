use crate::{
    build::{Builder, Error, Target},
    text::{Digit, Printable},
};

use super::{fixed_integer::FixedIntegerBuilder, FixedInteger};

#[derive(Debug, PartialEq)]
pub struct FixedReal<const I: usize, const F: usize>(
    FixedInteger<I>,
    Vec<Digit>,
);

impl<const I: usize, const F: usize> FixedReal<I, F> {
    pub fn start() -> impl Builder<Product = FixedReal<I, F>> {
        FixedRealBuilder::IntegerPart(FixedIntegerBuilder::new())
    }
}

#[derive(Debug, PartialEq)]
enum FixedRealBuilder<const I: usize, const F: usize> {
    IntegerPart(FixedIntegerBuilder<I>),
    Decimal(FixedInteger<I>),
    FractionalPart(FixedInteger<I>, Vec<Digit>),
}

impl<const I: usize, const F: usize> Builder for FixedRealBuilder<I, F> {
    type Product = FixedReal<I, F>;

    fn push(
        self,
        printable: Printable,
    ) -> Result<Target<Self::Product, Self>, Error> {
        match self {
            Self::IntegerPart(builder) => match builder.push(printable)? {
                Target::Builder(builder) => {
                    Ok(Target::Builder(Self::IntegerPart(builder)))
                }
                Target::Product(fixed_integer) => {
                    Ok(Target::Builder(Self::Decimal(fixed_integer)))
                }
            },
            Self::Decimal(fixed_integer) => {
                if printable == Printable::Dot {
                    Ok(Target::Builder(Self::FractionalPart(
                        fixed_integer,
                        Vec::new(),
                    )))
                } else {
                    Err(Error::Character(vec![Printable::Dot]))
                }
            }
            Self::FractionalPart(fixed_integer, mut digits) => {
                match Digit::from_printable(printable) {
                    Some(digit) => {
                        digits.push(digit);

                        if digits.len() == F {
                            Ok(Target::Product(FixedReal(
                                fixed_integer,
                                digits,
                            )))
                        } else {
                            Ok(Target::Builder(Self::FractionalPart(
                                fixed_integer,
                                digits,
                            )))
                        }
                    }
                    None => Err(Error::digit()),
                }
            }
        }
    }

    fn done(self) -> Option<Self::Product> {
        match self {
            Self::FractionalPart(fixed_integer, digits) => {
                if digits.len() == F {
                    Some(FixedReal(fixed_integer, digits))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod builder_push {
    use crate::primitive::Natural;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn integer_part_not_limit_non_zero() {
        let builder = FixedRealBuilder::<2, 3>::IntegerPart(
            FixedIntegerBuilder::<2>::Pad(0),
        );

        assert_eq!(
            builder.push(Printable::D4),
            Ok(Target::Builder(FixedRealBuilder::IntegerPart(
                FixedIntegerBuilder::Positive(0, Natural::from_int(4).unwrap())
            )))
        )
    }

    #[test]
    fn integer_part_limit_digit() {
        let builder = FixedRealBuilder::<1, 3>::IntegerPart(
            FixedIntegerBuilder::<1>::Pad(0),
        );

        assert_eq!(
            builder.push(Printable::D4),
            Ok(Target::Builder(FixedRealBuilder::Decimal(
                FixedInteger::from_int(4).unwrap()
            )))
        )
    }

    #[test]
    fn decimal_non_dot() {
        let builder = FixedRealBuilder::<1, 3>::Decimal(
            FixedInteger::from_int(2).unwrap(),
        );

        assert_eq!(
            builder.push(Printable::Space),
            Err(Error::Character(vec![Printable::Dot]))
        )
    }

    #[test]
    fn decimal_dot() {
        let builder = FixedRealBuilder::<1, 3>::Decimal(
            FixedInteger::from_int(2).unwrap(),
        );

        assert_eq!(
            builder.push(Printable::Dot),
            Ok(Target::Builder(FixedRealBuilder::FractionalPart(
                FixedInteger::from_int(2).unwrap(),
                vec![]
            )))
        )
    }

    #[test]
    fn fractional_non_digit() {
        let builder = FixedRealBuilder::<1, 3>::FractionalPart(
            FixedInteger::from_int(2).unwrap(),
            vec![],
        );

        assert_eq!(builder.push(Printable::X), Err(Error::digit()))
    }

    #[test]
    fn fractional_limit_digit() {
        let builder = FixedRealBuilder::<1, 1>::FractionalPart(
            FixedInteger::from_int(4).unwrap(),
            vec![],
        );

        assert_eq!(
            builder.push(Printable::D2),
            Ok(Target::Product(FixedReal(
                FixedInteger::from_int(4).unwrap(),
                vec![Digit::d2()]
            )))
        )
    }

    #[test]
    fn fractional_not_limit_digit() {
        let builder = FixedRealBuilder::<1, 2>::FractionalPart(
            FixedInteger::from_int(4).unwrap(),
            vec![],
        );

        assert_eq!(
            builder.push(Printable::D2),
            Ok(Target::Builder(FixedRealBuilder::FractionalPart(
                FixedInteger::from_int(4).unwrap(),
                vec![Digit::d2()]
            )))
        )
    }
}

#[cfg(test)]
mod builder_done {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn fractional_not_done() {
        let builder = FixedRealBuilder::<1, 2>::FractionalPart(
            FixedInteger::from_int(4).unwrap(),
            vec![],
        );

        assert_eq!(builder.done(), None)
    }

    #[test]
    fn fractional_done() {
        let builder = FixedRealBuilder::<1, 1>::FractionalPart(
            FixedInteger::from_int(4).unwrap(),
            vec![Digit::d2()],
        );

        assert_eq!(
            builder.done(),
            Some(FixedReal(
                FixedInteger::from_int(4).unwrap(),
                vec![Digit::d2()]
            ))
        )
    }

    #[test]
    fn non_fractional() {
        let builder = FixedRealBuilder::<2, 3>::IntegerPart(
            FixedIntegerBuilder::<2>::Pad(0),
        );

        assert_eq!(builder.done(), None)
    }
}
