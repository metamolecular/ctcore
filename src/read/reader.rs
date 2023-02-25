use std::iter::Peekable;

use crate::{
    build::{self, Builder, Target},
    text::{Character, Eol},
};

use super::Error;

pub struct Reader<'a> {
    iter: Peekable<&'a mut dyn Iterator<Item = u8>>,
    pub row: usize,
    pub column: usize,
}

impl<'a> Reader<'a> {
    pub fn new(iter: &'a mut dyn Iterator<Item = u8>) -> Self {
        Self {
            iter: iter.peekable(),
            row: 0,
            column: 0,
        }
    }

    pub fn read_line<P, B: Builder<Product = P>>(
        &mut self,
        target: Target<P, B>,
    ) -> Result<P, Error> {
        let product = self.read(target)?;
        self.next_line()?;

        Ok(product)
    }

    pub fn read<P, B: Builder<Product = P>>(
        &mut self,
        mut target: Target<P, B>,
    ) -> Result<P, Error> {
        loop {
            target = match target {
                Target::Builder(builder) => match self.iter.peek() {
                    Some(byte) => match Character::from_byte(*byte) {
                        Character::Eol(_) => match builder.done() {
                            Some(product) => break Ok(product),
                            None => break Err(Error::Eol(self.row)),
                        },
                        Character::Printable(printable) => {
                            match builder.push(printable) {
                                Ok(next) => {
                                    self.column += 1;

                                    self.iter.next();

                                    next
                                }
                                Err(build::Error::Character(allowed)) => {
                                    break Err(Error::Character(
                                        self.row,
                                        self.column,
                                        allowed,
                                    ))
                                }
                            }
                        }
                        Character::Unprintable(byte) => {
                            break Err(Error::Unprintable(
                                self.row,
                                self.column,
                                byte,
                            ))
                        }
                    },
                    None => match builder.done() {
                        Some(product) => break Ok(product),
                        None => break Err(Error::Eof(self.row)),
                    },
                },
                Target::Product(product) => break Ok(product),
            }
        }
    }

    pub fn next_line(&mut self) -> Result<(), Error> {
        let next = match self.iter.peek() {
            Some(byte) => Character::from_byte(*byte),
            None => return Err(Error::Eof(self.row)),
        };

        match next {
            Character::Eol(eol) => match eol {
                Eol::Cr => {
                    self.iter.next();

                    if let Some(byte) = self.iter.peek() {
                        if Character::from_byte(*byte).is_lf() {
                            self.iter.next();
                        }
                    }
                }
                Eol::Lf => {
                    self.iter.next();

                    if let Some(byte) = self.iter.peek() {
                        if Character::from_byte(*byte).is_cr() {
                            self.iter.next();
                        }
                    }
                }
                Eol::Rs => {
                    self.iter.next();
                }
            },
            Character::Printable(_) => {
                return Err(Error::Overflow(self.row, self.column))
            }
            Character::Unprintable(byte) => {
                return Err(Error::Unprintable(self.row, self.column, byte))
            }
        }

        self.row += 1;
        self.column = 0;

        Ok(())
    }

    pub fn has_blank(&mut self) -> bool {
        match self.iter.peek() {
            Some(byte) => match Character::from_byte(*byte) {
                Character::Eol(_) => true,
                _ => false,
            },
            None => false,
        }
    }
}
