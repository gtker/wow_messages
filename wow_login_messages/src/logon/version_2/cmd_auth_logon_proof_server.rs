use crate::Message;
use crate::ServerMessage;
use std::io::{Read, Write};

use crate::logon::version_2::LoginResult;

/// Reply to [`CMD_AUTH_LOGON_PROOF_Client`](crate::logon::version_2::CMD_AUTH_LOGON_PROOF_Client).
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm#L2):
/// ```text
/// slogin CMD_AUTH_LOGON_PROOF_Server = 0x01 {
///     LoginResult result;
///     if (result == SUCCESS) {
///         u8[20] server_proof;
///         u32 hardware_survey_id;
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CMD_AUTH_LOGON_PROOF_Server {
    Success {
        hardware_survey_id: u32,
        server_proof: [u8; 20],
    },
    FailUnknown0,
    FailUnknown1,
    FailBanned,
    FailUnknownAccount,
    FailIncorrectPassword,
    FailAlreadyOnline,
    FailNoTime,
    FailDbBusy,
    FailVersionInvalid,
    LoginDownloadFile,
    FailInvalidServer,
    FailSuspended,
    FailNoAccess,
    SuccessSurvey,
    FailParentalcontrol,
}

impl CMD_AUTH_LOGON_PROOF_Server {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // result: LoginResult
        w.write_all(&(self.as_int().to_le_bytes()))?;

        match &self {
            CMD_AUTH_LOGON_PROOF_Server::Success {
                hardware_survey_id,
                server_proof,
            } => {
                // server_proof: u8[20]
                for i in server_proof.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

                // hardware_survey_id: u32
                w.write_all(&hardware_survey_id.to_le_bytes())?;

            }
            _ => {}
        }

        Ok(())
    }
}

impl crate::private::Sealed for CMD_AUTH_LOGON_PROOF_Server {}

