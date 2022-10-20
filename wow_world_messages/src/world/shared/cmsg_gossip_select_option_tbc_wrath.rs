use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gossip/cmsg_gossip_select_option.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gossip/cmsg_gossip_select_option.wowm#L13):
/// ```text
/// cmsg CMSG_GOSSIP_SELECT_OPTION = 0x017C {
///     Guid guid;
///     u32 menu_id;
///     u32 gossip_list_id;
///     optional unknown {
///         CString code;
///     }
/// }
/// ```
pub struct CMSG_GOSSIP_SELECT_OPTION {
    pub guid: Guid,
    pub menu_id: u32,
    pub gossip_list_id: u32,
    pub unknown: Option<CMSG_GOSSIP_SELECT_OPTION_unknown>,
}

impl crate::Message for CMSG_GOSSIP_SELECT_OPTION {
    const OPCODE: u32 = 0x017c;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // menu_id: u32
        w.write_all(&self.menu_id.to_le_bytes())?;

        // gossip_list_id: u32
        w.write_all(&self.gossip_list_id.to_le_bytes())?;

        // optional unknown
        if let Some(v) = &self.unknown {
            // code: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.code.as_bytes().iter().rev().next(), Some(&0_u8), "String `code` must not be null-terminated.");
            w.write_all(v.code.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // menu_id: u32
        let menu_id = crate::util::read_u32_le(r)?;

        // gossip_list_id: u32
        let gossip_list_id = crate::util::read_u32_le(r)?;

        // optional unknown
        let current_size = {
            8 // guid: Guid
            + 4 // menu_id: u32
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
            menu_id,
            gossip_list_id,
            unknown,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_GOSSIP_SELECT_OPTION {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_GOSSIP_SELECT_OPTION {}

impl CMSG_GOSSIP_SELECT_OPTION {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // menu_id: u32
        + 4 // gossip_list_id: u32
        + if let Some(unknown) = &self.unknown {
            unknown.code.len() + 1 // code: CString
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CMSG_GOSSIP_SELECT_OPTION_unknown {
    pub code: String,
}

impl CMSG_GOSSIP_SELECT_OPTION_unknown {
    pub(crate) fn size(&self) -> usize {
        self.code.len() + 1 // code: CString
    }

}

