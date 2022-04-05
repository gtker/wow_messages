use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new2.wowm:330`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new2.wowm#L330):
/// ```text
/// smsg SMSG_TRAINER_BUY_SUCCEEDED = 0x1B3 {
///     u64 guid;
///     u32 spell_id;
/// }
/// ```
pub struct SMSG_TRAINER_BUY_SUCCEEDED {
    pub guid: u64,
    pub spell_id: u32,
}

impl WorldServerMessageWrite for SMSG_TRAINER_BUY_SUCCEEDED {
    const OPCODE: u16 = 0x1b3;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_TRAINER_BUY_SUCCEEDED {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        // spell_id: u32
        let spell_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            spell_id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        // spell_id: u32
        w.write_all(&self.spell_id.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_TRAINER_BUY_SUCCEEDED {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_TRAINER_BUY_SUCCEEDED {
    fn maximum_possible_size() -> usize {
        8 // guid: u64
        + 4 // spell_id: u32
    }
}

