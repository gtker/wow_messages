use std::convert::{TryFrom, TryInto};
use crate::logon::version_2::LoginResult;
use crate::logon::version_3::SecurityFlag;
use crate::ServerMessage;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMD_AUTH_LOGON_CHALLENGE_Server {
    pub login_result: CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult,
}

impl CMD_AUTH_LOGON_CHALLENGE_Server {
    pub const PROTOCOL_VERSION_VALUE: u8 = 0x00;

}

impl CMD_AUTH_LOGON_CHALLENGE_Server {
    pub(crate) fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // protocol_version: u8
        w.write_all(&Self::PROTOCOL_VERSION_VALUE.to_le_bytes())?;

        // login_result: LoginResult
        w.write_all(&(self.login_result.as_int() as u8).to_le_bytes())?;

        match &self.login_result {
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS {
                crc_salt,
                generator,
                large_safe_prime,
                salt,
                security_flag,
                server_public_key,
            } => {
                // server_public_key: u8[32]
                for i in server_public_key.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

                // generator_length: u8
                w.write_all(&(generator.len() as u8).to_le_bytes())?;

                // generator: u8[generator_length]
                for i in generator.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

                // large_safe_prime_length: u8
                w.write_all(&(large_safe_prime.len() as u8).to_le_bytes())?;

                // large_safe_prime: u8[large_safe_prime_length]
                for i in large_safe_prime.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

                // salt: u8[32]
                for i in salt.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

                // crc_salt: u8[16]
                for i in crc_salt.iter() {
                    w.write_all(&i.to_le_bytes())?;
                }

                // security_flag: SecurityFlag
                w.write_all(&(security_flag.as_int() as u8).to_le_bytes())?;

                match &security_flag {
                    CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag::NONE => {}
                    CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag::PIN {
                        pin_grid_seed,
                        pin_salt,
                    } => {
                        // pin_grid_seed: u32
                        w.write_all(&pin_grid_seed.to_le_bytes())?;

                        // pin_salt: u8[16]
                        for i in pin_salt.iter() {
                            w.write_all(&i.to_le_bytes())?;
                        }

                    }
                }

            }
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN0 => {}
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN1 => {}
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_BANNED => {}
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT => {}
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_INCORRECT_PASSWORD => {}
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_ALREADY_ONLINE => {}
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_NO_TIME => {}
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_DB_BUSY => {}
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_VERSION_INVALID => {}
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::LOGIN_DOWNLOAD_FILE => {}
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_INVALID_SERVER => {}
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_SUSPENDED => {}
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_NO_ACCESS => {}
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS_SURVEY => {}
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_PARENTALCONTROL => {}
        }

        Ok(())
    }
}

impl ServerMessage for CMD_AUTH_LOGON_CHALLENGE_Server {
    const OPCODE: u8 = 0;

    type Error = crate::errors::ParseError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // protocol_version: u8
        let _protocol_version = crate::util::read_u8_le(r)?;
        // protocol_version is expected to always be 0 (0)

        // login_result: LoginResult
        let login_result: LoginResult = crate::util::read_u8_le(r)?.try_into()?;

