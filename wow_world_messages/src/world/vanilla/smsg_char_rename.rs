use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::WorldResult;

/// Response to [`CMSG_CHAR_RENAME`](crate::vanilla::CMSG_CHAR_RENAME).
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm#L2):
/// ```text
/// smsg SMSG_CHAR_RENAME = 0x02C8 {
///     WorldResult result;
///     if (result == RESPONSE_SUCCESS) {
///         Guid character;
///         CString new_name;
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_CHAR_RENAME {
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

impl crate::private::Sealed for SMSG_CHAR_RENAME {}
impl SMSG_CHAR_RENAME {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(1..=265).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // result: WorldResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        let result_if = match result {
            WorldResult::ResponseSuccess => {
                // character: Guid
                let character = crate::util::read_guid(&mut r)?;

                // new_name: CString
                let new_name = {
                    let new_name = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(new_name)?
                };

                SMSG_CHAR_RENAME::ResponseSuccess {
                    character,
                    new_name,
                }
            }
            WorldResult::ResponseFailure => SMSG_CHAR_RENAME::ResponseFailure,
            WorldResult::ResponseCancelled => SMSG_CHAR_RENAME::ResponseCancelled,
            WorldResult::ResponseDisconnected => SMSG_CHAR_RENAME::ResponseDisconnected,
            WorldResult::ResponseFailedToConnect => SMSG_CHAR_RENAME::ResponseFailedToConnect,
            WorldResult::ResponseConnected => SMSG_CHAR_RENAME::ResponseConnected,
            WorldResult::ResponseVersionMismatch => SMSG_CHAR_RENAME::ResponseVersionMismatch,
            WorldResult::CstatusConnecting => SMSG_CHAR_RENAME::CstatusConnecting,
            WorldResult::CstatusNegotiatingSecurity => SMSG_CHAR_RENAME::CstatusNegotiatingSecurity,
            WorldResult::CstatusNegotiationComplete => SMSG_CHAR_RENAME::CstatusNegotiationComplete,
            WorldResult::CstatusNegotiationFailed => SMSG_CHAR_RENAME::CstatusNegotiationFailed,
            WorldResult::CstatusAuthenticating => SMSG_CHAR_RENAME::CstatusAuthenticating,
            WorldResult::AuthOk => SMSG_CHAR_RENAME::AuthOk,
            WorldResult::AuthFailed => SMSG_CHAR_RENAME::AuthFailed,
            WorldResult::AuthReject => SMSG_CHAR_RENAME::AuthReject,
            WorldResult::AuthBadServerProof => SMSG_CHAR_RENAME::AuthBadServerProof,
            WorldResult::AuthUnavailable => SMSG_CHAR_RENAME::AuthUnavailable,
            WorldResult::AuthSystemError => SMSG_CHAR_RENAME::AuthSystemError,
            WorldResult::AuthBillingError => SMSG_CHAR_RENAME::AuthBillingError,
            WorldResult::AuthBillingExpired => SMSG_CHAR_RENAME::AuthBillingExpired,
            WorldResult::AuthVersionMismatch => SMSG_CHAR_RENAME::AuthVersionMismatch,
            WorldResult::AuthUnknownAccount => SMSG_CHAR_RENAME::AuthUnknownAccount,
            WorldResult::AuthIncorrectPassword => SMSG_CHAR_RENAME::AuthIncorrectPassword,
            WorldResult::AuthSessionExpired => SMSG_CHAR_RENAME::AuthSessionExpired,
            WorldResult::AuthServerShuttingDown => SMSG_CHAR_RENAME::AuthServerShuttingDown,
            WorldResult::AuthAlreadyLoggingIn => SMSG_CHAR_RENAME::AuthAlreadyLoggingIn,
            WorldResult::AuthLoginServerNotFound => SMSG_CHAR_RENAME::AuthLoginServerNotFound,
            WorldResult::AuthWaitQueue => SMSG_CHAR_RENAME::AuthWaitQueue,
            WorldResult::AuthBanned => SMSG_CHAR_RENAME::AuthBanned,
            WorldResult::AuthAlreadyOnline => SMSG_CHAR_RENAME::AuthAlreadyOnline,
            WorldResult::AuthNoTime => SMSG_CHAR_RENAME::AuthNoTime,
            WorldResult::AuthDbBusy => SMSG_CHAR_RENAME::AuthDbBusy,
            WorldResult::AuthSuspended => SMSG_CHAR_RENAME::AuthSuspended,
            WorldResult::AuthParentalControl => SMSG_CHAR_RENAME::AuthParentalControl,
            WorldResult::RealmListInProgress => SMSG_CHAR_RENAME::RealmListInProgress,
            WorldResult::RealmListSuccess => SMSG_CHAR_RENAME::RealmListSuccess,
            WorldResult::RealmListFailed => SMSG_CHAR_RENAME::RealmListFailed,
            WorldResult::RealmListInvalid => SMSG_CHAR_RENAME::RealmListInvalid,
            WorldResult::RealmListRealmNotFound => SMSG_CHAR_RENAME::RealmListRealmNotFound,
            WorldResult::AccountCreateInProgress => SMSG_CHAR_RENAME::AccountCreateInProgress,
            WorldResult::AccountCreateSuccess => SMSG_CHAR_RENAME::AccountCreateSuccess,
            WorldResult::AccountCreateFailed => SMSG_CHAR_RENAME::AccountCreateFailed,
            WorldResult::CharListRetrieving => SMSG_CHAR_RENAME::CharListRetrieving,
            WorldResult::CharListRetrieved => SMSG_CHAR_RENAME::CharListRetrieved,
            WorldResult::CharListFailed => SMSG_CHAR_RENAME::CharListFailed,
            WorldResult::CharCreateInProgress => SMSG_CHAR_RENAME::CharCreateInProgress,
            WorldResult::CharCreateSuccess => SMSG_CHAR_RENAME::CharCreateSuccess,
            WorldResult::CharCreateError => SMSG_CHAR_RENAME::CharCreateError,
            WorldResult::CharCreateFailed => SMSG_CHAR_RENAME::CharCreateFailed,
            WorldResult::CharCreateNameInUse => SMSG_CHAR_RENAME::CharCreateNameInUse,
            WorldResult::CharCreateDisabled => SMSG_CHAR_RENAME::CharCreateDisabled,
            WorldResult::CharCreatePvpTeamsViolation => SMSG_CHAR_RENAME::CharCreatePvpTeamsViolation,
            WorldResult::CharCreateServerLimit => SMSG_CHAR_RENAME::CharCreateServerLimit,
            WorldResult::CharCreateAccountLimit => SMSG_CHAR_RENAME::CharCreateAccountLimit,
            WorldResult::CharCreateServerQueue => SMSG_CHAR_RENAME::CharCreateServerQueue,
            WorldResult::CharCreateOnlyExisting => SMSG_CHAR_RENAME::CharCreateOnlyExisting,
            WorldResult::CharDeleteInProgress => SMSG_CHAR_RENAME::CharDeleteInProgress,
            WorldResult::CharDeleteSuccess => SMSG_CHAR_RENAME::CharDeleteSuccess,
            WorldResult::CharDeleteFailed => SMSG_CHAR_RENAME::CharDeleteFailed,
            WorldResult::CharDeleteFailedLockedForTransfer => SMSG_CHAR_RENAME::CharDeleteFailedLockedForTransfer,
            WorldResult::CharLoginInProgress => SMSG_CHAR_RENAME::CharLoginInProgress,
            WorldResult::CharLoginSuccess => SMSG_CHAR_RENAME::CharLoginSuccess,
            WorldResult::CharLoginNoWorld => SMSG_CHAR_RENAME::CharLoginNoWorld,
            WorldResult::CharLoginDuplicateCharacter => SMSG_CHAR_RENAME::CharLoginDuplicateCharacter,
            WorldResult::CharLoginNoInstances => SMSG_CHAR_RENAME::CharLoginNoInstances,
            WorldResult::CharLoginFailed => SMSG_CHAR_RENAME::CharLoginFailed,
            WorldResult::CharLoginDisabled => SMSG_CHAR_RENAME::CharLoginDisabled,
            WorldResult::CharLoginNoCharacter => SMSG_CHAR_RENAME::CharLoginNoCharacter,
            WorldResult::CharLoginLockedForTransfer => SMSG_CHAR_RENAME::CharLoginLockedForTransfer,
            WorldResult::CharNameNoName => SMSG_CHAR_RENAME::CharNameNoName,
            WorldResult::CharNameTooShort => SMSG_CHAR_RENAME::CharNameTooShort,
            WorldResult::CharNameTooLong => SMSG_CHAR_RENAME::CharNameTooLong,
            WorldResult::CharNameOnlyLetters => SMSG_CHAR_RENAME::CharNameOnlyLetters,
            WorldResult::CharNameMixedLanguages => SMSG_CHAR_RENAME::CharNameMixedLanguages,
            WorldResult::CharNameProfane => SMSG_CHAR_RENAME::CharNameProfane,
            WorldResult::CharNameReserved => SMSG_CHAR_RENAME::CharNameReserved,
            WorldResult::CharNameInvalidApostrophe => SMSG_CHAR_RENAME::CharNameInvalidApostrophe,
            WorldResult::CharNameMultipleApostrophes => SMSG_CHAR_RENAME::CharNameMultipleApostrophes,
            WorldResult::CharNameThreeConsecutive => SMSG_CHAR_RENAME::CharNameThreeConsecutive,
            WorldResult::CharNameInvalidSpace => SMSG_CHAR_RENAME::CharNameInvalidSpace,
            WorldResult::CharNameSuccess => SMSG_CHAR_RENAME::CharNameSuccess,
            WorldResult::CharNameFailure => SMSG_CHAR_RENAME::CharNameFailure,
        };

        Ok(result_if)
    }

}

