use super::{Digit, FortranInt};

#[derive(Debug, PartialEq, Clone)]
pub struct FortranFloat<const I: usize, const F: usize> {
    integer_part: FortranInt<I>,
    fractional_part: Vec<Digit>,
}

impl<const I: usize, const F: usize> FortranFloat<I, F> {
    pub fn new(
        integer_part: FortranInt<I>,
        fractional_part: Vec<Digit>,
    ) -> Option<Self> {
        if fractional_part.len() > F {
            None
        } else {
            Some(Self {
                integer_part,
                fractional_part,
            })
        }
    }

    pub fn from_float(float: f64) -> Option<Self> {
        Some(FortranFloat {
            integer_part: FortranInt::from_int(float.trunc() as i32)?,
            fractional_part: fractional_digits(float, F),
        })
    }
}

fn fractional_digits(float: f64, limit: usize) -> Vec<Digit> {
    let mut value = float.abs().fract();

    for _ in 0..limit {
        value *= 10.
    }

    let mut value = value.round() as u32;
    let mut result = Vec::new();

    for _ in 0..limit {
        result.push(Digit::from_int(value % 10).unwrap());

        value /= 10;

        if result.len() == limit {
            break;
        }
    }

    result.reverse();

    result
}

#[cfg(test)]
mod fractional_digits {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn zero() {
        assert_eq!(fractional_digits(0., 2), vec![Digit::d0(), Digit::d0()])
    }

    #[test]
    fn rounding_up() {
        assert_eq!(
            fractional_digits(0.12999, 2),
            vec![Digit::d1(), Digit::d3()]
        )
    }

    #[test]
    fn rounding_down() {
        assert_eq!(fractional_digits(0.1211, 2), vec![Digit::d1(), Digit::d2()])
    }

    #[test]
    fn rounding_negative() {
        assert_eq!(
            fractional_digits(-0.12999, 2),
            vec![Digit::d1(), Digit::d3()]
        )
    }

    #[test]
    fn no_rounding() {
        assert_eq!(
            fractional_digits(0.1234, 4),
            vec![Digit::d1(), Digit::d2(), Digit::d3(), Digit::d4()]
        )
    }

    #[test]
    fn trailing_zero() {
        assert_eq!(
            fractional_digits(0.1234, 5),
            vec![
                Digit::d1(),
                Digit::d2(),
                Digit::d3(),
                Digit::d4(),
                Digit::d0()
            ]
        )
    }
}

#[cfg(test)]
mod from_float {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn zero() {
        assert_eq!(
            FortranFloat::<2, 2>::from_float(0.),
            Some(FortranFloat {
                integer_part: FortranInt::zero(),
                fractional_part: vec![Digit::d0(), Digit::d0()]
            })
        )
    }

    #[test]
    fn fraction() {
        assert_eq!(
            FortranFloat::<1, 3>::from_float(0.14),
            Some(FortranFloat {
                integer_part: FortranInt::zero(),
                fractional_part: vec![Digit::d1(), Digit::d4(), Digit::d0()]
            })
        )
    }

    #[test]
    fn pi_underflow_left_right() {
        assert_eq!(
            FortranFloat::<2, 3>::from_float(3.14),
            Some(FortranFloat {
                // integer_part: FortranInt::positive(NonZeroDigit::D3, vec![])
                // .unwrap(),
                integer_part: FortranInt::from_int(3).unwrap(),
                fractional_part: vec![Digit::d1(), Digit::d4(), Digit::d0()]
            })
        )
    }

    #[test]
    fn pi_underflow_left_right_minus() {
        assert_eq!(
            FortranFloat::<2, 3>::from_float(-3.14),
            Some(FortranFloat {
                // integer_part: FortranInt::negative(NonZeroDigit::D3, vec![])
                //     .unwrap(),
                integer_part: FortranInt::from_int(-3).unwrap(),
                fractional_part: vec![Digit::d1(), Digit::d4(), Digit::d0()]
            })
        )
    }
}
