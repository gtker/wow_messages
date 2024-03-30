use crate::Message;
use crate::ServerMessage;
use std::io::{Read, Write};

use crate::logon::version_2::LoginResult;

/// Reply to [`CMD_AUTH_RECONNECT_PROOF_Client`](crate::logon::version_2::CMD_AUTH_RECONNECT_PROOF_Client).
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm:27`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm#L27):
/// ```text
/// slogin CMD_AUTH_RECONNECT_PROOF_Server = 0x03 {
///     LoginResult result;
///     u16 padding = 0;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMD_AUTH_RECONNECT_PROOF_Server {
    pub result: LoginResult,
}

impl CMD_AUTH_RECONNECT_PROOF_Server {
    /// The field `padding` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `0` |
    /// | Hex | `0x00` |
    /// | Original | `0` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const PADDING_VALUE: u16 = 0x00;

}

impl CMD_AUTH_RECONNECT_PROOF_Server {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // result: LoginResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        // padding: u16
        w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

        Ok(())
    }
}

impl crate::private::Sealed for CMD_AUTH_RECONNECT_PROOF_Server {}

impl CMD_AUTH_RECONNECT_PROOF_Server {
    #[cfg(feature = "sync")]
    fn read_inner<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // result: LoginResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        // padding: u16
        let _padding = crate::util::read_u16_le(&mut r)?;
        // padding is expected to always be 0 (0)

        Ok(Self {
            result,
        })
    }

    #[cfg(feature = "tokio")]
    async fn tokio_read_inner<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // result: LoginResult
        let result = crate::util::tokio_read_u8_le(&mut r).await?.try_into()?;

        // padding: u16
        let _padding = crate::util::tokio_read_u16_le(&mut r).await?;
        // padding is expected to always be 0 (0)

        Ok(Self {
            result,
        })
    }

    #[cfg(feature = "async-std")]
    async fn astd_read_inner<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // result: LoginResult
        let result = crate::util::astd_read_u8_le(&mut r).await?.try_into()?;

        // padding: u16
        let _padding = crate::util::astd_read_u16_le(&mut r).await?;
        // padding is expected to always be 0 (0)

        Ok(Self {
            result,
        })
    }

}

impl Message for CMD_AUTH_RECONNECT_PROOF_Server {
    const OPCODE: u8 = 0x03;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read, I: crate::private::Sealed>(r: R) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r).map_err(|kind| crate::errors::ParseError::new(3, "CMD_AUTH_RECONNECT_PROOF_Server", kind))
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(4);
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
        Box::pin(async move {Self::tokio_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(3, "CMD_AUTH_RECONNECT_PROOF_Server", kind))})
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
            let mut v = Vec::with_capacity(4);
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
        Box::pin(async move {Self::astd_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(3, "CMD_AUTH_RECONNECT_PROOF_Server", kind))})
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
            let mut v = Vec::with_capacity(4);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

}

impl ServerMessage for CMD_AUTH_RECONNECT_PROOF_Server {}
#[cfg(test)]
mod test_version_5 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_AUTH_RECONNECT_PROOF_Server;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_5::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 4] = [ 0x03, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMD_AUTH_RECONNECT_PROOF_Server {
        CMD_AUTH_RECONNECT_PROOF_Server {
            result: LoginResult::Success,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm` line 34.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_reconnect_proof_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(3 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm` line 34.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_reconnect_proof_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(3 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm` line 34.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_reconnect_proof_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(3 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_6 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_AUTH_RECONNECT_PROOF_Server;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_6::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 4] = [ 0x03, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMD_AUTH_RECONNECT_PROOF_Server {
        CMD_AUTH_RECONNECT_PROOF_Server {
            result: LoginResult::Success,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm` line 34.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_reconnect_proof_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(3 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm` line 34.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_reconnect_proof_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(3 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm` line 34.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_reconnect_proof_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(3 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_7 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_AUTH_RECONNECT_PROOF_Server;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_7::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 4] = [ 0x03, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMD_AUTH_RECONNECT_PROOF_Server {
        CMD_AUTH_RECONNECT_PROOF_Server {
            result: LoginResult::Success,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm` line 34.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_reconnect_proof_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(3 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm` line 34.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_reconnect_proof_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(3 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm` line 34.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_reconnect_proof_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(3 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

