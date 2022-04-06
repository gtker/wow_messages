use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::MovementInfo;
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_spline_done.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_spline_done.wowm#L3):
/// ```text
/// cmsg CMSG_MOVE_SPLINE_DONE = 0x2C9 {
///     MovementInfo movement_info;
///     u32 movement_counter;
///     u32 unknown1;
/// }
/// ```
pub struct CMSG_MOVE_SPLINE_DONE {
    pub movement_info: MovementInfo,
    pub movement_counter: u32,
    pub unknown1: u32,
}

impl WorldClientMessageWrite for CMSG_MOVE_SPLINE_DONE {
    const OPCODE: u32 = 0x2c9;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (self.size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (self.size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_MOVE_SPLINE_DONE {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // movement_info: MovementInfo
        let movement_info = MovementInfo::read(r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        Ok(Self {
            movement_info,
            movement_counter,
            unknown1,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // movement_info: MovementInfo
        self.movement_info.write(w)?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for CMSG_MOVE_SPLINE_DONE {
    fn size(&self) -> usize {
        self.movement_info.size() // movement_info: MovementInfo
        + 4 // movement_counter: u32
        + 4 // unknown1: u32
    }
}

impl MaximumPossibleSized for CMSG_MOVE_SPLINE_DONE {
    fn maximum_possible_size() -> usize {
        MovementInfo::maximum_possible_size() // movement_info: MovementInfo
        + 4 // movement_counter: u32
        + 4 // unknown1: u32
    }
}

