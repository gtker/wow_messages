use std::io::{Read, Write};

use crate::{
    DateTime, Guid,
};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/achievement/smsg_criteria_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/achievement/smsg_criteria_update.wowm#L1):
/// ```text
/// smsg SMSG_CRITERIA_UPDATE = 0x046A {
///     u32 achievement;
///     PackedGuid progress_counter;
///     PackedGuid player;
///     u32 flags;
///     DateTime time;
///     Seconds time_elapsed;
///     u32 unknown;
/// }
/// ```
pub struct SMSG_CRITERIA_UPDATE {
    pub achievement: u32,
    /// trinitycore/azerothcore: This is a u32 passed to the `appendPackGUID` function which promotes it to u64.
    pub progress_counter: Guid,
    pub player: Guid,
    /// trinitycore: this are some flags, 1 is for keeping the counter at 0 in client
    pub flags: u32,
    pub time: DateTime,
    pub time_elapsed: Duration,
    pub unknown: u32,
}

#[cfg(feature = "print-testcase")]
impl SMSG_CRITERIA_UPDATE {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CRITERIA_UPDATE {{").unwrap();
        // Members
        writeln!(s, "    achievement = {};", self.achievement).unwrap();
        writeln!(s, "    progress_counter = {};", self.progress_counter.guid()).unwrap();
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        writeln!(s, "    flags = {};", self.flags).unwrap();
        writeln!(s, "    time = {};", self.time.as_int()).unwrap();
        writeln!(s, "    time_elapsed = {};", self.time_elapsed.as_secs()).unwrap();
        writeln!(s, "    unknown = {};", self.unknown).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1130_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "achievement", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.progress_counter), "progress_counter", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.player), "player", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_elapsed", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_CRITERIA_UPDATE {}
impl crate::Message for SMSG_CRITERIA_UPDATE {
    const OPCODE: u32 = 0x046a;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_CRITERIA_UPDATE::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // achievement: u32
        w.write_all(&self.achievement.to_le_bytes())?;

        // progress_counter: PackedGuid
        crate::util::write_packed_guid(&self.progress_counter, &mut w)?;

        // player: PackedGuid
        crate::util::write_packed_guid(&self.player, &mut w)?;

        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        // time: DateTime
        w.write_all(&self.time.as_int().to_le_bytes())?;

        // time_elapsed: Seconds
        w.write_all((self.time_elapsed.as_secs() as u32).to_le_bytes().as_slice())?;

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(24..=38).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x046A, size: body_size });
        }

        // achievement: u32
        let achievement = crate::util::read_u32_le(&mut r)?;

        // progress_counter: PackedGuid
        let progress_counter = crate::util::read_packed_guid(&mut r)?;

        // player: PackedGuid
        let player = crate::util::read_packed_guid(&mut r)?;

        // flags: u32
        let flags = crate::util::read_u32_le(&mut r)?;

        // time: DateTime
        let time = DateTime::try_from(crate::util::read_u32_le(&mut r)?)?;

        // time_elapsed: Seconds
        let time_elapsed = Duration::from_secs(crate::util::read_u32_le(&mut r)?.into());

        // unknown: u32
        let unknown = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            achievement,
            progress_counter,
            player,
            flags,
            time,
            time_elapsed,
            unknown,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CRITERIA_UPDATE {}

impl SMSG_CRITERIA_UPDATE {
    pub(crate) const fn size(&self) -> usize {
        4 // achievement: u32
        + crate::util::packed_guid_size(&self.progress_counter) // progress_counter: PackedGuid
        + crate::util::packed_guid_size(&self.player) // player: PackedGuid
        + 4 // flags: u32
        + 4 // time: DateTime
        + 4 // time_elapsed: Seconds
        + 4 // unknown: u32
    }
}

