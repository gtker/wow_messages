use std::convert::{TryFrom, TryInto};
use crate::helper::Guid;
use crate::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_start_swim.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_start_swim.wowm#L3):
/// ```text
/// smsg SMSG_SPLINE_MOVE_START_SWIM = 0x30B {
///     PackedGuid guid;
/// }
/// ```
pub struct SMSG_SPLINE_MOVE_START_SWIM {
    pub guid: Guid,
}

impl WorldServerMessageWrite for SMSG_SPLINE_MOVE_START_SWIM {
    const OPCODE: u16 = 0x30b;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_SPLINE_MOVE_START_SWIM {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        Ok(Self {
            guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed(w)?;

        Ok(())
    }
}

impl VariableSized for SMSG_SPLINE_MOVE_START_SWIM {
    fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
    }
}

impl MaximumPossibleSized for SMSG_SPLINE_MOVE_START_SWIM {
    fn maximum_possible_size() -> usize {
        9 // guid: PackedGuid
    }
}