        let login_result_if = match login_result {
            LoginResult::SUCCESS => {
                // server_public_key: u8[32]
                let mut server_public_key = [0_u8; 32];
                r.read_exact(&mut server_public_key)?;

                // generator_length: u8
                let generator_length = crate::util::read_u8_le(r)?;

                // generator: u8[generator_length]
                let mut generator = Vec::with_capacity(generator_length as usize);
                for i in 0..generator_length {
                    generator.push(crate::util::read_u8_le(r)?);
                }

                // large_safe_prime_length: u8
                let large_safe_prime_length = crate::util::read_u8_le(r)?;

                // large_safe_prime: u8[large_safe_prime_length]
                let mut large_safe_prime = Vec::with_capacity(large_safe_prime_length as usize);
                for i in 0..large_safe_prime_length {
                    large_safe_prime.push(crate::util::read_u8_le(r)?);
                }

                // salt: u8[32]
                let mut salt = [0_u8; 32];
                r.read_exact(&mut salt)?;

                // crc_salt: u8[16]
                let mut crc_salt = [0_u8; 16];
                r.read_exact(&mut crc_salt)?;

                // security_flag: SecurityFlag
                let security_flag: SecurityFlag = crate::util::read_u8_le(r)?.try_into()?;

                let security_flag_if = match security_flag {
                    SecurityFlag::NONE => CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag::NONE,
                    SecurityFlag::PIN => {
                        // pin_grid_seed: u32
                        let pin_grid_seed = crate::util::read_u32_le(r)?;

                        // pin_salt: u8[16]
                        let mut pin_salt = [0_u8; 16];
                        r.read_exact(&mut pin_salt)?;

                        CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag::PIN {
                            pin_grid_seed,
                            pin_salt,
                        }
                    }
                };

                CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS {
                    crc_salt,
                    generator,
                    large_safe_prime,
                    salt,
                    security_flag: security_flag_if,
                    server_public_key,
                }
            }
            LoginResult::FAIL_UNKNOWN0 => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN0,
            LoginResult::FAIL_UNKNOWN1 => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN1,
            LoginResult::FAIL_BANNED => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_BANNED,
            LoginResult::FAIL_UNKNOWN_ACCOUNT => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT,
            LoginResult::FAIL_INCORRECT_PASSWORD => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_INCORRECT_PASSWORD,
            LoginResult::FAIL_ALREADY_ONLINE => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_ALREADY_ONLINE,
            LoginResult::FAIL_NO_TIME => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_NO_TIME,
            LoginResult::FAIL_DB_BUSY => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_DB_BUSY,
            LoginResult::FAIL_VERSION_INVALID => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_VERSION_INVALID,
            LoginResult::LOGIN_DOWNLOAD_FILE => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::LOGIN_DOWNLOAD_FILE,
            LoginResult::FAIL_INVALID_SERVER => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_INVALID_SERVER,
            LoginResult::FAIL_SUSPENDED => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_SUSPENDED,
            LoginResult::FAIL_NO_ACCESS => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_NO_ACCESS,
            LoginResult::SUCCESS_SURVEY => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS_SURVEY,
            LoginResult::FAIL_PARENTALCONTROL => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_PARENTALCONTROL,
        };

