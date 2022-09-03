use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::Faction;
use crate::world::vanilla::ServerMessage;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm#L9):
/// ```text
/// smsg SMSG_SET_FACTION_STANDING = 0x0124 {
///     u32 amount_of_factions;
///     Faction[amount_of_factions] factions;
/// }
/// ```
pub struct SMSG_SET_FACTION_STANDING {
    pub factions: Vec<Faction>,
}

impl crate::Message for SMSG_SET_FACTION_STANDING {
    const OPCODE: u32 = 0x0124;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount_of_factions: u32
        w.write_all(&(self.factions.len() as u32).to_le_bytes())?;

        // factions: Faction[amount_of_factions]
        for i in self.factions.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // amount_of_factions: u32
        let amount_of_factions = crate::util::read_u32_le(r)?;

        // factions: Faction[amount_of_factions]
        let mut factions = Vec::with_capacity(amount_of_factions as usize);
        for i in 0..amount_of_factions {
            factions.push(Faction::read(r)?);
        }

        Ok(Self {
            factions,
        })
    }

}
impl ServerMessage for SMSG_SET_FACTION_STANDING {}

impl SMSG_SET_FACTION_STANDING {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_factions: u32
        + self.factions.len() * 8 // factions: Faction[amount_of_factions]
    }
}

