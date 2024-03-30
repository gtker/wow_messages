use crate::Message;
use crate::ServerMessage;
use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_xfer.wowm:23`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_xfer.wowm#L23):
/// ```text
/// slogin CMD_XFER_DATA = 0x31 {
///     u16 size;
///     u8[size] data;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMD_XFER_DATA {
    pub data: Vec<u8>,
}

impl CMD_XFER_DATA {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // size: u16
        w.write_all(&(self.data.len() as u16).to_le_bytes())?;

        // data: u8[size]
        for i in self.data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
}

impl crate::private::Sealed for CMD_XFER_DATA {}

impl CMD_XFER_DATA {
    #[cfg(feature = "sync")]
    fn read_inner<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // size: u16
        let size = crate::util::read_u16_le(&mut r)?;

        // data: u8[size]
        let data = {
            let mut data = Vec::with_capacity(size as usize);
            for _ in 0..size {
                data.push(crate::util::read_u8_le(&mut r)?);
            }
            data
        };

        Ok(Self {
            data,
        })
    }

    #[cfg(feature = "tokio")]
    async fn tokio_read_inner<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // size: u16
        let size = crate::util::tokio_read_u16_le(&mut r).await?;

        // data: u8[size]
        let data = {
            let mut data = Vec::with_capacity(size as usize);
            for _ in 0..size {
                data.push(crate::util::tokio_read_u8_le(&mut r).await?);
            }
            data
        };

        Ok(Self {
            data,
        })
    }

    #[cfg(feature = "async-std")]
    async fn astd_read_inner<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // size: u16
        let size = crate::util::astd_read_u16_le(&mut r).await?;

        // data: u8[size]
        let data = {
            let mut data = Vec::with_capacity(size as usize);
            for _ in 0..size {
                data.push(crate::util::astd_read_u8_le(&mut r).await?);
            }
            data
        };

        Ok(Self {
            data,
        })
    }

}

impl Message for CMD_XFER_DATA {
    const OPCODE: u8 = 0x31;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read, I: crate::private::Sealed>(r: R) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r).map_err(|kind| crate::errors::ParseError::new(49, "CMD_XFER_DATA", kind))
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(self.size() + 1);
        self.write_into_vec(&mut v)?;
        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_read<'async_trait, R, I: crate::private::Sealed>(
        r: R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + tokio::io::AsyncReadExt + Unpin + Send,
        Self: 'async_trait,
     {
        Box::pin(async move {Self::tokio_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(49, "CMD_XFER_DATA", kind))})
    }

    #[cfg(feature = "tokio")]
    fn tokio_write<'life0, 'async_trait, W>(
        &'life0 self,
        mut w: W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(self.size() + 1);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read<'async_trait, R, I: crate::private::Sealed>(
        r: R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + async_std::io::ReadExt + Unpin + Send,
        Self: 'async_trait,
     {
        Box::pin(async move {Self::astd_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(49, "CMD_XFER_DATA", kind))})
    }

    #[cfg(feature = "async-std")]
    fn astd_write<'life0, 'async_trait, W>(
        &'life0 self,
        mut w: W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(self.size() + 1);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

}

impl ServerMessage for CMD_XFER_DATA {}
impl CMD_XFER_DATA {
    pub(crate) fn size(&self) -> usize {
        2 // size: u16
        + self.data.len() * core::mem::size_of::<u8>() // data: u8[size]
    }
}

