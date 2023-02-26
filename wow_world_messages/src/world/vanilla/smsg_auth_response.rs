use crate::vanilla::WorldResult;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Response to [`CMSG_AUTH_SESSION`](crate::vanilla::CMSG_AUTH_SESSION).
///
/// vmangos/cmangos/mangoszero all have a variant of this message that contains fields from `AUTH_OK` for `AUTH_WAIT_QUEUE` as well (`https://github.com/vmangos/core/blob/cd896d43712ceafecdbd8f005846d7f676e55b4f/src/game/World.cpp#L322`) but this does not seem to be actually be a real thing.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm#L2):
/// ```text
/// smsg SMSG_AUTH_RESPONSE = 0x01EE {
///     WorldResult result;
///     if (result == AUTH_OK) {
///         u32 billing_time;
///         u8 billing_flags;
///         u32 billing_rested;
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

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // result: WorldResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        match &self.result {
            SMSG_AUTH_RESPONSE_WorldResult::ResponseSuccess => {}
            SMSG_AUTH_RESPONSE_WorldResult::ResponseFailure => {}
            SMSG_AUTH_RESPONSE_WorldResult::ResponseCancelled => {}
            SMSG_AUTH_RESPONSE_WorldResult::ResponseDisconnected => {}
            SMSG_AUTH_RESPONSE_WorldResult::ResponseFailedToConnect => {}
            SMSG_AUTH_RESPONSE_WorldResult::ResponseConnected => {}
            SMSG_AUTH_RESPONSE_WorldResult::ResponseVersionMismatch => {}
            SMSG_AUTH_RESPONSE_WorldResult::CstatusConnecting => {}
            SMSG_AUTH_RESPONSE_WorldResult::CstatusNegotiatingSecurity => {}
            SMSG_AUTH_RESPONSE_WorldResult::CstatusNegotiationComplete => {}
            SMSG_AUTH_RESPONSE_WorldResult::CstatusNegotiationFailed => {}
            SMSG_AUTH_RESPONSE_WorldResult::CstatusAuthenticating => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthOk {
                billing_flags,
                billing_rested,
                billing_time,
            } => {
                // billing_time: u32
                w.write_all(&billing_time.to_le_bytes())?;

                // billing_flags: u8
                w.write_all(&billing_flags.to_le_bytes())?;

                // billing_rested: u32
                w.write_all(&billing_rested.to_le_bytes())?;

            }
            SMSG_AUTH_RESPONSE_WorldResult::AuthFailed => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthReject => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthBadServerProof => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthUnavailable => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthSystemError => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthBillingError => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthBillingExpired => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthVersionMismatch => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthUnknownAccount => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthIncorrectPassword => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthSessionExpired => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthServerShuttingDown => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthAlreadyLoggingIn => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthLoginServerNotFound => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthWaitQueue {
                queue_position,
            } => {
                // queue_position: u32
                w.write_all(&queue_position.to_le_bytes())?;

            }
            SMSG_AUTH_RESPONSE_WorldResult::AuthBanned => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthAlreadyOnline => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthNoTime => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthDbBusy => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthSuspended => {}
            SMSG_AUTH_RESPONSE_WorldResult::AuthParentalControl => {}
            SMSG_AUTH_RESPONSE_WorldResult::RealmListInProgress => {}
            SMSG_AUTH_RESPONSE_WorldResult::RealmListSuccess => {}
            SMSG_AUTH_RESPONSE_WorldResult::RealmListFailed => {}
            SMSG_AUTH_RESPONSE_WorldResult::RealmListInvalid => {}
            SMSG_AUTH_RESPONSE_WorldResult::RealmListRealmNotFound => {}
            SMSG_AUTH_RESPONSE_WorldResult::AccountCreateInProgress => {}
            SMSG_AUTH_RESPONSE_WorldResult::AccountCreateSuccess => {}
            SMSG_AUTH_RESPONSE_WorldResult::AccountCreateFailed => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharListRetrieving => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharListRetrieved => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharListFailed => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharCreateInProgress => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharCreateSuccess => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharCreateError => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharCreateFailed => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharCreateNameInUse => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharCreateDisabled => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharCreatePvpTeamsViolation => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharCreateServerLimit => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharCreateAccountLimit => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharCreateServerQueue => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharCreateOnlyExisting => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharDeleteInProgress => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharDeleteSuccess => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharDeleteFailed => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharDeleteFailedLockedForTransfer => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharLoginInProgress => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharLoginSuccess => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharLoginNoWorld => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharLoginDuplicateCharacter => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharLoginNoInstances => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharLoginFailed => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharLoginDisabled => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharLoginNoCharacter => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharLoginLockedForTransfer => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharNameNoName => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharNameTooShort => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharNameTooLong => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharNameOnlyLetters => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharNameMixedLanguages => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharNameProfane => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharNameReserved => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharNameInvalidApostrophe => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharNameMultipleApostrophes => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharNameThreeConsecutive => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharNameInvalidSpace => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharNameSuccess => {}
            SMSG_AUTH_RESPONSE_WorldResult::CharNameFailure => {}
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=10).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01EE, size: body_size as u32 });
        }

        // result: WorldResult
        let result: WorldResult = crate::util::read_u8_le(r)?.try_into()?;

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
                let billing_time = crate::util::read_u32_le(r)?;

                // billing_flags: u8
                let billing_flags = crate::util::read_u8_le(r)?;

                // billing_rested: u32
                let billing_rested = crate::util::read_u32_le(r)?;

                SMSG_AUTH_RESPONSE_WorldResult::AuthOk {
                    billing_flags,
                    billing_rested,
                    billing_time,
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
                let queue_position = crate::util::read_u32_le(r)?;

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
            WorldResult::CharDeleteInProgress => SMSG_AUTH_RESPONSE_WorldResult::CharDeleteInProgress,
            WorldResult::CharDeleteSuccess => SMSG_AUTH_RESPONSE_WorldResult::CharDeleteSuccess,
            WorldResult::CharDeleteFailed => SMSG_AUTH_RESPONSE_WorldResult::CharDeleteFailed,
            WorldResult::CharDeleteFailedLockedForTransfer => SMSG_AUTH_RESPONSE_WorldResult::CharDeleteFailedLockedForTransfer,
            WorldResult::CharLoginInProgress => SMSG_AUTH_RESPONSE_WorldResult::CharLoginInProgress,
            WorldResult::CharLoginSuccess => SMSG_AUTH_RESPONSE_WorldResult::CharLoginSuccess,
            WorldResult::CharLoginNoWorld => SMSG_AUTH_RESPONSE_WorldResult::CharLoginNoWorld,
            WorldResult::CharLoginDuplicateCharacter => SMSG_AUTH_RESPONSE_WorldResult::CharLoginDuplicateCharacter,
            WorldResult::CharLoginNoInstances => SMSG_AUTH_RESPONSE_WorldResult::CharLoginNoInstances,
            WorldResult::CharLoginFailed => SMSG_AUTH_RESPONSE_WorldResult::CharLoginFailed,
            WorldResult::CharLoginDisabled => SMSG_AUTH_RESPONSE_WorldResult::CharLoginDisabled,
            WorldResult::CharLoginNoCharacter => SMSG_AUTH_RESPONSE_WorldResult::CharLoginNoCharacter,
            WorldResult::CharLoginLockedForTransfer => SMSG_AUTH_RESPONSE_WorldResult::CharLoginLockedForTransfer,
            WorldResult::CharNameNoName => SMSG_AUTH_RESPONSE_WorldResult::CharNameNoName,
            WorldResult::CharNameTooShort => SMSG_AUTH_RESPONSE_WorldResult::CharNameTooShort,
            WorldResult::CharNameTooLong => SMSG_AUTH_RESPONSE_WorldResult::CharNameTooLong,
            WorldResult::CharNameOnlyLetters => SMSG_AUTH_RESPONSE_WorldResult::CharNameOnlyLetters,
            WorldResult::CharNameMixedLanguages => SMSG_AUTH_RESPONSE_WorldResult::CharNameMixedLanguages,
            WorldResult::CharNameProfane => SMSG_AUTH_RESPONSE_WorldResult::CharNameProfane,
            WorldResult::CharNameReserved => SMSG_AUTH_RESPONSE_WorldResult::CharNameReserved,
            WorldResult::CharNameInvalidApostrophe => SMSG_AUTH_RESPONSE_WorldResult::CharNameInvalidApostrophe,
            WorldResult::CharNameMultipleApostrophes => SMSG_AUTH_RESPONSE_WorldResult::CharNameMultipleApostrophes,
            WorldResult::CharNameThreeConsecutive => SMSG_AUTH_RESPONSE_WorldResult::CharNameThreeConsecutive,
            WorldResult::CharNameInvalidSpace => SMSG_AUTH_RESPONSE_WorldResult::CharNameInvalidSpace,
            WorldResult::CharNameSuccess => SMSG_AUTH_RESPONSE_WorldResult::CharNameSuccess,
            WorldResult::CharNameFailure => SMSG_AUTH_RESPONSE_WorldResult::CharNameFailure,
        };

        Ok(Self {
            result: result_if,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_AUTH_RESPONSE {}

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
        billing_flags: u8,
        billing_rested: u32,
        billing_time: u32,
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
    CharDeleteInProgress,
    CharDeleteSuccess,
    CharDeleteFailed,
    CharDeleteFailedLockedForTransfer,
    CharLoginInProgress,
    CharLoginSuccess,
    CharLoginNoWorld,
    CharLoginDuplicateCharacter,
    CharLoginNoInstances,
    CharLoginFailed,
    CharLoginDisabled,
    CharLoginNoCharacter,
    CharLoginLockedForTransfer,
    CharNameNoName,
    CharNameTooShort,
    CharNameTooLong,
    CharNameOnlyLetters,
    CharNameMixedLanguages,
    CharNameProfane,
    CharNameReserved,
    CharNameInvalidApostrophe,
    CharNameMultipleApostrophes,
    CharNameThreeConsecutive,
    CharNameInvalidSpace,
    CharNameSuccess,
    CharNameFailure,
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
            Self::RealmListInProgress => 34,
            Self::RealmListSuccess => 35,
            Self::RealmListFailed => 36,
            Self::RealmListInvalid => 37,
            Self::RealmListRealmNotFound => 38,
            Self::AccountCreateInProgress => 39,
            Self::AccountCreateSuccess => 40,
            Self::AccountCreateFailed => 41,
            Self::CharListRetrieving => 42,
            Self::CharListRetrieved => 43,
            Self::CharListFailed => 44,
            Self::CharCreateInProgress => 45,
            Self::CharCreateSuccess => 46,
            Self::CharCreateError => 47,
            Self::CharCreateFailed => 48,
            Self::CharCreateNameInUse => 49,
            Self::CharCreateDisabled => 50,
            Self::CharCreatePvpTeamsViolation => 51,
            Self::CharCreateServerLimit => 52,
            Self::CharCreateAccountLimit => 53,
            Self::CharCreateServerQueue => 54,
            Self::CharCreateOnlyExisting => 55,
            Self::CharDeleteInProgress => 56,
            Self::CharDeleteSuccess => 57,
            Self::CharDeleteFailed => 58,
            Self::CharDeleteFailedLockedForTransfer => 59,
            Self::CharLoginInProgress => 60,
            Self::CharLoginSuccess => 61,
            Self::CharLoginNoWorld => 62,
            Self::CharLoginDuplicateCharacter => 63,
            Self::CharLoginNoInstances => 64,
            Self::CharLoginFailed => 65,
            Self::CharLoginDisabled => 66,
            Self::CharLoginNoCharacter => 67,
            Self::CharLoginLockedForTransfer => 68,
            Self::CharNameNoName => 69,
            Self::CharNameTooShort => 70,
            Self::CharNameTooLong => 71,
            Self::CharNameOnlyLetters => 72,
            Self::CharNameMixedLanguages => 73,
            Self::CharNameProfane => 74,
            Self::CharNameReserved => 75,
            Self::CharNameInvalidApostrophe => 76,
            Self::CharNameMultipleApostrophes => 77,
            Self::CharNameThreeConsecutive => 78,
            Self::CharNameInvalidSpace => 79,
            Self::CharNameSuccess => 80,
            Self::CharNameFailure => 81,
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
            } => {
                1
                + 1 // billing_flags: u8
                + 4 // billing_rested: u32
                + 4 // billing_time: u32
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
            Self::CharNameNoName => {
                1
            }
            Self::CharNameTooShort => {
                1
            }
            Self::CharNameTooLong => {
                1
            }
            Self::CharNameOnlyLetters => {
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
            Self::CharNameSuccess => {
                1
            }
            Self::CharNameFailure => {
                1
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::SMSG_AUTH_RESPONSE;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 5] = [ 0x00, 0x03, 0xEE, 0x01, 0x0D, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm` line 18.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_AUTH_RESPONSE0() {
        let expected = SMSG_AUTH_RESPONSE {
            result: SMSG_AUTH_RESPONSE_WorldResult::AuthFailed,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm` line 18.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_AUTH_RESPONSE0() {
        let expected = SMSG_AUTH_RESPONSE {
            result: SMSG_AUTH_RESPONSE_WorldResult::AuthFailed,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm` line 18.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_AUTH_RESPONSE0() {
        let expected = SMSG_AUTH_RESPONSE {
            result: SMSG_AUTH_RESPONSE_WorldResult::AuthFailed,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 9] = [ 0x00, 0x07, 0xEE, 0x01, 0x1B, 0xEF, 0xBE, 0xAD, 0xDE, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm` line 29.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_AUTH_RESPONSE1() {
        let expected = SMSG_AUTH_RESPONSE {
            result: SMSG_AUTH_RESPONSE_WorldResult::AuthWaitQueue {
                queue_position: 0xDEADBEEF,
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm` line 29.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_AUTH_RESPONSE1() {
        let expected = SMSG_AUTH_RESPONSE {
            result: SMSG_AUTH_RESPONSE_WorldResult::AuthWaitQueue {
                queue_position: 0xDEADBEEF,
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm` line 29.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_AUTH_RESPONSE1() {
        let expected = SMSG_AUTH_RESPONSE {
            result: SMSG_AUTH_RESPONSE_WorldResult::AuthWaitQueue {
                queue_position: 0xDEADBEEF,
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    const RAW2: [u8; 14] = [ 0x00, 0x0C, 0xEE, 0x01, 0x0C, 0xEF, 0xBE, 0xAD, 0xDE,
         0x00, 0x00, 0x00, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm` line 42.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_AUTH_RESPONSE2() {
        let expected = SMSG_AUTH_RESPONSE {
            result: SMSG_AUTH_RESPONSE_WorldResult::AuthOk {
                billing_flags: 0x0,
                billing_rested: 0x0,
                billing_time: 0xDEADBEEF,
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW2)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm` line 42.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_AUTH_RESPONSE2() {
        let expected = SMSG_AUTH_RESPONSE {
            result: SMSG_AUTH_RESPONSE_WorldResult::AuthOk {
                billing_flags: 0x0,
                billing_rested: 0x0,
                billing_time: 0xDEADBEEF,
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm` line 42.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_AUTH_RESPONSE2() {
        let expected = SMSG_AUTH_RESPONSE {
            result: SMSG_AUTH_RESPONSE_WorldResult::AuthOk {
                billing_flags: 0x0,
                billing_rested: 0x0,
                billing_time: 0xDEADBEEF,
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

}

