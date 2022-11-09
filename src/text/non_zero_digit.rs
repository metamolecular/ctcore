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
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x31 => Some(Self::D1),
            0x32 => Some(Self::D2),
            0x33 => Some(Self::D3),
            0x34 => Some(Self::D4),
            0x35 => Some(Self::D5),
            0x36 => Some(Self::D6),
            0x37 => Some(Self::D7),
            0x38 => Some(Self::D8),
            0x39 => Some(Self::D9),
            _ => None,
        }
    }
}
