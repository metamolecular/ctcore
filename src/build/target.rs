use super::Builder;

#[derive(Debug, PartialEq)]
pub enum Target<P, B: Builder<Product = P>> {
    Builder(B),
    Product(P),
}
