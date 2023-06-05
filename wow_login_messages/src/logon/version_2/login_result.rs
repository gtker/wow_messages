/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/common.wowm:20`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/common.wowm#L20):
/// ```text
/// enum LoginResult : u8 {
///     SUCCESS = 0x00;
///     FAIL_UNKNOWN0 = 0x01;
///     FAIL_UNKNOWN1 = 0x02;
///     FAIL_BANNED = 0x03;
///     FAIL_UNKNOWN_ACCOUNT = 0x04;
///     FAIL_INCORRECT_PASSWORD = 0x05;
///     FAIL_ALREADY_ONLINE = 0x06;
///     FAIL_NO_TIME = 0x07;
///     FAIL_DB_BUSY = 0x08;
///     FAIL_VERSION_INVALID = 0x09;
///     LOGIN_DOWNLOAD_FILE = 0x0A;
///     FAIL_INVALID_SERVER = 0x0B;
///     FAIL_SUSPENDED = 0x0C;
///     FAIL_NO_ACCESS = 0x0D;
///     SUCCESS_SURVEY = 0x0E;
///     FAIL_PARENTALCONTROL = 0x0F;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum LoginResult {
    Success,
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

impl LoginResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Success => 0x0,
            Self::FailUnknown0 => 0x1,
            Self::FailUnknown1 => 0x2,
            Self::FailBanned => 0x3,
            Self::FailUnknownAccount => 0x4,
            Self::FailIncorrectPassword => 0x5,
            Self::FailAlreadyOnline => 0x6,
            Self::FailNoTime => 0x7,
            Self::FailDbBusy => 0x8,
            Self::FailVersionInvalid => 0x9,
            Self::LoginDownloadFile => 0xa,
            Self::FailInvalidServer => 0xb,
            Self::FailSuspended => 0xc,
            Self::FailNoAccess => 0xd,
            Self::SuccessSurvey => 0xe,
            Self::FailParentalcontrol => 0xf,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl LoginResult {
    pub fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Success => "SUCCESS",
            Self::FailUnknown0 => "FAIL_UNKNOWN0",
            Self::FailUnknown1 => "FAIL_UNKNOWN1",
            Self::FailBanned => "FAIL_BANNED",
            Self::FailUnknownAccount => "FAIL_UNKNOWN_ACCOUNT",
            Self::FailIncorrectPassword => "FAIL_INCORRECT_PASSWORD",
            Self::FailAlreadyOnline => "FAIL_ALREADY_ONLINE",
            Self::FailNoTime => "FAIL_NO_TIME",
            Self::FailDbBusy => "FAIL_DB_BUSY",
            Self::FailVersionInvalid => "FAIL_VERSION_INVALID",
            Self::LoginDownloadFile => "LOGIN_DOWNLOAD_FILE",
            Self::FailInvalidServer => "FAIL_INVALID_SERVER",
            Self::FailSuspended => "FAIL_SUSPENDED",
            Self::FailNoAccess => "FAIL_NO_ACCESS",
            Self::SuccessSurvey => "SUCCESS_SURVEY",
            Self::FailParentalcontrol => "FAIL_PARENTALCONTROL",
        }
    }

}

impl Default for LoginResult {
    fn default() -> Self {
        Self::Success
    }
}

impl std::fmt::Display for LoginResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => f.write_str("Success"),
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

impl TryFrom<u8> for LoginResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Success),
            1 => Ok(Self::FailUnknown0),
            2 => Ok(Self::FailUnknown1),
            3 => Ok(Self::FailBanned),
            4 => Ok(Self::FailUnknownAccount),
            5 => Ok(Self::FailIncorrectPassword),
            6 => Ok(Self::FailAlreadyOnline),
            7 => Ok(Self::FailNoTime),
            8 => Ok(Self::FailDbBusy),
            9 => Ok(Self::FailVersionInvalid),
            10 => Ok(Self::LoginDownloadFile),
            11 => Ok(Self::FailInvalidServer),
            12 => Ok(Self::FailSuspended),
            13 => Ok(Self::FailNoAccess),
            14 => Ok(Self::SuccessSurvey),
            15 => Ok(Self::FailParentalcontrol),
            v => Err(crate::errors::EnumError::new("LoginResult", v as u64),)
        }
    }
}

