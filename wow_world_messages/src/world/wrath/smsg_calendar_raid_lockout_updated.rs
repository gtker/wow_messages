use std::io::{Read, Write};

use crate::DateTime;
use crate::wrath::Map;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_raid_lockout_updated.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_raid_lockout_updated.wowm#L1):
/// ```text
/// smsg SMSG_CALENDAR_RAID_LOCKOUT_UPDATED = 0x0471 {
///     DateTime current_time;
///     Map map;
///     u32 difficulty;
///     Seconds old_time_to_update;
///     Seconds new_time_to_update;
/// }
/// ```
pub struct SMSG_CALENDAR_RAID_LOCKOUT_UPDATED {
    pub current_time: DateTime,
    pub map: Map,
    pub difficulty: u32,
    pub old_time_to_update: Duration,
    pub new_time_to_update: Duration,
}

impl crate::private::Sealed for SMSG_CALENDAR_RAID_LOCKOUT_UPDATED {}
impl SMSG_CALENDAR_RAID_LOCKOUT_UPDATED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 20 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0471, size: body_size });
        }

        // current_time: DateTime
        let current_time = DateTime::try_from(crate::util::read_u32_le(&mut r)?)?;

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // difficulty: u32
        let difficulty = crate::util::read_u32_le(&mut r)?;

        // old_time_to_update: Seconds
        let old_time_to_update = Duration::from_secs(crate::util::read_u32_le(&mut r)?.into());

        // new_time_to_update: Seconds
        let new_time_to_update = Duration::from_secs(crate::util::read_u32_le(&mut r)?.into());

        Ok(Self {
            current_time,
            map,
            difficulty,
            old_time_to_update,
            new_time_to_update,
        })
    }

}

impl crate::Message for SMSG_CALENDAR_RAID_LOCKOUT_UPDATED {
    const OPCODE: u32 = 0x0471;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CALENDAR_RAID_LOCKOUT_UPDATED {{").unwrap();
        // Members
        writeln!(s, "    current_time = {};", self.current_time.as_int()).unwrap();
        writeln!(s, "    map = {};", self.map.as_test_case_value()).unwrap();
        writeln!(s, "    difficulty = {};", self.difficulty).unwrap();
        writeln!(s, "    old_time_to_update = {};", self.old_time_to_update.as_secs()).unwrap();
        writeln!(s, "    new_time_to_update = {};", self.new_time_to_update.as_secs()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 22_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1137_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "current_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "difficulty", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "old_time_to_update", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "new_time_to_update", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // current_time: DateTime
        w.write_all(&self.current_time.as_int().to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // difficulty: u32
        w.write_all(&self.difficulty.to_le_bytes())?;

        // old_time_to_update: Seconds
        w.write_all((self.old_time_to_update.as_secs() as u32).to_le_bytes().as_slice())?;

        // new_time_to_update: Seconds
        w.write_all((self.new_time_to_update.as_secs() as u32).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CALENDAR_RAID_LOCKOUT_UPDATED {}

