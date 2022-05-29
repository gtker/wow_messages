use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_GOSSIP_SELECT_OPTION {
    pub guid: Guid,
    pub gossip_list_id: u32,
    pub unknown: Option<CMSG_GOSSIP_SELECT_OPTIONunknown>,
}

impl ClientMessage for CMSG_GOSSIP_SELECT_OPTION {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

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
    const OPCODE: u16 = 0x017c;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // gossip_list_id: u32
        let gossip_list_id = crate::util::read_u32_le(r)?;

        // optional unknown
        let current_size = {
            0
            + 8 // guid: Guid
            + 4 // gossip_list_id: u32
        };
        let unknown = if current_size < body_size as usize {
            // code: CString
            let code = crate::util::read_c_string_to_vec(r)?;
            let code = String::from_utf8(code)?;

            Some(CMSG_GOSSIP_SELECT_OPTIONunknown {
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

}

impl CMSG_GOSSIP_SELECT_OPTION {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // gossip_list_id: u32
        + if let Some(unknown) = &self.unknown {
            unknown.code.len() + 1 // code: CString
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CMSG_GOSSIP_SELECT_OPTIONunknown {
    pub code: String,
}

impl CMSG_GOSSIP_SELECT_OPTIONunknown {
    pub(crate) fn size(&self) -> usize {
        self.code.len() + 1 // code: CString
    }

}

