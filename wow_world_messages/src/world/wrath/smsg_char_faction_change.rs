use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    Gender,
    Race,
    WorldResult,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_faction_change.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_faction_change.wowm#L1):
/// ```text
/// smsg SMSG_CHAR_FACTION_CHANGE = 0x04DA {
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
///         Race race;
///     }
/// }
/// ```
pub struct SMSG_CHAR_FACTION_CHANGE {
    pub result: SMSG_CHAR_FACTION_CHANGE_WorldResult,
}

impl crate::Message for SMSG_CHAR_FACTION_CHANGE {
    const OPCODE: u32 = 0x04da;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // result: WorldResult
        w.write_all(&u8::from(self.result.as_int()).to_le_bytes())?;

        match &self.result {
            SMSG_CHAR_FACTION_CHANGE_WorldResult::ResponseSuccess {
                face,
                facial_hair,
                gender,
                guid,
                hair_color,
                hair_style,
                name,
                race,
                skin_color,
            } => {
                // guid: Guid
                w.write_all(&guid.guid().to_le_bytes())?;

                // name: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
                w.write_all(name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // gender: Gender
                w.write_all(&u8::from(gender.as_int()).to_le_bytes())?;

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

                // race: Race
                w.write_all(&u8::from(race.as_int()).to_le_bytes())?;

            }
            SMSG_CHAR_FACTION_CHANGE_WorldResult::ResponseFailure => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::ResponseCancelled => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::ResponseDisconnected => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::ResponseFailedToConnect => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::ResponseConnected => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::ResponseVersionMismatch => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CstatusConnecting => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CstatusNegotiatingSecurity => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CstatusNegotiationComplete => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CstatusNegotiationFailed => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CstatusAuthenticating => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthOk => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthFailed => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthReject => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthBadServerProof => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthUnavailable => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthSystemError => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthBillingError => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthBillingExpired => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthVersionMismatch => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthUnknownAccount => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthIncorrectPassword => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthSessionExpired => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthServerShuttingDown => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthAlreadyLoggingIn => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthLoginServerNotFound => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthWaitQueue => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthBanned => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthAlreadyOnline => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthNoTime => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthDbBusy => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthSuspended => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthParentalControl => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthLockedEnforced => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::RealmListInProgress => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::RealmListSuccess => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::RealmListFailed => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::RealmListInvalid => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::RealmListRealmNotFound => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AccountCreateInProgress => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AccountCreateSuccess => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::AccountCreateFailed => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharListRetrieving => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharListRetrieved => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharListFailed => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateInProgress => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateSuccess => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateError => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateFailed => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateNameInUse => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateDisabled => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreatePvpTeamsViolation => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateServerLimit => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateAccountLimit => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateServerQueue => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateOnlyExisting => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateExpansion => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateExpansionClass => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateLevelRequirement => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateUniqueClassLimit => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateCharacterInGuild => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateRestrictedRaceclass => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateCharacterChooseRace => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateCharacterArenaLeader => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateCharacterDeleteMail => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateCharacterSwapFaction => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateCharacterRaceOnly => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateCharacterGoldLimit => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateForceLogin => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharDeleteInProgress => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharDeleteSuccess => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharDeleteFailed => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharDeleteFailedLockedForTransfer => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharDeleteFailedGuildLeader => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharDeleteFailedArenaCaptain => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginInProgress => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginSuccess => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginNoWorld => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginDuplicateCharacter => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginNoInstances => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginFailed => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginDisabled => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginNoCharacter => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginLockedForTransfer => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginLockedByBilling => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginLockedByMobileAh => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameSuccess => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameFailure => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameNoName => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameTooShort => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameTooLong => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameInvalidCharacter => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameMixedLanguages => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameProfane => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameReserved => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameInvalidApostrophe => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameMultipleApostrophes => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameThreeConsecutive => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameInvalidSpace => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameConsecutiveSpaces => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameRussianConsecutiveSilentCharacters => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameRussianSilentCharacterAtBeginningOrEnd => {}
            SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameDeclensionDoesntMatchBaseName => {}
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=272).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04DA, size: body_size as u32 });
        }

        // result: WorldResult
        let result: WorldResult = crate::util::read_u8_le(&mut r)?.try_into()?;

        let result_if = match result {
            WorldResult::ResponseSuccess => {
                // guid: Guid
                let guid = Guid::read(&mut r)?;

                // name: CString
                let name = {
                    let name = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(name)?
                };

                // gender: Gender
                let gender: Gender = crate::util::read_u8_le(&mut r)?.try_into()?;

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

                // race: Race
                let race: Race = crate::util::read_u8_le(&mut r)?.try_into()?;

                SMSG_CHAR_FACTION_CHANGE_WorldResult::ResponseSuccess {
                    face,
                    facial_hair,
                    gender,
                    guid,
                    hair_color,
                    hair_style,
                    name,
                    race,
                    skin_color,
                }
            }
            WorldResult::ResponseFailure => SMSG_CHAR_FACTION_CHANGE_WorldResult::ResponseFailure,
            WorldResult::ResponseCancelled => SMSG_CHAR_FACTION_CHANGE_WorldResult::ResponseCancelled,
            WorldResult::ResponseDisconnected => SMSG_CHAR_FACTION_CHANGE_WorldResult::ResponseDisconnected,
            WorldResult::ResponseFailedToConnect => SMSG_CHAR_FACTION_CHANGE_WorldResult::ResponseFailedToConnect,
            WorldResult::ResponseConnected => SMSG_CHAR_FACTION_CHANGE_WorldResult::ResponseConnected,
            WorldResult::ResponseVersionMismatch => SMSG_CHAR_FACTION_CHANGE_WorldResult::ResponseVersionMismatch,
            WorldResult::CstatusConnecting => SMSG_CHAR_FACTION_CHANGE_WorldResult::CstatusConnecting,
            WorldResult::CstatusNegotiatingSecurity => SMSG_CHAR_FACTION_CHANGE_WorldResult::CstatusNegotiatingSecurity,
            WorldResult::CstatusNegotiationComplete => SMSG_CHAR_FACTION_CHANGE_WorldResult::CstatusNegotiationComplete,
            WorldResult::CstatusNegotiationFailed => SMSG_CHAR_FACTION_CHANGE_WorldResult::CstatusNegotiationFailed,
            WorldResult::CstatusAuthenticating => SMSG_CHAR_FACTION_CHANGE_WorldResult::CstatusAuthenticating,
            WorldResult::AuthOk => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthOk,
            WorldResult::AuthFailed => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthFailed,
            WorldResult::AuthReject => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthReject,
            WorldResult::AuthBadServerProof => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthBadServerProof,
            WorldResult::AuthUnavailable => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthUnavailable,
            WorldResult::AuthSystemError => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthSystemError,
            WorldResult::AuthBillingError => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthBillingError,
            WorldResult::AuthBillingExpired => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthBillingExpired,
            WorldResult::AuthVersionMismatch => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthVersionMismatch,
            WorldResult::AuthUnknownAccount => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthUnknownAccount,
            WorldResult::AuthIncorrectPassword => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthIncorrectPassword,
            WorldResult::AuthSessionExpired => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthSessionExpired,
            WorldResult::AuthServerShuttingDown => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthServerShuttingDown,
            WorldResult::AuthAlreadyLoggingIn => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthAlreadyLoggingIn,
            WorldResult::AuthLoginServerNotFound => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthLoginServerNotFound,
            WorldResult::AuthWaitQueue => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthWaitQueue,
            WorldResult::AuthBanned => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthBanned,
            WorldResult::AuthAlreadyOnline => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthAlreadyOnline,
            WorldResult::AuthNoTime => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthNoTime,
            WorldResult::AuthDbBusy => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthDbBusy,
            WorldResult::AuthSuspended => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthSuspended,
            WorldResult::AuthParentalControl => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthParentalControl,
            WorldResult::AuthLockedEnforced => SMSG_CHAR_FACTION_CHANGE_WorldResult::AuthLockedEnforced,
            WorldResult::RealmListInProgress => SMSG_CHAR_FACTION_CHANGE_WorldResult::RealmListInProgress,
            WorldResult::RealmListSuccess => SMSG_CHAR_FACTION_CHANGE_WorldResult::RealmListSuccess,
            WorldResult::RealmListFailed => SMSG_CHAR_FACTION_CHANGE_WorldResult::RealmListFailed,
            WorldResult::RealmListInvalid => SMSG_CHAR_FACTION_CHANGE_WorldResult::RealmListInvalid,
            WorldResult::RealmListRealmNotFound => SMSG_CHAR_FACTION_CHANGE_WorldResult::RealmListRealmNotFound,
            WorldResult::AccountCreateInProgress => SMSG_CHAR_FACTION_CHANGE_WorldResult::AccountCreateInProgress,
            WorldResult::AccountCreateSuccess => SMSG_CHAR_FACTION_CHANGE_WorldResult::AccountCreateSuccess,
            WorldResult::AccountCreateFailed => SMSG_CHAR_FACTION_CHANGE_WorldResult::AccountCreateFailed,
            WorldResult::CharListRetrieving => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharListRetrieving,
            WorldResult::CharListRetrieved => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharListRetrieved,
            WorldResult::CharListFailed => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharListFailed,
            WorldResult::CharCreateInProgress => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateInProgress,
            WorldResult::CharCreateSuccess => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateSuccess,
            WorldResult::CharCreateError => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateError,
            WorldResult::CharCreateFailed => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateFailed,
            WorldResult::CharCreateNameInUse => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateNameInUse,
            WorldResult::CharCreateDisabled => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateDisabled,
            WorldResult::CharCreatePvpTeamsViolation => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreatePvpTeamsViolation,
            WorldResult::CharCreateServerLimit => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateServerLimit,
            WorldResult::CharCreateAccountLimit => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateAccountLimit,
            WorldResult::CharCreateServerQueue => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateServerQueue,
            WorldResult::CharCreateOnlyExisting => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateOnlyExisting,
            WorldResult::CharCreateExpansion => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateExpansion,
            WorldResult::CharCreateExpansionClass => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateExpansionClass,
            WorldResult::CharCreateLevelRequirement => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateLevelRequirement,
            WorldResult::CharCreateUniqueClassLimit => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateUniqueClassLimit,
            WorldResult::CharCreateCharacterInGuild => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateCharacterInGuild,
            WorldResult::CharCreateRestrictedRaceclass => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateRestrictedRaceclass,
            WorldResult::CharCreateCharacterChooseRace => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateCharacterChooseRace,
            WorldResult::CharCreateCharacterArenaLeader => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateCharacterArenaLeader,
            WorldResult::CharCreateCharacterDeleteMail => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateCharacterDeleteMail,
            WorldResult::CharCreateCharacterSwapFaction => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateCharacterSwapFaction,
            WorldResult::CharCreateCharacterRaceOnly => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateCharacterRaceOnly,
            WorldResult::CharCreateCharacterGoldLimit => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateCharacterGoldLimit,
            WorldResult::CharCreateForceLogin => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharCreateForceLogin,
            WorldResult::CharDeleteInProgress => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharDeleteInProgress,
            WorldResult::CharDeleteSuccess => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharDeleteSuccess,
            WorldResult::CharDeleteFailed => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharDeleteFailed,
            WorldResult::CharDeleteFailedLockedForTransfer => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharDeleteFailedLockedForTransfer,
            WorldResult::CharDeleteFailedGuildLeader => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharDeleteFailedGuildLeader,
            WorldResult::CharDeleteFailedArenaCaptain => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharDeleteFailedArenaCaptain,
            WorldResult::CharLoginInProgress => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginInProgress,
            WorldResult::CharLoginSuccess => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginSuccess,
            WorldResult::CharLoginNoWorld => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginNoWorld,
            WorldResult::CharLoginDuplicateCharacter => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginDuplicateCharacter,
            WorldResult::CharLoginNoInstances => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginNoInstances,
            WorldResult::CharLoginFailed => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginFailed,
            WorldResult::CharLoginDisabled => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginDisabled,
            WorldResult::CharLoginNoCharacter => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginNoCharacter,
            WorldResult::CharLoginLockedForTransfer => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginLockedForTransfer,
            WorldResult::CharLoginLockedByBilling => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginLockedByBilling,
            WorldResult::CharLoginLockedByMobileAh => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharLoginLockedByMobileAh,
            WorldResult::CharNameSuccess => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameSuccess,
            WorldResult::CharNameFailure => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameFailure,
            WorldResult::CharNameNoName => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameNoName,
            WorldResult::CharNameTooShort => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameTooShort,
            WorldResult::CharNameTooLong => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameTooLong,
            WorldResult::CharNameInvalidCharacter => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameInvalidCharacter,
            WorldResult::CharNameMixedLanguages => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameMixedLanguages,
            WorldResult::CharNameProfane => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameProfane,
            WorldResult::CharNameReserved => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameReserved,
            WorldResult::CharNameInvalidApostrophe => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameInvalidApostrophe,
            WorldResult::CharNameMultipleApostrophes => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameMultipleApostrophes,
            WorldResult::CharNameThreeConsecutive => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameThreeConsecutive,
            WorldResult::CharNameInvalidSpace => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameInvalidSpace,
            WorldResult::CharNameConsecutiveSpaces => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameConsecutiveSpaces,
            WorldResult::CharNameRussianConsecutiveSilentCharacters => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameRussianConsecutiveSilentCharacters,
            WorldResult::CharNameRussianSilentCharacterAtBeginningOrEnd => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameRussianSilentCharacterAtBeginningOrEnd,
            WorldResult::CharNameDeclensionDoesntMatchBaseName => SMSG_CHAR_FACTION_CHANGE_WorldResult::CharNameDeclensionDoesntMatchBaseName,
        };

        Ok(Self {
            result: result_if,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CHAR_FACTION_CHANGE {}

impl SMSG_CHAR_FACTION_CHANGE {
    pub(crate) fn size(&self) -> usize {
        self.result.size() // result: SMSG_CHAR_FACTION_CHANGE_WorldResult
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_CHAR_FACTION_CHANGE_WorldResult {
    ResponseSuccess {
        face: u8,
        facial_hair: u8,
        gender: Gender,
        guid: Guid,
        hair_color: u8,
        hair_style: u8,
        name: String,
        race: Race,
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

impl Default for SMSG_CHAR_FACTION_CHANGE_WorldResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::ResponseFailure
    }
}

impl SMSG_CHAR_FACTION_CHANGE_WorldResult {
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

impl SMSG_CHAR_FACTION_CHANGE_WorldResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::ResponseSuccess {
                face,
                facial_hair,
                gender,
                guid,
                hair_color,
                hair_style,
                name,
                race,
                skin_color,
            } => {
                1
                + 1 // face: u8
                + 1 // facial_hair: u8
                + 1 // gender: Gender
                + 8 // guid: Guid
                + 1 // hair_color: u8
                + 1 // hair_style: u8
                + name.len() + 1 // name: CString
                + 1 // race: Race
                + 1 // skin_color: u8
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
            Self::CharCreateExpansionClass => {
                1
            }
            Self::CharCreateLevelRequirement => {
                1
            }
            Self::CharCreateUniqueClassLimit => {
                1
            }
            Self::CharCreateCharacterInGuild => {
                1
            }
            Self::CharCreateRestrictedRaceclass => {
                1
            }
            Self::CharCreateCharacterChooseRace => {
                1
            }
            Self::CharCreateCharacterArenaLeader => {
                1
            }
            Self::CharCreateCharacterDeleteMail => {
                1
            }
            Self::CharCreateCharacterSwapFaction => {
                1
            }
            Self::CharCreateCharacterRaceOnly => {
                1
            }
            Self::CharCreateCharacterGoldLimit => {
                1
            }
            Self::CharCreateForceLogin => {
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
            Self::CharLoginLockedByMobileAh => {
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

