use crate::ServerMessage;
use std::io::{Read, Write};

use crate::logon::version_8::{
    LoginResult, SecurityFlag,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm:224`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm#L224):
/// ```text
/// slogin CMD_AUTH_LOGON_CHALLENGE_Server = 0x00 {
///     u8 protocol_version = 0;
///     LoginResult result;
///     if (result == SUCCESS) {
///         u8[32] server_public_key;
///         u8 generator_length;
///         u8[generator_length] generator;
///         u8 large_safe_prime_length;
///         u8[large_safe_prime_length] large_safe_prime;
///         u8[32] salt;
///         u8[16] crc_salt;
///         SecurityFlag security_flag;
///         if (security_flag & PIN) {
///             u32 pin_grid_seed;
///             u8[16] pin_salt;
///         }
///         if (security_flag & MATRIX_CARD) {
///             u8 width;
///             u8 height;
///             u8 digit_count;
///             u8 challenge_count;
///             u64 seed;
///         }
///         if (security_flag & AUTHENTICATOR) {
///             u8 required;
///         }
///     }
/// }
/// ```
pub struct CMD_AUTH_LOGON_CHALLENGE_Server {
    pub result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult,
}

impl CMD_AUTH_LOGON_CHALLENGE_Server {
    /// The field `protocol_version` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `0` |
    /// | Hex | `0x00` |
    /// | Original | `0` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const PROTOCOL_VERSION_VALUE: u8 = 0x00;

}

impl CMD_AUTH_LOGON_CHALLENGE_Server {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // protocol_version: u8
        w.write_all(&Self::PROTOCOL_VERSION_VALUE.to_le_bytes())?;

        // result: LoginResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

        match &self.result {
            CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
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
                w.write_all(&(security_flag.as_int().to_le_bytes()))?;

                if let Some(if_statement) = &security_flag.pin {
                    // pin_grid_seed: u32
                    w.write_all(&if_statement.pin_grid_seed.to_le_bytes())?;

                    // pin_salt: u8[16]
                    for i in if_statement.pin_salt.iter() {
                        w.write_all(&i.to_le_bytes())?;
                    }

                }

                if let Some(if_statement) = &security_flag.matrix_card {
                    // width: u8
                    w.write_all(&if_statement.width.to_le_bytes())?;

                    // height: u8
                    w.write_all(&if_statement.height.to_le_bytes())?;

                    // digit_count: u8
                    w.write_all(&if_statement.digit_count.to_le_bytes())?;

                    // challenge_count: u8
                    w.write_all(&if_statement.challenge_count.to_le_bytes())?;

                    // seed: u64
                    w.write_all(&if_statement.seed.to_le_bytes())?;

                }

                if let Some(if_statement) = &security_flag.authenticator {
                    // required: u8
                    w.write_all(&if_statement.required.to_le_bytes())?;

                }

            }
            _ => {}
        }

        Ok(())
    }
}

impl crate::private::Sealed for CMD_AUTH_LOGON_CHALLENGE_Server {}

impl CMD_AUTH_LOGON_CHALLENGE_Server {
    fn read_inner<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // protocol_version: u8
        let _protocol_version = crate::util::read_u8_le(&mut r)?;
        // protocol_version is expected to always be 0 (0)

