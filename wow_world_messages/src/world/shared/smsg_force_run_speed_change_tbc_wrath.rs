use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Tells the client that the running speed has changed.
/// Client replies with [`CMSG_FORCE_RUN_SPEED_CHANGE_ACK`](crate::tbc::CMSG_FORCE_RUN_SPEED_CHANGE_ACK).
/// vmangos sends this message to the client being changed and [`SMSG_SPLINE_SET_RUN_SPEED`](crate::vanilla::SMSG_SPLINE_SET_RUN_SPEED) to others.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_force_run_speed_change.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_force_run_speed_change.wowm#L30):
/// ```text
/// smsg SMSG_FORCE_RUN_SPEED_CHANGE = 0x00E2 {
///     PackedGuid guid;
///     u32 move_event;
///     u8 unknown;
///     f32 speed;
/// }
/// ```
pub struct SMSG_FORCE_RUN_SPEED_CHANGE {
    pub guid: Guid,
    /// cmangos/mangoszero/vmangos: set to 0
    /// cmangos/mangoszero/vmangos: moveEvent, NUM_PMOVE_EVTS = 0x39
    ///
    pub move_event: u32,
    /// mangosone sets to 0
    /// mangosone: new 2.1.0
    ///
    pub unknown: u8,
    pub speed: f32,
}

impl crate::Message for SMSG_FORCE_RUN_SPEED_CHANGE {
    const OPCODE: u32 = 0x00e2;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        // move_event: u32
        w.write_all(&self.move_event.to_le_bytes())?;

        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        // speed: f32
        w.write_all(&self.speed.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(11..=18).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00E2, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        // move_event: u32
        let move_event = crate::util::read_u32_le(&mut r)?;

        // unknown: u8
        let unknown = crate::util::read_u8_le(&mut r)?;

        // speed: f32
        let speed = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            guid,
            move_event,
            unknown,
            speed,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_FORCE_RUN_SPEED_CHANGE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_FORCE_RUN_SPEED_CHANGE {}

impl SMSG_FORCE_RUN_SPEED_CHANGE {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
        + 4 // move_event: u32
        + 1 // unknown: u8
        + 4 // speed: f32
    }
}

