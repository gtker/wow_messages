use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/world_result.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/world_result.wowm#L3):
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
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum WorldResult {
    /// Shows `Success` with `Okay` button.
    ///
    RESPONSE_SUCCESS,
    /// Shows `Failure` with `Okay` button.
    ///
    RESPONSE_FAILURE,
    /// Shows `Cancelled` with `Okay` button.
    ///
    RESPONSE_CANCELLED,
    /// Shows `Disconnected from server` with `Okay` button.
    ///
    RESPONSE_DISCONNECTED,
    /// Shows `Failed to connect. Please be sure that your computer is currently connected to the internet, and that no security features on your system might be blocking traffic. See www.wow-europe.com/en/support for more information.` with `Okay` button.
    ///
    RESPONSE_FAILED_TO_CONNECT,
    /// Shows `Connected` with `Okay` button.
    ///
    RESPONSE_CONNECTED,
    /// Shows `Wrong client version` with `Okay` button.
    ///
    RESPONSE_VERSION_MISMATCH,
    /// Shows `Connecting to server...` with `Okay` button.
    ///
    CSTATUS_CONNECTING,
    /// Shows `Negotiating security` with `Okay` button.
    ///
    CSTATUS_NEGOTIATING_SECURITY,
    /// Shows `Security negotiation complete` with `Okay` button.
    ///
    CSTATUS_NEGOTIATION_COMPLETE,
    /// Shows `Security negotiation failed` with `Okay` button.
    ///
    CSTATUS_NEGOTIATION_FAILED,
    /// Shows `Authenticating` with `Okay` button.
    ///
    CSTATUS_AUTHENTICATING,
    /// Shows `Authentication Successful` with `Okay` button.
    ///
    AUTH_OK,
    /// Shows `Authentication failed` with `Okay` button.
    ///
    AUTH_FAILED,
    /// Shows `Login unavailable - Please contact Technical Support at http://www.wow-europe.com/en/support/` with `Okay` button.
    ///
    AUTH_REJECT,
    /// Shows `Server is not valid` with `Okay` button.
    ///
    AUTH_BAD_SERVER_PROOF,
    /// Shows `System unavailable - Please try again later` with `Okay` button.
    ///
    AUTH_UNAVAILABLE,
    AUTH_SYSTEM_ERROR,
    /// Shows `Billing system error` with `Okay` button.
    ///
    AUTH_BILLING_ERROR,
    /// Shows `Account billing has expired` with `Okay` button.
    ///
    AUTH_BILLING_EXPIRED,
    /// Shows `Wrong client version` with `Okay` button.
    ///
    AUTH_VERSION_MISMATCH,
    /// Shows `Unknown account` with `Okay` button.
    ///
    AUTH_UNKNOWN_ACCOUNT,
    /// Shows `Incorrect Password` with `Okay` button.
    ///
    AUTH_INCORRECT_PASSWORD,
    /// Shows `Session Expired` with `Okay` button.
    ///
    AUTH_SESSION_EXPIRED,
    AUTH_SERVER_SHUTTING_DOWN,
    /// Shows `Already Logging In` with `Okay` button.
    ///
    AUTH_ALREADY_LOGGING_IN,
    /// Shows `Invalid Login Server` with `Okay` button.
    ///
    AUTH_LOGIN_SERVER_NOT_FOUND,
    /// If this is sent without a `queue_position` field it will either reuse the one from before or use 0.
    ///
    AUTH_WAIT_QUEUE,
    /// Shows `This account has been banned for violating the Terms of Use Agreement- http://www.wow-europe.com/en/lega. Please contact our GM department at http://www.wow-europe.com/en/support/ for more information.` with `Okay` button.
    ///
    AUTH_BANNED,
    /// Shows `This character is still logged on. If this character is not logged in and you continue to experience this issue for more than 15 minutes, please contact our Technical Support Department at http://www.wow-europe.com/en/support/` with `Okay` button.
    ///
    AUTH_ALREADY_ONLINE,
    /// Shows `Your World of Warcraft subscription has expired. You will need to reactivate your account. To do so, please visit http://signup.wow-europe.com/ for more information.` with `Okay` button.
    ///
    AUTH_NO_TIME,
    /// Shows `This session has timed out. Please try again at a later time or check the status of our WoW realms at http://www.wow-europe.com/en/serverstatus` with `Okay` button.
    ///
    AUTH_DB_BUSY,
    /// Shows 'This account has been temporarily suspended for violating the Terms of Use Agreement - `http://www.wow-europe.com/en/legal`. Please contact our GM department at `http://www.wow-europe.com/en/support/` for more information.' with 'Okay' button.
    ///
    AUTH_SUSPENDED,
    /// Shows 'Access to this account has been blocked by parental controls. Your settings may be changed in your preferences at `http://www.worldofwarcraft.com`.' with 'Okay' button.
    ///
    AUTH_PARENTAL_CONTROL,
    /// Shows 'Retrieving realm list' with 'Okay' button.
    ///
    REALM_LIST_IN_PROGRESS,
    /// Shows 'Realm list retrieved' with 'Okay' button.
    ///
    REALM_LIST_SUCCESS,
    /// Shows 'Unable to connect to realm list server' with 'Okay' button.
    ///
    REALM_LIST_FAILED,
    /// Shows 'Invalid realm list' with 'Okay' button.
    ///
    REALM_LIST_INVALID,
    /// Shows 'The game server you have chosen is currently down. Use the Change Realm button to choose another Realm. Check `http://www.wow-europe.com/en/serverstatus` for current server status.' with 'Okay' button.
    ///
    REALM_LIST_REALM_NOT_FOUND,
    /// Shows 'Creating account' with 'Okay' button.
    ///
    ACCOUNT_CREATE_IN_PROGRESS,
    /// Shows 'Account created' with 'Okay' button.
    ///
    ACCOUNT_CREATE_SUCCESS,
    /// Shows 'Account creation failed' with 'Okay' button.
    ///
    ACCOUNT_CREATE_FAILED,
    /// Shows 'Retrieving character list' with 'Okay' button.
    ///
    CHAR_LIST_RETRIEVING,
    /// Shows 'Character list retrieved' with 'Okay' button.
    ///
    CHAR_LIST_RETRIEVED,
    /// Shows 'Error retrieving character list' with 'Okay' button.
    ///
    CHAR_LIST_FAILED,
    /// Shows 'Creating character' with 'Okay' button.
    ///
    CHAR_CREATE_IN_PROGRESS,
    CHAR_CREATE_SUCCESS,
    CHAR_CREATE_ERROR,
    /// Shows 'Character creation failed' with 'Okay' button.
    ///
    CHAR_CREATE_FAILED,
    /// Shows 'That name is unavailable' with 'Okay' button.
    ///
    CHAR_CREATE_NAME_IN_USE,
    /// Shows 'Creation of that race and/or class is currently disabled.' with 'Okay' button.
    ///
    CHAR_CREATE_DISABLED,
    /// Shows 'You cannot have both a Horde and an Alliance character on the same PvP realm' with 'Okay' button.
    ///
    CHAR_CREATE_PVP_TEAMS_VIOLATION,
    CHAR_CREATE_SERVER_LIMIT,
    /// Shows 'You already have the maximum number of characters allowed on this account.' with 'Okay' button.
    ///
    CHAR_CREATE_ACCOUNT_LIMIT,
    CHAR_CREATE_SERVER_QUEUE,
    /// Shows 'Only players who already have characters on this realm are currently allowed to create characters.' with 'Okay' button.
    ///
    CHAR_CREATE_ONLY_EXISTING,
    /// Shows 'Deleting character' with 'Okay' button.
    ///
    CHAR_DELETE_IN_PROGRESS,
    /// Shows 'Character deleted' with 'Okay' button.
    ///
    CHAR_DELETE_SUCCESS,
    /// Shows 'Character deletion failed' with 'Okay' button.
    ///
    CHAR_DELETE_FAILED,
    /// Shows 'Your character is currently locked as part of the paid character transfer process.' with 'Okay' button.
    ///
    CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER,
    /// Shows 'Entering the World of Warcraft' with 'Okay' button.
    ///
    CHAR_LOGIN_IN_PROGRESS,
    /// Shows 'Login successful' with 'Okay' button.
    ///
    CHAR_LOGIN_SUCCESS,
    /// Shows 'World server is down' with 'Okay' button.
    ///
    CHAR_LOGIN_NO_WORLD,
    /// Shows 'A character with that name already exists' with 'Okay' button.
    ///
    CHAR_LOGIN_DUPLICATE_CHARACTER,
    /// Shows 'No instance servers are available' with 'Okay' button.
    ///
    CHAR_LOGIN_NO_INSTANCES,
    /// Shows 'Login failed' with 'Okay' button.
    ///
    CHAR_LOGIN_FAILED,
    /// Shows 'Login for that race, class or character is currently disabled.' with 'Okay' button.
    ///
    CHAR_LOGIN_DISABLED,
    /// Shows 'Character not found' with 'Okay' button.
    ///
    CHAR_LOGIN_NO_CHARACTER,
    /// Shows 'Your character is currently locked as part of the paid character transfer process.' with 'Okay' button.
    ///
    CHAR_LOGIN_LOCKED_FOR_TRANSFER,
    /// Shows 'Enter a name for your character' with 'Okay' button.
    ///
    CHAR_NAME_NO_NAME,
    /// Shows 'Names must be at least 2 characters' with 'Okay' button.
    ///
    CHAR_NAME_TOO_SHORT,
    /// Shows 'Names must be no more than 12 characters' with 'Okay' button.
    ///
    CHAR_NAME_TOO_LONG,
    /// Shows 'Names can only contain letters' with 'Okay' button.
    ///
    CHAR_NAME_ONLY_LETTERS,
    /// Shows 'Names must contain only one language' with 'Okay' button.
    ///
    CHAR_NAME_MIXED_LANGUAGES,
    /// Shows 'That name contains profanity' with 'Okay' button.
    ///
    CHAR_NAME_PROFANE,
    /// Shows 'That name is unavailable' with 'Okay' button.
    ///
    CHAR_NAME_RESERVED,
    /// Shows 'You cannot use an apostrophe as the first or last character of your name' with 'Okay' button.
    ///
    CHAR_NAME_INVALID_APOSTROPHE,
    /// Shows 'You can only have one apostrophe' with 'Okay' button.
    ///
    CHAR_NAME_MULTIPLE_APOSTROPHES,
    /// Shows 'You cannot use the same letter three times consecutively' with 'Okay' button.
    ///
    CHAR_NAME_THREE_CONSECUTIVE,
    /// Shows 'You cannot use a space as the first or last character of your name' with 'Okay' button.
    ///
    CHAR_NAME_INVALID_SPACE,
    /// Shows an empty box with 'Okay' button.
    ///
    CHAR_NAME_SUCCESS,
    /// Shows 'Invalid character name' with 'Okay' button.
    ///
    CHAR_NAME_FAILURE,
}

