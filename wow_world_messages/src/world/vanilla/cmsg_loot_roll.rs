use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::RollVote;
use crate::world::vanilla::ClientMessage;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/cmsg_loot_roll.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/cmsg_loot_roll.wowm#L3):
/// ```text
/// cmsg CMSG_LOOT_ROLL = 0x02A0 {
///     Guid item_guid;
///     u32 item_slot;
///     RollVote vote;
/// }
/// ```
pub struct CMSG_LOOT_ROLL {
    pub item_guid: Guid,
    pub item_slot: u32,
    pub vote: RollVote,
}

impl crate::Message for CMSG_LOOT_ROLL {
    const OPCODE: u32 = 0x02a0;

    fn size_without_header(&self) -> u32 {
        13
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item_guid: Guid
        w.write_all(&self.item_guid.guid().to_le_bytes())?;

        // item_slot: u32
        w.write_all(&self.item_slot.to_le_bytes())?;

        // vote: RollVote
        w.write_all(&(self.vote.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 13 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // item_guid: Guid
        let item_guid = Guid::read(r)?;

        // item_slot: u32
        let item_slot = crate::util::read_u32_le(r)?;

        // vote: RollVote
        let vote: RollVote = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            item_guid,
            item_slot,
            vote,
        })
    }

}
impl ClientMessage for CMSG_LOOT_ROLL {}