        Ok(Self {
            login_result: login_result_if,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(self.size());
        self.as_bytes(&mut v)?;
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
            // protocol_version: u8
            let _protocol_version = crate::util::tokio_read_u8_le(r).await?;
            // protocol_version is expected to always be 0 (0)

            // login_result: LoginResult
            let login_result: LoginResult = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            let login_result_if = match login_result {
                LoginResult::SUCCESS => {
                    // server_public_key: u8[32]
                    let mut server_public_key = [0_u8; 32];
                    r.read_exact(&mut server_public_key).await?;

                    // generator_length: u8
                    let generator_length = crate::util::tokio_read_u8_le(r).await?;

                    // generator: u8[generator_length]
                    let mut generator = Vec::with_capacity(generator_length as usize);
                    for i in 0..generator_length {
                        generator.push(crate::util::tokio_read_u8_le(r).await?);
                    }

                    // large_safe_prime_length: u8
                    let large_safe_prime_length = crate::util::tokio_read_u8_le(r).await?;

                    // large_safe_prime: u8[large_safe_prime_length]
                    let mut large_safe_prime = Vec::with_capacity(large_safe_prime_length as usize);
                    for i in 0..large_safe_prime_length {
                        large_safe_prime.push(crate::util::tokio_read_u8_le(r).await?);
                    }

                    // salt: u8[32]
                    let mut salt = [0_u8; 32];
                    r.read_exact(&mut salt).await?;

                    // crc_salt: u8[16]
                    let mut crc_salt = [0_u8; 16];
                    r.read_exact(&mut crc_salt).await?;

                    // security_flag: SecurityFlag
                    let security_flag: SecurityFlag = crate::util::tokio_read_u8_le(r).await?.try_into()?;

                    let security_flag_if = match security_flag {
                        SecurityFlag::NONE => CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag::NONE,
                        SecurityFlag::PIN => {
                            // pin_grid_seed: u32
                            let pin_grid_seed = crate::util::tokio_read_u32_le(r).await?;

                            // pin_salt: u8[16]
                            let mut pin_salt = [0_u8; 16];
                            r.read_exact(&mut pin_salt).await?;

                            CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag::PIN {
                                pin_grid_seed,
                                pin_salt,
                            }
                        }
                    };

                    CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS {
                        crc_salt,
                        generator,
                        large_safe_prime,
                        salt,
                        security_flag: security_flag_if,
                        server_public_key,
                    }
                }
                LoginResult::FAIL_UNKNOWN0 => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN0,
                LoginResult::FAIL_UNKNOWN1 => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN1,
                LoginResult::FAIL_BANNED => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_BANNED,
                LoginResult::FAIL_UNKNOWN_ACCOUNT => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT,
                LoginResult::FAIL_INCORRECT_PASSWORD => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_INCORRECT_PASSWORD,
                LoginResult::FAIL_ALREADY_ONLINE => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_ALREADY_ONLINE,
                LoginResult::FAIL_NO_TIME => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_NO_TIME,
                LoginResult::FAIL_DB_BUSY => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_DB_BUSY,
                LoginResult::FAIL_VERSION_INVALID => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_VERSION_INVALID,
                LoginResult::LOGIN_DOWNLOAD_FILE => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::LOGIN_DOWNLOAD_FILE,
                LoginResult::FAIL_INVALID_SERVER => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_INVALID_SERVER,
                LoginResult::FAIL_SUSPENDED => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_SUSPENDED,
                LoginResult::FAIL_NO_ACCESS => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_NO_ACCESS,
                LoginResult::SUCCESS_SURVEY => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS_SURVEY,
                LoginResult::FAIL_PARENTALCONTROL => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_PARENTALCONTROL,
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
            let mut v = Vec::with_capacity(self.size());
            self.as_bytes(&mut v)?;
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
            // protocol_version: u8
            let _protocol_version = crate::util::astd_read_u8_le(r).await?;
            // protocol_version is expected to always be 0 (0)

            // login_result: LoginResult
            let login_result: LoginResult = crate::util::astd_read_u8_le(r).await?.try_into()?;

            let login_result_if = match login_result {
                LoginResult::SUCCESS => {
                    // server_public_key: u8[32]
                    let mut server_public_key = [0_u8; 32];
                    r.read_exact(&mut server_public_key).await?;

                    // generator_length: u8
                    let generator_length = crate::util::astd_read_u8_le(r).await?;

                    // generator: u8[generator_length]
                    let mut generator = Vec::with_capacity(generator_length as usize);
                    for i in 0..generator_length {
                        generator.push(crate::util::astd_read_u8_le(r).await?);
                    }

                    // large_safe_prime_length: u8
                    let large_safe_prime_length = crate::util::astd_read_u8_le(r).await?;

                    // large_safe_prime: u8[large_safe_prime_length]
                    let mut large_safe_prime = Vec::with_capacity(large_safe_prime_length as usize);
                    for i in 0..large_safe_prime_length {
                        large_safe_prime.push(crate::util::astd_read_u8_le(r).await?);
                    }

                    // salt: u8[32]
                    let mut salt = [0_u8; 32];
                    r.read_exact(&mut salt).await?;

                    // crc_salt: u8[16]
                    let mut crc_salt = [0_u8; 16];
                    r.read_exact(&mut crc_salt).await?;

                    // security_flag: SecurityFlag
                    let security_flag: SecurityFlag = crate::util::astd_read_u8_le(r).await?.try_into()?;

                    let security_flag_if = match security_flag {
                        SecurityFlag::NONE => CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag::NONE,
                        SecurityFlag::PIN => {
                            // pin_grid_seed: u32
                            let pin_grid_seed = crate::util::astd_read_u32_le(r).await?;

                            // pin_salt: u8[16]
                            let mut pin_salt = [0_u8; 16];
                            r.read_exact(&mut pin_salt).await?;

                            CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag::PIN {
                                pin_grid_seed,
                                pin_salt,
                            }
                        }
                    };

                    CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS {
                        crc_salt,
                        generator,
                        large_safe_prime,
                        salt,
                        security_flag: security_flag_if,
                        server_public_key,
                    }
                }
                LoginResult::FAIL_UNKNOWN0 => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN0,
                LoginResult::FAIL_UNKNOWN1 => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN1,
                LoginResult::FAIL_BANNED => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_BANNED,
                LoginResult::FAIL_UNKNOWN_ACCOUNT => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT,
                LoginResult::FAIL_INCORRECT_PASSWORD => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_INCORRECT_PASSWORD,
                LoginResult::FAIL_ALREADY_ONLINE => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_ALREADY_ONLINE,
                LoginResult::FAIL_NO_TIME => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_NO_TIME,
                LoginResult::FAIL_DB_BUSY => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_DB_BUSY,
                LoginResult::FAIL_VERSION_INVALID => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_VERSION_INVALID,
                LoginResult::LOGIN_DOWNLOAD_FILE => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::LOGIN_DOWNLOAD_FILE,
                LoginResult::FAIL_INVALID_SERVER => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_INVALID_SERVER,
                LoginResult::FAIL_SUSPENDED => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_SUSPENDED,
                LoginResult::FAIL_NO_ACCESS => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_NO_ACCESS,
                LoginResult::SUCCESS_SURVEY => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS_SURVEY,
                LoginResult::FAIL_PARENTALCONTROL => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_PARENTALCONTROL,
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
            let mut v = Vec::with_capacity(self.size());
            self.as_bytes(&mut v)?;
            w.write_all(&v).await
        })
    }

}

impl CMD_AUTH_LOGON_CHALLENGE_Server {
    pub fn size(&self) -> usize {
        0
        + 1 // protocol_version: u8
        + self.login_result.size() // login_result: CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag {
    NONE,
    PIN {
        pin_grid_seed: u32,
        pin_salt: [u8; 16],
    },
}

impl Default for CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NONE
    }
}

