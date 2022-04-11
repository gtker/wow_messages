use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum WorldResult {
    RESPONSE_SUCCESS,
    RESPONSE_FAILURE,
    RESPONSE_CANCELLED,
    RESPONSE_DISCONNECTED,
    RESPONSE_FAILED_TO_CONNECT,
    RESPONSE_CONNECTED,
    RESPONSE_VERSION_MISMATCH,
    CSTATUS_CONNECTING,
    CSTATUS_NEGOTIATING_SECURITY,
    CSTATUS_NEGOTIATION_COMPLETE,
    CSTATUS_NEGOTIATION_FAILED,
    CSTATUS_AUTHENTICATING,
    AUTH_OK,
    AUTH_FAILED,
    AUTH_REJECT,
    AUTH_BAD_SERVER_PROOF,
    AUTH_UNAVAILABLE,
    AUTH_SYSTEM_ERROR,
    AUTH_BILLING_ERROR,
    AUTH_BILLING_EXPIRED,
    AUTH_VERSION_MISMATCH,
    AUTH_UNKNOWN_ACCOUNT,
    AUTH_INCORRECT_PASSWORD,
    AUTH_SESSION_EXPIRED,
    AUTH_SERVER_SHUTTING_DOWN,
    AUTH_ALREADY_LOGGING_IN,
    AUTH_LOGIN_SERVER_NOT_FOUND,
    AUTH_WAIT_QUEUE,
    AUTH_BANNED,
    AUTH_ALREADY_ONLINE,
    AUTH_NO_TIME,
    AUTH_DB_BUSY,
    AUTH_SUSPENDED,
    AUTH_PARENTAL_CONTROL,
    REALM_LIST_IN_PROGRESS,
    REALM_LIST_SUCCESS,
    REALM_LIST_FAILED,
    REALM_LIST_INVALID,
    REALM_LIST_REALM_NOT_FOUND,
    ACCOUNT_CREATE_IN_PROGRESS,
    ACCOUNT_CREATE_SUCCESS,
    ACCOUNT_CREATE_FAILED,
    CHAR_LIST_RETRIEVING,
    CHAR_LIST_RETRIEVED,
    CHAR_LIST_FAILED,
    CHAR_CREATE_IN_PROGRESS,
    CHAR_CREATE_SUCCESS,
    CHAR_CREATE_ERROR,
    CHAR_CREATE_FAILED,
    CHAR_CREATE_NAME_IN_USE,
    CHAR_CREATE_DISABLED,
    CHAR_CREATE_PVP_TEAMS_VIOLATION,
    CHAR_CREATE_SERVER_LIMIT,
    CHAR_CREATE_ACCOUNT_LIMIT,
    CHAR_CREATE_SERVER_QUEUE,
    CHAR_CREATE_ONLY_EXISTING,
    CHAR_DELETE_IN_PROGRESS,
    CHAR_DELETE_SUCCESS,
    CHAR_DELETE_FAILED,
    CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER,
    CHAR_LOGIN_IN_PROGRESS,
    CHAR_LOGIN_SUCCESS,
    CHAR_LOGIN_NO_WORLD,
    CHAR_LOGIN_DUPLICATE_CHARACTER,
    CHAR_LOGIN_NO_INSTANCES,
    CHAR_LOGIN_FAILED,
    CHAR_LOGIN_DISABLED,
    CHAR_LOGIN_NO_CHARACTER,
    CHAR_LOGIN_LOCKED_FOR_TRANSFER,
    CHAR_NAME_NO_NAME,
    CHAR_NAME_TOO_SHORT,
    CHAR_NAME_TOO_LONG,
    CHAR_NAME_ONLY_LETTERS,
    CHAR_NAME_MIXED_LANGUAGES,
    CHAR_NAME_PROFANE,
    CHAR_NAME_RESERVED,
    CHAR_NAME_INVALID_APOSTROPHE,
    CHAR_NAME_MULTIPLE_APOSTROPHES,
    CHAR_NAME_THREE_CONSECUTIVE,
    CHAR_NAME_INVALID_SPACE,
    CHAR_NAME_SUCCESS,
    CHAR_NAME_FAILURE,
}

