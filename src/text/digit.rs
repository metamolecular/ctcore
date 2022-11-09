use super::NonZeroDigit;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Digit {
    Zero,
    NonZero(NonZeroDigit),
}

impl Digit {
    pub fn d0() -> Self {
        Self::Zero
    }

    pub fn d1() -> Self {
        Self::NonZero(NonZeroDigit::D1)
    }

    pub fn d2() -> Self {
        Self::NonZero(NonZeroDigit::D2)
    }

    pub fn d3() -> Self {
        Self::NonZero(NonZeroDigit::D3)
    }

    pub fn d4() -> Self {
        Self::NonZero(NonZeroDigit::D4)
    }

    pub fn d5() -> Self {
        Self::NonZero(NonZeroDigit::D5)
    }

    pub fn d6() -> Self {
        Self::NonZero(NonZeroDigit::D6)
    }

    pub fn d7() -> Self {
        Self::NonZero(NonZeroDigit::D7)
    }

    pub fn d8() -> Self {
        Self::NonZero(NonZeroDigit::D8)
    }

    pub fn d9() -> Self {
        Self::NonZero(NonZeroDigit::D9)
    }

    pub fn from_int(int: u32) -> Option<Self> {
        match int {
            0 => Some(Self::Zero),
            1 => Some(Self::NonZero(NonZeroDigit::D1)),
            2 => Some(Self::NonZero(NonZeroDigit::D2)),
            3 => Some(Self::NonZero(NonZeroDigit::D3)),
            4 => Some(Self::NonZero(NonZeroDigit::D4)),
            5 => Some(Self::NonZero(NonZeroDigit::D5)),
            6 => Some(Self::NonZero(NonZeroDigit::D6)),
            7 => Some(Self::NonZero(NonZeroDigit::D7)),
            8 => Some(Self::NonZero(NonZeroDigit::D8)),
            9 => Some(Self::NonZero(NonZeroDigit::D9)),
            _ => None,
        }
    }

    pub fn from_byte(byte: u8) -> Option<Self> {
        if byte == 0x30 {
            Some(Self::Zero)
        } else {
            match NonZeroDigit::from_byte(byte) {
                Some(non_zero) => Some(Self::NonZero(non_zero)),
                None => None,
            }
        }
    }

    pub fn to_non_zero(self) -> Option<NonZeroDigit> {
        match self {
            Self::Zero => None,
            Self::NonZero(non_zero) => Some(non_zero),
        }
    }

    pub fn is_zero(&self) -> bool {
        self == &Self::Zero
    }
}
