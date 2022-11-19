use super::{Digit, Natural};

#[derive(Debug, PartialEq)]
pub enum Count {
    Positive(Natural),
    Zero,
}

impl Count {
    pub fn from_int(int: u32) -> Self {
        let digits = digits(int);

        match Natural::from_digits(digits) {
            Some(natural) => Count::Positive(natural),
            None => Count::Zero,
        }
    }
}

fn digits(mut int: u32) -> Vec<Digit> {
    let mut result = Vec::new();

    while int > 0 {
        result.push(Digit::from_int(int % 10).unwrap());

        int /= 10;
    }

    result.reverse();

    result
}

#[cfg(test)]
mod from_int {
    use crate::text::NonZeroDigit;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn zero() {
        assert_eq!(Count::from_int(0), Count::Zero)
    }

    #[test]
    fn non_zero() {
        assert_eq!(
            Count::from_int(123),
            Count::Positive(Natural {
                head: NonZeroDigit::D1,
                tail: vec![Digit::d2(), Digit::d3()]
            })
        )
    }
}
