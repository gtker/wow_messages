use std::io::{Read, Write};

use crate::wrath::{
    BillingPlanFlags, Expansion, WorldResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm:105`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm#L105):
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
pub struct SMSG_AUTH_RESPONSE {
    pub result: SMSG_AUTH_RESPONSE_WorldResult,
}

#[cfg(feature = "print-testcase")]
impl SMSG_AUTH_RESPONSE {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_AUTH_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    result = {};", crate::wrath::WorldResult::try_from(self.result.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.result {
            crate::wrath::SMSG_AUTH_RESPONSE_WorldResult::AuthOk {
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
            crate::wrath::SMSG_AUTH_RESPONSE_WorldResult::AuthWaitQueue {
                queue_position,
                realm_has_free_character_migration,
            } => {
                writeln!(s, "    queue_position = {};", queue_position).unwrap();
                writeln!(s, "    realm_has_free_character_migration = {};", if *realm_has_free_character_migration { "TRUE" } else { "FALSE" }).unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 494_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "result");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_AUTH_RESPONSE {}
impl crate::Message for SMSG_AUTH_RESPONSE {
    const OPCODE: u32 = 0x01ee;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // result: WorldResult
        w.write_all(&(self.result.as_int().to_le_bytes()))?;

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
                w.write_all(&(billing_flags.as_int().to_le_bytes()))?;

                // billing_rested: u32
                w.write_all(&billing_rested.to_le_bytes())?;

                // expansion: Expansion
                w.write_all(&(expansion.as_int().to_le_bytes()))?;

            }
            SMSG_AUTH_RESPONSE_WorldResult::AuthWaitQueue {
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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(1..=11).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01EE, size: body_size });
        }

        // result: WorldResult
        let result = crate::util::read_u8_le(&mut r)?.try_into()?;

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
                let expansion = crate::util::read_u8_le(&mut r)?.try_into()?;

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

                // realm_has_free_character_migration: Bool
                let realm_has_free_character_migration = crate::util::read_u8_le(&mut r)? != 0;

                SMSG_AUTH_RESPONSE_WorldResult::AuthWaitQueue {
                    queue_position,
                    realm_has_free_character_migration,
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
            WorldResult::CharCreateExpansionClass => SMSG_AUTH_RESPONSE_WorldResult::CharCreateExpansionClass,
            WorldResult::CharCreateLevelRequirement => SMSG_AUTH_RESPONSE_WorldResult::CharCreateLevelRequirement,
            WorldResult::CharCreateUniqueClassLimit => SMSG_AUTH_RESPONSE_WorldResult::CharCreateUniqueClassLimit,
            WorldResult::CharCreateCharacterInGuild => SMSG_AUTH_RESPONSE_WorldResult::CharCreateCharacterInGuild,
            WorldResult::CharCreateRestrictedRaceclass => SMSG_AUTH_RESPONSE_WorldResult::CharCreateRestrictedRaceclass,
            WorldResult::CharCreateCharacterChooseRace => SMSG_AUTH_RESPONSE_WorldResult::CharCreateCharacterChooseRace,
            WorldResult::CharCreateCharacterArenaLeader => SMSG_AUTH_RESPONSE_WorldResult::CharCreateCharacterArenaLeader,
            WorldResult::CharCreateCharacterDeleteMail => SMSG_AUTH_RESPONSE_WorldResult::CharCreateCharacterDeleteMail,
            WorldResult::CharCreateCharacterSwapFaction => SMSG_AUTH_RESPONSE_WorldResult::CharCreateCharacterSwapFaction,
            WorldResult::CharCreateCharacterRaceOnly => SMSG_AUTH_RESPONSE_WorldResult::CharCreateCharacterRaceOnly,
            WorldResult::CharCreateCharacterGoldLimit => SMSG_AUTH_RESPONSE_WorldResult::CharCreateCharacterGoldLimit,
            WorldResult::CharCreateForceLogin => SMSG_AUTH_RESPONSE_WorldResult::CharCreateForceLogin,
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
            WorldResult::CharLoginLockedByMobileAh => SMSG_AUTH_RESPONSE_WorldResult::CharLoginLockedByMobileAh,
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
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_AUTH_RESPONSE {}

impl SMSG_AUTH_RESPONSE {
    pub(crate) const fn size(&self) -> usize {
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

impl std::fmt::Display for SMSG_AUTH_RESPONSE_WorldResult {
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

impl SMSG_AUTH_RESPONSE_WorldResult {
    pub(crate) const fn size(&self) -> usize {
        match self {
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
        }
    }
}

