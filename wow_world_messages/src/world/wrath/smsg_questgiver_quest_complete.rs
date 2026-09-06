use std::io::{Read, Write};

use crate::shared::gold_vanilla_tbc_wrath::Gold;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_quest_complete.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_quest_complete.wowm#L30):
/// ```text
/// smsg SMSG_QUESTGIVER_QUEST_COMPLETE = 0x0191 {
///     u32 quest_id;
///     u32 experience_reward;
///     Gold money_reward;
///     u32 honor_reward;
///     u32 talent_reward;
///     u32 arena_point_reward;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_QUESTGIVER_QUEST_COMPLETE {
    pub quest_id: u32,
    pub experience_reward: u32,
    pub money_reward: Gold,
    pub honor_reward: u32,
    pub talent_reward: u32,
    pub arena_point_reward: u32,
}

impl crate::private::Sealed for SMSG_QUESTGIVER_QUEST_COMPLETE {}
impl SMSG_QUESTGIVER_QUEST_COMPLETE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 24 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // experience_reward: u32
        let experience_reward = crate::util::read_u32_le(&mut r)?;

        // money_reward: Gold
        let money_reward = Gold::new(crate::util::read_u32_le(&mut r)?);

        // honor_reward: u32
        let honor_reward = crate::util::read_u32_le(&mut r)?;

        // talent_reward: u32
        let talent_reward = crate::util::read_u32_le(&mut r)?;

        // arena_point_reward: u32
        let arena_point_reward = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            quest_id,
            experience_reward,
            money_reward,
            honor_reward,
            talent_reward,
            arena_point_reward,
        })
    }

}

impl crate::Message for SMSG_QUESTGIVER_QUEST_COMPLETE {
    const OPCODE: u32 = 0x0191;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_QUESTGIVER_QUEST_COMPLETE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUESTGIVER_QUEST_COMPLETE {{").unwrap();
        // Members
        writeln!(s, "    quest_id = {};", self.quest_id).unwrap();
        writeln!(s, "    experience_reward = {};", self.experience_reward).unwrap();
        writeln!(s, "    money_reward = {};", self.money_reward.as_int()).unwrap();
        writeln!(s, "    honor_reward = {};", self.honor_reward).unwrap();
        writeln!(s, "    talent_reward = {};", self.talent_reward).unwrap();
        writeln!(s, "    arena_point_reward = {};", self.arena_point_reward).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 26_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 401_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "experience_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "money_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "honor_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "talent_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "arena_point_reward", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        24
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // experience_reward: u32
        w.write_all(&self.experience_reward.to_le_bytes())?;

        // money_reward: Gold
        w.write_all((self.money_reward.as_int()).to_le_bytes().as_slice())?;

        // honor_reward: u32
        w.write_all(&self.honor_reward.to_le_bytes())?;

        // talent_reward: u32
        w.write_all(&self.talent_reward.to_le_bytes())?;

        // arena_point_reward: u32
        w.write_all(&self.arena_point_reward.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(401, "SMSG_QUESTGIVER_QUEST_COMPLETE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUESTGIVER_QUEST_COMPLETE {}

