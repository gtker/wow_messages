use std::convert::{TryFrom, TryInto};
use crate::logon::version_8::{LoginResult, LoginResultError};
use crate::logon::version_8::{SecurityFlag};
use crate::ServerMessage;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm:203`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm#L203):
/// ```text
/// slogin CMD_AUTH_LOGON_CHALLENGE_Server = 0x0 {
///     u8 protocol_version = 0;
///     LoginResult login_result;
///     if (login_result == SUCCESS) {
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
///         if (security_flag & UNKNOWN0) {
///             u8 unknown0;
///             u8 unknown1;
///             u8 unknown2;
///             u8 unknown3;
///             u64 unknown4;
///         }
///         if (security_flag & AUTHENTICATOR) {
///             u8 unknown5 = 1;
///         }
///     }
/// }
/// ```
pub struct CMD_AUTH_LOGON_CHALLENGE_Server {
    pub login_result: CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult,
}

impl ServerMessage for CMD_AUTH_LOGON_CHALLENGE_Server {
    const OPCODE: u8 = 0x00;
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
    /// **This field is not in the struct, but is written as this constant value.**
    pub const PROTOCOL_VERSION_VALUE: u8 = 0x00;

    /// The field `unknown5` is constantly specified to be:
    /// 
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `1` |
    /// | Hex | `0x01` |
    /// | Original | `1` |
    /// 
    /// **This field is not in the struct, but is written as this constant value.**
    pub const UNKNOWN5_VALUE: u8 = 0x01;

}

impl ReadableAndWritable for CMD_AUTH_LOGON_CHALLENGE_Server {
    type Error = CMD_AUTH_LOGON_CHALLENGE_ServerError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // protocol_version: u8
        let _protocol_version = crate::util::read_u8_le(r)?;
        // protocol_version is expected to always be 0 (0)

        // login_result: LoginResult
        let login_result = LoginResult::read(r)?;

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
                let security_flag = SecurityFlag::read(r)?;

                let security_flag_PIN = if security_flag.is_PIN() {
                    // pin_grid_seed: u32
                    let pin_grid_seed = crate::util::read_u32_le(r)?;

                    // pin_salt: u8[16]
                    let mut pin_salt = [0_u8; 16];
                    r.read_exact(&mut pin_salt)?;

                    Some(CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagPIN {
                        pin_grid_seed,
                        pin_salt,
                    })
                } else {
                    None
                };

                let security_flag_UNKNOWN0 = if security_flag.is_UNKNOWN0() {
                    // unknown0: u8
                    let unknown0 = crate::util::read_u8_le(r)?;

                    // unknown1: u8
                    let unknown1 = crate::util::read_u8_le(r)?;

                    // unknown2: u8
                    let unknown2 = crate::util::read_u8_le(r)?;

                    // unknown3: u8
                    let unknown3 = crate::util::read_u8_le(r)?;

                    // unknown4: u64
                    let unknown4 = crate::util::read_u64_le(r)?;

                    Some(CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagUNKNOWN0 {
                        unknown0,
                        unknown1,
                        unknown2,
                        unknown3,
                        unknown4,
                    })
                } else {
                    None
                };

                let security_flag_AUTHENTICATOR = if security_flag.is_AUTHENTICATOR() {
                    // unknown5: u8
                    let _unknown5 = crate::util::read_u8_le(r)?;
                    // unknown5 is expected to always be 1 (1)

                    Some(CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagAUTHENTICATOR {
                        unknown5: _unknown5,
                    })
                } else {
                    None
                };

                let security_flag = CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag {
                    inner: security_flag.as_u8(),
                    pin: security_flag_PIN,
                    unknown0: security_flag_UNKNOWN0,
                    authenticator: security_flag_AUTHENTICATOR,
                };

                CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS {
                    server_public_key,
                    generator,
                    large_safe_prime,
                    salt,
                    crc_salt,
                    security_flag,
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
            LoginResult::FAIL_LOCKED_ENFORCED => CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_LOCKED_ENFORCED,
        };