impl ReadableAndWritable for WorldResult {
    type Error = WorldResultError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl WorldResult {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, WorldResultError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, WorldResultError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, WorldResultError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::RESPONSE_SUCCESS => 0x0,
            Self::RESPONSE_FAILURE => 0x1,
            Self::RESPONSE_CANCELLED => 0x2,
            Self::RESPONSE_DISCONNECTED => 0x3,
            Self::RESPONSE_FAILED_TO_CONNECT => 0x4,
            Self::RESPONSE_CONNECTED => 0x5,
            Self::RESPONSE_VERSION_MISMATCH => 0x6,
            Self::CSTATUS_CONNECTING => 0x7,
            Self::CSTATUS_NEGOTIATING_SECURITY => 0x8,
            Self::CSTATUS_NEGOTIATION_COMPLETE => 0x9,
            Self::CSTATUS_NEGOTIATION_FAILED => 0xa,
            Self::CSTATUS_AUTHENTICATING => 0xb,
            Self::AUTH_OK => 0xc,
            Self::AUTH_FAILED => 0xd,
            Self::AUTH_REJECT => 0xe,
            Self::AUTH_BAD_SERVER_PROOF => 0xf,
            Self::AUTH_UNAVAILABLE => 0x10,
            Self::AUTH_SYSTEM_ERROR => 0x11,
            Self::AUTH_BILLING_ERROR => 0x12,
            Self::AUTH_BILLING_EXPIRED => 0x13,
            Self::AUTH_VERSION_MISMATCH => 0x14,
            Self::AUTH_UNKNOWN_ACCOUNT => 0x15,
            Self::AUTH_INCORRECT_PASSWORD => 0x16,
            Self::AUTH_SESSION_EXPIRED => 0x17,
            Self::AUTH_SERVER_SHUTTING_DOWN => 0x18,
            Self::AUTH_ALREADY_LOGGING_IN => 0x19,
            Self::AUTH_LOGIN_SERVER_NOT_FOUND => 0x1a,
            Self::AUTH_WAIT_QUEUE => 0x1b,
            Self::AUTH_BANNED => 0x1c,
            Self::AUTH_ALREADY_ONLINE => 0x1d,
            Self::AUTH_NO_TIME => 0x1e,
            Self::AUTH_DB_BUSY => 0x1f,
            Self::AUTH_SUSPENDED => 0x20,
            Self::AUTH_PARENTAL_CONTROL => 0x21,
            Self::REALM_LIST_IN_PROGRESS => 0x22,
            Self::REALM_LIST_SUCCESS => 0x23,
            Self::REALM_LIST_FAILED => 0x24,
            Self::REALM_LIST_INVALID => 0x25,
            Self::REALM_LIST_REALM_NOT_FOUND => 0x26,
            Self::ACCOUNT_CREATE_IN_PROGRESS => 0x27,
            Self::ACCOUNT_CREATE_SUCCESS => 0x28,
            Self::ACCOUNT_CREATE_FAILED => 0x29,
            Self::CHAR_LIST_RETRIEVING => 0x2a,
            Self::CHAR_LIST_RETRIEVED => 0x2b,
            Self::CHAR_LIST_FAILED => 0x2c,
            Self::CHAR_CREATE_IN_PROGRESS => 0x2d,
            Self::CHAR_CREATE_SUCCESS => 0x2e,
            Self::CHAR_CREATE_ERROR => 0x2f,
            Self::CHAR_CREATE_FAILED => 0x30,
            Self::CHAR_CREATE_NAME_IN_USE => 0x31,
            Self::CHAR_CREATE_DISABLED => 0x32,
            Self::CHAR_CREATE_PVP_TEAMS_VIOLATION => 0x33,
            Self::CHAR_CREATE_SERVER_LIMIT => 0x34,
            Self::CHAR_CREATE_ACCOUNT_LIMIT => 0x35,
            Self::CHAR_CREATE_SERVER_QUEUE => 0x36,
            Self::CHAR_CREATE_ONLY_EXISTING => 0x37,
            Self::CHAR_DELETE_IN_PROGRESS => 0x38,
            Self::CHAR_DELETE_SUCCESS => 0x39,
            Self::CHAR_DELETE_FAILED => 0x3a,
            Self::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER => 0x3b,
            Self::CHAR_LOGIN_IN_PROGRESS => 0x3c,
            Self::CHAR_LOGIN_SUCCESS => 0x3d,
            Self::CHAR_LOGIN_NO_WORLD => 0x3e,
            Self::CHAR_LOGIN_DUPLICATE_CHARACTER => 0x3f,
            Self::CHAR_LOGIN_NO_INSTANCES => 0x40,
            Self::CHAR_LOGIN_FAILED => 0x41,
            Self::CHAR_LOGIN_DISABLED => 0x42,
            Self::CHAR_LOGIN_NO_CHARACTER => 0x43,
            Self::CHAR_LOGIN_LOCKED_FOR_TRANSFER => 0x44,
            Self::CHAR_NAME_NO_NAME => 0x45,
            Self::CHAR_NAME_TOO_SHORT => 0x46,
            Self::CHAR_NAME_TOO_LONG => 0x47,
            Self::CHAR_NAME_ONLY_LETTERS => 0x48,
            Self::CHAR_NAME_MIXED_LANGUAGES => 0x49,
            Self::CHAR_NAME_PROFANE => 0x4a,
            Self::CHAR_NAME_RESERVED => 0x4b,
            Self::CHAR_NAME_INVALID_APOSTROPHE => 0x4c,
            Self::CHAR_NAME_MULTIPLE_APOSTROPHES => 0x4d,
            Self::CHAR_NAME_THREE_CONSECUTIVE => 0x4e,
            Self::CHAR_NAME_INVALID_SPACE => 0x4f,
            Self::CHAR_NAME_SUCCESS => 0x50,
            Self::CHAR_NAME_FAILURE => 0x51,
        }
    }

    pub const fn new() -> Self {
        Self::RESPONSE_SUCCESS
    }

}

