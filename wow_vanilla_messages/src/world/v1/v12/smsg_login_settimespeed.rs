use std::convert::{TryFrom, TryInto};
use crate::{ServerMessage, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_LOGIN_SETTIMESPEED {
    pub secs_to_time_bit_field: u32,
    pub game_speed: f32,
}

impl ServerMessage for SMSG_LOGIN_SETTIMESPEED {}

impl SMSG_LOGIN_SETTIMESPEED {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 8], std::io::Error> {
        let mut array_w = [0u8; 8];
        let mut w = array_w.as_mut_slice();
        // secs_to_time_bit_field: u32
        w.write_all(&self.secs_to_time_bit_field.to_le_bytes())?;

        // game_speed: f32
        w.write_all(&self.game_speed.to_le_bytes())?;

        Ok(array_w)
    }
}

impl MessageBody for SMSG_LOGIN_SETTIMESPEED {
    const OPCODE: u16 = 0x0042;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // secs_to_time_bit_field: u32
        let secs_to_time_bit_field = crate::util::read_u32_le(r)?;

        // game_speed: f32
        let game_speed = crate::util::read_f32_le(r)?;
        Ok(Self {
            secs_to_time_bit_field,
            game_speed,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
    }

    #[cfg(feature = "tokio")]
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
            // secs_to_time_bit_field: u32
            let secs_to_time_bit_field = crate::util::tokio_read_u32_le(r).await?;

            // game_speed: f32
            let game_speed = crate::util::tokio_read_f32_le(r).await?;
            Ok(Self {
                secs_to_time_bit_field,
                game_speed,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

    #[cfg(feature = "async-std")]
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
            // secs_to_time_bit_field: u32
            let secs_to_time_bit_field = crate::util::astd_read_u32_le(r).await?;

            // game_speed: f32
            let game_speed = crate::util::astd_read_f32_le(r).await?;
            Ok(Self {
                secs_to_time_bit_field,
                game_speed,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

}

