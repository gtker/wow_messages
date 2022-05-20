use std::convert::{TryFrom, TryInto};
use crate::logon::version_8::{LoginResult, LoginResultError};
use crate::ServerMessage;
use crate::ReadableAndWritable;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMD_AUTH_RECONNECT_CHALLENGE_Server {
    pub result: CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult,
}

impl ServerMessage for CMD_AUTH_RECONNECT_CHALLENGE_Server {
    const OPCODE: u8 = 0x02;
}
impl ReadableAndWritable for CMD_AUTH_RECONNECT_CHALLENGE_Server {
    type Error = CMD_AUTH_RECONNECT_CHALLENGE_ServerError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // result: LoginResult
        let result: LoginResult = crate::util::read_u8_le(r)?.try_into()?;

        let result_if = match result {
            LoginResult::SUCCESS => {
                // challenge_data: u8[16]
                let mut challenge_data = [0_u8; 16];
                r.read_exact(&mut challenge_data)?;

                // checksum_salt: u8[16]
                let mut checksum_salt = [0_u8; 16];
                r.read_exact(&mut checksum_salt)?;

                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS {
                    challenge_data,
                    checksum_salt,
                }
            }
            LoginResult::FAIL_UNKNOWN0 => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN0,
            LoginResult::FAIL_UNKNOWN1 => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN1,
            LoginResult::FAIL_BANNED => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_BANNED,
            LoginResult::FAIL_UNKNOWN_ACCOUNT => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT,
            LoginResult::FAIL_INCORRECT_PASSWORD => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_INCORRECT_PASSWORD,
            LoginResult::FAIL_ALREADY_ONLINE => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_ALREADY_ONLINE,
            LoginResult::FAIL_NO_TIME => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_NO_TIME,
            LoginResult::FAIL_DB_BUSY => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_DB_BUSY,
            LoginResult::FAIL_VERSION_INVALID => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_VERSION_INVALID,
            LoginResult::LOGIN_DOWNLOAD_FILE => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::LOGIN_DOWNLOAD_FILE,
            LoginResult::FAIL_INVALID_SERVER => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_INVALID_SERVER,
            LoginResult::FAIL_SUSPENDED => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_SUSPENDED,
            LoginResult::FAIL_NO_ACCESS => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_NO_ACCESS,
            LoginResult::SUCCESS_SURVEY => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS_SURVEY,
            LoginResult::FAIL_PARENTALCONTROL => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_PARENTALCONTROL,
            LoginResult::FAIL_LOCKED_ENFORCED => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_LOCKED_ENFORCED,
        };