impl ConstantSized for WorldResult {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for WorldResult {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for WorldResult {
    fn default() -> Self {
        Self::RESPONSE_SUCCESS
    }
}

impl std::fmt::Display for WorldResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RESPONSE_SUCCESS => f.write_str("RESPONSE_SUCCESS"),
            Self::RESPONSE_FAILURE => f.write_str("RESPONSE_FAILURE"),
            Self::RESPONSE_CANCELLED => f.write_str("RESPONSE_CANCELLED"),
            Self::RESPONSE_DISCONNECTED => f.write_str("RESPONSE_DISCONNECTED"),
            Self::RESPONSE_FAILED_TO_CONNECT => f.write_str("RESPONSE_FAILED_TO_CONNECT"),
            Self::RESPONSE_CONNECTED => f.write_str("RESPONSE_CONNECTED"),
            Self::RESPONSE_VERSION_MISMATCH => f.write_str("RESPONSE_VERSION_MISMATCH"),
            Self::CSTATUS_CONNECTING => f.write_str("CSTATUS_CONNECTING"),
            Self::CSTATUS_NEGOTIATING_SECURITY => f.write_str("CSTATUS_NEGOTIATING_SECURITY"),
            Self::CSTATUS_NEGOTIATION_COMPLETE => f.write_str("CSTATUS_NEGOTIATION_COMPLETE"),
            Self::CSTATUS_NEGOTIATION_FAILED => f.write_str("CSTATUS_NEGOTIATION_FAILED"),
            Self::CSTATUS_AUTHENTICATING => f.write_str("CSTATUS_AUTHENTICATING"),
            Self::AUTH_OK => f.write_str("AUTH_OK"),
            Self::AUTH_FAILED => f.write_str("AUTH_FAILED"),
            Self::AUTH_REJECT => f.write_str("AUTH_REJECT"),
            Self::AUTH_BAD_SERVER_PROOF => f.write_str("AUTH_BAD_SERVER_PROOF"),
            Self::AUTH_UNAVAILABLE => f.write_str("AUTH_UNAVAILABLE"),
            Self::AUTH_SYSTEM_ERROR => f.write_str("AUTH_SYSTEM_ERROR"),
            Self::AUTH_BILLING_ERROR => f.write_str("AUTH_BILLING_ERROR"),
            Self::AUTH_BILLING_EXPIRED => f.write_str("AUTH_BILLING_EXPIRED"),
            Self::AUTH_VERSION_MISMATCH => f.write_str("AUTH_VERSION_MISMATCH"),
            Self::AUTH_UNKNOWN_ACCOUNT => f.write_str("AUTH_UNKNOWN_ACCOUNT"),
            Self::AUTH_INCORRECT_PASSWORD => f.write_str("AUTH_INCORRECT_PASSWORD"),
            Self::AUTH_SESSION_EXPIRED => f.write_str("AUTH_SESSION_EXPIRED"),
            Self::AUTH_SERVER_SHUTTING_DOWN => f.write_str("AUTH_SERVER_SHUTTING_DOWN"),
            Self::AUTH_ALREADY_LOGGING_IN => f.write_str("AUTH_ALREADY_LOGGING_IN"),
            Self::AUTH_LOGIN_SERVER_NOT_FOUND => f.write_str("AUTH_LOGIN_SERVER_NOT_FOUND"),
            Self::AUTH_WAIT_QUEUE => f.write_str("AUTH_WAIT_QUEUE"),
            Self::AUTH_BANNED => f.write_str("AUTH_BANNED"),
            Self::AUTH_ALREADY_ONLINE => f.write_str("AUTH_ALREADY_ONLINE"),
            Self::AUTH_NO_TIME => f.write_str("AUTH_NO_TIME"),
            Self::AUTH_DB_BUSY => f.write_str("AUTH_DB_BUSY"),
            Self::AUTH_SUSPENDED => f.write_str("AUTH_SUSPENDED"),
            Self::AUTH_PARENTAL_CONTROL => f.write_str("AUTH_PARENTAL_CONTROL"),
            Self::REALM_LIST_IN_PROGRESS => f.write_str("REALM_LIST_IN_PROGRESS"),
            Self::REALM_LIST_SUCCESS => f.write_str("REALM_LIST_SUCCESS"),
            Self::REALM_LIST_FAILED => f.write_str("REALM_LIST_FAILED"),
            Self::REALM_LIST_INVALID => f.write_str("REALM_LIST_INVALID"),
            Self::REALM_LIST_REALM_NOT_FOUND => f.write_str("REALM_LIST_REALM_NOT_FOUND"),
            Self::ACCOUNT_CREATE_IN_PROGRESS => f.write_str("ACCOUNT_CREATE_IN_PROGRESS"),
            Self::ACCOUNT_CREATE_SUCCESS => f.write_str("ACCOUNT_CREATE_SUCCESS"),
            Self::ACCOUNT_CREATE_FAILED => f.write_str("ACCOUNT_CREATE_FAILED"),
            Self::CHAR_LIST_RETRIEVING => f.write_str("CHAR_LIST_RETRIEVING"),
            Self::CHAR_LIST_RETRIEVED => f.write_str("CHAR_LIST_RETRIEVED"),
            Self::CHAR_LIST_FAILED => f.write_str("CHAR_LIST_FAILED"),
            Self::CHAR_CREATE_IN_PROGRESS => f.write_str("CHAR_CREATE_IN_PROGRESS"),
            Self::CHAR_CREATE_SUCCESS => f.write_str("CHAR_CREATE_SUCCESS"),
            Self::CHAR_CREATE_ERROR => f.write_str("CHAR_CREATE_ERROR"),
            Self::CHAR_CREATE_FAILED => f.write_str("CHAR_CREATE_FAILED"),
            Self::CHAR_CREATE_NAME_IN_USE => f.write_str("CHAR_CREATE_NAME_IN_USE"),
            Self::CHAR_CREATE_DISABLED => f.write_str("CHAR_CREATE_DISABLED"),
            Self::CHAR_CREATE_PVP_TEAMS_VIOLATION => f.write_str("CHAR_CREATE_PVP_TEAMS_VIOLATION"),
            Self::CHAR_CREATE_SERVER_LIMIT => f.write_str("CHAR_CREATE_SERVER_LIMIT"),
            Self::CHAR_CREATE_ACCOUNT_LIMIT => f.write_str("CHAR_CREATE_ACCOUNT_LIMIT"),
            Self::CHAR_CREATE_SERVER_QUEUE => f.write_str("CHAR_CREATE_SERVER_QUEUE"),
            Self::CHAR_CREATE_ONLY_EXISTING => f.write_str("CHAR_CREATE_ONLY_EXISTING"),
            Self::CHAR_DELETE_IN_PROGRESS => f.write_str("CHAR_DELETE_IN_PROGRESS"),
            Self::CHAR_DELETE_SUCCESS => f.write_str("CHAR_DELETE_SUCCESS"),
            Self::CHAR_DELETE_FAILED => f.write_str("CHAR_DELETE_FAILED"),
            Self::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER => f.write_str("CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER"),
            Self::CHAR_LOGIN_IN_PROGRESS => f.write_str("CHAR_LOGIN_IN_PROGRESS"),
            Self::CHAR_LOGIN_SUCCESS => f.write_str("CHAR_LOGIN_SUCCESS"),
            Self::CHAR_LOGIN_NO_WORLD => f.write_str("CHAR_LOGIN_NO_WORLD"),
            Self::CHAR_LOGIN_DUPLICATE_CHARACTER => f.write_str("CHAR_LOGIN_DUPLICATE_CHARACTER"),
            Self::CHAR_LOGIN_NO_INSTANCES => f.write_str("CHAR_LOGIN_NO_INSTANCES"),
            Self::CHAR_LOGIN_FAILED => f.write_str("CHAR_LOGIN_FAILED"),
            Self::CHAR_LOGIN_DISABLED => f.write_str("CHAR_LOGIN_DISABLED"),
            Self::CHAR_LOGIN_NO_CHARACTER => f.write_str("CHAR_LOGIN_NO_CHARACTER"),
            Self::CHAR_LOGIN_LOCKED_FOR_TRANSFER => f.write_str("CHAR_LOGIN_LOCKED_FOR_TRANSFER"),
            Self::CHAR_NAME_NO_NAME => f.write_str("CHAR_NAME_NO_NAME"),
            Self::CHAR_NAME_TOO_SHORT => f.write_str("CHAR_NAME_TOO_SHORT"),
            Self::CHAR_NAME_TOO_LONG => f.write_str("CHAR_NAME_TOO_LONG"),
            Self::CHAR_NAME_ONLY_LETTERS => f.write_str("CHAR_NAME_ONLY_LETTERS"),
            Self::CHAR_NAME_MIXED_LANGUAGES => f.write_str("CHAR_NAME_MIXED_LANGUAGES"),
            Self::CHAR_NAME_PROFANE => f.write_str("CHAR_NAME_PROFANE"),
            Self::CHAR_NAME_RESERVED => f.write_str("CHAR_NAME_RESERVED"),
            Self::CHAR_NAME_INVALID_APOSTROPHE => f.write_str("CHAR_NAME_INVALID_APOSTROPHE"),
            Self::CHAR_NAME_MULTIPLE_APOSTROPHES => f.write_str("CHAR_NAME_MULTIPLE_APOSTROPHES"),
            Self::CHAR_NAME_THREE_CONSECUTIVE => f.write_str("CHAR_NAME_THREE_CONSECUTIVE"),
            Self::CHAR_NAME_INVALID_SPACE => f.write_str("CHAR_NAME_INVALID_SPACE"),
            Self::CHAR_NAME_SUCCESS => f.write_str("CHAR_NAME_SUCCESS"),
            Self::CHAR_NAME_FAILURE => f.write_str("CHAR_NAME_FAILURE"),
        }
    }
}

