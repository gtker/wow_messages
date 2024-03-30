use crate::Message;
use crate::ClientMessage;
use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_xfer.wowm:53`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_xfer.wowm#L53):
/// ```text
/// clogin CMD_XFER_RESUME = 0x33 {
///     u64 offset;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMD_XFER_RESUME {
    pub offset: u64,
}

impl CMD_XFER_RESUME {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // offset: u64
        w.write_all(&self.offset.to_le_bytes())?;

        Ok(())
    }
}

impl crate::private::Sealed for CMD_XFER_RESUME {}

impl CMD_XFER_RESUME {
    #[cfg(feature = "sync")]
    fn read_inner<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // offset: u64
        let offset = crate::util::read_u64_le(&mut r)?;

        Ok(Self {
            offset,
        })
    }

    #[cfg(feature = "tokio")]
    async fn tokio_read_inner<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // offset: u64
        let offset = crate::util::tokio_read_u64_le(&mut r).await?;

        Ok(Self {
            offset,
        })
    }

    #[cfg(feature = "async-std")]
    async fn astd_read_inner<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // offset: u64
        let offset = crate::util::astd_read_u64_le(&mut r).await?;

        Ok(Self {
            offset,
        })
    }

}

impl Message for CMD_XFER_RESUME {
    const OPCODE: u8 = 0x33;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read, I: crate::private::Sealed>(r: R) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r).map_err(|kind| crate::errors::ParseError::new(51, "CMD_XFER_RESUME", kind))
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(9);
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
        Box::pin(async move {Self::tokio_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(51, "CMD_XFER_RESUME", kind))})
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
            let mut v = Vec::with_capacity(9);
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
        Box::pin(async move {Self::astd_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(51, "CMD_XFER_RESUME", kind))})
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
            let mut v = Vec::with_capacity(9);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

}

impl ClientMessage for CMD_XFER_RESUME {}
#[cfg(test)]
mod test_version_2 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_XFER_RESUME;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_2::opcodes::ClientOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    fn assert(t: &CMD_XFER_RESUME, expected: &CMD_XFER_RESUME) {
        assert_eq!(t.offset, expected.offset);
    }

    const RAW0: [u8; 9] = [ 0x33, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMD_XFER_RESUME {
        CMD_XFER_RESUME {
            offset: 0x40,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 60.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_xfer_resume0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 60.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_xfer_resume0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 60.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_xfer_resume0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_3 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_XFER_RESUME;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ClientOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    fn assert(t: &CMD_XFER_RESUME, expected: &CMD_XFER_RESUME) {
        assert_eq!(t.offset, expected.offset);
    }

    const RAW0: [u8; 9] = [ 0x33, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMD_XFER_RESUME {
        CMD_XFER_RESUME {
            offset: 0x40,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 60.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_xfer_resume0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 60.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_xfer_resume0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 60.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_xfer_resume0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_5 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_XFER_RESUME;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_5::opcodes::ClientOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    fn assert(t: &CMD_XFER_RESUME, expected: &CMD_XFER_RESUME) {
        assert_eq!(t.offset, expected.offset);
    }

    const RAW0: [u8; 9] = [ 0x33, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMD_XFER_RESUME {
        CMD_XFER_RESUME {
            offset: 0x40,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 60.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_xfer_resume0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 60.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_xfer_resume0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 60.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_xfer_resume0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_6 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_XFER_RESUME;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_6::opcodes::ClientOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    fn assert(t: &CMD_XFER_RESUME, expected: &CMD_XFER_RESUME) {
        assert_eq!(t.offset, expected.offset);
    }

    const RAW0: [u8; 9] = [ 0x33, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMD_XFER_RESUME {
        CMD_XFER_RESUME {
            offset: 0x40,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 60.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_xfer_resume0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 60.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_xfer_resume0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 60.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_xfer_resume0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_7 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_XFER_RESUME;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_7::opcodes::ClientOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    fn assert(t: &CMD_XFER_RESUME, expected: &CMD_XFER_RESUME) {
        assert_eq!(t.offset, expected.offset);
    }

    const RAW0: [u8; 9] = [ 0x33, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMD_XFER_RESUME {
        CMD_XFER_RESUME {
            offset: 0x40,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 60.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_xfer_resume0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 60.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_xfer_resume0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 60.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_xfer_resume0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_8 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_XFER_RESUME;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_8::opcodes::ClientOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    fn assert(t: &CMD_XFER_RESUME, expected: &CMD_XFER_RESUME) {
        assert_eq!(t.offset, expected.offset);
    }

    const RAW0: [u8; 9] = [ 0x33, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMD_XFER_RESUME {
        CMD_XFER_RESUME {
            offset: 0x40,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 60.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_xfer_resume0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 60.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_xfer_resume0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_xfer.wowm` line 60.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_xfer_resume0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_XFER_RESUME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_XFER_RESUME, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