impl WorldResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::RESPONSE_SUCCESS => 0x0,
            Self::RESPONSE_FAILURE => 0x1,
            Self::RESPONSE_CANCELLED => 0x2,
            Self::RESPONSE_DISCONNECTED => 0x3,
            Self::RESPONSE_FAILED_TO_CONNECT => 0x4,
            Self::RESPONSE_CONNECTED => 0x5,
            Self::RESPONSE_VERSION_MISMATCH => 0x6,
            Self::CSTATUS_CONNECTING => 0x7,
            Self::CSTATUS_NEGOTIATING_SECURITY => 0x8,
            Self::CSTATUS_NEGOTIATION_COMPLETE => 0x9,
            Self::CSTATUS_NEGOTIATION_FAILED => 0xa,
            Self::CSTATUS_AUTHENTICATING => 0xb,
            Self::AUTH_OK => 0xc,
            Self::AUTH_FAILED => 0xd,
            Self::AUTH_REJECT => 0xe,
            Self::AUTH_BAD_SERVER_PROOF => 0xf,
            Self::AUTH_UNAVAILABLE => 0x10,
            Self::AUTH_SYSTEM_ERROR => 0x11,
            Self::AUTH_BILLING_ERROR => 0x12,
            Self::AUTH_BILLING_EXPIRED => 0x13,
            Self::AUTH_VERSION_MISMATCH => 0x14,
            Self::AUTH_UNKNOWN_ACCOUNT => 0x15,
            Self::AUTH_INCORRECT_PASSWORD => 0x16,
            Self::AUTH_SESSION_EXPIRED => 0x17,
            Self::AUTH_SERVER_SHUTTING_DOWN => 0x18,
            Self::AUTH_ALREADY_LOGGING_IN => 0x19,
            Self::AUTH_LOGIN_SERVER_NOT_FOUND => 0x1a,
            Self::AUTH_WAIT_QUEUE => 0x1b,
            Self::AUTH_BANNED => 0x1c,
            Self::AUTH_ALREADY_ONLINE => 0x1d,
            Self::AUTH_NO_TIME => 0x1e,
            Self::AUTH_DB_BUSY => 0x1f,
            Self::AUTH_SUSPENDED => 0x20,
            Self::AUTH_PARENTAL_CONTROL => 0x21,
            Self::REALM_LIST_IN_PROGRESS => 0x22,
            Self::REALM_LIST_SUCCESS => 0x23,
            Self::REALM_LIST_FAILED => 0x24,
            Self::REALM_LIST_INVALID => 0x25,
            Self::REALM_LIST_REALM_NOT_FOUND => 0x26,
            Self::ACCOUNT_CREATE_IN_PROGRESS => 0x27,
            Self::ACCOUNT_CREATE_SUCCESS => 0x28,
            Self::ACCOUNT_CREATE_FAILED => 0x29,
            Self::CHAR_LIST_RETRIEVING => 0x2a,
            Self::CHAR_LIST_RETRIEVED => 0x2b,
            Self::CHAR_LIST_FAILED => 0x2c,
            Self::CHAR_CREATE_IN_PROGRESS => 0x2d,
            Self::CHAR_CREATE_SUCCESS => 0x2e,
            Self::CHAR_CREATE_ERROR => 0x2f,
            Self::CHAR_CREATE_FAILED => 0x30,
            Self::CHAR_CREATE_NAME_IN_USE => 0x31,
            Self::CHAR_CREATE_DISABLED => 0x32,
            Self::CHAR_CREATE_PVP_TEAMS_VIOLATION => 0x33,
            Self::CHAR_CREATE_SERVER_LIMIT => 0x34,
            Self::CHAR_CREATE_ACCOUNT_LIMIT => 0x35,
            Self::CHAR_CREATE_SERVER_QUEUE => 0x36,
            Self::CHAR_CREATE_ONLY_EXISTING => 0x37,
            Self::CHAR_DELETE_IN_PROGRESS => 0x38,
            Self::CHAR_DELETE_SUCCESS => 0x39,
            Self::CHAR_DELETE_FAILED => 0x3a,
            Self::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER => 0x3b,
            Self::CHAR_LOGIN_IN_PROGRESS => 0x3c,
            Self::CHAR_LOGIN_SUCCESS => 0x3d,
            Self::CHAR_LOGIN_NO_WORLD => 0x3e,
            Self::CHAR_LOGIN_DUPLICATE_CHARACTER => 0x3f,
            Self::CHAR_LOGIN_NO_INSTANCES => 0x40,
            Self::CHAR_LOGIN_FAILED => 0x41,
            Self::CHAR_LOGIN_DISABLED => 0x42,
            Self::CHAR_LOGIN_NO_CHARACTER => 0x43,
            Self::CHAR_LOGIN_LOCKED_FOR_TRANSFER => 0x44,
            Self::CHAR_NAME_NO_NAME => 0x45,
            Self::CHAR_NAME_TOO_SHORT => 0x46,
            Self::CHAR_NAME_TOO_LONG => 0x47,
            Self::CHAR_NAME_ONLY_LETTERS => 0x48,
            Self::CHAR_NAME_MIXED_LANGUAGES => 0x49,
            Self::CHAR_NAME_PROFANE => 0x4a,
            Self::CHAR_NAME_RESERVED => 0x4b,
            Self::CHAR_NAME_INVALID_APOSTROPHE => 0x4c,
            Self::CHAR_NAME_MULTIPLE_APOSTROPHES => 0x4d,
            Self::CHAR_NAME_THREE_CONSECUTIVE => 0x4e,
            Self::CHAR_NAME_INVALID_SPACE => 0x4f,
            Self::CHAR_NAME_SUCCESS => 0x50,
            Self::CHAR_NAME_FAILURE => 0x51,
        }
    }

}

