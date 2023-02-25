use crate::primitive::{FixedInteger, FixedReal, Sequence};

#[derive(Debug, PartialEq)]
pub struct Parameters {
    pub user_initials: Sequence<2>,
    pub program_name: Sequence<8>,
    pub timestamp: Sequence<10>,
    pub dimensional_codes: Sequence<2>,
    pub major_scaling: FixedInteger<2>,
    pub minor_scaling: FixedReal<4, 5>,
    pub energy: FixedReal<6, 5>,
    pub registry_number: FixedInteger<6>,
}
