/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/world_result.wowm:345`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/world_result.wowm#L345):
/// ```text
/// enum WorldResult : u8 {
///     RESPONSE_SUCCESS = 0x00;
///     RESPONSE_FAILURE = 0x01;
///     RESPONSE_CANCELLED = 0x02;
///     RESPONSE_DISCONNECTED = 0x03;
///     RESPONSE_FAILED_TO_CONNECT = 0x04;
///     RESPONSE_CONNECTED = 0x05;
///     RESPONSE_VERSION_MISMATCH = 0x06;
///     CSTATUS_CONNECTING = 0x07;
///     CSTATUS_NEGOTIATING_SECURITY = 0x08;
///     CSTATUS_NEGOTIATION_COMPLETE = 0x09;
///     CSTATUS_NEGOTIATION_FAILED = 0x0A;
///     CSTATUS_AUTHENTICATING = 0x0B;
///     AUTH_OK = 0x0C;
///     AUTH_FAILED = 0x0D;
///     AUTH_REJECT = 0x0E;
///     AUTH_BAD_SERVER_PROOF = 0x0F;
///     AUTH_UNAVAILABLE = 0x10;
///     AUTH_SYSTEM_ERROR = 0x11;
///     AUTH_BILLING_ERROR = 0x12;
///     AUTH_BILLING_EXPIRED = 0x13;
///     AUTH_VERSION_MISMATCH = 0x14;
///     AUTH_UNKNOWN_ACCOUNT = 0x15;
///     AUTH_INCORRECT_PASSWORD = 0x16;
///     AUTH_SESSION_EXPIRED = 0x17;
///     AUTH_SERVER_SHUTTING_DOWN = 0x18;
///     AUTH_ALREADY_LOGGING_IN = 0x19;
///     AUTH_LOGIN_SERVER_NOT_FOUND = 0x1A;
///     AUTH_WAIT_QUEUE = 0x1B;
///     AUTH_BANNED = 0x1C;
///     AUTH_ALREADY_ONLINE = 0x1D;
///     AUTH_NO_TIME = 0x1E;
///     AUTH_DB_BUSY = 0x1F;
///     AUTH_SUSPENDED = 0x20;
///     AUTH_PARENTAL_CONTROL = 0x21;
///     AUTH_LOCKED_ENFORCED = 0x22;
///     REALM_LIST_IN_PROGRESS = 0x23;
///     REALM_LIST_SUCCESS = 0x24;
///     REALM_LIST_FAILED = 0x25;
///     REALM_LIST_INVALID = 0x26;
///     REALM_LIST_REALM_NOT_FOUND = 0x27;
///     ACCOUNT_CREATE_IN_PROGRESS = 0x28;
///     ACCOUNT_CREATE_SUCCESS = 0x29;
///     ACCOUNT_CREATE_FAILED = 0x2A;
///     CHAR_LIST_RETRIEVING = 0x2B;
///     CHAR_LIST_RETRIEVED = 0x2C;
///     CHAR_LIST_FAILED = 0x2D;
///     CHAR_CREATE_IN_PROGRESS = 0x2E;
///     CHAR_CREATE_SUCCESS = 0x2F;
///     CHAR_CREATE_ERROR = 0x30;
///     CHAR_CREATE_FAILED = 0x31;
///     CHAR_CREATE_NAME_IN_USE = 0x32;
///     CHAR_CREATE_DISABLED = 0x33;
///     CHAR_CREATE_PVP_TEAMS_VIOLATION = 0x34;
///     CHAR_CREATE_SERVER_LIMIT = 0x35;
///     CHAR_CREATE_ACCOUNT_LIMIT = 0x36;
///     CHAR_CREATE_SERVER_QUEUE = 0x37;
///     CHAR_CREATE_ONLY_EXISTING = 0x38;
///     CHAR_CREATE_EXPANSION = 0x39;
///     CHAR_CREATE_EXPANSION_CLASS = 0x3A;
///     CHAR_CREATE_LEVEL_REQUIREMENT = 0x3B;
///     CHAR_CREATE_UNIQUE_CLASS_LIMIT = 0x3C;
///     CHAR_CREATE_CHARACTER_IN_GUILD = 0x3D;
///     CHAR_CREATE_RESTRICTED_RACECLASS = 0x3E;
///     CHAR_CREATE_CHARACTER_CHOOSE_RACE = 0x3F;
///     CHAR_CREATE_CHARACTER_ARENA_LEADER = 0x40;
///     CHAR_CREATE_CHARACTER_DELETE_MAIL = 0x41;
///     CHAR_CREATE_CHARACTER_SWAP_FACTION = 0x42;
///     CHAR_CREATE_CHARACTER_RACE_ONLY = 0x43;
///     CHAR_CREATE_CHARACTER_GOLD_LIMIT = 0x44;
///     CHAR_CREATE_FORCE_LOGIN = 0x45;
///     CHAR_DELETE_IN_PROGRESS = 0x46;
///     CHAR_DELETE_SUCCESS = 0x47;
///     CHAR_DELETE_FAILED = 0x48;
///     CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER = 0x49;
///     CHAR_DELETE_FAILED_GUILD_LEADER = 0x4A;
///     CHAR_DELETE_FAILED_ARENA_CAPTAIN = 0x4B;
///     CHAR_LOGIN_IN_PROGRESS = 0x4C;
///     CHAR_LOGIN_SUCCESS = 0x4D;
///     CHAR_LOGIN_NO_WORLD = 0x4E;
///     CHAR_LOGIN_DUPLICATE_CHARACTER = 0x4F;
///     CHAR_LOGIN_NO_INSTANCES = 0x50;
///     CHAR_LOGIN_FAILED = 0x51;
///     CHAR_LOGIN_DISABLED = 0x52;
///     CHAR_LOGIN_NO_CHARACTER = 0x53;
///     CHAR_LOGIN_LOCKED_FOR_TRANSFER = 0x54;
///     CHAR_LOGIN_LOCKED_BY_BILLING = 0x55;
///     CHAR_LOGIN_LOCKED_BY_MOBILE_AH = 0x56;
///     CHAR_NAME_SUCCESS = 0x57;
///     CHAR_NAME_FAILURE = 0x58;
///     CHAR_NAME_NO_NAME = 0x59;
///     CHAR_NAME_TOO_SHORT = 0x5A;
///     CHAR_NAME_TOO_LONG = 0x5B;
///     CHAR_NAME_INVALID_CHARACTER = 0x5C;
///     CHAR_NAME_MIXED_LANGUAGES = 0x5D;
///     CHAR_NAME_PROFANE = 0x5E;
///     CHAR_NAME_RESERVED = 0x5F;
///     CHAR_NAME_INVALID_APOSTROPHE = 0x60;
///     CHAR_NAME_MULTIPLE_APOSTROPHES = 0x61;
///     CHAR_NAME_THREE_CONSECUTIVE = 0x62;
///     CHAR_NAME_INVALID_SPACE = 0x63;
///     CHAR_NAME_CONSECUTIVE_SPACES = 0x64;
///     CHAR_NAME_RUSSIAN_CONSECUTIVE_SILENT_CHARACTERS = 0x65;
///     CHAR_NAME_RUSSIAN_SILENT_CHARACTER_AT_BEGINNING_OR_END = 0x66;
///     CHAR_NAME_DECLENSION_DOESNT_MATCH_BASE_NAME = 0x67;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum WorldResult {
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

