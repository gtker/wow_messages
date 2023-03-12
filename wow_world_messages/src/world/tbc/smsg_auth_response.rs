use std::io::{Read, Write};

use crate::tbc::{
    BillingPlanFlags, Expansion, WorldResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm:82`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm#L82):
/// ```text
/// smsg SMSG_AUTH_RESPONSE = 0x01EE {
///     WorldResult result;
///     if (result == AUTH_OK) {
///         u32 billing_time;
///         BillingPlanFlags billing_flags;
///         u32 billing_rested;
///         Expansion expansion;
///     }
///     else if (result == AUTH_WAIT_QUEUE) {
///         u32 queue_position;
///     }
/// }
/// ```
pub struct SMSG_AUTH_RESPONSE {
    pub result: SMSG_AUTH_RESPONSE_WorldResult,
}

impl crate::Message for SMSG_AUTH_RESPONSE {
    const OPCODE: u32 = 0x01ee;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // result: WorldResult
        w.write_all(&u8::from(self.result.as_int()).to_le_bytes())?;

        match &self.result {
            SMSG_AUTH_RESPONSE_WorldResult::AuthOk {
                billing_flags,
                billing_rested,
                billing_time,
                expansion,
            } => {
                // billing_time: u32
                w.write_all(&billing_time.to_le_bytes())?;

                // billing_flags: BillingPlanFlags
                w.write_all(&u8::from(billing_flags.as_int()).to_le_bytes())?;

                // billing_rested: u32
                w.write_all(&billing_rested.to_le_bytes())?;

                // expansion: Expansion
                w.write_all(&u8::from(expansion.as_int()).to_le_bytes())?;

            }
            SMSG_AUTH_RESPONSE_WorldResult::AuthWaitQueue {
                queue_position,
            } => {
                // queue_position: u32
                w.write_all(&queue_position.to_le_bytes())?;

            }
            _ => {}
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=11).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01EE, size: body_size as u32 });
        }

        // result: WorldResult
        let result: WorldResult = crate::util::read_u8_le(&mut r)?.try_into()?;

        let result_if = match result {
            WorldResult::ResponseSuccess => SMSG_AUTH_RESPONSE_WorldResult::ResponseSuccess,
            WorldResult::ResponseFailure => SMSG_AUTH_RESPONSE_WorldResult::ResponseFailure,
            WorldResult::ResponseCancelled => SMSG_AUTH_RESPONSE_WorldResult::ResponseCancelled,
            WorldResult::ResponseDisconnected => SMSG_AUTH_RESPONSE_WorldResult::ResponseDisconnected,
            WorldResult::ResponseFailedToConnect => SMSG_AUTH_RESPONSE_WorldResult::ResponseFailedToConnect,
            WorldResult::ResponseConnected => SMSG_AUTH_RESPONSE_WorldResult::ResponseConnected,
            WorldResult::ResponseVersionMismatch => SMSG_AUTH_RESPONSE_WorldResult::ResponseVersionMismatch,
            WorldResult::CstatusConnecting => SMSG_AUTH_RESPONSE_WorldResult::CstatusConnecting,
            WorldResult::CstatusNegotiatingSecurity => SMSG_AUTH_RESPONSE_WorldResult::CstatusNegotiatingSecurity,
            WorldResult::CstatusNegotiationComplete => SMSG_AUTH_RESPONSE_WorldResult::CstatusNegotiationComplete,
            WorldResult::CstatusNegotiationFailed => SMSG_AUTH_RESPONSE_WorldResult::CstatusNegotiationFailed,
            WorldResult::CstatusAuthenticating => SMSG_AUTH_RESPONSE_WorldResult::CstatusAuthenticating,
            WorldResult::AuthOk => {
                // billing_time: u32
                let billing_time = crate::util::read_u32_le(&mut r)?;

                // billing_flags: BillingPlanFlags
                let billing_flags = BillingPlanFlags::new(crate::util::read_u8_le(&mut r)?);

                // billing_rested: u32
                let billing_rested = crate::util::read_u32_le(&mut r)?;

                // expansion: Expansion
                let expansion: Expansion = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_AUTH_RESPONSE_WorldResult::AuthOk {
                    billing_flags,
                    billing_rested,
                    billing_time,
                    expansion,
                }
            }
            WorldResult::AuthFailed => SMSG_AUTH_RESPONSE_WorldResult::AuthFailed,
            WorldResult::AuthReject => SMSG_AUTH_RESPONSE_WorldResult::AuthReject,
            WorldResult::AuthBadServerProof => SMSG_AUTH_RESPONSE_WorldResult::AuthBadServerProof,
            WorldResult::AuthUnavailable => SMSG_AUTH_RESPONSE_WorldResult::AuthUnavailable,
            WorldResult::AuthSystemError => SMSG_AUTH_RESPONSE_WorldResult::AuthSystemError,
            WorldResult::AuthBillingError => SMSG_AUTH_RESPONSE_WorldResult::AuthBillingError,
            WorldResult::AuthBillingExpired => SMSG_AUTH_RESPONSE_WorldResult::AuthBillingExpired,
            WorldResult::AuthVersionMismatch => SMSG_AUTH_RESPONSE_WorldResult::AuthVersionMismatch,
            WorldResult::AuthUnknownAccount => SMSG_AUTH_RESPONSE_WorldResult::AuthUnknownAccount,
            WorldResult::AuthIncorrectPassword => SMSG_AUTH_RESPONSE_WorldResult::AuthIncorrectPassword,
            WorldResult::AuthSessionExpired => SMSG_AUTH_RESPONSE_WorldResult::AuthSessionExpired,
            WorldResult::AuthServerShuttingDown => SMSG_AUTH_RESPONSE_WorldResult::AuthServerShuttingDown,
            WorldResult::AuthAlreadyLoggingIn => SMSG_AUTH_RESPONSE_WorldResult::AuthAlreadyLoggingIn,
            WorldResult::AuthLoginServerNotFound => SMSG_AUTH_RESPONSE_WorldResult::AuthLoginServerNotFound,
            WorldResult::AuthWaitQueue => {
                // queue_position: u32
                let queue_position = crate::util::read_u32_le(&mut r)?;

                SMSG_AUTH_RESPONSE_WorldResult::AuthWaitQueue {
                    queue_position,
                }
            }
            WorldResult::AuthBanned => SMSG_AUTH_RESPONSE_WorldResult::AuthBanned,
            WorldResult::AuthAlreadyOnline => SMSG_AUTH_RESPONSE_WorldResult::AuthAlreadyOnline,
            WorldResult::AuthNoTime => SMSG_AUTH_RESPONSE_WorldResult::AuthNoTime,
            WorldResult::AuthDbBusy => SMSG_AUTH_RESPONSE_WorldResult::AuthDbBusy,
            WorldResult::AuthSuspended => SMSG_AUTH_RESPONSE_WorldResult::AuthSuspended,
            WorldResult::AuthParentalControl => SMSG_AUTH_RESPONSE_WorldResult::AuthParentalControl,
            WorldResult::AuthLockedEnforced => SMSG_AUTH_RESPONSE_WorldResult::AuthLockedEnforced,
            WorldResult::RealmListInProgress => SMSG_AUTH_RESPONSE_WorldResult::RealmListInProgress,
            WorldResult::RealmListSuccess => SMSG_AUTH_RESPONSE_WorldResult::RealmListSuccess,
            WorldResult::RealmListFailed => SMSG_AUTH_RESPONSE_WorldResult::RealmListFailed,
            WorldResult::RealmListInvalid => SMSG_AUTH_RESPONSE_WorldResult::RealmListInvalid,
            WorldResult::RealmListRealmNotFound => SMSG_AUTH_RESPONSE_WorldResult::RealmListRealmNotFound,
            WorldResult::AccountCreateInProgress => SMSG_AUTH_RESPONSE_WorldResult::AccountCreateInProgress,
            WorldResult::AccountCreateSuccess => SMSG_AUTH_RESPONSE_WorldResult::AccountCreateSuccess,
            WorldResult::AccountCreateFailed => SMSG_AUTH_RESPONSE_WorldResult::AccountCreateFailed,
            WorldResult::CharListRetrieving => SMSG_AUTH_RESPONSE_WorldResult::CharListRetrieving,
            WorldResult::CharListRetrieved => SMSG_AUTH_RESPONSE_WorldResult::CharListRetrieved,
            WorldResult::CharListFailed => SMSG_AUTH_RESPONSE_WorldResult::CharListFailed,
            WorldResult::CharCreateInProgress => SMSG_AUTH_RESPONSE_WorldResult::CharCreateInProgress,
            WorldResult::CharCreateSuccess => SMSG_AUTH_RESPONSE_WorldResult::CharCreateSuccess,
            WorldResult::CharCreateError => SMSG_AUTH_RESPONSE_WorldResult::CharCreateError,
            WorldResult::CharCreateFailed => SMSG_AUTH_RESPONSE_WorldResult::CharCreateFailed,
            WorldResult::CharCreateNameInUse => SMSG_AUTH_RESPONSE_WorldResult::CharCreateNameInUse,
            WorldResult::CharCreateDisabled => SMSG_AUTH_RESPONSE_WorldResult::CharCreateDisabled,
            WorldResult::CharCreatePvpTeamsViolation => SMSG_AUTH_RESPONSE_WorldResult::CharCreatePvpTeamsViolation,
            WorldResult::CharCreateServerLimit => SMSG_AUTH_RESPONSE_WorldResult::CharCreateServerLimit,
            WorldResult::CharCreateAccountLimit => SMSG_AUTH_RESPONSE_WorldResult::CharCreateAccountLimit,
            WorldResult::CharCreateServerQueue => SMSG_AUTH_RESPONSE_WorldResult::CharCreateServerQueue,
            WorldResult::CharCreateOnlyExisting => SMSG_AUTH_RESPONSE_WorldResult::CharCreateOnlyExisting,
            WorldResult::CharCreateExpansion => SMSG_AUTH_RESPONSE_WorldResult::CharCreateExpansion,
            WorldResult::CharDeleteInProgress => SMSG_AUTH_RESPONSE_WorldResult::CharDeleteInProgress,
            WorldResult::CharDeleteSuccess => SMSG_AUTH_RESPONSE_WorldResult::CharDeleteSuccess,
            WorldResult::CharDeleteFailed => SMSG_AUTH_RESPONSE_WorldResult::CharDeleteFailed,
            WorldResult::CharDeleteFailedLockedForTransfer => SMSG_AUTH_RESPONSE_WorldResult::CharDeleteFailedLockedForTransfer,
            WorldResult::CharDeleteFailedGuildLeader => SMSG_AUTH_RESPONSE_WorldResult::CharDeleteFailedGuildLeader,
            WorldResult::CharDeleteFailedArenaCaptain => SMSG_AUTH_RESPONSE_WorldResult::CharDeleteFailedArenaCaptain,
            WorldResult::CharLoginInProgress => SMSG_AUTH_RESPONSE_WorldResult::CharLoginInProgress,
            WorldResult::CharLoginSuccess => SMSG_AUTH_RESPONSE_WorldResult::CharLoginSuccess,
            WorldResult::CharLoginNoWorld => SMSG_AUTH_RESPONSE_WorldResult::CharLoginNoWorld,
            WorldResult::CharLoginDuplicateCharacter => SMSG_AUTH_RESPONSE_WorldResult::CharLoginDuplicateCharacter,
            WorldResult::CharLoginNoInstances => SMSG_AUTH_RESPONSE_WorldResult::CharLoginNoInstances,
            WorldResult::CharLoginFailed => SMSG_AUTH_RESPONSE_WorldResult::CharLoginFailed,
            WorldResult::CharLoginDisabled => SMSG_AUTH_RESPONSE_WorldResult::CharLoginDisabled,
            WorldResult::CharLoginNoCharacter => SMSG_AUTH_RESPONSE_WorldResult::CharLoginNoCharacter,
            WorldResult::CharLoginLockedForTransfer => SMSG_AUTH_RESPONSE_WorldResult::CharLoginLockedForTransfer,
            WorldResult::CharLoginLockedByBilling => SMSG_AUTH_RESPONSE_WorldResult::CharLoginLockedByBilling,
            WorldResult::CharNameSuccess => SMSG_AUTH_RESPONSE_WorldResult::CharNameSuccess,
            WorldResult::CharNameFailure => SMSG_AUTH_RESPONSE_WorldResult::CharNameFailure,
            WorldResult::CharNameNoName => SMSG_AUTH_RESPONSE_WorldResult::CharNameNoName,
            WorldResult::CharNameTooShort => SMSG_AUTH_RESPONSE_WorldResult::CharNameTooShort,
            WorldResult::CharNameTooLong => SMSG_AUTH_RESPONSE_WorldResult::CharNameTooLong,
            WorldResult::CharNameInvalidCharacter => SMSG_AUTH_RESPONSE_WorldResult::CharNameInvalidCharacter,
            WorldResult::CharNameMixedLanguages => SMSG_AUTH_RESPONSE_WorldResult::CharNameMixedLanguages,
            WorldResult::CharNameProfane => SMSG_AUTH_RESPONSE_WorldResult::CharNameProfane,
            WorldResult::CharNameReserved => SMSG_AUTH_RESPONSE_WorldResult::CharNameReserved,
            WorldResult::CharNameInvalidApostrophe => SMSG_AUTH_RESPONSE_WorldResult::CharNameInvalidApostrophe,
            WorldResult::CharNameMultipleApostrophes => SMSG_AUTH_RESPONSE_WorldResult::CharNameMultipleApostrophes,
            WorldResult::CharNameThreeConsecutive => SMSG_AUTH_RESPONSE_WorldResult::CharNameThreeConsecutive,
            WorldResult::CharNameInvalidSpace => SMSG_AUTH_RESPONSE_WorldResult::CharNameInvalidSpace,
            WorldResult::CharNameConsecutiveSpaces => SMSG_AUTH_RESPONSE_WorldResult::CharNameConsecutiveSpaces,
            WorldResult::CharNameRussianConsecutiveSilentCharacters => SMSG_AUTH_RESPONSE_WorldResult::CharNameRussianConsecutiveSilentCharacters,
            WorldResult::CharNameRussianSilentCharacterAtBeginningOrEnd => SMSG_AUTH_RESPONSE_WorldResult::CharNameRussianSilentCharacterAtBeginningOrEnd,
            WorldResult::CharNameDeclensionDoesntMatchBaseName => SMSG_AUTH_RESPONSE_WorldResult::CharNameDeclensionDoesntMatchBaseName,
        };

        Ok(Self {
            result: result_if,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_AUTH_RESPONSE {}

impl SMSG_AUTH_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        self.result.size() // result: SMSG_AUTH_RESPONSE_WorldResult
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_AUTH_RESPONSE_WorldResult {
    ResponseSuccess,
    ResponseFailure,
    ResponseCancelled,
    ResponseDisconnected,
    ResponseFailedToConnect,
    ResponseConnected,
    ResponseVersionMismatch,
    CstatusConnecting,
    CstatusNegotiatingSecurity,
    CstatusNegotiationComplete,
    CstatusNegotiationFailed,
    CstatusAuthenticating,
    AuthOk {
        billing_flags: BillingPlanFlags,
        billing_rested: u32,
        billing_time: u32,
        expansion: Expansion,
    },
    AuthFailed,
    AuthReject,
    AuthBadServerProof,
    AuthUnavailable,
    AuthSystemError,
    AuthBillingError,
    AuthBillingExpired,
    AuthVersionMismatch,
    AuthUnknownAccount,
    AuthIncorrectPassword,
    AuthSessionExpired,
    AuthServerShuttingDown,
    AuthAlreadyLoggingIn,
    AuthLoginServerNotFound,
    AuthWaitQueue {
        queue_position: u32,
    },
    AuthBanned,
    AuthAlreadyOnline,
    AuthNoTime,
    AuthDbBusy,
    AuthSuspended,
    AuthParentalControl,
    AuthLockedEnforced,
    RealmListInProgress,
    RealmListSuccess,
    RealmListFailed,
    RealmListInvalid,
    RealmListRealmNotFound,
    AccountCreateInProgress,
    AccountCreateSuccess,
    AccountCreateFailed,
    CharListRetrieving,
    CharListRetrieved,
    CharListFailed,
    CharCreateInProgress,
    CharCreateSuccess,
    CharCreateError,
    CharCreateFailed,
    CharCreateNameInUse,
    CharCreateDisabled,
    CharCreatePvpTeamsViolation,
    CharCreateServerLimit,
    CharCreateAccountLimit,
    CharCreateServerQueue,
    CharCreateOnlyExisting,
    CharCreateExpansion,
    CharDeleteInProgress,
    CharDeleteSuccess,
    CharDeleteFailed,
    CharDeleteFailedLockedForTransfer,
    CharDeleteFailedGuildLeader,
    CharDeleteFailedArenaCaptain,
    CharLoginInProgress,
    CharLoginSuccess,
    CharLoginNoWorld,
    CharLoginDuplicateCharacter,
    CharLoginNoInstances,
    CharLoginFailed,
    CharLoginDisabled,
    CharLoginNoCharacter,
    CharLoginLockedForTransfer,
    CharLoginLockedByBilling,
    CharNameSuccess,
    CharNameFailure,
    CharNameNoName,
    CharNameTooShort,
    CharNameTooLong,
    CharNameInvalidCharacter,
    CharNameMixedLanguages,
    CharNameProfane,
    CharNameReserved,
    CharNameInvalidApostrophe,
    CharNameMultipleApostrophes,
    CharNameThreeConsecutive,
    CharNameInvalidSpace,
    CharNameConsecutiveSpaces,
    CharNameRussianConsecutiveSilentCharacters,
    CharNameRussianSilentCharacterAtBeginningOrEnd,
    CharNameDeclensionDoesntMatchBaseName,
}

impl Default for SMSG_AUTH_RESPONSE_WorldResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::ResponseSuccess
    }
}