        Ok(Self {
            login_result: login_result_if,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // protocol_version: u8
        w.write_all(&Self::PROTOCOL_VERSION_VALUE.to_le_bytes())?;

        // login_result: LoginResult
        self.login_result.write(w)?;

        match &self.login_result {
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS {
                server_public_key,
                generator,
                large_safe_prime,
                salt,
                crc_salt,
                security_flag,
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
                security_flag.write(w)?;

                if let Some(s) = &security_flag.pin {
                    s.write(w)?;
                }

                if let Some(s) = &security_flag.unknown0 {
                    s.write(w)?;
                }

                if let Some(s) = &security_flag.authenticator {
                    s.write(w)?;
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
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_LOCKED_ENFORCED => {}
        }

        Ok(())
    }

}

impl VariableSized for CMD_AUTH_LOGON_CHALLENGE_Server {
    fn size(&self) -> usize {
        1 // protocol_version: u8
        + self.login_result.size() // login_result: LoginResult and subfields
    }
}

impl MaximumPossibleSized for CMD_AUTH_LOGON_CHALLENGE_Server {
    fn maximum_possible_size() -> usize {
        1 // protocol_version: u8
        + LoginResult::maximum_possible_size() // login_result: LoginResult
    }
}

#[derive(Debug)]
pub enum CMD_AUTH_LOGON_CHALLENGE_ServerError {
    Io(std::io::Error),
    LoginResult(LoginResultError),
}

impl std::error::Error for CMD_AUTH_LOGON_CHALLENGE_ServerError {}
impl std::fmt::Display for CMD_AUTH_LOGON_CHALLENGE_ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::LoginResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMD_AUTH_LOGON_CHALLENGE_ServerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<LoginResultError> for CMD_AUTH_LOGON_CHALLENGE_ServerError {
    fn from(e: LoginResultError) -> Self {
        Self::LoginResult(e)
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag {
    inner: u8,
    pin: Option<CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagPIN>,
    unknown0: Option<CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagUNKNOWN0>,
    authenticator: Option<CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagAUTHENTICATOR>,
}

impl From<&CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag> for SecurityFlag {
    fn from(e: &CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag) -> Self {
        Self::new(e.inner)
    }
}

impl CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SecurityFlag = self.into();
        a.write(w)?;
        Ok(())
    }

    pub const fn new_NONE() -> Self {
        Self {
            inner: SecurityFlag::NONE,
            pin: None,
            unknown0: None,
            authenticator: None,
        }
    }

    pub const fn new_PIN(pin: CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagPIN) -> Self {
        Self {
            inner: SecurityFlag::PIN,
            pin: Some(pin),
            unknown0: None,
            authenticator: None,
        }
    }

    pub const fn new_UNKNOWN0(unknown0: CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagUNKNOWN0) -> Self {
        Self {
            inner: SecurityFlag::UNKNOWN0,
            pin: None,
            unknown0: Some(unknown0),
            authenticator: None,
        }
    }

    pub const fn new_AUTHENTICATOR(authenticator: CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagAUTHENTICATOR) -> Self {
        Self {
            inner: SecurityFlag::AUTHENTICATOR,
            pin: None,
            unknown0: None,
            authenticator: Some(authenticator),
        }
    }

}
impl VariableSized for CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag {
    fn size(&self) -> usize {
        1 // inner: SecurityFlag (u8)
        + {
            if let Some(s) = &self.pin {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.unknown0 {
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

impl MaximumPossibleSized for CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag {
    fn maximum_possible_size() -> usize {
        1 // inner: SecurityFlag (u8)
        + CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagPIN::maximum_possible_size() // PIN enumerator
        + CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagUNKNOWN0::maximum_possible_size() // UNKNOWN0 enumerator
        + CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagAUTHENTICATOR::maximum_possible_size() // AUTHENTICATOR enumerator
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagPIN {
    pub pin_grid_seed: u32,
    pub pin_salt: [u8; 16],
}

impl VariableSized for CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagPIN {
    fn size(&self) -> usize {
        4 // pin_grid_seed: u32
        + 16 * core::mem::size_of::<u8>() // pin_salt: u8[16]
    }
}

impl MaximumPossibleSized for CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagPIN {
    fn maximum_possible_size() -> usize {
        4 // pin_grid_seed: u32
        + 16 * core::mem::size_of::<u8>() // pin_salt: u8[16]
    }
}

impl CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagPIN {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.pin_grid_seed.to_le_bytes())?;

        for i in self.pin_salt.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagUNKNOWN0 {
    pub unknown0: u8,
    pub unknown1: u8,
    pub unknown2: u8,
    pub unknown3: u8,
    pub unknown4: u64,
}

impl VariableSized for CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagUNKNOWN0 {
    fn size(&self) -> usize {
        1 // unknown0: u8
        + 1 // unknown1: u8
        + 1 // unknown2: u8
        + 1 // unknown3: u8
        + 8 // unknown4: u64
    }
}

impl MaximumPossibleSized for CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagUNKNOWN0 {
    fn maximum_possible_size() -> usize {
        1 // unknown0: u8
        + 1 // unknown1: u8
        + 1 // unknown2: u8
        + 1 // unknown3: u8
        + 8 // unknown4: u64
    }
}

impl CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagUNKNOWN0 {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.unknown0.to_le_bytes())?;

        w.write_all(&self.unknown1.to_le_bytes())?;

        w.write_all(&self.unknown2.to_le_bytes())?;

        w.write_all(&self.unknown3.to_le_bytes())?;

        w.write_all(&self.unknown4.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagAUTHENTICATOR {
    pub unknown5: u8,
}

impl VariableSized for CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagAUTHENTICATOR {
    fn size(&self) -> usize {
        1 // unknown5: u8
    }
}

impl MaximumPossibleSized for CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagAUTHENTICATOR {
    fn maximum_possible_size() -> usize {
        1 // unknown5: u8
    }
}

impl CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagAUTHENTICATOR {
    /// The field `unknown5` is constantly specified to be:
    /// 
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `1` |
    /// | Hex | `0x01` |
    /// | Original | `1` |
    /// 
    /// **This field is not in the struct, but is written as this constant value.**
    pub const UNKNOWN5_VALUE: u8 = 0x01;

    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&Self::UNKNOWN5_VALUE.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult {
    SUCCESS {
        server_public_key: [u8; 32],
        generator: Vec<u8>,
        large_safe_prime: Vec<u8>,
        salt: [u8; 32],
        crc_salt: [u8; 16],
        security_flag: CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag,
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

impl From<&LoginResult> for CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult {
    fn from(e: &LoginResult) -> Self {
        match &e {
            LoginResult::SUCCESS => Self::SUCCESS {
                server_public_key: Default::default(),
                generator: Default::default(),
                large_safe_prime: Default::default(),
                salt: Default::default(),
                crc_salt: Default::default(),
                security_flag: Default::default(),
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

impl From<&CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult> for LoginResult {
    fn from(v: &CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult) -> Self {
        match &v {
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS { .. } => Self::SUCCESS,
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN0 => Self::FAIL_UNKNOWN0,
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN1 => Self::FAIL_UNKNOWN1,
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_BANNED => Self::FAIL_BANNED,
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_UNKNOWN_ACCOUNT => Self::FAIL_UNKNOWN_ACCOUNT,
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_INCORRECT_PASSWORD => Self::FAIL_INCORRECT_PASSWORD,
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_ALREADY_ONLINE => Self::FAIL_ALREADY_ONLINE,
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_NO_TIME => Self::FAIL_NO_TIME,
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_DB_BUSY => Self::FAIL_DB_BUSY,
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_VERSION_INVALID => Self::FAIL_VERSION_INVALID,
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::LOGIN_DOWNLOAD_FILE => Self::LOGIN_DOWNLOAD_FILE,
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_INVALID_SERVER => Self::FAIL_INVALID_SERVER,
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_SUSPENDED => Self::FAIL_SUSPENDED,
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_NO_ACCESS => Self::FAIL_NO_ACCESS,
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS_SURVEY => Self::SUCCESS_SURVEY,
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_PARENTALCONTROL => Self::FAIL_PARENTALCONTROL,
            CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_LOCKED_ENFORCED => Self::FAIL_LOCKED_ENFORCED,
        }
    }
}

impl Default for CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::SUCCESS {
            server_public_key: Default::default(),
            generator: Default::default(),
            large_safe_prime: Default::default(),
            salt: Default::default(),
            crc_salt: Default::default(),
            security_flag: Default::default(),
        }
    }
}

impl CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write(w)?;
        Ok(())
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write_u16_le(w)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write_u16_be(w)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write_u32_le(w)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write_u32_be(w)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write_u64_le(w)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: LoginResult = self.into();
        a.write_u64_be(w)
    }

}

impl VariableSized for CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult {
    fn size(&self) -> usize {
        match self {
            Self::SUCCESS  {
                server_public_key,
                generator,
                large_safe_prime,
                salt,
                crc_salt,
                security_flag,
            } => {
                1
                + 32 * core::mem::size_of::<u8>() // server_public_key: u8[32]
                + 1 // generator_length: u8
                + generator.len() * core::mem::size_of::<u8>() // generator: u8[generator_length]
                + 1 // large_safe_prime_length: u8
                + large_safe_prime.len() * core::mem::size_of::<u8>() // large_safe_prime: u8[large_safe_prime_length]
                + 32 * core::mem::size_of::<u8>() // salt: u8[32]
                + 16 * core::mem::size_of::<u8>() // crc_salt: u8[16]
                + security_flag.size() // security_flag: CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag and subfields
            }
            Self::FAIL_UNKNOWN0 =>  {
                1
            }
            Self::FAIL_UNKNOWN1 =>  {
                1
            }
            Self::FAIL_BANNED =>  {
                1
            }
            Self::FAIL_UNKNOWN_ACCOUNT =>  {
                1
            }
            Self::FAIL_INCORRECT_PASSWORD =>  {
                1
            }
            Self::FAIL_ALREADY_ONLINE =>  {
                1
            }
            Self::FAIL_NO_TIME =>  {
                1
            }
            Self::FAIL_DB_BUSY =>  {
                1
            }
            Self::FAIL_VERSION_INVALID =>  {
                1
            }
            Self::LOGIN_DOWNLOAD_FILE =>  {
                1
            }
            Self::FAIL_INVALID_SERVER =>  {
                1
            }
            Self::FAIL_SUSPENDED =>  {
                1
            }
            Self::FAIL_NO_ACCESS =>  {
                1
            }
            Self::SUCCESS_SURVEY =>  {
                1
            }
            Self::FAIL_PARENTALCONTROL =>  {
                1
            }
            Self::FAIL_LOCKED_ENFORCED =>  {
                1
            }
        }
    }
}

impl MaximumPossibleSized for CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMD_AUTH_LOGON_CHALLENGE_Server;
    use crate::VariableSized;
    use crate::logon::version_8::LoginResult;
    use crate::logon::version_8::SecurityFlag;
    use super::CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag;
    use super::CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagPIN;
    use super::CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagUNKNOWN0;
    use super::CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagAUTHENTICATOR;
    use super::CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult;
    use crate::logon::version_8::opcodes::ServerOpcodeMessage;

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 236.
    #[test]
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
             0xD2, 0xF1, 0x00, ];

        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            login_result: CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS {
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag {
                    inner: 0 | SecurityFlag::NONE,
                    pin: None,
                    unknown0: None,
                    authenticator: None,
                },
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 289.
    #[test]
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
             0xD2, 0xF1, 0x01, 0xEF, 0xBE, 0xAD, 0xDE, 0x00, 0x01, 0x02, 0x03, 0x04,
             0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ];

        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            login_result: CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS {
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag {
                    inner: 0 | SecurityFlag::PIN,
                    pin: Some(CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagPIN {
                        pin_grid_seed: 0xDEADBEEF,
                        pin_salt: [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                             0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, ],
                    }),
                    unknown0: None,
                    authenticator: None,
                },
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 349.
    #[test]
    fn CMD_AUTH_LOGON_CHALLENGE_Server2() {
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
             0xD2, 0xF1, 0x04, 0x01, ];

        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            login_result: CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS {
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag {
                    inner: 0 | SecurityFlag::AUTHENTICATOR,
                    pin: None,
                    unknown0: None,
                    authenticator: Some(CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagAUTHENTICATOR {
                        unknown5: 0x1,
                    }),
                },
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 404.
    #[test]
    fn CMD_AUTH_LOGON_CHALLENGE_Server3() {
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
             0xD2, 0xF1, 0x02, 0xFF, 0xEE, 0xDD, 0xCC, 0xDE, 0xCA, 0xFA, 0xEF, 0xBE,
             0xAD, 0xDE, 0x00, ];

        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            login_result: CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS {
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag {
                    inner: 0 | SecurityFlag::UNKNOWN0,
                    pin: None,
                    unknown0: Some(CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagUNKNOWN0 {
                        unknown0: 0xFF,
                        unknown1: 0xEE,
                        unknown2: 0xDD,
                        unknown3: 0xCC,
                        unknown4: 0xDEADBEEFFACADE,
                    }),
                    authenticator: None,
                },
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 467.
    #[test]
    fn CMD_AUTH_LOGON_CHALLENGE_Server4() {
        let raw: Vec<u8> = vec![ 0x00, 0x00, 0x05, ];

        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            login_result: CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_INCORRECT_PASSWORD,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 477.
    #[test]
    fn CMD_AUTH_LOGON_CHALLENGE_Server5() {
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
             0xD2, 0xF1, 0x06, 0xFF, 0xEE, 0xDD, 0xCC, 0xDE, 0xCA, 0xFA, 0xEF, 0xBE,
             0xAD, 0xDE, 0x00, 0x01, ];

        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            login_result: CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::SUCCESS {
                server_public_key: [ 0x49, 0xD8, 0xC2, 0xBC, 0x68, 0x5C, 0x2B, 0xCE,
                     0x4A, 0xF4, 0xFA, 0x07, 0x0A, 0x47, 0x93, 0x78, 0x58, 0x78,
                     0x46, 0xB5, 0x83, 0xD4, 0x41, 0x82, 0x9E, 0x24, 0xD8, 0x87,
                     0xCE, 0xDA, 0x34, 0x46, ],
                generator: vec![ 0x07, ],
                large_safe_prime: vec![ 0xB7, 0x9B, 0x3E, 0x2A, 0x87, 0x82, 0x3C,
                     0xAB, 0x8F, 0x5E, 0xBF, 0xBF, 0x8E, 0xB1, 0x01, 0x08, 0x53,
                     0x50, 0x06, 0x29, 0x8B, 0x5B, 0xAD, 0xBD, 0x5B, 0x53, 0xE1,
                     0x89, 0x5E, 0x64, 0x4B, 0x89, ],
                salt: [ 0xC7, 0x09, 0x87, 0x7D, 0x8C, 0x65, 0x52, 0x66, 0xA5, 0x7D,
                     0xB8, 0x65, 0x3D, 0x6E, 0xA6, 0x2B, 0xB5, 0x54, 0xF2, 0x0B,
                     0xCF, 0x74, 0xD6, 0x4A, 0x77, 0xA7, 0xD3, 0x3D, 0xF3, 0x30,
                     0x90, 0x87, ],
                crc_salt: [ 0xBA, 0xA3, 0x1E, 0x99, 0xA0, 0x0B, 0x21, 0x57, 0xFC,
                     0x37, 0x3F, 0xB3, 0x69, 0xCD, 0xD2, 0xF1, ],
                security_flag: CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlag {
                    inner: 0 | SecurityFlag::UNKNOWN0 | SecurityFlag::AUTHENTICATOR,
                    pin: None,
                    unknown0: Some(CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagUNKNOWN0 {
                        unknown0: 0xFF,
                        unknown1: 0xEE,
                        unknown2: 0xDD,
                        unknown3: 0xCC,
                        unknown4: 0xDEADBEEFFACADE,
                    }),
                    authenticator: Some(CMD_AUTH_LOGON_CHALLENGE_ServerSecurityFlagAUTHENTICATOR {
                        unknown5: 0x1,
                    }),
                },
            },
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_server.wowm` line 544.
    #[test]
    fn CMD_AUTH_LOGON_CHALLENGE_Server6() {
        let raw: Vec<u8> = vec![ 0x00, 0x00, 0x05, ];

        let expected = CMD_AUTH_LOGON_CHALLENGE_Server {
            login_result: CMD_AUTH_LOGON_CHALLENGE_ServerLoginResult::FAIL_INCORRECT_PASSWORD,
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.login_result, expected.login_result);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
