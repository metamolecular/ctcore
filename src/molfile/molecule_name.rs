use crate::{
    build::{Builder, Error, Target},
    text::Printable,
};

#[derive(Debug, PartialEq)]
pub struct MoleculeName(Vec<Printable>);

impl MoleculeName {
    pub fn start() -> impl Builder<Product = MoleculeName> {
        MoleculeNameBuilder(Vec::new())
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

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, printable: Printable) {
        self.0.push(printable)
    }
}

pub struct MoleculeNameBuilder(pub Vec<Printable>);

impl Builder for MoleculeNameBuilder {
    type Product = MoleculeName;

    fn push(
        mut self,
        printable: Printable,
    ) -> Result<Target<Self::Product, Self>, Error> {
        self.0.push(printable);

        Ok(if self.0.len() == 80 {
            Target::Product(MoleculeName(self.0))
        } else {
            Target::Builder(self)
        })
    }

    fn done(self) -> Option<Self::Product> {
        Some(MoleculeName(self.0))
    }
}