impl CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NONE => 0,
            Self::PIN { .. } => 1,
        }
    }

}

impl CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag {
    pub fn size(&self) -> usize {
        match self {
            Self::NONE => {
                1
            }
            Self::PIN {
                pin_grid_seed,
                pin_salt,
            } => {
                1
                + 4 // pin_grid_seed: u32
                + 16 * core::mem::size_of::<u8>() // pin_salt: u8[16]
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult {
    SUCCESS {
        crc_salt: [u8; 16],
        generator: Vec<u8>,
        large_safe_prime: Vec<u8>,
        salt: [u8; 32],
        security_flag: CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag,
        server_public_key: [u8; 32],
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

impl Default for CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::SUCCESS {
            crc_salt: Default::default(),
            generator: Default::default(),
            large_safe_prime: Default::default(),
            salt: Default::default(),
            security_flag: Default::default(),
            server_public_key: Default::default(),
        }
    }
}

impl CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult {
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

impl CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult {
    pub fn size(&self) -> usize {
        match self {
            Self::SUCCESS {
                crc_salt,
                generator,
                large_safe_prime,
                salt,
                security_flag,
                server_public_key,
            } => {
                1
                + 16 * core::mem::size_of::<u8>() // crc_salt: u8[16]
                + generator.len() * core::mem::size_of::<u8>() // generator: u8[generator_length]
                + 1 // generator_length: u8
                + large_safe_prime.len() * core::mem::size_of::<u8>() // large_safe_prime: u8[large_safe_prime_length]
                + 1 // large_safe_prime_length: u8
                + 32 * core::mem::size_of::<u8>() // salt: u8[32]
                + security_flag.size() // security_flag: CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag
                + 32 * core::mem::size_of::<u8>() // server_public_key: u8[32]
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
    use super::CMD_AUTH_LOGON_CHALLENGE_Server;
    use crate::logon::version_2::LoginResult;
    use crate::logon::version_3::SecurityFlag;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ServerOpcodeMessage;

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_CHALLENGE_Server0() {
        let raw: Vec<u8> = vec![ 0x00, 0x00, 0x00, 0x49, 0xD8, 0xC2, 0xBC, 0x68,
             0x5C, 0x2B, 0xCE, 0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58,
             0x78, 0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87, 0xCE,
             0xDA, 0x34, 0x46, 0x01, 0x07, 0x20, 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82,
             0x3C, 0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53, 0x50,
             0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1, 0x89, 0x5E, 0x64,
             0x4B, 0x89, 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
             0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B, 0xCF, 0x74,
             0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30, 0x90, 0x87, 0xBA, 0xA3,
             0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC, 0x37, 0x3F, 0xB3, 0x69, 0xCD,
             0xD2, 0xF1, 0x01, 0xEF, 0xBE, 0xAD, 0xDE, 0x00, 0x01, 0x02, 0x03, 0x04,
             0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ];

        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            login_result: CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag::PIN {
                    pin_grid_seed: 0xDEADBEEF,
                    pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                },
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_CHALLENGE_Server0() {
        let raw: Vec<u8> = vec![ 0x00, 0x00, 0x00, 0x49, 0xD8, 0xC2, 0xBC, 0x68,
             0x5C, 0x2B, 0xCE, 0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58,
             0x78, 0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87, 0xCE,
             0xDA, 0x34, 0x46, 0x01, 0x07, 0x20, 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82,
             0x3C, 0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53, 0x50,
             0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1, 0x89, 0x5E, 0x64,
             0x4B, 0x89, 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
             0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B, 0xCF, 0x74,
             0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30, 0x90, 0x87, 0xBA, 0xA3,
             0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC, 0x37, 0x3F, 0xB3, 0x69, 0xCD,
             0xD2, 0xF1, 0x01, 0xEF, 0xBE, 0xAD, 0xDE, 0x00, 0x01, 0x02, 0x03, 0x04,
             0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ];

        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            login_result: CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag::PIN {
                    pin_grid_seed: 0xDEADBEEF,
                    pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                },
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_CHALLENGE_Server0() {
        let raw: Vec<u8> = vec![ 0x00, 0x00, 0x00, 0x49, 0xD8, 0xC2, 0xBC, 0x68,
             0x5C, 0x2B, 0xCE, 0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58,
             0x78, 0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87, 0xCE,
             0xDA, 0x34, 0x46, 0x01, 0x07, 0x20, 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82,
             0x3C, 0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53, 0x50,
             0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1, 0x89, 0x5E, 0x64,
             0x4B, 0x89, 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
             0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B, 0xCF, 0x74,
             0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30, 0x90, 0x87, 0xBA, 0xA3,
             0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC, 0x37, 0x3F, 0xB3, 0x69, 0xCD,
             0xD2, 0xF1, 0x01, 0xEF, 0xBE, 0xAD, 0xDE, 0x00, 0x01, 0x02, 0x03, 0x04,
             0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ];

        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            login_result: CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag::PIN {
                    pin_grid_seed: 0xDEADBEEF,
                    pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                         0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                },
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_CHALLENGE_Server1() {
        let raw: Vec<u8> = vec![ 0x00, 0x00, 0x00, 0x49, 0xD8, 0xC2, 0xBC, 0x68,
             0x5C, 0x2B, 0xCE, 0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58,
             0x78, 0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87, 0xCE,
             0xDA, 0x34, 0x46, 0x01, 0x07, 0x20, 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82,
             0x3C, 0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53, 0x50,
             0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1, 0x89, 0x5E, 0x64,
             0x4B, 0x89, 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
             0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B, 0xCF, 0x74,
             0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30, 0x90, 0x87, 0xBA, 0xA3,
             0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC, 0x37, 0x3F, 0xB3, 0x69, 0xCD,
             0xD2, 0xF1, 0x00, ];

        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            login_result: CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag::NONE,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_CHALLENGE_Server1() {
        let raw: Vec<u8> = vec![ 0x00, 0x00, 0x00, 0x49, 0xD8, 0xC2, 0xBC, 0x68,
             0x5C, 0x2B, 0xCE, 0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58,
             0x78, 0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87, 0xCE,
             0xDA, 0x34, 0x46, 0x01, 0x07, 0x20, 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82,
             0x3C, 0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53, 0x50,
             0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1, 0x89, 0x5E, 0x64,
             0x4B, 0x89, 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
             0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B, 0xCF, 0x74,
             0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30, 0x90, 0x87, 0xBA, 0xA3,
             0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC, 0x37, 0x3F, 0xB3, 0x69, 0xCD,
             0xD2, 0xF1, 0x00, ];

        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            login_result: CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag::NONE,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_CHALLENGE_Server1() {
        let raw: Vec<u8> = vec![ 0x00, 0x00, 0x00, 0x49, 0xD8, 0xC2, 0xBC, 0x68,
             0x5C, 0x2B, 0xCE, 0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58,
             0x78, 0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87, 0xCE,
             0xDA, 0x34, 0x46, 0x01, 0x07, 0x20, 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82,
             0x3C, 0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53, 0x50,
             0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1, 0x89, 0x5E, 0x64,
             0x4B, 0x89, 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
             0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B, 0xCF, 0x74,
             0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30, 0x90, 0x87, 0xBA, 0xA3,
             0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC, 0x37, 0x3F, 0xB3, 0x69, 0xCD,
             0xD2, 0xF1, 0x00, ];

        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            login_result: CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS {
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag::NONE,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

}
