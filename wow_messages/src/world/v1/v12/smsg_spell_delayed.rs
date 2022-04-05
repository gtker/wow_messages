use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new2.wowm:387`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new2.wowm#L387):
/// ```text
/// smsg SMSG_SPELL_DELAYED = 0x1E2 {
///     u64 guid;
///     u32 delay_time;
/// }
/// ```
pub struct SMSG_SPELL_DELAYED {
    pub guid: u64,
    pub delay_time: u32,
}

impl WorldServerMessageWrite for SMSG_SPELL_DELAYED {
    const OPCODE: u16 = 0x1e2;

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
impl WorldMessageBody for SMSG_SPELL_DELAYED {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        // delay_time: u32
        let delay_time = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            delay_time,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        // delay_time: u32
        w.write_all(&self.delay_time.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_SPELL_DELAYED {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_SPELL_DELAYED {
    fn maximum_possible_size() -> usize {
        8 // guid: u64
        + 4 // delay_time: u32
    }
}

