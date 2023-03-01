use crate:: {
    Guid,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_force_flight_back_speed_change.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_force_flight_back_speed_change.wowm#L1):
/// ```text
/// smsg SMSG_FORCE_FLIGHT_BACK_SPEED_CHANGE = 0x0383 {
///     PackedGuid guid;
///     u32 move_event;
///     f32 speed;
/// }
/// ```
pub struct SMSG_FORCE_FLIGHT_BACK_SPEED_CHANGE {
    pub guid: Guid,
    /// cmangos/mangoszero/vmangos: set to 0
    /// cmangos/mangoszero/vmangos: moveEvent, NUM_PMOVE_EVTS = 0x39
    ///
    pub move_event: u32,
    pub speed: f32,
}

impl crate::Message for SMSG_FORCE_FLIGHT_BACK_SPEED_CHANGE {
    const OPCODE: u32 = 0x0383;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        // move_event: u32
        w.write_all(&self.move_event.to_le_bytes())?;

        // speed: f32
        w.write_all(&self.speed.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(10..=17).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0383, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        // move_event: u32
        let move_event = crate::util::read_u32_le(&mut r)?;

        // speed: f32
        let speed = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            guid,
            move_event,
            speed,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_FORCE_FLIGHT_BACK_SPEED_CHANGE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_FORCE_FLIGHT_BACK_SPEED_CHANGE {}

impl SMSG_FORCE_FLIGHT_BACK_SPEED_CHANGE {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 4 // move_event: u32
        + 4 // speed: f32
    }
}

