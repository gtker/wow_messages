use std::convert::{TryFrom, TryInto};
use crate::helper::Guid;
use crate::world::v1::v12::MovementInfo;
use crate::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_force_walk_speed_change_ack.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_force_walk_speed_change_ack.wowm#L3):
/// ```text
/// cmsg CMSG_FORCE_WALK_SPEED_CHANGE_ACK = 0x2DB {
///     Guid guid;
///     u32 counter;
///     MovementInfo movement_info;
///     f32 new_speed;
/// }
/// ```
pub struct CMSG_FORCE_WALK_SPEED_CHANGE_ACK {
    pub guid: Guid,
    pub counter: u32,
    pub movement_info: MovementInfo,
    pub new_speed: f32,
}

impl WorldClientMessageWrite for CMSG_FORCE_WALK_SPEED_CHANGE_ACK {
    const OPCODE: u32 = 0x2db;

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
impl WorldMessageBody for CMSG_FORCE_WALK_SPEED_CHANGE_ACK {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(r)?;

        // movement_info: MovementInfo
        let movement_info = MovementInfo::read(r)?;

        // new_speed: f32
        let new_speed = crate::util::read_f32_le(r)?;
        Ok(Self {
            guid,
            counter,
            movement_info,
            new_speed,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        // movement_info: MovementInfo
        self.movement_info.write(w)?;

        // new_speed: f32
        w.write_all(&self.new_speed.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for CMSG_FORCE_WALK_SPEED_CHANGE_ACK {
    fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // counter: u32
        + self.movement_info.size() // movement_info: MovementInfo
        + 4 // new_speed: f32
    }
}

impl MaximumPossibleSized for CMSG_FORCE_WALK_SPEED_CHANGE_ACK {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // counter: u32
        + MovementInfo::maximum_possible_size() // movement_info: MovementInfo
        + 4 // new_speed: f32
    }
}

