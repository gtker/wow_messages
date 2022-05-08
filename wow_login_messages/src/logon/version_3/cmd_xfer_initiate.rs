use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMD_XFER_INITIATE {
}

impl ServerMessage for CMD_XFER_INITIATE {
    const OPCODE: u8 = 0x30;
}
#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for CMD_XFER_INITIATE {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
        })
    }

    fn tokio_write<'life0, 'life1, 'async_trait, W>(
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
            // opcode: u8
            w.write_all(&Self::OPCODE.to_le_bytes()).await?;

            Ok(())
        })
    }
    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
        })
    }

    fn astd_write<'life0, 'life1, 'async_trait, W>(
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
            // opcode: u8
            w.write_all(&Self::OPCODE.to_le_bytes()).await?;

            Ok(())
        })
    }
}

impl ConstantSized for CMD_XFER_INITIATE {}

impl MaximumPossibleSized for CMD_XFER_INITIATE {
    fn maximum_possible_size() -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use super::CMD_XFER_INITIATE;
    use crate::ConstantSized;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ServerOpcodeMessage;

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_XFER_INITIATE0() {
        let raw: Vec<u8> = vec![ 0x30, ];

        let expected = CMD_XFER_INITIATE {
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(CMD_XFER_INITIATE::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut std::io::Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_tokio")]
    #[cfg_attr(feature = "async_tokio", async_std::test)]
    async fn tokio_CMD_XFER_INITIATE0() {
        let raw: Vec<u8> = vec![ 0x30, ];

        let expected = CMD_XFER_INITIATE {
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(CMD_XFER_INITIATE::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async_std")]
    #[cfg_attr(feature = "async_std", tokio::test)]
    async fn astd_CMD_XFER_INITIATE0() {
        let raw: Vec<u8> = vec![ 0x30, ];

        let expected = CMD_XFER_INITIATE {
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(CMD_XFER_INITIATE::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

}
