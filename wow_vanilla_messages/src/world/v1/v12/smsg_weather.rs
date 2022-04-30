use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{WeatherChangeType, WeatherChangeTypeError};
use crate::world::v1::v12::{WeatherType, WeatherTypeError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
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
#[derive(Copy)]
pub struct SMSG_WEATHER {
    pub weather_type: WeatherType,
    pub grade: f32,
    pub sound_id: u32,
    pub change: WeatherChangeType,
}

impl WorldServerMessageWrite for SMSG_WEATHER {
    const OPCODE: u16 = 0x2f4;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_WEATHER {
    type Error = SMSG_WEATHERError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // weather_type: WeatherType
        let weather_type = WeatherType::read(r)?;

        // grade: f32
        let grade = crate::util::read_f32_le(r)?;
        // sound_id: u32
        let sound_id = crate::util::read_u32_le(r)?;

        // change: WeatherChangeType
        let change = WeatherChangeType::read(r)?;

        Ok(Self {
            weather_type,
            grade,
            sound_id,
            change,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // weather_type: WeatherType
        self.weather_type.write(w)?;

        // grade: f32
        w.write_all(&self.grade.to_le_bytes())?;

        // sound_id: u32
        w.write_all(&self.sound_id.to_le_bytes())?;

        // change: WeatherChangeType
        self.change.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_WEATHER {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_WEATHER {
    fn maximum_possible_size() -> usize {
        WeatherType::size() // weather_type: WeatherType
        + 4 // grade: f32
        + 4 // sound_id: u32
        + WeatherChangeType::size() // change: WeatherChangeType
    }
}

#[derive(Debug)]
pub enum SMSG_WEATHERError {
    Io(std::io::Error),
    WeatherChangeType(WeatherChangeTypeError),
    WeatherType(WeatherTypeError),
}

impl std::error::Error for SMSG_WEATHERError {}
impl std::fmt::Display for SMSG_WEATHERError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::WeatherChangeType(i) => i.fmt(f),
            Self::WeatherType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_WEATHERError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<WeatherChangeTypeError> for SMSG_WEATHERError {
    fn from(e: WeatherChangeTypeError) -> Self {
        Self::WeatherChangeType(e)
    }
}

impl From<WeatherTypeError> for SMSG_WEATHERError {
    fn from(e: WeatherTypeError) -> Self {
        Self::WeatherType(e)
    }
}

