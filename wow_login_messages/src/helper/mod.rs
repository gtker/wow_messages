//! Utility functions for common operations.
//!
//! [`read_initial_message`] is used as the very first message sent by the client can
//! only be either a
//! [`CMD_AUTH_LOGON_CHALLENGE_Client`] or
//! [`CMD_AUTH_RECONNECT_CHALLENGE_Client`].
//!
//! [`expect_client_message`] and [`expect_server_message`]
//! are used when you're expecting exactly one specific message and all others are invalid.
//!
mod expected;
mod expected_protocol;

pub use expected::*;
pub use expected_protocol::*;

#[cfg(any(feature = "sync", feature = "tokio", feature = "async-std"))]
use crate::errors::ExpectedOpcodeError;
#[cfg(any(feature = "sync", feature = "tokio", feature = "async-std"))]
use crate::Message;

use crate::logon::all::CMD_AUTH_LOGON_CHALLENGE_Client;
use crate::logon::all::CMD_AUTH_RECONNECT_CHALLENGE_Client;
#[cfg(feature = "sync")]
use crate::util::read_u8_le;

/// Either of the first two initial messages that are allowed.
#[derive(Debug, Clone)]
pub enum InitialMessage {
    Logon(CMD_AUTH_LOGON_CHALLENGE_Client),
    Reconnect(CMD_AUTH_RECONNECT_CHALLENGE_Client),
}

/// Reads either a
/// [`CMD_AUTH_LOGON_CHALLENGE_Client`], a
/// [`CMD_AUTH_RECONNECT_CHALLENGE_Client`] or returns an error.
///
/// This is intended to be used on authentication servers as the very first
/// thing to be read from the socket.
///
/// This is provided instead of just having users use a
/// [`ClientOpcodeMessage`](crate::version_2::opcodes::ClientOpcodeMessage)
/// since [`CMD_AUTH_LOGON_CHALLENGE_Client`], and [`CMD_AUTH_RECONNECT_CHALLENGE_Client`] are valid for all versions,
/// and this creates a nicer abstraction around that.
///
/// ```
/// use wow_login_messages::helper::{InitialMessage, read_initial_message};
/// use wow_login_messages::all::{CMD_AUTH_LOGON_CHALLENGE_Client, CMD_AUTH_RECONNECT_CHALLENGE_Client};
/// # fn handle_logon(l: CMD_AUTH_LOGON_CHALLENGE_Client) {}
/// # fn handle_reconnect(l: CMD_AUTH_RECONNECT_CHALLENGE_Client) {}
/// # fn test(mut reader: impl std::io::Read) -> Result<(), Box<dyn std::error::Error>> {
///
/// // First thing we read from the socket
/// let opcode = read_initial_message(&mut reader)?;
/// // We now have either a logon attempt or a reconnect attempt
/// match opcode {
///     InitialMessage::Logon(l) => {
///         handle_logon(l);
///     }
///     InitialMessage::Reconnect(r) => {
///         handle_reconnect(r);
///     }
/// }
/// # Ok(())
/// # }
/// ```
///
///
#[cfg(feature = "sync")]
pub fn read_initial_message<R: std::io::Read>(
    mut r: R,
) -> Result<InitialMessage, ExpectedOpcodeError> {
    let opcode = read_u8_le(&mut r)?;
    match opcode {
        CMD_AUTH_LOGON_CHALLENGE_Client::OPCODE => Ok(InitialMessage::Logon(
            CMD_AUTH_LOGON_CHALLENGE_Client::read::<&mut R, crate::private::Internal>(&mut r)?,
        )),
        CMD_AUTH_RECONNECT_CHALLENGE_Client::OPCODE => Ok(InitialMessage::Reconnect(
            CMD_AUTH_RECONNECT_CHALLENGE_Client::read::<&mut R, crate::private::Internal>(&mut r)?,
        )),
        opcode => Err(ExpectedOpcodeError::Opcode(opcode as u32)),
    }
}

/// See docs for the sync version called [`read_initial_message`].
#[cfg(feature = "tokio")]
pub async fn tokio_read_initial_message<R: tokio::io::AsyncReadExt + Unpin + Send>(
    mut r: R,
) -> Result<InitialMessage, ExpectedOpcodeError> {
    let opcode = crate::util::tokio_read_u8_le(&mut r).await?;
    match opcode {
        CMD_AUTH_LOGON_CHALLENGE_Client::OPCODE => Ok(InitialMessage::Logon(
            CMD_AUTH_LOGON_CHALLENGE_Client::tokio_read::<&mut R, crate::private::Internal>(&mut r)
                .await?,
        )),
        CMD_AUTH_RECONNECT_CHALLENGE_Client::OPCODE => Ok(InitialMessage::Reconnect(
            CMD_AUTH_RECONNECT_CHALLENGE_Client::tokio_read::<&mut R, crate::private::Internal>(
                &mut r,
            )
            .await?,
        )),
        opcode => Err(ExpectedOpcodeError::Opcode(opcode as u32)),
    }
}

/// See docs for the sync version called [`read_initial_message`].
#[cfg(feature = "async-std")]
pub async fn astd_read_initial_message<R: async_std::io::ReadExt + Unpin + Send>(
    mut r: R,
) -> Result<InitialMessage, ExpectedOpcodeError> {
    let opcode = crate::util::astd_read_u8_le(&mut r).await?;
    match opcode {
        CMD_AUTH_LOGON_CHALLENGE_Client::OPCODE => Ok(InitialMessage::Logon(
            CMD_AUTH_LOGON_CHALLENGE_Client::astd_read::<&mut R, crate::private::Internal>(&mut r)
                .await?,
        )),
        CMD_AUTH_RECONNECT_CHALLENGE_Client::OPCODE => Ok(InitialMessage::Reconnect(
            CMD_AUTH_RECONNECT_CHALLENGE_Client::astd_read::<&mut R, crate::private::Internal>(
                &mut r,
            )
            .await?,
        )),
        opcode => Err(ExpectedOpcodeError::Opcode(opcode as u32)),
    }
}
