use std::convert::{TryFrom, TryInto};
use crate::logon::version_8::AccountFlag;
use crate::logon::version_8::LoginResult;
use crate::ServerMessage;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMD_AUTH_LOGON_PROOF_Server {
    pub login_result: CMD_AUTH_LOGON_PROOF_ServerLoginResult,
}

impl CMD_AUTH_LOGON_PROOF_Server {
    pub const PADDING_VALUE: u16 = 0x00;

}

impl CMD_AUTH_LOGON_PROOF_Server {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // login_result: LoginResult
        w.write_all(&(self.login_result.as_int() as u8).to_le_bytes())?;

        match &self.login_result {
            CMD_AUTH_LOGON_PROOF_ServerLoginResult::SUCCESS {
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
                w.write_all(&(account_flag.as_int() as u32).to_le_bytes())?;

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

impl ServerMessage for CMD_AUTH_LOGON_PROOF_Server {
    const OPCODE: u8 = 1;

    type Error = crate::errors::ParseError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // login_result: LoginResult
        let login_result: LoginResult = crate::util::read_u8_le(r)?.try_into()?;

        let login_result_if = match login_result {
            LoginResult::SUCCESS => {
                // server_proof: u8[20]
                let mut server_proof = [0_u8; 20];
                r.read_exact(&mut server_proof)?;

                // account_flag: AccountFlag
                let account_flag = AccountFlag::new(crate::util::read_u32_le(r)?);

                // hardware_survey_id: u32
                let hardware_survey_id = crate::util::read_u32_le(r)?;

                // unknown_flags: u16
                let unknown_flags = crate::util::read_u16_le(r)?;

                CMD_AUTH_LOGON_PROOF_ServerLoginResult::SUCCESS {
                    account_flag,
                    hardware_survey_id,
                    server_proof,
                    unknown_flags,
                }
            }
            LoginResult::FAIL_UNKNOWN0 => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN0,
            LoginResult::FAIL_UNKNOWN1 => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN1,
            LoginResult::FAIL_BANNED => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_BANNED,
            LoginResult::FAIL_UNKNOWN_ACCOUNT => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT,
            LoginResult::FAIL_INCORRECT_PASSWORD => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_INCORRECT_PASSWORD,
            LoginResult::FAIL_ALREADY_ONLINE => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_ALREADY_ONLINE,
            LoginResult::FAIL_NO_TIME => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_TIME,
            LoginResult::FAIL_DB_BUSY => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_DB_BUSY,
            LoginResult::FAIL_VERSION_INVALID => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_VERSION_INVALID,
            LoginResult::LOGIN_DOWNLOAD_FILE => CMD_AUTH_LOGON_PROOF_ServerLoginResult::LOGIN_DOWNLOAD_FILE,
            LoginResult::FAIL_INVALID_SERVER => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_INVALID_SERVER,
            LoginResult::FAIL_SUSPENDED => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_SUSPENDED,
            LoginResult::FAIL_NO_ACCESS => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_ACCESS,
            LoginResult::SUCCESS_SURVEY => CMD_AUTH_LOGON_PROOF_ServerLoginResult::SUCCESS_SURVEY,
            LoginResult::FAIL_PARENTALCONTROL => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_PARENTALCONTROL,
            LoginResult::FAIL_LOCKED_ENFORCED => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_LOCKED_ENFORCED,
        };

        Ok(Self {
            login_result: login_result_if,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(self.size() + 1);
        self.write_into_vec(&mut v)?;
        w.write_all(&v)
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
            // login_result: LoginResult
            let login_result: LoginResult = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            let login_result_if = match login_result {
                LoginResult::SUCCESS => {
                    // server_proof: u8[20]
                    let mut server_proof = [0_u8; 20];
                    r.read_exact(&mut server_proof).await?;

                    // account_flag: AccountFlag
                    let account_flag = AccountFlag::new(crate::util::tokio_read_u32_le(r).await?);

                    // hardware_survey_id: u32
                    let hardware_survey_id = crate::util::tokio_read_u32_le(r).await?;

                    // unknown_flags: u16
                    let unknown_flags = crate::util::tokio_read_u16_le(r).await?;

                    CMD_AUTH_LOGON_PROOF_ServerLoginResult::SUCCESS {
                        account_flag,
                        hardware_survey_id,
                        server_proof,
                        unknown_flags,
                    }
                }
                LoginResult::FAIL_UNKNOWN0 => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN0,
                LoginResult::FAIL_UNKNOWN1 => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN1,
                LoginResult::FAIL_BANNED => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_BANNED,
                LoginResult::FAIL_UNKNOWN_ACCOUNT => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT,
                LoginResult::FAIL_INCORRECT_PASSWORD => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_INCORRECT_PASSWORD,
                LoginResult::FAIL_ALREADY_ONLINE => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_ALREADY_ONLINE,
                LoginResult::FAIL_NO_TIME => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_TIME,
                LoginResult::FAIL_DB_BUSY => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_DB_BUSY,
                LoginResult::FAIL_VERSION_INVALID => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_VERSION_INVALID,
                LoginResult::LOGIN_DOWNLOAD_FILE => CMD_AUTH_LOGON_PROOF_ServerLoginResult::LOGIN_DOWNLOAD_FILE,
                LoginResult::FAIL_INVALID_SERVER => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_INVALID_SERVER,
                LoginResult::FAIL_SUSPENDED => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_SUSPENDED,
                LoginResult::FAIL_NO_ACCESS => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_ACCESS,
                LoginResult::SUCCESS_SURVEY => CMD_AUTH_LOGON_PROOF_ServerLoginResult::SUCCESS_SURVEY,
                LoginResult::FAIL_PARENTALCONTROL => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_PARENTALCONTROL,
                LoginResult::FAIL_LOCKED_ENFORCED => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_LOCKED_ENFORCED,
            };

            Ok(Self {
                login_result: login_result_if,
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
            let mut v = Vec::with_capacity(self.size() + 1);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
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
            // login_result: LoginResult
            let login_result: LoginResult = crate::util::astd_read_u8_le(r).await?.try_into()?;

            let login_result_if = match login_result {
                LoginResult::SUCCESS => {
                    // server_proof: u8[20]
                    let mut server_proof = [0_u8; 20];
                    r.read_exact(&mut server_proof).await?;

                    // account_flag: AccountFlag
                    let account_flag = AccountFlag::new(crate::util::astd_read_u32_le(r).await?);

                    // hardware_survey_id: u32
                    let hardware_survey_id = crate::util::astd_read_u32_le(r).await?;

                    // unknown_flags: u16
                    let unknown_flags = crate::util::astd_read_u16_le(r).await?;

                    CMD_AUTH_LOGON_PROOF_ServerLoginResult::SUCCESS {
                        account_flag,
                        hardware_survey_id,
                        server_proof,
                        unknown_flags,
                    }
                }
                LoginResult::FAIL_UNKNOWN0 => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN0,
                LoginResult::FAIL_UNKNOWN1 => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN1,
                LoginResult::FAIL_BANNED => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_BANNED,
                LoginResult::FAIL_UNKNOWN_ACCOUNT => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT,
                LoginResult::FAIL_INCORRECT_PASSWORD => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_INCORRECT_PASSWORD,
                LoginResult::FAIL_ALREADY_ONLINE => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_ALREADY_ONLINE,
                LoginResult::FAIL_NO_TIME => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_TIME,
                LoginResult::FAIL_DB_BUSY => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_DB_BUSY,
                LoginResult::FAIL_VERSION_INVALID => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_VERSION_INVALID,
                LoginResult::LOGIN_DOWNLOAD_FILE => CMD_AUTH_LOGON_PROOF_ServerLoginResult::LOGIN_DOWNLOAD_FILE,
                LoginResult::FAIL_INVALID_SERVER => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_INVALID_SERVER,
                LoginResult::FAIL_SUSPENDED => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_SUSPENDED,
                LoginResult::FAIL_NO_ACCESS => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_ACCESS,
                LoginResult::SUCCESS_SURVEY => CMD_AUTH_LOGON_PROOF_ServerLoginResult::SUCCESS_SURVEY,
                LoginResult::FAIL_PARENTALCONTROL => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_PARENTALCONTROL,
                LoginResult::FAIL_LOCKED_ENFORCED => CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_LOCKED_ENFORCED,
            };

            Ok(Self {
                login_result: login_result_if,
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
            let mut v = Vec::with_capacity(self.size() + 1);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

}

impl CMD_AUTH_LOGON_PROOF_Server {
    pub fn size(&self) -> usize {
        0
        + self.login_result.size() // login_result: CMD_AUTH_LOGON_PROOF_ServerLoginResult
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CMD_AUTH_LOGON_PROOF_ServerLoginResult {
    SUCCESS {
        account_flag: AccountFlag,
        hardware_survey_id: u32,
        server_proof: [u8; 20],
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

impl Default for CMD_AUTH_LOGON_PROOF_ServerLoginResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::SUCCESS {
            account_flag: Default::default(),
            hardware_survey_id: Default::default(),
            server_proof: Default::default(),
            unknown_flags: Default::default(),
        }
    }
}

impl CMD_AUTH_LOGON_PROOF_ServerLoginResult {
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

impl CMD_AUTH_LOGON_PROOF_ServerLoginResult {
    pub fn size(&self) -> usize {
        match self {
            Self::SUCCESS {
                account_flag,
                hardware_survey_id,
                server_proof,
                unknown_flags,
            } => {
                1
                + 4 // account_flag: AccountFlag
                + 4 // hardware_survey_id: u32
                + 20 * core::mem::size_of::<u8>() // server_proof: u8[20]
                + 2 // unknown_flags: u16
            }
            Self::FAIL_UNKNOWN0 => {
                1
                + 2 // padding: u16
            }
            Self::FAIL_UNKNOWN1 => {
                1
                + 2 // padding: u16
            }
            Self::FAIL_BANNED => {
                1
                + 2 // padding: u16
            }
            Self::FAIL_UNKNOWN_ACCOUNT => {
                1
                + 2 // padding: u16
            }
            Self::FAIL_INCORRECT_PASSWORD => {
                1
                + 2 // padding: u16
            }
            Self::FAIL_ALREADY_ONLINE => {
                1
                + 2 // padding: u16
            }
            Self::FAIL_NO_TIME => {
                1
                + 2 // padding: u16
            }
            Self::FAIL_DB_BUSY => {
                1
                + 2 // padding: u16
            }
            Self::FAIL_VERSION_INVALID => {
                1
                + 2 // padding: u16
            }
            Self::LOGIN_DOWNLOAD_FILE => {
                1
                + 2 // padding: u16
            }
            Self::FAIL_INVALID_SERVER => {
                1
                + 2 // padding: u16
            }
            Self::FAIL_SUSPENDED => {
                1
                + 2 // padding: u16
            }
            Self::FAIL_NO_ACCESS => {
                1
                + 2 // padding: u16
            }
            Self::SUCCESS_SURVEY => {
                1
                + 2 // padding: u16
            }
            Self::FAIL_PARENTALCONTROL => {
                1
                + 2 // padding: u16
            }
            Self::FAIL_LOCKED_ENFORCED => {
                1
                + 2 // padding: u16
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::CMD_AUTH_LOGON_PROOF_Server;
    use crate::logon::version_8::AccountFlag;
    use crate::logon::version_8::LoginResult;
    use super::*;
    use super::super::*;
    use crate::logon::version_8::opcodes::ServerOpcodeMessage;

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_PROOF_Server0() {
        let raw: Vec<u8> = vec![ 0x01, 0x07, 0x00, 0x00, ];

        let expected = CMD_AUTH_LOGON_PROOF_Server {
            login_result: CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_TIME,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_PROOF_Server0() {
        let raw: Vec<u8> = vec![ 0x01, 0x07, 0x00, 0x00, ];

        let expected = CMD_AUTH_LOGON_PROOF_Server {
            login_result: CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_TIME,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_PROOF_Server0() {
        let raw: Vec<u8> = vec![ 0x01, 0x07, 0x00, 0x00, ];

        let expected = CMD_AUTH_LOGON_PROOF_Server {
            login_result: CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_NO_TIME,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_PROOF_Server1() {
        let raw: Vec<u8> = vec![ 0x01, 0x08, 0x00, 0x00, ];

        let expected = CMD_AUTH_LOGON_PROOF_Server {
            login_result: CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_DB_BUSY,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_PROOF_Server1() {
        let raw: Vec<u8> = vec![ 0x01, 0x08, 0x00, 0x00, ];

        let expected = CMD_AUTH_LOGON_PROOF_Server {
            login_result: CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_DB_BUSY,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_PROOF_Server1() {
        let raw: Vec<u8> = vec![ 0x01, 0x08, 0x00, 0x00, ];

        let expected = CMD_AUTH_LOGON_PROOF_Server {
            login_result: CMD_AUTH_LOGON_PROOF_ServerLoginResult::FAIL_DB_BUSY,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

}
