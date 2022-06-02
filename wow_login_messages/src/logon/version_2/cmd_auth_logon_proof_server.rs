use std::convert::{TryFrom, TryInto};
use crate::logon::version_2::LoginResult;
use crate::ServerMessage;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm#L2):
/// ```text
/// slogin CMD_AUTH_LOGON_PROOF_Server = 0x01 {
///     LoginResult login_result;
///     if (login_result == SUCCESS) {
///         u8[20] server_proof;
///         u32 hardware_survey_id;
///     }
/// }
/// ```
pub struct CMD_AUTH_LOGON_PROOF_Server {
    pub login_result: CMD_AUTH_LOGON_PROOF_Server_LoginResult,
}

impl CMD_AUTH_LOGON_PROOF_Server {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // login_result: LoginResult
        w.write_all(&(self.login_result.as_int() as u8).to_le_bytes())?;

        match &self.login_result {
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::SUCCESS {
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
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_UNKNOWN0 => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_UNKNOWN1 => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_BANNED => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_UNKNOWN_ACCOUNT => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_INCORRECT_PASSWORD => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_ALREADY_ONLINE => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_NO_TIME => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_DB_BUSY => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_VERSION_INVALID => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::LOGIN_DOWNLOAD_FILE => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_INVALID_SERVER => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_SUSPENDED => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_NO_ACCESS => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::SUCCESS_SURVEY => {}
            CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_PARENTALCONTROL => {}
        }

        Ok(())
    }
}

impl ServerMessage for CMD_AUTH_LOGON_PROOF_Server {
    const OPCODE: u8 = 0x01;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // login_result: LoginResult
        let login_result: LoginResult = crate::util::read_u8_le(r)?.try_into()?;

        let login_result_if = match login_result {
            LoginResult::SUCCESS => {
                // server_proof: u8[20]
                let mut server_proof = [0_u8; 20];
                r.read_exact(&mut server_proof)?;

                // hardware_survey_id: u32
                let hardware_survey_id = crate::util::read_u32_le(r)?;

                CMD_AUTH_LOGON_PROOF_Server_LoginResult::SUCCESS {
                    hardware_survey_id,
                    server_proof,
                }
            }
            LoginResult::FAIL_UNKNOWN0 => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_UNKNOWN0,
            LoginResult::FAIL_UNKNOWN1 => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_UNKNOWN1,
            LoginResult::FAIL_BANNED => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_BANNED,
            LoginResult::FAIL_UNKNOWN_ACCOUNT => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_UNKNOWN_ACCOUNT,
            LoginResult::FAIL_INCORRECT_PASSWORD => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_INCORRECT_PASSWORD,
            LoginResult::FAIL_ALREADY_ONLINE => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_ALREADY_ONLINE,
            LoginResult::FAIL_NO_TIME => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_NO_TIME,
            LoginResult::FAIL_DB_BUSY => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_DB_BUSY,
            LoginResult::FAIL_VERSION_INVALID => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_VERSION_INVALID,
            LoginResult::LOGIN_DOWNLOAD_FILE => CMD_AUTH_LOGON_PROOF_Server_LoginResult::LOGIN_DOWNLOAD_FILE,
            LoginResult::FAIL_INVALID_SERVER => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_INVALID_SERVER,
            LoginResult::FAIL_SUSPENDED => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_SUSPENDED,
            LoginResult::FAIL_NO_ACCESS => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_NO_ACCESS,
            LoginResult::SUCCESS_SURVEY => CMD_AUTH_LOGON_PROOF_Server_LoginResult::SUCCESS_SURVEY,
            LoginResult::FAIL_PARENTALCONTROL => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_PARENTALCONTROL,
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
        dyn core::future::Future<Output = std::result::Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + tokio::io::AsyncReadExt + Unpin + Send,
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

                    // hardware_survey_id: u32
                    let hardware_survey_id = crate::util::tokio_read_u32_le(r).await?;

                    CMD_AUTH_LOGON_PROOF_Server_LoginResult::SUCCESS {
                        hardware_survey_id,
                        server_proof,
                    }
                }
                LoginResult::FAIL_UNKNOWN0 => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_UNKNOWN0,
                LoginResult::FAIL_UNKNOWN1 => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_UNKNOWN1,
                LoginResult::FAIL_BANNED => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_BANNED,
                LoginResult::FAIL_UNKNOWN_ACCOUNT => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_UNKNOWN_ACCOUNT,
                LoginResult::FAIL_INCORRECT_PASSWORD => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_INCORRECT_PASSWORD,
                LoginResult::FAIL_ALREADY_ONLINE => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_ALREADY_ONLINE,
                LoginResult::FAIL_NO_TIME => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_NO_TIME,
                LoginResult::FAIL_DB_BUSY => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_DB_BUSY,
                LoginResult::FAIL_VERSION_INVALID => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_VERSION_INVALID,
                LoginResult::LOGIN_DOWNLOAD_FILE => CMD_AUTH_LOGON_PROOF_Server_LoginResult::LOGIN_DOWNLOAD_FILE,
                LoginResult::FAIL_INVALID_SERVER => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_INVALID_SERVER,
                LoginResult::FAIL_SUSPENDED => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_SUSPENDED,
                LoginResult::FAIL_NO_ACCESS => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_NO_ACCESS,
                LoginResult::SUCCESS_SURVEY => CMD_AUTH_LOGON_PROOF_Server_LoginResult::SUCCESS_SURVEY,
                LoginResult::FAIL_PARENTALCONTROL => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_PARENTALCONTROL,
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
        dyn core::future::Future<Output = std::result::Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + async_std::io::ReadExt + Unpin + Send,
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

