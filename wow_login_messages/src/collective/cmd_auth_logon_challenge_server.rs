use crate::collective::CollectiveMessage;
use crate::errors::CollectiveError;

type Main = crate::version_8::CMD_AUTH_LOGON_CHALLENGE_Server;
type MainSecurityFlag = crate::version_8::CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag;
type MainSecurityFlagPin = crate::version_8::CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_Pin;
type MainSecurityFlagMatrixCard =
    crate::version_8::CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag_MatrixCard;

type V2Main = crate::version_2::CMD_AUTH_LOGON_CHALLENGE_Server;

type V3Main = crate::version_3::CMD_AUTH_LOGON_CHALLENGE_Server;
type V3MainSecurityFlag = crate::version_3::CMD_AUTH_LOGON_CHALLENGE_Server_SecurityFlag;

type V5Main = crate::version_5::CMD_AUTH_LOGON_CHALLENGE_Server;
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
        match v {
            V2Main::Success {
                crc_salt,
                generator,
                large_safe_prime,
                salt,
                server_public_key,
            } => Main::Success {
                crc_salt,
                generator,
                large_safe_prime,
                salt,
                security_flag: Default::default(),
                server_public_key,
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
        match v {
            V3Main::Success {
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

                Main::Success {
                    crc_salt,
                    generator,
                    large_safe_prime,
                    salt,
                    security_flag,
                    server_public_key,
                }
            }
            V3Main::FailUnknown0 => Main::FailUnknown0,
            V3Main::FailUnknown1 => Main::FailUnknown1,
            V3Main::FailBanned => Main::FailBanned,
            V3Main::FailUnknownAccount => Main::FailUnknownAccount,
            V3Main::FailIncorrectPassword => Main::FailIncorrectPassword,
            V3Main::FailAlreadyOnline => Main::FailAlreadyOnline,
            V3Main::FailNoTime => Main::FailNoTime,
            V3Main::FailDbBusy => Main::FailDbBusy,
            V3Main::FailVersionInvalid => Main::FailVersionInvalid,
            V3Main::LoginDownloadFile => Main::LoginDownloadFile,
            V3Main::FailInvalidServer => Main::FailInvalidServer,
            V3Main::FailSuspended => Main::FailSuspended,
            V3Main::FailNoAccess => Main::FailNoAccess,
            V3Main::SuccessSurvey => Main::SuccessSurvey,
            V3Main::FailParentalcontrol => Main::FailParentalcontrol,
        }
    }

    fn to_version_3(&self) -> Result<Self::Version3, CollectiveError> {
        Ok(match &self {
            Main::Success {
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

                V3Main::Success {
                    crc_salt: *crc_salt,
                    generator: generator.clone(),
                    large_safe_prime: large_safe_prime.clone(),
                    salt: *salt,
                    security_flag,
                    server_public_key: *server_public_key,
                }
            }
            Main::FailUnknown0 => V3Main::FailUnknown0,
            Main::FailUnknown1 => V3Main::FailUnknown1,
            Main::FailBanned => V3Main::FailBanned,
            Main::FailUnknownAccount => V3Main::FailUnknownAccount,
            Main::FailIncorrectPassword => V3Main::FailIncorrectPassword,
            Main::FailAlreadyOnline => V3Main::FailAlreadyOnline,
            Main::FailNoTime => V3Main::FailNoTime,
            Main::FailDbBusy => V3Main::FailDbBusy,
            Main::FailVersionInvalid => V3Main::FailVersionInvalid,
            Main::LoginDownloadFile => V3Main::LoginDownloadFile,
            Main::FailInvalidServer => V3Main::FailInvalidServer,
            Main::FailSuspended => V3Main::FailSuspended,
            Main::FailNoAccess => V3Main::FailNoAccess,
            Main::SuccessSurvey => V3Main::SuccessSurvey,
            Main::FailParentalcontrol => V3Main::FailParentalcontrol,
            Main::FailLockedEnforced => {
                return Err(CollectiveError::InvalidFieldSet);
            }
        })
    }

    fn from_version_5(v: Self::Version5) -> Self {
        match v {
            V5Main::Success {
                crc_salt,
                generator,
                large_safe_prime,
                salt,
                security_flag,
                server_public_key,
            } => Main::Success {
                crc_salt,
                generator,
                large_safe_prime,
                salt,
                security_flag: MainSecurityFlag::from_version_5(security_flag),
                server_public_key,
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
        Ok(match self.clone() {
            Main::Success {
                crc_salt,
                generator,
                large_safe_prime,
                salt,
                security_flag,
                server_public_key,
            } => V5Main::Success {
                crc_salt,
                generator,
                large_safe_prime,
                salt,
                security_flag: security_flag.to_version_5(),
                server_public_key,
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
            Main::FailLockedEnforced => return Err(CollectiveError::InvalidFieldSet),
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