impl CMD_AUTH_LOGON_PROOF_Server {
    #[cfg(feature = "sync")]
    fn read_inner<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // result: LoginResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        let result_if = match result {
            LoginResult::Success => {
                // server_proof: u8[20]
                let server_proof = {
                    let mut server_proof = [0_u8; 20];
                    r.read_exact(&mut server_proof)?;
                    server_proof
                };

                // hardware_survey_id: u32
                let hardware_survey_id = crate::util::read_u32_le(&mut r)?;

                CMD_AUTH_LOGON_PROOF_Server::Success {
                    hardware_survey_id,
                    server_proof,
                }
            }
            LoginResult::FailUnknown0 => CMD_AUTH_LOGON_PROOF_Server::FailUnknown0,
            LoginResult::FailUnknown1 => CMD_AUTH_LOGON_PROOF_Server::FailUnknown1,
            LoginResult::FailBanned => CMD_AUTH_LOGON_PROOF_Server::FailBanned,
            LoginResult::FailUnknownAccount => CMD_AUTH_LOGON_PROOF_Server::FailUnknownAccount,
            LoginResult::FailIncorrectPassword => CMD_AUTH_LOGON_PROOF_Server::FailIncorrectPassword,
            LoginResult::FailAlreadyOnline => CMD_AUTH_LOGON_PROOF_Server::FailAlreadyOnline,
            LoginResult::FailNoTime => CMD_AUTH_LOGON_PROOF_Server::FailNoTime,
            LoginResult::FailDbBusy => CMD_AUTH_LOGON_PROOF_Server::FailDbBusy,
            LoginResult::FailVersionInvalid => CMD_AUTH_LOGON_PROOF_Server::FailVersionInvalid,
            LoginResult::LoginDownloadFile => CMD_AUTH_LOGON_PROOF_Server::LoginDownloadFile,
            LoginResult::FailInvalidServer => CMD_AUTH_LOGON_PROOF_Server::FailInvalidServer,
            LoginResult::FailSuspended => CMD_AUTH_LOGON_PROOF_Server::FailSuspended,
            LoginResult::FailNoAccess => CMD_AUTH_LOGON_PROOF_Server::FailNoAccess,
            LoginResult::SuccessSurvey => CMD_AUTH_LOGON_PROOF_Server::SuccessSurvey,
            LoginResult::FailParentalcontrol => CMD_AUTH_LOGON_PROOF_Server::FailParentalcontrol,
        };

        Ok(result_if)
    }

    #[cfg(feature = "tokio")]
    async fn tokio_read_inner<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // result: LoginResult
        let result = crate::util::tokio_read_u8_le(&mut r).await?.try_into()?;

        let result_if = match result {
            LoginResult::Success => {
                // server_proof: u8[20]
                let server_proof = {
                    let mut server_proof = [0_u8; 20];
                    r.read_exact(&mut server_proof).await?;
                    server_proof
                };

                // hardware_survey_id: u32
                let hardware_survey_id = crate::util::tokio_read_u32_le(&mut r).await?;

                CMD_AUTH_LOGON_PROOF_Server::Success {
                    hardware_survey_id,
                    server_proof,
                }
            }
            LoginResult::FailUnknown0 => CMD_AUTH_LOGON_PROOF_Server::FailUnknown0,
            LoginResult::FailUnknown1 => CMD_AUTH_LOGON_PROOF_Server::FailUnknown1,
            LoginResult::FailBanned => CMD_AUTH_LOGON_PROOF_Server::FailBanned,
            LoginResult::FailUnknownAccount => CMD_AUTH_LOGON_PROOF_Server::FailUnknownAccount,
            LoginResult::FailIncorrectPassword => CMD_AUTH_LOGON_PROOF_Server::FailIncorrectPassword,
            LoginResult::FailAlreadyOnline => CMD_AUTH_LOGON_PROOF_Server::FailAlreadyOnline,
            LoginResult::FailNoTime => CMD_AUTH_LOGON_PROOF_Server::FailNoTime,
            LoginResult::FailDbBusy => CMD_AUTH_LOGON_PROOF_Server::FailDbBusy,
            LoginResult::FailVersionInvalid => CMD_AUTH_LOGON_PROOF_Server::FailVersionInvalid,
            LoginResult::LoginDownloadFile => CMD_AUTH_LOGON_PROOF_Server::LoginDownloadFile,
            LoginResult::FailInvalidServer => CMD_AUTH_LOGON_PROOF_Server::FailInvalidServer,
            LoginResult::FailSuspended => CMD_AUTH_LOGON_PROOF_Server::FailSuspended,
            LoginResult::FailNoAccess => CMD_AUTH_LOGON_PROOF_Server::FailNoAccess,
            LoginResult::SuccessSurvey => CMD_AUTH_LOGON_PROOF_Server::SuccessSurvey,
            LoginResult::FailParentalcontrol => CMD_AUTH_LOGON_PROOF_Server::FailParentalcontrol,
        };

        Ok(result_if)
    }

    #[cfg(feature = "async-std")]
    async fn astd_read_inner<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // result: LoginResult
        let result = crate::util::astd_read_u8_le(&mut r).await?.try_into()?;

        let result_if = match result {
            LoginResult::Success => {
                // server_proof: u8[20]
                let server_proof = {
                    let mut server_proof = [0_u8; 20];
                    r.read_exact(&mut server_proof).await?;
                    server_proof
                };

                // hardware_survey_id: u32
                let hardware_survey_id = crate::util::astd_read_u32_le(&mut r).await?;

                CMD_AUTH_LOGON_PROOF_Server::Success {
                    hardware_survey_id,
                    server_proof,
                }
            }
            LoginResult::FailUnknown0 => CMD_AUTH_LOGON_PROOF_Server::FailUnknown0,
            LoginResult::FailUnknown1 => CMD_AUTH_LOGON_PROOF_Server::FailUnknown1,
            LoginResult::FailBanned => CMD_AUTH_LOGON_PROOF_Server::FailBanned,
            LoginResult::FailUnknownAccount => CMD_AUTH_LOGON_PROOF_Server::FailUnknownAccount,
            LoginResult::FailIncorrectPassword => CMD_AUTH_LOGON_PROOF_Server::FailIncorrectPassword,
            LoginResult::FailAlreadyOnline => CMD_AUTH_LOGON_PROOF_Server::FailAlreadyOnline,
            LoginResult::FailNoTime => CMD_AUTH_LOGON_PROOF_Server::FailNoTime,
            LoginResult::FailDbBusy => CMD_AUTH_LOGON_PROOF_Server::FailDbBusy,
            LoginResult::FailVersionInvalid => CMD_AUTH_LOGON_PROOF_Server::FailVersionInvalid,
            LoginResult::LoginDownloadFile => CMD_AUTH_LOGON_PROOF_Server::LoginDownloadFile,
            LoginResult::FailInvalidServer => CMD_AUTH_LOGON_PROOF_Server::FailInvalidServer,
            LoginResult::FailSuspended => CMD_AUTH_LOGON_PROOF_Server::FailSuspended,
            LoginResult::FailNoAccess => CMD_AUTH_LOGON_PROOF_Server::FailNoAccess,
            LoginResult::SuccessSurvey => CMD_AUTH_LOGON_PROOF_Server::SuccessSurvey,
            LoginResult::FailParentalcontrol => CMD_AUTH_LOGON_PROOF_Server::FailParentalcontrol,
        };

        Ok(result_if)
    }

}

