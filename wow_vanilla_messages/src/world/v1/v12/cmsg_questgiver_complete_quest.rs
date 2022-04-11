use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/cmsg_questgiver_complete_quest.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/cmsg_questgiver_complete_quest.wowm#L3):
/// ```text
/// cmsg CMSG_QUESTGIVER_COMPLETE_QUEST = 0x18A {
///     Guid guid;
///     u32 quest_id;
/// }
/// ```
pub struct CMSG_QUESTGIVER_COMPLETE_QUEST {
    pub guid: Guid,
    pub quest_id: u32,
}

impl WorldClientMessageWrite for CMSG_QUESTGIVER_COMPLETE_QUEST {
    const OPCODE: u32 = 0x18a;

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
impl WorldMessageBody for CMSG_QUESTGIVER_COMPLETE_QUEST {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            quest_id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_QUESTGIVER_COMPLETE_QUEST {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_QUESTGIVER_COMPLETE_QUEST {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // quest_id: u32
    }
}

