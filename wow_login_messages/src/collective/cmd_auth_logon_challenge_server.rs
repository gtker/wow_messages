use crate::collective::CollectiveMessage;
use crate::errors::CollectiveError;

type Main = crate::version_8::CMD_AUTH_LOGON_CHALLENGE_Server;
type MainLoginResult = crate::version_8::CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult;
type MainSecurityFlag = crate::version_8::CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag;
type MainSecurityFlagPin = crate::version_8::CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin;

type V2Main = crate::version_2::CMD_AUTH_LOGON_CHALLENGE_Server;
type V2MainLoginResult = crate::version_2::CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult;

type V3Main = crate::version_3::CMD_AUTH_LOGON_CHALLENGE_Server;
type V3MainLoginResult = crate::version_3::CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult;
type V3MainSecurityFlag = crate::version_3::CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag;

impl CollectiveMessage for Main {
    type Version2 = V2Main;
    type Version3 = V3Main;
    type Version5 = Self::Version3;
    type Version6 = Self::Version3;
    type Version7 = Self::Version3;
    type Version8 = Self;

    fn from_version_2(v: Self::Version2) -> Self {
        Self {
            result: match v.result {
                V2MainLoginResult::Success {
                    crc_salt,
                    generator,
                    large_safe_prime,
                    salt,
                    server_public_key,
                } => MainLoginResult::Success {
                    crc_salt,
                    generator,
                    large_safe_prime,
                    salt,
                    security_flag: Default::default(),
                    server_public_key,
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
                    crc_salt,
                    generator,
                    large_safe_prime,
                    salt,
                    server_public_key,
                    ..
                } => V2MainLoginResult::Success {
                    crc_salt: *crc_salt,
                    generator: generator.clone(),
                    large_safe_prime: large_safe_prime.clone(),
                    salt: *salt,
                    server_public_key: *server_public_key,
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
        Self {
            result: match v.result {
                V3MainLoginResult::Success {
                    crc_salt,
                    generator,
                    large_safe_prime,
                    salt,
                    security_flag,
                    server_public_key,
                } => {
                    let security_flag = match security_flag {
                        V3MainSecurityFlag::None => MainSecurityFlag::empty(),
                        V3MainSecurityFlag::Pin {
                            pin_grid_seed,
                            pin_salt,
                        } => MainSecurityFlag::new_pin(MainSecurityFlagPin {
                            pin_grid_seed,
                            pin_salt,
                        }),
                    };

                    MainLoginResult::Success {
                        crc_salt,
                        generator,
                        large_safe_prime,
                        salt,
                        security_flag,
                        server_public_key,
                    }
                }
                V3MainLoginResult::FailUnknown0 => MainLoginResult::FailUnknown0,
                V3MainLoginResult::FailUnknown1 => MainLoginResult::FailUnknown1,
                V3MainLoginResult::FailBanned => MainLoginResult::FailBanned,
                V3MainLoginResult::FailUnknownAccount => MainLoginResult::FailUnknownAccount,
                V3MainLoginResult::FailIncorrectPassword => MainLoginResult::FailIncorrectPassword,
                V3MainLoginResult::FailAlreadyOnline => MainLoginResult::FailAlreadyOnline,
                V3MainLoginResult::FailNoTime => MainLoginResult::FailNoTime,
                V3MainLoginResult::FailDbBusy => MainLoginResult::FailDbBusy,
                V3MainLoginResult::FailVersionInvalid => MainLoginResult::FailVersionInvalid,
                V3MainLoginResult::LoginDownloadFile => MainLoginResult::LoginDownloadFile,
                V3MainLoginResult::FailInvalidServer => MainLoginResult::FailInvalidServer,
                V3MainLoginResult::FailSuspended => MainLoginResult::FailSuspended,
                V3MainLoginResult::FailNoAccess => MainLoginResult::FailNoAccess,
                V3MainLoginResult::SuccessSurvey => MainLoginResult::SuccessSurvey,
                V3MainLoginResult::FailParentalcontrol => MainLoginResult::FailParentalcontrol,
            },
        }
    }

    fn to_version_3(&self) -> Result<Self::Version3, CollectiveError> {
        Ok(Self::Version3 {
            result: match &self.result {
                MainLoginResult::Success {
                    crc_salt,
                    generator,
                    large_safe_prime,
                    salt,
                    security_flag,
                    server_public_key,
                } => {
                    let security_flag = if let Some(pin) = security_flag.get_pin() {
                        V3MainSecurityFlag::Pin {
                            pin_grid_seed: pin.pin_grid_seed,
                            pin_salt: pin.pin_salt,
                        }
                    } else {
                        V3MainSecurityFlag::None
                    };

                    V3MainLoginResult::Success {
                        crc_salt: *crc_salt,
                        generator: generator.clone(),
                        large_safe_prime: large_safe_prime.clone(),
                        salt: *salt,
                        security_flag,
                        server_public_key: *server_public_key,
                    }
                }
                MainLoginResult::FailUnknown0 => V3MainLoginResult::FailUnknown0,
                MainLoginResult::FailUnknown1 => V3MainLoginResult::FailUnknown1,
                MainLoginResult::FailBanned => V3MainLoginResult::FailBanned,
                MainLoginResult::FailUnknownAccount => V3MainLoginResult::FailUnknownAccount,
                MainLoginResult::FailIncorrectPassword => V3MainLoginResult::FailIncorrectPassword,
                MainLoginResult::FailAlreadyOnline => V3MainLoginResult::FailAlreadyOnline,
                MainLoginResult::FailNoTime => V3MainLoginResult::FailNoTime,
                MainLoginResult::FailDbBusy => V3MainLoginResult::FailDbBusy,
                MainLoginResult::FailVersionInvalid => V3MainLoginResult::FailVersionInvalid,
                MainLoginResult::LoginDownloadFile => V3MainLoginResult::LoginDownloadFile,
                MainLoginResult::FailInvalidServer => V3MainLoginResult::FailInvalidServer,
                MainLoginResult::FailSuspended => V3MainLoginResult::FailSuspended,
                MainLoginResult::FailNoAccess => V3MainLoginResult::FailNoAccess,
                MainLoginResult::SuccessSurvey => V3MainLoginResult::SuccessSurvey,
                MainLoginResult::FailParentalcontrol => V3MainLoginResult::FailParentalcontrol,
                MainLoginResult::FailLockedEnforced => {
                    return Err(CollectiveError::InvalidFieldSet);
                }
            },
        })
    }

    fn from_version_5(v: Self::Version5) -> Self {
        Self::from_version_3(v)
    }

    fn to_version_5(&self) -> Result<Self::Version5, CollectiveError> {
        self.to_version_3()
    }

    fn from_version_6(v: Self::Version6) -> Self {
        Self::from_version_3(v)
    }

    fn to_version_6(&self) -> Result<Self::Version6, CollectiveError> {
        self.to_version_3()
    }

    fn from_version_7(v: Self::Version7) -> Self {
        Self::from_version_3(v)
    }

    fn to_version_7(&self) -> Result<Self::Version7, CollectiveError> {
        self.to_version_3()
    }
}
