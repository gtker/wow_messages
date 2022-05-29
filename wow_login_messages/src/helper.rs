//! Utility functions for common operations.
//!
//! [`read_initial_opcode`] is used as the very first message sent by the client can
//! only be either a
//! [`CMD_AUTH_LOGON_CHALLENGE_Client`] or
//! [`CMD_AUTH_RECONNECT_CHALLENGE_Client`].
//!
//! [`expect_client_message`] and [`expect_server_message`]
//! are used when you're expecting exactly one specific message and all others are invalid.
//!
use crate::errors::{EnumError, ParseError};
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
) -> Result<M, ExpectedMessageError> {
    let opcode = read_u8_le(r)?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::read(r);
        match m {
            Ok(m) => Ok(m),
            Err(_) => Err(ExpectedMessageError::GenericError),
        }
    } else {
        Err(ExpectedMessageError::UnexpectedOpcode(opcode))
    }
}

#[cfg(feature = "tokio")]
pub async fn tokio_expect_client_message<
    M: ClientMessage,
    R: tokio::io::AsyncReadExt + Unpin + Send,
>(
    r: &mut R,
) -> Result<M, ExpectedMessageError> {
    let opcode = crate::util::tokio_read_u8_le(r).await?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::tokio_read(r).await;
        match m {
            Ok(m) => Ok(m),
            Err(_) => Err(ExpectedMessageError::GenericError),
        }
    } else {
        Err(ExpectedMessageError::UnexpectedOpcode(opcode))
    }
}

#[derive(Debug)]
pub enum ExpectedMessageError {
    Io(std::io::Error),
    UnexpectedOpcode(u8),
    GenericError,
}
impl std::error::Error for ExpectedMessageError {}

impl Display for ExpectedMessageError {
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

impl From<std::io::Error> for ExpectedMessageError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
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
) -> Result<M, ExpectedMessageError> {
    let opcode = read_u8_le(r)?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::read(r);
        match m {
            Ok(m) => Ok(m),
            Err(_) => Err(ExpectedMessageError::GenericError),
        }
    } else {
        Err(ExpectedMessageError::UnexpectedOpcode(opcode))
    }
}

#[cfg(feature = "tokio")]
pub async fn tokio_expect_server_message<
    M: ServerMessage,
    R: tokio::io::AsyncReadExt + Unpin + Send,
>(
    r: &mut R,
) -> Result<M, ExpectedMessageError> {
    let opcode = crate::util::tokio_read_u8_le(r).await?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {
        let m = M::tokio_read(r).await;
        match m {
            Ok(m) => Ok(m),
            Err(_) => Err(ExpectedMessageError::GenericError),
        }
    } else {
        Err(ExpectedMessageError::UnexpectedOpcode(opcode))
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
/// use wow_login_messages::helper::{InitialOpcode, read_initial_opcode};
/// use wow_login_messages::all::{CMD_AUTH_LOGON_CHALLENGE_Client, CMD_AUTH_RECONNECT_CHALLENGE_Client};
/// # fn handle_logon(l: CMD_AUTH_LOGON_CHALLENGE_Client) {}
/// # fn handle_reconnect(l: CMD_AUTH_RECONNECT_CHALLENGE_Client) {}
/// # fn test(mut reader: impl std::io::Read) -> Result<(), Box<dyn std::error::Error>> {
///
/// // First thing we read from the socket
/// let opcode = read_initial_opcode(&mut reader)?;
/// // We now have either a logon attempt or a reconnect attempt
/// match opcode {
///     InitialOpcode::Logon(l) => {
///         handle_logon(l);
///     }
///     InitialOpcode::Reconnect(r) => {
///         handle_reconnect(r);
///     }
/// }
/// # Ok(())
/// # }
/// ```
///
///
#[cfg(feature = "sync")]
pub fn read_initial_opcode<R: std::io::Read>(
    r: &mut R,
) -> Result<InitialOpcode, InitialOpcodeError> {
    let opcode = read_u8_le(r)?;
    match opcode {
        CMD_AUTH_LOGON_CHALLENGE_Client::OPCODE => Ok(InitialOpcode::Logon(
            CMD_AUTH_LOGON_CHALLENGE_Client::read(r)?,
        )),
        CMD_AUTH_RECONNECT_CHALLENGE_Client::OPCODE => Ok(InitialOpcode::Reconnect(
            CMD_AUTH_RECONNECT_CHALLENGE_Client::read(r)?,
        )),
        opcode => Err(InitialOpcodeError::InvalidOpcode(opcode)),
    }
}

#[cfg(feature = "tokio")]
pub async fn tokio_read_initial_opcode<R: tokio::io::AsyncReadExt + Unpin + Send>(
    r: &mut R,
) -> Result<InitialOpcode, InitialOpcodeError> {
    let opcode = crate::util::tokio_read_u8_le(r).await?;
    match opcode {
        CMD_AUTH_LOGON_CHALLENGE_Client::OPCODE => Ok(InitialOpcode::Logon(
            CMD_AUTH_LOGON_CHALLENGE_Client::tokio_read(r).await?,
        )),
        CMD_AUTH_RECONNECT_CHALLENGE_Client::OPCODE => Ok(InitialOpcode::Reconnect(
            CMD_AUTH_RECONNECT_CHALLENGE_Client::tokio_read(r).await?,
        )),
        opcode => Err(InitialOpcodeError::InvalidOpcode(opcode)),
    }
}

#[derive(Debug)]
pub enum InitialOpcode {
    Logon(CMD_AUTH_LOGON_CHALLENGE_Client),
    Reconnect(CMD_AUTH_RECONNECT_CHALLENGE_Client),
}

#[derive(Debug)]
pub enum InitialOpcodeError {
    Io(std::io::Error),
    InvalidOpcode(u8),
    String(std::string::FromUtf8Error),
    Enum(crate::errors::EnumError),
}
impl Display for InitialOpcodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InvalidOpcode(i) => f.write_fmt(format_args!("opcode that is not CMD_AUTH_LOGON_CHALLENGE or CMD_AUTH_RECONNECT_CHALLENGE received: '{}'", i)),
            InitialOpcodeError::String(i) => i.fmt(f),
            InitialOpcodeError::Enum(i) => i.fmt(f),
        }
    }
}
impl std::error::Error for InitialOpcodeError {}

impl From<std::io::Error> for InitialOpcodeError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for InitialOpcodeError {
    fn from(e: FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<crate::errors::EnumError> for InitialOpcodeError {
    fn from(e: EnumError) -> Self {
        Self::Enum(e)
    }
}

impl From<ParseError> for InitialOpcodeError {
    fn from(e: ParseError) -> Self {
        match e {
            ParseError::Io(i) => Self::Io(i),
            ParseError::Enum(i) => Self::Enum(i),
            ParseError::String(i) => Self::String(i),
        }
    }
}
