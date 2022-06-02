use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_2::WorldResult;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm#L3):
/// ```text
/// smsg SMSG_CHAR_RENAME = 0x02C8 {
///     WorldResult result;
///     if (result == RESPONSE_SUCCESS) {
///         Guid character;
///         CString new_name;
///     }
/// }
/// ```
/// # Description
///
/// Response to [CMSG_CHAR_RENAME](crate::world::version_1_12::CMSG_CHAR_RENAME).
pub struct SMSG_CHAR_RENAME {
    pub result: SMSG_CHAR_RENAME_WorldResult,
}

impl ServerMessage for SMSG_CHAR_RENAME {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: WorldResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        match &self.result {
            SMSG_CHAR_RENAME_WorldResult::RESPONSE_SUCCESS {
                character,
                new_name,
            } => {
                // character: Guid
                w.write_all(&character.guid().to_le_bytes())?;

                // new_name: CString
                w.write_all(new_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_CHAR_RENAME_WorldResult::RESPONSE_FAILURE => {}
            SMSG_CHAR_RENAME_WorldResult::RESPONSE_CANCELLED => {}
            SMSG_CHAR_RENAME_WorldResult::RESPONSE_DISCONNECTED => {}
            SMSG_CHAR_RENAME_WorldResult::RESPONSE_FAILED_TO_CONNECT => {}
            SMSG_CHAR_RENAME_WorldResult::RESPONSE_CONNECTED => {}
            SMSG_CHAR_RENAME_WorldResult::RESPONSE_VERSION_MISMATCH => {}
            SMSG_CHAR_RENAME_WorldResult::CSTATUS_CONNECTING => {}
            SMSG_CHAR_RENAME_WorldResult::CSTATUS_NEGOTIATING_SECURITY => {}
            SMSG_CHAR_RENAME_WorldResult::CSTATUS_NEGOTIATION_COMPLETE => {}
            SMSG_CHAR_RENAME_WorldResult::CSTATUS_NEGOTIATION_FAILED => {}
            SMSG_CHAR_RENAME_WorldResult::CSTATUS_AUTHENTICATING => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_OK => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_FAILED => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_REJECT => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_BAD_SERVER_PROOF => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_UNAVAILABLE => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_SYSTEM_ERROR => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_BILLING_ERROR => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_BILLING_EXPIRED => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_VERSION_MISMATCH => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_UNKNOWN_ACCOUNT => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_INCORRECT_PASSWORD => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_SESSION_EXPIRED => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_SERVER_SHUTTING_DOWN => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_ALREADY_LOGGING_IN => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_LOGIN_SERVER_NOT_FOUND => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_WAIT_QUEUE => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_BANNED => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_ALREADY_ONLINE => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_NO_TIME => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_DB_BUSY => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_SUSPENDED => {}
            SMSG_CHAR_RENAME_WorldResult::AUTH_PARENTAL_CONTROL => {}
            SMSG_CHAR_RENAME_WorldResult::REALM_LIST_IN_PROGRESS => {}
            SMSG_CHAR_RENAME_WorldResult::REALM_LIST_SUCCESS => {}
            SMSG_CHAR_RENAME_WorldResult::REALM_LIST_FAILED => {}
            SMSG_CHAR_RENAME_WorldResult::REALM_LIST_INVALID => {}
            SMSG_CHAR_RENAME_WorldResult::REALM_LIST_REALM_NOT_FOUND => {}
            SMSG_CHAR_RENAME_WorldResult::ACCOUNT_CREATE_IN_PROGRESS => {}
            SMSG_CHAR_RENAME_WorldResult::ACCOUNT_CREATE_SUCCESS => {}
            SMSG_CHAR_RENAME_WorldResult::ACCOUNT_CREATE_FAILED => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_LIST_RETRIEVING => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_LIST_RETRIEVED => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_LIST_FAILED => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_IN_PROGRESS => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_SUCCESS => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_ERROR => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_FAILED => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_NAME_IN_USE => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_DISABLED => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_PVP_TEAMS_VIOLATION => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_SERVER_LIMIT => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_ACCOUNT_LIMIT => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_SERVER_QUEUE => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_ONLY_EXISTING => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_DELETE_IN_PROGRESS => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_DELETE_SUCCESS => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_DELETE_FAILED => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_LOGIN_IN_PROGRESS => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_LOGIN_SUCCESS => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_LOGIN_NO_WORLD => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_LOGIN_DUPLICATE_CHARACTER => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_LOGIN_NO_INSTANCES => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_LOGIN_FAILED => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_LOGIN_DISABLED => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_LOGIN_NO_CHARACTER => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_LOGIN_LOCKED_FOR_TRANSFER => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_NO_NAME => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_TOO_SHORT => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_TOO_LONG => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_ONLY_LETTERS => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_MIXED_LANGUAGES => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_PROFANE => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_RESERVED => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_INVALID_APOSTROPHE => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_MULTIPLE_APOSTROPHES => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_THREE_CONSECUTIVE => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_INVALID_SPACE => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_SUCCESS => {}
            SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_FAILURE => {}
        }

        Ok(())
    }
    const OPCODE: u16 = 0x02c8;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // result: WorldResult
        let result: WorldResult = crate::util::read_u32_le(r)?.try_into()?;

