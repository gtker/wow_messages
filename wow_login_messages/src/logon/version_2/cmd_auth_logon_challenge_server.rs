use crate::Message;
use crate::ServerMessage;
use std::io::{Read, Write};

use crate::logon::version_2::LoginResult;

/// Reply to [`CMD_AUTH_LOGON_CHALLENGE_Client`](crate::logon::all::CMD_AUTH_LOGON_CHALLENGE_Client).
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm#L2):
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
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CMD_AUTH_LOGON_CHALLENGE_Server {
    Success {
        crc_salt: [u8; 16],
        generator: Vec<u8>,
        large_safe_prime: Vec<u8>,
        salt: [u8; 32],
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
        w.write_all(&(self.as_int().to_le_bytes()))?;

        match &self {
            CMD_AUTH_LOGON_CHALLENGE_Server::Success {
                crc_salt,
                generator,
                large_safe_prime,
                salt,
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

            }
            _ => {}
        }

        Ok(())
    }
}

impl crate::private::Sealed for CMD_AUTH_LOGON_CHALLENGE_Server {}

impl CMD_AUTH_LOGON_CHALLENGE_Server {
    #[cfg(feature = "sync")]
    fn read_inner<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
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

                CMD_AUTH_LOGON_CHALLENGE_Server::Success {
                    crc_salt,
                    generator,
                    large_safe_prime,
                    salt,
                    server_public_key,
                }
            }
            LoginResult::FailUnknown0 => CMD_AUTH_LOGON_CHALLENGE_Server::FailUnknown0,
            LoginResult::FailUnknown1 => CMD_AUTH_LOGON_CHALLENGE_Server::FailUnknown1,
            LoginResult::FailBanned => CMD_AUTH_LOGON_CHALLENGE_Server::FailBanned,
            LoginResult::FailUnknownAccount => CMD_AUTH_LOGON_CHALLENGE_Server::FailUnknownAccount,
            LoginResult::FailIncorrectPassword => CMD_AUTH_LOGON_CHALLENGE_Server::FailIncorrectPassword,
            LoginResult::FailAlreadyOnline => CMD_AUTH_LOGON_CHALLENGE_Server::FailAlreadyOnline,
            LoginResult::FailNoTime => CMD_AUTH_LOGON_CHALLENGE_Server::FailNoTime,
            LoginResult::FailDbBusy => CMD_AUTH_LOGON_CHALLENGE_Server::FailDbBusy,
            LoginResult::FailVersionInvalid => CMD_AUTH_LOGON_CHALLENGE_Server::FailVersionInvalid,
            LoginResult::LoginDownloadFile => CMD_AUTH_LOGON_CHALLENGE_Server::LoginDownloadFile,
            LoginResult::FailInvalidServer => CMD_AUTH_LOGON_CHALLENGE_Server::FailInvalidServer,
            LoginResult::FailSuspended => CMD_AUTH_LOGON_CHALLENGE_Server::FailSuspended,
            LoginResult::FailNoAccess => CMD_AUTH_LOGON_CHALLENGE_Server::FailNoAccess,
            LoginResult::SuccessSurvey => CMD_AUTH_LOGON_CHALLENGE_Server::SuccessSurvey,
            LoginResult::FailParentalcontrol => CMD_AUTH_LOGON_CHALLENGE_Server::FailParentalcontrol,
        };

        Ok(result_if)
    }

    #[cfg(feature = "tokio")]
    async fn tokio_read_inner<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
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

                CMD_AUTH_LOGON_CHALLENGE_Server::Success {
                    crc_salt,
                    generator,
                    large_safe_prime,
                    salt,
                    server_public_key,
                }
            }
            LoginResult::FailUnknown0 => CMD_AUTH_LOGON_CHALLENGE_Server::FailUnknown0,
            LoginResult::FailUnknown1 => CMD_AUTH_LOGON_CHALLENGE_Server::FailUnknown1,
            LoginResult::FailBanned => CMD_AUTH_LOGON_CHALLENGE_Server::FailBanned,
            LoginResult::FailUnknownAccount => CMD_AUTH_LOGON_CHALLENGE_Server::FailUnknownAccount,
            LoginResult::FailIncorrectPassword => CMD_AUTH_LOGON_CHALLENGE_Server::FailIncorrectPassword,
            LoginResult::FailAlreadyOnline => CMD_AUTH_LOGON_CHALLENGE_Server::FailAlreadyOnline,
            LoginResult::FailNoTime => CMD_AUTH_LOGON_CHALLENGE_Server::FailNoTime,
            LoginResult::FailDbBusy => CMD_AUTH_LOGON_CHALLENGE_Server::FailDbBusy,
            LoginResult::FailVersionInvalid => CMD_AUTH_LOGON_CHALLENGE_Server::FailVersionInvalid,
            LoginResult::LoginDownloadFile => CMD_AUTH_LOGON_CHALLENGE_Server::LoginDownloadFile,
            LoginResult::FailInvalidServer => CMD_AUTH_LOGON_CHALLENGE_Server::FailInvalidServer,
            LoginResult::FailSuspended => CMD_AUTH_LOGON_CHALLENGE_Server::FailSuspended,
            LoginResult::FailNoAccess => CMD_AUTH_LOGON_CHALLENGE_Server::FailNoAccess,
            LoginResult::SuccessSurvey => CMD_AUTH_LOGON_CHALLENGE_Server::SuccessSurvey,
            LoginResult::FailParentalcontrol => CMD_AUTH_LOGON_CHALLENGE_Server::FailParentalcontrol,
        };

        Ok(result_if)
    }

    #[cfg(feature = "async-std")]
    async fn astd_read_inner<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
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

                CMD_AUTH_LOGON_CHALLENGE_Server::Success {
                    crc_salt,
                    generator,
                    large_safe_prime,
                    salt,
                    server_public_key,
                }
            }
            LoginResult::FailUnknown0 => CMD_AUTH_LOGON_CHALLENGE_Server::FailUnknown0,
            LoginResult::FailUnknown1 => CMD_AUTH_LOGON_CHALLENGE_Server::FailUnknown1,
            LoginResult::FailBanned => CMD_AUTH_LOGON_CHALLENGE_Server::FailBanned,
            LoginResult::FailUnknownAccount => CMD_AUTH_LOGON_CHALLENGE_Server::FailUnknownAccount,
            LoginResult::FailIncorrectPassword => CMD_AUTH_LOGON_CHALLENGE_Server::FailIncorrectPassword,
            LoginResult::FailAlreadyOnline => CMD_AUTH_LOGON_CHALLENGE_Server::FailAlreadyOnline,
            LoginResult::FailNoTime => CMD_AUTH_LOGON_CHALLENGE_Server::FailNoTime,
            LoginResult::FailDbBusy => CMD_AUTH_LOGON_CHALLENGE_Server::FailDbBusy,
            LoginResult::FailVersionInvalid => CMD_AUTH_LOGON_CHALLENGE_Server::FailVersionInvalid,
            LoginResult::LoginDownloadFile => CMD_AUTH_LOGON_CHALLENGE_Server::LoginDownloadFile,
            LoginResult::FailInvalidServer => CMD_AUTH_LOGON_CHALLENGE_Server::FailInvalidServer,
            LoginResult::FailSuspended => CMD_AUTH_LOGON_CHALLENGE_Server::FailSuspended,
            LoginResult::FailNoAccess => CMD_AUTH_LOGON_CHALLENGE_Server::FailNoAccess,
            LoginResult::SuccessSurvey => CMD_AUTH_LOGON_CHALLENGE_Server::SuccessSurvey,
            LoginResult::FailParentalcontrol => CMD_AUTH_LOGON_CHALLENGE_Server::FailParentalcontrol,
        };

        Ok(result_if)
    }

}

