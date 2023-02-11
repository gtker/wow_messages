use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_queue_status.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_queue_status.wowm#L1):
/// ```text
/// smsg SMSG_LFG_QUEUE_STATUS = 0x0365 {
///     u32 dungeon;
///     i32 average_wait_time;
///     i32 wait_time;
///     i32 wait_time_tank;
///     i32 wait_time_healer;
///     i32 wait_time_dps;
///     u8 tanks_needed;
///     u8 healers_needed;
///     u8 dps_needed;
///     u32 queue_time;
/// }
/// ```
pub struct SMSG_LFG_QUEUE_STATUS {
    pub dungeon: u32,
    pub average_wait_time: i32,
    pub wait_time: i32,
    pub wait_time_tank: i32,
    pub wait_time_healer: i32,
    pub wait_time_dps: i32,
    pub tanks_needed: u8,
    pub healers_needed: u8,
    pub dps_needed: u8,
    pub queue_time: u32,
}

impl crate::Message for SMSG_LFG_QUEUE_STATUS {
    const OPCODE: u32 = 0x0365;

    fn size_without_header(&self) -> u32 {
        31
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // dungeon: u32
        w.write_all(&self.dungeon.to_le_bytes())?;

        // average_wait_time: i32
        w.write_all(&self.average_wait_time.to_le_bytes())?;

        // wait_time: i32
        w.write_all(&self.wait_time.to_le_bytes())?;

        // wait_time_tank: i32
        w.write_all(&self.wait_time_tank.to_le_bytes())?;

        // wait_time_healer: i32
        w.write_all(&self.wait_time_healer.to_le_bytes())?;

        // wait_time_dps: i32
        w.write_all(&self.wait_time_dps.to_le_bytes())?;

        // tanks_needed: u8
        w.write_all(&self.tanks_needed.to_le_bytes())?;

        // healers_needed: u8
        w.write_all(&self.healers_needed.to_le_bytes())?;

        // dps_needed: u8
        w.write_all(&self.dps_needed.to_le_bytes())?;

        // queue_time: u32
        w.write_all(&self.queue_time.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 31 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0365, size: body_size as u32 });
        }

        // dungeon: u32
        let dungeon = crate::util::read_u32_le(r)?;

        // average_wait_time: i32
        let average_wait_time = crate::util::read_i32_le(r)?;

        // wait_time: i32
        let wait_time = crate::util::read_i32_le(r)?;

        // wait_time_tank: i32
        let wait_time_tank = crate::util::read_i32_le(r)?;

        // wait_time_healer: i32
        let wait_time_healer = crate::util::read_i32_le(r)?;

        // wait_time_dps: i32
        let wait_time_dps = crate::util::read_i32_le(r)?;

        // tanks_needed: u8
        let tanks_needed = crate::util::read_u8_le(r)?;

        // healers_needed: u8
        let healers_needed = crate::util::read_u8_le(r)?;

        // dps_needed: u8
        let dps_needed = crate::util::read_u8_le(r)?;

        // queue_time: u32
        let queue_time = crate::util::read_u32_le(r)?;

        Ok(Self {
            dungeon,
            average_wait_time,
            wait_time,
            wait_time_tank,
            wait_time_healer,
            wait_time_dps,
            tanks_needed,
            healers_needed,
            dps_needed,
            queue_time,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_QUEUE_STATUS {}

