use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::MovementInfo;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_force_run_back_speed_change_ack.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_force_run_back_speed_change_ack.wowm#L3):
/// ```text
/// cmsg CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK = 0x00E5 {
///     Guid guid;
///     u32 counter;
///     MovementInfo info;
///     f32 new_speed;
/// }
/// ```
pub struct CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK {
    pub guid: Guid,
    pub counter: u32,
    pub info: MovementInfo,
    pub new_speed: f32,
}

impl crate::Message for CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK {
    const OPCODE: u32 = 0x00e5;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(w)?;

        // new_speed: f32
        w.write_all(&self.new_speed.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(r)?;

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        // new_speed: f32
        let new_speed = crate::util::read_f32_le(r)?;
        Ok(Self {
            guid,
            counter,
            info,
            new_speed,
        })
    }

}
impl ClientMessage for CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK {}

impl CMSG_FORCE_RUN_BACK_SPEED_CHANGE_ACK {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // counter: u32
        + self.info.size() // info: MovementInfo
        + 4 // new_speed: f32
    }
}

