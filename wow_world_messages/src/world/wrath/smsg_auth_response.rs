use std::io::{Read, Write};

use crate::wrath::{
    BillingPlanFlags, Expansion, WorldResult,
};

/// Response to [`CMSG_AUTH_SESSION`](crate::wrath::CMSG_AUTH_SESSION).
/// Usually followed by [`CMSG_CHAR_ENUM`](crate::vanilla::CMSG_CHAR_ENUM) if login was successful (`AUTH_OK`).
/// vmangos/cmangos/mangoszero all have a variant of this message that contains fields from `AUTH_OK` for `AUTH_WAIT_QUEUE` as well (`https://github.com/vmangos/core/blob/cd896d43712ceafecdbd8f005846d7f676e55b4f/src/game/World.cpp#L322`) but this does not seem to be actually be a real thing.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm:110`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm#L110):
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
///         Bool realm_has_free_character_migration;
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_AUTH_RESPONSE {
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
        realm_has_free_character_migration: bool,
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

impl crate::private::Sealed for SMSG_AUTH_RESPONSE {}
impl SMSG_AUTH_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(1..=11).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // result: WorldResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

        let result_if = match result {
            WorldResult::ResponseSuccess => SMSG_AUTH_RESPONSE::ResponseSuccess,
            WorldResult::ResponseFailure => SMSG_AUTH_RESPONSE::ResponseFailure,
            WorldResult::ResponseCancelled => SMSG_AUTH_RESPONSE::ResponseCancelled,
            WorldResult::ResponseDisconnected => SMSG_AUTH_RESPONSE::ResponseDisconnected,
            WorldResult::ResponseFailedToConnect => SMSG_AUTH_RESPONSE::ResponseFailedToConnect,
            WorldResult::ResponseConnected => SMSG_AUTH_RESPONSE::ResponseConnected,
            WorldResult::ResponseVersionMismatch => SMSG_AUTH_RESPONSE::ResponseVersionMismatch,
            WorldResult::CstatusConnecting => SMSG_AUTH_RESPONSE::CstatusConnecting,
            WorldResult::CstatusNegotiatingSecurity => SMSG_AUTH_RESPONSE::CstatusNegotiatingSecurity,
            WorldResult::CstatusNegotiationComplete => SMSG_AUTH_RESPONSE::CstatusNegotiationComplete,
            WorldResult::CstatusNegotiationFailed => SMSG_AUTH_RESPONSE::CstatusNegotiationFailed,
            WorldResult::CstatusAuthenticating => SMSG_AUTH_RESPONSE::CstatusAuthenticating,
            WorldResult::AuthOk => {
                // billing_time: u32
                let billing_time = crate::util::read_u32_le(&mut r)?;

                // billing_flags: BillingPlanFlags
                let billing_flags = BillingPlanFlags::new(crate::util::read_u8_le(&mut r)?);

                // billing_rested: u32
                let billing_rested = crate::util::read_u32_le(&mut r)?;

                // expansion: Expansion
                let expansion = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_AUTH_RESPONSE::AuthOk {
                    billing_flags,
                    billing_rested,
                    billing_time,
                    expansion,
                }
            }
            WorldResult::AuthFailed => SMSG_AUTH_RESPONSE::AuthFailed,
            WorldResult::AuthReject => SMSG_AUTH_RESPONSE::AuthReject,
            WorldResult::AuthBadServerProof => SMSG_AUTH_RESPONSE::AuthBadServerProof,
            WorldResult::AuthUnavailable => SMSG_AUTH_RESPONSE::AuthUnavailable,
            WorldResult::AuthSystemError => SMSG_AUTH_RESPONSE::AuthSystemError,
            WorldResult::AuthBillingError => SMSG_AUTH_RESPONSE::AuthBillingError,
            WorldResult::AuthBillingExpired => SMSG_AUTH_RESPONSE::AuthBillingExpired,
            WorldResult::AuthVersionMismatch => SMSG_AUTH_RESPONSE::AuthVersionMismatch,
            WorldResult::AuthUnknownAccount => SMSG_AUTH_RESPONSE::AuthUnknownAccount,
            WorldResult::AuthIncorrectPassword => SMSG_AUTH_RESPONSE::AuthIncorrectPassword,
            WorldResult::AuthSessionExpired => SMSG_AUTH_RESPONSE::AuthSessionExpired,
            WorldResult::AuthServerShuttingDown => SMSG_AUTH_RESPONSE::AuthServerShuttingDown,
            WorldResult::AuthAlreadyLoggingIn => SMSG_AUTH_RESPONSE::AuthAlreadyLoggingIn,
            WorldResult::AuthLoginServerNotFound => SMSG_AUTH_RESPONSE::AuthLoginServerNotFound,
            WorldResult::AuthWaitQueue => {
                // queue_position: u32
                let queue_position = crate::util::read_u32_le(&mut r)?;

                // realm_has_free_character_migration: Bool
                let realm_has_free_character_migration = crate::util::read_bool_u8(&mut r)?;

                SMSG_AUTH_RESPONSE::AuthWaitQueue {
                    queue_position,
                    realm_has_free_character_migration,
                }
            }
            WorldResult::AuthBanned => SMSG_AUTH_RESPONSE::AuthBanned,
            WorldResult::AuthAlreadyOnline => SMSG_AUTH_RESPONSE::AuthAlreadyOnline,
            WorldResult::AuthNoTime => SMSG_AUTH_RESPONSE::AuthNoTime,
            WorldResult::AuthDbBusy => SMSG_AUTH_RESPONSE::AuthDbBusy,
            WorldResult::AuthSuspended => SMSG_AUTH_RESPONSE::AuthSuspended,
            WorldResult::AuthParentalControl => SMSG_AUTH_RESPONSE::AuthParentalControl,
            WorldResult::AuthLockedEnforced => SMSG_AUTH_RESPONSE::AuthLockedEnforced,
            WorldResult::RealmListInProgress => SMSG_AUTH_RESPONSE::RealmListInProgress,
            WorldResult::RealmListSuccess => SMSG_AUTH_RESPONSE::RealmListSuccess,
            WorldResult::RealmListFailed => SMSG_AUTH_RESPONSE::RealmListFailed,
            WorldResult::RealmListInvalid => SMSG_AUTH_RESPONSE::RealmListInvalid,
            WorldResult::RealmListRealmNotFound => SMSG_AUTH_RESPONSE::RealmListRealmNotFound,
            WorldResult::AccountCreateInProgress => SMSG_AUTH_RESPONSE::AccountCreateInProgress,
            WorldResult::AccountCreateSuccess => SMSG_AUTH_RESPONSE::AccountCreateSuccess,
            WorldResult::AccountCreateFailed => SMSG_AUTH_RESPONSE::AccountCreateFailed,
            WorldResult::CharListRetrieving => SMSG_AUTH_RESPONSE::CharListRetrieving,
            WorldResult::CharListRetrieved => SMSG_AUTH_RESPONSE::CharListRetrieved,
            WorldResult::CharListFailed => SMSG_AUTH_RESPONSE::CharListFailed,
            WorldResult::CharCreateInProgress => SMSG_AUTH_RESPONSE::CharCreateInProgress,
            WorldResult::CharCreateSuccess => SMSG_AUTH_RESPONSE::CharCreateSuccess,
            WorldResult::CharCreateError => SMSG_AUTH_RESPONSE::CharCreateError,
            WorldResult::CharCreateFailed => SMSG_AUTH_RESPONSE::CharCreateFailed,
            WorldResult::CharCreateNameInUse => SMSG_AUTH_RESPONSE::CharCreateNameInUse,
            WorldResult::CharCreateDisabled => SMSG_AUTH_RESPONSE::CharCreateDisabled,
            WorldResult::CharCreatePvpTeamsViolation => SMSG_AUTH_RESPONSE::CharCreatePvpTeamsViolation,
            WorldResult::CharCreateServerLimit => SMSG_AUTH_RESPONSE::CharCreateServerLimit,
            WorldResult::CharCreateAccountLimit => SMSG_AUTH_RESPONSE::CharCreateAccountLimit,
            WorldResult::CharCreateServerQueue => SMSG_AUTH_RESPONSE::CharCreateServerQueue,
            WorldResult::CharCreateOnlyExisting => SMSG_AUTH_RESPONSE::CharCreateOnlyExisting,
            WorldResult::CharCreateExpansion => SMSG_AUTH_RESPONSE::CharCreateExpansion,
            WorldResult::CharCreateExpansionClass => SMSG_AUTH_RESPONSE::CharCreateExpansionClass,
            WorldResult::CharCreateLevelRequirement => SMSG_AUTH_RESPONSE::CharCreateLevelRequirement,
            WorldResult::CharCreateUniqueClassLimit => SMSG_AUTH_RESPONSE::CharCreateUniqueClassLimit,
            WorldResult::CharCreateCharacterInGuild => SMSG_AUTH_RESPONSE::CharCreateCharacterInGuild,
            WorldResult::CharCreateRestrictedRaceclass => SMSG_AUTH_RESPONSE::CharCreateRestrictedRaceclass,
            WorldResult::CharCreateCharacterChooseRace => SMSG_AUTH_RESPONSE::CharCreateCharacterChooseRace,
            WorldResult::CharCreateCharacterArenaLeader => SMSG_AUTH_RESPONSE::CharCreateCharacterArenaLeader,
            WorldResult::CharCreateCharacterDeleteMail => SMSG_AUTH_RESPONSE::CharCreateCharacterDeleteMail,
            WorldResult::CharCreateCharacterSwapFaction => SMSG_AUTH_RESPONSE::CharCreateCharacterSwapFaction,
            WorldResult::CharCreateCharacterRaceOnly => SMSG_AUTH_RESPONSE::CharCreateCharacterRaceOnly,
            WorldResult::CharCreateCharacterGoldLimit => SMSG_AUTH_RESPONSE::CharCreateCharacterGoldLimit,
            WorldResult::CharCreateForceLogin => SMSG_AUTH_RESPONSE::CharCreateForceLogin,
            WorldResult::CharDeleteInProgress => SMSG_AUTH_RESPONSE::CharDeleteInProgress,
            WorldResult::CharDeleteSuccess => SMSG_AUTH_RESPONSE::CharDeleteSuccess,
            WorldResult::CharDeleteFailed => SMSG_AUTH_RESPONSE::CharDeleteFailed,
            WorldResult::CharDeleteFailedLockedForTransfer => SMSG_AUTH_RESPONSE::CharDeleteFailedLockedForTransfer,
            WorldResult::CharDeleteFailedGuildLeader => SMSG_AUTH_RESPONSE::CharDeleteFailedGuildLeader,
            WorldResult::CharDeleteFailedArenaCaptain => SMSG_AUTH_RESPONSE::CharDeleteFailedArenaCaptain,
            WorldResult::CharLoginInProgress => SMSG_AUTH_RESPONSE::CharLoginInProgress,
            WorldResult::CharLoginSuccess => SMSG_AUTH_RESPONSE::CharLoginSuccess,
            WorldResult::CharLoginNoWorld => SMSG_AUTH_RESPONSE::CharLoginNoWorld,
            WorldResult::CharLoginDuplicateCharacter => SMSG_AUTH_RESPONSE::CharLoginDuplicateCharacter,
            WorldResult::CharLoginNoInstances => SMSG_AUTH_RESPONSE::CharLoginNoInstances,
            WorldResult::CharLoginFailed => SMSG_AUTH_RESPONSE::CharLoginFailed,
            WorldResult::CharLoginDisabled => SMSG_AUTH_RESPONSE::CharLoginDisabled,
            WorldResult::CharLoginNoCharacter => SMSG_AUTH_RESPONSE::CharLoginNoCharacter,
            WorldResult::CharLoginLockedForTransfer => SMSG_AUTH_RESPONSE::CharLoginLockedForTransfer,
            WorldResult::CharLoginLockedByBilling => SMSG_AUTH_RESPONSE::CharLoginLockedByBilling,
            WorldResult::CharLoginLockedByMobileAh => SMSG_AUTH_RESPONSE::CharLoginLockedByMobileAh,
            WorldResult::CharNameSuccess => SMSG_AUTH_RESPONSE::CharNameSuccess,
            WorldResult::CharNameFailure => SMSG_AUTH_RESPONSE::CharNameFailure,
            WorldResult::CharNameNoName => SMSG_AUTH_RESPONSE::CharNameNoName,
            WorldResult::CharNameTooShort => SMSG_AUTH_RESPONSE::CharNameTooShort,
            WorldResult::CharNameTooLong => SMSG_AUTH_RESPONSE::CharNameTooLong,
            WorldResult::CharNameInvalidCharacter => SMSG_AUTH_RESPONSE::CharNameInvalidCharacter,
            WorldResult::CharNameMixedLanguages => SMSG_AUTH_RESPONSE::CharNameMixedLanguages,
            WorldResult::CharNameProfane => SMSG_AUTH_RESPONSE::CharNameProfane,
            WorldResult::CharNameReserved => SMSG_AUTH_RESPONSE::CharNameReserved,
            WorldResult::CharNameInvalidApostrophe => SMSG_AUTH_RESPONSE::CharNameInvalidApostrophe,
            WorldResult::CharNameMultipleApostrophes => SMSG_AUTH_RESPONSE::CharNameMultipleApostrophes,
            WorldResult::CharNameThreeConsecutive => SMSG_AUTH_RESPONSE::CharNameThreeConsecutive,
            WorldResult::CharNameInvalidSpace => SMSG_AUTH_RESPONSE::CharNameInvalidSpace,
            WorldResult::CharNameConsecutiveSpaces => SMSG_AUTH_RESPONSE::CharNameConsecutiveSpaces,
            WorldResult::CharNameRussianConsecutiveSilentCharacters => SMSG_AUTH_RESPONSE::CharNameRussianConsecutiveSilentCharacters,
            WorldResult::CharNameRussianSilentCharacterAtBeginningOrEnd => SMSG_AUTH_RESPONSE::CharNameRussianSilentCharacterAtBeginningOrEnd,
            WorldResult::CharNameDeclensionDoesntMatchBaseName => SMSG_AUTH_RESPONSE::CharNameDeclensionDoesntMatchBaseName,
        };

        Ok(result_if)
    }

}

