use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::MovementInfo;
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_MOVE_WATER_WALK_ACK {
    pub guid: Guid,
    pub movement_counter: u32,
    pub movement_info: MovementInfo,
    pub apply: u32,
}

impl WorldClientMessageWrite for CMSG_MOVE_WATER_WALK_ACK {
    const OPCODE: u32 = 0x2d0;

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
impl WorldMessageBody for CMSG_MOVE_WATER_WALK_ACK {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(r)?;

        // movement_info: MovementInfo
        let movement_info = MovementInfo::read(r)?;

        // apply: u32
        let apply = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            movement_counter,
            movement_info,
            apply,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // movement_info: MovementInfo
        self.movement_info.write(w)?;

        // apply: u32
        w.write_all(&self.apply.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for CMSG_MOVE_WATER_WALK_ACK {
    fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // movement_counter: u32
        + self.movement_info.size() // movement_info: MovementInfo
        + 4 // apply: u32
    }
}

impl MaximumPossibleSized for CMSG_MOVE_WATER_WALK_ACK {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // movement_counter: u32
        + MovementInfo::maximum_possible_size() // movement_info: MovementInfo
        + 4 // apply: u32
    }
}

