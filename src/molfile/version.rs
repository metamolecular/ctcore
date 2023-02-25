use crate::{
    build::{Builder, Error, Target},
    text::Printable,
};

#[derive(Debug, PartialEq)]
pub enum Version {
    V2,
    V3,
}

impl Version {
    pub fn start() -> impl Builder<Product = Version> {
        VersionBuilder::Pad
    }
}

#[derive(Debug, PartialEq)]
enum VersionBuilder {
    Pad,
    V,
    Digit,
    V2(usize),
    V3(usize),
}

impl Builder for VersionBuilder {
    type Product = Version;

    fn push(
        self,
        printable: Printable,
    ) -> Result<Target<Self::Product, Self>, Error> {
        match self {
            Self::Pad => {
                if printable == Printable::Space {
                    Ok(Target::Builder(VersionBuilder::V))
                } else {
                    Err(Error::space())
                }
            }
            Self::V => {
                if printable == Printable::V {
                    Ok(Target::Builder(VersionBuilder::Digit))
                } else {
                    Err(Error::v())
                }
            }
            Self::Digit => match printable {
                Printable::D2 => Ok(Target::Builder(VersionBuilder::V2(0))),
                Printable::D3 => Ok(Target::Builder(VersionBuilder::V3(0))),
                _ => Err(Error::two_or_three()),
            },
            Self::V2(zeroes) => {
                if printable == Printable::D0 {
                    if zeroes == 2 {
                        Ok(Target::Product(Version::V2))
                    } else {
                        Ok(Target::Builder(VersionBuilder::V2(zeroes + 1)))
                    }
                } else {
                    Err(Error::zero())
                }
            }
            Self::V3(zeroes) => {
                if printable == Printable::D0 {
                    if zeroes == 2 {
                        Ok(Target::Product(Version::V3))
                    } else {
                        Ok(Target::Builder(VersionBuilder::V3(zeroes + 1)))
                    }
                } else {
                    Err(Error::zero())
                }
            }
        }
    }

    fn done(self) -> Option<Self::Product> {
        None
    }
}

#[cfg(test)]
mod builder_push {
    use crate::build::Target;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn pad_non_space() {
        let builder = VersionBuilder::Pad;

        assert_eq!(builder.push(Printable::A), Err(Error::space()))
    }

    #[test]
    fn pad_space() {
        let builder = VersionBuilder::Pad;

        assert_eq!(
            builder.push(Printable::Space),
            Ok(Target::Builder(VersionBuilder::V))
        )
    }

    #[test]
    fn v_letter() {
        let builder = VersionBuilder::V;

        assert_eq!(builder.push(Printable::A), Err(Error::v()))
    }

    #[test]
    fn v_v() {
        let builder = VersionBuilder::V;

        assert_eq!(
            builder.push(Printable::V),
            Ok(Target::Builder(VersionBuilder::Digit))
        )
    }

    #[test]
    fn digit_two() {
        let builder = VersionBuilder::Digit;

        assert_eq!(
            builder.push(Printable::D2),
            Ok(Target::Builder(VersionBuilder::V2(0)))
        )
    }

    #[test]
    fn digit_three() {
        let builder = VersionBuilder::Digit;

        assert_eq!(
            builder.push(Printable::D3),
            Ok(Target::Builder(VersionBuilder::V3(0)))
        )
    }

    #[test]
    fn digit_letter() {
        let builder = VersionBuilder::Digit;

        assert_eq!(builder.push(Printable::X), Err(Error::two_or_three()))
    }

    #[test]
    fn v2_non_zero() {
        let builder = VersionBuilder::V2(0);

        assert_eq!(
            builder.push(Printable::D1),
            Err(Error::Character(vec![Printable::D0]))
        )
    }

    #[test]
    fn v2_first_zero() {
        let builder = VersionBuilder::V2(0);

        assert_eq!(
            builder.push(Printable::D0),
            Ok(Target::Builder(VersionBuilder::V2(1)))
        )
    }

    #[test]
    fn v2_last_zero() {
        let builder = VersionBuilder::V2(2);

        assert_eq!(
            builder.push(Printable::D0),
            Ok(Target::Product(Version::V2))
        )
    }

    #[test]
    fn v3_non_zero() {
        let builder = VersionBuilder::V3(0);

        assert_eq!(
            builder.push(Printable::D1),
            Err(Error::Character(vec![Printable::D0]))
        )
    }

    #[test]
    fn v3_zero() {
        let builder = VersionBuilder::V3(0);

        assert_eq!(
            builder.push(Printable::D0),
            Ok(Target::Builder(VersionBuilder::V3(1)))
        )
    }

    #[test]
    fn v3_last_zero() {
        let builder = VersionBuilder::V3(2);

        assert_eq!(
            builder.push(Printable::D0),
            Ok(Target::Product(Version::V3))
        )
    }
}

#[cfg(test)]
mod builder_done {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn not_done() {
        let builder = VersionBuilder::V;

        assert_eq!(builder.done(), None)
    }
}