        let result_if = match result {
            WorldResult::RESPONSE_SUCCESS => {
                // character: Guid
                let character = Guid::read(r)?;

                // new_name: CString
                let new_name = crate::util::read_c_string_to_vec(r)?;
                let new_name = String::from_utf8(new_name)?;

                SMSG_CHAR_RENAME_WorldResult::RESPONSE_SUCCESS {
                    character,
                    new_name,
                }
            }
            WorldResult::RESPONSE_FAILURE => SMSG_CHAR_RENAME_WorldResult::RESPONSE_FAILURE,
            WorldResult::RESPONSE_CANCELLED => SMSG_CHAR_RENAME_WorldResult::RESPONSE_CANCELLED,
            WorldResult::RESPONSE_DISCONNECTED => SMSG_CHAR_RENAME_WorldResult::RESPONSE_DISCONNECTED,
            WorldResult::RESPONSE_FAILED_TO_CONNECT => SMSG_CHAR_RENAME_WorldResult::RESPONSE_FAILED_TO_CONNECT,
            WorldResult::RESPONSE_CONNECTED => SMSG_CHAR_RENAME_WorldResult::RESPONSE_CONNECTED,
            WorldResult::RESPONSE_VERSION_MISMATCH => SMSG_CHAR_RENAME_WorldResult::RESPONSE_VERSION_MISMATCH,
            WorldResult::CSTATUS_CONNECTING => SMSG_CHAR_RENAME_WorldResult::CSTATUS_CONNECTING,
            WorldResult::CSTATUS_NEGOTIATING_SECURITY => SMSG_CHAR_RENAME_WorldResult::CSTATUS_NEGOTIATING_SECURITY,
            WorldResult::CSTATUS_NEGOTIATION_COMPLETE => SMSG_CHAR_RENAME_WorldResult::CSTATUS_NEGOTIATION_COMPLETE,
            WorldResult::CSTATUS_NEGOTIATION_FAILED => SMSG_CHAR_RENAME_WorldResult::CSTATUS_NEGOTIATION_FAILED,
            WorldResult::CSTATUS_AUTHENTICATING => SMSG_CHAR_RENAME_WorldResult::CSTATUS_AUTHENTICATING,
            WorldResult::AUTH_OK => SMSG_CHAR_RENAME_WorldResult::AUTH_OK,
            WorldResult::AUTH_FAILED => SMSG_CHAR_RENAME_WorldResult::AUTH_FAILED,
            WorldResult::AUTH_REJECT => SMSG_CHAR_RENAME_WorldResult::AUTH_REJECT,
            WorldResult::AUTH_BAD_SERVER_PROOF => SMSG_CHAR_RENAME_WorldResult::AUTH_BAD_SERVER_PROOF,
            WorldResult::AUTH_UNAVAILABLE => SMSG_CHAR_RENAME_WorldResult::AUTH_UNAVAILABLE,
            WorldResult::AUTH_SYSTEM_ERROR => SMSG_CHAR_RENAME_WorldResult::AUTH_SYSTEM_ERROR,
            WorldResult::AUTH_BILLING_ERROR => SMSG_CHAR_RENAME_WorldResult::AUTH_BILLING_ERROR,
            WorldResult::AUTH_BILLING_EXPIRED => SMSG_CHAR_RENAME_WorldResult::AUTH_BILLING_EXPIRED,
            WorldResult::AUTH_VERSION_MISMATCH => SMSG_CHAR_RENAME_WorldResult::AUTH_VERSION_MISMATCH,
            WorldResult::AUTH_UNKNOWN_ACCOUNT => SMSG_CHAR_RENAME_WorldResult::AUTH_UNKNOWN_ACCOUNT,
            WorldResult::AUTH_INCORRECT_PASSWORD => SMSG_CHAR_RENAME_WorldResult::AUTH_INCORRECT_PASSWORD,
            WorldResult::AUTH_SESSION_EXPIRED => SMSG_CHAR_RENAME_WorldResult::AUTH_SESSION_EXPIRED,
            WorldResult::AUTH_SERVER_SHUTTING_DOWN => SMSG_CHAR_RENAME_WorldResult::AUTH_SERVER_SHUTTING_DOWN,
            WorldResult::AUTH_ALREADY_LOGGING_IN => SMSG_CHAR_RENAME_WorldResult::AUTH_ALREADY_LOGGING_IN,
            WorldResult::AUTH_LOGIN_SERVER_NOT_FOUND => SMSG_CHAR_RENAME_WorldResult::AUTH_LOGIN_SERVER_NOT_FOUND,
            WorldResult::AUTH_WAIT_QUEUE => SMSG_CHAR_RENAME_WorldResult::AUTH_WAIT_QUEUE,
            WorldResult::AUTH_BANNED => SMSG_CHAR_RENAME_WorldResult::AUTH_BANNED,
            WorldResult::AUTH_ALREADY_ONLINE => SMSG_CHAR_RENAME_WorldResult::AUTH_ALREADY_ONLINE,
            WorldResult::AUTH_NO_TIME => SMSG_CHAR_RENAME_WorldResult::AUTH_NO_TIME,
            WorldResult::AUTH_DB_BUSY => SMSG_CHAR_RENAME_WorldResult::AUTH_DB_BUSY,
            WorldResult::AUTH_SUSPENDED => SMSG_CHAR_RENAME_WorldResult::AUTH_SUSPENDED,
            WorldResult::AUTH_PARENTAL_CONTROL => SMSG_CHAR_RENAME_WorldResult::AUTH_PARENTAL_CONTROL,
            WorldResult::REALM_LIST_IN_PROGRESS => SMSG_CHAR_RENAME_WorldResult::REALM_LIST_IN_PROGRESS,
            WorldResult::REALM_LIST_SUCCESS => SMSG_CHAR_RENAME_WorldResult::REALM_LIST_SUCCESS,
            WorldResult::REALM_LIST_FAILED => SMSG_CHAR_RENAME_WorldResult::REALM_LIST_FAILED,
            WorldResult::REALM_LIST_INVALID => SMSG_CHAR_RENAME_WorldResult::REALM_LIST_INVALID,
            WorldResult::REALM_LIST_REALM_NOT_FOUND => SMSG_CHAR_RENAME_WorldResult::REALM_LIST_REALM_NOT_FOUND,
            WorldResult::ACCOUNT_CREATE_IN_PROGRESS => SMSG_CHAR_RENAME_WorldResult::ACCOUNT_CREATE_IN_PROGRESS,
            WorldResult::ACCOUNT_CREATE_SUCCESS => SMSG_CHAR_RENAME_WorldResult::ACCOUNT_CREATE_SUCCESS,
            WorldResult::ACCOUNT_CREATE_FAILED => SMSG_CHAR_RENAME_WorldResult::ACCOUNT_CREATE_FAILED,
            WorldResult::CHAR_LIST_RETRIEVING => SMSG_CHAR_RENAME_WorldResult::CHAR_LIST_RETRIEVING,
            WorldResult::CHAR_LIST_RETRIEVED => SMSG_CHAR_RENAME_WorldResult::CHAR_LIST_RETRIEVED,
            WorldResult::CHAR_LIST_FAILED => SMSG_CHAR_RENAME_WorldResult::CHAR_LIST_FAILED,
            WorldResult::CHAR_CREATE_IN_PROGRESS => SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_IN_PROGRESS,
            WorldResult::CHAR_CREATE_SUCCESS => SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_SUCCESS,
            WorldResult::CHAR_CREATE_ERROR => SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_ERROR,
            WorldResult::CHAR_CREATE_FAILED => SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_FAILED,
            WorldResult::CHAR_CREATE_NAME_IN_USE => SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_NAME_IN_USE,
            WorldResult::CHAR_CREATE_DISABLED => SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_DISABLED,
            WorldResult::CHAR_CREATE_PVP_TEAMS_VIOLATION => SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_PVP_TEAMS_VIOLATION,
            WorldResult::CHAR_CREATE_SERVER_LIMIT => SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_SERVER_LIMIT,
            WorldResult::CHAR_CREATE_ACCOUNT_LIMIT => SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_ACCOUNT_LIMIT,
            WorldResult::CHAR_CREATE_SERVER_QUEUE => SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_SERVER_QUEUE,
            WorldResult::CHAR_CREATE_ONLY_EXISTING => SMSG_CHAR_RENAME_WorldResult::CHAR_CREATE_ONLY_EXISTING,
            WorldResult::CHAR_DELETE_IN_PROGRESS => SMSG_CHAR_RENAME_WorldResult::CHAR_DELETE_IN_PROGRESS,
            WorldResult::CHAR_DELETE_SUCCESS => SMSG_CHAR_RENAME_WorldResult::CHAR_DELETE_SUCCESS,
            WorldResult::CHAR_DELETE_FAILED => SMSG_CHAR_RENAME_WorldResult::CHAR_DELETE_FAILED,
            WorldResult::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER => SMSG_CHAR_RENAME_WorldResult::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER,
            WorldResult::CHAR_LOGIN_IN_PROGRESS => SMSG_CHAR_RENAME_WorldResult::CHAR_LOGIN_IN_PROGRESS,
            WorldResult::CHAR_LOGIN_SUCCESS => SMSG_CHAR_RENAME_WorldResult::CHAR_LOGIN_SUCCESS,
            WorldResult::CHAR_LOGIN_NO_WORLD => SMSG_CHAR_RENAME_WorldResult::CHAR_LOGIN_NO_WORLD,
            WorldResult::CHAR_LOGIN_DUPLICATE_CHARACTER => SMSG_CHAR_RENAME_WorldResult::CHAR_LOGIN_DUPLICATE_CHARACTER,
            WorldResult::CHAR_LOGIN_NO_INSTANCES => SMSG_CHAR_RENAME_WorldResult::CHAR_LOGIN_NO_INSTANCES,
            WorldResult::CHAR_LOGIN_FAILED => SMSG_CHAR_RENAME_WorldResult::CHAR_LOGIN_FAILED,
            WorldResult::CHAR_LOGIN_DISABLED => SMSG_CHAR_RENAME_WorldResult::CHAR_LOGIN_DISABLED,
            WorldResult::CHAR_LOGIN_NO_CHARACTER => SMSG_CHAR_RENAME_WorldResult::CHAR_LOGIN_NO_CHARACTER,
            WorldResult::CHAR_LOGIN_LOCKED_FOR_TRANSFER => SMSG_CHAR_RENAME_WorldResult::CHAR_LOGIN_LOCKED_FOR_TRANSFER,
            WorldResult::CHAR_NAME_NO_NAME => SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_NO_NAME,
            WorldResult::CHAR_NAME_TOO_SHORT => SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_TOO_SHORT,
            WorldResult::CHAR_NAME_TOO_LONG => SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_TOO_LONG,
            WorldResult::CHAR_NAME_ONLY_LETTERS => SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_ONLY_LETTERS,
            WorldResult::CHAR_NAME_MIXED_LANGUAGES => SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_MIXED_LANGUAGES,
            WorldResult::CHAR_NAME_PROFANE => SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_PROFANE,
            WorldResult::CHAR_NAME_RESERVED => SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_RESERVED,
            WorldResult::CHAR_NAME_INVALID_APOSTROPHE => SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_INVALID_APOSTROPHE,
            WorldResult::CHAR_NAME_MULTIPLE_APOSTROPHES => SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_MULTIPLE_APOSTROPHES,
            WorldResult::CHAR_NAME_THREE_CONSECUTIVE => SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_THREE_CONSECUTIVE,
            WorldResult::CHAR_NAME_INVALID_SPACE => SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_INVALID_SPACE,
            WorldResult::CHAR_NAME_SUCCESS => SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_SUCCESS,
            WorldResult::CHAR_NAME_FAILURE => SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_FAILURE,
        };

