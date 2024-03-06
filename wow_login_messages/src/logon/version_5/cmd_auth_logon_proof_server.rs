use crate::Message;
use crate::ServerMessage;
use std::io::{Read, Write};

use crate::logon::version_2::LoginResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Reply to [`CMD_AUTH_LOGON_PROOF_Client`](crate::logon::version_5::CMD_AUTH_LOGON_PROOF_Client).
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm:35`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm#L35):
/// ```text
/// slogin CMD_AUTH_LOGON_PROOF_Server = 0x01 {
///     LoginResult result;
///     if (result == SUCCESS) {
///         u8[20] server_proof;
///         u32 hardware_survey_id;
///         u16 unknown;
///     }
/// }
/// ```
pub struct CMD_AUTH_LOGON_PROOF_Server {
    pub result: CMD_AUTH_LOGON_PROOF_Server_LoginResult,
}

impl CMD_AUTH_LOGON_PROOF_Server {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // result: LoginResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        match &self.result {
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                hardware_survey_id,
                server_proof,
                unknown,
            } => {
                // server_proof: u8[20]
                for i in server_proof.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

                // hardware_survey_id: u32
                w.write_all(&hardware_survey_id.to_le_bytes())?;

                // unknown: u16
                w.write_all(&unknown.to_le_bytes())?;

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

                // unknown: u16
                let unknown = crate::util::read_u16_le(&mut r)?;

                CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                    hardware_survey_id,
                    server_proof,
                    unknown,
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
        };

        Ok(Self {
            result: result_if,
        })
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

                // unknown: u16
                let unknown = crate::util::tokio_read_u16_le(&mut r).await?;

                CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                    hardware_survey_id,
                    server_proof,
                    unknown,
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
        };

        Ok(Self {
            result: result_if,
        })
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

                // unknown: u16
                let unknown = crate::util::astd_read_u16_le(&mut r).await?;

                CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                    hardware_survey_id,
                    server_proof,
                    unknown,
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
        };

        Ok(Self {
            result: result_if,
        })
    }

}

impl Message for CMD_AUTH_LOGON_PROOF_Server {
    const OPCODE: u8 = 0x01;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;

        let mut s = String::new();

        writeln!(s, "test CMD_AUTH_LOGON_PROOF_Server {{").unwrap();
        // Members
        writeln!(s, "    result = {};", LoginResult::try_from(self.result.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.result {
            crate::logon::version_5::CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                hardware_survey_id,
                server_proof,
                unknown,
            } => {
                writeln!(s, "    server_proof = [").unwrap();
                for v in server_proof.as_slice() {
                    write!(s, "{v:#04X}, ").unwrap();
                }
                writeln!(s, "    ];").unwrap();
                writeln!(s, "    hardware_survey_id = {};", hardware_survey_id).unwrap();
                writeln!(s, "    unknown = {};", unknown).unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        writeln!(s, "    {:#04X}, /* opcode */ ", bytes.next().unwrap()).unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 1, "result", "    ");
        match &self.result {
            crate::logon::version_5::CMD_AUTH_LOGON_PROOF_Server_LoginResult::Success {
                hardware_survey_id,
                server_proof,
                unknown,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, server_proof.len(), "server_proof", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "hardware_survey_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 2, "unknown", "    ");
            }
            _ => {}
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    login_versions = \"{}\";", std::env::var("WOWM_TEST_CASE_LOGIN_VERSION").unwrap_or("5 6 7".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

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
        self.result.size() // result: CMD_AUTH_LOGON_PROOF_Server_LoginResult
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CMD_AUTH_LOGON_PROOF_Server_LoginResult {
    Success {
        hardware_survey_id: u32,
        server_proof: [u8; 20],
        unknown: u16,
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
        }
    }

}

impl std::fmt::Display for CMD_AUTH_LOGON_PROOF_Server_LoginResult {
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

impl CMD_AUTH_LOGON_PROOF_Server_LoginResult {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Success {
                ..
            } => {
                1
                + 4 // hardware_survey_id: u32
                + 20 // server_proof: u8[20]
                + 2 // unknown: u16
            }
            _ => 1,
        }
    }
}

