use std::io::{Read, Write};

use crate::vanilla::WorldResult;

/// Response to [`CMSG_AUTH_SESSION`](crate::vanilla::CMSG_AUTH_SESSION).
/// Usually followed by [`CMSG_CHAR_ENUM`](crate::vanilla::CMSG_CHAR_ENUM) if login was successful (`AUTH_OK`).
/// vmangos/cmangos/mangoszero all have a variant of this message that contains fields from `AUTH_OK` for `AUTH_WAIT_QUEUE` as well (`https://github.com/vmangos/core/blob/cd896d43712ceafecdbd8f005846d7f676e55b4f/src/game/World.cpp#L322`) but this does not seem to be actually be a real thing.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm:4`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm#L4):
/// ```text
/// smsg SMSG_AUTH_RESPONSE = 0x01EE {
///     WorldResult result;
///     if (result == AUTH_OK) {
///         u32 billing_time;
///         u8 billing_flags;
///         u32 billing_rested;
///     }
///     else if (result == AUTH_WAIT_QUEUE) {
///         u32 queue_position;
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
        billing_flags: u8,
        billing_rested: u32,
        billing_time: u32,
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
    },
    AuthBanned,
    AuthAlreadyOnline,
    AuthNoTime,
    AuthDbBusy,
    AuthSuspended,
    AuthParentalControl,
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
    CharDeleteInProgress,
    CharDeleteSuccess,
    CharDeleteFailed,
    CharDeleteFailedLockedForTransfer,
    CharLoginInProgress,
    CharLoginSuccess,
    CharLoginNoWorld,
    CharLoginDuplicateCharacter,
    CharLoginNoInstances,
    CharLoginFailed,
    CharLoginDisabled,
    CharLoginNoCharacter,
    CharLoginLockedForTransfer,
    CharNameNoName,
    CharNameTooShort,
    CharNameTooLong,
    CharNameOnlyLetters,
    CharNameMixedLanguages,
    CharNameProfane,
    CharNameReserved,
    CharNameInvalidApostrophe,
    CharNameMultipleApostrophes,
    CharNameThreeConsecutive,
    CharNameInvalidSpace,
    CharNameSuccess,
    CharNameFailure,
}

impl crate::private::Sealed for SMSG_AUTH_RESPONSE {}
impl SMSG_AUTH_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(1..=10).contains(&body_size) {
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

                // billing_flags: u8
                let billing_flags = crate::util::read_u8_le(&mut r)?;

                // billing_rested: u32
                let billing_rested = crate::util::read_u32_le(&mut r)?;

