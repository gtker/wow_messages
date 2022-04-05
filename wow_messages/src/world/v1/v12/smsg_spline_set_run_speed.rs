use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/movement.wowm:57`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/movement.wowm#L57):
/// ```text
/// smsg SMSG_SPLINE_SET_RUN_SPEED = 0x2FE {
///     PackedGuid guid;
///     f32 speed;
/// }
/// ```
pub struct SMSG_SPLINE_SET_RUN_SPEED {
    pub guid: Guid,
    pub speed: f32,
}

impl WorldServerMessageWrite for SMSG_SPLINE_SET_RUN_SPEED {
    const OPCODE: u16 = 0x2fe;

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
impl WorldMessageBody for SMSG_SPLINE_SET_RUN_SPEED {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // speed: f32
        let speed = crate::util::read_f32_le(r)?;
        Ok(Self {
            guid,
            speed,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed(w)?;

        // speed: f32
        w.write_all(&self.speed.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_SPLINE_SET_RUN_SPEED {
    fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
        + 4 // speed: f32
    }
}

impl MaximumPossibleSized for SMSG_SPLINE_SET_RUN_SPEED {
    fn maximum_possible_size() -> usize {
        9 // guid: PackedGuid
        + 4 // speed: f32
    }
}

