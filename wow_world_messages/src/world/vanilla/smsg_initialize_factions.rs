use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::FactionInitializer;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_initialize_factions.wowm:33`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_initialize_factions.wowm#L33):
/// ```text
/// smsg SMSG_INITIALIZE_FACTIONS = 0x0122 {
///     u32 amount_of_factions;
///     FactionInitializer[amount_of_factions] factions;
/// }
/// ```
pub struct SMSG_INITIALIZE_FACTIONS {
    pub factions: Vec<FactionInitializer>,
}

impl crate::Message for SMSG_INITIALIZE_FACTIONS {
    const OPCODE: u32 = 0x0122;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount_of_factions: u32
        w.write_all(&(self.factions.len() as u32).to_le_bytes())?;

        // factions: FactionInitializer[amount_of_factions]
        for i in self.factions.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // amount_of_factions: u32
        let amount_of_factions = crate::util::read_u32_le(r)?;

        // factions: FactionInitializer[amount_of_factions]
        let mut factions = Vec::with_capacity(amount_of_factions as usize);
        for i in 0..amount_of_factions {
            factions.push(FactionInitializer::read(r)?);
        }

        Ok(Self {
            factions,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ServerMessage for SMSG_INITIALIZE_FACTIONS {}

impl SMSG_INITIALIZE_FACTIONS {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_factions: u32
        + self.factions.len() * 5 // factions: FactionInitializer[amount_of_factions]
    }
}

