use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gossip/cmsg_gossip_select_option.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gossip/cmsg_gossip_select_option.wowm#L3):
/// ```text
/// cmsg CMSG_GOSSIP_SELECT_OPTION = 0x17C {
///     Guid guid;
///     u32 gossip_list_id;
///     OPTIONAL-STATEMENT-DOCC: unimplemented
/// }
/// ```
pub struct CMSG_GOSSIP_SELECT_OPTION {
    pub guid: Guid,
    pub gossip_list_id: u32,
    pub unknown: Option<CMSG_GOSSIP_SELECT_OPTION_unknown>,
}

impl WorldClientMessageWrite for CMSG_GOSSIP_SELECT_OPTION {
    const OPCODE: u32 = 0x17c;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (self.size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (self.size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_GOSSIP_SELECT_OPTION {
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

