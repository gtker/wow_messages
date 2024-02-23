use crate::collective::CollectiveMessage;
use crate::errors::CollectiveError;

type Main = crate::version_8::CMD_AUTH_RECONNECT_CHALLENGE_Server;
type MainLoginResult = crate::version_8::CMD_AUTH_RECONNECT_CHALLENGE_Server_LoginResult;

type V2Main = crate::version_2::CMD_AUTH_RECONNECT_CHALLENGE_Server;
type V2MainLoginResult = crate::version_2::CMD_AUTH_RECONNECT_CHALLENGE_Server_LoginResult;

impl CollectiveMessage for Main {
    type Version2 = V2Main;
    type Version3 = Self::Version2;
    type Version5 = Self::Version2;
    type Version6 = Self::Version2;
    type Version7 = Self::Version2;
    type Version8 = Self;

    fn from_version_2(v: Self::Version2) -> Self {
        Self {
            result: MainLoginResult::from_version_2(v.result),
        }
    }

    fn to_version_2(&self) -> Result<Self::Version2, CollectiveError> {
        Ok(V2Main {
            result: self.result.to_version_2()?,
        })
    }

    fn from_version_3(v: Self::Version3) -> Self {
        Self::from_version_2(v)
    }

    fn to_version_3(&self) -> Result<Self::Version3, CollectiveError> {
        self.to_version_2()
    }

    fn from_version_5(v: Self::Version5) -> Self {
        Self::from_version_2(v)
    }

    fn to_version_5(&self) -> Result<Self::Version5, CollectiveError> {
        self.to_version_2()
    }

    fn from_version_6(v: Self::Version6) -> Self {
        Self::from_version_2(v)
    }

    fn to_version_6(&self) -> Result<Self::Version6, CollectiveError> {
        self.to_version_2()
    }

    fn from_version_7(v: Self::Version7) -> Self {
        Self::from_version_2(v)
    }

    fn to_version_7(&self) -> Result<Self::Version7, CollectiveError> {
        self.to_version_2()
    }
}

impl MainLoginResult {
    const fn from_version_2(v: V2MainLoginResult) -> Self {
        match v {
            V2MainLoginResult::Success {
                challenge_data,
                checksum_salt,
            } => Self::Success {
                challenge_data,
                checksum_salt,
            },
            V2MainLoginResult::FailUnknown0 => Self::FailUnknown0,
            V2MainLoginResult::FailUnknown1 => Self::FailUnknown1,
            V2MainLoginResult::FailBanned => Self::FailBanned,
            V2MainLoginResult::FailUnknownAccount => Self::FailUnknownAccount,
            V2MainLoginResult::FailIncorrectPassword => Self::FailIncorrectPassword,
            V2MainLoginResult::FailAlreadyOnline => Self::FailAlreadyOnline,
            V2MainLoginResult::FailNoTime => Self::FailNoTime,
            V2MainLoginResult::FailDbBusy => Self::FailDbBusy,
            V2MainLoginResult::FailVersionInvalid => Self::FailVersionInvalid,
            V2MainLoginResult::LoginDownloadFile => Self::LoginDownloadFile,
            V2MainLoginResult::FailInvalidServer => Self::FailInvalidServer,
            V2MainLoginResult::FailSuspended => Self::FailSuspended,
            V2MainLoginResult::FailNoAccess => Self::FailNoAccess,
            V2MainLoginResult::SuccessSurvey => Self::SuccessSurvey,
            V2MainLoginResult::FailParentalcontrol => Self::FailParentalcontrol,
        }
    }

    #[allow(clippy::wrong_self_convention)]
    const fn to_version_2(&self) -> Result<V2MainLoginResult, CollectiveError> {
        Ok(match self {
            MainLoginResult::Success {
                challenge_data,
                checksum_salt,
            } => V2MainLoginResult::Success {
                challenge_data: *challenge_data,
                checksum_salt: *checksum_salt,
            },
            MainLoginResult::FailUnknown0 => V2MainLoginResult::FailUnknown0,
            MainLoginResult::FailUnknown1 => V2MainLoginResult::FailUnknown1,
            MainLoginResult::FailBanned => V2MainLoginResult::FailBanned,
            MainLoginResult::FailUnknownAccount => V2MainLoginResult::FailUnknownAccount,
            MainLoginResult::FailIncorrectPassword => V2MainLoginResult::FailIncorrectPassword,
            MainLoginResult::FailAlreadyOnline => V2MainLoginResult::FailAlreadyOnline,
            MainLoginResult::FailNoTime => V2MainLoginResult::FailNoTime,
            MainLoginResult::FailDbBusy => V2MainLoginResult::FailDbBusy,
            MainLoginResult::FailVersionInvalid => V2MainLoginResult::FailVersionInvalid,
            MainLoginResult::LoginDownloadFile => V2MainLoginResult::LoginDownloadFile,
            MainLoginResult::FailInvalidServer => V2MainLoginResult::FailInvalidServer,
            MainLoginResult::FailSuspended => V2MainLoginResult::FailSuspended,
            MainLoginResult::FailNoAccess => V2MainLoginResult::FailNoAccess,
            MainLoginResult::SuccessSurvey => V2MainLoginResult::SuccessSurvey,
            MainLoginResult::FailParentalcontrol => V2MainLoginResult::FailParentalcontrol,
            MainLoginResult::FailLockedEnforced => return Err(CollectiveError::InvalidFieldSet),
        })
    }
}
