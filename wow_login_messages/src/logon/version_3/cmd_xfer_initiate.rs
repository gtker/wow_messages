use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMD_XFER_INITIATE {
}

impl CMD_XFER_INITIATE {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        Ok(())
    }
}

impl ServerMessage for CMD_XFER_INITIATE {
    const OPCODE: u8 = 48;

    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(1);
        self.write_into_vec(&mut v)?;
        w.write_all(&v)
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
            let mut v = Vec::with_capacity(1);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
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
            let mut v = Vec::with_capacity(1);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

}

#[cfg(test)]
mod test {
    use super::CMD_XFER_INITIATE;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ServerOpcodeMessage;

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_XFER_INITIATE0() {
        const RAW: [u8; 1] = [ 0x30, ];

        let expected = CMD_XFER_INITIATE {
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(0 + header_size, RAW.len());

        let mut dest = Vec::with_capacity(RAW.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_XFER_INITIATE0() {
        const RAW: [u8; 1] = [ 0x30, ];

        let expected = CMD_XFER_INITIATE {
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(0 + header_size, RAW.len());

        let mut dest = Vec::with_capacity(RAW.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_XFER_INITIATE0() {
        const RAW: [u8; 1] = [ 0x30, ];

        let expected = CMD_XFER_INITIATE {
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(0 + header_size, RAW.len());

        let mut dest = Vec::with_capacity(RAW.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW);
    }

}
