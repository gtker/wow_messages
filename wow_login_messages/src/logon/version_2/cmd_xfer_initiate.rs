use crate::Message;
use crate::ServerMessage;
use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_xfer.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_xfer.wowm#L1):
/// ```text
/// slogin CMD_XFER_INITIATE = 0x30 {
///     String filename;
///     u64 file_size;
///     u8[16] file_md5;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMD_XFER_INITIATE {
    pub filename: String,
    pub file_size: u64,
    pub file_md5: [u8; 16],
}

impl CMD_XFER_INITIATE {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // filename: String
        w.write_all(&(self.filename.len() as u8).to_le_bytes())?;
        w.write_all(self.filename.as_bytes())?;

        // file_size: u64
        w.write_all(&self.file_size.to_le_bytes())?;

        // file_md5: u8[16]
        for i in self.file_md5.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
}

impl crate::private::Sealed for CMD_XFER_INITIATE {}

impl CMD_XFER_INITIATE {
    #[cfg(feature = "sync")]
    fn read_inner<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // filename: String
        let filename = {
            let filename = crate::util::read_u8_le(&mut r)?;
            let filename = crate::util::read_fixed_string_to_vec(&mut r, filename as usize)?;
            String::from_utf8(filename)?
        };

        // file_size: u64
        let file_size = crate::util::read_u64_le(&mut r)?;

        // file_md5: u8[16]
        let file_md5 = {
            let mut file_md5 = [0_u8; 16];
            r.read_exact(&mut file_md5)?;
            file_md5
        };

        Ok(Self {
            filename,
            file_size,
            file_md5,
        })
    }

    #[cfg(feature = "tokio")]
    async fn tokio_read_inner<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // filename: String
        let filename = {
            let filename = crate::util::tokio_read_u8_le(&mut r).await?;
            let filename = crate::util::tokio_read_fixed_string_to_vec(&mut r, filename as usize).await?;
            String::from_utf8(filename)?
        };

        // file_size: u64
        let file_size = crate::util::tokio_read_u64_le(&mut r).await?;

        // file_md5: u8[16]
        let file_md5 = {
            let mut file_md5 = [0_u8; 16];
            r.read_exact(&mut file_md5).await?;
            file_md5
        };

        Ok(Self {
            filename,
            file_size,
            file_md5,
        })
    }

    #[cfg(feature = "async-std")]
    async fn astd_read_inner<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // filename: String
        let filename = {
            let filename = crate::util::astd_read_u8_le(&mut r).await?;
            let filename = crate::util::astd_read_fixed_string_to_vec(&mut r, filename as usize).await?;
            String::from_utf8(filename)?
        };

        // file_size: u64
        let file_size = crate::util::astd_read_u64_le(&mut r).await?;

        // file_md5: u8[16]
        let file_md5 = {
            let mut file_md5 = [0_u8; 16];
            r.read_exact(&mut file_md5).await?;
            file_md5
        };

        Ok(Self {
            filename,
            file_size,
            file_md5,
        })
    }

}

impl Message for CMD_XFER_INITIATE {
    const OPCODE: u8 = 0x30;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read, I: crate::private::Sealed>(r: R) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r).map_err(|kind| crate::errors::ParseError::new(48, "CMD_XFER_INITIATE", kind))
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
        Box::pin(async move {Self::tokio_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(48, "CMD_XFER_INITIATE", kind))})
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
        Box::pin(async move {Self::astd_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(48, "CMD_XFER_INITIATE", kind))})
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

impl ServerMessage for CMD_XFER_INITIATE {}
impl CMD_XFER_INITIATE {
    pub(crate) fn size(&self) -> usize {
        self.filename.len() + 1 // filename: String
        + 8 // file_size: u64
        + 16 // file_md5: u8[16]
    }
}

