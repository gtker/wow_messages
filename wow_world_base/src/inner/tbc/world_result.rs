/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/world_result.wowm:164`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/world_result.wowm#L164):
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
///     CHAR_DELETE_IN_PROGRESS = 0x3A;
///     CHAR_DELETE_SUCCESS = 0x3B;
///     CHAR_DELETE_FAILED = 0x3C;
///     CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER = 0x3D;
///     CHAR_DELETE_FAILED_GUILD_LEADER = 0x3E;
///     CHAR_DELETE_FAILED_ARENA_CAPTAIN = 0x3F;
///     CHAR_LOGIN_IN_PROGRESS = 0x40;
///     CHAR_LOGIN_SUCCESS = 0x41;
///     CHAR_LOGIN_NO_WORLD = 0x42;
///     CHAR_LOGIN_DUPLICATE_CHARACTER = 0x43;
///     CHAR_LOGIN_NO_INSTANCES = 0x44;
///     CHAR_LOGIN_FAILED = 0x45;
///     CHAR_LOGIN_DISABLED = 0x46;
///     CHAR_LOGIN_NO_CHARACTER = 0x47;
///     CHAR_LOGIN_LOCKED_FOR_TRANSFER = 0x48;
///     CHAR_LOGIN_LOCKED_BY_BILLING = 0x49;
///     CHAR_NAME_SUCCESS = 0x4A;
///     CHAR_NAME_FAILURE = 0x4B;
///     CHAR_NAME_NO_NAME = 0x4C;
///     CHAR_NAME_TOO_SHORT = 0x4D;
///     CHAR_NAME_TOO_LONG = 0x4E;
///     CHAR_NAME_INVALID_CHARACTER = 0x4F;
///     CHAR_NAME_MIXED_LANGUAGES = 0x50;
///     CHAR_NAME_PROFANE = 0x51;
///     CHAR_NAME_RESERVED = 0x52;
///     CHAR_NAME_INVALID_APOSTROPHE = 0x53;
///     CHAR_NAME_MULTIPLE_APOSTROPHES = 0x54;
///     CHAR_NAME_THREE_CONSECUTIVE = 0x55;
///     CHAR_NAME_INVALID_SPACE = 0x56;
///     CHAR_NAME_CONSECUTIVE_SPACES = 0x57;
///     CHAR_NAME_RUSSIAN_CONSECUTIVE_SILENT_CHARACTERS = 0x58;
///     CHAR_NAME_RUSSIAN_SILENT_CHARACTER_AT_BEGINNING_OR_END = 0x59;
///     CHAR_NAME_DECLENSION_DOESNT_MATCH_BASE_NAME = 0x5A;
/// }
/// ```
#[derive(Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum WorldResult {
    #[default]
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
            Self::CharDeleteInProgress => 0x3a,
            Self::CharDeleteSuccess => 0x3b,
            Self::CharDeleteFailed => 0x3c,
            Self::CharDeleteFailedLockedForTransfer => 0x3d,
            Self::CharDeleteFailedGuildLeader => 0x3e,
            Self::CharDeleteFailedArenaCaptain => 0x3f,
            Self::CharLoginInProgress => 0x40,
            Self::CharLoginSuccess => 0x41,
            Self::CharLoginNoWorld => 0x42,
            Self::CharLoginDuplicateCharacter => 0x43,
            Self::CharLoginNoInstances => 0x44,
            Self::CharLoginFailed => 0x45,
            Self::CharLoginDisabled => 0x46,
            Self::CharLoginNoCharacter => 0x47,
            Self::CharLoginLockedForTransfer => 0x48,
            Self::CharLoginLockedByBilling => 0x49,
            Self::CharNameSuccess => 0x4a,
            Self::CharNameFailure => 0x4b,
            Self::CharNameNoName => 0x4c,
            Self::CharNameTooShort => 0x4d,
            Self::CharNameTooLong => 0x4e,
            Self::CharNameInvalidCharacter => 0x4f,
            Self::CharNameMixedLanguages => 0x50,
            Self::CharNameProfane => 0x51,
            Self::CharNameReserved => 0x52,
            Self::CharNameInvalidApostrophe => 0x53,
            Self::CharNameMultipleApostrophes => 0x54,
            Self::CharNameThreeConsecutive => 0x55,
            Self::CharNameInvalidSpace => 0x56,
            Self::CharNameConsecutiveSpaces => 0x57,
            Self::CharNameRussianConsecutiveSilentCharacters => 0x58,
            Self::CharNameRussianSilentCharacterAtBeginningOrEnd => 0x59,
            Self::CharNameDeclensionDoesntMatchBaseName => 0x5a,
        }
    }

    pub const fn variants() -> [Self; 91] {
        [
            Self::ResponseSuccess,
            Self::ResponseFailure,
            Self::ResponseCancelled,
            Self::ResponseDisconnected,
            Self::ResponseFailedToConnect,
            Self::ResponseConnected,
            Self::ResponseVersionMismatch,
            Self::CstatusConnecting,
            Self::CstatusNegotiatingSecurity,
            Self::CstatusNegotiationComplete,
            Self::CstatusNegotiationFailed,
            Self::CstatusAuthenticating,
            Self::AuthOk,
            Self::AuthFailed,
            Self::AuthReject,
            Self::AuthBadServerProof,
            Self::AuthUnavailable,
            Self::AuthSystemError,
            Self::AuthBillingError,
            Self::AuthBillingExpired,
            Self::AuthVersionMismatch,
            Self::AuthUnknownAccount,
            Self::AuthIncorrectPassword,
            Self::AuthSessionExpired,
            Self::AuthServerShuttingDown,
            Self::AuthAlreadyLoggingIn,
            Self::AuthLoginServerNotFound,
            Self::AuthWaitQueue,
            Self::AuthBanned,
            Self::AuthAlreadyOnline,
            Self::AuthNoTime,
            Self::AuthDbBusy,
            Self::AuthSuspended,
            Self::AuthParentalControl,
            Self::AuthLockedEnforced,
            Self::RealmListInProgress,
            Self::RealmListSuccess,
            Self::RealmListFailed,
            Self::RealmListInvalid,
            Self::RealmListRealmNotFound,
            Self::AccountCreateInProgress,
            Self::AccountCreateSuccess,
            Self::AccountCreateFailed,
            Self::CharListRetrieving,
            Self::CharListRetrieved,
            Self::CharListFailed,
            Self::CharCreateInProgress,
            Self::CharCreateSuccess,
            Self::CharCreateError,
            Self::CharCreateFailed,
            Self::CharCreateNameInUse,
            Self::CharCreateDisabled,
            Self::CharCreatePvpTeamsViolation,
            Self::CharCreateServerLimit,
            Self::CharCreateAccountLimit,
            Self::CharCreateServerQueue,
            Self::CharCreateOnlyExisting,
            Self::CharCreateExpansion,
            Self::CharDeleteInProgress,
            Self::CharDeleteSuccess,
            Self::CharDeleteFailed,
            Self::CharDeleteFailedLockedForTransfer,
            Self::CharDeleteFailedGuildLeader,
            Self::CharDeleteFailedArenaCaptain,
            Self::CharLoginInProgress,
            Self::CharLoginSuccess,
            Self::CharLoginNoWorld,
            Self::CharLoginDuplicateCharacter,
            Self::CharLoginNoInstances,
            Self::CharLoginFailed,
            Self::CharLoginDisabled,
            Self::CharLoginNoCharacter,
            Self::CharLoginLockedForTransfer,
            Self::CharLoginLockedByBilling,
            Self::CharNameSuccess,
            Self::CharNameFailure,
            Self::CharNameNoName,
            Self::CharNameTooShort,
            Self::CharNameTooLong,
            Self::CharNameInvalidCharacter,
            Self::CharNameMixedLanguages,
            Self::CharNameProfane,
            Self::CharNameReserved,
            Self::CharNameInvalidApostrophe,
            Self::CharNameMultipleApostrophes,
            Self::CharNameThreeConsecutive,
            Self::CharNameInvalidSpace,
            Self::CharNameConsecutiveSpaces,
            Self::CharNameRussianConsecutiveSilentCharacters,
            Self::CharNameRussianSilentCharacterAtBeginningOrEnd,
            Self::CharNameDeclensionDoesntMatchBaseName,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
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
            58 => Ok(Self::CharDeleteInProgress),
            59 => Ok(Self::CharDeleteSuccess),
            60 => Ok(Self::CharDeleteFailed),
            61 => Ok(Self::CharDeleteFailedLockedForTransfer),
            62 => Ok(Self::CharDeleteFailedGuildLeader),
            63 => Ok(Self::CharDeleteFailedArenaCaptain),
            64 => Ok(Self::CharLoginInProgress),
            65 => Ok(Self::CharLoginSuccess),
            66 => Ok(Self::CharLoginNoWorld),
            67 => Ok(Self::CharLoginDuplicateCharacter),
            68 => Ok(Self::CharLoginNoInstances),
            69 => Ok(Self::CharLoginFailed),
            70 => Ok(Self::CharLoginDisabled),
            71 => Ok(Self::CharLoginNoCharacter),
            72 => Ok(Self::CharLoginLockedForTransfer),
            73 => Ok(Self::CharLoginLockedByBilling),
            74 => Ok(Self::CharNameSuccess),
            75 => Ok(Self::CharNameFailure),
            76 => Ok(Self::CharNameNoName),
            77 => Ok(Self::CharNameTooShort),
            78 => Ok(Self::CharNameTooLong),
            79 => Ok(Self::CharNameInvalidCharacter),
            80 => Ok(Self::CharNameMixedLanguages),
            81 => Ok(Self::CharNameProfane),
            82 => Ok(Self::CharNameReserved),
            83 => Ok(Self::CharNameInvalidApostrophe),
            84 => Ok(Self::CharNameMultipleApostrophes),
            85 => Ok(Self::CharNameThreeConsecutive),
            86 => Ok(Self::CharNameInvalidSpace),
            87 => Ok(Self::CharNameConsecutiveSpaces),
            88 => Ok(Self::CharNameRussianConsecutiveSilentCharacters),
            89 => Ok(Self::CharNameRussianSilentCharacterAtBeginningOrEnd),
            90 => Ok(Self::CharNameDeclensionDoesntMatchBaseName),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl WorldResult {
    pub const fn as_test_case_value(&self) -> &'static str {
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

const NAME: &str = "WorldResult";

impl std::fmt::Display for WorldResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ResponseSuccess => "ResponseSuccess",
            Self::ResponseFailure => "ResponseFailure",
            Self::ResponseCancelled => "ResponseCancelled",
            Self::ResponseDisconnected => "ResponseDisconnected",
            Self::ResponseFailedToConnect => "ResponseFailedToConnect",
            Self::ResponseConnected => "ResponseConnected",
            Self::ResponseVersionMismatch => "ResponseVersionMismatch",
            Self::CstatusConnecting => "CstatusConnecting",
            Self::CstatusNegotiatingSecurity => "CstatusNegotiatingSecurity",
            Self::CstatusNegotiationComplete => "CstatusNegotiationComplete",
            Self::CstatusNegotiationFailed => "CstatusNegotiationFailed",
            Self::CstatusAuthenticating => "CstatusAuthenticating",
            Self::AuthOk => "AuthOk",
            Self::AuthFailed => "AuthFailed",
            Self::AuthReject => "AuthReject",
            Self::AuthBadServerProof => "AuthBadServerProof",
            Self::AuthUnavailable => "AuthUnavailable",
            Self::AuthSystemError => "AuthSystemError",
            Self::AuthBillingError => "AuthBillingError",
            Self::AuthBillingExpired => "AuthBillingExpired",
            Self::AuthVersionMismatch => "AuthVersionMismatch",
            Self::AuthUnknownAccount => "AuthUnknownAccount",
            Self::AuthIncorrectPassword => "AuthIncorrectPassword",
            Self::AuthSessionExpired => "AuthSessionExpired",
            Self::AuthServerShuttingDown => "AuthServerShuttingDown",
            Self::AuthAlreadyLoggingIn => "AuthAlreadyLoggingIn",
            Self::AuthLoginServerNotFound => "AuthLoginServerNotFound",
            Self::AuthWaitQueue => "AuthWaitQueue",
            Self::AuthBanned => "AuthBanned",
            Self::AuthAlreadyOnline => "AuthAlreadyOnline",
            Self::AuthNoTime => "AuthNoTime",
            Self::AuthDbBusy => "AuthDbBusy",
            Self::AuthSuspended => "AuthSuspended",
            Self::AuthParentalControl => "AuthParentalControl",
            Self::AuthLockedEnforced => "AuthLockedEnforced",
            Self::RealmListInProgress => "RealmListInProgress",
            Self::RealmListSuccess => "RealmListSuccess",
            Self::RealmListFailed => "RealmListFailed",
            Self::RealmListInvalid => "RealmListInvalid",
            Self::RealmListRealmNotFound => "RealmListRealmNotFound",
            Self::AccountCreateInProgress => "AccountCreateInProgress",
            Self::AccountCreateSuccess => "AccountCreateSuccess",
            Self::AccountCreateFailed => "AccountCreateFailed",
            Self::CharListRetrieving => "CharListRetrieving",
            Self::CharListRetrieved => "CharListRetrieved",
            Self::CharListFailed => "CharListFailed",
            Self::CharCreateInProgress => "CharCreateInProgress",
            Self::CharCreateSuccess => "CharCreateSuccess",
            Self::CharCreateError => "CharCreateError",
            Self::CharCreateFailed => "CharCreateFailed",
            Self::CharCreateNameInUse => "CharCreateNameInUse",
            Self::CharCreateDisabled => "CharCreateDisabled",
            Self::CharCreatePvpTeamsViolation => "CharCreatePvpTeamsViolation",
            Self::CharCreateServerLimit => "CharCreateServerLimit",
            Self::CharCreateAccountLimit => "CharCreateAccountLimit",
            Self::CharCreateServerQueue => "CharCreateServerQueue",
            Self::CharCreateOnlyExisting => "CharCreateOnlyExisting",
            Self::CharCreateExpansion => "CharCreateExpansion",
            Self::CharDeleteInProgress => "CharDeleteInProgress",
            Self::CharDeleteSuccess => "CharDeleteSuccess",
            Self::CharDeleteFailed => "CharDeleteFailed",
            Self::CharDeleteFailedLockedForTransfer => "CharDeleteFailedLockedForTransfer",
            Self::CharDeleteFailedGuildLeader => "CharDeleteFailedGuildLeader",
            Self::CharDeleteFailedArenaCaptain => "CharDeleteFailedArenaCaptain",
            Self::CharLoginInProgress => "CharLoginInProgress",
            Self::CharLoginSuccess => "CharLoginSuccess",
            Self::CharLoginNoWorld => "CharLoginNoWorld",
            Self::CharLoginDuplicateCharacter => "CharLoginDuplicateCharacter",
            Self::CharLoginNoInstances => "CharLoginNoInstances",
            Self::CharLoginFailed => "CharLoginFailed",
            Self::CharLoginDisabled => "CharLoginDisabled",
            Self::CharLoginNoCharacter => "CharLoginNoCharacter",
            Self::CharLoginLockedForTransfer => "CharLoginLockedForTransfer",
            Self::CharLoginLockedByBilling => "CharLoginLockedByBilling",
            Self::CharNameSuccess => "CharNameSuccess",
            Self::CharNameFailure => "CharNameFailure",
            Self::CharNameNoName => "CharNameNoName",
            Self::CharNameTooShort => "CharNameTooShort",
            Self::CharNameTooLong => "CharNameTooLong",
            Self::CharNameInvalidCharacter => "CharNameInvalidCharacter",
            Self::CharNameMixedLanguages => "CharNameMixedLanguages",
            Self::CharNameProfane => "CharNameProfane",
            Self::CharNameReserved => "CharNameReserved",
            Self::CharNameInvalidApostrophe => "CharNameInvalidApostrophe",
            Self::CharNameMultipleApostrophes => "CharNameMultipleApostrophes",
            Self::CharNameThreeConsecutive => "CharNameThreeConsecutive",
            Self::CharNameInvalidSpace => "CharNameInvalidSpace",
            Self::CharNameConsecutiveSpaces => "CharNameConsecutiveSpaces",
            Self::CharNameRussianConsecutiveSilentCharacters => "CharNameRussianConsecutiveSilentCharacters",
            Self::CharNameRussianSilentCharacterAtBeginningOrEnd => "CharNameRussianSilentCharacterAtBeginningOrEnd",
            Self::CharNameDeclensionDoesntMatchBaseName => "CharNameDeclensionDoesntMatchBaseName",
        })
    }
}

impl TryFrom<u8> for WorldResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for WorldResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for WorldResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for WorldResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for WorldResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for WorldResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for WorldResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for WorldResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for WorldResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

