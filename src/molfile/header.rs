use crate::primitive::Line;

use super::{Counts, MoleculeName, Parameters};

#[derive(Debug, PartialEq)]
pub struct Header {
    pub molecule_name: MoleculeName,
    pub parameters: Option<Parameters>,
    pub comment: Line<80>,
    pub counts: Counts,
}
