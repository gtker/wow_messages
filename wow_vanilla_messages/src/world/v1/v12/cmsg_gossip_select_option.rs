use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_GOSSIP_SELECT_OPTION {
    pub guid: Guid,
    pub gossip_list_id: u32,
    pub unknown: Option<CMSG_GOSSIP_SELECT_OPTION_unknown>,
}

impl ClientMessageWrite for CMSG_GOSSIP_SELECT_OPTION {
    const OPCODE: u32 = 0x17c;

    fn size_without_size_field(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for CMSG_GOSSIP_SELECT_OPTION {
    type Error = CMSG_GOSSIP_SELECT_OPTIONError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // gossip_list_id: u32
        let gossip_list_id = crate::util::read_u32_le(r)?;

        // optional unknown
        let current_size = {
            0 // If no fields are present, TODO remove when not needed
            + 8 // guid: Guid
            + 4 // gossip_list_id: u32
        };
        let unknown = if current_size < body_size as usize {
            // code: CString
            let code = crate::util::read_c_string_to_vec(r)?;
            let code = String::from_utf8(code)?;

            Some(CMSG_GOSSIP_SELECT_OPTION_unknown {
                code,
            })
        } else {
            None
        };

        Ok(Self {
            guid,
            gossip_list_id,
            unknown,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // gossip_list_id: u32
        w.write_all(&self.gossip_list_id.to_le_bytes())?;

        // optional unknown
        if let Some(v) = &self.unknown {
            // code: CString
            w.write_all(v.code.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

        }

        Ok(())
    }
}

impl VariableSized for CMSG_GOSSIP_SELECT_OPTION {
    fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // gossip_list_id: u32
        + {
            if let Some(v) = &self.unknown {
                v.size()
            } else {
                0
            }
        } // optional unknown
    }
}

impl MaximumPossibleSized for CMSG_GOSSIP_SELECT_OPTION {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // gossip_list_id: u32
        + 65536 // optional unknown
    }
}

#[derive(Debug)]
pub enum CMSG_GOSSIP_SELECT_OPTIONError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_GOSSIP_SELECT_OPTIONError {}
impl std::fmt::Display for CMSG_GOSSIP_SELECT_OPTIONError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GOSSIP_SELECT_OPTIONError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GOSSIP_SELECT_OPTIONError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CMSG_GOSSIP_SELECT_OPTION_unknown {
    pub code: String,
}

impl CMSG_GOSSIP_SELECT_OPTION_unknown {
    pub fn size(&self) -> usize {
        self.code.len() + 1 // code: CString and Null Terminator
    }
}

