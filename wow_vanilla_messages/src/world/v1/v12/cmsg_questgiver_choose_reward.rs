use std::convert::{TryFrom, TryInto};
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/cmsg_questgiver_choose_reward.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/cmsg_questgiver_choose_reward.wowm#L3):
/// ```text
/// cmsg CMSG_QUESTGIVER_CHOOSE_REWARD = 0x18E {
///     u64 guid;
///     u32 quest_id;
///     u32 reward;
/// }
/// ```
pub struct CMSG_QUESTGIVER_CHOOSE_REWARD {
    pub guid: u64,
    pub quest_id: u32,
    pub reward: u32,
}

impl WorldClientMessageWrite for CMSG_QUESTGIVER_CHOOSE_REWARD {
    const OPCODE: u32 = 0x18e;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (Self::size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (Self::size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_QUESTGIVER_CHOOSE_REWARD {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // reward: u32
        w.write_all(&self.reward.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_QUESTGIVER_CHOOSE_REWARD {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_QUESTGIVER_CHOOSE_REWARD {
    fn maximum_possible_size() -> usize {
        8 // guid: u64
        + 4 // quest_id: u32
        + 4 // reward: u32
    }
}