        Ok(Self {
            result: result_if,
        })
    }

}

impl SMSG_CHAR_RENAME {
    pub(crate) fn size(&self) -> usize {
        self.result.size() // result: SMSG_CHAR_RENAME_WorldResult
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_CHAR_RENAME_WorldResult {
    RESPONSE_SUCCESS {
        character: Guid,
        new_name: String,
    },
    RESPONSE_FAILURE,
    RESPONSE_CANCELLED,
    RESPONSE_DISCONNECTED,
    RESPONSE_FAILED_TO_CONNECT,
    RESPONSE_CONNECTED,
    RESPONSE_VERSION_MISMATCH,
    CSTATUS_CONNECTING,
    CSTATUS_NEGOTIATING_SECURITY,
    CSTATUS_NEGOTIATION_COMPLETE,
    CSTATUS_NEGOTIATION_FAILED,
    CSTATUS_AUTHENTICATING,
    AUTH_OK,
    AUTH_FAILED,
    AUTH_REJECT,
    AUTH_BAD_SERVER_PROOF,
    AUTH_UNAVAILABLE,
    AUTH_SYSTEM_ERROR,
    AUTH_BILLING_ERROR,
    AUTH_BILLING_EXPIRED,
    AUTH_VERSION_MISMATCH,
    AUTH_UNKNOWN_ACCOUNT,
    AUTH_INCORRECT_PASSWORD,
    AUTH_SESSION_EXPIRED,
    AUTH_SERVER_SHUTTING_DOWN,
    AUTH_ALREADY_LOGGING_IN,
    AUTH_LOGIN_SERVER_NOT_FOUND,
    AUTH_WAIT_QUEUE,
    AUTH_BANNED,
    AUTH_ALREADY_ONLINE,
    AUTH_NO_TIME,
    AUTH_DB_BUSY,
    AUTH_SUSPENDED,
    AUTH_PARENTAL_CONTROL,
    REALM_LIST_IN_PROGRESS,
    REALM_LIST_SUCCESS,
    REALM_LIST_FAILED,
    REALM_LIST_INVALID,
    REALM_LIST_REALM_NOT_FOUND,
    ACCOUNT_CREATE_IN_PROGRESS,
    ACCOUNT_CREATE_SUCCESS,
    ACCOUNT_CREATE_FAILED,
    CHAR_LIST_RETRIEVING,
    CHAR_LIST_RETRIEVED,
    CHAR_LIST_FAILED,
    CHAR_CREATE_IN_PROGRESS,
    CHAR_CREATE_SUCCESS,
    CHAR_CREATE_ERROR,
    CHAR_CREATE_FAILED,
    CHAR_CREATE_NAME_IN_USE,
    CHAR_CREATE_DISABLED,
    CHAR_CREATE_PVP_TEAMS_VIOLATION,
    CHAR_CREATE_SERVER_LIMIT,
    CHAR_CREATE_ACCOUNT_LIMIT,
    CHAR_CREATE_SERVER_QUEUE,
    CHAR_CREATE_ONLY_EXISTING,
    CHAR_DELETE_IN_PROGRESS,
    CHAR_DELETE_SUCCESS,
    CHAR_DELETE_FAILED,
    CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER,
    CHAR_LOGIN_IN_PROGRESS,
    CHAR_LOGIN_SUCCESS,
    CHAR_LOGIN_NO_WORLD,
    CHAR_LOGIN_DUPLICATE_CHARACTER,
    CHAR_LOGIN_NO_INSTANCES,
    CHAR_LOGIN_FAILED,
    CHAR_LOGIN_DISABLED,
    CHAR_LOGIN_NO_CHARACTER,
    CHAR_LOGIN_LOCKED_FOR_TRANSFER,
    CHAR_NAME_NO_NAME,
    CHAR_NAME_TOO_SHORT,
    CHAR_NAME_TOO_LONG,
    CHAR_NAME_ONLY_LETTERS,
    CHAR_NAME_MIXED_LANGUAGES,
    CHAR_NAME_PROFANE,
    CHAR_NAME_RESERVED,
    CHAR_NAME_INVALID_APOSTROPHE,
    CHAR_NAME_MULTIPLE_APOSTROPHES,
    CHAR_NAME_THREE_CONSECUTIVE,
    CHAR_NAME_INVALID_SPACE,
    CHAR_NAME_SUCCESS,
    CHAR_NAME_FAILURE,
}

impl Default for SMSG_CHAR_RENAME_WorldResult {
    fn default() -> Self {
        // First enumerator without any fields
        Self::RESPONSE_SUCCESS {
            character: Default::default(),
            new_name: Default::default(),
        }
    }
}

impl SMSG_CHAR_RENAME_WorldResult {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::RESPONSE_SUCCESS { .. } => 0,
            Self::RESPONSE_FAILURE => 1,
            Self::RESPONSE_CANCELLED => 2,
            Self::RESPONSE_DISCONNECTED => 3,
            Self::RESPONSE_FAILED_TO_CONNECT => 4,
            Self::RESPONSE_CONNECTED => 5,
            Self::RESPONSE_VERSION_MISMATCH => 6,
            Self::CSTATUS_CONNECTING => 7,
            Self::CSTATUS_NEGOTIATING_SECURITY => 8,
            Self::CSTATUS_NEGOTIATION_COMPLETE => 9,
            Self::CSTATUS_NEGOTIATION_FAILED => 10,
            Self::CSTATUS_AUTHENTICATING => 11,
            Self::AUTH_OK => 12,
            Self::AUTH_FAILED => 13,
            Self::AUTH_REJECT => 14,
            Self::AUTH_BAD_SERVER_PROOF => 15,
            Self::AUTH_UNAVAILABLE => 16,
            Self::AUTH_SYSTEM_ERROR => 17,
            Self::AUTH_BILLING_ERROR => 18,
            Self::AUTH_BILLING_EXPIRED => 19,
            Self::AUTH_VERSION_MISMATCH => 20,
            Self::AUTH_UNKNOWN_ACCOUNT => 21,
            Self::AUTH_INCORRECT_PASSWORD => 22,
            Self::AUTH_SESSION_EXPIRED => 23,
            Self::AUTH_SERVER_SHUTTING_DOWN => 24,
            Self::AUTH_ALREADY_LOGGING_IN => 25,
            Self::AUTH_LOGIN_SERVER_NOT_FOUND => 26,
            Self::AUTH_WAIT_QUEUE => 27,
            Self::AUTH_BANNED => 28,
            Self::AUTH_ALREADY_ONLINE => 29,
            Self::AUTH_NO_TIME => 30,
            Self::AUTH_DB_BUSY => 31,
            Self::AUTH_SUSPENDED => 32,
            Self::AUTH_PARENTAL_CONTROL => 33,
            Self::REALM_LIST_IN_PROGRESS => 34,
            Self::REALM_LIST_SUCCESS => 35,
            Self::REALM_LIST_FAILED => 36,
            Self::REALM_LIST_INVALID => 37,
            Self::REALM_LIST_REALM_NOT_FOUND => 38,
            Self::ACCOUNT_CREATE_IN_PROGRESS => 39,
            Self::ACCOUNT_CREATE_SUCCESS => 40,
            Self::ACCOUNT_CREATE_FAILED => 41,
            Self::CHAR_LIST_RETRIEVING => 42,
            Self::CHAR_LIST_RETRIEVED => 43,
            Self::CHAR_LIST_FAILED => 44,
            Self::CHAR_CREATE_IN_PROGRESS => 45,
            Self::CHAR_CREATE_SUCCESS => 46,
            Self::CHAR_CREATE_ERROR => 47,
            Self::CHAR_CREATE_FAILED => 48,
            Self::CHAR_CREATE_NAME_IN_USE => 49,
            Self::CHAR_CREATE_DISABLED => 50,
            Self::CHAR_CREATE_PVP_TEAMS_VIOLATION => 51,
            Self::CHAR_CREATE_SERVER_LIMIT => 52,
            Self::CHAR_CREATE_ACCOUNT_LIMIT => 53,
            Self::CHAR_CREATE_SERVER_QUEUE => 54,
            Self::CHAR_CREATE_ONLY_EXISTING => 55,
            Self::CHAR_DELETE_IN_PROGRESS => 56,
            Self::CHAR_DELETE_SUCCESS => 57,
            Self::CHAR_DELETE_FAILED => 58,
            Self::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER => 59,
            Self::CHAR_LOGIN_IN_PROGRESS => 60,
            Self::CHAR_LOGIN_SUCCESS => 61,
            Self::CHAR_LOGIN_NO_WORLD => 62,
            Self::CHAR_LOGIN_DUPLICATE_CHARACTER => 63,
            Self::CHAR_LOGIN_NO_INSTANCES => 64,
            Self::CHAR_LOGIN_FAILED => 65,
            Self::CHAR_LOGIN_DISABLED => 66,
            Self::CHAR_LOGIN_NO_CHARACTER => 67,
            Self::CHAR_LOGIN_LOCKED_FOR_TRANSFER => 68,
            Self::CHAR_NAME_NO_NAME => 69,
            Self::CHAR_NAME_TOO_SHORT => 70,
            Self::CHAR_NAME_TOO_LONG => 71,
            Self::CHAR_NAME_ONLY_LETTERS => 72,
            Self::CHAR_NAME_MIXED_LANGUAGES => 73,
            Self::CHAR_NAME_PROFANE => 74,
            Self::CHAR_NAME_RESERVED => 75,
            Self::CHAR_NAME_INVALID_APOSTROPHE => 76,
            Self::CHAR_NAME_MULTIPLE_APOSTROPHES => 77,
            Self::CHAR_NAME_THREE_CONSECUTIVE => 78,
            Self::CHAR_NAME_INVALID_SPACE => 79,
            Self::CHAR_NAME_SUCCESS => 80,
            Self::CHAR_NAME_FAILURE => 81,
        }
    }

}

