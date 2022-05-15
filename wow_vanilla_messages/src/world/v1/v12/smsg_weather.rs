use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{WeatherChangeType, WeatherChangeTypeError};
use crate::world::v1::v12::{WeatherType, WeatherTypeError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
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

impl ServerMessageWrite for SMSG_WEATHER {}

impl MessageBody for SMSG_WEATHER {
    const OPCODE: u16 = 0x02f4;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_WEATHERError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // weather_type: WeatherType
        let weather_type: WeatherType = crate::util::read_u32_le(r)?.try_into()?;

        // grade: f32
        let grade = crate::util::read_f32_le(r)?;
        // sound_id: u32
        let sound_id = crate::util::read_u32_le(r)?;

        // change: WeatherChangeType
        let change: WeatherChangeType = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            weather_type,
            grade,
            sound_id,
            change,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // weather_type: WeatherType
        w.write_all(&(self.weather_type.as_int() as u32).to_le_bytes())?;

        // grade: f32
        w.write_all(&self.grade.to_le_bytes())?;

        // sound_id: u32
        w.write_all(&self.sound_id.to_le_bytes())?;

        // change: WeatherChangeType
        w.write_all(&(self.change.as_int() as u8).to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // weather_type: WeatherType
            let weather_type: WeatherType = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            // grade: f32
            let grade = crate::util::tokio_read_f32_le(r).await?;
            // sound_id: u32
            let sound_id = crate::util::tokio_read_u32_le(r).await?;

            // change: WeatherChangeType
            let change: WeatherChangeType = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                weather_type,
                grade,
                sound_id,
                change,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // weather_type: WeatherType
            w.write_all(&(self.weather_type.as_int() as u32).to_le_bytes()).await?;

            // grade: f32
            w.write_all(&self.grade.to_le_bytes()).await?;

            // sound_id: u32
            w.write_all(&self.sound_id.to_le_bytes()).await?;

            // change: WeatherChangeType
            w.write_all(&(self.change.as_int() as u8).to_le_bytes()).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // weather_type: WeatherType
            let weather_type: WeatherType = crate::util::astd_read_u32_le(r).await?.try_into()?;

            // grade: f32
            let grade = crate::util::astd_read_f32_le(r).await?;
            // sound_id: u32
            let sound_id = crate::util::astd_read_u32_le(r).await?;

            // change: WeatherChangeType
            let change: WeatherChangeType = crate::util::astd_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                weather_type,
                grade,
                sound_id,
                change,
            })
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // weather_type: WeatherType
            w.write_all(&(self.weather_type.as_int() as u32).to_le_bytes()).await?;

            // grade: f32
            w.write_all(&self.grade.to_le_bytes()).await?;

            // sound_id: u32
            w.write_all(&self.sound_id.to_le_bytes()).await?;

            // change: WeatherChangeType
            w.write_all(&(self.change.as_int() as u8).to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_WEATHER {}

impl MaximumPossibleSized for SMSG_WEATHER {
    fn maximum_possible_size() -> usize {
        0
        + 4 // weather_type: WeatherType
        + 4 // grade: f32
        + 4 // sound_id: u32
        + 1 // change: WeatherChangeType
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

