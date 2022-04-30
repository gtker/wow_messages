use std::convert::{TryFrom, TryInto};
use crate::logon::version_8::{AccountFlag};
use crate::logon::version_8::{LoginResult, LoginResultError};
use crate::ServerMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMD_AUTH_LOGON_PROOF_Server {
    pub login_result: CMD_AUTH_LOGON_PROOF_ServerLoginResult,
}

impl ServerMessage for CMD_AUTH_LOGON_PROOF_Server {
    const OPCODE: u8 = 0x01;
}
impl CMD_AUTH_LOGON_PROOF_Server {
    pub const PADDING_VALUE: u16 = 0x00;

}

impl ReadableAndWritable for CMD_AUTH_LOGON_PROOF_Server {
    type Error = CMD_AUTH_LOGON_PROOF_ServerError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // login_result: LoginResult
        let login_result = LoginResult::read(r)?;

        let login_result_if = match login_result {
            LoginResult::SUCCESS => {
                // server_proof: u8[20]
                let mut server_proof = [0_u8; 20];
                r.read_exact(&mut server_proof)?;

                // account_flag: AccountFlag
                let account_flag = AccountFlag::read(r)?;

                // hardware_survey_id: u32
                let hardware_survey_id = crate::util::read_u32_le(r)?;

                // unknown_flags: u16
                let unknown_flags = crate::util::read_u16_le(r)?;

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::SUCCESS {
                    server_proof,
                    account_flag,
                    hardware_survey_id,
                    unknown_flags,
                }
            }
            LoginResult::FAIL_UNKNOWN0 => {
                // padding: u16
                let _padding = crate::util::read_u16_le(r)?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN0 {
                }
            }
            LoginResult::FAIL_UNKNOWN1 => {
                // padding: u16
                let _padding = crate::util::read_u16_le(r)?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN1 {
                }
            }
            LoginResult::FAIL_BANNED => {
                // padding: u16
                let _padding = crate::util::read_u16_le(r)?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_BANNED {
                }
            }
            LoginResult::FAIL_UNKNOWN_ACCOUNT => {
                // padding: u16
                let _padding = crate::util::read_u16_le(r)?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT {
                }
            }
            LoginResult::FAIL_INCORRECT_PASSWORD => {
                // padding: u16
                let _padding = crate::util::read_u16_le(r)?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_INCORRECT_PASSWORD {
                }
            }
            LoginResult::FAIL_ALREADY_ONLINE => {
                // padding: u16
                let _padding = crate::util::read_u16_le(r)?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_ALREADY_ONLINE {
                }
            }
            LoginResult::FAIL_NO_TIME => {
                // padding: u16
                let _padding = crate::util::read_u16_le(r)?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_TIME {
                }
            }
            LoginResult::FAIL_DB_BUSY => {
                // padding: u16
                let _padding = crate::util::read_u16_le(r)?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_DB_BUSY {
                }
            }
            LoginResult::FAIL_VERSION_INVALID => {
                // padding: u16
                let _padding = crate::util::read_u16_le(r)?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_VERSION_INVALID {
                }
            }
            LoginResult::LOGIN_DOWNLOAD_FILE => {
                // padding: u16
                let _padding = crate::util::read_u16_le(r)?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::LOGIN_DOWNLOAD_FILE {
                }
            }
            LoginResult::FAIL_INVALID_SERVER => {
                // padding: u16
                let _padding = crate::util::read_u16_le(r)?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_INVALID_SERVER {
                }
            }
            LoginResult::FAIL_SUSPENDED => {
                // padding: u16
                let _padding = crate::util::read_u16_le(r)?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_SUSPENDED {
                }
            }
            LoginResult::FAIL_NO_ACCESS => {
                // padding: u16
                let _padding = crate::util::read_u16_le(r)?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_ACCESS {
                }
            }
            LoginResult::SUCCESS_SURVEY => {
                // padding: u16
                let _padding = crate::util::read_u16_le(r)?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::SUCCESS_SURVEY {
                }
            }
            LoginResult::FAIL_PARENTALCONTROL => {
                // padding: u16
                let _padding = crate::util::read_u16_le(r)?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_PARENTALCONTROL {
                }
            }
            LoginResult::FAIL_LOCKED_ENFORCED => {
                // padding: u16
                let _padding = crate::util::read_u16_le(r)?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_LOCKED_ENFORCED {
                }
            }
        };

