use crate::{
    build::{Builder, Target},
    text::Printable,
};

#[derive(Debug, PartialEq)]
pub struct Sequence<const A: usize>(Vec<Printable>);

impl<const A: usize> Sequence<A> {
    pub fn start() -> impl Builder<Product = Sequence<A>> {
        SequenceBuilder::<A>(Vec::new())
    }
}

struct SequenceBuilder<const A: usize>(Vec<Printable>);

impl<const A: usize> Builder for SequenceBuilder<A> {
    type Product = Sequence<A>;

    fn push(
        mut self,
        printable: Printable,
    ) -> Result<Target<Self::Product, Self>, crate::build::Error> {
        self.0.push(printable);

        Ok(if self.0.len() == A {
            Target::Product(Sequence(self.0))
        } else {
            Target::Builder(self)
        })
    }

    fn done(self) -> Option<Self::Product> {
        if self.0.len() == A {
            Some(Sequence(self.0))
        } else {
            None
        }
    }
}
