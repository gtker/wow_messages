use crate::collective::CollectiveMessage;
use crate::errors::CollectiveError;

type Main = crate::version_8::CMD_AUTH_LOGON_PROOF_Server;
type MainLoginResult = crate::version_8::CMD_AUTH_LOGON_PROOF_Server_LoginResult;

type V2Main = crate::version_2::CMD_AUTH_LOGON_PROOF_Server;
type V2MainLoginResult = crate::version_2::CMD_AUTH_LOGON_PROOF_Server_LoginResult;

type V5Main = crate::version_5::CMD_AUTH_LOGON_PROOF_Server;
type V5MainLoginResult = crate::version_5::CMD_AUTH_LOGON_PROOF_Server_LoginResult;

impl CollectiveMessage for Main {
    type Version2 = V2Main;
    type Version3 = Self::Version2;
    type Version5 = V5Main;
    type Version6 = Self::Version5;
    type Version7 = Self::Version5;
    type Version8 = Self;

    fn from_version_2(v: Self::Version2) -> Self {
        Self {
            result: match v.result {
                V2MainLoginResult::Success {
                    hardware_survey_id,
                    server_proof,
                } => MainLoginResult::Success {
                    account_flag: Default::default(),
                    hardware_survey_id,
                    server_proof,
                    unknown: Default::default(),
                },
                V2MainLoginResult::FailUnknown0 => MainLoginResult::FailUnknown0,
                V2MainLoginResult::FailUnknown1 => MainLoginResult::FailUnknown1,
                V2MainLoginResult::FailBanned => MainLoginResult::FailBanned,
                V2MainLoginResult::FailUnknownAccount => MainLoginResult::FailUnknownAccount,
                V2MainLoginResult::FailIncorrectPassword => MainLoginResult::FailIncorrectPassword,
                V2MainLoginResult::FailAlreadyOnline => MainLoginResult::FailAlreadyOnline,
                V2MainLoginResult::FailNoTime => MainLoginResult::FailNoTime,
                V2MainLoginResult::FailDbBusy => MainLoginResult::FailDbBusy,
                V2MainLoginResult::FailVersionInvalid => MainLoginResult::FailVersionInvalid,
                V2MainLoginResult::LoginDownloadFile => MainLoginResult::LoginDownloadFile,
                V2MainLoginResult::FailInvalidServer => MainLoginResult::FailInvalidServer,
                V2MainLoginResult::FailSuspended => MainLoginResult::FailSuspended,
                V2MainLoginResult::FailNoAccess => MainLoginResult::FailNoAccess,
                V2MainLoginResult::SuccessSurvey => MainLoginResult::SuccessSurvey,
                V2MainLoginResult::FailParentalcontrol => MainLoginResult::FailParentalcontrol,
            },
        }
    }

    fn to_version_2(&self) -> Result<Self::Version2, CollectiveError> {
        Ok(Self::Version2 {
            result: match &self.result {
                MainLoginResult::Success {
                    hardware_survey_id,
                    server_proof,
                    ..
                } => V2MainLoginResult::Success {
                    hardware_survey_id: *hardware_survey_id,
                    server_proof: *server_proof,
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
                MainLoginResult::FailLockedEnforced => {
                    return Err(CollectiveError::InvalidFieldSet);
                }
            },
        })
    }

    fn from_version_3(v: Self::Version3) -> Self {
        Self::from_version_2(v)
    }

    fn to_version_3(&self) -> Result<Self::Version3, CollectiveError> {
        self.to_version_2()
    }

    fn from_version_5(v: Self::Version5) -> Self {
        Self {
            result: match v.result {
                V5MainLoginResult::Success {
                    hardware_survey_id,
                    server_proof,
                    unknown,
                } => MainLoginResult::Success {
                    account_flag: Default::default(),
                    hardware_survey_id,
                    server_proof,
                    unknown,
                },
                V5MainLoginResult::FailUnknown0 => MainLoginResult::FailUnknown0,
                V5MainLoginResult::FailUnknown1 => MainLoginResult::FailUnknown1,
                V5MainLoginResult::FailBanned => MainLoginResult::FailBanned,
                V5MainLoginResult::FailUnknownAccount => MainLoginResult::FailUnknownAccount,
                V5MainLoginResult::FailIncorrectPassword => MainLoginResult::FailIncorrectPassword,
                V5MainLoginResult::FailAlreadyOnline => MainLoginResult::FailAlreadyOnline,
                V5MainLoginResult::FailNoTime => MainLoginResult::FailNoTime,
                V5MainLoginResult::FailDbBusy => MainLoginResult::FailDbBusy,
                V5MainLoginResult::FailVersionInvalid => MainLoginResult::FailVersionInvalid,
                V5MainLoginResult::LoginDownloadFile => MainLoginResult::LoginDownloadFile,
                V5MainLoginResult::FailInvalidServer => MainLoginResult::FailInvalidServer,
                V5MainLoginResult::FailSuspended => MainLoginResult::FailSuspended,
                V5MainLoginResult::FailNoAccess => MainLoginResult::FailNoAccess,
                V5MainLoginResult::SuccessSurvey => MainLoginResult::SuccessSurvey,
                V5MainLoginResult::FailParentalcontrol => MainLoginResult::FailParentalcontrol,
            },
        }
    }

    fn to_version_5(&self) -> Result<Self::Version5, CollectiveError> {
        Ok(Self::Version5 {
            result: match self.result {
                MainLoginResult::Success {
                    hardware_survey_id,
                    server_proof,
                    unknown,
                    ..
                } => V5MainLoginResult::Success {
                    hardware_survey_id,
                    server_proof,
                    unknown,
                },
                MainLoginResult::FailUnknown0 => V5MainLoginResult::FailUnknown0,
                MainLoginResult::FailUnknown1 => V5MainLoginResult::FailUnknown1,
                MainLoginResult::FailBanned => V5MainLoginResult::FailBanned,
                MainLoginResult::FailUnknownAccount => V5MainLoginResult::FailUnknownAccount,
                MainLoginResult::FailIncorrectPassword => V5MainLoginResult::FailIncorrectPassword,
                MainLoginResult::FailAlreadyOnline => V5MainLoginResult::FailAlreadyOnline,
                MainLoginResult::FailNoTime => V5MainLoginResult::FailNoTime,
                MainLoginResult::FailDbBusy => V5MainLoginResult::FailDbBusy,
                MainLoginResult::FailVersionInvalid => V5MainLoginResult::FailVersionInvalid,
                MainLoginResult::LoginDownloadFile => V5MainLoginResult::LoginDownloadFile,
                MainLoginResult::FailInvalidServer => V5MainLoginResult::FailInvalidServer,
                MainLoginResult::FailSuspended => V5MainLoginResult::FailSuspended,
                MainLoginResult::FailNoAccess => V5MainLoginResult::FailNoAccess,
                MainLoginResult::SuccessSurvey => V5MainLoginResult::SuccessSurvey,
                MainLoginResult::FailParentalcontrol => V5MainLoginResult::FailParentalcontrol,
                MainLoginResult::FailLockedEnforced => {
                    return Err(CollectiveError::InvalidFieldSet);
                }
            },
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
