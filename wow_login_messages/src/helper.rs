//! Utility functions for common operations.
//!
//! [`read_initial_opcode`] is used as the very first message sent by the client can
//! only be either a
//! [`CMD_AUTH_LOGON_CHALLENGE_Client`] or
//! [`CMD_AUTH_RECONNECT_CHALLENGE_Client`].
//!
//! [`read_expect_client_login_message`] and [`read_expect_server_login_message`]
//! are used when you're expecting exactly one specific message and all others are invalid.
//!
use crate::logon::all::{CMD_AUTH_LOGON_CHALLENGE_Client, CMD_AUTH_LOGON_CHALLENGE_ClientError};
use crate::logon::all::{
    CMD_AUTH_RECONNECT_CHALLENGE_Client, CMD_AUTH_RECONNECT_CHALLENGE_ClientError,
};
use crate::util::read_u8_le;
use crate::{ClientMessage, ReadableAndWritable, ServerMessage};
use std::fmt::{Display, Formatter};
#[cfg(feature = "sync")]
use std::io::Read;

/// Read a complete message _from_ the **client** or return an error otherwise.
///
/// ```
/// use wow_login_messages::helper::{
///             read_expect_client_login_message
/// };
/// use wow_login_messages::version_2::CMD_AUTH_LOGON_PROOF_Client;
/// # fn test(mut reader: impl std::io::Read) -> Result<(), Box<dyn std::error::Error>> {
///
/// let client = read_expect_client_login_message::<CMD_AUTH_LOGON_PROOF_Client, _>(&mut reader)?;
/// // We can now use the message
/// let client_proof = client.client_proof;
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "sync")]
pub fn read_expect_client_login_message<M: ClientMessage, R: Read>(
    r: &mut R,
) -> Result<M, ExpectedServerLoginMessageError> {
    let opcode = read_u8_le(r)?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::read(r);
        match m {
            Ok(m) => Ok(m),
            Err(_) => Err(ExpectedServerLoginMessageError::GenericError),
        }
    } else {
        Err(ExpectedServerLoginMessageError::UnexpectedOpcode(opcode))
    }
}

#[cfg(feature = "tokio")]
pub async fn tokio_read_expect_client_login_message<
    M: ClientMessage,
    R: tokio::io::AsyncReadExt + Unpin + Send,
>(
    r: &mut R,
) -> Result<M, ExpectedServerLoginMessageError> {
    let opcode = crate::util::tokio_read_u8_le(r).await?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::tokio_read(r).await;
        match m {
            Ok(m) => Ok(m),
            Err(_) => Err(ExpectedServerLoginMessageError::GenericError),
        }
    } else {
        Err(ExpectedServerLoginMessageError::UnexpectedOpcode(opcode))
    }
}

#[derive(Debug)]
pub enum ExpectedClientLoginMessageError {
    Io(std::io::Error),
    UnexpectedOpcode(u8),
    GenericError,
}
impl std::error::Error for ExpectedClientLoginMessageError {}

impl Display for ExpectedClientLoginMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::UnexpectedOpcode(i) => {
                f.write_fmt(format_args!("unexpected opcode returned: '{}'", i))
            }
            Self::GenericError => f.write_str("something went wrong parsing the message"),
        }
    }
}

impl From<std::io::Error> for ExpectedClientLoginMessageError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// Read a complete message _from_ the **server** or return an error otherwise.
///
/// ```
/// use wow_login_messages::helper::{
///             read_expect_server_login_message
/// };
/// use wow_login_messages::version_2::CMD_AUTH_LOGON_PROOF_Server;
/// # fn test(mut reader: impl std::io::Read) -> Result<(), Box<dyn std::error::Error>> {
///
/// let server = read_expect_server_login_message::<CMD_AUTH_LOGON_PROOF_Server, _>(&mut reader)?;
/// // We can now use the message
/// let login_result = server.login_result;
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "sync")]
pub fn read_expect_server_login_message<M: ServerMessage, R: Read>(
    r: &mut R,
) -> Result<M, ExpectedServerLoginMessageError> {
    let opcode = read_u8_le(r)?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::read(r);
        match m {
            Ok(m) => Ok(m),
            Err(_) => Err(ExpectedServerLoginMessageError::GenericError),
        }
    } else {
        Err(ExpectedServerLoginMessageError::UnexpectedOpcode(opcode))
    }
}

#[cfg(feature = "tokio")]
pub async fn tokio_read_expect_server_login_message<
    M: ServerMessage,
    R: tokio::io::AsyncReadExt + Unpin + Send,
>(
    r: &mut R,
) -> Result<M, ExpectedServerLoginMessageError> {
    let opcode = crate::util::tokio_read_u8_le(r).await?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::tokio_read(r).await;
        match m {
            Ok(m) => Ok(m),
            Err(_) => Err(ExpectedServerLoginMessageError::GenericError),
        }
    } else {
        Err(ExpectedServerLoginMessageError::UnexpectedOpcode(opcode))
    }
}

#[derive(Debug)]
pub enum ExpectedServerLoginMessageError {
    Io(std::io::Error),
    UnexpectedOpcode(u8),
    GenericError,
}
impl std::error::Error for ExpectedServerLoginMessageError {}

