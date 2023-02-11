use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_query_quests_completed_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_query_quests_completed_response.wowm#L1):
/// ```text
/// smsg SMSG_QUERY_QUESTS_COMPLETED_RESPONSE = 0x0501 {
///     u32 amount_of_reward_quests;
///     u32[amount_of_reward_quests] reward_quests;
/// }
/// ```
pub struct SMSG_QUERY_QUESTS_COMPLETED_RESPONSE {
    pub reward_quests: Vec<u32>,
}

impl crate::Message for SMSG_QUERY_QUESTS_COMPLETED_RESPONSE {
    const OPCODE: u32 = 0x0501;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // amount_of_reward_quests: u32
        w.write_all(&(self.reward_quests.len() as u32).to_le_bytes())?;

        // reward_quests: u32[amount_of_reward_quests]
        for i in self.reward_quests.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0501, size: body_size as u32 });
        }

        // amount_of_reward_quests: u32
        let amount_of_reward_quests = crate::util::read_u32_le(r)?;

        // reward_quests: u32[amount_of_reward_quests]
        let mut reward_quests = Vec::with_capacity(amount_of_reward_quests as usize);
        for i in 0..amount_of_reward_quests {
            reward_quests.push(crate::util::read_u32_le(r)?);
        }

        Ok(Self {
            reward_quests,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUERY_QUESTS_COMPLETED_RESPONSE {}

impl SMSG_QUERY_QUESTS_COMPLETED_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_reward_quests: u32
        + self.reward_quests.len() * core::mem::size_of::<u32>() // reward_quests: u32[amount_of_reward_quests]
    }
}

