use std::io::{Read, Write};

use crate::vanilla::Faction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_set_faction_visible.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_set_faction_visible.wowm#L1):
/// ```text
/// smsg SMSG_SET_FACTION_VISIBLE = 0x0123 {
///     Faction faction;
/// }
/// ```
pub struct SMSG_SET_FACTION_VISIBLE {
    pub faction: Faction,
}

impl crate::private::Sealed for SMSG_SET_FACTION_VISIBLE {}
impl crate::Message for SMSG_SET_FACTION_VISIBLE {
    const OPCODE: u32 = 0x0123;

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // faction: Faction
        w.write_all(&(self.faction.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0123, size: body_size });
        }

        // faction: Faction
        let faction = crate::util::read_u16_le(&mut r)?.try_into()?;

        Ok(Self {
            faction,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SET_FACTION_VISIBLE {}

