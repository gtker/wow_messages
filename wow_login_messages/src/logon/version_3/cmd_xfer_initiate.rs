use crate::ServerMessage;
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_xfer.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_xfer.wowm#L3):
/// ```text
/// slogin CMD_XFER_INITIATE = 0x30 {
/// }
/// ```
pub struct CMD_XFER_INITIATE {
}

impl CMD_XFER_INITIATE {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        Ok(())
    }
}

impl ServerMessage for CMD_XFER_INITIATE {
    const OPCODE: u8 = 0x30;

    fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        Ok(Self {
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, mut w: W) -> std::result::Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(1);
        self.write_into_vec(&mut v)?;
        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_read<'async_trait, R>(
        mut r: R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + tokio::io::AsyncReadExt + Unpin + Send,
        Self: 'async_trait,
     {
        Box::pin(async move {
            Ok(Self {
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write<'life0, 'async_trait, W>(
        &'life0 self,
        mut w: W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(1);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read<'async_trait, R>(
        mut r: R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + async_std::io::ReadExt + Unpin + Send,
        Self: 'async_trait,
     {
        Box::pin(async move {
            Ok(Self {
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write<'life0, 'async_trait, W>(
        &'life0 self,
        mut w: W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
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
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ServerOpcodeMessage;

    const RAW0: [u8; 1] = [ 0x30, ];

    pub(crate) fn expected0() -> CMD_XFER_INITIATE {
        CMD_XFER_INITIATE {
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 5.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_XFER_INITIATE0() {
        let expected = expected0();
        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE => {}
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 5.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_XFER_INITIATE0() {
        let expected = expected0();
        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE => {}
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 5.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_XFER_INITIATE0() {
        let expected = expected0();
        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE => {}
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

