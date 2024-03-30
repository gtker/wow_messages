use crate::Message;
use crate::ServerMessage;
use std::io::{Read, Write};

use crate::logon::version_8::LoginResult;

/// Reply to [`CMD_AUTH_RECONNECT_CHALLENGE_Client`](crate::logon::all::CMD_AUTH_RECONNECT_CHALLENGE_Client).
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_server.wowm:35`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_server.wowm#L35):
/// ```text
/// slogin CMD_AUTH_RECONNECT_CHALLENGE_Server = 0x02 {
///     LoginResult result;
///     if (result == SUCCESS) {
///         u8[16] challenge_data;
///         u8[16] checksum_salt;
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CMD_AUTH_RECONNECT_CHALLENGE_Server {
    Success {
        challenge_data: [u8; 16],
        checksum_salt: [u8; 16],
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
    FailLockedEnforced,
}

impl CMD_AUTH_RECONNECT_CHALLENGE_Server {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // result: LoginResult
        w.write_all(&(self.as_int().to_le_bytes()))?;

        match &self {
            CMD_AUTH_RECONNECT_CHALLENGE_Server::Success {
                challenge_data,
                checksum_salt,
            } => {
                // challenge_data: u8[16]
                for i in challenge_data.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

                // checksum_salt: u8[16]
                for i in checksum_salt.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

            }
            _ => {}
        }

        Ok(())
    }
}

impl crate::private::Sealed for CMD_AUTH_RECONNECT_CHALLENGE_Server {}

impl CMD_AUTH_RECONNECT_CHALLENGE_Server {
    #[cfg(feature = "sync")]
    fn read_inner<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // result: LoginResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        let result_if = match result {
            LoginResult::Success => {
                // challenge_data: u8[16]
                let challenge_data = {
                    let mut challenge_data = [0_u8; 16];
                    r.read_exact(&mut challenge_data)?;
                    challenge_data
                };

                // checksum_salt: u8[16]
                let checksum_salt = {
                    let mut checksum_salt = [0_u8; 16];
                    r.read_exact(&mut checksum_salt)?;
                    checksum_salt
                };

                CMD_AUTH_RECONNECT_CHALLENGE_Server::Success {
                    challenge_data,
                    checksum_salt,
                }
            }
            LoginResult::FailUnknown0 => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailUnknown0,
            LoginResult::FailUnknown1 => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailUnknown1,
            LoginResult::FailBanned => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailBanned,
            LoginResult::FailUnknownAccount => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailUnknownAccount,
            LoginResult::FailIncorrectPassword => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailIncorrectPassword,
            LoginResult::FailAlreadyOnline => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailAlreadyOnline,
            LoginResult::FailNoTime => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailNoTime,
            LoginResult::FailDbBusy => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailDbBusy,
            LoginResult::FailVersionInvalid => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailVersionInvalid,
            LoginResult::LoginDownloadFile => CMD_AUTH_RECONNECT_CHALLENGE_Server::LoginDownloadFile,
            LoginResult::FailInvalidServer => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailInvalidServer,
            LoginResult::FailSuspended => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailSuspended,
            LoginResult::FailNoAccess => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailNoAccess,
            LoginResult::SuccessSurvey => CMD_AUTH_RECONNECT_CHALLENGE_Server::SuccessSurvey,
            LoginResult::FailParentalcontrol => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailParentalcontrol,
            LoginResult::FailLockedEnforced => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailLockedEnforced,
        };

        Ok(result_if)
    }

    #[cfg(feature = "tokio")]
    async fn tokio_read_inner<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // result: LoginResult
        let result = crate::util::tokio_read_u8_le(&mut r).await?.try_into()?;

        let result_if = match result {
            LoginResult::Success => {
                // challenge_data: u8[16]
                let challenge_data = {
                    let mut challenge_data = [0_u8; 16];
                    r.read_exact(&mut challenge_data).await?;
                    challenge_data
                };

                // checksum_salt: u8[16]
                let checksum_salt = {
                    let mut checksum_salt = [0_u8; 16];
                    r.read_exact(&mut checksum_salt).await?;
                    checksum_salt
                };

                CMD_AUTH_RECONNECT_CHALLENGE_Server::Success {
                    challenge_data,
                    checksum_salt,
                }
            }
            LoginResult::FailUnknown0 => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailUnknown0,
            LoginResult::FailUnknown1 => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailUnknown1,
            LoginResult::FailBanned => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailBanned,
            LoginResult::FailUnknownAccount => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailUnknownAccount,
            LoginResult::FailIncorrectPassword => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailIncorrectPassword,
            LoginResult::FailAlreadyOnline => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailAlreadyOnline,
            LoginResult::FailNoTime => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailNoTime,
            LoginResult::FailDbBusy => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailDbBusy,
            LoginResult::FailVersionInvalid => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailVersionInvalid,
            LoginResult::LoginDownloadFile => CMD_AUTH_RECONNECT_CHALLENGE_Server::LoginDownloadFile,
            LoginResult::FailInvalidServer => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailInvalidServer,
            LoginResult::FailSuspended => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailSuspended,
            LoginResult::FailNoAccess => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailNoAccess,
            LoginResult::SuccessSurvey => CMD_AUTH_RECONNECT_CHALLENGE_Server::SuccessSurvey,
            LoginResult::FailParentalcontrol => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailParentalcontrol,
            LoginResult::FailLockedEnforced => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailLockedEnforced,
        };

        Ok(result_if)
    }

    #[cfg(feature = "async-std")]
    async fn astd_read_inner<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // result: LoginResult
        let result = crate::util::astd_read_u8_le(&mut r).await?.try_into()?;

        let result_if = match result {
            LoginResult::Success => {
                // challenge_data: u8[16]
                let challenge_data = {
                    let mut challenge_data = [0_u8; 16];
                    r.read_exact(&mut challenge_data).await?;
                    challenge_data
                };

                // checksum_salt: u8[16]
                let checksum_salt = {
                    let mut checksum_salt = [0_u8; 16];
                    r.read_exact(&mut checksum_salt).await?;
                    checksum_salt
                };

                CMD_AUTH_RECONNECT_CHALLENGE_Server::Success {
                    challenge_data,
                    checksum_salt,
                }
            }
            LoginResult::FailUnknown0 => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailUnknown0,
            LoginResult::FailUnknown1 => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailUnknown1,
            LoginResult::FailBanned => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailBanned,
            LoginResult::FailUnknownAccount => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailUnknownAccount,
            LoginResult::FailIncorrectPassword => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailIncorrectPassword,
            LoginResult::FailAlreadyOnline => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailAlreadyOnline,
            LoginResult::FailNoTime => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailNoTime,
            LoginResult::FailDbBusy => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailDbBusy,
            LoginResult::FailVersionInvalid => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailVersionInvalid,
            LoginResult::LoginDownloadFile => CMD_AUTH_RECONNECT_CHALLENGE_Server::LoginDownloadFile,
            LoginResult::FailInvalidServer => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailInvalidServer,
            LoginResult::FailSuspended => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailSuspended,
            LoginResult::FailNoAccess => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailNoAccess,
            LoginResult::SuccessSurvey => CMD_AUTH_RECONNECT_CHALLENGE_Server::SuccessSurvey,
            LoginResult::FailParentalcontrol => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailParentalcontrol,
            LoginResult::FailLockedEnforced => CMD_AUTH_RECONNECT_CHALLENGE_Server::FailLockedEnforced,
        };

        Ok(result_if)
    }

}