#[cfg(test)]
mod test_version_2 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_XFER_DATA;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_2::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 67] = [ 0x31, 0x40, 0x00, 0x4D, 0x50, 0x51, 0x1A, 0x2C, 0x00,
         0x00, 0x00, 0x3C, 0xE0, 0x26, 0x00, 0x01, 0x00, 0x03, 0x00, 0xFC, 0xD9,
         0x26, 0x00, 0xFC, 0xDD, 0x26, 0x00, 0x40, 0x00, 0x00, 0x00, 0x24, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x02, 0x78, 0xDA, 0xEC, 0x7D, 0x7B, 0x7C, 0x54, 0xC5, 0xF5,
         0xF8, 0x6E, 0xF6, 0x86, 0x5C, 0x74, 0x25, 0x0B, 0xAE, 0xB8, ];

    pub(crate) fn expected0() -> CMD_XFER_DATA {
        CMD_XFER_DATA {
            data: vec![ 0x4D, 0x50, 0x51, 0x1A, 0x2C, 0x00, 0x00, 0x00, 0x3C, 0xE0,
                 0x26, 0x00, 0x01, 0x00, 0x03, 0x00, 0xFC, 0xD9, 0x26, 0x00, 0xFC,
                 0xDD, 0x26, 0x00, 0x40, 0x00, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x02, 0x78, 0xDA, 0xEC, 0x7D, 0x7B, 0x7C, 0x54, 0xC5, 0xF5,
                 0xF8, 0x6E, 0xF6, 0x86, 0x5C, 0x74, 0x25, 0x0B, 0xAE, 0xB8, ],
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 31.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_xfer_data0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 31.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_xfer_data0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 31.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_xfer_data0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_3 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_XFER_DATA;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 67] = [ 0x31, 0x40, 0x00, 0x4D, 0x50, 0x51, 0x1A, 0x2C, 0x00,
         0x00, 0x00, 0x3C, 0xE0, 0x26, 0x00, 0x01, 0x00, 0x03, 0x00, 0xFC, 0xD9,
         0x26, 0x00, 0xFC, 0xDD, 0x26, 0x00, 0x40, 0x00, 0x00, 0x00, 0x24, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x02, 0x78, 0xDA, 0xEC, 0x7D, 0x7B, 0x7C, 0x54, 0xC5, 0xF5,
         0xF8, 0x6E, 0xF6, 0x86, 0x5C, 0x74, 0x25, 0x0B, 0xAE, 0xB8, ];

    pub(crate) fn expected0() -> CMD_XFER_DATA {
        CMD_XFER_DATA {
            data: vec![ 0x4D, 0x50, 0x51, 0x1A, 0x2C, 0x00, 0x00, 0x00, 0x3C, 0xE0,
                 0x26, 0x00, 0x01, 0x00, 0x03, 0x00, 0xFC, 0xD9, 0x26, 0x00, 0xFC,
                 0xDD, 0x26, 0x00, 0x40, 0x00, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x02, 0x78, 0xDA, 0xEC, 0x7D, 0x7B, 0x7C, 0x54, 0xC5, 0xF5,
                 0xF8, 0x6E, 0xF6, 0x86, 0x5C, 0x74, 0x25, 0x0B, 0xAE, 0xB8, ],
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 31.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_xfer_data0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 31.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_xfer_data0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 31.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_xfer_data0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_5 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_XFER_DATA;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_5::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 67] = [ 0x31, 0x40, 0x00, 0x4D, 0x50, 0x51, 0x1A, 0x2C, 0x00,
         0x00, 0x00, 0x3C, 0xE0, 0x26, 0x00, 0x01, 0x00, 0x03, 0x00, 0xFC, 0xD9,
         0x26, 0x00, 0xFC, 0xDD, 0x26, 0x00, 0x40, 0x00, 0x00, 0x00, 0x24, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x02, 0x78, 0xDA, 0xEC, 0x7D, 0x7B, 0x7C, 0x54, 0xC5, 0xF5,
         0xF8, 0x6E, 0xF6, 0x86, 0x5C, 0x74, 0x25, 0x0B, 0xAE, 0xB8, ];

    pub(crate) fn expected0() -> CMD_XFER_DATA {
        CMD_XFER_DATA {
            data: vec![ 0x4D, 0x50, 0x51, 0x1A, 0x2C, 0x00, 0x00, 0x00, 0x3C, 0xE0,
                 0x26, 0x00, 0x01, 0x00, 0x03, 0x00, 0xFC, 0xD9, 0x26, 0x00, 0xFC,
                 0xDD, 0x26, 0x00, 0x40, 0x00, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x02, 0x78, 0xDA, 0xEC, 0x7D, 0x7B, 0x7C, 0x54, 0xC5, 0xF5,
                 0xF8, 0x6E, 0xF6, 0x86, 0x5C, 0x74, 0x25, 0x0B, 0xAE, 0xB8, ],
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 31.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_xfer_data0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 31.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_xfer_data0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 31.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_xfer_data0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_6 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_XFER_DATA;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_6::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 67] = [ 0x31, 0x40, 0x00, 0x4D, 0x50, 0x51, 0x1A, 0x2C, 0x00,
         0x00, 0x00, 0x3C, 0xE0, 0x26, 0x00, 0x01, 0x00, 0x03, 0x00, 0xFC, 0xD9,
         0x26, 0x00, 0xFC, 0xDD, 0x26, 0x00, 0x40, 0x00, 0x00, 0x00, 0x24, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x02, 0x78, 0xDA, 0xEC, 0x7D, 0x7B, 0x7C, 0x54, 0xC5, 0xF5,
         0xF8, 0x6E, 0xF6, 0x86, 0x5C, 0x74, 0x25, 0x0B, 0xAE, 0xB8, ];

    pub(crate) fn expected0() -> CMD_XFER_DATA {
        CMD_XFER_DATA {
            data: vec![ 0x4D, 0x50, 0x51, 0x1A, 0x2C, 0x00, 0x00, 0x00, 0x3C, 0xE0,
                 0x26, 0x00, 0x01, 0x00, 0x03, 0x00, 0xFC, 0xD9, 0x26, 0x00, 0xFC,
                 0xDD, 0x26, 0x00, 0x40, 0x00, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x02, 0x78, 0xDA, 0xEC, 0x7D, 0x7B, 0x7C, 0x54, 0xC5, 0xF5,
                 0xF8, 0x6E, 0xF6, 0x86, 0x5C, 0x74, 0x25, 0x0B, 0xAE, 0xB8, ],
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 31.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_xfer_data0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 31.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_xfer_data0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 31.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_xfer_data0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_7 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_XFER_DATA;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_7::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 67] = [ 0x31, 0x40, 0x00, 0x4D, 0x50, 0x51, 0x1A, 0x2C, 0x00,
         0x00, 0x00, 0x3C, 0xE0, 0x26, 0x00, 0x01, 0x00, 0x03, 0x00, 0xFC, 0xD9,
         0x26, 0x00, 0xFC, 0xDD, 0x26, 0x00, 0x40, 0x00, 0x00, 0x00, 0x24, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x02, 0x78, 0xDA, 0xEC, 0x7D, 0x7B, 0x7C, 0x54, 0xC5, 0xF5,
         0xF8, 0x6E, 0xF6, 0x86, 0x5C, 0x74, 0x25, 0x0B, 0xAE, 0xB8, ];

    pub(crate) fn expected0() -> CMD_XFER_DATA {
        CMD_XFER_DATA {
            data: vec![ 0x4D, 0x50, 0x51, 0x1A, 0x2C, 0x00, 0x00, 0x00, 0x3C, 0xE0,
                 0x26, 0x00, 0x01, 0x00, 0x03, 0x00, 0xFC, 0xD9, 0x26, 0x00, 0xFC,
                 0xDD, 0x26, 0x00, 0x40, 0x00, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x02, 0x78, 0xDA, 0xEC, 0x7D, 0x7B, 0x7C, 0x54, 0xC5, 0xF5,
                 0xF8, 0x6E, 0xF6, 0x86, 0x5C, 0x74, 0x25, 0x0B, 0xAE, 0xB8, ],
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 31.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_xfer_data0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 31.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_xfer_data0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 31.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_xfer_data0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_8 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_XFER_DATA;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_8::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 67] = [ 0x31, 0x40, 0x00, 0x4D, 0x50, 0x51, 0x1A, 0x2C, 0x00,
         0x00, 0x00, 0x3C, 0xE0, 0x26, 0x00, 0x01, 0x00, 0x03, 0x00, 0xFC, 0xD9,
         0x26, 0x00, 0xFC, 0xDD, 0x26, 0x00, 0x40, 0x00, 0x00, 0x00, 0x24, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x02, 0x78, 0xDA, 0xEC, 0x7D, 0x7B, 0x7C, 0x54, 0xC5, 0xF5,
         0xF8, 0x6E, 0xF6, 0x86, 0x5C, 0x74, 0x25, 0x0B, 0xAE, 0xB8, ];

    pub(crate) fn expected0() -> CMD_XFER_DATA {
        CMD_XFER_DATA {
            data: vec![ 0x4D, 0x50, 0x51, 0x1A, 0x2C, 0x00, 0x00, 0x00, 0x3C, 0xE0,
                 0x26, 0x00, 0x01, 0x00, 0x03, 0x00, 0xFC, 0xD9, 0x26, 0x00, 0xFC,
                 0xDD, 0x26, 0x00, 0x40, 0x00, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x02, 0x78, 0xDA, 0xEC, 0x7D, 0x7B, 0x7C, 0x54, 0xC5, 0xF5,
                 0xF8, 0x6E, 0xF6, 0x86, 0x5C, 0x74, 0x25, 0x0B, 0xAE, 0xB8, ],
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 31.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_xfer_data0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 31.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_xfer_data0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 31.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_xfer_data0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_DATA, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

