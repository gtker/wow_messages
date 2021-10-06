use std::io;
use std::io::{Read, Write};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct AuraMask {}

impl AuraMask {
    pub fn read(r: &mut impl Read) -> Result<Self, io::Error> {
        todo!()
    }

    pub fn write(&self, w: &mut impl Write) -> Result<(), io::Error> {
        todo!()
    }

    pub fn size(&self) -> usize {
        todo!()
    }
}
