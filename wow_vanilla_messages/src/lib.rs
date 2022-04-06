#![allow(dead_code)]
#![allow(unused)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::util::read_u8_le;
use std::fmt::{Display, Formatter};
use std::io::{Write, Error, Read};

use wow_srp::header_crypto::{Decrypter, Encrypter};

pub(crate) mod util;
mod world;
pub mod helper;

pub use world::*;

pub use helper::guid::Guid;
pub use helper::aura_mask::AuraMask;
pub use helper::update_mask::UpdateMask;

const DEFAULT_PORT: u16 = 8085;

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

pub trait WorldServerMessageWrite: WorldMessageBody {
    const OPCODE: u16;

    fn write_unencrypted_server<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error>;

    fn write_encrypted_server<W: Write, E: Encrypter>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error>;
}

pub trait WorldClientMessageWrite: WorldMessageBody {
    const OPCODE: u32;

    fn write_unencrypted_client<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error>;

    fn write_encrypted_client<W: Write, E: Encrypter>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error>;
}

pub trait WorldMessageBody: Sized {
    type Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> Result<Self, Self::Error>;

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error>;
}

pub trait WorldMessage: Sized {
    type Error;

    fn read_unencrypted<R: std::io::Read>(r: &mut R) -> Result<Self, Self::Error>;

    fn write_unencrypted<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error>;

    fn read_encrypted<R: std::io::Read, D: Decrypter>(
        r: &mut R,
        d: &mut D,
    ) -> std::result::Result<Self, Self::Error>;

    fn write_encrypted<W: std::io::Write, E: Encrypter>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> std::result::Result<(), std::io::Error>;
}
