use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::v1::v12::MovementInfo;
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/movement.wowm:63`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/movement.wowm):
/// ```text
/// cmsg CMSG_MOVE_KNOCK_BACK_ACK = 0xF0 {
///     Guid guid;
///     u32 counter;
///     MovementInfo movement_info;
/// }
/// ```
pub struct CMSG_MOVE_KNOCK_BACK_ACK {
    pub guid: Guid,
    pub counter: u32,
    pub movement_info: MovementInfo,
}

impl WorldClientMessageWrite for CMSG_MOVE_KNOCK_BACK_ACK {
    const OPCODE: u32 = 0xf0;

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
impl WorldMessageBody for CMSG_MOVE_KNOCK_BACK_ACK {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(r)?;

        // movement_info: MovementInfo
        let movement_info = MovementInfo::read(r)?;

        Ok(Self {
            guid,
            counter,
            movement_info,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        // movement_info: MovementInfo
        self.movement_info.write(w)?;

        Ok(())
    }
}

impl VariableSized for CMSG_MOVE_KNOCK_BACK_ACK {
    fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // counter: u32
        + self.movement_info.size() // movement_info: MovementInfo
    }
}

impl MaximumPossibleSized for CMSG_MOVE_KNOCK_BACK_ACK {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // counter: u32
        + MovementInfo::maximum_possible_size() // movement_info: MovementInfo
    }
}

