use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::roll_vote_tbc_wrath::RollVote;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/cmsg_loot_roll.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/cmsg_loot_roll.wowm#L9):
/// ```text
/// cmsg CMSG_LOOT_ROLL = 0x02A0 {
///     Guid item;
///     u32 item_slot;
///     RollVote vote;
/// }
/// ```
pub struct CMSG_LOOT_ROLL {
    pub item: Guid,
    pub item_slot: u32,
    pub vote: RollVote,
}

impl crate::private::Sealed for CMSG_LOOT_ROLL {}
impl crate::Message for CMSG_LOOT_ROLL {
    const OPCODE: u32 = 0x02a0;

    fn size_without_header(&self) -> u32 {
        13
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // item_slot: u32
        w.write_all(&self.item_slot.to_le_bytes())?;

        // vote: RollVote
        w.write_all(&(self.vote.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 13 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02A0, size: body_size });
        }

        // item: Guid
        let item = crate::util::read_guid(&mut r)?;

        // item_slot: u32
        let item_slot = crate::util::read_u32_le(&mut r)?;

        // vote: RollVote
        let vote: RollVote = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            item,
            item_slot,
            vote,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_LOOT_ROLL {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LOOT_ROLL {}