impl SMSG_CHAR_RENAME_WorldResult {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::RESPONSE_SUCCESS {
                character,
                new_name,
            } => {
                4
                + 8 // character: Guid
                + new_name.len() + 1 // new_name: CString
            }
            Self::RESPONSE_FAILURE => {
                4
            }
            Self::RESPONSE_CANCELLED => {
                4
            }
            Self::RESPONSE_DISCONNECTED => {
                4
            }
            Self::RESPONSE_FAILED_TO_CONNECT => {
                4
            }
            Self::RESPONSE_CONNECTED => {
                4
            }
            Self::RESPONSE_VERSION_MISMATCH => {
                4
            }
            Self::CSTATUS_CONNECTING => {
                4
            }
            Self::CSTATUS_NEGOTIATING_SECURITY => {
                4
            }
            Self::CSTATUS_NEGOTIATION_COMPLETE => {
                4
            }
            Self::CSTATUS_NEGOTIATION_FAILED => {
                4
            }
            Self::CSTATUS_AUTHENTICATING => {
                4
            }
            Self::AUTH_OK => {
                4
            }
            Self::AUTH_FAILED => {
                4
            }
            Self::AUTH_REJECT => {
                4
            }
            Self::AUTH_BAD_SERVER_PROOF => {
                4
            }
            Self::AUTH_UNAVAILABLE => {
                4
            }
            Self::AUTH_SYSTEM_ERROR => {
                4
            }
            Self::AUTH_BILLING_ERROR => {
                4
            }
            Self::AUTH_BILLING_EXPIRED => {
                4
            }
            Self::AUTH_VERSION_MISMATCH => {
                4
            }
            Self::AUTH_UNKNOWN_ACCOUNT => {
                4
            }
            Self::AUTH_INCORRECT_PASSWORD => {
                4
            }
            Self::AUTH_SESSION_EXPIRED => {
                4
            }
            Self::AUTH_SERVER_SHUTTING_DOWN => {
                4
            }
            Self::AUTH_ALREADY_LOGGING_IN => {
                4
            }
            Self::AUTH_LOGIN_SERVER_NOT_FOUND => {
                4
            }
            Self::AUTH_WAIT_QUEUE => {
                4
            }
            Self::AUTH_BANNED => {
                4
            }
            Self::AUTH_ALREADY_ONLINE => {
                4
            }
            Self::AUTH_NO_TIME => {
                4
            }
            Self::AUTH_DB_BUSY => {
                4
            }
            Self::AUTH_SUSPENDED => {
                4
            }
            Self::AUTH_PARENTAL_CONTROL => {
                4
            }
            Self::REALM_LIST_IN_PROGRESS => {
                4
            }
            Self::REALM_LIST_SUCCESS => {
                4
            }
            Self::REALM_LIST_FAILED => {
                4
            }
            Self::REALM_LIST_INVALID => {
                4
            }
            Self::REALM_LIST_REALM_NOT_FOUND => {
                4
            }
            Self::ACCOUNT_CREATE_IN_PROGRESS => {
                4
            }
            Self::ACCOUNT_CREATE_SUCCESS => {
                4
            }
            Self::ACCOUNT_CREATE_FAILED => {
                4
            }
            Self::CHAR_LIST_RETRIEVING => {
                4
            }
            Self::CHAR_LIST_RETRIEVED => {
                4
            }
            Self::CHAR_LIST_FAILED => {
                4
            }
            Self::CHAR_CREATE_IN_PROGRESS => {
                4
            }
            Self::CHAR_CREATE_SUCCESS => {
                4
            }
            Self::CHAR_CREATE_ERROR => {
                4
            }
            Self::CHAR_CREATE_FAILED => {
                4
            }
            Self::CHAR_CREATE_NAME_IN_USE => {
                4
            }
            Self::CHAR_CREATE_DISABLED => {
                4
            }
            Self::CHAR_CREATE_PVP_TEAMS_VIOLATION => {
                4
            }
            Self::CHAR_CREATE_SERVER_LIMIT => {
                4
            }
            Self::CHAR_CREATE_ACCOUNT_LIMIT => {
                4
            }
            Self::CHAR_CREATE_SERVER_QUEUE => {
                4
            }
            Self::CHAR_CREATE_ONLY_EXISTING => {
                4
            }
            Self::CHAR_DELETE_IN_PROGRESS => {
                4
            }
            Self::CHAR_DELETE_SUCCESS => {
                4
            }
            Self::CHAR_DELETE_FAILED => {
                4
            }
            Self::CHAR_DELETE_FAILED_LOCKED_FOR_TRANSFER => {
                4
            }
            Self::CHAR_LOGIN_IN_PROGRESS => {
                4
            }
            Self::CHAR_LOGIN_SUCCESS => {
                4
            }
            Self::CHAR_LOGIN_NO_WORLD => {
                4
            }
            Self::CHAR_LOGIN_DUPLICATE_CHARACTER => {
                4
            }
            Self::CHAR_LOGIN_NO_INSTANCES => {
                4
            }
            Self::CHAR_LOGIN_FAILED => {
                4
            }
            Self::CHAR_LOGIN_DISABLED => {
                4
            }
            Self::CHAR_LOGIN_NO_CHARACTER => {
                4
            }
            Self::CHAR_LOGIN_LOCKED_FOR_TRANSFER => {
                4
            }
            Self::CHAR_NAME_NO_NAME => {
                4
            }
            Self::CHAR_NAME_TOO_SHORT => {
                4
            }
            Self::CHAR_NAME_TOO_LONG => {
                4
            }
            Self::CHAR_NAME_ONLY_LETTERS => {
                4
            }
            Self::CHAR_NAME_MIXED_LANGUAGES => {
                4
            }
            Self::CHAR_NAME_PROFANE => {
                4
            }
            Self::CHAR_NAME_RESERVED => {
                4
            }
            Self::CHAR_NAME_INVALID_APOSTROPHE => {
                4
            }
            Self::CHAR_NAME_MULTIPLE_APOSTROPHES => {
                4
            }
            Self::CHAR_NAME_THREE_CONSECUTIVE => {
                4
            }
            Self::CHAR_NAME_INVALID_SPACE => {
                4
            }
            Self::CHAR_NAME_SUCCESS => {
                4
            }
            Self::CHAR_NAME_FAILURE => {
                4
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::SMSG_CHAR_RENAME;
    use crate::world::version_1_2::WorldResult;
    use super::*;
    use super::super::*;
    use crate::world::version_1_12::opcodes::ServerOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 8] = [ 0x00, 0x06, 0xC8, 0x02, 0x47, 0x00, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm` line 13.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_CHAR_RENAME0() {
        let expected = SMSG_CHAR_RENAME {
            result: SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_TOO_LONG,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm` line 13.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_CHAR_RENAME0() {
        let expected = SMSG_CHAR_RENAME {
            result: SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_TOO_LONG,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm` line 13.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_CHAR_RENAME0() {
        let expected = SMSG_CHAR_RENAME {
            result: SMSG_CHAR_RENAME_WorldResult::CHAR_NAME_TOO_LONG,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 25] = [ 0x00, 0x17, 0xC8, 0x02, 0x00, 0x00, 0x00, 0x00, 0xEF,
         0xBE, 0xAD, 0xDE, 0x00, 0x00, 0x00, 0x00, 0x44, 0x65, 0x61, 0x64, 0x62,
         0x65, 0x65, 0x66, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm` line 21.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_CHAR_RENAME1() {
        let expected = SMSG_CHAR_RENAME {
            result: SMSG_CHAR_RENAME_WorldResult::RESPONSE_SUCCESS {
                character: Guid::new(0xDEADBEEF),
                new_name: String::from("Deadbeef"),
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm` line 21.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_CHAR_RENAME1() {
        let expected = SMSG_CHAR_RENAME {
            result: SMSG_CHAR_RENAME_WorldResult::RESPONSE_SUCCESS {
                character: Guid::new(0xDEADBEEF),
                new_name: String::from("Deadbeef"),
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_char_rename.wowm` line 21.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_CHAR_RENAME1() {
        let expected = SMSG_CHAR_RENAME {
            result: SMSG_CHAR_RENAME_WorldResult::RESPONSE_SUCCESS {
                character: Guid::new(0xDEADBEEF),
                new_name: String::from("Deadbeef"),
            },
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_CHAR_RENAME(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_CHAR_RENAME, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.result, expected.result);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}
