use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::map::{Map, map_try_from, map_as_int};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm#L3):
/// ```text
/// smsg SMSG_BATTLEFIELD_LIST = 0x023D {
///     Guid battlemaster;
///     Map map;
///     u8 unknown1;
///     u32 unknown2;
///     u8 unknown3;
///     u32 number_of_battlegrounds;
///     u32[number_of_battlegrounds] battlegrounds;
/// }
/// ```
pub struct SMSG_BATTLEFIELD_LIST {
    pub battlemaster: Guid,
    pub map: Map,
    pub unknown1: u8,
    /// vmangos: number of bg instances, this is also present on the number_of_battlegrounds field
    ///
    pub unknown2: u32,
    pub unknown3: u8,
    pub battlegrounds: Vec<u32>,
}

impl ServerMessage for SMSG_BATTLEFIELD_LIST {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // battlemaster: Guid
        w.write_all(&self.battlemaster.guid().to_le_bytes())?;

        // map: Map
        w.write_all(&(map_as_int(&self.map) as u32).to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: u8
        w.write_all(&self.unknown3.to_le_bytes())?;

        // number_of_battlegrounds: u32
        w.write_all(&(self.battlegrounds.len() as u32).to_le_bytes())?;

        // battlegrounds: u32[number_of_battlegrounds]
        for i in self.battlegrounds.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x023d;

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // battlemaster: Guid
        let battlemaster = Guid::read(r)?;

        // map: Map
        let map: Map = map_try_from(crate::util::read_u32_le(r)?)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(r)?;

        // unknown3: u8
        let unknown3 = crate::util::read_u8_le(r)?;

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
            unknown1,
            unknown2,
            unknown3,
            battlegrounds,
        })
    }

}

impl SMSG_BATTLEFIELD_LIST {
    pub(crate) fn size(&self) -> usize {
        8 // battlemaster: Guid
        + 4 // map: Map
        + 1 // unknown1: u8
        + 4 // unknown2: u32
        + 1 // unknown3: u8
        + 4 // number_of_battlegrounds: u32
        + self.battlegrounds.len() * core::mem::size_of::<u32>() // battlegrounds: u32[number_of_battlegrounds]
    }
}

