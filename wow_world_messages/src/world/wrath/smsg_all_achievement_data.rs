use std::io::{Read, Write};

use crate::wrath::{
    AchievementDone, AchievementInProgress,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/achievement/smsg_respond_inspect_achievements.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/achievement/smsg_respond_inspect_achievements.wowm#L9):
/// ```text
/// smsg SMSG_ALL_ACHIEVEMENT_DATA = 0x047D {
///     AchievementDoneArray done;
///     AchievementInProgressArray in_progress;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_ALL_ACHIEVEMENT_DATA {
    pub done: Vec<AchievementDone>,
    pub in_progress: Vec<AchievementInProgress>,
}

impl crate::private::Sealed for SMSG_ALL_ACHIEVEMENT_DATA {}
impl SMSG_ALL_ACHIEVEMENT_DATA {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size > 16777215 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // done: AchievementDoneArray
        let done = crate::util::read_achievement_done(&mut r)?;

        // in_progress: AchievementInProgressArray
        let in_progress = crate::util::read_achievement_in_progress(&mut r)?;

        Ok(Self {
            done,
            in_progress,
        })
    }

}

impl crate::Message for SMSG_ALL_ACHIEVEMENT_DATA {
    const OPCODE: u32 = 0x047d;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ALL_ACHIEVEMENT_DATA"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ALL_ACHIEVEMENT_DATA {{").unwrap();
        // Members
        panic!("unsupported type for test case printing: 'AchievementDoneArray' for variable 'done'");
        panic!("unsupported type for test case printing: 'AchievementInProgressArray' for variable 'in_progress'");

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1149_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        panic!("unsupported type Vec<AchievementDone> for variable 'done'");
        panic!("unsupported type Vec<AchievementInProgress> for variable 'in_progress'");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // done: AchievementDoneArray
        crate::util::write_achievement_done(self.done.as_slice(), &mut w)?;

        // in_progress: AchievementInProgressArray
        crate::util::write_achievement_in_progress(self.in_progress.as_slice(), &mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1149, "SMSG_ALL_ACHIEVEMENT_DATA", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ALL_ACHIEVEMENT_DATA {}

impl SMSG_ALL_ACHIEVEMENT_DATA {
    pub(crate) fn size(&self) -> usize {
        self.done.len() * 4 // done: AchievementDoneArray
        + self.in_progress.iter().fold(0, |acc, x| acc + x.size()) // in_progress: AchievementInProgressArray
    }
}