impl TryFrom<u32> for WorldResult {
    type Error = TryFromWorldResultError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::RESPONSE_SUCCESS),
            1 => Ok(Self::RESPONSE_FAILURE),
            2 => Ok(Self::RESPONSE_CANCELLED),
            3 => Ok(Self::RESPONSE_DISCONNECTED),
            4 => Ok(Self::RESPONSE_FAILED_TO_CONNECT),
            5 => Ok(Self::RESPONSE_CONNECTED),
            6 => Ok(Self::RESPONSE_VERSION_MISMATCH),
            7 => Ok(Self::CSTATUS_CONNECTING),
            8 => Ok(Self::CSTATUS_NEGOTIATING_SECURITY),
            9 => Ok(Self::CSTATUS_NEGOTIATION_COMPLETE),
            10 => Ok(Self::CSTATUS_NEGOTIATION_FAILED),
            11 => Ok(Self::CSTATUS_AUTHENTICATING),
            12 => Ok(Self::AUTH_OK),
            13 => Ok(Self::AUTH_FAILED),
            14 => Ok(Self::AUTH_REJECT),
            15 => Ok(Self::AUTH_BAD_SERVER_PROOF),
            16 => Ok(Self::AUTH_UNAVAILABLE),
            17 => Ok(Self::AUTH_SYSTEM_ERROR),
            18 => Ok(Self::AUTH_BILLING_ERROR),
            19 => Ok(Self::AUTH_BILLING_EXPIRED),
            20 => Ok(Self::AUTH_VERSION_MISMATCH),
            21 => Ok(Self::AUTH_UNKNOWN_ACCOUNT),
            22 => Ok(Self::AUTH_INCORRECT_PASSWORD),
            23 => Ok(Self::AUTH_SESSION_EXPIRED),
            24 => Ok(Self::AUTH_SERVER_SHUTTING_DOWN),
            25 => Ok(Self::AUTH_ALREADY_LOGGING_IN),
            26 => Ok(Self::AUTH_LOGIN_SERVER_NOT_FOUND),
            27 => Ok(Self::AUTH_WAIT_QUEUE),
            28 => Ok(Self::AUTH_BANNED),
            29 => Ok(Self::AUTH_ALREADY_ONLINE),
            30 => Ok(Self::AUTH_NO_TIME),
            31 => Ok(Self::AUTH_DB_BUSY),
            32 => Ok(Self::AUTH_SUSPENDED),
            33 => Ok(Self::AUTH_PARENTAL_CONTROL),
            34 => Ok(Self::REALM_LIST_IN_PROGRESS),
            35 => Ok(Self::REALM_LIST_SUCCESS),
            36 => Ok(Self::REALM_LIST_FAILED),
            37 => Ok(Self::REALM_LIST_INVALID),
            38 => Ok(Self::REALM_LIST_REALM_NOT_FOUND),
            39 => Ok(Self::ACCOUNT_CREATE_IN_PROGRESS),
            40 => Ok(Self::ACCOUNT_CREATE_SUCCESS),
            41 => Ok(Self::ACCOUNT_CREATE_FAILED),
            42 => Ok(Self::CHAR_LIST_RETRIEVING),
            43 => Ok(Self::CHAR_LIST_RETRIEVED),
            44 => Ok(Self::CHAR_LIST_FAILED),
            45 => Ok(Self::CHAR_CREATE_IN_PROGRESS),
            46 => Ok(Self::CHAR_CREATE_SUCCESS),
            47 => Ok(Self::CHAR_CREATE_ERROR),
            48 => Ok(Self::CHAR_CREATE_FAILED),
            49 => Ok(Self::CHAR_CREATE_NAME_IN_USE),
            50 => Ok(Self::CHAR_CREATE_DISABLED),
            51 => Ok(Self::CHAR_CREATE_PVP_TEAMS_VIOLATION),
            52 => Ok(Self::CHAR_CREATE_SERVER_LIMIT),
            53 => Ok(Self::CHAR_CREATE_ACCOUNT_LIMIT),
            54 => Ok(Self::CHAR_CREATE_SERVER_QUEUE),
            55 => Ok(Self::CHAR_CREATE_ONLY_EXISTING),
            56 => Ok(Self::CHAR_DELETE_IN_PROGRESS),
            57 => Ok(Self::CHAR_DELETE_SUCCESS),
            58 => Ok(Self::CHAR_DELETE_FAILED),
            59 => Ok(Self::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER),
            60 => Ok(Self::CHAR_LOGIN_IN_PROGRESS),
            61 => Ok(Self::CHAR_LOGIN_SUCCESS),
            62 => Ok(Self::CHAR_LOGIN_NO_WORLD),
            63 => Ok(Self::CHAR_LOGIN_DUPLICATE_CHARACTER),
            64 => Ok(Self::CHAR_LOGIN_NO_INSTANCES),
            65 => Ok(Self::CHAR_LOGIN_FAILED),
            66 => Ok(Self::CHAR_LOGIN_DISABLED),
            67 => Ok(Self::CHAR_LOGIN_NO_CHARACTER),
            68 => Ok(Self::CHAR_LOGIN_LOCKED_FOR_TRANSFER),
            69 => Ok(Self::CHAR_NAME_NO_NAME),
            70 => Ok(Self::CHAR_NAME_TOO_SHORT),
            71 => Ok(Self::CHAR_NAME_TOO_LONG),
            72 => Ok(Self::CHAR_NAME_ONLY_LETTERS),
            73 => Ok(Self::CHAR_NAME_MIXED_LANGUAGES),
            74 => Ok(Self::CHAR_NAME_PROFANE),
            75 => Ok(Self::CHAR_NAME_RESERVED),
            76 => Ok(Self::CHAR_NAME_INVALID_APOSTROPHE),
            77 => Ok(Self::CHAR_NAME_MULTIPLE_APOSTROPHES),
            78 => Ok(Self::CHAR_NAME_THREE_CONSECUTIVE),
            79 => Ok(Self::CHAR_NAME_INVALID_SPACE),
            80 => Ok(Self::CHAR_NAME_SUCCESS),
            81 => Ok(Self::CHAR_NAME_FAILURE),
            _ => Err(TryFromWorldResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromWorldResultError {
    value: u32,
}

impl TryFromWorldResultError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum WorldResultError {
    Read(std::io::Error),
    TryFrom(TryFromWorldResultError),
}

impl std::error::Error for WorldResultError {}
impl std::fmt::Display for TryFromWorldResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'WorldResult': '{}'", self.value))
    }
}

impl std::fmt::Display for WorldResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for WorldResultError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromWorldResultError> for WorldResultError {
    fn from(value: TryFromWorldResultError) -> Self {
        Self::TryFrom(value)
    }
}

