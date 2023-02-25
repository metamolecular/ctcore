use crate::build::{Builder, Error, Target};
use crate::text::{Digit, Printable};

use super::Natural;

#[derive(Debug, PartialEq)]
pub enum FixedCount<const I: usize> {
    Zero,
    Natural(Natural),
}

impl<const I: usize> FixedCount<I> {
    pub fn start() -> impl Builder<Product = FixedCount<I>> {
        FixedCountBuilder::<I>::Pad(0)
    }

    pub fn from_int(int: u32) -> Option<Self> {
        if int == 0 {
            Some(Self::Zero)
        } else {
            let natural = Natural::from_int(int as u32)?;

            if natural.len() > I {
                None
            } else {
                Some(Self::Natural(natural))
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum FixedCountBuilder<const I: usize> {
    Pad(usize),
    Count(usize, Natural),
}

impl<const I: usize> Builder for FixedCountBuilder<I> {
    type Product = FixedCount<I>;

    fn push(
        self,
        printable: Printable,
    ) -> Result<Target<Self::Product, Self>, Error> {
        match self {
            Self::Pad(padding) => match printable {
                Printable::Space => {
                    if padding + 1 == I {
                        Err(Error::digit())
                    } else {
                        Ok(Target::Builder(FixedCountBuilder::Pad(padding + 1)))
                    }
                }
                printable => match Digit::from_printable(printable) {
                    Some(Digit::Zero) => {
                        if padding + 1 == I {
                            Ok(Target::Product(FixedCount::Zero))
                        } else {
                            Err(Error::non_zero_digit())
                        }
                    }
                    Some(Digit::NonZero(non_zero)) => {
                        if padding + 1 == I {
                            Ok(Target::Product(FixedCount::Natural(
                                Natural::new(non_zero),
                            )))
                        } else {
                            Ok(Target::Builder(FixedCountBuilder::Count(
                                padding,
                                Natural::new(non_zero),
                            )))
                        }
                    }
                    None => Err(Error::digit()),
                },
            },
            Self::Count(padding, mut natural) => {
                match Digit::from_printable(printable) {
                    Some(digit) => {
                        natural.push(digit);

                        if padding + natural.len() == I {
                            Ok(Target::Product(FixedCount::Natural(natural)))
                        } else {
                            Ok(Target::Builder(FixedCountBuilder::Count(
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
            Self::Pad(_) => None,
            Self::Count(padding, natural) => {
                if padding + natural.len() == I {
                    Some(FixedCount::Natural(natural))
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod from_int {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn zero() {
        assert_eq!(FixedCount::<1>::from_int(0), Some(FixedCount::Zero))
    }

    #[test]
    fn non_zero_in_bounds() {
        assert_eq!(
            FixedCount::<2>::from_int(42),
            Some(FixedCount::Natural(Natural::from_int(42).unwrap()))
        )
    }

    #[test]
    fn non_zero_out_of_bounds() {
        assert_eq!(FixedCount::<1>::from_int(42), None)
    }
}

#[cfg(test)]
mod builder_push {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn pad_limit_space() {
        let builder = FixedCountBuilder::<1>::Pad(0);

        assert_eq!(builder.push(Printable::Space), Err(Error::digit()))
    }

    #[test]
    fn pad_not_limit_space() {
        let builder = FixedCountBuilder::<2>::Pad(0);

        assert_eq!(
            builder.push(Printable::Space),
            Ok(Target::Builder(FixedCountBuilder::Pad(1)))
        )
    }

    #[test]
    fn pad_limit_zero() {
        let builder = FixedCountBuilder::<1>::Pad(0);

        assert_eq!(
            builder.push(Printable::D0),
            Ok(Target::Product(FixedCount::Zero))
        )
    }

    #[test]
    fn pad_not_limit_zero() {
        let builder = FixedCountBuilder::<2>::Pad(0);

        assert_eq!(builder.push(Printable::D0), Err(Error::non_zero_digit()))
    }

    #[test]
    fn pad_limit_non_zero() {
        let builder = FixedCountBuilder::<1>::Pad(0);

        assert_eq!(
            builder.push(Printable::D1),
            Ok(Target::Product(FixedCount::Natural(
                Natural::from_int(1).unwrap()
            )))
        )
    }

    #[test]
    fn pad_not_limit_non_zero() {
        let builder = FixedCountBuilder::<3>::Pad(1);

        assert_eq!(
            builder.push(Printable::D1),
            Ok(Target::Builder(FixedCountBuilder::Count(
                1,
                Natural::from_int(1).unwrap()
            )))
        )
    }

    #[test]
    fn count_limit_space() {
        let builder =
            FixedCountBuilder::<2>::Count(0, Natural::from_int(1).unwrap());

        assert_eq!(builder.push(Printable::Space), Err(Error::digit()))
    }

    #[test]
    fn count_limit_digit() {
        let builder =
            FixedCountBuilder::<2>::Count(0, Natural::from_int(4).unwrap());

        assert_eq!(
            builder.push(Printable::D2),
            Ok(Target::Product(FixedCount::from_int(42).unwrap()))
        )
    }

    #[test]
    fn count_not_limit_digit() {
        let builder =
            FixedCountBuilder::<3>::Count(0, Natural::from_int(4).unwrap());

        assert_eq!(
            builder.push(Printable::D2),
            Ok(Target::Builder(FixedCountBuilder::Count(
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
    fn pad() {
        let builder = FixedCountBuilder::<3>::Pad(2);

        assert_eq!(builder.done(), None)
    }

    #[test]
    fn count_not_limit() {
        let builder =
            FixedCountBuilder::<3>::Count(1, Natural::from_int(4).unwrap());

        assert_eq!(builder.done(), None)
    }

    #[test]
    fn count_limit() {
        let builder =
            FixedCountBuilder::<3>::Count(1, Natural::from_int(42).unwrap());

        assert_eq!(builder.done(), Some(FixedCount::from_int(42).unwrap()))
    }
}