impl Default for WorldResult {
    fn default() -> Self {
        Self::RESPONSE_SUCCESS
    }
}

impl std::fmt::Display for WorldResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RESPONSE_SUCCESS => f.write_str("RESPONSE_SUCCESS"),
            Self::RESPONSE_FAILURE => f.write_str("RESPONSE_FAILURE"),
            Self::RESPONSE_CANCELLED => f.write_str("RESPONSE_CANCELLED"),
            Self::RESPONSE_DISCONNECTED => f.write_str("RESPONSE_DISCONNECTED"),
            Self::RESPONSE_FAILED_TO_CONNECT => f.write_str("RESPONSE_FAILED_TO_CONNECT"),
            Self::RESPONSE_CONNECTED => f.write_str("RESPONSE_CONNECTED"),
            Self::RESPONSE_VERSION_MISMATCH => f.write_str("RESPONSE_VERSION_MISMATCH"),
            Self::CSTATUS_CONNECTING => f.write_str("CSTATUS_CONNECTING"),
            Self::CSTATUS_NEGOTIATING_SECURITY => f.write_str("CSTATUS_NEGOTIATING_SECURITY"),
            Self::CSTATUS_NEGOTIATION_COMPLETE => f.write_str("CSTATUS_NEGOTIATION_COMPLETE"),
            Self::CSTATUS_NEGOTIATION_FAILED => f.write_str("CSTATUS_NEGOTIATION_FAILED"),
            Self::CSTATUS_AUTHENTICATING => f.write_str("CSTATUS_AUTHENTICATING"),
            Self::AUTH_OK => f.write_str("AUTH_OK"),
            Self::AUTH_FAILED => f.write_str("AUTH_FAILED"),
            Self::AUTH_REJECT => f.write_str("AUTH_REJECT"),
            Self::AUTH_BAD_SERVER_PROOF => f.write_str("AUTH_BAD_SERVER_PROOF"),
            Self::AUTH_UNAVAILABLE => f.write_str("AUTH_UNAVAILABLE"),
            Self::AUTH_SYSTEM_ERROR => f.write_str("AUTH_SYSTEM_ERROR"),
            Self::AUTH_BILLING_ERROR => f.write_str("AUTH_BILLING_ERROR"),
            Self::AUTH_BILLING_EXPIRED => f.write_str("AUTH_BILLING_EXPIRED"),
            Self::AUTH_VERSION_MISMATCH => f.write_str("AUTH_VERSION_MISMATCH"),
            Self::AUTH_UNKNOWN_ACCOUNT => f.write_str("AUTH_UNKNOWN_ACCOUNT"),
            Self::AUTH_INCORRECT_PASSWORD => f.write_str("AUTH_INCORRECT_PASSWORD"),
            Self::AUTH_SESSION_EXPIRED => f.write_str("AUTH_SESSION_EXPIRED"),
            Self::AUTH_SERVER_SHUTTING_DOWN => f.write_str("AUTH_SERVER_SHUTTING_DOWN"),
            Self::AUTH_ALREADY_LOGGING_IN => f.write_str("AUTH_ALREADY_LOGGING_IN"),
            Self::AUTH_LOGIN_SERVER_NOT_FOUND => f.write_str("AUTH_LOGIN_SERVER_NOT_FOUND"),
            Self::AUTH_WAIT_QUEUE => f.write_str("AUTH_WAIT_QUEUE"),
            Self::AUTH_BANNED => f.write_str("AUTH_BANNED"),
            Self::AUTH_ALREADY_ONLINE => f.write_str("AUTH_ALREADY_ONLINE"),
            Self::AUTH_NO_TIME => f.write_str("AUTH_NO_TIME"),
            Self::AUTH_DB_BUSY => f.write_str("AUTH_DB_BUSY"),
            Self::AUTH_SUSPENDED => f.write_str("AUTH_SUSPENDED"),
            Self::AUTH_PARENTAL_CONTROL => f.write_str("AUTH_PARENTAL_CONTROL"),
            Self::REALM_LIST_IN_PROGRESS => f.write_str("REALM_LIST_IN_PROGRESS"),
            Self::REALM_LIST_SUCCESS => f.write_str("REALM_LIST_SUCCESS"),
            Self::REALM_LIST_FAILED => f.write_str("REALM_LIST_FAILED"),
            Self::REALM_LIST_INVALID => f.write_str("REALM_LIST_INVALID"),
            Self::REALM_LIST_REALM_NOT_FOUND => f.write_str("REALM_LIST_REALM_NOT_FOUND"),
            Self::ACCOUNT_CREATE_IN_PROGRESS => f.write_str("ACCOUNT_CREATE_IN_PROGRESS"),
            Self::ACCOUNT_CREATE_SUCCESS => f.write_str("ACCOUNT_CREATE_SUCCESS"),
            Self::ACCOUNT_CREATE_FAILED => f.write_str("ACCOUNT_CREATE_FAILED"),
            Self::CHAR_LIST_RETRIEVING => f.write_str("CHAR_LIST_RETRIEVING"),
            Self::CHAR_LIST_RETRIEVED => f.write_str("CHAR_LIST_RETRIEVED"),
            Self::CHAR_LIST_FAILED => f.write_str("CHAR_LIST_FAILED"),
            Self::CHAR_CREATE_IN_PROGRESS => f.write_str("CHAR_CREATE_IN_PROGRESS"),
            Self::CHAR_CREATE_SUCCESS => f.write_str("CHAR_CREATE_SUCCESS"),
            Self::CHAR_CREATE_ERROR => f.write_str("CHAR_CREATE_ERROR"),
            Self::CHAR_CREATE_FAILED => f.write_str("CHAR_CREATE_FAILED"),
            Self::CHAR_CREATE_NAME_IN_USE => f.write_str("CHAR_CREATE_NAME_IN_USE"),
            Self::CHAR_CREATE_DISABLED => f.write_str("CHAR_CREATE_DISABLED"),
            Self::CHAR_CREATE_PVP_TEAMS_VIOLATION => f.write_str("CHAR_CREATE_PVP_TEAMS_VIOLATION"),
            Self::CHAR_CREATE_SERVER_LIMIT => f.write_str("CHAR_CREATE_SERVER_LIMIT"),
            Self::CHAR_CREATE_ACCOUNT_LIMIT => f.write_str("CHAR_CREATE_ACCOUNT_LIMIT"),
            Self::CHAR_CREATE_SERVER_QUEUE => f.write_str("CHAR_CREATE_SERVER_QUEUE"),
            Self::CHAR_CREATE_ONLY_EXISTING => f.write_str("CHAR_CREATE_ONLY_EXISTING"),
            Self::CHAR_DELETE_IN_PROGRESS => f.write_str("CHAR_DELETE_IN_PROGRESS"),
            Self::CHAR_DELETE_SUCCESS => f.write_str("CHAR_DELETE_SUCCESS"),
            Self::CHAR_DELETE_FAILED => f.write_str("CHAR_DELETE_FAILED"),
            Self::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER => f.write_str("CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER"),
            Self::CHAR_LOGIN_IN_PROGRESS => f.write_str("CHAR_LOGIN_IN_PROGRESS"),
            Self::CHAR_LOGIN_SUCCESS => f.write_str("CHAR_LOGIN_SUCCESS"),
            Self::CHAR_LOGIN_NO_WORLD => f.write_str("CHAR_LOGIN_NO_WORLD"),
            Self::CHAR_LOGIN_DUPLICATE_CHARACTER => f.write_str("CHAR_LOGIN_DUPLICATE_CHARACTER"),
            Self::CHAR_LOGIN_NO_INSTANCES => f.write_str("CHAR_LOGIN_NO_INSTANCES"),
            Self::CHAR_LOGIN_FAILED => f.write_str("CHAR_LOGIN_FAILED"),
            Self::CHAR_LOGIN_DISABLED => f.write_str("CHAR_LOGIN_DISABLED"),
            Self::CHAR_LOGIN_NO_CHARACTER => f.write_str("CHAR_LOGIN_NO_CHARACTER"),
            Self::CHAR_LOGIN_LOCKED_FOR_TRANSFER => f.write_str("CHAR_LOGIN_LOCKED_FOR_TRANSFER"),
            Self::CHAR_NAME_NO_NAME => f.write_str("CHAR_NAME_NO_NAME"),
            Self::CHAR_NAME_TOO_SHORT => f.write_str("CHAR_NAME_TOO_SHORT"),
            Self::CHAR_NAME_TOO_LONG => f.write_str("CHAR_NAME_TOO_LONG"),
            Self::CHAR_NAME_ONLY_LETTERS => f.write_str("CHAR_NAME_ONLY_LETTERS"),
            Self::CHAR_NAME_MIXED_LANGUAGES => f.write_str("CHAR_NAME_MIXED_LANGUAGES"),
            Self::CHAR_NAME_PROFANE => f.write_str("CHAR_NAME_PROFANE"),
            Self::CHAR_NAME_RESERVED => f.write_str("CHAR_NAME_RESERVED"),
            Self::CHAR_NAME_INVALID_APOSTROPHE => f.write_str("CHAR_NAME_INVALID_APOSTROPHE"),
            Self::CHAR_NAME_MULTIPLE_APOSTROPHES => f.write_str("CHAR_NAME_MULTIPLE_APOSTROPHES"),
            Self::CHAR_NAME_THREE_CONSECUTIVE => f.write_str("CHAR_NAME_THREE_CONSECUTIVE"),
            Self::CHAR_NAME_INVALID_SPACE => f.write_str("CHAR_NAME_INVALID_SPACE"),
            Self::CHAR_NAME_SUCCESS => f.write_str("CHAR_NAME_SUCCESS"),
            Self::CHAR_NAME_FAILURE => f.write_str("CHAR_NAME_FAILURE"),
        }
    }
}

