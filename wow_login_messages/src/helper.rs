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
use crate::errors::{EnumError, ExpectedOpcodeError, ParseError};
use crate::logon::all::CMD_AUTH_LOGON_CHALLENGE_Client;
use crate::logon::all::CMD_AUTH_RECONNECT_CHALLENGE_Client;
use crate::util::read_u8_le;
use crate::{ClientMessage, ServerMessage};
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

/// Read a complete message _from_ the **client** or return an error otherwise.
///
/// ```
/// use wow_login_messages::helper::expect_client_message;
///
/// use wow_login_messages::version_2::CMD_AUTH_LOGON_PROOF_Client;
/// # fn test(mut reader: impl std::io::Read) -> Result<(), Box<dyn std::error::Error>> {
///
/// let client = expect_client_message::<CMD_AUTH_LOGON_PROOF_Client, _>(&mut reader)?;
/// // We can now use the message
/// let client_proof = client.client_proof;
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "sync")]
pub fn expect_client_message<M: ClientMessage, R: std::io::Read>(
    r: &mut R,
) -> Result<M, ExpectedOpcodeError> {
    let opcode = read_u8_le(r)?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::read(r);
        match m {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(ExpectedOpcodeError::Opcode(opcode as u32))
    }
}

#[cfg(feature = "tokio")]
/// See docs for the sync version called [`expect_client_message`].
pub async fn tokio_expect_client_message<
    M: ClientMessage,
    R: tokio::io::AsyncReadExt + Unpin + Send,
>(
    r: &mut R,
) -> Result<M, ExpectedOpcodeError> {
    let opcode = crate::util::tokio_read_u8_le(r).await?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::tokio_read(r).await;
        match m {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(ExpectedOpcodeError::Opcode(opcode as u32))
    }
}

#[cfg(feature = "async-std")]
/// See docs for the sync version called [`expect_client_message`].
pub async fn astd_expect_client_message<
    M: ClientMessage,
    R: async_std::io::ReadExt + Unpin + Send,
>(
    r: &mut R,
) -> Result<M, ExpectedOpcodeError> {
    let opcode = crate::util::astd_read_u8_le(r).await?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::astd_read(r).await;
        match m {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(ExpectedOpcodeError::Opcode(opcode as u32))
    }
}

/// Read a complete message _from_ the **server** or return an error otherwise.
///
/// ```
/// use wow_login_messages::helper::expect_server_message;
///
/// use wow_login_messages::version_2::CMD_AUTH_LOGON_PROOF_Server;
/// # fn test(mut reader: impl std::io::Read) -> Result<(), Box<dyn std::error::Error>> {
///
/// let server = expect_server_message::<CMD_AUTH_LOGON_PROOF_Server, _>(&mut reader)?;
/// // We can now use the message
/// let login_result = server.login_result;
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "sync")]
pub fn expect_server_message<M: ServerMessage, R: std::io::Read>(
    r: &mut R,
) -> Result<M, ExpectedOpcodeError> {
    let opcode = read_u8_le(r)?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::read(r);
        match m {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(ExpectedOpcodeError::Opcode(opcode as u32))
    }
}

/// See docs for the sync version called [`expect_server_message`].
#[cfg(feature = "tokio")]
pub async fn tokio_expect_server_message<
    M: ServerMessage,
    R: tokio::io::AsyncReadExt + Unpin + Send,
>(
    r: &mut R,
) -> Result<M, ExpectedOpcodeError> {
    let opcode = crate::util::tokio_read_u8_le(r).await?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::tokio_read(r).await;
        match m {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(ExpectedOpcodeError::Opcode(opcode as u32))
    }
}

/// See docs for the sync version called [`expect_server_message`].
#[cfg(feature = "async-std")]
pub async fn astd_expect_server_message<
    M: ServerMessage,
    R: async_std::io::ReadExt + Unpin + Send,
>(
    r: &mut R,
) -> Result<M, ExpectedOpcodeError> {
    let opcode = crate::util::astd_read_u8_le(r).await?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::astd_read(r).await;
        match m {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }
    } else {
        Err(ExpectedOpcodeError::Opcode(opcode as u32))
    }
}

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
    r: &mut R,
) -> Result<InitialMessage, ExpectedOpcodeError> {
    let opcode = read_u8_le(r)?;
    match opcode {
        CMD_AUTH_LOGON_CHALLENGE_Client::OPCODE => Ok(InitialMessage::Logon(
            CMD_AUTH_LOGON_CHALLENGE_Client::read(r)?,
        )),
        CMD_AUTH_RECONNECT_CHALLENGE_Client::OPCODE => Ok(InitialMessage::Reconnect(
            CMD_AUTH_RECONNECT_CHALLENGE_Client::read(r)?,
        )),
        opcode => Err(ExpectedOpcodeError::Opcode(opcode as u32)),
    }
}

/// See docs for the sync version called [`read_initial_message`].
#[cfg(feature = "tokio")]
pub async fn tokio_read_initial_message<R: tokio::io::AsyncReadExt + Unpin + Send>(
    r: &mut R,
) -> Result<InitialMessage, ExpectedOpcodeError> {
    let opcode = crate::util::tokio_read_u8_le(r).await?;
    match opcode {
        CMD_AUTH_LOGON_CHALLENGE_Client::OPCODE => Ok(InitialMessage::Logon(
            CMD_AUTH_LOGON_CHALLENGE_Client::tokio_read(r).await?,
        )),
        CMD_AUTH_RECONNECT_CHALLENGE_Client::OPCODE => Ok(InitialMessage::Reconnect(
            CMD_AUTH_RECONNECT_CHALLENGE_Client::tokio_read(r).await?,
        )),
        opcode => Err(ExpectedOpcodeError::Opcode(opcode as u32)),
    }
}

/// See docs for the sync version called [`read_initial_message`].
#[cfg(feature = "async-std")]
pub async fn astd_read_initial_message<R: async_std::io::ReadExt + Unpin + Send>(
    r: &mut R,
) -> Result<InitialMessage, ExpectedOpcodeError> {
    let opcode = crate::util::astd_read_u8_le(r).await?;
    match opcode {
        CMD_AUTH_LOGON_CHALLENGE_Client::OPCODE => Ok(InitialMessage::Logon(
            CMD_AUTH_LOGON_CHALLENGE_Client::astd_read(r).await?,
        )),
        CMD_AUTH_RECONNECT_CHALLENGE_Client::OPCODE => Ok(InitialMessage::Reconnect(
            CMD_AUTH_RECONNECT_CHALLENGE_Client::astd_read(r).await?,
        )),
        opcode => Err(ExpectedOpcodeError::Opcode(opcode as u32)),
    }
}
