use std::fmt::{Display, Formatter};
use std::io;
use std::io::{Error, Read, Write};

use wow_srp::header_crypto::{Decrypter, Encrypter};

pub use aura_mask::AuraMask;
pub use guid::Guid;
pub use update_mask::UpdateMask;

use crate::util::{read_u16_le, read_u32_le, read_u64_le, read_u8_le};
use crate::ReadableAndWritable;

mod aura_mask;
mod expected;
mod guid;
mod update_mask;

pub trait WorldServerMessageWrite: WorldMessageBody {
    const OPCODE: u16;

    fn write_unencrypted_server<W: Write>(&self, w: &mut W) -> Result<(), io::Error>;

    fn write_encrypted_server<W: Write, E: Encrypter>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), io::Error>;
}

pub trait WorldClientMessageWrite: WorldMessageBody {
    const OPCODE: u32;

    fn write_unencrypted_client<W: Write>(&self, w: &mut W) -> Result<(), io::Error>;

    fn write_encrypted_client<W: Write, E: Encrypter>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), io::Error>;
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