impl WorldResult {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::ResponseSuccess => 0x0,
            Self::ResponseFailure => 0x1,
            Self::ResponseCancelled => 0x2,
            Self::ResponseDisconnected => 0x3,
            Self::ResponseFailedToConnect => 0x4,
            Self::ResponseConnected => 0x5,
            Self::ResponseVersionMismatch => 0x6,
            Self::CstatusConnecting => 0x7,
            Self::CstatusNegotiatingSecurity => 0x8,
            Self::CstatusNegotiationComplete => 0x9,
            Self::CstatusNegotiationFailed => 0xa,
            Self::CstatusAuthenticating => 0xb,
            Self::AuthOk => 0xc,
            Self::AuthFailed => 0xd,
            Self::AuthReject => 0xe,
            Self::AuthBadServerProof => 0xf,
            Self::AuthUnavailable => 0x10,
            Self::AuthSystemError => 0x11,
            Self::AuthBillingError => 0x12,
            Self::AuthBillingExpired => 0x13,
            Self::AuthVersionMismatch => 0x14,
            Self::AuthUnknownAccount => 0x15,
            Self::AuthIncorrectPassword => 0x16,
            Self::AuthSessionExpired => 0x17,
            Self::AuthServerShuttingDown => 0x18,
            Self::AuthAlreadyLoggingIn => 0x19,
            Self::AuthLoginServerNotFound => 0x1a,
            Self::AuthWaitQueue => 0x1b,
            Self::AuthBanned => 0x1c,
            Self::AuthAlreadyOnline => 0x1d,
            Self::AuthNoTime => 0x1e,
            Self::AuthDbBusy => 0x1f,
            Self::AuthSuspended => 0x20,
            Self::AuthParentalControl => 0x21,
            Self::AuthLockedEnforced => 0x22,
            Self::RealmListInProgress => 0x23,
            Self::RealmListSuccess => 0x24,
            Self::RealmListFailed => 0x25,
            Self::RealmListInvalid => 0x26,
            Self::RealmListRealmNotFound => 0x27,
            Self::AccountCreateInProgress => 0x28,
            Self::AccountCreateSuccess => 0x29,
            Self::AccountCreateFailed => 0x2a,
            Self::CharListRetrieving => 0x2b,
            Self::CharListRetrieved => 0x2c,
            Self::CharListFailed => 0x2d,
            Self::CharCreateInProgress => 0x2e,
            Self::CharCreateSuccess => 0x2f,
            Self::CharCreateError => 0x30,
            Self::CharCreateFailed => 0x31,
            Self::CharCreateNameInUse => 0x32,
            Self::CharCreateDisabled => 0x33,
            Self::CharCreatePvpTeamsViolation => 0x34,
            Self::CharCreateServerLimit => 0x35,
            Self::CharCreateAccountLimit => 0x36,
            Self::CharCreateServerQueue => 0x37,
            Self::CharCreateOnlyExisting => 0x38,
            Self::CharCreateExpansion => 0x39,
            Self::CharCreateExpansionClass => 0x3a,
            Self::CharCreateLevelRequirement => 0x3b,
            Self::CharCreateUniqueClassLimit => 0x3c,
            Self::CharCreateCharacterInGuild => 0x3d,
            Self::CharCreateRestrictedRaceclass => 0x3e,
            Self::CharCreateCharacterChooseRace => 0x3f,
            Self::CharCreateCharacterArenaLeader => 0x40,
            Self::CharCreateCharacterDeleteMail => 0x41,
            Self::CharCreateCharacterSwapFaction => 0x42,
            Self::CharCreateCharacterRaceOnly => 0x43,
            Self::CharCreateCharacterGoldLimit => 0x44,
            Self::CharCreateForceLogin => 0x45,
            Self::CharDeleteInProgress => 0x46,
            Self::CharDeleteSuccess => 0x47,
            Self::CharDeleteFailed => 0x48,
            Self::CharDeleteFailedLockedForTransfer => 0x49,
            Self::CharDeleteFailedGuildLeader => 0x4a,
            Self::CharDeleteFailedArenaCaptain => 0x4b,
            Self::CharLoginInProgress => 0x4c,
            Self::CharLoginSuccess => 0x4d,
            Self::CharLoginNoWorld => 0x4e,
            Self::CharLoginDuplicateCharacter => 0x4f,
            Self::CharLoginNoInstances => 0x50,
            Self::CharLoginFailed => 0x51,
            Self::CharLoginDisabled => 0x52,
            Self::CharLoginNoCharacter => 0x53,
            Self::CharLoginLockedForTransfer => 0x54,
            Self::CharLoginLockedByBilling => 0x55,
            Self::CharLoginLockedByMobileAh => 0x56,
            Self::CharNameSuccess => 0x57,
            Self::CharNameFailure => 0x58,
            Self::CharNameNoName => 0x59,
            Self::CharNameTooShort => 0x5a,
            Self::CharNameTooLong => 0x5b,
            Self::CharNameInvalidCharacter => 0x5c,
            Self::CharNameMixedLanguages => 0x5d,
            Self::CharNameProfane => 0x5e,
            Self::CharNameReserved => 0x5f,
            Self::CharNameInvalidApostrophe => 0x60,
            Self::CharNameMultipleApostrophes => 0x61,
            Self::CharNameThreeConsecutive => 0x62,
            Self::CharNameInvalidSpace => 0x63,
            Self::CharNameConsecutiveSpaces => 0x64,
            Self::CharNameRussianConsecutiveSilentCharacters => 0x65,
            Self::CharNameRussianSilentCharacterAtBeginningOrEnd => 0x66,
            Self::CharNameDeclensionDoesntMatchBaseName => 0x67,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl WorldResult {
    pub fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::ResponseSuccess => "RESPONSE_SUCCESS",
            Self::ResponseFailure => "RESPONSE_FAILURE",
            Self::ResponseCancelled => "RESPONSE_CANCELLED",
            Self::ResponseDisconnected => "RESPONSE_DISCONNECTED",
            Self::ResponseFailedToConnect => "RESPONSE_FAILED_TO_CONNECT",
            Self::ResponseConnected => "RESPONSE_CONNECTED",
            Self::ResponseVersionMismatch => "RESPONSE_VERSION_MISMATCH",
            Self::CstatusConnecting => "CSTATUS_CONNECTING",
            Self::CstatusNegotiatingSecurity => "CSTATUS_NEGOTIATING_SECURITY",
            Self::CstatusNegotiationComplete => "CSTATUS_NEGOTIATION_COMPLETE",
            Self::CstatusNegotiationFailed => "CSTATUS_NEGOTIATION_FAILED",
            Self::CstatusAuthenticating => "CSTATUS_AUTHENTICATING",
            Self::AuthOk => "AUTH_OK",
            Self::AuthFailed => "AUTH_FAILED",
            Self::AuthReject => "AUTH_REJECT",
            Self::AuthBadServerProof => "AUTH_BAD_SERVER_PROOF",
            Self::AuthUnavailable => "AUTH_UNAVAILABLE",
            Self::AuthSystemError => "AUTH_SYSTEM_ERROR",
            Self::AuthBillingError => "AUTH_BILLING_ERROR",
            Self::AuthBillingExpired => "AUTH_BILLING_EXPIRED",
            Self::AuthVersionMismatch => "AUTH_VERSION_MISMATCH",
            Self::AuthUnknownAccount => "AUTH_UNKNOWN_ACCOUNT",
            Self::AuthIncorrectPassword => "AUTH_INCORRECT_PASSWORD",
            Self::AuthSessionExpired => "AUTH_SESSION_EXPIRED",
            Self::AuthServerShuttingDown => "AUTH_SERVER_SHUTTING_DOWN",
            Self::AuthAlreadyLoggingIn => "AUTH_ALREADY_LOGGING_IN",
            Self::AuthLoginServerNotFound => "AUTH_LOGIN_SERVER_NOT_FOUND",
            Self::AuthWaitQueue => "AUTH_WAIT_QUEUE",
            Self::AuthBanned => "AUTH_BANNED",
            Self::AuthAlreadyOnline => "AUTH_ALREADY_ONLINE",
            Self::AuthNoTime => "AUTH_NO_TIME",
            Self::AuthDbBusy => "AUTH_DB_BUSY",
            Self::AuthSuspended => "AUTH_SUSPENDED",
            Self::AuthParentalControl => "AUTH_PARENTAL_CONTROL",
            Self::AuthLockedEnforced => "AUTH_LOCKED_ENFORCED",
            Self::RealmListInProgress => "REALM_LIST_IN_PROGRESS",
            Self::RealmListSuccess => "REALM_LIST_SUCCESS",
            Self::RealmListFailed => "REALM_LIST_FAILED",
            Self::RealmListInvalid => "REALM_LIST_INVALID",
            Self::RealmListRealmNotFound => "REALM_LIST_REALM_NOT_FOUND",
            Self::AccountCreateInProgress => "ACCOUNT_CREATE_IN_PROGRESS",
            Self::AccountCreateSuccess => "ACCOUNT_CREATE_SUCCESS",
            Self::AccountCreateFailed => "ACCOUNT_CREATE_FAILED",
            Self::CharListRetrieving => "CHAR_LIST_RETRIEVING",
            Self::CharListRetrieved => "CHAR_LIST_RETRIEVED",
            Self::CharListFailed => "CHAR_LIST_FAILED",
            Self::CharCreateInProgress => "CHAR_CREATE_IN_PROGRESS",
            Self::CharCreateSuccess => "CHAR_CREATE_SUCCESS",
            Self::CharCreateError => "CHAR_CREATE_ERROR",
            Self::CharCreateFailed => "CHAR_CREATE_FAILED",
            Self::CharCreateNameInUse => "CHAR_CREATE_NAME_IN_USE",
            Self::CharCreateDisabled => "CHAR_CREATE_DISABLED",
            Self::CharCreatePvpTeamsViolation => "CHAR_CREATE_PVP_TEAMS_VIOLATION",
            Self::CharCreateServerLimit => "CHAR_CREATE_SERVER_LIMIT",
            Self::CharCreateAccountLimit => "CHAR_CREATE_ACCOUNT_LIMIT",
            Self::CharCreateServerQueue => "CHAR_CREATE_SERVER_QUEUE",
            Self::CharCreateOnlyExisting => "CHAR_CREATE_ONLY_EXISTING",
            Self::CharCreateExpansion => "CHAR_CREATE_EXPANSION",
            Self::CharCreateExpansionClass => "CHAR_CREATE_EXPANSION_CLASS",
            Self::CharCreateLevelRequirement => "CHAR_CREATE_LEVEL_REQUIREMENT",
            Self::CharCreateUniqueClassLimit => "CHAR_CREATE_UNIQUE_CLASS_LIMIT",
            Self::CharCreateCharacterInGuild => "CHAR_CREATE_CHARACTER_IN_GUILD",
            Self::CharCreateRestrictedRaceclass => "CHAR_CREATE_RESTRICTED_RACECLASS",
            Self::CharCreateCharacterChooseRace => "CHAR_CREATE_CHARACTER_CHOOSE_RACE",
            Self::CharCreateCharacterArenaLeader => "CHAR_CREATE_CHARACTER_ARENA_LEADER",
            Self::CharCreateCharacterDeleteMail => "CHAR_CREATE_CHARACTER_DELETE_MAIL",
            Self::CharCreateCharacterSwapFaction => "CHAR_CREATE_CHARACTER_SWAP_FACTION",
            Self::CharCreateCharacterRaceOnly => "CHAR_CREATE_CHARACTER_RACE_ONLY",
            Self::CharCreateCharacterGoldLimit => "CHAR_CREATE_CHARACTER_GOLD_LIMIT",
            Self::CharCreateForceLogin => "CHAR_CREATE_FORCE_LOGIN",
            Self::CharDeleteInProgress => "CHAR_DELETE_IN_PROGRESS",
            Self::CharDeleteSuccess => "CHAR_DELETE_SUCCESS",
            Self::CharDeleteFailed => "CHAR_DELETE_FAILED",
            Self::CharDeleteFailedLockedForTransfer => "CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER",
            Self::CharDeleteFailedGuildLeader => "CHAR_DELETE_FAILED_GUILD_LEADER",
            Self::CharDeleteFailedArenaCaptain => "CHAR_DELETE_FAILED_ARENA_CAPTAIN",
            Self::CharLoginInProgress => "CHAR_LOGIN_IN_PROGRESS",
            Self::CharLoginSuccess => "CHAR_LOGIN_SUCCESS",
            Self::CharLoginNoWorld => "CHAR_LOGIN_NO_WORLD",
            Self::CharLoginDuplicateCharacter => "CHAR_LOGIN_DUPLICATE_CHARACTER",
            Self::CharLoginNoInstances => "CHAR_LOGIN_NO_INSTANCES",
            Self::CharLoginFailed => "CHAR_LOGIN_FAILED",
            Self::CharLoginDisabled => "CHAR_LOGIN_DISABLED",
            Self::CharLoginNoCharacter => "CHAR_LOGIN_NO_CHARACTER",
            Self::CharLoginLockedForTransfer => "CHAR_LOGIN_LOCKED_FOR_TRANSFER",
            Self::CharLoginLockedByBilling => "CHAR_LOGIN_LOCKED_BY_BILLING",
            Self::CharLoginLockedByMobileAh => "CHAR_LOGIN_LOCKED_BY_MOBILE_AH",
            Self::CharNameSuccess => "CHAR_NAME_SUCCESS",
            Self::CharNameFailure => "CHAR_NAME_FAILURE",
            Self::CharNameNoName => "CHAR_NAME_NO_NAME",
            Self::CharNameTooShort => "CHAR_NAME_TOO_SHORT",
            Self::CharNameTooLong => "CHAR_NAME_TOO_LONG",
            Self::CharNameInvalidCharacter => "CHAR_NAME_INVALID_CHARACTER",
            Self::CharNameMixedLanguages => "CHAR_NAME_MIXED_LANGUAGES",
            Self::CharNameProfane => "CHAR_NAME_PROFANE",
            Self::CharNameReserved => "CHAR_NAME_RESERVED",
            Self::CharNameInvalidApostrophe => "CHAR_NAME_INVALID_APOSTROPHE",
            Self::CharNameMultipleApostrophes => "CHAR_NAME_MULTIPLE_APOSTROPHES",
            Self::CharNameThreeConsecutive => "CHAR_NAME_THREE_CONSECUTIVE",
            Self::CharNameInvalidSpace => "CHAR_NAME_INVALID_SPACE",
            Self::CharNameConsecutiveSpaces => "CHAR_NAME_CONSECUTIVE_SPACES",
            Self::CharNameRussianConsecutiveSilentCharacters => "CHAR_NAME_RUSSIAN_CONSECUTIVE_SILENT_CHARACTERS",
            Self::CharNameRussianSilentCharacterAtBeginningOrEnd => "CHAR_NAME_RUSSIAN_SILENT_CHARACTER_AT_BEGINNING_OR_END",
            Self::CharNameDeclensionDoesntMatchBaseName => "CHAR_NAME_DECLENSION_DOESNT_MATCH_BASE_NAME",
        }
    }

}