impl Display for ExpectedServerLoginMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::UnexpectedOpcode(i) => {
                f.write_fmt(format_args!("unexpected opcode returned: '{}'", i))
            }
            Self::GenericError => f.write_str("something went wrong parsing the message"),
        }
    }
}

impl From<std::io::Error> for ExpectedServerLoginMessageError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// Reads either a
/// [`CMD_AUTH_LOGON_CHALLENGE_Client`], a
/// [`CMD_AUTH_RECONNECT_CHALLENGE_Client`] or returns an error.
///
/// This is intended to be used on authentication servers as the very first
/// thing to be read from the socket.
///
/// ```
/// use wow_login_messages::helper::{InitialLoginOpcode, read_initial_opcode};
/// use wow_login_messages::all::{CMD_AUTH_LOGON_CHALLENGE_Client, CMD_AUTH_RECONNECT_CHALLENGE_Client};
/// # fn handle_logon(l: CMD_AUTH_LOGON_CHALLENGE_Client) {}
/// # fn handle_reconnect(l: CMD_AUTH_RECONNECT_CHALLENGE_Client) {}
/// # fn test(mut reader: impl std::io::Read) -> Result<(), Box<dyn std::error::Error>> {
///
/// // First thing we read from the socket
/// let opcode = read_initial_opcode(&mut reader)?;
/// // We now have either a logon attempt or a reconnect attempt
/// match opcode {
///     InitialLoginOpcode::Logon(l) => {
///         handle_logon(l);
///     }
///     InitialLoginOpcode::Reconnect(r) => {
///         handle_reconnect(r);
///     }
/// }
/// # Ok(())
/// # }
/// ```
///
///
#[cfg(feature = "sync")]
pub fn read_initial_opcode<R: Read>(
    r: &mut R,
) -> Result<InitialLoginOpcode, InitialLoginOpcodeError> {
    let opcode = read_u8_le(r)?;
    match opcode {
        CMD_AUTH_LOGON_CHALLENGE_Client::OPCODE => Ok(InitialLoginOpcode::Logon(
            CMD_AUTH_LOGON_CHALLENGE_Client::read(r)?,
        )),
        CMD_AUTH_RECONNECT_CHALLENGE_Client::OPCODE => Ok(InitialLoginOpcode::Reconnect(
            CMD_AUTH_RECONNECT_CHALLENGE_Client::read(r)?,
        )),
        opcode => Err(InitialLoginOpcodeError::InvalidOpcode(opcode)),
    }
}

#[cfg(feature = "tokio")]
pub async fn tokio_read_initial_opcode<R: tokio::io::AsyncReadExt + Unpin + Send>(
    r: &mut R,
) -> Result<InitialLoginOpcode, InitialLoginOpcodeError> {
    let opcode = crate::util::tokio_read_u8_le(r).await?;
    match opcode {
        CMD_AUTH_LOGON_CHALLENGE_Client::OPCODE => Ok(InitialLoginOpcode::Logon(
            CMD_AUTH_LOGON_CHALLENGE_Client::tokio_read(r).await?,
        )),
        CMD_AUTH_RECONNECT_CHALLENGE_Client::OPCODE => Ok(InitialLoginOpcode::Reconnect(
            CMD_AUTH_RECONNECT_CHALLENGE_Client::tokio_read(r).await?,
        )),
        opcode => Err(InitialLoginOpcodeError::InvalidOpcode(opcode)),
    }
}

#[derive(Debug)]
pub enum InitialLoginOpcode {
    Logon(CMD_AUTH_LOGON_CHALLENGE_Client),
    Reconnect(CMD_AUTH_RECONNECT_CHALLENGE_Client),
}

#[derive(Debug)]
pub enum InitialLoginOpcodeError {
    Io(std::io::Error),
    InvalidOpcode(u8),
    CMD_AUTH_LOGON_CHALLENGE(CMD_AUTH_LOGON_CHALLENGE_ClientError),
    CMD_AUTH_RECONNECT_CHALLENGE(CMD_AUTH_RECONNECT_CHALLENGE_ClientError),
}
impl Display for InitialLoginOpcodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InitialLoginOpcodeError::Io(i) => i.fmt(f),
            InitialLoginOpcodeError::InvalidOpcode(i) => f.write_fmt(format_args!("opcode that is not CMD_AUTH_LOGON_CHALLENGE or CMD_AUTH_RECONNECT_CHALLENGE received: '{}'", i)),
            InitialLoginOpcodeError::CMD_AUTH_LOGON_CHALLENGE(i) => i.fmt(f),
            InitialLoginOpcodeError::CMD_AUTH_RECONNECT_CHALLENGE(i) => i.fmt(f),
        }
    }
}
impl std::error::Error for InitialLoginOpcodeError {}

impl From<std::io::Error> for InitialLoginOpcodeError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<CMD_AUTH_LOGON_CHALLENGE_ClientError> for InitialLoginOpcodeError {
    fn from(e: CMD_AUTH_LOGON_CHALLENGE_ClientError) -> Self {
        Self::CMD_AUTH_LOGON_CHALLENGE(e)
    }
}

impl From<CMD_AUTH_RECONNECT_CHALLENGE_ClientError> for InitialLoginOpcodeError {
    fn from(e: CMD_AUTH_RECONNECT_CHALLENGE_ClientError) -> Self {
        Self::CMD_AUTH_RECONNECT_CHALLENGE(e)
    }
}
