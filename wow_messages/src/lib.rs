#![allow(dead_code)]
#![allow(unused)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::util::read_u8_le;
use std::fmt::{Display, Formatter};
use std::io::{Error, Read};

pub(crate) mod util;
pub mod world;

pub trait ReadableAndWritable: Sized {
    type Error;
    fn read<R: std::io::Read>(r: &mut R) -> Result<Self, Self::Error>;
    fn write<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error>;
}

pub trait ConstantSized: MaximumPossibleSized {
    fn size() -> usize;
}

pub trait VariableSized: MaximumPossibleSized {
    fn size(&self) -> usize;
}

pub trait MaximumPossibleSized {
    fn maximum_possible_size() -> usize;
}