impl Message for CMD_AUTH_LOGON_PROOF_Server {
    const OPCODE: u8 = 0x01;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read, I: crate::private::Sealed>(r: R) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r).map_err(|kind| crate::errors::ParseError::new(1, "CMD_AUTH_LOGON_PROOF_Server", kind))
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
        Box::pin(async move {Self::tokio_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(1, "CMD_AUTH_LOGON_PROOF_Server", kind))})
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
        Box::pin(async move {Self::astd_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(1, "CMD_AUTH_LOGON_PROOF_Server", kind))})
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

impl ServerMessage for CMD_AUTH_LOGON_PROOF_Server {}
impl CMD_AUTH_LOGON_PROOF_Server {
    pub(crate) const fn size(&self) -> usize {
        (match self {
            Self::Success {
                ..
            } => {
                1
                + 4 // hardware_survey_id: u32
                + 20 // server_proof: u8[20]
            }
            _ => 1,
        }) // result: CMD_AUTH_LOGON_PROOF_Server
    }
}

impl Default for CMD_AUTH_LOGON_PROOF_Server {
    fn default() -> Self {
        // First enumerator without any fields
        Self::FailUnknown0
    }
}

impl CMD_AUTH_LOGON_PROOF_Server {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Success { .. } => 0,
            Self::FailUnknown0 => 1,
            Self::FailUnknown1 => 2,
            Self::FailBanned => 3,
            Self::FailUnknownAccount => 4,
            Self::FailIncorrectPassword => 5,
            Self::FailAlreadyOnline => 6,
            Self::FailNoTime => 7,
            Self::FailDbBusy => 8,
            Self::FailVersionInvalid => 9,
            Self::LoginDownloadFile => 10,
            Self::FailInvalidServer => 11,
            Self::FailSuspended => 12,
            Self::FailNoAccess => 13,
            Self::SuccessSurvey => 14,
            Self::FailParentalcontrol => 15,
        }
    }

}

impl std::fmt::Display for CMD_AUTH_LOGON_PROOF_Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success{ .. } => f.write_str("Success"),
            Self::FailUnknown0 => f.write_str("FailUnknown0"),
            Self::FailUnknown1 => f.write_str("FailUnknown1"),
            Self::FailBanned => f.write_str("FailBanned"),
            Self::FailUnknownAccount => f.write_str("FailUnknownAccount"),
            Self::FailIncorrectPassword => f.write_str("FailIncorrectPassword"),
            Self::FailAlreadyOnline => f.write_str("FailAlreadyOnline"),
            Self::FailNoTime => f.write_str("FailNoTime"),
            Self::FailDbBusy => f.write_str("FailDbBusy"),
            Self::FailVersionInvalid => f.write_str("FailVersionInvalid"),
            Self::LoginDownloadFile => f.write_str("LoginDownloadFile"),
            Self::FailInvalidServer => f.write_str("FailInvalidServer"),
            Self::FailSuspended => f.write_str("FailSuspended"),
            Self::FailNoAccess => f.write_str("FailNoAccess"),
            Self::SuccessSurvey => f.write_str("SuccessSurvey"),
            Self::FailParentalcontrol => f.write_str("FailParentalcontrol"),
        }
    }
}

#[cfg(test)]
mod test_version_2 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_AUTH_LOGON_PROOF_Server;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_2::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 26] = [ 0x01, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
         0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12,
         0x13, 0xEF, 0xBE, 0xAD, 0xDE, ];

    pub(crate) fn expected0() -> CMD_AUTH_LOGON_PROOF_Server {
        CMD_AUTH_LOGON_PROOF_Server::Success {
            hardware_survey_id: 0xDEADBEEF,
            server_proof: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, ],
        }
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 12.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_logon_proof_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 12.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_logon_proof_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 12.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_logon_proof_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
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
    use super::CMD_AUTH_LOGON_PROOF_Server;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 26] = [ 0x01, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
         0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12,
         0x13, 0xEF, 0xBE, 0xAD, 0xDE, ];

    pub(crate) fn expected0() -> CMD_AUTH_LOGON_PROOF_Server {
        CMD_AUTH_LOGON_PROOF_Server::Success {
            hardware_survey_id: 0xDEADBEEF,
            server_proof: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, ],
        }
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 12.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_logon_proof_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 12.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_logon_proof_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 12.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_logon_proof_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

