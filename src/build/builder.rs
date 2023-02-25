use crate::text::Printable;

use super::{Error, Target};

pub trait Builder: Sized {
    type Product;

    fn push(
        self,
        printable: Printable,
    ) -> Result<Target<Self::Product, Self>, Error>;

    fn done(self) -> Option<Self::Product>;
}