impl Message for CMD_AUTH_LOGON_CHALLENGE_Server {
    const OPCODE: u8 = 0x00;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read, I: crate::private::Sealed>(r: R) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r).map_err(|kind| crate::errors::ParseError::new(0, "CMD_AUTH_LOGON_CHALLENGE_Server", kind))
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
        Box::pin(async move {Self::tokio_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(0, "CMD_AUTH_LOGON_CHALLENGE_Server", kind))})
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
        Box::pin(async move {Self::astd_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(0, "CMD_AUTH_LOGON_CHALLENGE_Server", kind))})
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

impl ServerMessage for CMD_AUTH_LOGON_CHALLENGE_Server {}
impl CMD_AUTH_LOGON_CHALLENGE_Server {
    pub(crate) fn size(&self) -> usize {
        1 // protocol_version: u8
        + (match self {
            Self::Success {
                generator,
                large_safe_prime,
                ..
            } => {
                1
                + 16 // crc_salt: u8[16]
                + generator.len() * core::mem::size_of::<u8>() // generator: u8[generator_length]
                + 1 // generator_length: u8
                + large_safe_prime.len() * core::mem::size_of::<u8>() // large_safe_prime: u8[large_safe_prime_length]
                + 1 // large_safe_prime_length: u8
                + 32 // salt: u8[32]
                + 32 // server_public_key: u8[32]
            }
            _ => 1,
        }) // result: CMD_AUTH_LOGON_CHALLENGE_Server
    }
}

impl Default for CMD_AUTH_LOGON_CHALLENGE_Server {
    fn default() -> Self {
        // First enumerator without any fields
        Self::FailUnknown0
    }
}

impl CMD_AUTH_LOGON_CHALLENGE_Server {
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

impl std::fmt::Display for CMD_AUTH_LOGON_CHALLENGE_Server {
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
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_AUTH_LOGON_CHALLENGE_Server;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_2::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 118] = [ 0x00, 0x00, 0x00, 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C,
         0x2B, 0xCE, 0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
         0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87, 0xCE, 0xDA,
         0x34, 0x46, 0x01, 0x07, 0x20, 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
         0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53, 0x50, 0x06,
         0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1, 0x89, 0x5E, 0x64, 0x4B,
         0x89, 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D, 0xB8,
         0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B, 0xCF, 0x74, 0xD6,
         0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30, 0x90, 0x87, 0xBA, 0xA3, 0x1E,
         0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC, 0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2,
         0xF1, ];

    pub(crate) fn expected0() -> CMD_AUTH_LOGON_CHALLENGE_Server {
        CMD_AUTH_LOGON_CHALLENGE_Server::Success {
            crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC, 0x37,
                 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
            generator: vec![ 0x07, ],
            large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C, 0xAB,
                 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53, 0x50, 0x06,
                 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1, 0x89, 0x5E, 0x64,
                 0x4B, 0x89, ],
            salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D, 0xB8,
                 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B, 0xCF, 0x74,
                 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30, 0x90, 0x87, ],
            server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                 0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78, 0x46,
                 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87, 0xCE, 0xDA,
                 0x34, 0x46, ],
        }
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 26.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_logon_challenge_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 26.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_logon_challenge_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 26.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_logon_challenge_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

