use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new_all.wowm:2563`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new_all.wowm#L2563):
/// ```text
/// smsg SMSG_PARTYKILLLOG = 0x1F5 {
///     u64 player_with_killing_blow;
///     u64 victim;
/// }
/// ```
pub struct SMSG_PARTYKILLLOG {
    pub player_with_killing_blow: u64,
    pub victim: u64,
}

impl WorldServerMessageWrite for SMSG_PARTYKILLLOG {
    const OPCODE: u16 = 0x1f5;

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
impl WorldMessageBody for SMSG_PARTYKILLLOG {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // player_with_killing_blow: u64
        let player_with_killing_blow = crate::util::read_u64_le(r)?;

        // victim: u64
        let victim = crate::util::read_u64_le(r)?;

        Ok(Self {
            player_with_killing_blow,
            victim,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player_with_killing_blow: u64
        w.write_all(&self.player_with_killing_blow.to_le_bytes())?;

        // victim: u64
        w.write_all(&self.victim.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_PARTYKILLLOG {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_PARTYKILLLOG {
    fn maximum_possible_size() -> usize {
        8 // player_with_killing_blow: u64
        + 8 // victim: u64
    }
}