impl SMSG_AUTH_RESPONSE_WorldResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::ResponseSuccess => 0,
            Self::ResponseFailure => 1,
            Self::ResponseCancelled => 2,
            Self::ResponseDisconnected => 3,
            Self::ResponseFailedToConnect => 4,
            Self::ResponseConnected => 5,
            Self::ResponseVersionMismatch => 6,
            Self::CstatusConnecting => 7,
            Self::CstatusNegotiatingSecurity => 8,
            Self::CstatusNegotiationComplete => 9,
            Self::CstatusNegotiationFailed => 10,
            Self::CstatusAuthenticating => 11,
            Self::AuthOk { .. } => 12,
            Self::AuthFailed => 13,
            Self::AuthReject => 14,
            Self::AuthBadServerProof => 15,
            Self::AuthUnavailable => 16,
            Self::AuthSystemError => 17,
            Self::AuthBillingError => 18,
            Self::AuthBillingExpired => 19,
            Self::AuthVersionMismatch => 20,
            Self::AuthUnknownAccount => 21,
            Self::AuthIncorrectPassword => 22,
            Self::AuthSessionExpired => 23,
            Self::AuthServerShuttingDown => 24,
            Self::AuthAlreadyLoggingIn => 25,
            Self::AuthLoginServerNotFound => 26,
            Self::AuthWaitQueue { .. } => 27,
            Self::AuthBanned => 28,
            Self::AuthAlreadyOnline => 29,
            Self::AuthNoTime => 30,
            Self::AuthDbBusy => 31,
            Self::AuthSuspended => 32,
            Self::AuthParentalControl => 33,
            Self::AuthLockedEnforced => 34,
            Self::RealmListInProgress => 35,
            Self::RealmListSuccess => 36,
            Self::RealmListFailed => 37,
            Self::RealmListInvalid => 38,
            Self::RealmListRealmNotFound => 39,
            Self::AccountCreateInProgress => 40,
            Self::AccountCreateSuccess => 41,
            Self::AccountCreateFailed => 42,
            Self::CharListRetrieving => 43,
            Self::CharListRetrieved => 44,
            Self::CharListFailed => 45,
            Self::CharCreateInProgress => 46,
            Self::CharCreateSuccess => 47,
            Self::CharCreateError => 48,
            Self::CharCreateFailed => 49,
            Self::CharCreateNameInUse => 50,
            Self::CharCreateDisabled => 51,
            Self::CharCreatePvpTeamsViolation => 52,
            Self::CharCreateServerLimit => 53,
            Self::CharCreateAccountLimit => 54,
            Self::CharCreateServerQueue => 55,
            Self::CharCreateOnlyExisting => 56,
            Self::CharCreateExpansion => 57,
            Self::CharDeleteInProgress => 58,
            Self::CharDeleteSuccess => 59,
            Self::CharDeleteFailed => 60,
            Self::CharDeleteFailedLockedForTransfer => 61,
            Self::CharDeleteFailedGuildLeader => 62,
            Self::CharDeleteFailedArenaCaptain => 63,
            Self::CharLoginInProgress => 64,
            Self::CharLoginSuccess => 65,
            Self::CharLoginNoWorld => 66,
            Self::CharLoginDuplicateCharacter => 67,
            Self::CharLoginNoInstances => 68,
            Self::CharLoginFailed => 69,
            Self::CharLoginDisabled => 70,
            Self::CharLoginNoCharacter => 71,
            Self::CharLoginLockedForTransfer => 72,
            Self::CharLoginLockedByBilling => 73,
            Self::CharNameSuccess => 74,
            Self::CharNameFailure => 75,
            Self::CharNameNoName => 76,
            Self::CharNameTooShort => 77,
            Self::CharNameTooLong => 78,
            Self::CharNameInvalidCharacter => 79,
            Self::CharNameMixedLanguages => 80,
            Self::CharNameProfane => 81,
            Self::CharNameReserved => 82,
            Self::CharNameInvalidApostrophe => 83,
            Self::CharNameMultipleApostrophes => 84,
            Self::CharNameThreeConsecutive => 85,
            Self::CharNameInvalidSpace => 86,
            Self::CharNameConsecutiveSpaces => 87,
            Self::CharNameRussianConsecutiveSilentCharacters => 88,
            Self::CharNameRussianSilentCharacterAtBeginningOrEnd => 89,
            Self::CharNameDeclensionDoesntMatchBaseName => 90,
        }
    }

}

