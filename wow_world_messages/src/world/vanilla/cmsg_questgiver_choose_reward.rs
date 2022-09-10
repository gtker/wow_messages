use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/cmsg_questgiver_choose_reward.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/cmsg_questgiver_choose_reward.wowm#L3):
/// ```text
/// cmsg CMSG_QUESTGIVER_CHOOSE_REWARD = 0x018E {
///     Guid guid;
///     u32 quest_id;
///     u32 reward;
/// }
/// ```
pub struct CMSG_QUESTGIVER_CHOOSE_REWARD {
    pub guid: Guid,
    pub quest_id: u32,
    pub reward: u32,
}

impl crate::Message for CMSG_QUESTGIVER_CHOOSE_REWARD {
    const OPCODE: u32 = 0x018e;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // reward: u32
        w.write_all(&self.reward.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // reward: u32
        let reward = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            quest_id,
            reward,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ClientMessage for CMSG_QUESTGIVER_CHOOSE_REWARD {}

