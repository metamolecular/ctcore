use crate::{
    build::{Builder, Error, Target},
    text::{Digit, NonZeroDigit, Printable},
};

use super::Natural;

#[derive(Debug, PartialEq)]
pub enum FixedInteger<const I: usize> {
    Zero,
    Positive(Natural),
    Negative(Natural),
}

impl<const I: usize> FixedInteger<I> {
    pub fn start() -> impl Builder<Product = FixedInteger<I>> {
        FixedIntegerBuilder::<I>::Pad(0)
    }

    pub fn from_int(int: i32) -> Option<Self> {
        if int < 0 {
            let natural = Natural::from_int(int.abs() as u32)?;

            if 1 + natural.len() > I {
                None
            } else {
                Some(Self::Negative(natural))
            }
        } else if int == 0 {
            Some(Self::Zero)
        } else {
            let natural = Natural::from_int(int as u32)?;

            if natural.len() > I {
                None
            } else {
                Some(Self::Positive(natural))
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FixedIntegerBuilder<const I: usize> {
    Pad(usize),
    Minus(usize),
    Negative(usize, Natural),
    Positive(usize, Natural),
}

impl<const I: usize> FixedIntegerBuilder<I> {
    pub fn new() -> Self {
        Self::Pad(0)
    }
}

impl<const I: usize> Builder for FixedIntegerBuilder<I> {
    type Product = FixedInteger<I>;

    fn push(
        self,
        printable: Printable,
    ) -> Result<Target<Self::Product, Self>, Error> {
        match self {
            Self::Pad(padding) => {
                if padding + 1 == I {
                    match printable {
                        Printable::D0 => {
                            Ok(Target::Product(FixedInteger::Zero))
                        }
                        printable => {
                            match NonZeroDigit::from_printable(printable) {
                                Some(non_zero) => {
                                    Ok(Target::Product(FixedInteger::Positive(
                                        Natural::new(non_zero),
                                    )))
                                }
                                None => Err(Error::digit()),
                            }
                        }
                    }
                } else {
                    match printable {
                        Printable::Space => {
                            Ok(Target::Builder(Self::Pad(padding + 1)))
                        }
                        Printable::Minus => {
                            Ok(Target::Builder(Self::Minus(padding)))
                        }
                        printable => {
                            match NonZeroDigit::from_printable(printable) {
                                Some(non_zero) => Ok(Target::Builder(
                                    FixedIntegerBuilder::Positive(
                                        padding,
                                        Natural::new(non_zero),
                                    ),
                                )),
                                None => Err(Error::integer_leading()),
                            }
                        }
                    }
                }
            }
            Self::Minus(padding) => {
                match NonZeroDigit::from_printable(printable) {
                    Some(non_zero) => {
                        let natural = Natural::new(non_zero);

                        if padding + 2 == I {
                            Ok(Target::Product(FixedInteger::Negative(natural)))
                        } else {
                            Ok(Target::Builder(Self::Negative(
                                padding, natural,
                            )))
                        }
                    }
                    None => Err(Error::non_zero_digit()),
                }
            }
            Self::Negative(padding, mut natural) => {
                match Digit::from_printable(printable) {
                    Some(digit) => {
                        natural.push(digit);
                        if 1 + padding + natural.len() == I {
                            Ok(Target::Product(FixedInteger::Negative(natural)))
                        } else {
                            Ok(Target::Builder(Self::Negative(
                                padding, natural,
                            )))
                        }
                    }
                    None => Err(Error::digit()),
                }
            }
            Self::Positive(padding, mut natural) => {
                match Digit::from_printable(printable) {
                    Some(digit) => {
                        natural.push(digit);

                        if padding + natural.len() == I {
                            Ok(Target::Product(FixedInteger::Positive(natural)))
                        } else {
                            Ok(Target::Builder(Self::Positive(
                                padding, natural,
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
            Self::Negative(padding, natural) => {
                if padding + 1 + natural.len() == I {
                    Some(FixedInteger::Negative(natural))
                } else {
                    None
                }
            }
            Self::Positive(padding, natural) => {
                if padding + natural.len() == I {
                    Some(FixedInteger::Positive(natural))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod from_int {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn zero() {
        assert_eq!(FixedInteger::<2>::from_int(0), Some(FixedInteger::Zero))
    }

    #[test]
    fn negative_over_limit() {
        assert_eq!(FixedInteger::<2>::from_int(-42), None)
    }

    #[test]
    fn negative_at_limit() {
        assert_eq!(
            FixedInteger::<3>::from_int(-42),
            Some(FixedInteger::Negative(Natural::from_int(42).unwrap()))
        )
    }

    #[test]
    fn positive_at_limit() {
        assert_eq!(
            FixedInteger::<2>::from_int(42),
            Some(FixedInteger::Positive(Natural::from_int(42).unwrap()))
        )
    }

    #[test]
    fn positive_over_limit() {
        assert_eq!(FixedInteger::<1>::from_int(42), None)
    }
}

#[cfg(test)]
mod builder_push {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn pad_limit_space() {
        let builder = FixedIntegerBuilder::<1>::Pad(0);

        assert_eq!(builder.push(Printable::Space), Err(Error::digit()))
    }

    #[test]
    fn pad_limit_zero() {
        let builder = FixedIntegerBuilder::<1>::Pad(0);

        assert_eq!(
            builder.push(Printable::D0),
            Ok(Target::Product(FixedInteger::<1>::Zero))
        )
    }

    #[test]
    fn pad_limit_non_zero() {
        let builder = FixedIntegerBuilder::<1>::Pad(0);

        assert_eq!(
            builder.push(Printable::D1),
            Ok(Target::Product(FixedInteger::<1>::Positive(Natural::new(
                NonZeroDigit::D1
            ))))
        )
    }

    #[test]
    fn pad_not_limit_space() {
        let builder = FixedIntegerBuilder::<2>::Pad(0);

        assert_eq!(
            builder.push(Printable::Space),
            Ok(Target::Builder(FixedIntegerBuilder::<2>::Pad(1)))
        )
    }

    #[test]
    fn pad_not_limit_minus() {
        let builder = FixedIntegerBuilder::<2>::Pad(0);

        assert_eq!(
            builder.push(Printable::Minus),
            Ok(Target::Builder(FixedIntegerBuilder::<2>::Minus(0)))
        )
    }

    #[test]
    fn pad_not_limit_zero() {
        let builder = FixedIntegerBuilder::<2>::Pad(0);

        assert_eq!(builder.push(Printable::D0), Err(Error::integer_leading()))
    }

    #[test]
    fn pad_not_limit_non_zero() {
        let builder = FixedIntegerBuilder::<2>::Pad(0);

        assert_eq!(
            builder.push(Printable::D1),
            Ok(Target::Builder(FixedIntegerBuilder::<2>::Positive(
                0,
                Natural::new(NonZeroDigit::D1)
            )))
        )
    }

    #[test]
    fn minus_minus() {
        let builder = FixedIntegerBuilder::<2>::Minus(0);

        assert_eq!(builder.push(Printable::Minus), Err(Error::non_zero_digit()))
    }

    #[test]
    fn minus_zero() {
        let builder = FixedIntegerBuilder::<2>::Minus(0);

        assert_eq!(builder.push(Printable::Minus), Err(Error::non_zero_digit()))
    }

    #[test]
    fn minus_not_limit_non_zero() {
        let builder = FixedIntegerBuilder::<3>::Minus(0);

        assert_eq!(
            builder.push(Printable::D1),
            Ok(Target::Builder(FixedIntegerBuilder::<3>::Negative(
                0,
                Natural::new(NonZeroDigit::D1)
            )))
        )
    }

    #[test]
    fn minus_limit_non_zero() {
        let builder = FixedIntegerBuilder::<2>::Minus(0);

        assert_eq!(
            builder.push(Printable::D1),
            Ok(Target::Product(FixedInteger::Negative(Natural::new(
                NonZeroDigit::D1
            ))))
        )
    }

    #[test]
    fn negative_non_digit() {
        let builder = FixedIntegerBuilder::<3>::Negative(
            0,
            Natural::new(NonZeroDigit::D1),
        );

        assert_eq!(builder.push(Printable::Space), Err(Error::digit()))
    }

    #[test]
    fn negative_limit_digit() {
        let builder = FixedIntegerBuilder::<3>::Negative(
            0,
            Natural::new(NonZeroDigit::D4),
        );

        assert_eq!(
            builder.push(Printable::D2),
            Ok(Target::Product(FixedInteger::Negative(
                Natural::from_int(42).unwrap()
            )))
        )
    }

    #[test]
    fn negative_not_limit_digit() {
        let builder = FixedIntegerBuilder::<4>::Negative(
            0,
            Natural::new(NonZeroDigit::D4),
        );

        assert_eq!(
            builder.push(Printable::D2),
            Ok(Target::Builder(FixedIntegerBuilder::Negative(
                0,
                Natural::from_int(42).unwrap()
            )))
        )
    }

    #[test]
    fn positive_non_digit() {
        let builder = FixedIntegerBuilder::<2>::Positive(
            0,
            Natural::new(NonZeroDigit::D4),
        );

        assert_eq!(builder.push(Printable::Space), Err(Error::digit()))
    }

    #[test]
    fn positive_limit_digit() {
        let builder = FixedIntegerBuilder::<2>::Positive(
            0,
            Natural::new(NonZeroDigit::D4),
        );

        assert_eq!(
            builder.push(Printable::D2),
            Ok(Target::Product(FixedInteger::Positive(
                Natural::from_int(42).unwrap()
            )))
        )
    }

    #[test]
    fn positive_not_limit_digit() {
        let builder = FixedIntegerBuilder::<3>::Positive(
            0,
            Natural::new(NonZeroDigit::D4),
        );

        assert_eq!(
            builder.push(Printable::D2),
            Ok(Target::Builder(FixedIntegerBuilder::Positive(
                0,
                Natural::from_int(42).unwrap()
            )))
        )
    }
}

#[cfg(test)]
mod builder_done {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn postitive_not_done() {
        let builder = FixedIntegerBuilder::<3>::Positive(
            0,
            Natural::new(NonZeroDigit::D4),
        );

        assert_eq!(builder.done(), None)
    }

    #[test]
    fn positive_done() {
        let builder = FixedIntegerBuilder::<1>::Positive(
            0,
            Natural::new(NonZeroDigit::D1),
        );

        assert_eq!(
            builder.done(),
            Some(FixedInteger::Positive(Natural::from_int(1).unwrap()))
        )
    }

    #[test]
    fn negative_not_done() {
        let builder = FixedIntegerBuilder::<3>::Negative(
            0,
            Natural::new(NonZeroDigit::D4),
        );

        assert_eq!(builder.done(), None)
    }

    #[test]
    fn negative_done() {
        let builder = FixedIntegerBuilder::<2>::Negative(
            0,
            Natural::new(NonZeroDigit::D1),
        );

        assert_eq!(
            builder.done(),
            Some(FixedInteger::Negative(Natural::from_int(1).unwrap()))
        )
    }

    #[test]
    fn positive_not_done() {
        let builder = FixedIntegerBuilder::<3>::Positive(
            0,
            Natural::new(NonZeroDigit::D4),
        );

        assert_eq!(builder.done(), None)
    }
}
