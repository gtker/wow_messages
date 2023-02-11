use crate::tbc::Faction;
use crate::tbc::FactionFlag;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/cmsg_set_faction_atwar.wowm:28`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/cmsg_set_faction_atwar.wowm#L28):
/// ```text
/// cmsg CMSG_SET_FACTION_ATWAR = 0x0125 {
///     Faction faction;
///     FactionFlag flags;
/// }
/// ```
pub struct CMSG_SET_FACTION_ATWAR {
    pub faction: Faction,
    pub flags: FactionFlag,
}

impl crate::Message for CMSG_SET_FACTION_ATWAR {
    const OPCODE: u32 = 0x0125;

    fn size_without_header(&self) -> u32 {
        3
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // faction: Faction
        w.write_all(&(self.faction.as_int() as u16).to_le_bytes())?;

        // flags: FactionFlag
        w.write_all(&(self.flags.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 3 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0125, size: body_size as u32 });
        }

        // faction: Faction
        let faction: Faction = crate::util::read_u16_le(r)?.try_into()?;

        // flags: FactionFlag
        let flags = FactionFlag::new(crate::util::read_u8_le(r)?);

        Ok(Self {
            faction,
            flags,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_FACTION_ATWAR {}