#[cfg(test)]
mod test_version_2 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_XFER_INITIATE;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_2::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 31] = [ 0x30, 0x05, 0x50, 0x61, 0x74, 0x63, 0x68, 0xBC, 0x09,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x5B, 0x55, 0x59, 0x7F, 0xB7,
         0xDF, 0x0E, 0x86, 0xD9, 0xB3, 0xAE, 0x5A, 0xEB, 0xCB, 0x62, ];

    pub(crate) fn expected0() -> CMD_XFER_INITIATE {
        CMD_XFER_INITIATE {
            filename: String::from("Patch"),
            file_size: 0x9BC,
            file_md5: [ 0x11, 0x5B, 0x55, 0x59, 0x7F, 0xB7, 0xDF, 0x0E, 0x86, 0xD9,
                 0xB3, 0xAE, 0x5A, 0xEB, 0xCB, 0x62, ],
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_xfer_initiate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_xfer_initiate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_xfer_initiate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}"),
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
    use super::CMD_XFER_INITIATE;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 31] = [ 0x30, 0x05, 0x50, 0x61, 0x74, 0x63, 0x68, 0xBC, 0x09,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x5B, 0x55, 0x59, 0x7F, 0xB7,
         0xDF, 0x0E, 0x86, 0xD9, 0xB3, 0xAE, 0x5A, 0xEB, 0xCB, 0x62, ];

    pub(crate) fn expected0() -> CMD_XFER_INITIATE {
        CMD_XFER_INITIATE {
            filename: String::from("Patch"),
            file_size: 0x9BC,
            file_md5: [ 0x11, 0x5B, 0x55, 0x59, 0x7F, 0xB7, 0xDF, 0x0E, 0x86, 0xD9,
                 0xB3, 0xAE, 0x5A, 0xEB, 0xCB, 0x62, ],
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_xfer_initiate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_xfer_initiate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_xfer_initiate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}"),
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
    use super::CMD_XFER_INITIATE;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_5::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 31] = [ 0x30, 0x05, 0x50, 0x61, 0x74, 0x63, 0x68, 0xBC, 0x09,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x5B, 0x55, 0x59, 0x7F, 0xB7,
         0xDF, 0x0E, 0x86, 0xD9, 0xB3, 0xAE, 0x5A, 0xEB, 0xCB, 0x62, ];

    pub(crate) fn expected0() -> CMD_XFER_INITIATE {
        CMD_XFER_INITIATE {
            filename: String::from("Patch"),
            file_size: 0x9BC,
            file_md5: [ 0x11, 0x5B, 0x55, 0x59, 0x7F, 0xB7, 0xDF, 0x0E, 0x86, 0xD9,
                 0xB3, 0xAE, 0x5A, 0xEB, 0xCB, 0x62, ],
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_xfer_initiate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_xfer_initiate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_xfer_initiate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}"),
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
    use super::CMD_XFER_INITIATE;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_6::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 31] = [ 0x30, 0x05, 0x50, 0x61, 0x74, 0x63, 0x68, 0xBC, 0x09,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x5B, 0x55, 0x59, 0x7F, 0xB7,
         0xDF, 0x0E, 0x86, 0xD9, 0xB3, 0xAE, 0x5A, 0xEB, 0xCB, 0x62, ];

    pub(crate) fn expected0() -> CMD_XFER_INITIATE {
        CMD_XFER_INITIATE {
            filename: String::from("Patch"),
            file_size: 0x9BC,
            file_md5: [ 0x11, 0x5B, 0x55, 0x59, 0x7F, 0xB7, 0xDF, 0x0E, 0x86, 0xD9,
                 0xB3, 0xAE, 0x5A, 0xEB, 0xCB, 0x62, ],
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_xfer_initiate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_xfer_initiate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_xfer_initiate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}"),
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
    use super::CMD_XFER_INITIATE;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_7::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 31] = [ 0x30, 0x05, 0x50, 0x61, 0x74, 0x63, 0x68, 0xBC, 0x09,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x5B, 0x55, 0x59, 0x7F, 0xB7,
         0xDF, 0x0E, 0x86, 0xD9, 0xB3, 0xAE, 0x5A, 0xEB, 0xCB, 0x62, ];

    pub(crate) fn expected0() -> CMD_XFER_INITIATE {
        CMD_XFER_INITIATE {
            filename: String::from("Patch"),
            file_size: 0x9BC,
            file_md5: [ 0x11, 0x5B, 0x55, 0x59, 0x7F, 0xB7, 0xDF, 0x0E, 0x86, 0xD9,
                 0xB3, 0xAE, 0x5A, 0xEB, 0xCB, 0x62, ],
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_xfer_initiate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_xfer_initiate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_xfer_initiate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}"),
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
    use super::CMD_XFER_INITIATE;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_8::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 31] = [ 0x30, 0x05, 0x50, 0x61, 0x74, 0x63, 0x68, 0xBC, 0x09,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x5B, 0x55, 0x59, 0x7F, 0xB7,
         0xDF, 0x0E, 0x86, 0xD9, 0xB3, 0xAE, 0x5A, 0xEB, 0xCB, 0x62, ];

    pub(crate) fn expected0() -> CMD_XFER_INITIATE {
        CMD_XFER_INITIATE {
            filename: String::from("Patch"),
            file_size: 0x9BC,
            file_md5: [ 0x11, 0x5B, 0x55, 0x59, 0x7F, 0xB7, 0xDF, 0x0E, 0x86, 0xD9,
                 0xB3, 0xAE, 0x5A, 0xEB, 0xCB, 0x62, ],
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_xfer_initiate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_xfer_initiate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_xfer_initiate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_XFER_INITIATE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_INITIATE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

