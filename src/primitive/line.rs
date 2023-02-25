use crate::{
    build::{Builder, Target},
    text::Printable,
};

#[derive(Debug, PartialEq)]
pub struct Line<const L: usize>(Vec<Printable>);

impl<const L: usize> Line<L> {
    pub fn start() -> impl Builder<Product = Line<L>> {
        LineBuilder::<L>(Vec::new())
    }

    pub fn from_str(str: &str) -> Option<Self> {
        let mut builder = Self::start();

        for byte in str.bytes() {
            builder = match builder.push(Printable::from_byte(byte)?) {
                Ok(Target::Builder(builder)) => builder,
                Ok(Target::Product(product)) => return Some(product),
                Err(_) => return None,
            }
        }

        builder.done()
    }
}

struct LineBuilder<const L: usize>(Vec<Printable>);

impl<const L: usize> Builder for LineBuilder<L> {
    type Product = Line<L>;

    fn push(
        mut self,
        printable: Printable,
    ) -> Result<crate::build::Target<Self::Product, Self>, crate::build::Error>
    {
        self.0.push(printable);

        Ok(if self.0.len() == L {
            Target::Product(Line(self.0))
        } else {
            Target::Builder(self)
        })
    }

    fn done(self) -> Option<Self::Product> {
        Some(Line(self.0))
    }
}