        Ok(Self {
            login_result: login_result_if,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // login_result: LoginResult
        self.login_result.write(w)?;

        match &self.login_result {
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::SUCCESS {
                server_proof,
                account_flag,
                hardware_survey_id,
                unknown_flags,
            } => {
                // server_proof: u8[20]
                for i in server_proof.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

                // account_flag: AccountFlag
                account_flag.write(w)?;

                // hardware_survey_id: u32
                w.write_all(&hardware_survey_id.to_le_bytes())?;

                // unknown_flags: u16
                w.write_all(&unknown_flags.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN0 {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN1 {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_BANNED {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_INCORRECT_PASSWORD {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_ALREADY_ONLINE {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_TIME {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_DB_BUSY {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_VERSION_INVALID {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::LOGIN_DOWNLOAD_FILE {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_INVALID_SERVER {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_SUSPENDED {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_ACCESS {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::SUCCESS_SURVEY {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_PARENTALCONTROL {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_LOCKED_ENFORCED {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

            }
        }

        Ok(())
    }

}

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
#[async_trait]
impl AsyncReadWrite for CMD_AUTH_LOGON_PROOF_Server {
    type Error = CMD_AUTH_LOGON_PROOF_ServerError;

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // login_result: LoginResult
        let login_result = LoginResult::tokio_read(r).await?;

        let login_result_if = match login_result {
            LoginResult::SUCCESS => {
                // server_proof: u8[20]
                let mut server_proof = [0_u8; 20];
                r.read_exact(&mut server_proof).await?;

                // account_flag: AccountFlag
                let account_flag = AccountFlag::tokio_read(r).await?;

                // hardware_survey_id: u32
                let hardware_survey_id = crate::util::tokio_read_u32_le(r).await?;

                // unknown_flags: u16
                let unknown_flags = crate::util::tokio_read_u16_le(r).await?;

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::SUCCESS {
                    server_proof,
                    account_flag,
                    hardware_survey_id,
                    unknown_flags,
                }
            }
            LoginResult::FAIL_UNKNOWN0 => {
                // padding: u16
                let _padding = crate::util::tokio_read_u16_le(r).await?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN0 {
                }
            }
            LoginResult::FAIL_UNKNOWN1 => {
                // padding: u16
                let _padding = crate::util::tokio_read_u16_le(r).await?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN1 {
                }
            }
            LoginResult::FAIL_BANNED => {
                // padding: u16
                let _padding = crate::util::tokio_read_u16_le(r).await?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_BANNED {
                }
            }
            LoginResult::FAIL_UNKNOWN_ACCOUNT => {
                // padding: u16
                let _padding = crate::util::tokio_read_u16_le(r).await?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT {
                }
            }
            LoginResult::FAIL_INCORRECT_PASSWORD => {
                // padding: u16
                let _padding = crate::util::tokio_read_u16_le(r).await?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_INCORRECT_PASSWORD {
                }
            }
            LoginResult::FAIL_ALREADY_ONLINE => {
                // padding: u16
                let _padding = crate::util::tokio_read_u16_le(r).await?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_ALREADY_ONLINE {
                }
            }
            LoginResult::FAIL_NO_TIME => {
                // padding: u16
                let _padding = crate::util::tokio_read_u16_le(r).await?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_TIME {
                }
            }
            LoginResult::FAIL_DB_BUSY => {
                // padding: u16
                let _padding = crate::util::tokio_read_u16_le(r).await?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_DB_BUSY {
                }
            }
            LoginResult::FAIL_VERSION_INVALID => {
                // padding: u16
                let _padding = crate::util::tokio_read_u16_le(r).await?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_VERSION_INVALID {
                }
            }
            LoginResult::LOGIN_DOWNLOAD_FILE => {
                // padding: u16
                let _padding = crate::util::tokio_read_u16_le(r).await?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::LOGIN_DOWNLOAD_FILE {
                }
            }
            LoginResult::FAIL_INVALID_SERVER => {
                // padding: u16
                let _padding = crate::util::tokio_read_u16_le(r).await?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_INVALID_SERVER {
                }
            }
            LoginResult::FAIL_SUSPENDED => {
                // padding: u16
                let _padding = crate::util::tokio_read_u16_le(r).await?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_SUSPENDED {
                }
            }
            LoginResult::FAIL_NO_ACCESS => {
                // padding: u16
                let _padding = crate::util::tokio_read_u16_le(r).await?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_ACCESS {
                }
            }
            LoginResult::SUCCESS_SURVEY => {
                // padding: u16
                let _padding = crate::util::tokio_read_u16_le(r).await?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::SUCCESS_SURVEY {
                }
            }
            LoginResult::FAIL_PARENTALCONTROL => {
                // padding: u16
                let _padding = crate::util::tokio_read_u16_le(r).await?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_PARENTALCONTROL {
                }
            }
            LoginResult::FAIL_LOCKED_ENFORCED => {
                // padding: u16
                let _padding = crate::util::tokio_read_u16_le(r).await?;
                // padding is expected to always be 0 (0)

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_LOCKED_ENFORCED {
                }
            }
        };

        Ok(Self {
            login_result: login_result_if,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes()).await?;

        // login_result: LoginResult
        self.login_result.tokio_write(w).await?;

        match &self.login_result {
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::SUCCESS {
                server_proof,
                account_flag,
                hardware_survey_id,
                unknown_flags,
            } => {
                // server_proof: u8[20]
                for i in server_proof.iter() {
                    w.write_all(&i.to_le_bytes()).await?;
                }

                // account_flag: AccountFlag
                account_flag.tokio_write(w).await?;

                // hardware_survey_id: u32
                w.write_all(&hardware_survey_id.to_le_bytes()).await?;

                // unknown_flags: u16
                w.write_all(&unknown_flags.to_le_bytes()).await?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN0 {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN1 {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_BANNED {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_INCORRECT_PASSWORD {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_ALREADY_ONLINE {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_TIME {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_DB_BUSY {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_VERSION_INVALID {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::LOGIN_DOWNLOAD_FILE {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_INVALID_SERVER {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_SUSPENDED {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_ACCESS {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::SUCCESS_SURVEY {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_PARENTALCONTROL {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

            }
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_LOCKED_ENFORCED {
            } => {
                // padding: u16
                w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

            }
        }

        Ok(())
    }

}

impl VariableSized for CMD_AUTH_LOGON_PROOF_Server {
    fn size(&self) -> usize {
        self.login_result.size() // login_result: LoginResult and subfields
    }
}

impl MaximumPossibleSized for CMD_AUTH_LOGON_PROOF_Server {
    fn maximum_possible_size() -> usize {
        LoginResult::maximum_possible_size() // login_result: LoginResult
    }
}

#[derive(Debug)]
pub enum CMD_AUTH_LOGON_PROOF_ServerError {
    Io(std::io::Error),
    LoginResult(LoginResultError),
}

impl std::error::Error for CMD_AUTH_LOGON_PROOF_ServerError {}
impl std::fmt::Display for CMD_AUTH_LOGON_PROOF_ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::LoginResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMD_AUTH_LOGON_PROOF_ServerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<LoginResultError> for CMD_AUTH_LOGON_PROOF_ServerError {
    fn from(e: LoginResultError) -> Self {
        Self::LoginResult(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CMD_AUTH_LOGON_PROOF_ServerLoginResult {
    SUCCESS {
        server_proof: [u8; 20],
        account_flag: AccountFlag,
        hardware_survey_id: u32,
        unknown_flags: u16,
    },
    FAIL_UNKNOWN0,
    FAIL_UNKNOWN1,
    FAIL_BANNED,
    FAIL_UNKNOWN_ACCOUNT,
    FAIL_INCORRECT_PASSWORD,
    FAIL_ALREADY_ONLINE,
    FAIL_NO_TIME,
    FAIL_DB_BUSY,
    FAIL_VERSION_INVALID,
    LOGIN_DOWNLOAD_FILE,
    FAIL_INVALID_SERVER,
    FAIL_SUSPENDED,
    FAIL_NO_ACCESS,
    SUCCESS_SURVEY,
    FAIL_PARENTALCONTROL,
    FAIL_LOCKED_ENFORCED,
}

impl From<&LoginResult> for CMD_AUTH_LOGON_PROOF_ServerLoginResult {
    fn from(e: &LoginResult) -> Self {
        match &e {
            LoginResult::SUCCESS => Self::SUCCESS {
                server_proof: Default::default(),
                account_flag: Default::default(),
                hardware_survey_id: Default::default(),
                unknown_flags: Default::default(),
            },
            LoginResult::FAIL_UNKNOWN0 => Self::FAIL_UNKNOWN0,
            LoginResult::FAIL_UNKNOWN1 => Self::FAIL_UNKNOWN1,
            LoginResult::FAIL_BANNED => Self::FAIL_BANNED,
            LoginResult::FAIL_UNKNOWN_ACCOUNT => Self::FAIL_UNKNOWN_ACCOUNT,
            LoginResult::FAIL_INCORRECT_PASSWORD => Self::FAIL_INCORRECT_PASSWORD,
            LoginResult::FAIL_ALREADY_ONLINE => Self::FAIL_ALREADY_ONLINE,
            LoginResult::FAIL_NO_TIME => Self::FAIL_NO_TIME,
            LoginResult::FAIL_DB_BUSY => Self::FAIL_DB_BUSY,
            LoginResult::FAIL_VERSION_INVALID => Self::FAIL_VERSION_INVALID,
            LoginResult::LOGIN_DOWNLOAD_FILE => Self::LOGIN_DOWNLOAD_FILE,
            LoginResult::FAIL_INVALID_SERVER => Self::FAIL_INVALID_SERVER,
            LoginResult::FAIL_SUSPENDED => Self::FAIL_SUSPENDED,
            LoginResult::FAIL_NO_ACCESS => Self::FAIL_NO_ACCESS,
            LoginResult::SUCCESS_SURVEY => Self::SUCCESS_SURVEY,
            LoginResult::FAIL_PARENTALCONTROL => Self::FAIL_PARENTALCONTROL,
            LoginResult::FAIL_LOCKED_ENFORCED => Self::FAIL_LOCKED_ENFORCED,
        }
    }
}

impl From<&CMD_AUTH_LOGON_PROOF_ServerLoginResult> for LoginResult {
    fn from(v: &CMD_AUTH_LOGON_PROOF_ServerLoginResult) -> Self {
        match &v {
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::SUCCESS { .. } => Self::SUCCESS,
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN0 => Self::FAIL_UNKNOWN0,
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN1 => Self::FAIL_UNKNOWN1,
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_BANNED => Self::FAIL_BANNED,
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT => Self::FAIL_UNKNOWN_ACCOUNT,
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_INCORRECT_PASSWORD => Self::FAIL_INCORRECT_PASSWORD,
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_ALREADY_ONLINE => Self::FAIL_ALREADY_ONLINE,
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_TIME => Self::FAIL_NO_TIME,
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_DB_BUSY => Self::FAIL_DB_BUSY,
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_VERSION_INVALID => Self::FAIL_VERSION_INVALID,
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::LOGIN_DOWNLOAD_FILE => Self::LOGIN_DOWNLOAD_FILE,
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_INVALID_SERVER => Self::FAIL_INVALID_SERVER,
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_SUSPENDED => Self::FAIL_SUSPENDED,
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_ACCESS => Self::FAIL_NO_ACCESS,
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::SUCCESS_SURVEY => Self::SUCCESS_SURVEY,
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_PARENTALCONTROL => Self::FAIL_PARENTALCONTROL,
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_LOCKED_ENFORCED => Self::FAIL_LOCKED_ENFORCED,
        }
    }
}

impl Default for CMD_AUTH_LOGON_PROOF_ServerLoginResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::SUCCESS {
            server_proof: Default::default(),
            account_flag: Default::default(),
            hardware_survey_id: Default::default(),
            unknown_flags: Default::default(),
        }
    }
}

impl CMD_AUTH_LOGON_PROOF_ServerLoginResult {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write_u16_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.tokio_write_u16_le(w).await
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write_u16_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.tokio_write_u16_be(w).await
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write_u32_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.tokio_write_u32_le(w).await
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write_u32_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.tokio_write_u32_be(w).await
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write_u64_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.tokio_write_u64_le(w).await
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write_u64_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.tokio_write_u64_be(w).await
    }

}

impl VariableSized for CMD_AUTH_LOGON_PROOF_ServerLoginResult {
    fn size(&self) -> usize {
        match self {
            Self::SUCCESS  {
                server_proof,
                account_flag,
                hardware_survey_id,
                unknown_flags,
            } => {
                1
                + 20 * core::mem::size_of::<u8>() // server_proof: u8[20]
                + AccountFlag::size() // account_flag: AccountFlag
                + 4 // hardware_survey_id: u32
                + 2 // unknown_flags: u16
            }
            Self::FAIL_UNKNOWN0 =>  {
                1
                + 2 // padding: u16
            }
            Self::FAIL_UNKNOWN1 =>  {
                1
                + 2 // padding: u16
            }
            Self::FAIL_BANNED =>  {
                1
                + 2 // padding: u16
            }
            Self::FAIL_UNKNOWN_ACCOUNT =>  {
                1
                + 2 // padding: u16
            }
            Self::FAIL_INCORRECT_PASSWORD =>  {
                1
                + 2 // padding: u16
            }
            Self::FAIL_ALREADY_ONLINE =>  {
                1
                + 2 // padding: u16
            }
            Self::FAIL_NO_TIME =>  {
                1
                + 2 // padding: u16
            }
            Self::FAIL_DB_BUSY =>  {
                1
                + 2 // padding: u16
            }
            Self::FAIL_VERSION_INVALID =>  {
                1
                + 2 // padding: u16
            }
            Self::LOGIN_DOWNLOAD_FILE =>  {
                1
                + 2 // padding: u16
            }
            Self::FAIL_INVALID_SERVER =>  {
                1
                + 2 // padding: u16
            }
            Self::FAIL_SUSPENDED =>  {
                1
                + 2 // padding: u16
            }
            Self::FAIL_NO_ACCESS =>  {
                1
                + 2 // padding: u16
            }
            Self::SUCCESS_SURVEY =>  {
                1
                + 2 // padding: u16
            }
            Self::FAIL_PARENTALCONTROL =>  {
                1
                + 2 // padding: u16
            }
            Self::FAIL_LOCKED_ENFORCED =>  {
                1
                + 2 // padding: u16
            }
        }
    }
}

impl MaximumPossibleSized for CMD_AUTH_LOGON_PROOF_ServerLoginResult {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMD_AUTH_LOGON_PROOF_Server;
    use crate::VariableSized;
    use crate::logon::version_8::AccountFlag;
    use crate::logon::version_8::LoginResult;
    use super::*;
    use super::super::*;
    use crate::logon::version_8::opcodes::ServerOpcodeMessage;

    #[test]
    fn CMD_AUTH_LOGON_PROOF_Server0() {
        let raw: Vec<u8> = vec![ 0x01, 0x07, 0x00, 0x00, ];

        let expected = CMD_AUTH_LOGON_PROOF_Server {
            login_result: CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_TIME,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[test]
    fn CMD_AUTH_LOGON_PROOF_Server1() {
        let raw: Vec<u8> = vec![ 0x01, 0x08, 0x00, 0x00, ];

        let expected = CMD_AUTH_LOGON_PROOF_Server {
            login_result: CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_DB_BUSY,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
