use std::convert::{TryFrom, TryInto};
use crate::helper::Guid;
use crate::world::v1::v12::MovementInfo;
use crate::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_not_active_mover.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_not_active_mover.wowm#L3):
/// ```text
/// cmsg CMSG_MOVE_NOT_ACTIVE_MOVER = 0x2D1 {
///     Guid old_mover;
///     MovementInfo movement_info;
/// }
/// ```
pub struct CMSG_MOVE_NOT_ACTIVE_MOVER {
    pub old_mover: Guid,
    pub movement_info: MovementInfo,
}

impl WorldClientMessageWrite for CMSG_MOVE_NOT_ACTIVE_MOVER {
    const OPCODE: u32 = 0x2d1;

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
impl WorldMessageBody for CMSG_MOVE_NOT_ACTIVE_MOVER {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // old_mover: Guid
        let old_mover = Guid::read(r)?;

        // movement_info: MovementInfo
        let movement_info = MovementInfo::read(r)?;

        Ok(Self {
            old_mover,
            movement_info,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // old_mover: Guid
        self.old_mover.write(w)?;

        // movement_info: MovementInfo
        self.movement_info.write(w)?;

        Ok(())
    }
}

impl VariableSized for CMSG_MOVE_NOT_ACTIVE_MOVER {
    fn size(&self) -> usize {
        8 // old_mover: Guid
        + self.movement_info.size() // movement_info: MovementInfo
    }
}

impl MaximumPossibleSized for CMSG_MOVE_NOT_ACTIVE_MOVER {
    fn maximum_possible_size() -> usize {
        8 // old_mover: Guid
        + MovementInfo::maximum_possible_size() // movement_info: MovementInfo
    }
}

