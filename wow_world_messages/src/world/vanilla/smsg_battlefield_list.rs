use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::BattlegroundBracket;
use crate::world::vanilla::Map;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm:24`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm#L24):
/// ```text
/// smsg SMSG_BATTLEFIELD_LIST = 0x023D {
///     Guid battlemaster;
///     Map map;
///     BattlegroundBracket bracket;
///     u32 number_of_battlegrounds;
///     u32[number_of_battlegrounds] battlegrounds;
/// }
/// ```
pub struct SMSG_BATTLEFIELD_LIST {
    pub battlemaster: Guid,
    pub map: Map,
    pub bracket: BattlegroundBracket,
    pub battlegrounds: Vec<u32>,
}

impl crate::Message for SMSG_BATTLEFIELD_LIST {
    const OPCODE: u32 = 0x023d;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // battlemaster: Guid
        w.write_all(&self.battlemaster.guid().to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // bracket: BattlegroundBracket
        w.write_all(&(self.bracket.as_int() as u8).to_le_bytes())?;

        // number_of_battlegrounds: u32
        w.write_all(&(self.battlegrounds.len() as u32).to_le_bytes())?;

        // battlegrounds: u32[number_of_battlegrounds]
        for i in self.battlegrounds.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // battlemaster: Guid
        let battlemaster = Guid::read(r)?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // bracket: BattlegroundBracket
        let bracket: BattlegroundBracket = crate::util::read_u8_le(r)?.try_into()?;

        // number_of_battlegrounds: u32
        let number_of_battlegrounds = crate::util::read_u32_le(r)?;

        // battlegrounds: u32[number_of_battlegrounds]
        let mut battlegrounds = Vec::with_capacity(number_of_battlegrounds as usize);
        for i in 0..number_of_battlegrounds {
            battlegrounds.push(crate::util::read_u32_le(r)?);
        }

        Ok(Self {
            battlemaster,
            map,
            bracket,
            battlegrounds,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_BATTLEFIELD_LIST {}

impl SMSG_BATTLEFIELD_LIST {
    pub(crate) fn size(&self) -> usize {
        8 // battlemaster: Guid
        + 4 // map: Map
        + 1 // bracket: BattlegroundBracket
        + 4 // number_of_battlegrounds: u32
        + self.battlegrounds.len() * core::mem::size_of::<u32>() // battlegrounds: u32[number_of_battlegrounds]
    }
}