impl crate::Message for SMSG_AUTH_RESPONSE {
    const OPCODE: u32 = 0x01ee;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_AUTH_RESPONSE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_AUTH_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    result = {};", WorldResult::try_from(self.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self {
            crate::wrath::SMSG_AUTH_RESPONSE::AuthOk {
                billing_flags,
                billing_rested,
                billing_time,
                expansion,
            } => {
                writeln!(s, "    billing_time = {};", billing_time).unwrap();
                writeln!(s, "    billing_flags = {};", billing_flags.as_test_case_value()).unwrap();
                writeln!(s, "    billing_rested = {};", billing_rested).unwrap();
                writeln!(s, "    expansion = {};", expansion.as_test_case_value()).unwrap();
            }
            crate::wrath::SMSG_AUTH_RESPONSE::AuthWaitQueue {
                queue_position,
                realm_has_free_character_migration,
            } => {
                writeln!(s, "    queue_position = {};", queue_position).unwrap();
                writeln!(s, "    realm_has_free_character_migration = {};", if *realm_has_free_character_migration { "TRUE" } else { "FALSE" }).unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 494_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "result", "    ");
        match &self {
            crate::wrath::SMSG_AUTH_RESPONSE::AuthOk {
                billing_flags,
                billing_rested,
                billing_time,
                expansion,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "billing_time", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "billing_flags", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "billing_rested", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "expansion", "    ");
            }
            crate::wrath::SMSG_AUTH_RESPONSE::AuthWaitQueue {
                queue_position,
                realm_has_free_character_migration,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "queue_position", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "realm_has_free_character_migration", "    ");
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
            SMSG_AUTH_RESPONSE::AuthOk {
                billing_flags,
                billing_rested,
                billing_time,
                expansion,
            } => {
                // billing_time: u32
                w.write_all(&billing_time.to_le_bytes())?;

                // billing_flags: BillingPlanFlags
                w.write_all(&(billing_flags.as_int().to_le_bytes()))?;

                // billing_rested: u32
                w.write_all(&billing_rested.to_le_bytes())?;

                // expansion: Expansion
                w.write_all(&(expansion.as_int().to_le_bytes()))?;

            }
            SMSG_AUTH_RESPONSE::AuthWaitQueue {
                queue_position,
                realm_has_free_character_migration,
            } => {
                // queue_position: u32
                w.write_all(&queue_position.to_le_bytes())?;

                // realm_has_free_character_migration: Bool
                w.write_all(u8::from(*realm_has_free_character_migration).to_le_bytes().as_slice())?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(494, "SMSG_AUTH_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_AUTH_RESPONSE {}

impl SMSG_AUTH_RESPONSE {
    pub(crate) const fn size(&self) -> usize {
        (match self {
            Self::AuthOk {
                ..
            } => {
                1
                + 1 // billing_flags: BillingPlanFlags
                + 4 // billing_rested: u32
                + 4 // billing_time: u32
                + 1 // expansion: Expansion
            }
            Self::AuthWaitQueue {
                ..
            } => {
                1
                + 4 // queue_position: u32
                + 1 // realm_has_free_character_migration: Bool
            }
            _ => 1,
        }) // result: SMSG_AUTH_RESPONSE
    }
}

impl Default for SMSG_AUTH_RESPONSE {
    fn default() -> Self {
        // First enumerator without any fields
        Self::ResponseSuccess
    }
}

impl SMSG_AUTH_RESPONSE {
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

impl std::fmt::Display for SMSG_AUTH_RESPONSE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ResponseSuccess => f.write_str("ResponseSuccess"),
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
            Self::AuthOk{ .. } => f.write_str("AuthOk"),
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
            Self::AuthWaitQueue{ .. } => f.write_str("AuthWaitQueue"),
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

