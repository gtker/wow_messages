use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_SET_ACTIONBAR_TOGGLES {
    pub action_bar: u8,
}

impl ClientMessageWrite for CMSG_SET_ACTIONBAR_TOGGLES {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_SET_ACTIONBAR_TOGGLES {
    const OPCODE: u16 = 0x02bf;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // action_bar: u8
        let action_bar = crate::util::read_u8_le(r)?;

        Ok(Self {
            action_bar,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // action_bar: u8
        w.write_all(&self.action_bar.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // action_bar: u8
        let action_bar = crate::util::tokio_read_u8_le(r).await?;

        Ok(Self {
            action_bar,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // action_bar: u8
        w.write_all(&self.action_bar.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // action_bar: u8
        let action_bar = crate::util::astd_read_u8_le(r).await?;

        Ok(Self {
            action_bar,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // action_bar: u8
        w.write_all(&self.action_bar.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for CMSG_SET_ACTIONBAR_TOGGLES {}

impl MaximumPossibleSized for CMSG_SET_ACTIONBAR_TOGGLES {
    fn maximum_possible_size() -> usize {
        1 // action_bar: u8
    }
}

