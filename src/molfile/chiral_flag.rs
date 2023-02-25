use crate::{
    build::{Builder, Error, Target},
    text::Printable,
};

#[derive(Debug, PartialEq)]
pub enum ChiralFlag {
    Chiral,
    NotChiral,
}

impl ChiralFlag {
    pub fn start() -> impl Builder<Product = ChiralFlag> {
        ChiralFlagBuilder(0)
    }
}

#[derive(Debug, PartialEq)]
struct ChiralFlagBuilder(usize);

impl Builder for ChiralFlagBuilder {
    type Product = ChiralFlag;

    fn push(
        self,
        printable: Printable,
    ) -> Result<Target<Self::Product, Self>, Error> {
        if self.0 < 2 {
            if printable == Printable::Space {
                Ok(Target::Builder(Self(self.0 + 1)))
            } else {
                Err(Error::space())
            }
        } else {
            match printable {
                Printable::D0 => Ok(Target::Product(ChiralFlag::NotChiral)),
                Printable::D1 => Ok(Target::Product(ChiralFlag::Chiral)),
                _ => Err(Error::flag()),
            }
        }
    }

    fn done(self) -> Option<Self::Product> {
        None
    }
}

#[cfg(test)]
mod builder_push {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn not_limit_zero() {
        let builder = ChiralFlagBuilder(0);

        assert_eq!(builder.push(Printable::D0), Err(Error::space()))
    }

    #[test]
    fn not_limit_space() {
        let builder = ChiralFlagBuilder(1);

        assert_eq!(
            builder.push(Printable::Space),
            Ok(Target::Builder(ChiralFlagBuilder(2)))
        )
    }

    #[test]
    fn limit_space() {
        let builder = ChiralFlagBuilder(2);

        assert_eq!(builder.push(Printable::Space), Err(Error::flag()))
    }

    #[test]
    fn limit_zero() {
        let builder = ChiralFlagBuilder(2);

        assert_eq!(
            builder.push(Printable::D0),
            Ok(Target::Product(ChiralFlag::NotChiral))
        )
    }

    #[test]
    fn limit_one() {
        let builder = ChiralFlagBuilder(2);

        assert_eq!(
            builder.push(Printable::D1),
            Ok(Target::Product(ChiralFlag::Chiral))
        )
    }
}

#[cfg(test)]
mod buider_done {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn not_done() {
        let builder = ChiralFlagBuilder(2);

        assert_eq!(builder.done(), None)
    }
}
