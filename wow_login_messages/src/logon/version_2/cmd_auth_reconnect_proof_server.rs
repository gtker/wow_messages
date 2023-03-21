use crate::ServerMessage;
use std::io::{Read, Write};

use crate::logon::version_2::LoginResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Reply to [`CMD_AUTH_RECONNECT_PROOF_Client`](crate::logon::version_2::CMD_AUTH_RECONNECT_PROOF_Client).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm#L1):
/// ```text
/// slogin CMD_AUTH_RECONNECT_PROOF_Server = 0x03 {
///     LoginResult result;
/// }
/// ```
pub struct CMD_AUTH_RECONNECT_PROOF_Server {
    pub result: LoginResult,
}

impl CMD_AUTH_RECONNECT_PROOF_Server {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // result: LoginResult
        w.write_all(&u8::from(self.result.as_int()).to_le_bytes())?;

        Ok(())
    }
}

impl ServerMessage for CMD_AUTH_RECONNECT_PROOF_Server {
    const OPCODE: u8 = 0x03;

    fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // result: LoginResult
        let result: LoginResult = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, mut w: W) -> std::result::Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(2);
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
            // result: LoginResult
            let result: LoginResult = crate::util::tokio_read_u8_le(&mut r).await?.try_into()?;

            Ok(Self {
                result,
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
            let mut v = Vec::with_capacity(2);
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
            // result: LoginResult
            let result: LoginResult = crate::util::astd_read_u8_le(&mut r).await?.try_into()?;

            Ok(Self {
                result,
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
            let mut v = Vec::with_capacity(2);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

}

#[cfg(test)]
mod test {
    use super::CMD_AUTH_RECONNECT_PROOF_Server;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_2::opcodes::ServerOpcodeMessage;

    const RAW0: [u8; 2] = [ 0x03, 0x00, ];

    pub(crate) fn expected0() -> CMD_AUTH_RECONNECT_PROOF_Server {
        CMD_AUTH_RECONNECT_PROOF_Server {
            result: LoginResult::Success,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_RECONNECT_PROOF_Server0() {
        let expected = expected0();
        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(1 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_RECONNECT_PROOF_Server0() {
        let expected = expected0();
        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(1 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_RECONNECT_PROOF_Server0() {
        let expected = expected0();
        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(1 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 2] = [ 0x03, 0x0E, ];

    pub(crate) fn expected1() -> CMD_AUTH_RECONNECT_PROOF_Server {
        CMD_AUTH_RECONNECT_PROOF_Server {
            result: LoginResult::SuccessSurvey,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm` line 17.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_RECONNECT_PROOF_Server1() {
        let expected = expected1();
        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(1 + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm` line 17.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_RECONNECT_PROOF_Server1() {
        let expected = expected1();
        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(1 + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm` line 17.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_RECONNECT_PROOF_Server1() {
        let expected = expected1();
        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(1 + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    const RAW2: [u8; 2] = [ 0x03, 0x0E, ];

    pub(crate) fn expected2() -> CMD_AUTH_RECONNECT_PROOF_Server {
        CMD_AUTH_RECONNECT_PROOF_Server {
            result: LoginResult::SuccessSurvey,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm` line 44.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_RECONNECT_PROOF_Server2() {
        let expected = expected2();
        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW2)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(1 + header_size, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm` line 44.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_RECONNECT_PROOF_Server2() {
        let expected = expected2();
        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(1 + header_size, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/proof_server.wowm` line 44.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_RECONNECT_PROOF_Server2() {
        let expected = expected2();
        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(1 + header_size, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

}

