use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_master_list.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_master_list.wowm#L3):
/// ```text
/// smsg SMSG_LOOT_MASTER_LIST = 0x02A4 {
///     u8 amount_of_players;
///     Guid[amount_of_players] guids;
/// }
/// ```
pub struct SMSG_LOOT_MASTER_LIST {
    pub guids: Vec<Guid>,
}

impl crate::Message for SMSG_LOOT_MASTER_LIST {
    const OPCODE: u32 = 0x02a4;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount_of_players: u8
        w.write_all(&(self.guids.len() as u8).to_le_bytes())?;

        // guids: Guid[amount_of_players]
        for i in self.guids.iter() {
            w.write_all(&i.guid().to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // amount_of_players: u8
        let amount_of_players = crate::util::read_u8_le(r)?;

        // guids: Guid[amount_of_players]
        let mut guids = Vec::with_capacity(amount_of_players as usize);
        for i in 0..amount_of_players {
            guids.push(Guid::read(r)?);
        }

        Ok(Self {
            guids,
        })
    }

}
impl ServerMessage for SMSG_LOOT_MASTER_LIST {}

impl SMSG_LOOT_MASTER_LIST {
    pub(crate) fn size(&self) -> usize {
        1 // amount_of_players: u8
        + self.guids.iter().fold(0, |acc, _| acc + 8) // guids: Guid[amount_of_players]
    }
}

