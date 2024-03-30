use crate::collective::CollectiveMessage;
use crate::errors::CollectiveError;

type Main = crate::version_8::CMD_AUTH_LOGON_CHALLENGE_Server;
type MainLoginResult = crate::version_8::CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult;
type MainSecurityFlag = crate::version_8::CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag;
type MainSecurityFlagPin = crate::version_8::CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin;
type MainSecurityFlagMatrixCard =
    crate::version_8::CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_MatrixCard;

type V2Main = crate::version_2::CMD_AUTH_LOGON_CHALLENGE_Server;

type V3Main = crate::version_3::CMD_AUTH_LOGON_CHALLENGE_Server;
type V3MainLoginResult = crate::version_3::CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult;
type V3MainSecurityFlag = crate::version_3::CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag;

type V5Main = crate::version_5::CMD_AUTH_LOGON_CHALLENGE_Server;
type V5MainLoginResult = crate::version_5::CMD_AUTH_LOGON_CHALLENGE_Server_LoginResult;
type V5MainSecurityFlag = crate::version_5::CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag;
type V5MainSecurityFlagPin = crate::version_5::CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin;
type V5MainSecurityFlagMatrixCard =
    crate::version_5::CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_MatrixCard;

impl CollectiveMessage for Main {
    type Version2 = V2Main;
    type Version3 = V3Main;
    type Version5 = V5Main;
    type Version6 = Self::Version5;
    type Version7 = Self::Version5;
    type Version8 = Self;

    fn from_version_2(v: Self::Version2) -> Self {
        Self {
            result: match v {
                V2Main::Success {
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
                V2Main::FailUnknown0 => MainLoginResult::FailUnknown0,
                V2Main::FailUnknown1 => MainLoginResult::FailUnknown1,
                V2Main::FailBanned => MainLoginResult::FailBanned,
                V2Main::FailUnknownAccount => MainLoginResult::FailUnknownAccount,
                V2Main::FailIncorrectPassword => MainLoginResult::FailIncorrectPassword,
                V2Main::FailAlreadyOnline => MainLoginResult::FailAlreadyOnline,
                V2Main::FailNoTime => MainLoginResult::FailNoTime,
                V2Main::FailDbBusy => MainLoginResult::FailDbBusy,
                V2Main::FailVersionInvalid => MainLoginResult::FailVersionInvalid,
                V2Main::LoginDownloadFile => MainLoginResult::LoginDownloadFile,
                V2Main::FailInvalidServer => MainLoginResult::FailInvalidServer,
                V2Main::FailSuspended => MainLoginResult::FailSuspended,
                V2Main::FailNoAccess => MainLoginResult::FailNoAccess,
                V2Main::SuccessSurvey => MainLoginResult::SuccessSurvey,
                V2Main::FailParentalcontrol => MainLoginResult::FailParentalcontrol,
            },
        }
    }

    fn to_version_2(&self) -> Result<Self::Version2, CollectiveError> {
        Ok(match &self.result {
            MainLoginResult::Success {
                crc_salt,
                generator,
                large_safe_prime,
                salt,
                server_public_key,
                ..
            } => V2Main::Success {
                crc_salt: *crc_salt,
                generator: generator.clone(),
                large_safe_prime: large_safe_prime.clone(),
                salt: *salt,
                server_public_key: *server_public_key,
            },
            MainLoginResult::FailUnknown0 => V2Main::FailUnknown0,
            MainLoginResult::FailUnknown1 => V2Main::FailUnknown1,
            MainLoginResult::FailBanned => V2Main::FailBanned,
            MainLoginResult::FailUnknownAccount => V2Main::FailUnknownAccount,
            MainLoginResult::FailIncorrectPassword => V2Main::FailIncorrectPassword,
            MainLoginResult::FailAlreadyOnline => V2Main::FailAlreadyOnline,
            MainLoginResult::FailNoTime => V2Main::FailNoTime,
            MainLoginResult::FailDbBusy => V2Main::FailDbBusy,
            MainLoginResult::FailVersionInvalid => V2Main::FailVersionInvalid,
            MainLoginResult::LoginDownloadFile => V2Main::LoginDownloadFile,
            MainLoginResult::FailInvalidServer => V2Main::FailInvalidServer,
            MainLoginResult::FailSuspended => V2Main::FailSuspended,
            MainLoginResult::FailNoAccess => V2Main::FailNoAccess,
            MainLoginResult::SuccessSurvey => V2Main::SuccessSurvey,
            MainLoginResult::FailParentalcontrol => V2Main::FailParentalcontrol,
            MainLoginResult::FailLockedEnforced => {
                return Err(CollectiveError::InvalidFieldSet);
            }
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
        Self {
            result: match v.result {
                V5MainLoginResult::Success {
                    crc_salt,
                    generator,
                    large_safe_prime,
                    salt,
                    security_flag,
                    server_public_key,
                } => MainLoginResult::Success {
                    crc_salt,
                    generator,
                    large_safe_prime,
                    salt,
                    security_flag: MainSecurityFlag::from_version_5(security_flag),
                    server_public_key,
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
        Ok(V5Main {
            result: match self.result.clone() {
                MainLoginResult::Success {
                    crc_salt,
                    generator,
                    large_safe_prime,
                    salt,
                    security_flag,
                    server_public_key,
                } => V5MainLoginResult::Success {
                    crc_salt,
                    generator,
                    large_safe_prime,
                    salt,
                    security_flag: security_flag.to_version_5(),
                    server_public_key,
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
                    return Err(CollectiveError::InvalidFieldSet)
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

impl MainSecurityFlag {
    pub fn from_version_5(v: V5MainSecurityFlag) -> Self {
        let mut security_flag = MainSecurityFlag::empty();

        if let Some(pin) = v.get_pin() {
            security_flag = security_flag.set_pin(MainSecurityFlagPin {
                pin_grid_seed: pin.pin_grid_seed,
                pin_salt: pin.pin_salt,
            });
        }

        if let Some(matrix_card) = v.get_matrix_card() {
            security_flag = security_flag.set_matrix_card(MainSecurityFlagMatrixCard {
                challenge_count: matrix_card.challenge_count,
                digit_count: matrix_card.digit_count,
                height: matrix_card.height,
                seed: matrix_card.seed,
                width: matrix_card.width,
            });
        }

        security_flag
    }

    pub fn to_version_5(&self) -> V5MainSecurityFlag {
        let mut security_flag = V5MainSecurityFlag::empty();

        if let Some(pin) = self.get_pin() {
            security_flag = security_flag.set_pin(V5MainSecurityFlagPin {
                pin_grid_seed: pin.pin_grid_seed,
                pin_salt: pin.pin_salt,
            });
        }

        if let Some(matrix_card) = self.get_matrix_card() {
            security_flag = security_flag.set_matrix_card(V5MainSecurityFlagMatrixCard {
                challenge_count: matrix_card.challenge_count,
                digit_count: matrix_card.digit_count,
                height: matrix_card.height,
                seed: matrix_card.seed,
                width: matrix_card.width,
            });
        }

        security_flag
    }
}