        Ok(Self {
            result: result_if,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // result: LoginResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        match &self.result {
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS {
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
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN0 => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN1 => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_BANNED => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_INCORRECT_PASSWORD => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_ALREADY_ONLINE => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_NO_TIME => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_DB_BUSY => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_VERSION_INVALID => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::LOGIN_DOWNLOAD_FILE => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_INVALID_SERVER => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_SUSPENDED => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_NO_ACCESS => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS_SURVEY => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_PARENTALCONTROL => {}
            CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_LOCKED_ENFORCED => {}
        }

        Ok(())
    }

    #[cfg(feature = "tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // result: LoginResult
            let result: LoginResult = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            let result_if = match result {
                LoginResult::SUCCESS => {
                    // challenge_data: u8[16]
                    let mut challenge_data = [0_u8; 16];
                    r.read_exact(&mut challenge_data).await?;

                    // checksum_salt: u8[16]
                    let mut checksum_salt = [0_u8; 16];
                    r.read_exact(&mut checksum_salt).await?;

                    CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS {
                        challenge_data,
                        checksum_salt,
                    }
                }
                LoginResult::FAIL_UNKNOWN0 => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN0,
                LoginResult::FAIL_UNKNOWN1 => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN1,
                LoginResult::FAIL_BANNED => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_BANNED,
                LoginResult::FAIL_UNKNOWN_ACCOUNT => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT,
                LoginResult::FAIL_INCORRECT_PASSWORD => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_INCORRECT_PASSWORD,
                LoginResult::FAIL_ALREADY_ONLINE => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_ALREADY_ONLINE,
                LoginResult::FAIL_NO_TIME => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_NO_TIME,
                LoginResult::FAIL_DB_BUSY => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_DB_BUSY,
                LoginResult::FAIL_VERSION_INVALID => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_VERSION_INVALID,
                LoginResult::LOGIN_DOWNLOAD_FILE => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::LOGIN_DOWNLOAD_FILE,
                LoginResult::FAIL_INVALID_SERVER => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_INVALID_SERVER,
                LoginResult::FAIL_SUSPENDED => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_SUSPENDED,
                LoginResult::FAIL_NO_ACCESS => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_NO_ACCESS,
                LoginResult::SUCCESS_SURVEY => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS_SURVEY,
                LoginResult::FAIL_PARENTALCONTROL => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_PARENTALCONTROL,
                LoginResult::FAIL_LOCKED_ENFORCED => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_LOCKED_ENFORCED,
            };

            Ok(Self {
                result: result_if,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // opcode: u8
            w.write_all(&Self::OPCODE.to_le_bytes()).await?;

            // result: LoginResult
            w.write_all(&(self.result.as_int() as u8).to_le_bytes()).await?;

            match &self.result {
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS {
                    challenge_data,
                    checksum_salt,
                } => {
                    // challenge_data: u8[16]
                    for i in challenge_data.iter() {
                        w.write_all(&i.to_le_bytes()).await?;
                    }

                    // checksum_salt: u8[16]
                    for i in checksum_salt.iter() {
                        w.write_all(&i.to_le_bytes()).await?;
                    }

                }
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN0 => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN1 => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_BANNED => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_INCORRECT_PASSWORD => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_ALREADY_ONLINE => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_NO_TIME => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_DB_BUSY => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_VERSION_INVALID => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::LOGIN_DOWNLOAD_FILE => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_INVALID_SERVER => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_SUSPENDED => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_NO_ACCESS => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS_SURVEY => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_PARENTALCONTROL => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_LOCKED_ENFORCED => {}
            }

            Ok(())
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // result: LoginResult
            let result: LoginResult = crate::util::astd_read_u8_le(r).await?.try_into()?;

            let result_if = match result {
                LoginResult::SUCCESS => {
                    // challenge_data: u8[16]
                    let mut challenge_data = [0_u8; 16];
                    r.read_exact(&mut challenge_data).await?;

                    // checksum_salt: u8[16]
                    let mut checksum_salt = [0_u8; 16];
                    r.read_exact(&mut checksum_salt).await?;

                    CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS {
                        challenge_data,
                        checksum_salt,
                    }
                }
                LoginResult::FAIL_UNKNOWN0 => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN0,
                LoginResult::FAIL_UNKNOWN1 => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN1,
                LoginResult::FAIL_BANNED => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_BANNED,
                LoginResult::FAIL_UNKNOWN_ACCOUNT => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT,
                LoginResult::FAIL_INCORRECT_PASSWORD => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_INCORRECT_PASSWORD,
                LoginResult::FAIL_ALREADY_ONLINE => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_ALREADY_ONLINE,
                LoginResult::FAIL_NO_TIME => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_NO_TIME,
                LoginResult::FAIL_DB_BUSY => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_DB_BUSY,
                LoginResult::FAIL_VERSION_INVALID => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_VERSION_INVALID,
                LoginResult::LOGIN_DOWNLOAD_FILE => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::LOGIN_DOWNLOAD_FILE,
                LoginResult::FAIL_INVALID_SERVER => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_INVALID_SERVER,
                LoginResult::FAIL_SUSPENDED => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_SUSPENDED,
                LoginResult::FAIL_NO_ACCESS => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_NO_ACCESS,
                LoginResult::SUCCESS_SURVEY => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS_SURVEY,
                LoginResult::FAIL_PARENTALCONTROL => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_PARENTALCONTROL,
                LoginResult::FAIL_LOCKED_ENFORCED => CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_LOCKED_ENFORCED,
            };

            Ok(Self {
                result: result_if,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // opcode: u8
            w.write_all(&Self::OPCODE.to_le_bytes()).await?;

            // result: LoginResult
            w.write_all(&(self.result.as_int() as u8).to_le_bytes()).await?;

            match &self.result {
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS {
                    challenge_data,
                    checksum_salt,
                } => {
                    // challenge_data: u8[16]
                    for i in challenge_data.iter() {
                        w.write_all(&i.to_le_bytes()).await?;
                    }

                    // checksum_salt: u8[16]
                    for i in checksum_salt.iter() {
                        w.write_all(&i.to_le_bytes()).await?;
                    }

                }
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN0 => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN1 => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_BANNED => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_INCORRECT_PASSWORD => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_ALREADY_ONLINE => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_NO_TIME => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_DB_BUSY => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_VERSION_INVALID => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::LOGIN_DOWNLOAD_FILE => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_INVALID_SERVER => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_SUSPENDED => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_NO_ACCESS => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS_SURVEY => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_PARENTALCONTROL => {}
                CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_LOCKED_ENFORCED => {}
            }

            Ok(())
        })
    }

}

impl CMD_AUTH_RECONNECT_CHALLENGE_Server {
    pub fn size(&self) -> usize {
        0
        + self.result.size() // result: CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult
    }
}

#[derive(Debug)]
pub enum CMD_AUTH_RECONNECT_CHALLENGE_ServerError {
    Io(std::io::Error),
    LoginResult(LoginResultError),
}

impl std::error::Error for CMD_AUTH_RECONNECT_CHALLENGE_ServerError {}
impl std::fmt::Display for CMD_AUTH_RECONNECT_CHALLENGE_ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::LoginResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMD_AUTH_RECONNECT_CHALLENGE_ServerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<LoginResultError> for CMD_AUTH_RECONNECT_CHALLENGE_ServerError {
    fn from(e: LoginResultError) -> Self {
        Self::LoginResult(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult {
    SUCCESS {
        challenge_data: [u8; 16],
        checksum_salt: [u8; 16],
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

impl Default for CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::SUCCESS {
            challenge_data: Default::default(),
            checksum_salt: Default::default(),
        }
    }
}

impl CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::SUCCESS { .. } => 0,
            Self::FAIL_UNKNOWN0 => 1,
            Self::FAIL_UNKNOWN1 => 2,
            Self::FAIL_BANNED => 3,
            Self::FAIL_UNKNOWN_ACCOUNT => 4,
            Self::FAIL_INCORRECT_PASSWORD => 5,
            Self::FAIL_ALREADY_ONLINE => 6,
            Self::FAIL_NO_TIME => 7,
            Self::FAIL_DB_BUSY => 8,
            Self::FAIL_VERSION_INVALID => 9,
            Self::LOGIN_DOWNLOAD_FILE => 10,
            Self::FAIL_INVALID_SERVER => 11,
            Self::FAIL_SUSPENDED => 12,
            Self::FAIL_NO_ACCESS => 13,
            Self::SUCCESS_SURVEY => 14,
            Self::FAIL_PARENTALCONTROL => 15,
            Self::FAIL_LOCKED_ENFORCED => 16,
        }
    }

}

impl CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult {
    pub fn size(&self) -> usize {
        match self {
            Self::SUCCESS {
                challenge_data,
                checksum_salt,
            } => {
                1
                + 16 * core::mem::size_of::<u8>() // challenge_data: u8[16]
                + 16 * core::mem::size_of::<u8>() // checksum_salt: u8[16]
            }
            Self::FAIL_UNKNOWN0 => {
                1
            }
            Self::FAIL_UNKNOWN1 => {
                1
            }
            Self::FAIL_BANNED => {
                1
            }
            Self::FAIL_UNKNOWN_ACCOUNT => {
                1
            }
            Self::FAIL_INCORRECT_PASSWORD => {
                1
            }
            Self::FAIL_ALREADY_ONLINE => {
                1
            }
            Self::FAIL_NO_TIME => {
                1
            }
            Self::FAIL_DB_BUSY => {
                1
            }
            Self::FAIL_VERSION_INVALID => {
                1
            }
            Self::LOGIN_DOWNLOAD_FILE => {
                1
            }
            Self::FAIL_INVALID_SERVER => {
                1
            }
            Self::FAIL_SUSPENDED => {
                1
            }
            Self::FAIL_NO_ACCESS => {
                1
            }
            Self::SUCCESS_SURVEY => {
                1
            }
            Self::FAIL_PARENTALCONTROL => {
                1
            }
            Self::FAIL_LOCKED_ENFORCED => {
                1
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::CMD_AUTH_RECONNECT_CHALLENGE_Server;
    use crate::logon::version_8::LoginResult;
    use super::*;
    use super::super::*;
    use crate::logon::version_8::opcodes::ServerOpcodeMessage;

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_RECONNECT_CHALLENGE_Server0() {
        let raw: Vec<u8> = vec![ 0x02, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
             0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0xFF, 0xFE,
             0xFD, 0xFC, 0xFB, 0xFA, 0xF9, 0xF8, 0xF7, 0xF6, 0xF5, 0xF4, 0xF3, 0xF2,
             0xF1, 0xF0, ];

        let expected = CMD_AUTH_RECONNECT_CHALLENGE_Server {
            result: CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS {
                challenge_data: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                     0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                checksum_salt: [ 0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0xFA, 0xF9, 0xF8,
                     0xF7, 0xF6, 0xF5, 0xF4, 0xF3, 0xF2, 0xF1, 0xF0, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut std::io::Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_RECONNECT_CHALLENGE_Server0() {
        let raw: Vec<u8> = vec![ 0x02, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
             0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0xFF, 0xFE,
             0xFD, 0xFC, 0xFB, 0xFA, 0xF9, 0xF8, 0xF7, 0xF6, 0xF5, 0xF4, 0xF3, 0xF2,
             0xF1, 0xF0, ];

        let expected = CMD_AUTH_RECONNECT_CHALLENGE_Server {
            result: CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS {
                challenge_data: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                     0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                checksum_salt: [ 0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0xFA, 0xF9, 0xF8,
                     0xF7, 0xF6, 0xF5, 0xF4, 0xF3, 0xF2, 0xF1, 0xF0, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_RECONNECT_CHALLENGE_Server0() {
        let raw: Vec<u8> = vec![ 0x02, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
             0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0xFF, 0xFE,
             0xFD, 0xFC, 0xFB, 0xFA, 0xF9, 0xF8, 0xF7, 0xF6, 0xF5, 0xF4, 0xF3, 0xF2,
             0xF1, 0xF0, ];

        let expected = CMD_AUTH_RECONNECT_CHALLENGE_Server {
            result: CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::SUCCESS {
                challenge_data: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                     0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                checksum_salt: [ 0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0xFA, 0xF9, 0xF8,
                     0xF7, 0xF6, 0xF5, 0xF4, 0xF3, 0xF2, 0xF1, 0xF0, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_RECONNECT_CHALLENGE_Server1() {
        let raw: Vec<u8> = vec![ 0x02, 0x03, ];

        let expected = CMD_AUTH_RECONNECT_CHALLENGE_Server {
            result: CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_BANNED,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut std::io::Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_RECONNECT_CHALLENGE_Server1() {
        let raw: Vec<u8> = vec![ 0x02, 0x03, ];

        let expected = CMD_AUTH_RECONNECT_CHALLENGE_Server {
            result: CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_BANNED,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_RECONNECT_CHALLENGE_Server1() {
        let raw: Vec<u8> = vec![ 0x02, 0x03, ];

        let expected = CMD_AUTH_RECONNECT_CHALLENGE_Server {
            result: CMD_AUTH_RECONNECT_CHALLENGE_ServerLoginResult::FAIL_BANNED,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await;

        assert_eq!(dest, raw);
    }

}
