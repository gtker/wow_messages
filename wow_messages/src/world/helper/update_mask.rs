use std::io;
use std::io::{Read, Write};

pub struct UpdateMask {}

impl UpdateMask {
    pub fn read(r: &mut impl Read) -> Result<Self, io::Error> {
        todo!()
    }

    pub fn write(&self, w: &mut impl Write) -> Result<(), io::Error> {
        todo!()
    }
}