impl Message for CMD_AUTH_RECONNECT_CHALLENGE_Server {
    const OPCODE: u8 = 0x02;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read, I: crate::private::Sealed>(r: R) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r).map_err(|kind| crate::errors::ParseError::new(2, "CMD_AUTH_RECONNECT_CHALLENGE_Server", kind))
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
        Box::pin(async move {Self::tokio_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(2, "CMD_AUTH_RECONNECT_CHALLENGE_Server", kind))})
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
        Box::pin(async move {Self::astd_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(2, "CMD_AUTH_RECONNECT_CHALLENGE_Server", kind))})
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

impl ServerMessage for CMD_AUTH_RECONNECT_CHALLENGE_Server {}
impl CMD_AUTH_RECONNECT_CHALLENGE_Server {
    pub(crate) const fn size(&self) -> usize {
        (match self {
            Self::Success {
                ..
            } => {
                1
                + 16 // challenge_data: u8[16]
                + 16 // checksum_salt: u8[16]
            }
            _ => 1,
        }) // result: CMD_AUTH_RECONNECT_CHALLENGE_Server
    }
}

impl Default for CMD_AUTH_RECONNECT_CHALLENGE_Server {
    fn default() -> Self {
        // First enumerator without any fields
        Self::FailUnknown0
    }
}

impl CMD_AUTH_RECONNECT_CHALLENGE_Server {
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
            Self::FailLockedEnforced => 16,
        }
    }

}

impl std::fmt::Display for CMD_AUTH_RECONNECT_CHALLENGE_Server {
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
            Self::FailLockedEnforced => f.write_str("FailLockedEnforced"),
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_AUTH_RECONNECT_CHALLENGE_Server;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_8::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 34] = [ 0x02, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
         0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0xFF, 0xFE, 0xFD,
         0xFC, 0xFB, 0xFA, 0xF9, 0xF8, 0xF7, 0xF6, 0xF5, 0xF4, 0xF3, 0xF2, 0xF1,
         0xF0, ];

    pub(crate) fn expected0() -> CMD_AUTH_RECONNECT_CHALLENGE_Server {
        CMD_AUTH_RECONNECT_CHALLENGE_Server::Success {
            challenge_data: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
            checksum_salt: [ 0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0xFA, 0xF9, 0xF8, 0xF7,
                 0xF6, 0xF5, 0xF4, 0xF3, 0xF2, 0xF1, 0xF0, ],
        }
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_server.wowm` line 45.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_reconnect_challenge_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_server.wowm` line 45.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_reconnect_challenge_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_server.wowm` line 45.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_reconnect_challenge_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 2] = [ 0x02, 0x03, ];

    pub(crate) fn expected1() -> CMD_AUTH_RECONNECT_CHALLENGE_Server {
        CMD_AUTH_RECONNECT_CHALLENGE_Server::FailBanned
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_server.wowm` line 58.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_reconnect_challenge_server1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_server.wowm` line 58.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_reconnect_challenge_server1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_server.wowm` line 58.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_reconnect_challenge_server1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}

