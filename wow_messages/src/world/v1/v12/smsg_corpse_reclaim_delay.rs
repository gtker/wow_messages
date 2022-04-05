use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new5.wowm:134`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new5.wowm#L134):
/// ```text
/// smsg SMSG_CORPSE_RECLAIM_DELAY = 0x269 {
///     u32 delay_in_seconds;
/// }
/// ```
pub struct SMSG_CORPSE_RECLAIM_DELAY {
    pub delay_in_seconds: u32,
}

impl WorldServerMessageWrite for SMSG_CORPSE_RECLAIM_DELAY {
    const OPCODE: u16 = 0x269;

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
impl WorldMessageBody for SMSG_CORPSE_RECLAIM_DELAY {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // delay_in_seconds: u32
        let delay_in_seconds = crate::util::read_u32_le(r)?;

        Ok(Self {
            delay_in_seconds,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // delay_in_seconds: u32
        w.write_all(&self.delay_in_seconds.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_CORPSE_RECLAIM_DELAY {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_CORPSE_RECLAIM_DELAY {
    fn maximum_possible_size() -> usize {
        4 // delay_in_seconds: u32
    }
}

