use crate::Message;
use crate::ServerMessage;
use std::io::{Read, Write};

use crate::logon::version_2::LoginResult;
use crate::logon::version_5::SecurityFlag;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm:219`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm#L219):
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

                let security_flag = CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag {
                    inner: security_flag.as_int(),
                    pin: security_flag_pin,
                    matrix_card: security_flag_matrix_card,
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
        };

        Ok(Self {
            result: result_if,
        })
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

                let security_flag = CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag {
                    inner: security_flag.as_int(),
                    pin: security_flag_pin,
                    matrix_card: security_flag_matrix_card,
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
        };

        Ok(Self {
            result: result_if,
        })
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

                let security_flag = CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag {
                    inner: security_flag.as_int(),
                    pin: security_flag_pin,
                    matrix_card: security_flag_matrix_card,
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
        };

        Ok(Self {
            result: result_if,
        })
    }

}

impl Message for CMD_AUTH_LOGON_CHALLENGE_Server {
    const OPCODE: u8 = 0x00;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;

        let mut s = String::new();

        writeln!(s, "test CMD_AUTH_LOGON_CHALLENGE_Server {{").unwrap();
        // Members
        writeln!(s, "    result = {};", LoginResult::try_from(self.result.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.result {
            crate::logon::version_5::CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                crc_salt,
                generator,
                large_safe_prime,
                salt,
                security_flag,
                server_public_key,
            } => {
                writeln!(s, "    server_public_key = [").unwrap();
                for v in server_public_key.as_slice() {
                    write!(s, "{v:#04X}, ").unwrap();
                }
                writeln!(s, "    ];").unwrap();
                writeln!(s, "    generator_length = {};", generator.len()).unwrap();
                writeln!(s, "    generator = [").unwrap();
                for v in generator.as_slice() {
                    write!(s, "{v:#04X}, ").unwrap();
                }
                writeln!(s, "    ];").unwrap();
                writeln!(s, "    large_safe_prime_length = {};", large_safe_prime.len()).unwrap();
                writeln!(s, "    large_safe_prime = [").unwrap();
                for v in large_safe_prime.as_slice() {
                    write!(s, "{v:#04X}, ").unwrap();
                }
                writeln!(s, "    ];").unwrap();
                writeln!(s, "    salt = [").unwrap();
                for v in salt.as_slice() {
                    write!(s, "{v:#04X}, ").unwrap();
                }
                writeln!(s, "    ];").unwrap();
                writeln!(s, "    crc_salt = [").unwrap();
                for v in crc_salt.as_slice() {
                    write!(s, "{v:#04X}, ").unwrap();
                }
                writeln!(s, "    ];").unwrap();
                writeln!(s, "    security_flag = {};", SecurityFlag::new(security_flag.as_int()).as_test_case_value()).unwrap();
                if let Some(if_statement) = &security_flag.get_pin() {
                    writeln!(s, "    pin_grid_seed = {};", if_statement.pin_grid_seed).unwrap();
                    writeln!(s, "    pin_salt = [").unwrap();
                    for v in if_statement.pin_salt.as_slice() {
                        write!(s, "{v:#04X}, ").unwrap();
                    }
                    writeln!(s, "    ];").unwrap();
                }

                if let Some(if_statement) = &security_flag.get_matrix_card() {
                    writeln!(s, "    width = {};", if_statement.width).unwrap();
                    writeln!(s, "    height = {};", if_statement.height).unwrap();
                    writeln!(s, "    digit_count = {};", if_statement.digit_count).unwrap();
                    writeln!(s, "    challenge_count = {};", if_statement.challenge_count).unwrap();
                    writeln!(s, "    seed = {};", if_statement.seed).unwrap();
                }

            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        writeln!(s, "    {:#04X}, /* opcode */ ", bytes.next().unwrap()).unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 1, "protocol_version", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "result", "    ");
        match &self.result {
            crate::logon::version_5::CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult::Success {
                crc_salt,
                generator,
                large_safe_prime,
                salt,
                security_flag,
                server_public_key,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, server_public_key.len(), "server_public_key", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "generator_length", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, generator.len(), "generator", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "large_safe_prime_length", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, large_safe_prime.len(), "large_safe_prime", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, salt.len(), "salt", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, crc_salt.len(), "crc_salt", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "security_flag", "    ");
                if let Some(if_statement) = &security_flag.get_pin() {
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "pin_grid_seed", "    ");
                    crate::util::write_bytes(&mut s, &mut bytes, if_statement.pin_salt.len(), "pin_salt", "    ");
                }

                if let Some(if_statement) = &security_flag.get_matrix_card() {
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "width", "    ");
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "height", "    ");
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "digit_count", "    ");
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "challenge_count", "    ");
                    crate::util::write_bytes(&mut s, &mut bytes, 8, "seed", "    ");
                }

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
        + self.result.size() // result: CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag {
    inner: u8,
    pin: Option<CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin>,
    matrix_card: Option<CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_MatrixCard>,
}

impl CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag {
    pub const fn new(inner: u8, pin: Option<CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin>,matrix_card: Option<CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_MatrixCard>,) -> Self {
        Self {
            inner,
            pin, 
            matrix_card, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            pin: None,
            matrix_card: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.pin.is_none()
        && self.matrix_card.is_none()
    }

    pub const fn new_pin(pin: CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin) -> Self {
        Self {
            inner: SecurityFlag::PIN,
            pin: Some(pin),
            matrix_card: None,
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

