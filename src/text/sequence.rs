use std::fmt;

use super::Character;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Sequence<const L: usize>(Vec<Character>);

impl<const L: usize> Sequence<L> {
    pub fn new(chars: Vec<Character>) -> Option<Self> {
        if chars.len() > L {
            None
        } else {
            Some(Self(chars))
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn from_str(string: &str) -> Option<Self> {
        let mut chars = Vec::new();

        for byte in string.bytes() {
            match Character::from_byte(byte) {
                Some(char) => chars.push(char),
                None => return None,
            }
        }

        if chars.len() > L {
            None
        } else {
            Some(Self(chars.into()))
        }
    }
}

impl<const L: usize> fmt::Display for Sequence<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for character in self.0.iter() {
            character.fmt(f)?
        }

        Ok(())
    }
}
