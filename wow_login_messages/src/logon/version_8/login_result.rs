use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum LoginResult {
    SUCCESS,
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

impl LoginResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::SUCCESS => 0x0,
            Self::FAIL_UNKNOWN0 => 0x1,
            Self::FAIL_UNKNOWN1 => 0x2,
            Self::FAIL_BANNED => 0x3,
            Self::FAIL_UNKNOWN_ACCOUNT => 0x4,
            Self::FAIL_INCORRECT_PASSWORD => 0x5,
            Self::FAIL_ALREADY_ONLINE => 0x6,
            Self::FAIL_NO_TIME => 0x7,
            Self::FAIL_DB_BUSY => 0x8,
            Self::FAIL_VERSION_INVALID => 0x9,
            Self::LOGIN_DOWNLOAD_FILE => 0xa,
            Self::FAIL_INVALID_SERVER => 0xb,
            Self::FAIL_SUSPENDED => 0xc,
            Self::FAIL_NO_ACCESS => 0xd,
            Self::SUCCESS_SURVEY => 0xe,
            Self::FAIL_PARENTALCONTROL => 0xf,
            Self::FAIL_LOCKED_ENFORCED => 0x10,
        }
    }

}

impl Default for LoginResult {
    fn default() -> Self {
        Self::SUCCESS
    }
}

impl std::fmt::Display for LoginResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SUCCESS => f.write_str("SUCCESS"),
            Self::FAIL_UNKNOWN0 => f.write_str("FAIL_UNKNOWN0"),
            Self::FAIL_UNKNOWN1 => f.write_str("FAIL_UNKNOWN1"),
            Self::FAIL_BANNED => f.write_str("FAIL_BANNED"),
            Self::FAIL_UNKNOWN_ACCOUNT => f.write_str("FAIL_UNKNOWN_ACCOUNT"),
            Self::FAIL_INCORRECT_PASSWORD => f.write_str("FAIL_INCORRECT_PASSWORD"),
            Self::FAIL_ALREADY_ONLINE => f.write_str("FAIL_ALREADY_ONLINE"),
            Self::FAIL_NO_TIME => f.write_str("FAIL_NO_TIME"),
            Self::FAIL_DB_BUSY => f.write_str("FAIL_DB_BUSY"),
            Self::FAIL_VERSION_INVALID => f.write_str("FAIL_VERSION_INVALID"),
            Self::LOGIN_DOWNLOAD_FILE => f.write_str("LOGIN_DOWNLOAD_FILE"),
            Self::FAIL_INVALID_SERVER => f.write_str("FAIL_INVALID_SERVER"),
            Self::FAIL_SUSPENDED => f.write_str("FAIL_SUSPENDED"),
            Self::FAIL_NO_ACCESS => f.write_str("FAIL_NO_ACCESS"),
            Self::SUCCESS_SURVEY => f.write_str("SUCCESS_SURVEY"),
            Self::FAIL_PARENTALCONTROL => f.write_str("FAIL_PARENTALCONTROL"),
            Self::FAIL_LOCKED_ENFORCED => f.write_str("FAIL_LOCKED_ENFORCED"),
        }
    }
}

impl TryFrom<u8> for LoginResult {
    type Error = LoginResultError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SUCCESS),
            1 => Ok(Self::FAIL_UNKNOWN0),
            2 => Ok(Self::FAIL_UNKNOWN1),
            3 => Ok(Self::FAIL_BANNED),
            4 => Ok(Self::FAIL_UNKNOWN_ACCOUNT),
            5 => Ok(Self::FAIL_INCORRECT_PASSWORD),
            6 => Ok(Self::FAIL_ALREADY_ONLINE),
            7 => Ok(Self::FAIL_NO_TIME),
            8 => Ok(Self::FAIL_DB_BUSY),
            9 => Ok(Self::FAIL_VERSION_INVALID),
            10 => Ok(Self::LOGIN_DOWNLOAD_FILE),
            11 => Ok(Self::FAIL_INVALID_SERVER),
            12 => Ok(Self::FAIL_SUSPENDED),
            13 => Ok(Self::FAIL_NO_ACCESS),
            14 => Ok(Self::SUCCESS_SURVEY),
            15 => Ok(Self::FAIL_PARENTALCONTROL),
            16 => Ok(Self::FAIL_LOCKED_ENFORCED),
            _ => Err(LoginResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct LoginResultError {
    pub value: u8,
}

impl LoginResultError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for LoginResultError {}
impl std::fmt::Display for LoginResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'LoginResult': '{}'", self.value))
    }
}

