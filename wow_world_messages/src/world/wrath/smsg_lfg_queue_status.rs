use std::io::{Read, Write};

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

impl crate::private::Sealed for SMSG_LFG_QUEUE_STATUS {}
impl crate::Message for SMSG_LFG_QUEUE_STATUS {
    const OPCODE: u32 = 0x0365;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LFG_QUEUE_STATUS {{").unwrap();
        // Members
        writeln!(s, "    dungeon = {};", self.dungeon).unwrap();
        writeln!(s, "    average_wait_time = {};", self.average_wait_time).unwrap();
        writeln!(s, "    wait_time = {};", self.wait_time).unwrap();
        writeln!(s, "    wait_time_tank = {};", self.wait_time_tank).unwrap();
        writeln!(s, "    wait_time_healer = {};", self.wait_time_healer).unwrap();
        writeln!(s, "    wait_time_dps = {};", self.wait_time_dps).unwrap();
        writeln!(s, "    tanks_needed = {};", self.tanks_needed).unwrap();
        writeln!(s, "    healers_needed = {};", self.healers_needed).unwrap();
        writeln!(s, "    dps_needed = {};", self.dps_needed).unwrap();
        writeln!(s, "    queue_time = {};", self.queue_time).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 33_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 869_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "dungeon", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "average_wait_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "wait_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "wait_time_tank", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "wait_time_healer", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "wait_time_dps", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "tanks_needed", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "healers_needed", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "dps_needed", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "queue_time", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        31
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
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

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 31 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0365, size: body_size });
        }

        // dungeon: u32
        let dungeon = crate::util::read_u32_le(&mut r)?;

        // average_wait_time: i32
        let average_wait_time = crate::util::read_i32_le(&mut r)?;

        // wait_time: i32
        let wait_time = crate::util::read_i32_le(&mut r)?;

        // wait_time_tank: i32
        let wait_time_tank = crate::util::read_i32_le(&mut r)?;

        // wait_time_healer: i32
        let wait_time_healer = crate::util::read_i32_le(&mut r)?;

        // wait_time_dps: i32
        let wait_time_dps = crate::util::read_i32_le(&mut r)?;

        // tanks_needed: u8
        let tanks_needed = crate::util::read_u8_le(&mut r)?;

        // healers_needed: u8
        let healers_needed = crate::util::read_u8_le(&mut r)?;

        // dps_needed: u8
        let dps_needed = crate::util::read_u8_le(&mut r)?;

        // queue_time: u32
        let queue_time = crate::util::read_u32_le(&mut r)?;

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

