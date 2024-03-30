use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    Gender, WorldResult,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_customize.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_customize.wowm#L1):
/// ```text
/// smsg SMSG_CHAR_CUSTOMIZE = 0x0474 {
///     WorldResult result;
///     if (result == RESPONSE_SUCCESS) {
///         Guid guid;
///         CString name;
///         Gender gender;
///         u8 skin_color;
///         u8 face;
///         u8 hair_style;
///         u8 hair_color;
///         u8 facial_hair;
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_CHAR_CUSTOMIZE {
    ResponseSuccess {
        face: u8,
        facial_hair: u8,
        gender: Gender,
        guid: Guid,
        hair_color: u8,
        hair_style: u8,
        name: String,
        skin_color: u8,
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
    CharCreateExpansionClass,
    CharCreateLevelRequirement,
    CharCreateUniqueClassLimit,
    CharCreateCharacterInGuild,
    CharCreateRestrictedRaceclass,
    CharCreateCharacterChooseRace,
    CharCreateCharacterArenaLeader,
    CharCreateCharacterDeleteMail,
    CharCreateCharacterSwapFaction,
    CharCreateCharacterRaceOnly,
    CharCreateCharacterGoldLimit,
    CharCreateForceLogin,
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
    CharLoginLockedByMobileAh,
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

impl crate::private::Sealed for SMSG_CHAR_CUSTOMIZE {}
impl SMSG_CHAR_CUSTOMIZE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(1..=271).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // result: WorldResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        let result_if = match result {
            WorldResult::ResponseSuccess => {
                // guid: Guid
                let guid = crate::util::read_guid(&mut r)?;

                // name: CString
                let name = {
                    let name = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(name)?
                };

                // gender: Gender
                let gender = crate::util::read_u8_le(&mut r)?.try_into()?;

                // skin_color: u8
                let skin_color = crate::util::read_u8_le(&mut r)?;

                // face: u8
                let face = crate::util::read_u8_le(&mut r)?;

                // hair_style: u8
                let hair_style = crate::util::read_u8_le(&mut r)?;

                // hair_color: u8
                let hair_color = crate::util::read_u8_le(&mut r)?;

                // facial_hair: u8
                let facial_hair = crate::util::read_u8_le(&mut r)?;

                SMSG_CHAR_CUSTOMIZE::ResponseSuccess {
                    face,
                    facial_hair,
                    gender,
                    guid,
                    hair_color,
                    hair_style,
                    name,
                    skin_color,
                }
            }
            WorldResult::ResponseFailure => SMSG_CHAR_CUSTOMIZE::ResponseFailure,
            WorldResult::ResponseCancelled => SMSG_CHAR_CUSTOMIZE::ResponseCancelled,
            WorldResult::ResponseDisconnected => SMSG_CHAR_CUSTOMIZE::ResponseDisconnected,
            WorldResult::ResponseFailedToConnect => SMSG_CHAR_CUSTOMIZE::ResponseFailedToConnect,
            WorldResult::ResponseConnected => SMSG_CHAR_CUSTOMIZE::ResponseConnected,
            WorldResult::ResponseVersionMismatch => SMSG_CHAR_CUSTOMIZE::ResponseVersionMismatch,
            WorldResult::CstatusConnecting => SMSG_CHAR_CUSTOMIZE::CstatusConnecting,
            WorldResult::CstatusNegotiatingSecurity => SMSG_CHAR_CUSTOMIZE::CstatusNegotiatingSecurity,
            WorldResult::CstatusNegotiationComplete => SMSG_CHAR_CUSTOMIZE::CstatusNegotiationComplete,
            WorldResult::CstatusNegotiationFailed => SMSG_CHAR_CUSTOMIZE::CstatusNegotiationFailed,
            WorldResult::CstatusAuthenticating => SMSG_CHAR_CUSTOMIZE::CstatusAuthenticating,
            WorldResult::AuthOk => SMSG_CHAR_CUSTOMIZE::AuthOk,
            WorldResult::AuthFailed => SMSG_CHAR_CUSTOMIZE::AuthFailed,
            WorldResult::AuthReject => SMSG_CHAR_CUSTOMIZE::AuthReject,
            WorldResult::AuthBadServerProof => SMSG_CHAR_CUSTOMIZE::AuthBadServerProof,
            WorldResult::AuthUnavailable => SMSG_CHAR_CUSTOMIZE::AuthUnavailable,
            WorldResult::AuthSystemError => SMSG_CHAR_CUSTOMIZE::AuthSystemError,
            WorldResult::AuthBillingError => SMSG_CHAR_CUSTOMIZE::AuthBillingError,
            WorldResult::AuthBillingExpired => SMSG_CHAR_CUSTOMIZE::AuthBillingExpired,
            WorldResult::AuthVersionMismatch => SMSG_CHAR_CUSTOMIZE::AuthVersionMismatch,
            WorldResult::AuthUnknownAccount => SMSG_CHAR_CUSTOMIZE::AuthUnknownAccount,
            WorldResult::AuthIncorrectPassword => SMSG_CHAR_CUSTOMIZE::AuthIncorrectPassword,
            WorldResult::AuthSessionExpired => SMSG_CHAR_CUSTOMIZE::AuthSessionExpired,
            WorldResult::AuthServerShuttingDown => SMSG_CHAR_CUSTOMIZE::AuthServerShuttingDown,
            WorldResult::AuthAlreadyLoggingIn => SMSG_CHAR_CUSTOMIZE::AuthAlreadyLoggingIn,
            WorldResult::AuthLoginServerNotFound => SMSG_CHAR_CUSTOMIZE::AuthLoginServerNotFound,
            WorldResult::AuthWaitQueue => SMSG_CHAR_CUSTOMIZE::AuthWaitQueue,
            WorldResult::AuthBanned => SMSG_CHAR_CUSTOMIZE::AuthBanned,
            WorldResult::AuthAlreadyOnline => SMSG_CHAR_CUSTOMIZE::AuthAlreadyOnline,
            WorldResult::AuthNoTime => SMSG_CHAR_CUSTOMIZE::AuthNoTime,
            WorldResult::AuthDbBusy => SMSG_CHAR_CUSTOMIZE::AuthDbBusy,
            WorldResult::AuthSuspended => SMSG_CHAR_CUSTOMIZE::AuthSuspended,
            WorldResult::AuthParentalControl => SMSG_CHAR_CUSTOMIZE::AuthParentalControl,
            WorldResult::AuthLockedEnforced => SMSG_CHAR_CUSTOMIZE::AuthLockedEnforced,
            WorldResult::RealmListInProgress => SMSG_CHAR_CUSTOMIZE::RealmListInProgress,
            WorldResult::RealmListSuccess => SMSG_CHAR_CUSTOMIZE::RealmListSuccess,
            WorldResult::RealmListFailed => SMSG_CHAR_CUSTOMIZE::RealmListFailed,
            WorldResult::RealmListInvalid => SMSG_CHAR_CUSTOMIZE::RealmListInvalid,
            WorldResult::RealmListRealmNotFound => SMSG_CHAR_CUSTOMIZE::RealmListRealmNotFound,
            WorldResult::AccountCreateInProgress => SMSG_CHAR_CUSTOMIZE::AccountCreateInProgress,
            WorldResult::AccountCreateSuccess => SMSG_CHAR_CUSTOMIZE::AccountCreateSuccess,
            WorldResult::AccountCreateFailed => SMSG_CHAR_CUSTOMIZE::AccountCreateFailed,
            WorldResult::CharListRetrieving => SMSG_CHAR_CUSTOMIZE::CharListRetrieving,
            WorldResult::CharListRetrieved => SMSG_CHAR_CUSTOMIZE::CharListRetrieved,
            WorldResult::CharListFailed => SMSG_CHAR_CUSTOMIZE::CharListFailed,
            WorldResult::CharCreateInProgress => SMSG_CHAR_CUSTOMIZE::CharCreateInProgress,
            WorldResult::CharCreateSuccess => SMSG_CHAR_CUSTOMIZE::CharCreateSuccess,
            WorldResult::CharCreateError => SMSG_CHAR_CUSTOMIZE::CharCreateError,
            WorldResult::CharCreateFailed => SMSG_CHAR_CUSTOMIZE::CharCreateFailed,
            WorldResult::CharCreateNameInUse => SMSG_CHAR_CUSTOMIZE::CharCreateNameInUse,
            WorldResult::CharCreateDisabled => SMSG_CHAR_CUSTOMIZE::CharCreateDisabled,
            WorldResult::CharCreatePvpTeamsViolation => SMSG_CHAR_CUSTOMIZE::CharCreatePvpTeamsViolation,
            WorldResult::CharCreateServerLimit => SMSG_CHAR_CUSTOMIZE::CharCreateServerLimit,
            WorldResult::CharCreateAccountLimit => SMSG_CHAR_CUSTOMIZE::CharCreateAccountLimit,
            WorldResult::CharCreateServerQueue => SMSG_CHAR_CUSTOMIZE::CharCreateServerQueue,
            WorldResult::CharCreateOnlyExisting => SMSG_CHAR_CUSTOMIZE::CharCreateOnlyExisting,
            WorldResult::CharCreateExpansion => SMSG_CHAR_CUSTOMIZE::CharCreateExpansion,
            WorldResult::CharCreateExpansionClass => SMSG_CHAR_CUSTOMIZE::CharCreateExpansionClass,
            WorldResult::CharCreateLevelRequirement => SMSG_CHAR_CUSTOMIZE::CharCreateLevelRequirement,
            WorldResult::CharCreateUniqueClassLimit => SMSG_CHAR_CUSTOMIZE::CharCreateUniqueClassLimit,
            WorldResult::CharCreateCharacterInGuild => SMSG_CHAR_CUSTOMIZE::CharCreateCharacterInGuild,
            WorldResult::CharCreateRestrictedRaceclass => SMSG_CHAR_CUSTOMIZE::CharCreateRestrictedRaceclass,
            WorldResult::CharCreateCharacterChooseRace => SMSG_CHAR_CUSTOMIZE::CharCreateCharacterChooseRace,
            WorldResult::CharCreateCharacterArenaLeader => SMSG_CHAR_CUSTOMIZE::CharCreateCharacterArenaLeader,
            WorldResult::CharCreateCharacterDeleteMail => SMSG_CHAR_CUSTOMIZE::CharCreateCharacterDeleteMail,
            WorldResult::CharCreateCharacterSwapFaction => SMSG_CHAR_CUSTOMIZE::CharCreateCharacterSwapFaction,
            WorldResult::CharCreateCharacterRaceOnly => SMSG_CHAR_CUSTOMIZE::CharCreateCharacterRaceOnly,
            WorldResult::CharCreateCharacterGoldLimit => SMSG_CHAR_CUSTOMIZE::CharCreateCharacterGoldLimit,
            WorldResult::CharCreateForceLogin => SMSG_CHAR_CUSTOMIZE::CharCreateForceLogin,
            WorldResult::CharDeleteInProgress => SMSG_CHAR_CUSTOMIZE::CharDeleteInProgress,
            WorldResult::CharDeleteSuccess => SMSG_CHAR_CUSTOMIZE::CharDeleteSuccess,
            WorldResult::CharDeleteFailed => SMSG_CHAR_CUSTOMIZE::CharDeleteFailed,
            WorldResult::CharDeleteFailedLockedForTransfer => SMSG_CHAR_CUSTOMIZE::CharDeleteFailedLockedForTransfer,
            WorldResult::CharDeleteFailedGuildLeader => SMSG_CHAR_CUSTOMIZE::CharDeleteFailedGuildLeader,
            WorldResult::CharDeleteFailedArenaCaptain => SMSG_CHAR_CUSTOMIZE::CharDeleteFailedArenaCaptain,
            WorldResult::CharLoginInProgress => SMSG_CHAR_CUSTOMIZE::CharLoginInProgress,
            WorldResult::CharLoginSuccess => SMSG_CHAR_CUSTOMIZE::CharLoginSuccess,
            WorldResult::CharLoginNoWorld => SMSG_CHAR_CUSTOMIZE::CharLoginNoWorld,
            WorldResult::CharLoginDuplicateCharacter => SMSG_CHAR_CUSTOMIZE::CharLoginDuplicateCharacter,
            WorldResult::CharLoginNoInstances => SMSG_CHAR_CUSTOMIZE::CharLoginNoInstances,
            WorldResult::CharLoginFailed => SMSG_CHAR_CUSTOMIZE::CharLoginFailed,
            WorldResult::CharLoginDisabled => SMSG_CHAR_CUSTOMIZE::CharLoginDisabled,
            WorldResult::CharLoginNoCharacter => SMSG_CHAR_CUSTOMIZE::CharLoginNoCharacter,
            WorldResult::CharLoginLockedForTransfer => SMSG_CHAR_CUSTOMIZE::CharLoginLockedForTransfer,
            WorldResult::CharLoginLockedByBilling => SMSG_CHAR_CUSTOMIZE::CharLoginLockedByBilling,
            WorldResult::CharLoginLockedByMobileAh => SMSG_CHAR_CUSTOMIZE::CharLoginLockedByMobileAh,
            WorldResult::CharNameSuccess => SMSG_CHAR_CUSTOMIZE::CharNameSuccess,
            WorldResult::CharNameFailure => SMSG_CHAR_CUSTOMIZE::CharNameFailure,
            WorldResult::CharNameNoName => SMSG_CHAR_CUSTOMIZE::CharNameNoName,
            WorldResult::CharNameTooShort => SMSG_CHAR_CUSTOMIZE::CharNameTooShort,
            WorldResult::CharNameTooLong => SMSG_CHAR_CUSTOMIZE::CharNameTooLong,
            WorldResult::CharNameInvalidCharacter => SMSG_CHAR_CUSTOMIZE::CharNameInvalidCharacter,
            WorldResult::CharNameMixedLanguages => SMSG_CHAR_CUSTOMIZE::CharNameMixedLanguages,
            WorldResult::CharNameProfane => SMSG_CHAR_CUSTOMIZE::CharNameProfane,
            WorldResult::CharNameReserved => SMSG_CHAR_CUSTOMIZE::CharNameReserved,
            WorldResult::CharNameInvalidApostrophe => SMSG_CHAR_CUSTOMIZE::CharNameInvalidApostrophe,
            WorldResult::CharNameMultipleApostrophes => SMSG_CHAR_CUSTOMIZE::CharNameMultipleApostrophes,
            WorldResult::CharNameThreeConsecutive => SMSG_CHAR_CUSTOMIZE::CharNameThreeConsecutive,
            WorldResult::CharNameInvalidSpace => SMSG_CHAR_CUSTOMIZE::CharNameInvalidSpace,
            WorldResult::CharNameConsecutiveSpaces => SMSG_CHAR_CUSTOMIZE::CharNameConsecutiveSpaces,
            WorldResult::CharNameRussianConsecutiveSilentCharacters => SMSG_CHAR_CUSTOMIZE::CharNameRussianConsecutiveSilentCharacters,
            WorldResult::CharNameRussianSilentCharacterAtBeginningOrEnd => SMSG_CHAR_CUSTOMIZE::CharNameRussianSilentCharacterAtBeginningOrEnd,
            WorldResult::CharNameDeclensionDoesntMatchBaseName => SMSG_CHAR_CUSTOMIZE::CharNameDeclensionDoesntMatchBaseName,
        };

        Ok(result_if)
    }

}

