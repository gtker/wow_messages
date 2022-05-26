use std::fmt::{Display, Formatter};
use std::io;
use std::io::{Error, Read, Write};
use tokio::io::AsyncReadExt;
use wow_srp::header_crypto::{Decrypter, CLIENT_HEADER_LENGTH};

use crate::util::{read_u16_le, read_u32_le, read_u64_le, read_u8_le};
use crate::ClientMessageWrite;
pub use expected::*;

pub(crate) mod aura_mask;
pub(crate) mod expected;
pub(crate) mod guid;
pub(crate) mod update_mask;

const CLIENT_OPCODE_LENGTH: u16 = 4;

#[cfg(feature = "tokio")]
pub async fn tokio_read_expect_client_message_encryption<
    M: ClientMessageWrite,
    R: tokio::io::AsyncReadExt + Unpin + Send,
    D: Decrypter,
>(
    r: &mut R,
    d: &mut D,
) -> Result<M, ExpectedServerLoginMessageError> {
    let mut buf = [0u8; CLIENT_HEADER_LENGTH as usize];
    r.read_exact(&mut buf).await?;
    let d = d.decrypt_client_header(buf);

    let size = d.size;
    let opcode = d.opcode;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode as u16 == M::OPCODE {
        let m = M::tokio_read_body(r, (size - CLIENT_OPCODE_LENGTH) as u32).await;
        match m {
            Ok(m) => Ok(m),
            Err(_) => Err(ExpectedServerLoginMessageError::GenericError),
        }
    } else {
        Err(ExpectedServerLoginMessageError::UnexpectedOpcode(opcode))
    }
}

#[cfg(feature = "tokio")]
pub async fn tokio_read_expect_client_message<
    M: ClientMessageWrite,
    R: tokio::io::AsyncReadExt + Unpin + Send,
>(
    r: &mut R,
) -> Result<M, ExpectedServerLoginMessageError> {
    let size = crate::util::tokio_read_u16_be(r).await?;
    let opcode = crate::util::tokio_read_u32_le(r).await?;

    // Unable to match on associated const M::OPCODE, so we do if
    if opcode as u16 == M::OPCODE {
        let m = M::tokio_read_body(r, (size - CLIENT_OPCODE_LENGTH) as u32).await;
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
    UnexpectedOpcode(u32),
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
