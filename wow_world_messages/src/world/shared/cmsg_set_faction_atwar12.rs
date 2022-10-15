use std::convert::{TryFrom, TryInto};
use crate::world::shared::faction_flag12::FactionFlag;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/cmsg_set_faction_atwar.wowm:28`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/cmsg_set_faction_atwar.wowm#L28):
/// ```text
/// cmsg CMSG_SET_FACTION_ATWAR = 0x0125 {
///     u32 reputation_list_id;
///     FactionFlag flags;
/// }
/// ```
pub struct CMSG_SET_FACTION_ATWAR {
    pub reputation_list_id: u32,
    pub flags: FactionFlag,
}

impl crate::Message for CMSG_SET_FACTION_ATWAR {
    const OPCODE: u32 = 0x0125;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes())?;

        // flags: FactionFlag
        w.write_all(&(self.flags.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // reputation_list_id: u32
        let reputation_list_id = crate::util::read_u32_le(r)?;

        // flags: FactionFlag
        let flags = FactionFlag::new(crate::util::read_u8_le(r)?);

        Ok(Self {
            reputation_list_id,
            flags,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_SET_FACTION_ATWAR {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_SET_FACTION_ATWAR {}

