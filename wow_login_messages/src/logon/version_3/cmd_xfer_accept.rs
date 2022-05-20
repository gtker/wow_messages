use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use crate::ReadableAndWritable;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMD_XFER_ACCEPT {
}

impl ClientMessage for CMD_XFER_ACCEPT {
    const OPCODE: u8 = 0x32;
}
impl CMD_XFER_ACCEPT {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 1], std::io::Error> {
        let mut array_w = [0u8; 1];
        let mut w = array_w.as_mut_slice();
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ReadableAndWritable for CMD_XFER_ACCEPT {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
    }

    #[cfg(feature = "tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            Ok(Self {
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
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
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            Ok(Self {
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
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

impl CMD_XFER_ACCEPT {
    pub(crate) fn size() -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use super::CMD_XFER_ACCEPT;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ClientOpcodeMessage;

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_XFER_ACCEPT0() {
        let raw: Vec<u8> = vec![ 0x32, ];

        let expected = CMD_XFER_ACCEPT {
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_ACCEPT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_ACCEPT, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(CMD_XFER_ACCEPT::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut std::io::Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_XFER_ACCEPT0() {
        let raw: Vec<u8> = vec![ 0x32, ];

        let expected = CMD_XFER_ACCEPT {
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_ACCEPT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_ACCEPT, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(CMD_XFER_ACCEPT::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_XFER_ACCEPT0() {
        let raw: Vec<u8> = vec![ 0x32, ];

        let expected = CMD_XFER_ACCEPT {
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_ACCEPT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_ACCEPT, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(CMD_XFER_ACCEPT::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

}
