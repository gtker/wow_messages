use crate::collective::CollectiveMessage;
use crate::errors::CollectiveError;

type Main = crate::version_8::CMD_AUTH_LOGON_PROOF_Server;

type V2Main = crate::version_2::CMD_AUTH_LOGON_PROOF_Server;

type V5Main = crate::version_5::CMD_AUTH_LOGON_PROOF_Server;

impl CollectiveMessage for Main {
    type Version2 = V2Main;
    type Version3 = Self::Version2;
    type Version5 = V5Main;
    type Version6 = Self::Version5;
    type Version7 = Self::Version5;
    type Version8 = Self;

    fn from_version_2(v: Self::Version2) -> Self {
        match v {
            V2Main::Success {
                hardware_survey_id,
                server_proof,
            } => Main::Success {
                account_flag: Default::default(),
                hardware_survey_id,
                server_proof,
                unknown: Default::default(),
            },
            V2Main::FailUnknown0 => Main::FailUnknown0,
            V2Main::FailUnknown1 => Main::FailUnknown1,
            V2Main::FailBanned => Main::FailBanned,
            V2Main::FailUnknownAccount => Main::FailUnknownAccount,
            V2Main::FailIncorrectPassword => Main::FailIncorrectPassword,
            V2Main::FailAlreadyOnline => Main::FailAlreadyOnline,
            V2Main::FailNoTime => Main::FailNoTime,
            V2Main::FailDbBusy => Main::FailDbBusy,
            V2Main::FailVersionInvalid => Main::FailVersionInvalid,
            V2Main::LoginDownloadFile => Main::LoginDownloadFile,
            V2Main::FailInvalidServer => Main::FailInvalidServer,
            V2Main::FailSuspended => Main::FailSuspended,
            V2Main::FailNoAccess => Main::FailNoAccess,
            V2Main::SuccessSurvey => Main::SuccessSurvey,
            V2Main::FailParentalcontrol => Main::FailParentalcontrol,
        }
    }

    fn to_version_2(&self) -> Result<Self::Version2, CollectiveError> {
        Ok(match &self {
            Main::Success {
                hardware_survey_id,
                server_proof,
                ..
            } => V2Main::Success {
                hardware_survey_id: *hardware_survey_id,
                server_proof: *server_proof,
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
            Main::FailLockedEnforced => {
                return Err(CollectiveError::InvalidFieldSet);
            }
        })
    }

    fn from_version_3(v: Self::Version3) -> Self {
        Self::from_version_2(v)
    }

    fn to_version_3(&self) -> Result<Self::Version3, CollectiveError> {
        self.to_version_2()
    }

    fn from_version_5(v: Self::Version5) -> Self {
        match v {
            V5Main::Success {
                hardware_survey_id,
                server_proof,
                unknown,
            } => Main::Success {
                account_flag: Default::default(),
                hardware_survey_id,
                server_proof,
                unknown,
            },
            V5Main::FailUnknown0 => Main::FailUnknown0,
            V5Main::FailUnknown1 => Main::FailUnknown1,
            V5Main::FailBanned => Main::FailBanned,
            V5Main::FailUnknownAccount => Main::FailUnknownAccount,
            V5Main::FailIncorrectPassword => Main::FailIncorrectPassword,
            V5Main::FailAlreadyOnline => Main::FailAlreadyOnline,
            V5Main::FailNoTime => Main::FailNoTime,
            V5Main::FailDbBusy => Main::FailDbBusy,
            V5Main::FailVersionInvalid => Main::FailVersionInvalid,
            V5Main::LoginDownloadFile => Main::LoginDownloadFile,
            V5Main::FailInvalidServer => Main::FailInvalidServer,
            V5Main::FailSuspended => Main::FailSuspended,
            V5Main::FailNoAccess => Main::FailNoAccess,
            V5Main::SuccessSurvey => Main::SuccessSurvey,
            V5Main::FailParentalcontrol => Main::FailParentalcontrol,
        }
    }

    fn to_version_5(&self) -> Result<Self::Version5, CollectiveError> {
        Ok(match *self {
            Main::Success {
                hardware_survey_id,
                server_proof,
                unknown,
                ..
            } => V5Main::Success {
                hardware_survey_id,
                server_proof,
                unknown,
            },
            Main::FailUnknown0 => V5Main::FailUnknown0,
            Main::FailUnknown1 => V5Main::FailUnknown1,
            Main::FailBanned => V5Main::FailBanned,
            Main::FailUnknownAccount => V5Main::FailUnknownAccount,
            Main::FailIncorrectPassword => V5Main::FailIncorrectPassword,
            Main::FailAlreadyOnline => V5Main::FailAlreadyOnline,
            Main::FailNoTime => V5Main::FailNoTime,
            Main::FailDbBusy => V5Main::FailDbBusy,
            Main::FailVersionInvalid => V5Main::FailVersionInvalid,
            Main::LoginDownloadFile => V5Main::LoginDownloadFile,
            Main::FailInvalidServer => V5Main::FailInvalidServer,
            Main::FailSuspended => V5Main::FailSuspended,
            Main::FailNoAccess => V5Main::FailNoAccess,
            Main::SuccessSurvey => V5Main::SuccessSurvey,
            Main::FailParentalcontrol => V5Main::FailParentalcontrol,
            Main::FailLockedEnforced => {
                return Err(CollectiveError::InvalidFieldSet);
            }
        })
    }

    fn from_version_6(v: Self::Version6) -> Self {
        Self::from_version_5(v)
    }

    fn to_version_6(&self) -> Result<Self::Version6, CollectiveError> {
        self.to_version_5()
    }

    fn from_version_7(v: Self::Version7) -> Self {
        Self::from_version_5(v)
    }

    fn to_version_7(&self) -> Result<Self::Version7, CollectiveError> {
        self.to_version_5()
    }
}