        // result: LoginResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        let result_if = match result {
            LoginResult::Success => {
                // server_public_key: u8[32]
                let server_public_key = {
                    let mut server_public_key = [0_u8; 32];
                    r.read_exact(&mut server_public_key)?;
                    server_public_key
                };

                // generator_length: u8
                let generator_length = crate::util::read_u8_le(&mut r)?;

                // generator: u8[generator_length]
                let generator = {
                    let mut generator = Vec::with_capacity(generator_length as usize);
                    for _ in 0..generator_length {
                        generator.push(crate::util::read_u8_le(&mut r)?);
                    }
                    generator
                };

                // large_safe_prime_length: u8
                let large_safe_prime_length = crate::util::read_u8_le(&mut r)?;

                // large_safe_prime: u8[large_safe_prime_length]
                let large_safe_prime = {
                    let mut large_safe_prime = Vec::with_capacity(large_safe_prime_length as usize);
                    for _ in 0..large_safe_prime_length {
                        large_safe_prime.push(crate::util::read_u8_le(&mut r)?);
                    }
                    large_safe_prime
                };

                // salt: u8[32]
                let salt = {
                    let mut salt = [0_u8; 32];
                    r.read_exact(&mut salt)?;
                    salt
                };

                // crc_salt: u8[16]
                let crc_salt = {
                    let mut crc_salt = [0_u8; 16];
                    r.read_exact(&mut crc_salt)?;
                    crc_salt
                };

                // security_flag: SecurityFlag
                let security_flag = SecurityFlag::new(crate::util::read_u8_le(&mut r)?);

                let security_flag_pin = if security_flag.is_pin() {
                    // pin_grid_seed: u32
                    let pin_grid_seed = crate::util::read_u32_le(&mut r)?;

                    // pin_salt: u8[16]
                    let pin_salt = {
                        let mut pin_salt = [0_u8; 16];
                        r.read_exact(&mut pin_salt)?;
                        pin_salt
                    };

                    Some(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin {
                        pin_grid_seed,
                        pin_salt,
                    })
                }
                else {
                    None
                };

                let security_flag_matrix_card = if security_flag.is_matrix_card() {
                    // width: u8
                    let width = crate::util::read_u8_le(&mut r)?;

                    // height: u8
                    let height = crate::util::read_u8_le(&mut r)?;

                    // digit_count: u8
                    let digit_count = crate::util::read_u8_le(&mut r)?;

                    // challenge_count: u8
                    let challenge_count = crate::util::read_u8_le(&mut r)?;

                    // seed: u64
                    let seed = crate::util::read_u64_le(&mut r)?;

                    Some(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_MatrixCard {
                        challenge_count,
                        digit_count,
                        height,
                        seed,
                        width,
                    })
                }
                else {
                    None
                };

                let security_flag_authenticator = if security_flag.is_authenticator() {
                    // required: u8
                    let required = crate::util::read_u8_le(&mut r)?;

                    Some(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator {
                        required,
                    })
                }
                else {
                    None
                };

                let security_flag = CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag {
                    inner: security_flag.as_int(),
                    pin: security_flag_pin,
                    matrix_card: security_flag_matrix_card,
                    authenticator: security_flag_authenticator,
                };

                CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                    crc_salt,
                    generator,
                    large_safe_prime,
                    salt,
                    security_flag,
                    server_public_key,
                }
            }
            LoginResult::FailUnknown0 => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknown0,
            LoginResult::FailUnknown1 => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknown1,
            LoginResult::FailBanned => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailBanned,
            LoginResult::FailUnknownAccount => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknownAccount,
            LoginResult::FailIncorrectPassword => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailIncorrectPassword,
            LoginResult::FailAlreadyOnline => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailAlreadyOnline,
            LoginResult::FailNoTime => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailNoTime,
            LoginResult::FailDbBusy => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailDbBusy,
            LoginResult::FailVersionInvalid => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailVersionInvalid,
            LoginResult::LoginDownloadFile => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::LoginDownloadFile,
            LoginResult::FailInvalidServer => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailInvalidServer,
            LoginResult::FailSuspended => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailSuspended,
            LoginResult::FailNoAccess => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailNoAccess,
            LoginResult::SuccessSurvey => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::SuccessSurvey,
            LoginResult::FailParentalcontrol => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailParentalcontrol,
            LoginResult::FailLockedEnforced => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailLockedEnforced,
        };

        Ok(Self {
            result: result_if,
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_read_inner<'async_trait, R>(
        mut r: R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = Result<Self, crate::errors::ParseErrorKind>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + tokio::io::AsyncReadExt + Unpin + Send,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // protocol_version: u8
            let _protocol_version = crate::util::tokio_read_u8_le(&mut r).await?;
            // protocol_version is expected to always be 0 (0)

            // result: LoginResult
            let result = crate::util::tokio_read_u8_le(&mut r).await?.try_into()?;

            let result_if = match result {
                LoginResult::Success => {
                    // server_public_key: u8[32]
                    let server_public_key = {
                        let mut server_public_key = [0_u8; 32];
                        r.read_exact(&mut server_public_key).await?;
                        server_public_key
                    };

                    // generator_length: u8
                    let generator_length = crate::util::tokio_read_u8_le(&mut r).await?;

                    // generator: u8[generator_length]
                    let generator = {
                        let mut generator = Vec::with_capacity(generator_length as usize);
                        for _ in 0..generator_length {
                            generator.push(crate::util::tokio_read_u8_le(&mut r).await?);
                        }
                        generator
                    };

                    // large_safe_prime_length: u8
                    let large_safe_prime_length = crate::util::tokio_read_u8_le(&mut r).await?;

                    // large_safe_prime: u8[large_safe_prime_length]
                    let large_safe_prime = {
                        let mut large_safe_prime = Vec::with_capacity(large_safe_prime_length as usize);
                        for _ in 0..large_safe_prime_length {
                            large_safe_prime.push(crate::util::tokio_read_u8_le(&mut r).await?);
                        }
                        large_safe_prime
                    };

                    // salt: u8[32]
                    let salt = {
                        let mut salt = [0_u8; 32];
                        r.read_exact(&mut salt).await?;
                        salt
                    };

                    // crc_salt: u8[16]
                    let crc_salt = {
                        let mut crc_salt = [0_u8; 16];
                        r.read_exact(&mut crc_salt).await?;
                        crc_salt
                    };

                    // security_flag: SecurityFlag
                    let security_flag = SecurityFlag::new(crate::util::tokio_read_u8_le(&mut r).await?);

                    let security_flag_pin = if security_flag.is_pin() {
                        // pin_grid_seed: u32
                        let pin_grid_seed = crate::util::tokio_read_u32_le(&mut r).await?;

                        // pin_salt: u8[16]
                        let pin_salt = {
                            let mut pin_salt = [0_u8; 16];
                            r.read_exact(&mut pin_salt).await?;
                            pin_salt
                        };

                        Some(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin {
                            pin_grid_seed,
                            pin_salt,
                        })
                    }
                    else {
                        None
                    };

                    let security_flag_matrix_card = if security_flag.is_matrix_card() {
                        // width: u8
                        let width = crate::util::tokio_read_u8_le(&mut r).await?;

                        // height: u8
                        let height = crate::util::tokio_read_u8_le(&mut r).await?;

                        // digit_count: u8
                        let digit_count = crate::util::tokio_read_u8_le(&mut r).await?;

                        // challenge_count: u8
                        let challenge_count = crate::util::tokio_read_u8_le(&mut r).await?;

                        // seed: u64
                        let seed = crate::util::tokio_read_u64_le(&mut r).await?;

                        Some(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_MatrixCard {
                            challenge_count,
                            digit_count,
                            height,
                            seed,
                            width,
                        })
                    }
                    else {
                        None
                    };

                    let security_flag_authenticator = if security_flag.is_authenticator() {
                        // required: u8
                        let required = crate::util::tokio_read_u8_le(&mut r).await?;

                        Some(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator {
                            required,
                        })
                    }
                    else {
                        None
                    };

                    let security_flag = CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag {
                        inner: security_flag.as_int(),
                        pin: security_flag_pin,
                        matrix_card: security_flag_matrix_card,
                        authenticator: security_flag_authenticator,
                    };

                    CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                        crc_salt,
                        generator,
                        large_safe_prime,
                        salt,
                        security_flag,
                        server_public_key,
                    }
                }
                LoginResult::FailUnknown0 => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknown0,
                LoginResult::FailUnknown1 => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknown1,
                LoginResult::FailBanned => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailBanned,
                LoginResult::FailUnknownAccount => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknownAccount,
                LoginResult::FailIncorrectPassword => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailIncorrectPassword,
                LoginResult::FailAlreadyOnline => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailAlreadyOnline,
                LoginResult::FailNoTime => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailNoTime,
                LoginResult::FailDbBusy => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailDbBusy,
                LoginResult::FailVersionInvalid => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailVersionInvalid,
                LoginResult::LoginDownloadFile => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::LoginDownloadFile,
                LoginResult::FailInvalidServer => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailInvalidServer,
                LoginResult::FailSuspended => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailSuspended,
                LoginResult::FailNoAccess => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailNoAccess,
                LoginResult::SuccessSurvey => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::SuccessSurvey,
                LoginResult::FailParentalcontrol => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailParentalcontrol,
                LoginResult::FailLockedEnforced => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailLockedEnforced,
            };

            Ok(Self {
                result: result_if,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read_inner<'async_trait, R>(
        mut r: R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = Result<Self, crate::errors::ParseErrorKind>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + async_std::io::ReadExt + Unpin + Send,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // protocol_version: u8
            let _protocol_version = crate::util::astd_read_u8_le(&mut r).await?;
            // protocol_version is expected to always be 0 (0)

            // result: LoginResult
            let result = crate::util::astd_read_u8_le(&mut r).await?.try_into()?;

            let result_if = match result {
                LoginResult::Success => {
                    // server_public_key: u8[32]
                    let server_public_key = {
                        let mut server_public_key = [0_u8; 32];
                        r.read_exact(&mut server_public_key).await?;
                        server_public_key
                    };

                    // generator_length: u8
                    let generator_length = crate::util::astd_read_u8_le(&mut r).await?;

                    // generator: u8[generator_length]
                    let generator = {
                        let mut generator = Vec::with_capacity(generator_length as usize);
                        for _ in 0..generator_length {
                            generator.push(crate::util::astd_read_u8_le(&mut r).await?);
                        }
                        generator
                    };

                    // large_safe_prime_length: u8
                    let large_safe_prime_length = crate::util::astd_read_u8_le(&mut r).await?;

                    // large_safe_prime: u8[large_safe_prime_length]
                    let large_safe_prime = {
                        let mut large_safe_prime = Vec::with_capacity(large_safe_prime_length as usize);
                        for _ in 0..large_safe_prime_length {
                            large_safe_prime.push(crate::util::astd_read_u8_le(&mut r).await?);
                        }
                        large_safe_prime
                    };

                    // salt: u8[32]
                    let salt = {
                        let mut salt = [0_u8; 32];
                        r.read_exact(&mut salt).await?;
                        salt
                    };

                    // crc_salt: u8[16]
                    let crc_salt = {
                        let mut crc_salt = [0_u8; 16];
                        r.read_exact(&mut crc_salt).await?;
                        crc_salt
                    };

                    // security_flag: SecurityFlag
                    let security_flag = SecurityFlag::new(crate::util::astd_read_u8_le(&mut r).await?);

                    let security_flag_pin = if security_flag.is_pin() {
                        // pin_grid_seed: u32
                        let pin_grid_seed = crate::util::astd_read_u32_le(&mut r).await?;

                        // pin_salt: u8[16]
                        let pin_salt = {
                            let mut pin_salt = [0_u8; 16];
                            r.read_exact(&mut pin_salt).await?;
                            pin_salt
                        };

                        Some(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin {
                            pin_grid_seed,
                            pin_salt,
                        })
                    }
                    else {
                        None
                    };

                    let security_flag_matrix_card = if security_flag.is_matrix_card() {
                        // width: u8
                        let width = crate::util::astd_read_u8_le(&mut r).await?;

                        // height: u8
                        let height = crate::util::astd_read_u8_le(&mut r).await?;

                        // digit_count: u8
                        let digit_count = crate::util::astd_read_u8_le(&mut r).await?;

                        // challenge_count: u8
                        let challenge_count = crate::util::astd_read_u8_le(&mut r).await?;

                        // seed: u64
                        let seed = crate::util::astd_read_u64_le(&mut r).await?;

                        Some(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_MatrixCard {
                            challenge_count,
                            digit_count,
                            height,
                            seed,
                            width,
                        })
                    }
                    else {
                        None
                    };

                    let security_flag_authenticator = if security_flag.is_authenticator() {
                        // required: u8
                        let required = crate::util::astd_read_u8_le(&mut r).await?;

                        Some(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator {
                            required,
                        })
                    }
                    else {
                        None
                    };

                    let security_flag = CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag {
                        inner: security_flag.as_int(),
                        pin: security_flag_pin,
                        matrix_card: security_flag_matrix_card,
                        authenticator: security_flag_authenticator,
                    };

                    CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                        crc_salt,
                        generator,
                        large_safe_prime,
                        salt,
                        security_flag,
                        server_public_key,
                    }
                }
                LoginResult::FailUnknown0 => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknown0,
                LoginResult::FailUnknown1 => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknown1,
                LoginResult::FailBanned => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailBanned,
                LoginResult::FailUnknownAccount => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailUnknownAccount,
                LoginResult::FailIncorrectPassword => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailIncorrectPassword,
                LoginResult::FailAlreadyOnline => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailAlreadyOnline,
                LoginResult::FailNoTime => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailNoTime,
                LoginResult::FailDbBusy => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailDbBusy,
                LoginResult::FailVersionInvalid => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailVersionInvalid,
                LoginResult::LoginDownloadFile => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::LoginDownloadFile,
                LoginResult::FailInvalidServer => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailInvalidServer,
                LoginResult::FailSuspended => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailSuspended,
                LoginResult::FailNoAccess => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailNoAccess,
                LoginResult::SuccessSurvey => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::SuccessSurvey,
                LoginResult::FailParentalcontrol => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailParentalcontrol,
                LoginResult::FailLockedEnforced => CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailLockedEnforced,
            };

            Ok(Self {
                result: result_if,
            })
        })
    }

}