                SMSG_AUTH_RESPONSE::AuthOk {
                    billing_flags,
                    billing_rested,
                    billing_time,
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

                SMSG_AUTH_RESPONSE::AuthWaitQueue {
                    queue_position,
                }
            }
            WorldResult::AuthBanned => SMSG_AUTH_RESPONSE::AuthBanned,
            WorldResult::AuthAlreadyOnline => SMSG_AUTH_RESPONSE::AuthAlreadyOnline,
            WorldResult::AuthNoTime => SMSG_AUTH_RESPONSE::AuthNoTime,
            WorldResult::AuthDbBusy => SMSG_AUTH_RESPONSE::AuthDbBusy,
            WorldResult::AuthSuspended => SMSG_AUTH_RESPONSE::AuthSuspended,
            WorldResult::AuthParentalControl => SMSG_AUTH_RESPONSE::AuthParentalControl,
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
            WorldResult::CharDeleteInProgress => SMSG_AUTH_RESPONSE::CharDeleteInProgress,
            WorldResult::CharDeleteSuccess => SMSG_AUTH_RESPONSE::CharDeleteSuccess,
            WorldResult::CharDeleteFailed => SMSG_AUTH_RESPONSE::CharDeleteFailed,
            WorldResult::CharDeleteFailedLockedForTransfer => SMSG_AUTH_RESPONSE::CharDeleteFailedLockedForTransfer,
            WorldResult::CharLoginInProgress => SMSG_AUTH_RESPONSE::CharLoginInProgress,
            WorldResult::CharLoginSuccess => SMSG_AUTH_RESPONSE::CharLoginSuccess,
            WorldResult::CharLoginNoWorld => SMSG_AUTH_RESPONSE::CharLoginNoWorld,
            WorldResult::CharLoginDuplicateCharacter => SMSG_AUTH_RESPONSE::CharLoginDuplicateCharacter,
            WorldResult::CharLoginNoInstances => SMSG_AUTH_RESPONSE::CharLoginNoInstances,
            WorldResult::CharLoginFailed => SMSG_AUTH_RESPONSE::CharLoginFailed,
            WorldResult::CharLoginDisabled => SMSG_AUTH_RESPONSE::CharLoginDisabled,
            WorldResult::CharLoginNoCharacter => SMSG_AUTH_RESPONSE::CharLoginNoCharacter,
            WorldResult::CharLoginLockedForTransfer => SMSG_AUTH_RESPONSE::CharLoginLockedForTransfer,
            WorldResult::CharNameNoName => SMSG_AUTH_RESPONSE::CharNameNoName,
            WorldResult::CharNameTooShort => SMSG_AUTH_RESPONSE::CharNameTooShort,
            WorldResult::CharNameTooLong => SMSG_AUTH_RESPONSE::CharNameTooLong,
            WorldResult::CharNameOnlyLetters => SMSG_AUTH_RESPONSE::CharNameOnlyLetters,
            WorldResult::CharNameMixedLanguages => SMSG_AUTH_RESPONSE::CharNameMixedLanguages,
            WorldResult::CharNameProfane => SMSG_AUTH_RESPONSE::CharNameProfane,
            WorldResult::CharNameReserved => SMSG_AUTH_RESPONSE::CharNameReserved,
            WorldResult::CharNameInvalidApostrophe => SMSG_AUTH_RESPONSE::CharNameInvalidApostrophe,
            WorldResult::CharNameMultipleApostrophes => SMSG_AUTH_RESPONSE::CharNameMultipleApostrophes,
            WorldResult::CharNameThreeConsecutive => SMSG_AUTH_RESPONSE::CharNameThreeConsecutive,
            WorldResult::CharNameInvalidSpace => SMSG_AUTH_RESPONSE::CharNameInvalidSpace,
            WorldResult::CharNameSuccess => SMSG_AUTH_RESPONSE::CharNameSuccess,
            WorldResult::CharNameFailure => SMSG_AUTH_RESPONSE::CharNameFailure,
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
            } => {
                // billing_time: u32
                w.write_all(&billing_time.to_le_bytes())?;

                // billing_flags: u8
                w.write_all(&billing_flags.to_le_bytes())?;

                // billing_rested: u32
                w.write_all(&billing_rested.to_le_bytes())?;

            }
            SMSG_AUTH_RESPONSE::AuthWaitQueue {
                queue_position,
            } => {
                // queue_position: u32
                w.write_all(&queue_position.to_le_bytes())?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(494, "SMSG_AUTH_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_AUTH_RESPONSE {}

impl SMSG_AUTH_RESPONSE {
    pub(crate) const fn size(&self) -> usize {
        (match self {
            Self::AuthOk {
                ..
            } => {
                1
                + 1 // billing_flags: u8
                + 4 // billing_rested: u32
                + 4 // billing_time: u32
            }
            Self::AuthWaitQueue {
                ..
            } => {
                1
                + 4 // queue_position: u32
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
            Self::RealmListInProgress => 34,
            Self::RealmListSuccess => 35,
            Self::RealmListFailed => 36,
            Self::RealmListInvalid => 37,
            Self::RealmListRealmNotFound => 38,
            Self::AccountCreateInProgress => 39,
            Self::AccountCreateSuccess => 40,
            Self::AccountCreateFailed => 41,
            Self::CharListRetrieving => 42,
            Self::CharListRetrieved => 43,
            Self::CharListFailed => 44,
            Self::CharCreateInProgress => 45,
            Self::CharCreateSuccess => 46,
            Self::CharCreateError => 47,
            Self::CharCreateFailed => 48,
            Self::CharCreateNameInUse => 49,
            Self::CharCreateDisabled => 50,
            Self::CharCreatePvpTeamsViolation => 51,
            Self::CharCreateServerLimit => 52,
            Self::CharCreateAccountLimit => 53,
            Self::CharCreateServerQueue => 54,
            Self::CharCreateOnlyExisting => 55,
            Self::CharDeleteInProgress => 56,
            Self::CharDeleteSuccess => 57,
            Self::CharDeleteFailed => 58,
            Self::CharDeleteFailedLockedForTransfer => 59,
            Self::CharLoginInProgress => 60,
            Self::CharLoginSuccess => 61,
            Self::CharLoginNoWorld => 62,
            Self::CharLoginDuplicateCharacter => 63,
            Self::CharLoginNoInstances => 64,
            Self::CharLoginFailed => 65,
            Self::CharLoginDisabled => 66,
            Self::CharLoginNoCharacter => 67,
            Self::CharLoginLockedForTransfer => 68,
            Self::CharNameNoName => 69,
            Self::CharNameTooShort => 70,
            Self::CharNameTooLong => 71,
            Self::CharNameOnlyLetters => 72,
            Self::CharNameMixedLanguages => 73,
            Self::CharNameProfane => 74,
            Self::CharNameReserved => 75,
            Self::CharNameInvalidApostrophe => 76,
            Self::CharNameMultipleApostrophes => 77,
            Self::CharNameThreeConsecutive => 78,
            Self::CharNameInvalidSpace => 79,
            Self::CharNameSuccess => 80,
            Self::CharNameFailure => 81,
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
            Self::CharDeleteInProgress => f.write_str("CharDeleteInProgress"),
            Self::CharDeleteSuccess => f.write_str("CharDeleteSuccess"),
            Self::CharDeleteFailed => f.write_str("CharDeleteFailed"),
            Self::CharDeleteFailedLockedForTransfer => f.write_str("CharDeleteFailedLockedForTransfer"),
            Self::CharLoginInProgress => f.write_str("CharLoginInProgress"),
            Self::CharLoginSuccess => f.write_str("CharLoginSuccess"),
            Self::CharLoginNoWorld => f.write_str("CharLoginNoWorld"),
            Self::CharLoginDuplicateCharacter => f.write_str("CharLoginDuplicateCharacter"),
            Self::CharLoginNoInstances => f.write_str("CharLoginNoInstances"),
            Self::CharLoginFailed => f.write_str("CharLoginFailed"),
            Self::CharLoginDisabled => f.write_str("CharLoginDisabled"),
            Self::CharLoginNoCharacter => f.write_str("CharLoginNoCharacter"),
            Self::CharLoginLockedForTransfer => f.write_str("CharLoginLockedForTransfer"),
            Self::CharNameNoName => f.write_str("CharNameNoName"),
            Self::CharNameTooShort => f.write_str("CharNameTooShort"),
            Self::CharNameTooLong => f.write_str("CharNameTooLong"),
            Self::CharNameOnlyLetters => f.write_str("CharNameOnlyLetters"),
            Self::CharNameMixedLanguages => f.write_str("CharNameMixedLanguages"),
            Self::CharNameProfane => f.write_str("CharNameProfane"),
            Self::CharNameReserved => f.write_str("CharNameReserved"),
            Self::CharNameInvalidApostrophe => f.write_str("CharNameInvalidApostrophe"),
            Self::CharNameMultipleApostrophes => f.write_str("CharNameMultipleApostrophes"),
            Self::CharNameThreeConsecutive => f.write_str("CharNameThreeConsecutive"),
            Self::CharNameInvalidSpace => f.write_str("CharNameInvalidSpace"),
            Self::CharNameSuccess => f.write_str("CharNameSuccess"),
            Self::CharNameFailure => f.write_str("CharNameFailure"),
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_AUTH_RESPONSE;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 5] = [ 0x00, 0x03, 0xEE, 0x01, 0x0D, ];

    pub(crate) fn expected0() -> SMSG_AUTH_RESPONSE {
        SMSG_AUTH_RESPONSE::AuthFailed
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm` line 19.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_auth_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm` line 19.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_auth_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm` line 19.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_auth_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 9] = [ 0x00, 0x07, 0xEE, 0x01, 0x1B, 0xEF, 0xBE, 0xAD, 0xDE, ];

    pub(crate) fn expected1() -> SMSG_AUTH_RESPONSE {
        SMSG_AUTH_RESPONSE::AuthWaitQueue {
            queue_position: 0xDEADBEEF,
        }
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm` line 30.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_auth_response1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm` line 30.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_auth_response1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm` line 30.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_auth_response1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    const RAW2: [u8; 14] = [ 0x00, 0x0C, 0xEE, 0x01, 0x0C, 0xEF, 0xBE, 0xAD, 0xDE,
         0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected2() -> SMSG_AUTH_RESPONSE {
        SMSG_AUTH_RESPONSE::AuthOk {
            billing_flags: 0x0,
            billing_rested: 0x0,
            billing_time: 0xDEADBEEF,
        }
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm` line 43.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_auth_response2() {
        let expected = expected2();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW2)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm` line 43.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_auth_response2() {
        let expected = expected2();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_auth_response.wowm` line 43.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_auth_response2() {
        let expected = expected2();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_AUTH_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_AUTH_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

}