impl crate::Message for SMSG_CHAR_RENAME {
    const OPCODE: u32 = 0x02c8;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_CHAR_RENAME"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // result: WorldResult
        w.write_all(&(self.as_int().to_le_bytes()))?;

        match &self {
            SMSG_CHAR_RENAME::ResponseSuccess {
                character,
                new_name,
            } => {
                // character: Guid
                w.write_all(&character.guid().to_le_bytes())?;

                // new_name: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(new_name.as_bytes().iter().next_back(), Some(&0_u8), "String `new_name` must not be null-terminated.");
                w.write_all(new_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(712, "SMSG_CHAR_RENAME", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_CHAR_RENAME {}

impl SMSG_CHAR_RENAME {
    pub(crate) fn size(&self) -> usize {
        (match self {
            Self::ResponseSuccess {
                new_name,
                ..
            } => {
                1
                + 8 // character: Guid
                + new_name.len() + 1 // new_name: CString
            }
            _ => 1,
        }) // result: SMSG_CHAR_RENAME
    }
}

impl Default for SMSG_CHAR_RENAME {
    fn default() -> Self {
        // First enumerator without any fields
        Self::ResponseFailure
    }
}

impl SMSG_CHAR_RENAME {
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

impl std::fmt::Display for SMSG_CHAR_RENAME {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ResponseSuccess{ .. } => f.write_str("ResponseSuccess"),
            Self::ResponseFailure => f.write_str("ResponseFailure"),
            Self::ResponseCancelled => f.write_str("ResponseCancelled"),
            Self::ResponseDisconnected => f.write_str("ResponseDisconnected"),
            Self::ResponseFailedToConnect => f.write_str("ResponseFailedToConnect"),
            Self::ResponseConnected => f.write_str("ResponseConnected"),
            Self::ResponseVersionMismatch => f.write_str("ResponseVersionMismatch"),
            Self::CstatusConnecting => f.write_str("CstatusConnecting"),
            Self::CstatusNegotiatingSecurity => f.write_str("CstatusNegotiatingSecurity"),
            Self::CstatusNegotiationComplete => f.write_str("CstatusNegotiationComplete"),
            Self::CstatusNegotiationFailed => f.write_str("CstatusNegotiationFailed"),
            Self::CstatusAuthenticating => f.write_str("CstatusAuthenticating"),
            Self::AuthOk => f.write_str("AuthOk"),
            Self::AuthFailed => f.write_str("AuthFailed"),
            Self::AuthReject => f.write_str("AuthReject"),
            Self::AuthBadServerProof => f.write_str("AuthBadServerProof"),
            Self::AuthUnavailable => f.write_str("AuthUnavailable"),
            Self::AuthSystemError => f.write_str("AuthSystemError"),
            Self::AuthBillingError => f.write_str("AuthBillingError"),
            Self::AuthBillingExpired => f.write_str("AuthBillingExpired"),
            Self::AuthVersionMismatch => f.write_str("AuthVersionMismatch"),
            Self::AuthUnknownAccount => f.write_str("AuthUnknownAccount"),
            Self::AuthIncorrectPassword => f.write_str("AuthIncorrectPassword"),
            Self::AuthSessionExpired => f.write_str("AuthSessionExpired"),
            Self::AuthServerShuttingDown => f.write_str("AuthServerShuttingDown"),
            Self::AuthAlreadyLoggingIn => f.write_str("AuthAlreadyLoggingIn"),
            Self::AuthLoginServerNotFound => f.write_str("AuthLoginServerNotFound"),
            Self::AuthWaitQueue => f.write_str("AuthWaitQueue"),
            Self::AuthBanned => f.write_str("AuthBanned"),
            Self::AuthAlreadyOnline => f.write_str("AuthAlreadyOnline"),
            Self::AuthNoTime => f.write_str("AuthNoTime"),
            Self::AuthDbBusy => f.write_str("AuthDbBusy"),
            Self::AuthSuspended => f.write_str("AuthSuspended"),
            Self::AuthParentalControl => f.write_str("AuthParentalControl"),
            Self::RealmListInProgress => f.write_str("RealmListInProgress"),
            Self::RealmListSuccess => f.write_str("RealmListSuccess"),
            Self::RealmListFailed => f.write_str("RealmListFailed"),
            Self::RealmListInvalid => f.write_str("RealmListInvalid"),
            Self::RealmListRealmNotFound => f.write_str("RealmListRealmNotFound"),
            Self::AccountCreateInProgress => f.write_str("AccountCreateInProgress"),
            Self::AccountCreateSuccess => f.write_str("AccountCreateSuccess"),
            Self::AccountCreateFailed => f.write_str("AccountCreateFailed"),
            Self::CharListRetrieving => f.write_str("CharListRetrieving"),
            Self::CharListRetrieved => f.write_str("CharListRetrieved"),
            Self::CharListFailed => f.write_str("CharListFailed"),
            Self::CharCreateInProgress => f.write_str("CharCreateInProgress"),
            Self::CharCreateSuccess => f.write_str("CharCreateSuccess"),
            Self::CharCreateError => f.write_str("CharCreateError"),
            Self::CharCreateFailed => f.write_str("CharCreateFailed"),
            Self::CharCreateNameInUse => f.write_str("CharCreateNameInUse"),
            Self::CharCreateDisabled => f.write_str("CharCreateDisabled"),
            Self::CharCreatePvpTeamsViolation => f.write_str("CharCreatePvpTeamsViolation"),
            Self::CharCreateServerLimit => f.write_str("CharCreateServerLimit"),
            Self::CharCreateAccountLimit => f.write_str("CharCreateAccountLimit"),
            Self::CharCreateServerQueue => f.write_str("CharCreateServerQueue"),
            Self::CharCreateOnlyExisting => f.write_str("CharCreateOnlyExisting"),
            Self::CharDeleteInProgress => f.write_str("CharDeleteInProgress"),
            Self::CharDeleteSuccess => f.write_str("CharDeleteSuccess"),
            Self::CharDeleteFailed => f.write_str("CharDeleteFailed"),
            Self::CharDeleteFailedLockedForTransfer => f.write_str("CharDeleteFailedLockedForTransfer"),
            Self::CharLoginInProgress => f.write_str("CharLoginInProgress"),
            Self::CharLoginSuccess => f.write_str("CharLoginSuccess"),
            Self::CharLoginNoWorld => f.write_str("CharLoginNoWorld"),
            Self::CharLoginDuplicateCharacter => f.write_str("CharLoginDuplicateCharacter"),
            Self::CharLoginNoInstances => f.write_str("CharLoginNoInstances"),
            Self::CharLoginFailed => f.write_str("CharLoginFailed"),
            Self::CharLoginDisabled => f.write_str("CharLoginDisabled"),
            Self::CharLoginNoCharacter => f.write_str("CharLoginNoCharacter"),
            Self::CharLoginLockedForTransfer => f.write_str("CharLoginLockedForTransfer"),
            Self::CharNameNoName => f.write_str("CharNameNoName"),
            Self::CharNameTooShort => f.write_str("CharNameTooShort"),
            Self::CharNameTooLong => f.write_str("CharNameTooLong"),
            Self::CharNameOnlyLetters => f.write_str("CharNameOnlyLetters"),
            Self::CharNameMixedLanguages => f.write_str("CharNameMixedLanguages"),
            Self::CharNameProfane => f.write_str("CharNameProfane"),
            Self::CharNameReserved => f.write_str("CharNameReserved"),
            Self::CharNameInvalidApostrophe => f.write_str("CharNameInvalidApostrophe"),
            Self::CharNameMultipleApostrophes => f.write_str("CharNameMultipleApostrophes"),
            Self::CharNameThreeConsecutive => f.write_str("CharNameThreeConsecutive"),
            Self::CharNameInvalidSpace => f.write_str("CharNameInvalidSpace"),
            Self::CharNameSuccess => f.write_str("CharNameSuccess"),
            Self::CharNameFailure => f.write_str("CharNameFailure"),
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_CHAR_RENAME;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 5] = [ 0x00, 0x03, 0xC8, 0x02, 0x47, ];

    pub(crate) fn expected0() -> SMSG_CHAR_RENAME {
        SMSG_CHAR_RENAME::CharNameTooLong
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm` line 12.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_char_rename0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_RENAME, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm` line 12.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_char_rename0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_RENAME, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm` line 12.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_char_rename0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_RENAME, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 22] = [ 0x00, 0x14, 0xC8, 0x02, 0x00, 0xEF, 0xBE, 0xAD, 0xDE,
         0x00, 0x00, 0x00, 0x00, 0x44, 0x65, 0x61, 0x64, 0x62, 0x65, 0x65, 0x66,
         0x00, ];

    pub(crate) fn expected1() -> SMSG_CHAR_RENAME {
        SMSG_CHAR_RENAME::ResponseSuccess {
            character: Guid::new(0xDEADBEEF),
            new_name: String::from("Deadbeef"),
        }
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm` line 22.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_char_rename1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_RENAME, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm` line 22.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_char_rename1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_RENAME, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm` line 22.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_char_rename1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_RENAME, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}

