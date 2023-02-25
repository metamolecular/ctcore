use super::Printable;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NonZeroDigit {
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
}

impl NonZeroDigit {
    pub fn from_printable(printable: Printable) -> Option<Self> {
        match printable {
            Printable::D1 => Some(NonZeroDigit::D1),
            Printable::D2 => Some(NonZeroDigit::D2),
            Printable::D3 => Some(NonZeroDigit::D3),
            Printable::D4 => Some(NonZeroDigit::D4),
            Printable::D5 => Some(NonZeroDigit::D5),
            Printable::D6 => Some(NonZeroDigit::D6),
            Printable::D7 => Some(NonZeroDigit::D7),
            Printable::D8 => Some(NonZeroDigit::D8),
            Printable::D9 => Some(NonZeroDigit::D9),
            _ => None,
        }
    }
}
