use crate::ClientMessage;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_xfer.wowm:40`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_xfer.wowm#L40):
/// ```text
/// clogin CMD_XFER_RESUME = 0x33 {
///     u64 offset;
/// }
/// ```
pub struct CMD_XFER_RESUME {
    pub offset: u64,
}

impl CMD_XFER_RESUME {
    pub(crate) fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // offset: u64
        w.write_all(&self.offset.to_le_bytes())?;

        Ok(())
    }
}

impl ClientMessage for CMD_XFER_RESUME {
    const OPCODE: u8 = 0x33;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // offset: u64
        let offset = crate::util::read_u64_le(r)?;

        Ok(Self {
            offset,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(9);
        self.write_into_vec(&mut v)?;
        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + tokio::io::AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // offset: u64
            let offset = crate::util::tokio_read_u64_le(r).await?;

            Ok(Self {
                offset,
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
            let mut v = Vec::with_capacity(9);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + async_std::io::ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // offset: u64
            let offset = crate::util::astd_read_u64_le(r).await?;

            Ok(Self {
                offset,
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
            let mut v = Vec::with_capacity(9);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

}

#[cfg(test)]
mod test {
    use super::CMD_XFER_RESUME;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ClientOpcodeMessage;

    const RAW0: [u8; 9] = [ 0x33, 0xAD, 0xDE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 44.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_XFER_RESUME0() {
        let expected = CMD_XFER_RESUME {
            offset: 0xDEAD,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.offset, expected.offset);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 44.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_XFER_RESUME0() {
        let expected = CMD_XFER_RESUME {
            offset: 0xDEAD,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.offset, expected.offset);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 44.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_XFER_RESUME0() {
        let expected = CMD_XFER_RESUME {
            offset: 0xDEAD,
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.offset, expected.offset);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