                    // hardware_survey_id: u32
                    let hardware_survey_id = crate::util::astd_read_u32_le(r).await?;

                    CMD_AUTH_LOGON_PROOF_Server_LoginResult::SUCCESS {
                        hardware_survey_id,
                        server_proof,
                    }
                }
                LoginResult::FAIL_UNKNOWN0 => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_UNKNOWN0,
                LoginResult::FAIL_UNKNOWN1 => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_UNKNOWN1,
                LoginResult::FAIL_BANNED => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_BANNED,
                LoginResult::FAIL_UNKNOWN_ACCOUNT => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_UNKNOWN_ACCOUNT,
                LoginResult::FAIL_INCORRECT_PASSWORD => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_INCORRECT_PASSWORD,
                LoginResult::FAIL_ALREADY_ONLINE => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_ALREADY_ONLINE,
                LoginResult::FAIL_NO_TIME => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_NO_TIME,
                LoginResult::FAIL_DB_BUSY => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_DB_BUSY,
                LoginResult::FAIL_VERSION_INVALID => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_VERSION_INVALID,
                LoginResult::LOGIN_DOWNLOAD_FILE => CMD_AUTH_LOGON_PROOF_Server_LoginResult::LOGIN_DOWNLOAD_FILE,
                LoginResult::FAIL_INVALID_SERVER => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_INVALID_SERVER,
                LoginResult::FAIL_SUSPENDED => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_SUSPENDED,
                LoginResult::FAIL_NO_ACCESS => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_NO_ACCESS,
                LoginResult::SUCCESS_SURVEY => CMD_AUTH_LOGON_PROOF_Server_LoginResult::SUCCESS_SURVEY,
                LoginResult::FAIL_PARENTALCONTROL => CMD_AUTH_LOGON_PROOF_Server_LoginResult::FAIL_PARENTALCONTROL,
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
    pub(crate) fn size(&self) -> usize {
        self.login_result.size() // login_result: CMD_AUTH_LOGON_PROOF_Server_LoginResult
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CMD_AUTH_LOGON_PROOF_Server_LoginResult {
    SUCCESS {
        hardware_survey_id: u32,
        server_proof: [u8; 20],
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
}

impl Default for CMD_AUTH_LOGON_PROOF_Server_LoginResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::SUCCESS {
            hardware_survey_id: Default::default(),
            server_proof: Default::default(),
        }
    }
}

impl CMD_AUTH_LOGON_PROOF_Server_LoginResult {
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
        }
    }

}

impl CMD_AUTH_LOGON_PROOF_Server_LoginResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::SUCCESS {
                hardware_survey_id,
                server_proof,
            } => {
                1
                + 4 // hardware_survey_id: u32
                + 20 * core::mem::size_of::<u8>() // server_proof: u8[20]
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
        }
    }
}

#[cfg(test)]
mod test {
    use super::CMD_AUTH_LOGON_PROOF_Server;
    use crate::logon::version_2::LoginResult;
    use super::*;
    use super::super::*;
    use crate::logon::version_2::opcodes::ServerOpcodeMessage;

    const RAW0: [u8; 26] = [ 0x01, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
         0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12,
         0x13, 0xEF, 0xBE, 0xAD, 0xDE, ];

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 12.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_PROOF_Server0() {
        let expected = CMD_AUTH_LOGON_PROOF_Server {
            login_result: CMD_AUTH_LOGON_PROOF_Server_LoginResult::SUCCESS {
                hardware_survey_id: 0xDEADBEEF,
                server_proof: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                     0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                     0x12, 0x13, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 12.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_PROOF_Server0() {
        let expected = CMD_AUTH_LOGON_PROOF_Server {
            login_result: CMD_AUTH_LOGON_PROOF_Server_LoginResult::SUCCESS {
                hardware_survey_id: 0xDEADBEEF,
                server_proof: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                     0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                     0x12, 0x13, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/proof_server.wowm` line 12.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_PROOF_Server0() {
        let expected = CMD_AUTH_LOGON_PROOF_Server {
            login_result: CMD_AUTH_LOGON_PROOF_Server_LoginResult::SUCCESS {
                hardware_survey_id: 0xDEADBEEF,
                server_proof: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                     0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
                     0x12, 0x13, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_PROOF(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_PROOF, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
