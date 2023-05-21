use std::io::{Read, Write};

use crate::{
    DateTime, Guid,
};
use crate::wrath::Map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_raid_lockout_added.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_raid_lockout_added.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_RAID_LOCKOUT_ADDED = 0x043E {
///     DateTime time;
///     Map map;
///     u32 difficulty;
///     u32 remaining_time;
///     Guid instance_id;
/// }
/// ```
pub struct SMSG_CALENDAR_RAID_LOCKOUT_ADDED {
    pub time: DateTime,
    pub map: Map,
    pub difficulty: u32,
    pub remaining_time: u32,
    pub instance_id: Guid,
}

impl crate::private::Sealed for SMSG_CALENDAR_RAID_LOCKOUT_ADDED {}
impl SMSG_CALENDAR_RAID_LOCKOUT_ADDED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 24 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x043E, size: body_size });
        }

        // time: DateTime
        let time = DateTime::try_from(crate::util::read_u32_le(&mut r)?)?;

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // difficulty: u32
        let difficulty = crate::util::read_u32_le(&mut r)?;

        // remaining_time: u32
        let remaining_time = crate::util::read_u32_le(&mut r)?;

        // instance_id: Guid
        let instance_id = crate::util::read_guid(&mut r)?;

        Ok(Self {
            time,
            map,
            difficulty,
            remaining_time,
            instance_id,
        })
    }

}

impl crate::Message for SMSG_CALENDAR_RAID_LOCKOUT_ADDED {
    const OPCODE: u32 = 0x043e;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CALENDAR_RAID_LOCKOUT_ADDED {{").unwrap();
        // Members
        writeln!(s, "    time = {};", self.time.as_int()).unwrap();
        writeln!(s, "    map = {};", self.map.as_test_case_value()).unwrap();
        writeln!(s, "    difficulty = {};", self.difficulty).unwrap();
        writeln!(s, "    remaining_time = {};", self.remaining_time).unwrap();
        writeln!(s, "    instance_id = {};", self.instance_id.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 26_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1086_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "difficulty", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "remaining_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "instance_id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        24
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // time: DateTime
        w.write_all(&self.time.as_int().to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // difficulty: u32
        w.write_all(&self.difficulty.to_le_bytes())?;

        // remaining_time: u32
        w.write_all(&self.remaining_time.to_le_bytes())?;

        // instance_id: Guid
        w.write_all(&self.instance_id.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_RAID_LOCKOUT_ADDED {}

