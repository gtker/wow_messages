use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::WorldResult;

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
            WorldResult::AuthLockedEnforced => SMSG_CHAR_RENAME::AuthLockedEnforced,
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
            WorldResult::CharCreateExpansion => SMSG_CHAR_RENAME::CharCreateExpansion,
            WorldResult::CharDeleteInProgress => SMSG_CHAR_RENAME::CharDeleteInProgress,
            WorldResult::CharDeleteSuccess => SMSG_CHAR_RENAME::CharDeleteSuccess,
            WorldResult::CharDeleteFailed => SMSG_CHAR_RENAME::CharDeleteFailed,
            WorldResult::CharDeleteFailedLockedForTransfer => SMSG_CHAR_RENAME::CharDeleteFailedLockedForTransfer,
            WorldResult::CharDeleteFailedGuildLeader => SMSG_CHAR_RENAME::CharDeleteFailedGuildLeader,
            WorldResult::CharDeleteFailedArenaCaptain => SMSG_CHAR_RENAME::CharDeleteFailedArenaCaptain,
            WorldResult::CharLoginInProgress => SMSG_CHAR_RENAME::CharLoginInProgress,
            WorldResult::CharLoginSuccess => SMSG_CHAR_RENAME::CharLoginSuccess,
            WorldResult::CharLoginNoWorld => SMSG_CHAR_RENAME::CharLoginNoWorld,
            WorldResult::CharLoginDuplicateCharacter => SMSG_CHAR_RENAME::CharLoginDuplicateCharacter,
            WorldResult::CharLoginNoInstances => SMSG_CHAR_RENAME::CharLoginNoInstances,
            WorldResult::CharLoginFailed => SMSG_CHAR_RENAME::CharLoginFailed,
            WorldResult::CharLoginDisabled => SMSG_CHAR_RENAME::CharLoginDisabled,
            WorldResult::CharLoginNoCharacter => SMSG_CHAR_RENAME::CharLoginNoCharacter,
            WorldResult::CharLoginLockedForTransfer => SMSG_CHAR_RENAME::CharLoginLockedForTransfer,
            WorldResult::CharLoginLockedByBilling => SMSG_CHAR_RENAME::CharLoginLockedByBilling,
            WorldResult::CharNameSuccess => SMSG_CHAR_RENAME::CharNameSuccess,
            WorldResult::CharNameFailure => SMSG_CHAR_RENAME::CharNameFailure,
            WorldResult::CharNameNoName => SMSG_CHAR_RENAME::CharNameNoName,
            WorldResult::CharNameTooShort => SMSG_CHAR_RENAME::CharNameTooShort,
            WorldResult::CharNameTooLong => SMSG_CHAR_RENAME::CharNameTooLong,
            WorldResult::CharNameInvalidCharacter => SMSG_CHAR_RENAME::CharNameInvalidCharacter,
            WorldResult::CharNameMixedLanguages => SMSG_CHAR_RENAME::CharNameMixedLanguages,
            WorldResult::CharNameProfane => SMSG_CHAR_RENAME::CharNameProfane,
            WorldResult::CharNameReserved => SMSG_CHAR_RENAME::CharNameReserved,
            WorldResult::CharNameInvalidApostrophe => SMSG_CHAR_RENAME::CharNameInvalidApostrophe,
            WorldResult::CharNameMultipleApostrophes => SMSG_CHAR_RENAME::CharNameMultipleApostrophes,
            WorldResult::CharNameThreeConsecutive => SMSG_CHAR_RENAME::CharNameThreeConsecutive,
            WorldResult::CharNameInvalidSpace => SMSG_CHAR_RENAME::CharNameInvalidSpace,
            WorldResult::CharNameConsecutiveSpaces => SMSG_CHAR_RENAME::CharNameConsecutiveSpaces,
            WorldResult::CharNameRussianConsecutiveSilentCharacters => SMSG_CHAR_RENAME::CharNameRussianConsecutiveSilentCharacters,
            WorldResult::CharNameRussianSilentCharacterAtBeginningOrEnd => SMSG_CHAR_RENAME::CharNameRussianSilentCharacterAtBeginningOrEnd,
            WorldResult::CharNameDeclensionDoesntMatchBaseName => SMSG_CHAR_RENAME::CharNameDeclensionDoesntMatchBaseName,
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

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CHAR_RENAME {{").unwrap();
        // Members
        writeln!(s, "    result = {};", WorldResult::try_from(self.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self {
            crate::tbc::SMSG_CHAR_RENAME::ResponseSuccess {
                character,
                new_name,
            } => {
                writeln!(s, "    character = {};", character.guid()).unwrap();
                writeln!(s, "    new_name = \"{}\";", new_name).unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 712_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "result", "    ");
        match &self {
            crate::tbc::SMSG_CHAR_RENAME::ResponseSuccess {
                character,
                new_name,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "character", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, new_name.len() + 1, "new_name", "    ");
            }
            _ => {}
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
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

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_CHAR_RENAME {}

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
            Self::AuthLockedEnforced => f.write_str("AuthLockedEnforced"),
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
            Self::CharCreateExpansion => f.write_str("CharCreateExpansion"),
            Self::CharDeleteInProgress => f.write_str("CharDeleteInProgress"),
            Self::CharDeleteSuccess => f.write_str("CharDeleteSuccess"),
            Self::CharDeleteFailed => f.write_str("CharDeleteFailed"),
            Self::CharDeleteFailedLockedForTransfer => f.write_str("CharDeleteFailedLockedForTransfer"),
            Self::CharDeleteFailedGuildLeader => f.write_str("CharDeleteFailedGuildLeader"),
            Self::CharDeleteFailedArenaCaptain => f.write_str("CharDeleteFailedArenaCaptain"),
            Self::CharLoginInProgress => f.write_str("CharLoginInProgress"),
            Self::CharLoginSuccess => f.write_str("CharLoginSuccess"),
            Self::CharLoginNoWorld => f.write_str("CharLoginNoWorld"),
            Self::CharLoginDuplicateCharacter => f.write_str("CharLoginDuplicateCharacter"),
            Self::CharLoginNoInstances => f.write_str("CharLoginNoInstances"),
            Self::CharLoginFailed => f.write_str("CharLoginFailed"),
            Self::CharLoginDisabled => f.write_str("CharLoginDisabled"),
            Self::CharLoginNoCharacter => f.write_str("CharLoginNoCharacter"),
            Self::CharLoginLockedForTransfer => f.write_str("CharLoginLockedForTransfer"),
            Self::CharLoginLockedByBilling => f.write_str("CharLoginLockedByBilling"),
            Self::CharNameSuccess => f.write_str("CharNameSuccess"),
            Self::CharNameFailure => f.write_str("CharNameFailure"),
            Self::CharNameNoName => f.write_str("CharNameNoName"),
            Self::CharNameTooShort => f.write_str("CharNameTooShort"),
            Self::CharNameTooLong => f.write_str("CharNameTooLong"),
            Self::CharNameInvalidCharacter => f.write_str("CharNameInvalidCharacter"),
            Self::CharNameMixedLanguages => f.write_str("CharNameMixedLanguages"),
            Self::CharNameProfane => f.write_str("CharNameProfane"),
            Self::CharNameReserved => f.write_str("CharNameReserved"),
            Self::CharNameInvalidApostrophe => f.write_str("CharNameInvalidApostrophe"),
            Self::CharNameMultipleApostrophes => f.write_str("CharNameMultipleApostrophes"),
            Self::CharNameThreeConsecutive => f.write_str("CharNameThreeConsecutive"),
            Self::CharNameInvalidSpace => f.write_str("CharNameInvalidSpace"),
            Self::CharNameConsecutiveSpaces => f.write_str("CharNameConsecutiveSpaces"),
            Self::CharNameRussianConsecutiveSilentCharacters => f.write_str("CharNameRussianConsecutiveSilentCharacters"),
            Self::CharNameRussianSilentCharacterAtBeginningOrEnd => f.write_str("CharNameRussianSilentCharacterAtBeginningOrEnd"),
            Self::CharNameDeclensionDoesntMatchBaseName => f.write_str("CharNameDeclensionDoesntMatchBaseName"),
        }
    }
}

