use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gossip/cmsg_gossip_select_option.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gossip/cmsg_gossip_select_option.wowm#L3):
/// ```text
/// cmsg CMSG_GOSSIP_SELECT_OPTION = 0x017C {
///     Guid guid;
///     u32 gossip_list_id;
///     optional unknown {
///         CString code;
///     }
/// }
/// ```
pub struct CMSG_GOSSIP_SELECT_OPTION {
    pub guid: Guid,
    pub gossip_list_id: u32,
    pub unknown: Option<CMSG_GOSSIP_SELECT_OPTION_unknown>,
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

    fn client_size(&self) -> u16 {
        (self.size() + 6) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // gossip_list_id: u32
        let gossip_list_id = crate::util::read_u32_le(r)?;

        // optional unknown
        let current_size = {
            8 // guid: Guid
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
pub struct CMSG_GOSSIP_SELECT_OPTION_unknown {
    pub code: String,
}

impl CMSG_GOSSIP_SELECT_OPTION_unknown {
    pub(crate) fn size(&self) -> usize {
        self.code.len() + 1 // code: CString
    }

}