impl ServerMessage for CMD_AUTH_LOGON_CHALLENGE_Server {
    const OPCODE: u8 = 0x00;

    fn read<R: Read, I: crate::private::Sealed>(r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r)
    }

    #[cfg(feature = "sync")]
    fn write<W: Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(self.size() + 1);
        self.write_into_vec(&mut v)?;
        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_read<'async_trait, R, I: crate::private::Sealed>(
        r: R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = Result<Self, crate::errors::ParseErrorKind>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + tokio::io::AsyncReadExt + Unpin + Send,
        Self: 'async_trait,
     {
        Self::tokio_read_inner(r)
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
        dyn core::future::Future<Output = Result<Self, crate::errors::ParseErrorKind>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + async_std::io::ReadExt + Unpin + Send,
        Self: 'async_trait,
     {
        Self::astd_read_inner(r)
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

impl CMD_AUTH_LOGON_CHALLENGE_Server {
    pub(crate) fn size(&self) -> usize {
        1 // protocol_version: u8
        + self.result.size() // result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag {
    inner: u8,
    pin: Option<CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin>,
    matrix_card: Option<CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_MatrixCard>,
    authenticator: Option<CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator>,
}

impl CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag {
    pub const fn new(inner: u8, pin: Option<CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin>,matrix_card: Option<CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_MatrixCard>,authenticator: Option<CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator>,) -> Self {
        Self {
            inner,
            pin, 
            matrix_card, 
            authenticator, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            pin: None,
            matrix_card: None,
            authenticator: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.pin.is_none()
        && self.matrix_card.is_none()
        && self.authenticator.is_none()
    }

    pub const fn new_pin(pin: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin) -> Self {
        Self {
            inner: SecurityFlag::PIN,
            pin: Some(pin),
            matrix_card: None,
            authenticator: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pin(mut self, pin: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin) -> Self {
        self.inner |= SecurityFlag::PIN;
        self.pin = Some(pin);
        self
    }

    pub const fn get_pin(&self) -> Option<&CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin> {
        self.pin.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pin(mut self) -> Self {
        self.inner &= SecurityFlag::PIN.reverse_bits();
        self.pin = None;
        self
    }

    pub const fn new_matrix_card(matrix_card: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_MatrixCard) -> Self {
        Self {
            inner: SecurityFlag::MATRIX_CARD,
            pin: None,
            matrix_card: Some(matrix_card),
            authenticator: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_matrix_card(mut self, matrix_card: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_MatrixCard) -> Self {
        self.inner |= SecurityFlag::MATRIX_CARD;
        self.matrix_card = Some(matrix_card);
        self
    }

    pub const fn get_matrix_card(&self) -> Option<&CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_MatrixCard> {
        self.matrix_card.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_matrix_card(mut self) -> Self {
        self.inner &= SecurityFlag::MATRIX_CARD.reverse_bits();
        self.matrix_card = None;
        self
    }

    pub const fn new_authenticator(authenticator: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator) -> Self {
        Self {
            inner: SecurityFlag::AUTHENTICATOR,
            pin: None,
            matrix_card: None,
            authenticator: Some(authenticator),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_authenticator(mut self, authenticator: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator) -> Self {
        self.inner |= SecurityFlag::AUTHENTICATOR;
        self.authenticator = Some(authenticator);
        self
    }

    pub const fn get_authenticator(&self) -> Option<&CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator> {
        self.authenticator.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_authenticator(mut self) -> Self {
        self.inner &= SecurityFlag::AUTHENTICATOR.reverse_bits();
        self.authenticator = None;
        self
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}
impl CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag {
    pub(crate) const fn size(&self) -> usize {
        1 // inner
        + {
            if let Some(s) = &self.pin {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.matrix_card {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.authenticator {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin {
    pub pin_grid_seed: u32,
    pub pin_salt: [u8; 16],
}

impl CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin {
    pub(crate) const fn size(&self) -> usize {
        4 // pin_grid_seed: u32
        + 16 // pin_salt: u8[16]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_MatrixCard {
    pub challenge_count: u8,
    pub digit_count: u8,
    pub height: u8,
    pub seed: u64,
    pub width: u8,
}

impl CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_MatrixCard {
    pub(crate) const fn size(&self) -> usize {
        1 // challenge_count: u8
        + 1 // digit_count: u8
        + 1 // height: u8
        + 8 // seed: u64
        + 1 // width: u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator {
    pub required: u8,
}

impl CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator {
    pub(crate) const fn size(&self) -> usize {
        1 // required: u8
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult {
    Success {
        crc_salt: [u8; 16],
        generator: Vec<u8>,
        large_safe_prime: Vec<u8>,
        salt: [u8; 32],
        security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag,
        server_public_key: [u8; 32],
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

impl Default for CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::FailUnknown0
    }
}

impl CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult {
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

impl std::fmt::Display for CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult {
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

impl CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Success {
                generator,
                large_safe_prime,
                security_flag,
                ..
            } => {
                1
                + 16 // crc_salt: u8[16]
                + generator.len() * core::mem::size_of::<u8>() // generator: u8[generator_length]
                + 1 // generator_length: u8
                + large_safe_prime.len() * core::mem::size_of::<u8>() // large_safe_prime: u8[large_safe_prime_length]
                + 1 // large_safe_prime_length: u8
                + 32 // salt: u8[32]
                + security_flag.size() // security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag
                + 32 // server_public_key: u8[32]
            }
            _ => 1,
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_AUTH_LOGON_CHALLENGE_Server;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_8::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    fn assert(t: &CMD_AUTH_LOGON_CHALLENGE_Server, expected: &CMD_AUTH_LOGON_CHALLENGE_Server) {
        assert_eq!(t.result, expected.result);
    }

    const RAW0: [u8; 119] = [ 0x00, 0x00, 0x00, 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C,
         0x2B, 0xCE, 0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
         0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87, 0xCE, 0xDA,
         0x34, 0x46, 0x01, 0x07, 0x20, 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
         0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53, 0x50, 0x06,
         0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1, 0x89, 0x5E, 0x64, 0x4B,
         0x89, 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D, 0xB8,
         0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B, 0xCF, 0x74, 0xD6,
         0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30, 0x90, 0x87, 0xBA, 0xA3, 0x1E,
         0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC, 0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2,
         0xF1, 0x00, ];

    pub(crate) fn expected0() -> CMD_AUTH_LOGON_CHALLENGE_Server {
        CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
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
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 276.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_logon_challenge_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 276.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_logon_challenge_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 276.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_logon_challenge_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 139] = [ 0x00, 0x00, 0x00, 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C,
         0x2B, 0xCE, 0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
         0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87, 0xCE, 0xDA,
         0x34, 0x46, 0x01, 0x07, 0x20, 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
         0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53, 0x50, 0x06,
         0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1, 0x89, 0x5E, 0x64, 0x4B,
         0x89, 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D, 0xB8,
         0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B, 0xCF, 0x74, 0xD6,
         0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30, 0x90, 0x87, 0xBA, 0xA3, 0x1E,
         0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC, 0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2,
         0xF1, 0x01, 0xEF, 0xBE, 0xAD, 0xDE, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
         0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ];

    pub(crate) fn expected1() -> CMD_AUTH_LOGON_CHALLENGE_Server {
        CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
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
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    .set_pin(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin {
                        pin_grid_seed: 0xDEADBEEF,
                        pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                             0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                    })
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 329.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_logon_challenge_server1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 329.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_logon_challenge_server1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 329.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_logon_challenge_server1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    const RAW2: [u8; 120] = [ 0x00, 0x00, 0x00, 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C,
         0x2B, 0xCE, 0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
         0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87, 0xCE, 0xDA,
         0x34, 0x46, 0x01, 0x07, 0x20, 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
         0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53, 0x50, 0x06,
         0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1, 0x89, 0x5E, 0x64, 0x4B,
         0x89, 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D, 0xB8,
         0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B, 0xCF, 0x74, 0xD6,
         0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30, 0x90, 0x87, 0xBA, 0xA3, 0x1E,
         0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC, 0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2,
         0xF1, 0x04, 0x01, ];

    pub(crate) fn expected2() -> CMD_AUTH_LOGON_CHALLENGE_Server {
        CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
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
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    .set_authenticator(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator {
                        required: 0x1,
                    })
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 389.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_logon_challenge_server2() {
        let expected = expected2();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW2)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 389.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_logon_challenge_server2() {
        let expected = expected2();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 389.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_logon_challenge_server2() {
        let expected = expected2();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    const RAW3: [u8; 131] = [ 0x00, 0x00, 0x00, 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C,
         0x2B, 0xCE, 0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
         0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87, 0xCE, 0xDA,
         0x34, 0x46, 0x01, 0x07, 0x20, 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
         0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53, 0x50, 0x06,
         0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1, 0x89, 0x5E, 0x64, 0x4B,
         0x89, 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D, 0xB8,
         0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B, 0xCF, 0x74, 0xD6,
         0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30, 0x90, 0x87, 0xBA, 0xA3, 0x1E,
         0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC, 0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2,
         0xF1, 0x02, 0xFF, 0xEE, 0xDD, 0xCC, 0xDE, 0xCA, 0xFA, 0xEF, 0xBE, 0xAD,
         0xDE, 0x00, ];

    pub(crate) fn expected3() -> CMD_AUTH_LOGON_CHALLENGE_Server {
        CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
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
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    .set_matrix_card(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_MatrixCard {
                        challenge_count: 0xCC,
                        digit_count: 0xDD,
                        height: 0xEE,
                        seed: 0xDEADBEEFFACADE,
                        width: 0xFF,
                    })
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 444.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_logon_challenge_server3() {
        let expected = expected3();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW3)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW3);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 444.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_logon_challenge_server3() {
        let expected = expected3();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW3)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW3);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 444.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_logon_challenge_server3() {
        let expected = expected3();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW3)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW3);
    }

    const RAW4: [u8; 3] = [ 0x00, 0x00, 0x05, ];

    pub(crate) fn expected4() -> CMD_AUTH_LOGON_CHALLENGE_Server {
        CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailIncorrectPassword,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 508.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_logon_challenge_server4() {
        let expected = expected4();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW4)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW4.len());

        let mut dest = Vec::with_capacity(RAW4.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW4);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 508.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_logon_challenge_server4() {
        let expected = expected4();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW4)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW4.len());

        let mut dest = Vec::with_capacity(RAW4.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW4);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 508.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_logon_challenge_server4() {
        let expected = expected4();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW4)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW4.len());

        let mut dest = Vec::with_capacity(RAW4.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW4);
    }

    const RAW5: [u8; 132] = [ 0x00, 0x00, 0x00, 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C,
         0x2B, 0xCE, 0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
         0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87, 0xCE, 0xDA,
         0x34, 0x46, 0x01, 0x07, 0x20, 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
         0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53, 0x50, 0x06,
         0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1, 0x89, 0x5E, 0x64, 0x4B,
         0x89, 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D, 0xB8,
         0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B, 0xCF, 0x74, 0xD6,
         0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30, 0x90, 0x87, 0xBA, 0xA3, 0x1E,
         0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC, 0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2,
         0xF1, 0x06, 0xFF, 0xEE, 0xDD, 0xCC, 0xDE, 0xCA, 0xFA, 0xEF, 0xBE, 0xAD,
         0xDE, 0x00, 0x01, ];

    pub(crate) fn expected5() -> CMD_AUTH_LOGON_CHALLENGE_Server {
        CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
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
                security_flag: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag::empty()
                    .set_matrix_card(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_MatrixCard {
                        challenge_count: 0xCC,
                        digit_count: 0xDD,
                        height: 0xEE,
                        seed: 0xDEADBEEFFACADE,
                        width: 0xFF,
                    })
                    .set_authenticator(CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Authenticator {
                        required: 0x1,
                    })
                    ,
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
            },
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 518.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_logon_challenge_server5() {
        let expected = expected5();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW5)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW5.len());

        let mut dest = Vec::with_capacity(RAW5.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW5);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 518.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_logon_challenge_server5() {
        let expected = expected5();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW5)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW5.len());

        let mut dest = Vec::with_capacity(RAW5.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW5);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 518.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_logon_challenge_server5() {
        let expected = expected5();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW5)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW5.len());

        let mut dest = Vec::with_capacity(RAW5.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW5);
    }

    const RAW6: [u8; 3] = [ 0x00, 0x00, 0x05, ];

    pub(crate) fn expected6() -> CMD_AUTH_LOGON_CHALLENGE_Server {
        CMD_AUTH_LOGON_CHALLENGE_Server {
            result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::FailIncorrectPassword,
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 585.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_logon_challenge_server6() {
        let expected = expected6();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW6)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW6.len());

        let mut dest = Vec::with_capacity(RAW6.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW6);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 585.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_logon_challenge_server6() {
        let expected = expected6();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW6)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW6.len());

        let mut dest = Vec::with_capacity(RAW6.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW6);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 585.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_logon_challenge_server6() {
        let expected = expected6();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW6)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW6.len());

        let mut dest = Vec::with_capacity(RAW6.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW6);
    }

}

