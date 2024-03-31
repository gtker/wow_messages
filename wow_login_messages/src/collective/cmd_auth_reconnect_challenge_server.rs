use crate::collective::CollectiveMessage;

type Main = crate::version_8::CMD_AUTH_RECONNECT_CHALLENGE_Server;

type V2Main = crate::version_2::CMD_AUTH_RECONNECT_CHALLENGE_Server;

impl CollectiveMessage for Main {
    type Version2 = V2Main;
    type Version3 = Self::Version2;
    type Version5 = Self::Version2;
    type Version6 = Self::Version2;
    type Version7 = Self::Version2;
    type Version8 = Self;

    fn from_version_2(v: Self::Version2) -> Self {
        Main::from_version_2(v)
    }

    fn to_version_2(&self) -> Self::Version2 {
        self.to_version_2()
    }

    fn from_version_3(v: Self::Version3) -> Self {
        Self::from_version_2(v)
    }

    fn to_version_3(&self) -> Self::Version3 {
        self.to_version_2()
    }

    fn from_version_5(v: Self::Version5) -> Self {
        Self::from_version_2(v)
    }

    fn to_version_5(&self) -> Self::Version5 {
        self.to_version_2()
    }

    fn from_version_6(v: Self::Version6) -> Self {
        Self::from_version_2(v)
    }

    fn to_version_6(&self) -> Self::Version6 {
        self.to_version_2()
    }

    fn from_version_7(v: Self::Version7) -> Self {
        Self::from_version_2(v)
    }

    fn to_version_7(&self) -> Self::Version7 {
        self.to_version_2()
    }
}

impl Main {
    const fn from_version_2(v: V2Main) -> Self {
        match v {
            V2Main::Success {
                challenge_data,
                checksum_salt,
            } => Self::Success {
                challenge_data,
                checksum_salt,
            },
            V2Main::FailUnknown0 => Self::FailUnknown0,
            V2Main::FailUnknown1 => Self::FailUnknown1,
            V2Main::FailBanned => Self::FailBanned,
            V2Main::FailUnknownAccount => Self::FailUnknownAccount,
            V2Main::FailIncorrectPassword => Self::FailIncorrectPassword,
            V2Main::FailAlreadyOnline => Self::FailAlreadyOnline,
            V2Main::FailNoTime => Self::FailNoTime,
            V2Main::FailDbBusy => Self::FailDbBusy,
            V2Main::FailVersionInvalid => Self::FailVersionInvalid,
            V2Main::LoginDownloadFile => Self::LoginDownloadFile,
            V2Main::FailInvalidServer => Self::FailInvalidServer,
            V2Main::FailSuspended => Self::FailSuspended,
            V2Main::FailNoAccess => Self::FailNoAccess,
            V2Main::SuccessSurvey => Self::SuccessSurvey,
            V2Main::FailParentalcontrol => Self::FailParentalcontrol,
        }
    }

    #[allow(clippy::wrong_self_convention)]
    const fn to_version_2(&self) -> V2Main {
        match self {
            Main::Success {
                challenge_data,
                checksum_salt,
            } => V2Main::Success {
                challenge_data: *challenge_data,
                checksum_salt: *checksum_salt,
            },
            Main::FailUnknown0 => V2Main::FailUnknown0,
            Main::FailUnknown1 => V2Main::FailUnknown1,
            Main::FailBanned => V2Main::FailBanned,
            Main::FailUnknownAccount => V2Main::FailUnknownAccount,
            Main::FailIncorrectPassword => V2Main::FailIncorrectPassword,
            Main::FailAlreadyOnline => V2Main::FailAlreadyOnline,
            Main::FailNoTime => V2Main::FailNoTime,
            Main::FailDbBusy => V2Main::FailDbBusy,
            Main::FailVersionInvalid => V2Main::FailVersionInvalid,
            Main::LoginDownloadFile => V2Main::LoginDownloadFile,
            Main::FailInvalidServer => V2Main::FailInvalidServer,
            Main::FailSuspended => V2Main::FailSuspended,
            Main::FailNoAccess => V2Main::FailNoAccess,
            Main::SuccessSurvey => V2Main::SuccessSurvey,
            Main::FailParentalcontrol => V2Main::FailParentalcontrol,
            Main::FailLockedEnforced => V2Main::FailParentalcontrol,
        }
    }
}
