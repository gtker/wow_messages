use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::WorldResult;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Response to [`CMSG_CHAR_RENAME`](crate::world::vanilla::CMSG_CHAR_RENAME).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm#L3):
/// ```text
/// smsg SMSG_CHAR_RENAME = 0x02C8 {
///     WorldResult result;
///     if (result == RESPONSE_SUCCESS) {
///         Guid character;
///         CString new_name;
///     }
/// }
/// ```
pub struct SMSG_CHAR_RENAME {
    pub result: SMSG_CHAR_RENAME_WorldResult,
}

impl crate::Message for SMSG_CHAR_RENAME {
    const OPCODE: u32 = 0x02c8;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: WorldResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        match &self.result {
            SMSG_CHAR_RENAME_WorldResult::ResponseSuccess {
                character,
                new_name,
            } => {
                // character: Guid
                w.write_all(&character.guid().to_le_bytes())?;

                // new_name: CString
                w.write_all(new_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_CHAR_RENAME_WorldResult::ResponseFailure => {}
            SMSG_CHAR_RENAME_WorldResult::ResponseCancelled => {}
            SMSG_CHAR_RENAME_WorldResult::ResponseDisconnected => {}
            SMSG_CHAR_RENAME_WorldResult::ResponseFailedToConnect => {}
            SMSG_CHAR_RENAME_WorldResult::ResponseConnected => {}
            SMSG_CHAR_RENAME_WorldResult::ResponseVersionMismatch => {}
            SMSG_CHAR_RENAME_WorldResult::CstatusConnecting => {}
            SMSG_CHAR_RENAME_WorldResult::CstatusNegotiatingSecurity => {}
            SMSG_CHAR_RENAME_WorldResult::CstatusNegotiationComplete => {}
            SMSG_CHAR_RENAME_WorldResult::CstatusNegotiationFailed => {}
            SMSG_CHAR_RENAME_WorldResult::CstatusAuthenticating => {}
            SMSG_CHAR_RENAME_WorldResult::AuthOk => {}
            SMSG_CHAR_RENAME_WorldResult::AuthFailed => {}
            SMSG_CHAR_RENAME_WorldResult::AuthReject => {}
            SMSG_CHAR_RENAME_WorldResult::AuthBadServerProof => {}
            SMSG_CHAR_RENAME_WorldResult::AuthUnavailable => {}
            SMSG_CHAR_RENAME_WorldResult::AuthSystemError => {}
            SMSG_CHAR_RENAME_WorldResult::AuthBillingError => {}
            SMSG_CHAR_RENAME_WorldResult::AuthBillingExpired => {}
            SMSG_CHAR_RENAME_WorldResult::AuthVersionMismatch => {}
            SMSG_CHAR_RENAME_WorldResult::AuthUnknownAccount => {}
            SMSG_CHAR_RENAME_WorldResult::AuthIncorrectPassword => {}
            SMSG_CHAR_RENAME_WorldResult::AuthSessionExpired => {}
            SMSG_CHAR_RENAME_WorldResult::AuthServerShuttingDown => {}
            SMSG_CHAR_RENAME_WorldResult::AuthAlreadyLoggingIn => {}
            SMSG_CHAR_RENAME_WorldResult::AuthLoginServerNotFound => {}
            SMSG_CHAR_RENAME_WorldResult::AuthWaitQueue => {}
            SMSG_CHAR_RENAME_WorldResult::AuthBanned => {}
            SMSG_CHAR_RENAME_WorldResult::AuthAlreadyOnline => {}
            SMSG_CHAR_RENAME_WorldResult::AuthNoTime => {}
            SMSG_CHAR_RENAME_WorldResult::AuthDbBusy => {}
            SMSG_CHAR_RENAME_WorldResult::AuthSuspended => {}
            SMSG_CHAR_RENAME_WorldResult::AuthParentalControl => {}
            SMSG_CHAR_RENAME_WorldResult::RealmListInProgress => {}
            SMSG_CHAR_RENAME_WorldResult::RealmListSuccess => {}
            SMSG_CHAR_RENAME_WorldResult::RealmListFailed => {}
            SMSG_CHAR_RENAME_WorldResult::RealmListInvalid => {}
            SMSG_CHAR_RENAME_WorldResult::RealmListRealmNotFound => {}
            SMSG_CHAR_RENAME_WorldResult::AccountCreateInProgress => {}
            SMSG_CHAR_RENAME_WorldResult::AccountCreateSuccess => {}
            SMSG_CHAR_RENAME_WorldResult::AccountCreateFailed => {}
            SMSG_CHAR_RENAME_WorldResult::CharListRetrieving => {}
            SMSG_CHAR_RENAME_WorldResult::CharListRetrieved => {}
            SMSG_CHAR_RENAME_WorldResult::CharListFailed => {}
            SMSG_CHAR_RENAME_WorldResult::CharCreateInProgress => {}
            SMSG_CHAR_RENAME_WorldResult::CharCreateSuccess => {}
            SMSG_CHAR_RENAME_WorldResult::CharCreateError => {}
            SMSG_CHAR_RENAME_WorldResult::CharCreateFailed => {}
            SMSG_CHAR_RENAME_WorldResult::CharCreateNameInUse => {}
            SMSG_CHAR_RENAME_WorldResult::CharCreateDisabled => {}
            SMSG_CHAR_RENAME_WorldResult::CharCreatePvpTeamsViolation => {}
            SMSG_CHAR_RENAME_WorldResult::CharCreateServerLimit => {}
            SMSG_CHAR_RENAME_WorldResult::CharCreateAccountLimit => {}
            SMSG_CHAR_RENAME_WorldResult::CharCreateServerQueue => {}
            SMSG_CHAR_RENAME_WorldResult::CharCreateOnlyExisting => {}
            SMSG_CHAR_RENAME_WorldResult::CharDeleteInProgress => {}
            SMSG_CHAR_RENAME_WorldResult::CharDeleteSuccess => {}
            SMSG_CHAR_RENAME_WorldResult::CharDeleteFailed => {}
            SMSG_CHAR_RENAME_WorldResult::CharDeleteFailedLockedForTransfer => {}
            SMSG_CHAR_RENAME_WorldResult::CharLoginInProgress => {}
            SMSG_CHAR_RENAME_WorldResult::CharLoginSuccess => {}
            SMSG_CHAR_RENAME_WorldResult::CharLoginNoWorld => {}
            SMSG_CHAR_RENAME_WorldResult::CharLoginDuplicateCharacter => {}
            SMSG_CHAR_RENAME_WorldResult::CharLoginNoInstances => {}
            SMSG_CHAR_RENAME_WorldResult::CharLoginFailed => {}
            SMSG_CHAR_RENAME_WorldResult::CharLoginDisabled => {}
            SMSG_CHAR_RENAME_WorldResult::CharLoginNoCharacter => {}
            SMSG_CHAR_RENAME_WorldResult::CharLoginLockedForTransfer => {}
            SMSG_CHAR_RENAME_WorldResult::CharNameNoName => {}
            SMSG_CHAR_RENAME_WorldResult::CharNameTooShort => {}
            SMSG_CHAR_RENAME_WorldResult::CharNameTooLong => {}
            SMSG_CHAR_RENAME_WorldResult::CharNameOnlyLetters => {}
            SMSG_CHAR_RENAME_WorldResult::CharNameMixedLanguages => {}
            SMSG_CHAR_RENAME_WorldResult::CharNameProfane => {}
            SMSG_CHAR_RENAME_WorldResult::CharNameReserved => {}
            SMSG_CHAR_RENAME_WorldResult::CharNameInvalidApostrophe => {}
            SMSG_CHAR_RENAME_WorldResult::CharNameMultipleApostrophes => {}
            SMSG_CHAR_RENAME_WorldResult::CharNameThreeConsecutive => {}
            SMSG_CHAR_RENAME_WorldResult::CharNameInvalidSpace => {}
            SMSG_CHAR_RENAME_WorldResult::CharNameSuccess => {}
            SMSG_CHAR_RENAME_WorldResult::CharNameFailure => {}
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // result: WorldResult
        let result: WorldResult = crate::util::read_u8_le(r)?.try_into()?;

        let result_if = match result {
            WorldResult::ResponseSuccess => {
                // character: Guid
                let character = Guid::read(r)?;

                // new_name: CString
                let new_name = crate::util::read_c_string_to_vec(r)?;
                let new_name = String::from_utf8(new_name)?;

                SMSG_CHAR_RENAME_WorldResult::ResponseSuccess {
                    character,
                    new_name,
                }
            }
            WorldResult::ResponseFailure => SMSG_CHAR_RENAME_WorldResult::ResponseFailure,
            WorldResult::ResponseCancelled => SMSG_CHAR_RENAME_WorldResult::ResponseCancelled,
            WorldResult::ResponseDisconnected => SMSG_CHAR_RENAME_WorldResult::ResponseDisconnected,
            WorldResult::ResponseFailedToConnect => SMSG_CHAR_RENAME_WorldResult::ResponseFailedToConnect,
            WorldResult::ResponseConnected => SMSG_CHAR_RENAME_WorldResult::ResponseConnected,
            WorldResult::ResponseVersionMismatch => SMSG_CHAR_RENAME_WorldResult::ResponseVersionMismatch,
            WorldResult::CstatusConnecting => SMSG_CHAR_RENAME_WorldResult::CstatusConnecting,
            WorldResult::CstatusNegotiatingSecurity => SMSG_CHAR_RENAME_WorldResult::CstatusNegotiatingSecurity,
            WorldResult::CstatusNegotiationComplete => SMSG_CHAR_RENAME_WorldResult::CstatusNegotiationComplete,
            WorldResult::CstatusNegotiationFailed => SMSG_CHAR_RENAME_WorldResult::CstatusNegotiationFailed,
            WorldResult::CstatusAuthenticating => SMSG_CHAR_RENAME_WorldResult::CstatusAuthenticating,
            WorldResult::AuthOk => SMSG_CHAR_RENAME_WorldResult::AuthOk,
            WorldResult::AuthFailed => SMSG_CHAR_RENAME_WorldResult::AuthFailed,
            WorldResult::AuthReject => SMSG_CHAR_RENAME_WorldResult::AuthReject,
            WorldResult::AuthBadServerProof => SMSG_CHAR_RENAME_WorldResult::AuthBadServerProof,
            WorldResult::AuthUnavailable => SMSG_CHAR_RENAME_WorldResult::AuthUnavailable,
            WorldResult::AuthSystemError => SMSG_CHAR_RENAME_WorldResult::AuthSystemError,
            WorldResult::AuthBillingError => SMSG_CHAR_RENAME_WorldResult::AuthBillingError,
            WorldResult::AuthBillingExpired => SMSG_CHAR_RENAME_WorldResult::AuthBillingExpired,
            WorldResult::AuthVersionMismatch => SMSG_CHAR_RENAME_WorldResult::AuthVersionMismatch,
            WorldResult::AuthUnknownAccount => SMSG_CHAR_RENAME_WorldResult::AuthUnknownAccount,
            WorldResult::AuthIncorrectPassword => SMSG_CHAR_RENAME_WorldResult::AuthIncorrectPassword,
            WorldResult::AuthSessionExpired => SMSG_CHAR_RENAME_WorldResult::AuthSessionExpired,
            WorldResult::AuthServerShuttingDown => SMSG_CHAR_RENAME_WorldResult::AuthServerShuttingDown,
            WorldResult::AuthAlreadyLoggingIn => SMSG_CHAR_RENAME_WorldResult::AuthAlreadyLoggingIn,
            WorldResult::AuthLoginServerNotFound => SMSG_CHAR_RENAME_WorldResult::AuthLoginServerNotFound,
            WorldResult::AuthWaitQueue => SMSG_CHAR_RENAME_WorldResult::AuthWaitQueue,
            WorldResult::AuthBanned => SMSG_CHAR_RENAME_WorldResult::AuthBanned,
            WorldResult::AuthAlreadyOnline => SMSG_CHAR_RENAME_WorldResult::AuthAlreadyOnline,
            WorldResult::AuthNoTime => SMSG_CHAR_RENAME_WorldResult::AuthNoTime,
            WorldResult::AuthDbBusy => SMSG_CHAR_RENAME_WorldResult::AuthDbBusy,
            WorldResult::AuthSuspended => SMSG_CHAR_RENAME_WorldResult::AuthSuspended,
            WorldResult::AuthParentalControl => SMSG_CHAR_RENAME_WorldResult::AuthParentalControl,
            WorldResult::RealmListInProgress => SMSG_CHAR_RENAME_WorldResult::RealmListInProgress,
            WorldResult::RealmListSuccess => SMSG_CHAR_RENAME_WorldResult::RealmListSuccess,
            WorldResult::RealmListFailed => SMSG_CHAR_RENAME_WorldResult::RealmListFailed,
            WorldResult::RealmListInvalid => SMSG_CHAR_RENAME_WorldResult::RealmListInvalid,
            WorldResult::RealmListRealmNotFound => SMSG_CHAR_RENAME_WorldResult::RealmListRealmNotFound,
            WorldResult::AccountCreateInProgress => SMSG_CHAR_RENAME_WorldResult::AccountCreateInProgress,
            WorldResult::AccountCreateSuccess => SMSG_CHAR_RENAME_WorldResult::AccountCreateSuccess,
            WorldResult::AccountCreateFailed => SMSG_CHAR_RENAME_WorldResult::AccountCreateFailed,
            WorldResult::CharListRetrieving => SMSG_CHAR_RENAME_WorldResult::CharListRetrieving,
            WorldResult::CharListRetrieved => SMSG_CHAR_RENAME_WorldResult::CharListRetrieved,
            WorldResult::CharListFailed => SMSG_CHAR_RENAME_WorldResult::CharListFailed,
            WorldResult::CharCreateInProgress => SMSG_CHAR_RENAME_WorldResult::CharCreateInProgress,
            WorldResult::CharCreateSuccess => SMSG_CHAR_RENAME_WorldResult::CharCreateSuccess,
            WorldResult::CharCreateError => SMSG_CHAR_RENAME_WorldResult::CharCreateError,
            WorldResult::CharCreateFailed => SMSG_CHAR_RENAME_WorldResult::CharCreateFailed,
            WorldResult::CharCreateNameInUse => SMSG_CHAR_RENAME_WorldResult::CharCreateNameInUse,
            WorldResult::CharCreateDisabled => SMSG_CHAR_RENAME_WorldResult::CharCreateDisabled,
            WorldResult::CharCreatePvpTeamsViolation => SMSG_CHAR_RENAME_WorldResult::CharCreatePvpTeamsViolation,
            WorldResult::CharCreateServerLimit => SMSG_CHAR_RENAME_WorldResult::CharCreateServerLimit,
            WorldResult::CharCreateAccountLimit => SMSG_CHAR_RENAME_WorldResult::CharCreateAccountLimit,
            WorldResult::CharCreateServerQueue => SMSG_CHAR_RENAME_WorldResult::CharCreateServerQueue,
            WorldResult::CharCreateOnlyExisting => SMSG_CHAR_RENAME_WorldResult::CharCreateOnlyExisting,
            WorldResult::CharDeleteInProgress => SMSG_CHAR_RENAME_WorldResult::CharDeleteInProgress,
            WorldResult::CharDeleteSuccess => SMSG_CHAR_RENAME_WorldResult::CharDeleteSuccess,
            WorldResult::CharDeleteFailed => SMSG_CHAR_RENAME_WorldResult::CharDeleteFailed,
            WorldResult::CharDeleteFailedLockedForTransfer => SMSG_CHAR_RENAME_WorldResult::CharDeleteFailedLockedForTransfer,
            WorldResult::CharLoginInProgress => SMSG_CHAR_RENAME_WorldResult::CharLoginInProgress,
            WorldResult::CharLoginSuccess => SMSG_CHAR_RENAME_WorldResult::CharLoginSuccess,
            WorldResult::CharLoginNoWorld => SMSG_CHAR_RENAME_WorldResult::CharLoginNoWorld,
            WorldResult::CharLoginDuplicateCharacter => SMSG_CHAR_RENAME_WorldResult::CharLoginDuplicateCharacter,
            WorldResult::CharLoginNoInstances => SMSG_CHAR_RENAME_WorldResult::CharLoginNoInstances,
            WorldResult::CharLoginFailed => SMSG_CHAR_RENAME_WorldResult::CharLoginFailed,
            WorldResult::CharLoginDisabled => SMSG_CHAR_RENAME_WorldResult::CharLoginDisabled,
            WorldResult::CharLoginNoCharacter => SMSG_CHAR_RENAME_WorldResult::CharLoginNoCharacter,
            WorldResult::CharLoginLockedForTransfer => SMSG_CHAR_RENAME_WorldResult::CharLoginLockedForTransfer,
            WorldResult::CharNameNoName => SMSG_CHAR_RENAME_WorldResult::CharNameNoName,
            WorldResult::CharNameTooShort => SMSG_CHAR_RENAME_WorldResult::CharNameTooShort,
            WorldResult::CharNameTooLong => SMSG_CHAR_RENAME_WorldResult::CharNameTooLong,
            WorldResult::CharNameOnlyLetters => SMSG_CHAR_RENAME_WorldResult::CharNameOnlyLetters,
            WorldResult::CharNameMixedLanguages => SMSG_CHAR_RENAME_WorldResult::CharNameMixedLanguages,
            WorldResult::CharNameProfane => SMSG_CHAR_RENAME_WorldResult::CharNameProfane,
            WorldResult::CharNameReserved => SMSG_CHAR_RENAME_WorldResult::CharNameReserved,
            WorldResult::CharNameInvalidApostrophe => SMSG_CHAR_RENAME_WorldResult::CharNameInvalidApostrophe,
            WorldResult::CharNameMultipleApostrophes => SMSG_CHAR_RENAME_WorldResult::CharNameMultipleApostrophes,
            WorldResult::CharNameThreeConsecutive => SMSG_CHAR_RENAME_WorldResult::CharNameThreeConsecutive,
            WorldResult::CharNameInvalidSpace => SMSG_CHAR_RENAME_WorldResult::CharNameInvalidSpace,
            WorldResult::CharNameSuccess => SMSG_CHAR_RENAME_WorldResult::CharNameSuccess,
            WorldResult::CharNameFailure => SMSG_CHAR_RENAME_WorldResult::CharNameFailure,
        };

        Ok(Self {
            result: result_if,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_CHAR_RENAME {}

impl SMSG_CHAR_RENAME {
    pub(crate) fn size(&self) -> usize {
        self.result.size() // result: SMSG_CHAR_RENAME_WorldResult
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SMSG_CHAR_RENAME_WorldResult {
    ResponseSuccess {
        character: Guid,
        new_name: String,
    },
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
    AuthOk,
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
    AuthWaitQueue,
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

impl Default for SMSG_CHAR_RENAME_WorldResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::ResponseSuccess {
            character: Default::default(),
            new_name: Default::default(),
        }
    }
}

impl SMSG_CHAR_RENAME_WorldResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::ResponseSuccess { .. } => 0,
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
            Self::AuthOk => 12,
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
            Self::AuthWaitQueue => 27,
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

impl SMSG_CHAR_RENAME_WorldResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::ResponseSuccess {
                character,
                new_name,
            } => {
                1
                + 8 // character: Guid
                + new_name.len() + 1 // new_name: CString
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
            Self::AuthOk => {
                1
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
            Self::AuthWaitQueue => {
                1
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
    use super::SMSG_CHAR_RENAME;
    use crate::world::vanilla::WorldResult;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::world::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 5] = [ 0x00, 0x03, 0xC8, 0x02, 0x47, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm` line 13.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_CHAR_RENAME0() {
        let expected = SMSG_CHAR_RENAME {
            result: SMSG_CHAR_RENAME_WorldResult::CharNameTooLong,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm` line 13.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_CHAR_RENAME0() {
        let expected = SMSG_CHAR_RENAME {
            result: SMSG_CHAR_RENAME_WorldResult::CharNameTooLong,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm` line 13.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_CHAR_RENAME0() {
        let expected = SMSG_CHAR_RENAME {
            result: SMSG_CHAR_RENAME_WorldResult::CharNameTooLong,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 22] = [ 0x00, 0x14, 0xC8, 0x02, 0x00, 0xEF, 0xBE, 0xAD, 0xDE,
         0x00, 0x00, 0x00, 0x00, 0x44, 0x65, 0x61, 0x64, 0x62, 0x65, 0x65, 0x66,
         0x00, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm` line 21.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_CHAR_RENAME1() {
        let expected = SMSG_CHAR_RENAME {
            result: SMSG_CHAR_RENAME_WorldResult::ResponseSuccess {
                character: Guid::new(0xDEADBEEF),
                new_name: String::from("Deadbeef"),
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm` line 21.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_CHAR_RENAME1() {
        let expected = SMSG_CHAR_RENAME {
            result: SMSG_CHAR_RENAME_WorldResult::ResponseSuccess {
                character: Guid::new(0xDEADBEEF),
                new_name: String::from("Deadbeef"),
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm` line 21.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_CHAR_RENAME1() {
        let expected = SMSG_CHAR_RENAME {
            result: SMSG_CHAR_RENAME_WorldResult::ResponseSuccess {
                character: Guid::new(0xDEADBEEF),
                new_name: String::from("Deadbeef"),
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}