impl Default for WorldResult {
    fn default() -> Self {
        Self::ResponseSuccess
    }
}

impl std::fmt::Display for WorldResult {
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

impl TryFrom<u8> for WorldResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ResponseSuccess),
            1 => Ok(Self::ResponseFailure),
            2 => Ok(Self::ResponseCancelled),
            3 => Ok(Self::ResponseDisconnected),
            4 => Ok(Self::ResponseFailedToConnect),
            5 => Ok(Self::ResponseConnected),
            6 => Ok(Self::ResponseVersionMismatch),
            7 => Ok(Self::CstatusConnecting),
            8 => Ok(Self::CstatusNegotiatingSecurity),
            9 => Ok(Self::CstatusNegotiationComplete),
            10 => Ok(Self::CstatusNegotiationFailed),
            11 => Ok(Self::CstatusAuthenticating),
            12 => Ok(Self::AuthOk),
            13 => Ok(Self::AuthFailed),
            14 => Ok(Self::AuthReject),
            15 => Ok(Self::AuthBadServerProof),
            16 => Ok(Self::AuthUnavailable),
            17 => Ok(Self::AuthSystemError),
            18 => Ok(Self::AuthBillingError),
            19 => Ok(Self::AuthBillingExpired),
            20 => Ok(Self::AuthVersionMismatch),
            21 => Ok(Self::AuthUnknownAccount),
            22 => Ok(Self::AuthIncorrectPassword),
            23 => Ok(Self::AuthSessionExpired),
            24 => Ok(Self::AuthServerShuttingDown),
            25 => Ok(Self::AuthAlreadyLoggingIn),
            26 => Ok(Self::AuthLoginServerNotFound),
            27 => Ok(Self::AuthWaitQueue),
            28 => Ok(Self::AuthBanned),
            29 => Ok(Self::AuthAlreadyOnline),
            30 => Ok(Self::AuthNoTime),
            31 => Ok(Self::AuthDbBusy),
            32 => Ok(Self::AuthSuspended),
            33 => Ok(Self::AuthParentalControl),
            34 => Ok(Self::AuthLockedEnforced),
            35 => Ok(Self::RealmListInProgress),
            36 => Ok(Self::RealmListSuccess),
            37 => Ok(Self::RealmListFailed),
            38 => Ok(Self::RealmListInvalid),
            39 => Ok(Self::RealmListRealmNotFound),
            40 => Ok(Self::AccountCreateInProgress),
            41 => Ok(Self::AccountCreateSuccess),
            42 => Ok(Self::AccountCreateFailed),
            43 => Ok(Self::CharListRetrieving),
            44 => Ok(Self::CharListRetrieved),
            45 => Ok(Self::CharListFailed),
            46 => Ok(Self::CharCreateInProgress),
            47 => Ok(Self::CharCreateSuccess),
            48 => Ok(Self::CharCreateError),
            49 => Ok(Self::CharCreateFailed),
            50 => Ok(Self::CharCreateNameInUse),
            51 => Ok(Self::CharCreateDisabled),
            52 => Ok(Self::CharCreatePvpTeamsViolation),
            53 => Ok(Self::CharCreateServerLimit),
            54 => Ok(Self::CharCreateAccountLimit),
            55 => Ok(Self::CharCreateServerQueue),
            56 => Ok(Self::CharCreateOnlyExisting),
            57 => Ok(Self::CharCreateExpansion),
            58 => Ok(Self::CharCreateExpansionClass),
            59 => Ok(Self::CharCreateLevelRequirement),
            60 => Ok(Self::CharCreateUniqueClassLimit),
            61 => Ok(Self::CharCreateCharacterInGuild),
            62 => Ok(Self::CharCreateRestrictedRaceclass),
            63 => Ok(Self::CharCreateCharacterChooseRace),
            64 => Ok(Self::CharCreateCharacterArenaLeader),
            65 => Ok(Self::CharCreateCharacterDeleteMail),
            66 => Ok(Self::CharCreateCharacterSwapFaction),
            67 => Ok(Self::CharCreateCharacterRaceOnly),
            68 => Ok(Self::CharCreateCharacterGoldLimit),
            69 => Ok(Self::CharCreateForceLogin),
            70 => Ok(Self::CharDeleteInProgress),
            71 => Ok(Self::CharDeleteSuccess),
            72 => Ok(Self::CharDeleteFailed),
            73 => Ok(Self::CharDeleteFailedLockedForTransfer),
            74 => Ok(Self::CharDeleteFailedGuildLeader),
            75 => Ok(Self::CharDeleteFailedArenaCaptain),
            76 => Ok(Self::CharLoginInProgress),
            77 => Ok(Self::CharLoginSuccess),
            78 => Ok(Self::CharLoginNoWorld),
            79 => Ok(Self::CharLoginDuplicateCharacter),
            80 => Ok(Self::CharLoginNoInstances),
            81 => Ok(Self::CharLoginFailed),
            82 => Ok(Self::CharLoginDisabled),
            83 => Ok(Self::CharLoginNoCharacter),
            84 => Ok(Self::CharLoginLockedForTransfer),
            85 => Ok(Self::CharLoginLockedByBilling),
            86 => Ok(Self::CharLoginLockedByMobileAh),
            87 => Ok(Self::CharNameSuccess),
            88 => Ok(Self::CharNameFailure),
            89 => Ok(Self::CharNameNoName),
            90 => Ok(Self::CharNameTooShort),
            91 => Ok(Self::CharNameTooLong),
            92 => Ok(Self::CharNameInvalidCharacter),
            93 => Ok(Self::CharNameMixedLanguages),
            94 => Ok(Self::CharNameProfane),
            95 => Ok(Self::CharNameReserved),
            96 => Ok(Self::CharNameInvalidApostrophe),
            97 => Ok(Self::CharNameMultipleApostrophes),
            98 => Ok(Self::CharNameThreeConsecutive),
            99 => Ok(Self::CharNameInvalidSpace),
            100 => Ok(Self::CharNameConsecutiveSpaces),
            101 => Ok(Self::CharNameRussianConsecutiveSilentCharacters),
            102 => Ok(Self::CharNameRussianSilentCharacterAtBeginningOrEnd),
            103 => Ok(Self::CharNameDeclensionDoesntMatchBaseName),
            v => Err(crate::errors::EnumError::new("WorldResult", v as u64),)
        }
    }
}

