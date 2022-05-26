use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::CinematicSequenceId;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_TRIGGER_CINEMATIC {
    pub cinematic_sequence_id: CinematicSequenceId,
}

impl SMSG_TRIGGER_CINEMATIC {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 4], std::io::Error> {
        let mut array_w = [0u8; 4];
        let mut w = array_w.as_mut_slice();
        // cinematic_sequence_id: CinematicSequenceId
        w.write_all(&(self.cinematic_sequence_id.as_int() as u32).to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_TRIGGER_CINEMATIC {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // cinematic_sequence_id: CinematicSequenceId
        w.write_all(&(self.cinematic_sequence_id.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x00fa;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    type Error = crate::errors::ParseError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // cinematic_sequence_id: CinematicSequenceId
        let cinematic_sequence_id: CinematicSequenceId = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            cinematic_sequence_id,
        })
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
            // cinematic_sequence_id: CinematicSequenceId
            let cinematic_sequence_id: CinematicSequenceId = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                cinematic_sequence_id,
            })
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
            // cinematic_sequence_id: CinematicSequenceId
            let cinematic_sequence_id: CinematicSequenceId = crate::util::astd_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                cinematic_sequence_id,
            })
        })
    }

}

