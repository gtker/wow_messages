/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/world_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/world_result.wowm#L1):
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
///     REALM_LIST_IN_PROGRESS = 0x22;
///     REALM_LIST_SUCCESS = 0x23;
///     REALM_LIST_FAILED = 0x24;
///     REALM_LIST_INVALID = 0x25;
///     REALM_LIST_REALM_NOT_FOUND = 0x26;
///     ACCOUNT_CREATE_IN_PROGRESS = 0x27;
///     ACCOUNT_CREATE_SUCCESS = 0x28;
///     ACCOUNT_CREATE_FAILED = 0x29;
///     CHAR_LIST_RETRIEVING = 0x2A;
///     CHAR_LIST_RETRIEVED = 0x2B;
///     CHAR_LIST_FAILED = 0x2C;
///     CHAR_CREATE_IN_PROGRESS = 0x2D;
///     CHAR_CREATE_SUCCESS = 0x2E;
///     CHAR_CREATE_ERROR = 0x2F;
///     CHAR_CREATE_FAILED = 0x30;
///     CHAR_CREATE_NAME_IN_USE = 0x31;
///     CHAR_CREATE_DISABLED = 0x32;
///     CHAR_CREATE_PVP_TEAMS_VIOLATION = 0x33;
///     CHAR_CREATE_SERVER_LIMIT = 0x34;
///     CHAR_CREATE_ACCOUNT_LIMIT = 0x35;
///     CHAR_CREATE_SERVER_QUEUE = 0x36;
///     CHAR_CREATE_ONLY_EXISTING = 0x37;
///     CHAR_DELETE_IN_PROGRESS = 0x38;
///     CHAR_DELETE_SUCCESS = 0x39;
///     CHAR_DELETE_FAILED = 0x3A;
///     CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER = 0x3B;
///     CHAR_LOGIN_IN_PROGRESS = 0x3C;
///     CHAR_LOGIN_SUCCESS = 0x3D;
///     CHAR_LOGIN_NO_WORLD = 0x3E;
///     CHAR_LOGIN_DUPLICATE_CHARACTER = 0x3F;
///     CHAR_LOGIN_NO_INSTANCES = 0x40;
///     CHAR_LOGIN_FAILED = 0x41;
///     CHAR_LOGIN_DISABLED = 0x42;
///     CHAR_LOGIN_NO_CHARACTER = 0x43;
///     CHAR_LOGIN_LOCKED_FOR_TRANSFER = 0x44;
///     CHAR_NAME_NO_NAME = 0x45;
///     CHAR_NAME_TOO_SHORT = 0x46;
///     CHAR_NAME_TOO_LONG = 0x47;
///     CHAR_NAME_ONLY_LETTERS = 0x48;
///     CHAR_NAME_MIXED_LANGUAGES = 0x49;
///     CHAR_NAME_PROFANE = 0x4A;
///     CHAR_NAME_RESERVED = 0x4B;
///     CHAR_NAME_INVALID_APOSTROPHE = 0x4C;
///     CHAR_NAME_MULTIPLE_APOSTROPHES = 0x4D;
///     CHAR_NAME_THREE_CONSECUTIVE = 0x4E;
///     CHAR_NAME_INVALID_SPACE = 0x4F;
///     CHAR_NAME_SUCCESS = 0x50;
///     CHAR_NAME_FAILURE = 0x51;
/// }
/// ```
#[derive(Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum WorldResult {
    /// Shows `Success` with `Okay` button.
    #[default]
    ResponseSuccess,
    /// Shows `Failure` with `Okay` button.
    ResponseFailure,
    /// Shows `Cancelled` with `Okay` button.
    ResponseCancelled,
    /// Shows `Disconnected from server` with `Okay` button.
    ResponseDisconnected,
    /// Shows `Failed to connect. Please be sure that your computer is currently connected to the internet, and that no security features on your system might be blocking traffic. See www.wow-europe.com/en/support for more information.` with `Okay` button.
    ResponseFailedToConnect,
    /// Shows `Connected` with `Okay` button.
    ResponseConnected,
    /// Shows `Wrong client version` with `Okay` button.
    ResponseVersionMismatch,
    /// Shows `Connecting to server...` with `Okay` button.
    CstatusConnecting,
    /// Shows `Negotiating security` with `Okay` button.
    CstatusNegotiatingSecurity,
    /// Shows `Security negotiation complete` with `Okay` button.
    CstatusNegotiationComplete,
    /// Shows `Security negotiation failed` with `Okay` button.
    CstatusNegotiationFailed,
    /// Shows `Authenticating` with `Okay` button.
    CstatusAuthenticating,
    /// Shows `Authentication Successful` with `Okay` button.
    AuthOk,
    /// Shows `Authentication failed` with `Okay` button.
    AuthFailed,
    /// Shows `Login unavailable - Please contact Technical Support at http://www.wow-europe.com/en/support/` with `Okay` button.
    AuthReject,
    /// Shows `Server is not valid` with `Okay` button.
    AuthBadServerProof,
    /// Shows `System unavailable - Please try again later` with `Okay` button.
    AuthUnavailable,
    AuthSystemError,
    /// Shows `Billing system error` with `Okay` button.
    AuthBillingError,
    /// Shows `Account billing has expired` with `Okay` button.
    AuthBillingExpired,
    /// Shows `Wrong client version` with `Okay` button.
    AuthVersionMismatch,
    /// Shows `Unknown account` with `Okay` button.
    AuthUnknownAccount,
    /// Shows `Incorrect Password` with `Okay` button.
    AuthIncorrectPassword,
    /// Shows `Session Expired` with `Okay` button.
    AuthSessionExpired,
    AuthServerShuttingDown,
    /// Shows `Already Logging In` with `Okay` button.
    AuthAlreadyLoggingIn,
    /// Shows `Invalid Login Server` with `Okay` button.
    AuthLoginServerNotFound,
    /// If this is sent without a `queue_position` field it will either reuse the one from before or use 0.
    AuthWaitQueue,
    /// Shows `This account has been banned for violating the Terms of Use Agreement- http://www.wow-europe.com/en/lega. Please contact our GM department at http://www.wow-europe.com/en/support/ for more information.` with `Okay` button.
    AuthBanned,
    /// Shows `This character is still logged on. If this character is not logged in and you continue to experience this issue for more than 15 minutes, please contact our Technical Support Department at http://www.wow-europe.com/en/support/` with `Okay` button.
    AuthAlreadyOnline,
    /// Shows `Your World of Warcraft subscription has expired. You will need to reactivate your account. To do so, please visit http://signup.wow-europe.com/ for more information.` with `Okay` button.
    AuthNoTime,
    /// Shows `This session has timed out. Please try again at a later time or check the status of our WoW realms at http://www.wow-europe.com/en/serverstatus` with `Okay` button.
    AuthDbBusy,
    /// Shows 'This account has been temporarily suspended for violating the Terms of Use Agreement - `http://www.wow-europe.com/en/legal`. Please contact our GM department at `http://www.wow-europe.com/en/support/` for more information.' with 'Okay' button.
    AuthSuspended,
    /// Shows 'Access to this account has been blocked by parental controls. Your settings may be changed in your preferences at `http://www.worldofwarcraft.com`.' with 'Okay' button.
    AuthParentalControl,
    /// Shows 'Retrieving realm list' with 'Okay' button.
    RealmListInProgress,
    /// Shows 'Realm list retrieved' with 'Okay' button.
    RealmListSuccess,
    /// Shows 'Unable to connect to realm list server' with 'Okay' button.
    RealmListFailed,
    /// Shows 'Invalid realm list' with 'Okay' button.
    RealmListInvalid,
    /// Shows 'The game server you have chosen is currently down. Use the Change Realm button to choose another Realm. Check `http://www.wow-europe.com/en/serverstatus` for current server status.' with 'Okay' button.
    RealmListRealmNotFound,
    /// Shows 'Creating account' with 'Okay' button.
    AccountCreateInProgress,
    /// Shows 'Account created' with 'Okay' button.
    AccountCreateSuccess,
    /// Shows 'Account creation failed' with 'Okay' button.
    AccountCreateFailed,
    /// Shows 'Retrieving character list' with 'Okay' button.
    CharListRetrieving,
    /// Shows 'Character list retrieved' with 'Okay' button.
    CharListRetrieved,
    /// Shows 'Error retrieving character list' with 'Okay' button.
    CharListFailed,
    /// Shows 'Creating character' with 'Okay' button.
    CharCreateInProgress,
    CharCreateSuccess,
    CharCreateError,
    /// Shows 'Character creation failed' with 'Okay' button.
    CharCreateFailed,
    /// Shows 'That name is unavailable' with 'Okay' button.
    CharCreateNameInUse,
    /// Shows 'Creation of that race and/or class is currently disabled.' with 'Okay' button.
    CharCreateDisabled,
    /// Shows 'You cannot have both a Horde and an Alliance character on the same PvP realm' with 'Okay' button.
    CharCreatePvpTeamsViolation,
    CharCreateServerLimit,
    /// Shows 'You already have the maximum number of characters allowed on this account.' with 'Okay' button.
    CharCreateAccountLimit,
    CharCreateServerQueue,
    /// Shows 'Only players who already have characters on this realm are currently allowed to create characters.' with 'Okay' button.
    CharCreateOnlyExisting,
    /// Shows 'Deleting character' with 'Okay' button.
    CharDeleteInProgress,
    /// Shows 'Character deleted' with 'Okay' button.
    CharDeleteSuccess,
    /// Shows 'Character deletion failed' with 'Okay' button.
    CharDeleteFailed,
    /// Shows 'Your character is currently locked as part of the paid character transfer process.' with 'Okay' button.
    CharDeleteFailedLockedForTransfer,
    /// Shows 'Entering the World of Warcraft' with 'Okay' button.
    CharLoginInProgress,
    /// Shows 'Login successful' with 'Okay' button.
    CharLoginSuccess,
    /// Shows 'World server is down' with 'Okay' button.
    CharLoginNoWorld,
    /// Shows 'A character with that name already exists' with 'Okay' button.
    CharLoginDuplicateCharacter,
    /// Shows 'No instance servers are available' with 'Okay' button.
    CharLoginNoInstances,
    /// Shows 'Login failed' with 'Okay' button.
    CharLoginFailed,
    /// Shows 'Login for that race, class or character is currently disabled.' with 'Okay' button.
    CharLoginDisabled,
    /// Shows 'Character not found' with 'Okay' button.
    CharLoginNoCharacter,
    /// Shows 'Your character is currently locked as part of the paid character transfer process.' with 'Okay' button.
    CharLoginLockedForTransfer,
    /// Shows 'Enter a name for your character' with 'Okay' button.
    CharNameNoName,
    /// Shows 'Names must be at least 2 characters' with 'Okay' button.
    CharNameTooShort,
    /// Shows 'Names must be no more than 12 characters' with 'Okay' button.
    CharNameTooLong,
    /// Shows 'Names can only contain letters' with 'Okay' button.
    CharNameOnlyLetters,
    /// Shows 'Names must contain only one language' with 'Okay' button.
    CharNameMixedLanguages,
    /// Shows 'That name contains profanity' with 'Okay' button.
    CharNameProfane,
    /// Shows 'That name is unavailable' with 'Okay' button.
    CharNameReserved,
    /// Shows 'You cannot use an apostrophe as the first or last character of your name' with 'Okay' button.
    CharNameInvalidApostrophe,
    /// Shows 'You can only have one apostrophe' with 'Okay' button.
    CharNameMultipleApostrophes,
    /// Shows 'You cannot use the same letter three times consecutively' with 'Okay' button.
    CharNameThreeConsecutive,
    /// Shows 'You cannot use a space as the first or last character of your name' with 'Okay' button.
    CharNameInvalidSpace,
    /// Shows an empty box with 'Okay' button.
    CharNameSuccess,
    /// Shows 'Invalid character name' with 'Okay' button.
    CharNameFailure,
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
            Self::RealmListInProgress => 0x22,
            Self::RealmListSuccess => 0x23,
            Self::RealmListFailed => 0x24,
            Self::RealmListInvalid => 0x25,
            Self::RealmListRealmNotFound => 0x26,
            Self::AccountCreateInProgress => 0x27,
            Self::AccountCreateSuccess => 0x28,
            Self::AccountCreateFailed => 0x29,
            Self::CharListRetrieving => 0x2a,
            Self::CharListRetrieved => 0x2b,
            Self::CharListFailed => 0x2c,
            Self::CharCreateInProgress => 0x2d,
            Self::CharCreateSuccess => 0x2e,
            Self::CharCreateError => 0x2f,
            Self::CharCreateFailed => 0x30,
            Self::CharCreateNameInUse => 0x31,
            Self::CharCreateDisabled => 0x32,
            Self::CharCreatePvpTeamsViolation => 0x33,
            Self::CharCreateServerLimit => 0x34,
            Self::CharCreateAccountLimit => 0x35,
            Self::CharCreateServerQueue => 0x36,
            Self::CharCreateOnlyExisting => 0x37,
            Self::CharDeleteInProgress => 0x38,
            Self::CharDeleteSuccess => 0x39,
            Self::CharDeleteFailed => 0x3a,
            Self::CharDeleteFailedLockedForTransfer => 0x3b,
            Self::CharLoginInProgress => 0x3c,
            Self::CharLoginSuccess => 0x3d,
            Self::CharLoginNoWorld => 0x3e,
            Self::CharLoginDuplicateCharacter => 0x3f,
            Self::CharLoginNoInstances => 0x40,
            Self::CharLoginFailed => 0x41,
            Self::CharLoginDisabled => 0x42,
            Self::CharLoginNoCharacter => 0x43,
            Self::CharLoginLockedForTransfer => 0x44,
            Self::CharNameNoName => 0x45,
            Self::CharNameTooShort => 0x46,
            Self::CharNameTooLong => 0x47,
            Self::CharNameOnlyLetters => 0x48,
            Self::CharNameMixedLanguages => 0x49,
            Self::CharNameProfane => 0x4a,
            Self::CharNameReserved => 0x4b,
            Self::CharNameInvalidApostrophe => 0x4c,
            Self::CharNameMultipleApostrophes => 0x4d,
            Self::CharNameThreeConsecutive => 0x4e,
            Self::CharNameInvalidSpace => 0x4f,
            Self::CharNameSuccess => 0x50,
            Self::CharNameFailure => 0x51,
        }
    }

    pub const fn variants() -> [Self; 82] {
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
            Self::CharDeleteInProgress,
            Self::CharDeleteSuccess,
            Self::CharDeleteFailed,
            Self::CharDeleteFailedLockedForTransfer,
            Self::CharLoginInProgress,
            Self::CharLoginSuccess,
            Self::CharLoginNoWorld,
            Self::CharLoginDuplicateCharacter,
            Self::CharLoginNoInstances,
            Self::CharLoginFailed,
            Self::CharLoginDisabled,
            Self::CharLoginNoCharacter,
            Self::CharLoginLockedForTransfer,
            Self::CharNameNoName,
            Self::CharNameTooShort,
            Self::CharNameTooLong,
            Self::CharNameOnlyLetters,
            Self::CharNameMixedLanguages,
            Self::CharNameProfane,
            Self::CharNameReserved,
            Self::CharNameInvalidApostrophe,
            Self::CharNameMultipleApostrophes,
            Self::CharNameThreeConsecutive,
            Self::CharNameInvalidSpace,
            Self::CharNameSuccess,
            Self::CharNameFailure,
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
            34 => Ok(Self::RealmListInProgress),
            35 => Ok(Self::RealmListSuccess),
            36 => Ok(Self::RealmListFailed),
            37 => Ok(Self::RealmListInvalid),
            38 => Ok(Self::RealmListRealmNotFound),
            39 => Ok(Self::AccountCreateInProgress),
            40 => Ok(Self::AccountCreateSuccess),
            41 => Ok(Self::AccountCreateFailed),
            42 => Ok(Self::CharListRetrieving),
            43 => Ok(Self::CharListRetrieved),
            44 => Ok(Self::CharListFailed),
            45 => Ok(Self::CharCreateInProgress),
            46 => Ok(Self::CharCreateSuccess),
            47 => Ok(Self::CharCreateError),
            48 => Ok(Self::CharCreateFailed),
            49 => Ok(Self::CharCreateNameInUse),
            50 => Ok(Self::CharCreateDisabled),
            51 => Ok(Self::CharCreatePvpTeamsViolation),
            52 => Ok(Self::CharCreateServerLimit),
            53 => Ok(Self::CharCreateAccountLimit),
            54 => Ok(Self::CharCreateServerQueue),
            55 => Ok(Self::CharCreateOnlyExisting),
            56 => Ok(Self::CharDeleteInProgress),
            57 => Ok(Self::CharDeleteSuccess),
            58 => Ok(Self::CharDeleteFailed),
            59 => Ok(Self::CharDeleteFailedLockedForTransfer),
            60 => Ok(Self::CharLoginInProgress),
            61 => Ok(Self::CharLoginSuccess),
            62 => Ok(Self::CharLoginNoWorld),
            63 => Ok(Self::CharLoginDuplicateCharacter),
            64 => Ok(Self::CharLoginNoInstances),
            65 => Ok(Self::CharLoginFailed),
            66 => Ok(Self::CharLoginDisabled),
            67 => Ok(Self::CharLoginNoCharacter),
            68 => Ok(Self::CharLoginLockedForTransfer),
            69 => Ok(Self::CharNameNoName),
            70 => Ok(Self::CharNameTooShort),
            71 => Ok(Self::CharNameTooLong),
            72 => Ok(Self::CharNameOnlyLetters),
            73 => Ok(Self::CharNameMixedLanguages),
            74 => Ok(Self::CharNameProfane),
            75 => Ok(Self::CharNameReserved),
            76 => Ok(Self::CharNameInvalidApostrophe),
            77 => Ok(Self::CharNameMultipleApostrophes),
            78 => Ok(Self::CharNameThreeConsecutive),
            79 => Ok(Self::CharNameInvalidSpace),
            80 => Ok(Self::CharNameSuccess),
            81 => Ok(Self::CharNameFailure),
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
            Self::CharDeleteInProgress => "CHAR_DELETE_IN_PROGRESS",
            Self::CharDeleteSuccess => "CHAR_DELETE_SUCCESS",
            Self::CharDeleteFailed => "CHAR_DELETE_FAILED",
            Self::CharDeleteFailedLockedForTransfer => "CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER",
            Self::CharLoginInProgress => "CHAR_LOGIN_IN_PROGRESS",
            Self::CharLoginSuccess => "CHAR_LOGIN_SUCCESS",
            Self::CharLoginNoWorld => "CHAR_LOGIN_NO_WORLD",
            Self::CharLoginDuplicateCharacter => "CHAR_LOGIN_DUPLICATE_CHARACTER",
            Self::CharLoginNoInstances => "CHAR_LOGIN_NO_INSTANCES",
            Self::CharLoginFailed => "CHAR_LOGIN_FAILED",
            Self::CharLoginDisabled => "CHAR_LOGIN_DISABLED",
            Self::CharLoginNoCharacter => "CHAR_LOGIN_NO_CHARACTER",
            Self::CharLoginLockedForTransfer => "CHAR_LOGIN_LOCKED_FOR_TRANSFER",
            Self::CharNameNoName => "CHAR_NAME_NO_NAME",
            Self::CharNameTooShort => "CHAR_NAME_TOO_SHORT",
            Self::CharNameTooLong => "CHAR_NAME_TOO_LONG",
            Self::CharNameOnlyLetters => "CHAR_NAME_ONLY_LETTERS",
            Self::CharNameMixedLanguages => "CHAR_NAME_MIXED_LANGUAGES",
            Self::CharNameProfane => "CHAR_NAME_PROFANE",
            Self::CharNameReserved => "CHAR_NAME_RESERVED",
            Self::CharNameInvalidApostrophe => "CHAR_NAME_INVALID_APOSTROPHE",
            Self::CharNameMultipleApostrophes => "CHAR_NAME_MULTIPLE_APOSTROPHES",
            Self::CharNameThreeConsecutive => "CHAR_NAME_THREE_CONSECUTIVE",
            Self::CharNameInvalidSpace => "CHAR_NAME_INVALID_SPACE",
            Self::CharNameSuccess => "CHAR_NAME_SUCCESS",
            Self::CharNameFailure => "CHAR_NAME_FAILURE",
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
            Self::CharDeleteInProgress => "CharDeleteInProgress",
            Self::CharDeleteSuccess => "CharDeleteSuccess",
            Self::CharDeleteFailed => "CharDeleteFailed",
            Self::CharDeleteFailedLockedForTransfer => "CharDeleteFailedLockedForTransfer",
            Self::CharLoginInProgress => "CharLoginInProgress",
            Self::CharLoginSuccess => "CharLoginSuccess",
            Self::CharLoginNoWorld => "CharLoginNoWorld",
            Self::CharLoginDuplicateCharacter => "CharLoginDuplicateCharacter",
            Self::CharLoginNoInstances => "CharLoginNoInstances",
            Self::CharLoginFailed => "CharLoginFailed",
            Self::CharLoginDisabled => "CharLoginDisabled",
            Self::CharLoginNoCharacter => "CharLoginNoCharacter",
            Self::CharLoginLockedForTransfer => "CharLoginLockedForTransfer",
            Self::CharNameNoName => "CharNameNoName",
            Self::CharNameTooShort => "CharNameTooShort",
            Self::CharNameTooLong => "CharNameTooLong",
            Self::CharNameOnlyLetters => "CharNameOnlyLetters",
            Self::CharNameMixedLanguages => "CharNameMixedLanguages",
            Self::CharNameProfane => "CharNameProfane",
            Self::CharNameReserved => "CharNameReserved",
            Self::CharNameInvalidApostrophe => "CharNameInvalidApostrophe",
            Self::CharNameMultipleApostrophes => "CharNameMultipleApostrophes",
            Self::CharNameThreeConsecutive => "CharNameThreeConsecutive",
            Self::CharNameInvalidSpace => "CharNameInvalidSpace",
            Self::CharNameSuccess => "CharNameSuccess",
            Self::CharNameFailure => "CharNameFailure",
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

