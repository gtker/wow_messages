use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Sent to acknowledge the new speed. Reply to [`SMSG_FORCE_RUN_SPEED_CHANGE`](crate::tbc::SMSG_FORCE_RUN_SPEED_CHANGE).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_force_run_speed_change_ack.wowm:44`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_force_run_speed_change_ack.wowm#L44):
/// ```text
/// cmsg CMSG_FORCE_RUN_SPEED_CHANGE_ACK = 0x00E3 {
///     Guid guid;
///     u32 counter;
///     MovementInfo info;
///     f32 new_speed;
/// }
/// ```
pub struct CMSG_FORCE_RUN_SPEED_CHANGE_ACK {
    pub guid: Guid,
    pub counter: u32,
    pub info: MovementInfo,
    pub new_speed: f32,
}

impl crate::Message for CMSG_FORCE_RUN_SPEED_CHANGE_ACK {
    const OPCODE: u32 = 0x00e3;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        // new_speed: f32
        w.write_all(&self.new_speed.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(45..=98).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00E3, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        // new_speed: f32
        let new_speed = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            guid,
            counter,
            info,
            new_speed,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_FORCE_RUN_SPEED_CHANGE_ACK {}

impl CMSG_FORCE_RUN_SPEED_CHANGE_ACK {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // counter: u32
        + self.info.size() // info: MovementInfo
        + 4 // new_speed: f32
    }
}

