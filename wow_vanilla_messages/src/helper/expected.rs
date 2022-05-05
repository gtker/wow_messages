use std::fmt::{Display, Formatter};
use std::io::{Error, Read};

use crate::util::{read_u16_le, read_u32_le};
use crate::{ClientMessageWrite, MessageBody, ServerMessageWrite};

#[cfg(feature = "sync")]
pub fn read_expected_client_world_message<M: ClientMessageWrite + MessageBody, R: Read>(
    r: &mut R,
) -> Result<M, ExpectedClientWorldMessageError> {
    let size = read_u16_le(r)?;
    let opcode = read_u32_le(r)?;

    return if opcode == M::OPCODE as u32 {
        let m = M::read_body(r, (size - 4) as u32);
        match m {
            Ok(m) => Ok(m),
            Err(_) => Err(ExpectedClientWorldMessageError::GenericError),
        }
    } else {
        Err(ExpectedClientWorldMessageError::UnexpectedOpcode(opcode))
    };
}

#[derive(Debug)]
pub enum ExpectedClientWorldMessageError {
    Io(std::io::Error),
    UnexpectedOpcode(u32),
    GenericError,
}

impl std::error::Error for ExpectedClientWorldMessageError {}

impl Display for ExpectedClientWorldMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectedClientWorldMessageError::Io(i) => i.fmt(f),
            ExpectedClientWorldMessageError::UnexpectedOpcode(i) => {
                f.write_fmt(format_args!("unexpected opcode returned: '{}'", i))
            }
            ExpectedClientWorldMessageError::GenericError => {
                f.write_str("something went wrong parsing the message")
            }
        }
    }
}

impl From<std::io::Error> for ExpectedClientWorldMessageError {
    fn from(e: Error) -> Self {
        Self::Io(e)
    }
}

#[cfg(feature = "sync")]
pub fn read_expected_server_world_message<M: ServerMessageWrite + MessageBody, R: Read>(
    r: &mut R,
) -> Result<M, ExpectedServerWorldMessageError> {
    let size = read_u16_le(r)?;
    let opcode = read_u16_le(r)?;

    return if opcode == M::OPCODE {
        let m = M::read_body(r, (size - 2) as u32);
        match m {
            Ok(m) => Ok(m),
            Err(_) => Err(ExpectedServerWorldMessageError::GenericError),
        }
    } else {
        Err(ExpectedServerWorldMessageError::UnexpectedOpcode(opcode))
    };
}

#[derive(Debug)]
pub enum ExpectedServerWorldMessageError {
    Io(std::io::Error),
    UnexpectedOpcode(u16),
    GenericError,
}

impl std::error::Error for ExpectedServerWorldMessageError {}

impl Display for ExpectedServerWorldMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectedServerWorldMessageError::Io(i) => i.fmt(f),
            ExpectedServerWorldMessageError::UnexpectedOpcode(i) => {
                f.write_fmt(format_args!("unexpected opcode returned: '{}'", i))
            }
            ExpectedServerWorldMessageError::GenericError => {
                f.write_str("something went wrong parsing the message")
            }
        }
    }
}

impl From<std::io::Error> for ExpectedServerWorldMessageError {
    fn from(e: Error) -> Self {
        Self::Io(e)
    }
}