impl SMSG_AUTH_RESPONSE_WorldResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::ResponseSuccess => {
                1
            }
            Self::ResponseFailure => {
                1
            }
            Self::ResponseCancelled => {
                1
            }
            Self::ResponseDisconnected => {
                1
            }
            Self::ResponseFailedToConnect => {
                1
            }
            Self::ResponseConnected => {
                1
            }
            Self::ResponseVersionMismatch => {
                1
            }
            Self::CstatusConnecting => {
                1
            }
            Self::CstatusNegotiatingSecurity => {
                1
            }
            Self::CstatusNegotiationComplete => {
                1
            }
            Self::CstatusNegotiationFailed => {
                1
            }
            Self::CstatusAuthenticating => {
                1
            }
            Self::AuthOk {
                billing_flags,
                billing_rested,
                billing_time,
                expansion,
            } => {
                1
                + 1 // billing_flags: BillingPlanFlags
                + 4 // billing_rested: u32
                + 4 // billing_time: u32
                + 1 // expansion: Expansion
            }
            Self::AuthFailed => {
                1
            }
            Self::AuthReject => {
                1
            }
            Self::AuthBadServerProof => {
                1
            }
            Self::AuthUnavailable => {
                1
            }
            Self::AuthSystemError => {
                1
            }
            Self::AuthBillingError => {
                1
            }
            Self::AuthBillingExpired => {
                1
            }
            Self::AuthVersionMismatch => {
                1
            }
            Self::AuthUnknownAccount => {
                1
            }
            Self::AuthIncorrectPassword => {
                1
            }
            Self::AuthSessionExpired => {
                1
            }
            Self::AuthServerShuttingDown => {
                1
            }
            Self::AuthAlreadyLoggingIn => {
                1
            }
            Self::AuthLoginServerNotFound => {
                1
            }
            Self::AuthWaitQueue {
                queue_position,
            } => {
                1
                + 4 // queue_position: u32
            }
            Self::AuthBanned => {
                1
            }
            Self::AuthAlreadyOnline => {
                1
            }
            Self::AuthNoTime => {
                1
            }
            Self::AuthDbBusy => {
                1
            }
            Self::AuthSuspended => {
                1
            }
            Self::AuthParentalControl => {
                1
            }
            Self::AuthLockedEnforced => {
                1
            }
            Self::RealmListInProgress => {
                1
            }
            Self::RealmListSuccess => {
                1
            }
            Self::RealmListFailed => {
                1
            }
            Self::RealmListInvalid => {
                1
            }
            Self::RealmListRealmNotFound => {
                1
            }
            Self::AccountCreateInProgress => {
                1
            }
            Self::AccountCreateSuccess => {
                1
            }
            Self::AccountCreateFailed => {
                1
            }
            Self::CharListRetrieving => {
                1
            }
            Self::CharListRetrieved => {
                1
            }
            Self::CharListFailed => {
                1
            }
            Self::CharCreateInProgress => {
                1
            }
            Self::CharCreateSuccess => {
                1
            }
            Self::CharCreateError => {
                1
            }
            Self::CharCreateFailed => {
                1
            }
            Self::CharCreateNameInUse => {
                1
            }
            Self::CharCreateDisabled => {
                1
            }
            Self::CharCreatePvpTeamsViolation => {
                1
            }
            Self::CharCreateServerLimit => {
                1
            }
            Self::CharCreateAccountLimit => {
                1
            }
            Self::CharCreateServerQueue => {
                1
            }
            Self::CharCreateOnlyExisting => {
                1
            }
            Self::CharCreateExpansion => {
                1
            }
            Self::CharDeleteInProgress => {
                1
            }
            Self::CharDeleteSuccess => {
                1
            }
            Self::CharDeleteFailed => {
                1
            }
            Self::CharDeleteFailedLockedForTransfer => {
                1
            }
            Self::CharDeleteFailedGuildLeader => {
                1
            }
            Self::CharDeleteFailedArenaCaptain => {
                1
            }
            Self::CharLoginInProgress => {
                1
            }
            Self::CharLoginSuccess => {
                1
            }
            Self::CharLoginNoWorld => {
                1
            }
            Self::CharLoginDuplicateCharacter => {
                1
            }
            Self::CharLoginNoInstances => {
                1
            }
            Self::CharLoginFailed => {
                1
            }
            Self::CharLoginDisabled => {
                1
            }
            Self::CharLoginNoCharacter => {
                1
            }
            Self::CharLoginLockedForTransfer => {
                1
            }
            Self::CharLoginLockedByBilling => {
                1
            }
            Self::CharNameSuccess => {
                1
            }
            Self::CharNameFailure => {
                1
            }
            Self::CharNameNoName => {
                1
            }
            Self::CharNameTooShort => {
                1
            }
            Self::CharNameTooLong => {
                1
            }
            Self::CharNameInvalidCharacter => {
                1
            }
            Self::CharNameMixedLanguages => {
                1
            }
            Self::CharNameProfane => {
                1
            }
            Self::CharNameReserved => {
                1
            }
            Self::CharNameInvalidApostrophe => {
                1
            }
            Self::CharNameMultipleApostrophes => {
                1
            }
            Self::CharNameThreeConsecutive => {
                1
            }
            Self::CharNameInvalidSpace => {
                1
            }
            Self::CharNameConsecutiveSpaces => {
                1
            }
            Self::CharNameRussianConsecutiveSilentCharacters => {
                1
            }
            Self::CharNameRussianSilentCharacterAtBeginningOrEnd => {
                1
            }
            Self::CharNameDeclensionDoesntMatchBaseName => {
                1
            }
        }
    }
}

