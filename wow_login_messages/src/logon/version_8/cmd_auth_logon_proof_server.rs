use crate::ServerMessage;
use std::io::{Read, Write};

use crate::logon::version_8::{
    AccountFlag, LoginResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm:36`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm#L36):
/// ```text
/// slogin CMD_AUTH_LOGON_PROOF_Server = 0x01 {
///     LoginResult result;
///     if (result == SUCCESS) {
///         u8[20] server_proof;
///         AccountFlag account_flag;
///         u32 hardware_survey_id;
///         u16 unknown_flags;
///     }
///     else {
///         u16 padding = 0;
///     }
/// }
/// ```
pub struct CMD_AUTH_LOGON_PROOF_Server {
    pub result: CMD_AUTH_LOGON_PROOF_Server_LoginResult,
}

impl CMD_AUTH_LOGON_PROOF_Server {
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

impl CMD_AUTH_LOGON_PROOF_Server {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // result: LoginResult
        w.write_all(&u8::from(self.result.as_int()).to_le_bytes())?;

        match &self.result {
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                account_flag,
                hardware_survey_id,
                server_proof,
                unknown_flags,
            } => {
                // server_proof: u8[20]
                for i in server_proof.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

                // account_flag: AccountFlag
                w.write_all(&u32::from(account_flag.as_int()).to_le_bytes())?;

                // hardware_survey_id: u32
                w.write_all(&hardware_survey_id.to_le_bytes())?;

                // unknown_flags: u16
                w.write_all(&unknown_flags.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknown0 {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknown1 {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailBanned {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknownAccount {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailIncorrectPassword {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailAlreadyOnline {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailNoTime {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailDbBusy {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailVersionInvalid {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::LoginDownloadFile {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailInvalidServer {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailSuspended {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailNoAccess {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::SuccessSurvey {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailParentalcontrol {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailLockedEnforced {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
        }

        Ok(())
    }
}

impl ServerMessage for CMD_AUTH_LOGON_PROOF_Server {
    const OPCODE: u8 = 0x01;

    fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // result: LoginResult
        let result: LoginResult = crate::util::read_u8_le(&mut r)?.try_into()?;

        let result_if = match result {
            LoginResult::Success => {
                // server_proof: u8[20]
                let server_proof = {
                    let mut server_proof = [0_u8; 20];
                    r.read_exact(&mut server_proof)?;
                    server_proof
                };

                // account_flag: AccountFlag
                let account_flag = AccountFlag::new(crate::util::read_u32_le(&mut r)?);

                // hardware_survey_id: u32
                let hardware_survey_id = crate::util::read_u32_le(&mut r)?;

                // unknown_flags: u16
                let unknown_flags = crate::util::read_u16_le(&mut r)?;

                CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                    account_flag,
                    hardware_survey_id,
                    server_proof,
                    unknown_flags,
                }
            }
            LoginResult::FailUnknown0 => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknown0,
            LoginResult::FailUnknown1 => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknown1,
            LoginResult::FailBanned => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailBanned,
            LoginResult::FailUnknownAccount => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknownAccount,
            LoginResult::FailIncorrectPassword => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailIncorrectPassword,
            LoginResult::FailAlreadyOnline => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailAlreadyOnline,
            LoginResult::FailNoTime => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailNoTime,
            LoginResult::FailDbBusy => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailDbBusy,
            LoginResult::FailVersionInvalid => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailVersionInvalid,
            LoginResult::LoginDownloadFile => CMD_AUTH_LOGON_PROOF_Server_LoginResult::LoginDownloadFile,
            LoginResult::FailInvalidServer => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailInvalidServer,
            LoginResult::FailSuspended => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailSuspended,
            LoginResult::FailNoAccess => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailNoAccess,
            LoginResult::SuccessSurvey => CMD_AUTH_LOGON_PROOF_Server_LoginResult::SuccessSurvey,
            LoginResult::FailParentalcontrol => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailParentalcontrol,
            LoginResult::FailLockedEnforced => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailLockedEnforced,
        };

        Ok(Self {
            result: result_if,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, mut w: W) -> std::result::Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(self.size() + 1);
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

            let result_if = match result {
                LoginResult::Success => {
                    // server_proof: u8[20]
                    let server_proof = {
                        let mut server_proof = [0_u8; 20];
                        r.read_exact(&mut server_proof).await?;
                        server_proof
                    };

                    // account_flag: AccountFlag
                    let account_flag = AccountFlag::new(crate::util::tokio_read_u32_le(&mut r).await?);

                    // hardware_survey_id: u32
                    let hardware_survey_id = crate::util::tokio_read_u32_le(&mut r).await?;

                    // unknown_flags: u16
                    let unknown_flags = crate::util::tokio_read_u16_le(&mut r).await?;

                    CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                        account_flag,
                        hardware_survey_id,
                        server_proof,
                        unknown_flags,
                    }
                }
                LoginResult::FailUnknown0 => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknown0,
                LoginResult::FailUnknown1 => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknown1,
                LoginResult::FailBanned => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailBanned,
                LoginResult::FailUnknownAccount => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknownAccount,
                LoginResult::FailIncorrectPassword => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailIncorrectPassword,
                LoginResult::FailAlreadyOnline => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailAlreadyOnline,
                LoginResult::FailNoTime => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailNoTime,
                LoginResult::FailDbBusy => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailDbBusy,
                LoginResult::FailVersionInvalid => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailVersionInvalid,
                LoginResult::LoginDownloadFile => CMD_AUTH_LOGON_PROOF_Server_LoginResult::LoginDownloadFile,
                LoginResult::FailInvalidServer => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailInvalidServer,
                LoginResult::FailSuspended => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailSuspended,
                LoginResult::FailNoAccess => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailNoAccess,
                LoginResult::SuccessSurvey => CMD_AUTH_LOGON_PROOF_Server_LoginResult::SuccessSurvey,
                LoginResult::FailParentalcontrol => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailParentalcontrol,
                LoginResult::FailLockedEnforced => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailLockedEnforced,
            };

            Ok(Self {
                result: result_if,
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
            let mut v = Vec::with_capacity(self.size() + 1);
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

            let result_if = match result {
                LoginResult::Success => {
                    // server_proof: u8[20]
                    let server_proof = {
                        let mut server_proof = [0_u8; 20];
                        r.read_exact(&mut server_proof).await?;
                        server_proof
                    };

                    // account_flag: AccountFlag
                    let account_flag = AccountFlag::new(crate::util::astd_read_u32_le(&mut r).await?);

                    // hardware_survey_id: u32
                    let hardware_survey_id = crate::util::astd_read_u32_le(&mut r).await?;

                    // unknown_flags: u16
                    let unknown_flags = crate::util::astd_read_u16_le(&mut r).await?;

                    CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                        account_flag,
                        hardware_survey_id,
                        server_proof,
                        unknown_flags,
                    }
                }
                LoginResult::FailUnknown0 => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknown0,
                LoginResult::FailUnknown1 => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknown1,
                LoginResult::FailBanned => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailBanned,
                LoginResult::FailUnknownAccount => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailUnknownAccount,
                LoginResult::FailIncorrectPassword => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailIncorrectPassword,
                LoginResult::FailAlreadyOnline => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailAlreadyOnline,
                LoginResult::FailNoTime => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailNoTime,
                LoginResult::FailDbBusy => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailDbBusy,
                LoginResult::FailVersionInvalid => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailVersionInvalid,
                LoginResult::LoginDownloadFile => CMD_AUTH_LOGON_PROOF_Server_LoginResult::LoginDownloadFile,
                LoginResult::FailInvalidServer => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailInvalidServer,
                LoginResult::FailSuspended => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailSuspended,
                LoginResult::FailNoAccess => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailNoAccess,
                LoginResult::SuccessSurvey => CMD_AUTH_LOGON_PROOF_Server_LoginResult::SuccessSurvey,
                LoginResult::FailParentalcontrol => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailParentalcontrol,
                LoginResult::FailLockedEnforced => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailLockedEnforced,
            };

            Ok(Self {
                result: result_if,
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
            let mut v = Vec::with_capacity(self.size() + 1);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

}

impl CMD_AUTH_LOGON_PROOF_Server {
    pub(crate) fn size(&self) -> usize {
        self.result.size() // result: CMD_AUTH_LOGON_PROOF_Server_LoginResult
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CMD_AUTH_LOGON_PROOF_Server_LoginResult {
    Success {
        account_flag: AccountFlag,
        hardware_survey_id: u32,
        server_proof: [u8; 20],
        unknown_flags: u16,
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

impl Default for CMD_AUTH_LOGON_PROOF_Server_LoginResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::FailUnknown0
    }
}

impl CMD_AUTH_LOGON_PROOF_Server_LoginResult {
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

impl CMD_AUTH_LOGON_PROOF_Server_LoginResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Success {
                account_flag,
                hardware_survey_id,
                server_proof,
                unknown_flags,
            } => {
                1
                + 4 // account_flag: AccountFlag
                + 4 // hardware_survey_id: u32
                + 20 // server_proof: u8[20]
                + 2 // unknown_flags: u16
            }
            Self::FailUnknown0 => {
                1
                + 2 // padding: u16
            }
            Self::FailUnknown1 => {
                1
                + 2 // padding: u16
            }
            Self::FailBanned => {
                1
                + 2 // padding: u16
            }
            Self::FailUnknownAccount => {
                1
                + 2 // padding: u16
            }
            Self::FailIncorrectPassword => {
                1
                + 2 // padding: u16
            }
            Self::FailAlreadyOnline => {
                1
                + 2 // padding: u16
            }
            Self::FailNoTime => {
                1
                + 2 // padding: u16
            }
            Self::FailDbBusy => {
                1
                + 2 // padding: u16
            }
            Self::FailVersionInvalid => {
                1
                + 2 // padding: u16
            }
            Self::LoginDownloadFile => {
                1
                + 2 // padding: u16
            }
            Self::FailInvalidServer => {
                1
                + 2 // padding: u16
            }
            Self::FailSuspended => {
                1
                + 2 // padding: u16
            }
            Self::FailNoAccess => {
                1
                + 2 // padding: u16
            }
            Self::SuccessSurvey => {
                1
                + 2 // padding: u16
            }
            Self::FailParentalcontrol => {
                1
                + 2 // padding: u16
            }
            Self::FailLockedEnforced => {
                1
                + 2 // padding: u16
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::CMD_AUTH_LOGON_PROOF_Server;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_8::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    fn assert(t: &CMD_AUTH_LOGON_PROOF_Server, expected: &CMD_AUTH_LOGON_PROOF_Server) {
        assert_eq!(t.result, expected.result);
    }

    const RAW0: [u8; 4] = [ 0x01, 0x07, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMD_AUTH_LOGON_PROOF_Server {
        CMD_AUTH_LOGON_PROOF_Server {
            result: CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailNoTime,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 50.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_PROOF_Server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 50.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_PROOF_Server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 50.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_PROOF_Server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 4] = [ 0x01, 0x08, 0x00, 0x00, ];

    pub(crate) fn expected1() -> CMD_AUTH_LOGON_PROOF_Server {
        CMD_AUTH_LOGON_PROOF_Server {
            result: CMD_AUTH_LOGON_PROOF_Server_LoginResult::FailDbBusy,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 60.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_PROOF_Server1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 60.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_PROOF_Server1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 60.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_PROOF_Server1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}