impl crate::Message for SMSG_CHAR_CUSTOMIZE {
    const OPCODE: u32 = 0x0474;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_CHAR_CUSTOMIZE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CHAR_CUSTOMIZE {{").unwrap();
        // Members
        writeln!(s, "    result = {};", WorldResult::try_from(self.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self {
            crate::wrath::SMSG_CHAR_CUSTOMIZE::ResponseSuccess {
                face,
                facial_hair,
                gender,
                guid,
                hair_color,
                hair_style,
                name,
                skin_color,
            } => {
                writeln!(s, "    guid = {};", guid.guid()).unwrap();
                writeln!(s, "    name = \"{}\";", name).unwrap();
                writeln!(s, "    gender = {};", gender.as_test_case_value()).unwrap();
                writeln!(s, "    skin_color = {};", skin_color).unwrap();
                writeln!(s, "    face = {};", face).unwrap();
                writeln!(s, "    hair_style = {};", hair_style).unwrap();
                writeln!(s, "    hair_color = {};", hair_color).unwrap();
                writeln!(s, "    facial_hair = {};", facial_hair).unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1140_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "result", "    ");
        match &self {
            crate::wrath::SMSG_CHAR_CUSTOMIZE::ResponseSuccess {
                face,
                facial_hair,
                gender,
                guid,
                hair_color,
                hair_style,
                name,
                skin_color,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, name.len() + 1, "name", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "gender", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "skin_color", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "face", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "hair_style", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "hair_color", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "facial_hair", "    ");
            }
            _ => {}
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
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
            SMSG_CHAR_CUSTOMIZE::ResponseSuccess {
                face,
                facial_hair,
                gender,
                guid,
                hair_color,
                hair_style,
                name,
                skin_color,
            } => {
                // guid: Guid
                w.write_all(&guid.guid().to_le_bytes())?;

                // name: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(name.as_bytes().iter().next_back(), Some(&0_u8), "String `name` must not be null-terminated.");
                w.write_all(name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // gender: Gender
                w.write_all(&(gender.as_int().to_le_bytes()))?;

                // skin_color: u8
                w.write_all(&skin_color.to_le_bytes())?;

                // face: u8
                w.write_all(&face.to_le_bytes())?;

                // hair_style: u8
                w.write_all(&hair_style.to_le_bytes())?;

                // hair_color: u8
                w.write_all(&hair_color.to_le_bytes())?;

                // facial_hair: u8
                w.write_all(&facial_hair.to_le_bytes())?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1140, "SMSG_CHAR_CUSTOMIZE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CHAR_CUSTOMIZE {}

impl SMSG_CHAR_CUSTOMIZE {
    pub(crate) fn size(&self) -> usize {
        (match self {
            Self::ResponseSuccess {
                name,
                ..
            } => {
                1
                + 1 // face: u8
                + 1 // facial_hair: u8
                + 1 // gender: Gender
                + 8 // guid: Guid
                + 1 // hair_color: u8
                + 1 // hair_style: u8
                + name.len() + 1 // name: CString
                + 1 // skin_color: u8
            }
            _ => 1,
        }) // result: SMSG_CHAR_CUSTOMIZE
    }
}

impl Default for SMSG_CHAR_CUSTOMIZE {
    fn default() -> Self {
        // First enumerator without any fields
        Self::ResponseFailure
    }
}

impl SMSG_CHAR_CUSTOMIZE {
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
            Self::CharCreateExpansionClass => 58,
            Self::CharCreateLevelRequirement => 59,
            Self::CharCreateUniqueClassLimit => 60,
            Self::CharCreateCharacterInGuild => 61,
            Self::CharCreateRestrictedRaceclass => 62,
            Self::CharCreateCharacterChooseRace => 63,
            Self::CharCreateCharacterArenaLeader => 64,
            Self::CharCreateCharacterDeleteMail => 65,
            Self::CharCreateCharacterSwapFaction => 66,
            Self::CharCreateCharacterRaceOnly => 67,
            Self::CharCreateCharacterGoldLimit => 68,
            Self::CharCreateForceLogin => 69,
            Self::CharDeleteInProgress => 70,
            Self::CharDeleteSuccess => 71,
            Self::CharDeleteFailed => 72,
            Self::CharDeleteFailedLockedForTransfer => 73,
            Self::CharDeleteFailedGuildLeader => 74,
            Self::CharDeleteFailedArenaCaptain => 75,
            Self::CharLoginInProgress => 76,
            Self::CharLoginSuccess => 77,
            Self::CharLoginNoWorld => 78,
            Self::CharLoginDuplicateCharacter => 79,
            Self::CharLoginNoInstances => 80,
            Self::CharLoginFailed => 81,
            Self::CharLoginDisabled => 82,
            Self::CharLoginNoCharacter => 83,
            Self::CharLoginLockedForTransfer => 84,
            Self::CharLoginLockedByBilling => 85,
            Self::CharLoginLockedByMobileAh => 86,
            Self::CharNameSuccess => 87,
            Self::CharNameFailure => 88,
            Self::CharNameNoName => 89,
            Self::CharNameTooShort => 90,
            Self::CharNameTooLong => 91,
            Self::CharNameInvalidCharacter => 92,
            Self::CharNameMixedLanguages => 93,
            Self::CharNameProfane => 94,
            Self::CharNameReserved => 95,
            Self::CharNameInvalidApostrophe => 96,
            Self::CharNameMultipleApostrophes => 97,
            Self::CharNameThreeConsecutive => 98,
            Self::CharNameInvalidSpace => 99,
            Self::CharNameConsecutiveSpaces => 100,
            Self::CharNameRussianConsecutiveSilentCharacters => 101,
            Self::CharNameRussianSilentCharacterAtBeginningOrEnd => 102,
            Self::CharNameDeclensionDoesntMatchBaseName => 103,
        }
    }

}

impl std::fmt::Display for SMSG_CHAR_CUSTOMIZE {
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
            Self::CharCreateExpansionClass => f.write_str("CharCreateExpansionClass"),
            Self::CharCreateLevelRequirement => f.write_str("CharCreateLevelRequirement"),
            Self::CharCreateUniqueClassLimit => f.write_str("CharCreateUniqueClassLimit"),
            Self::CharCreateCharacterInGuild => f.write_str("CharCreateCharacterInGuild"),
            Self::CharCreateRestrictedRaceclass => f.write_str("CharCreateRestrictedRaceclass"),
            Self::CharCreateCharacterChooseRace => f.write_str("CharCreateCharacterChooseRace"),
            Self::CharCreateCharacterArenaLeader => f.write_str("CharCreateCharacterArenaLeader"),
            Self::CharCreateCharacterDeleteMail => f.write_str("CharCreateCharacterDeleteMail"),
            Self::CharCreateCharacterSwapFaction => f.write_str("CharCreateCharacterSwapFaction"),
            Self::CharCreateCharacterRaceOnly => f.write_str("CharCreateCharacterRaceOnly"),
            Self::CharCreateCharacterGoldLimit => f.write_str("CharCreateCharacterGoldLimit"),
            Self::CharCreateForceLogin => f.write_str("CharCreateForceLogin"),
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
            Self::CharLoginLockedByMobileAh => f.write_str("CharLoginLockedByMobileAh"),
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

