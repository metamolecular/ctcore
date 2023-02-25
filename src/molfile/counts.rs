use crate::primitive::FixedCount;

use super::{ChiralFlag, Version};

#[derive(Debug, PartialEq)]
pub struct Counts {
    pub atoms: FixedCount<3>,
    pub bonds: FixedCount<3>,
    pub atom_lists: FixedCount<3>,
    pub chiral: ChiralFlag,
    pub version: Version,
}
