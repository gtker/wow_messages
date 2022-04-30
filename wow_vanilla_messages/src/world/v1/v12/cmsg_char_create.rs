use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Class, ClassError};
use crate::world::v1::v12::{Gender, GenderError};
use crate::world::v1::v12::{Race, RaceError};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_CHAR_CREATE {
    pub name: String,
    pub race: Race,
    pub class: Class,
    pub gender: Gender,
    pub skin: u8,
    pub face: u8,
    pub hairstyle: u8,
    pub haircolor: u8,
    pub facialhair: u8,
    pub outfit_id: u8,
}

impl ClientMessageWrite for CMSG_CHAR_CREATE {
    const OPCODE: u32 = 0x36;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for CMSG_CHAR_CREATE {
    type Error = CMSG_CHAR_CREATEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // race: Race
        let race = Race::read(r)?;

        // class: Class
        let class = Class::read(r)?;

        // gender: Gender
        let gender = Gender::read(r)?;

        // skin: u8
        let skin = crate::util::read_u8_le(r)?;

        // face: u8
        let face = crate::util::read_u8_le(r)?;

        // hairstyle: u8
        let hairstyle = crate::util::read_u8_le(r)?;

        // haircolor: u8
        let haircolor = crate::util::read_u8_le(r)?;

        // facialhair: u8
        let facialhair = crate::util::read_u8_le(r)?;

        // outfit_id: u8
        let outfit_id = crate::util::read_u8_le(r)?;

        Ok(Self {
            name,
            race,
            class,
            gender,
            skin,
            face,
            hairstyle,
            haircolor,
            facialhair,
            outfit_id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // race: Race
        self.race.write(w)?;

        // class: Class
        self.class.write(w)?;

        // gender: Gender
        self.gender.write(w)?;

        // skin: u8
        w.write_all(&self.skin.to_le_bytes())?;

        // face: u8
        w.write_all(&self.face.to_le_bytes())?;

        // hairstyle: u8
        w.write_all(&self.hairstyle.to_le_bytes())?;

        // haircolor: u8
        w.write_all(&self.haircolor.to_le_bytes())?;

        // facialhair: u8
        w.write_all(&self.facialhair.to_le_bytes())?;

        // outfit_id: u8
        w.write_all(&self.outfit_id.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for CMSG_CHAR_CREATE {
    fn size(&self) -> usize {
        self.name.len() + 1 // name: CString and Null Terminator
        + Race::size() // race: Race
        + Class::size() // class: Class
        + Gender::size() // gender: Gender
        + 1 // skin: u8
        + 1 // face: u8
        + 1 // hairstyle: u8
        + 1 // haircolor: u8
        + 1 // facialhair: u8
        + 1 // outfit_id: u8
    }
}

impl MaximumPossibleSized for CMSG_CHAR_CREATE {
    fn maximum_possible_size() -> usize {
        256 // name: CString
        + Race::maximum_possible_size() // race: Race
        + Class::maximum_possible_size() // class: Class
        + Gender::maximum_possible_size() // gender: Gender
        + 1 // skin: u8
        + 1 // face: u8
        + 1 // hairstyle: u8
        + 1 // haircolor: u8
        + 1 // facialhair: u8
        + 1 // outfit_id: u8
    }
}

#[derive(Debug)]
pub enum CMSG_CHAR_CREATEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    Class(ClassError),
    Gender(GenderError),
    Race(RaceError),
}

impl std::error::Error for CMSG_CHAR_CREATEError {}
impl std::fmt::Display for CMSG_CHAR_CREATEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Class(i) => i.fmt(f),
            Self::Gender(i) => i.fmt(f),
            Self::Race(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_CHAR_CREATEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_CHAR_CREATEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<ClassError> for CMSG_CHAR_CREATEError {
    fn from(e: ClassError) -> Self {
        Self::Class(e)
    }
}

impl From<GenderError> for CMSG_CHAR_CREATEError {
    fn from(e: GenderError) -> Self {
        Self::Gender(e)
    }
}

impl From<RaceError> for CMSG_CHAR_CREATEError {
    fn from(e: RaceError) -> Self {
        Self::Race(e)
    }
}

