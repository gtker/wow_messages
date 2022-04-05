use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new_all.wowm:1915`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new_all.wowm#L1915):
/// ```text
/// smsg SMSG_UPDATE_AURA_DURATION = 0x137 {
///     u8 aura_slot;
///     u32 aura_duration;
/// }
/// ```
pub struct SMSG_UPDATE_AURA_DURATION {
    pub aura_slot: u8,
    pub aura_duration: u32,
}

impl WorldServerMessageWrite for SMSG_UPDATE_AURA_DURATION {
    const OPCODE: u16 = 0x137;

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
impl WorldMessageBody for SMSG_UPDATE_AURA_DURATION {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // aura_slot: u8
        let aura_slot = crate::util::read_u8_le(r)?;

        // aura_duration: u32
        let aura_duration = crate::util::read_u32_le(r)?;

        Ok(Self {
            aura_slot,
            aura_duration,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // aura_slot: u8
        w.write_all(&self.aura_slot.to_le_bytes())?;

        // aura_duration: u32
        w.write_all(&self.aura_duration.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_UPDATE_AURA_DURATION {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_UPDATE_AURA_DURATION {
    fn maximum_possible_size() -> usize {
        1 // aura_slot: u8
        + 4 // aura_duration: u32
    }
}

