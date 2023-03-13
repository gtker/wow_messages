use std::io::{Read, Write};

use crate::shared::faction_initializer_vanilla_tbc::FactionInitializer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_initialize_factions.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_initialize_factions.wowm#L9):
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

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // amount_of_factions: u32
        w.write_all(&(self.factions.len() as u32).to_le_bytes())?;

        // factions: FactionInitializer[amount_of_factions]
        for i in self.factions.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=65535).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0122, size: body_size as u32 });
        }

        // amount_of_factions: u32
        let amount_of_factions = crate::util::read_u32_le(&mut r)?;

        // factions: FactionInitializer[amount_of_factions]
        let factions = {
            let mut factions = Vec::with_capacity(amount_of_factions as usize);
            for i in 0..amount_of_factions {
                factions.push(FactionInitializer::read(&mut r)?);
            }
            factions
        };

        Ok(Self {
            factions,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_INITIALIZE_FACTIONS {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_INITIALIZE_FACTIONS {}

impl SMSG_INITIALIZE_FACTIONS {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_factions: u32
        + self.factions.len() * 5 // factions: FactionInitializer[amount_of_factions]
    }
}