impl TryFrom<u8> for WorldResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::RESPONSE_SUCCESS),
            1 => Ok(Self::RESPONSE_FAILURE),
            2 => Ok(Self::RESPONSE_CANCELLED),
            3 => Ok(Self::RESPONSE_DISCONNECTED),
            4 => Ok(Self::RESPONSE_FAILED_TO_CONNECT),
            5 => Ok(Self::RESPONSE_CONNECTED),
            6 => Ok(Self::RESPONSE_VERSION_MISMATCH),
            7 => Ok(Self::CSTATUS_CONNECTING),
            8 => Ok(Self::CSTATUS_NEGOTIATING_SECURITY),
            9 => Ok(Self::CSTATUS_NEGOTIATION_COMPLETE),
            10 => Ok(Self::CSTATUS_NEGOTIATION_FAILED),
            11 => Ok(Self::CSTATUS_AUTHENTICATING),
            12 => Ok(Self::AUTH_OK),
            13 => Ok(Self::AUTH_FAILED),
            14 => Ok(Self::AUTH_REJECT),
            15 => Ok(Self::AUTH_BAD_SERVER_PROOF),
            16 => Ok(Self::AUTH_UNAVAILABLE),
            17 => Ok(Self::AUTH_SYSTEM_ERROR),
            18 => Ok(Self::AUTH_BILLING_ERROR),
            19 => Ok(Self::AUTH_BILLING_EXPIRED),
            20 => Ok(Self::AUTH_VERSION_MISMATCH),
            21 => Ok(Self::AUTH_UNKNOWN_ACCOUNT),
            22 => Ok(Self::AUTH_INCORRECT_PASSWORD),
            23 => Ok(Self::AUTH_SESSION_EXPIRED),
            24 => Ok(Self::AUTH_SERVER_SHUTTING_DOWN),
            25 => Ok(Self::AUTH_ALREADY_LOGGING_IN),
            26 => Ok(Self::AUTH_LOGIN_SERVER_NOT_FOUND),
            27 => Ok(Self::AUTH_WAIT_QUEUE),
            28 => Ok(Self::AUTH_BANNED),
            29 => Ok(Self::AUTH_ALREADY_ONLINE),
            30 => Ok(Self::AUTH_NO_TIME),
            31 => Ok(Self::AUTH_DB_BUSY),
            32 => Ok(Self::AUTH_SUSPENDED),
            33 => Ok(Self::AUTH_PARENTAL_CONTROL),
            34 => Ok(Self::REALM_LIST_IN_PROGRESS),
            35 => Ok(Self::REALM_LIST_SUCCESS),
            36 => Ok(Self::REALM_LIST_FAILED),
            37 => Ok(Self::REALM_LIST_INVALID),
            38 => Ok(Self::REALM_LIST_REALM_NOT_FOUND),
            39 => Ok(Self::ACCOUNT_CREATE_IN_PROGRESS),
            40 => Ok(Self::ACCOUNT_CREATE_SUCCESS),
            41 => Ok(Self::ACCOUNT_CREATE_FAILED),
            42 => Ok(Self::CHAR_LIST_RETRIEVING),
            43 => Ok(Self::CHAR_LIST_RETRIEVED),
            44 => Ok(Self::CHAR_LIST_FAILED),
            45 => Ok(Self::CHAR_CREATE_IN_PROGRESS),
            46 => Ok(Self::CHAR_CREATE_SUCCESS),
            47 => Ok(Self::CHAR_CREATE_ERROR),
            48 => Ok(Self::CHAR_CREATE_FAILED),
            49 => Ok(Self::CHAR_CREATE_NAME_IN_USE),
            50 => Ok(Self::CHAR_CREATE_DISABLED),
            51 => Ok(Self::CHAR_CREATE_PVP_TEAMS_VIOLATION),
            52 => Ok(Self::CHAR_CREATE_SERVER_LIMIT),
            53 => Ok(Self::CHAR_CREATE_ACCOUNT_LIMIT),
            54 => Ok(Self::CHAR_CREATE_SERVER_QUEUE),
            55 => Ok(Self::CHAR_CREATE_ONLY_EXISTING),
            56 => Ok(Self::CHAR_DELETE_IN_PROGRESS),
            57 => Ok(Self::CHAR_DELETE_SUCCESS),
            58 => Ok(Self::CHAR_DELETE_FAILED),
            59 => Ok(Self::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER),
            60 => Ok(Self::CHAR_LOGIN_IN_PROGRESS),
            61 => Ok(Self::CHAR_LOGIN_SUCCESS),
            62 => Ok(Self::CHAR_LOGIN_NO_WORLD),
            63 => Ok(Self::CHAR_LOGIN_DUPLICATE_CHARACTER),
            64 => Ok(Self::CHAR_LOGIN_NO_INSTANCES),
            65 => Ok(Self::CHAR_LOGIN_FAILED),
            66 => Ok(Self::CHAR_LOGIN_DISABLED),
            67 => Ok(Self::CHAR_LOGIN_NO_CHARACTER),
            68 => Ok(Self::CHAR_LOGIN_LOCKED_FOR_TRANSFER),
            69 => Ok(Self::CHAR_NAME_NO_NAME),
            70 => Ok(Self::CHAR_NAME_TOO_SHORT),
            71 => Ok(Self::CHAR_NAME_TOO_LONG),
            72 => Ok(Self::CHAR_NAME_ONLY_LETTERS),
            73 => Ok(Self::CHAR_NAME_MIXED_LANGUAGES),
            74 => Ok(Self::CHAR_NAME_PROFANE),
            75 => Ok(Self::CHAR_NAME_RESERVED),
            76 => Ok(Self::CHAR_NAME_INVALID_APOSTROPHE),
            77 => Ok(Self::CHAR_NAME_MULTIPLE_APOSTROPHES),
            78 => Ok(Self::CHAR_NAME_THREE_CONSECUTIVE),
            79 => Ok(Self::CHAR_NAME_INVALID_SPACE),
            80 => Ok(Self::CHAR_NAME_SUCCESS),
            81 => Ok(Self::CHAR_NAME_FAILURE),
            v => Err(crate::errors::EnumError::new("WorldResult", v as u32),)
        }
    }
}

