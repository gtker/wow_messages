use std::io::{Read, Write};

use crate::{
    DateTime, Guid,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/achievement/smsg_achievement_earned.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/achievement/smsg_achievement_earned.wowm#L1):
/// ```text
/// smsg SMSG_ACHIEVEMENT_EARNED = 0x0468 {
///     PackedGuid player;
///     u32 achievement;
///     DateTime earn_time;
///     u32 unknown;
/// }
/// ```
pub struct SMSG_ACHIEVEMENT_EARNED {
    pub player: Guid,
    pub achievement: u32,
    pub earn_time: DateTime,
    /// All emus set to 0.
    pub unknown: u32,
}

impl crate::private::Sealed for SMSG_ACHIEVEMENT_EARNED {}
impl SMSG_ACHIEVEMENT_EARNED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(13..=21).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // player: PackedGuid
        let player = crate::util::read_packed_guid(&mut r)?;

        // achievement: u32
        let achievement = crate::util::read_u32_le(&mut r)?;

        // earn_time: DateTime
        let earn_time = DateTime::try_from(crate::util::read_u32_le(&mut r)?)?;

        // unknown: u32
        let unknown = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            player,
            achievement,
            earn_time,
            unknown,
        })
    }

}

impl crate::Message for SMSG_ACHIEVEMENT_EARNED {
    const OPCODE: u32 = 0x0468;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ACHIEVEMENT_EARNED"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ACHIEVEMENT_EARNED {{").unwrap();
        // Members
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        writeln!(s, "    achievement = {};", self.achievement).unwrap();
        writeln!(s, "    earn_time = {};", self.earn_time.as_int()).unwrap();
        writeln!(s, "    unknown = {};", self.unknown).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1128_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.player), "player", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "achievement", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "earn_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        crate::util::write_packed_guid(&self.player, &mut w)?;

        // achievement: u32
        w.write_all(&self.achievement.to_le_bytes())?;

        // earn_time: DateTime
        w.write_all(&self.earn_time.as_int().to_le_bytes())?;

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1128, "SMSG_ACHIEVEMENT_EARNED", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ACHIEVEMENT_EARNED {}

impl SMSG_ACHIEVEMENT_EARNED {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.player) // player: PackedGuid
        + 4 // achievement: u32
        + 4 // earn_time: DateTime
        + 4 // unknown: u32
    }
}

