use std::io::{Read, Write};

use crate::wrath::{
    LfgAvailableDungeon, LfgJoinLockedDungeon,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_player_info.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_player_info.wowm#L22):
/// ```text
/// smsg SMSG_LFG_PLAYER_INFO = 0x036F {
///     u8 amount_of_available_dungeons;
///     LfgAvailableDungeon[amount_of_available_dungeons] available_dungeons;
///     u8 amount_of_locked_dungeons;
///     LfgJoinLockedDungeon[amount_of_locked_dungeons] locked_dungeons;
/// }
/// ```
pub struct SMSG_LFG_PLAYER_INFO {
    pub available_dungeons: Vec<LfgAvailableDungeon>,
    pub locked_dungeons: Vec<LfgJoinLockedDungeon>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_LFG_PLAYER_INFO {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LFG_PLAYER_INFO {{").unwrap();
        // Members
        writeln!(s, "    amount_of_available_dungeons = {};", self.available_dungeons.len()).unwrap();
        write!(s, "    available_dungeons = [").unwrap();
        for v in self.available_dungeons.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "    dungeon_entry = {};", v.dungeon_entry).unwrap();
            writeln!(s, "    done = {};", if v.done { "TRUE" } else { "FALSE" }).unwrap();
            writeln!(s, "    quest_reward = {};", v.quest_reward).unwrap();
            writeln!(s, "    xp_reward = {};", v.xp_reward).unwrap();
            writeln!(s, "    unknown1 = {};", v.unknown1).unwrap();
            writeln!(s, "    unknown2 = {};", v.unknown2).unwrap();
            writeln!(s, "    amount_of_rewards = {};", v.rewards.len()).unwrap();
            write!(s, "    rewards = [").unwrap();
            for v in v.rewards.as_slice() {
                writeln!(s, "{{").unwrap();
                // Members
                writeln!(s, "    item = {};", v.item).unwrap();
                writeln!(s, "    display_id = {};", v.display_id).unwrap();
                writeln!(s, "    amount_of_rewards = {};", v.amount_of_rewards).unwrap();

                writeln!(s, "    }},").unwrap();
            }
            writeln!(s, "];").unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    amount_of_locked_dungeons = {};", self.locked_dungeons.len()).unwrap();
        write!(s, "    locked_dungeons = [").unwrap();
        for v in self.locked_dungeons.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "    dungeon_entry = {};", v.dungeon_entry).unwrap();
            writeln!(s, "    reason = {};", v.reason).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 879_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_available_dungeons");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_LFG_PLAYER_INFO {}
impl crate::Message for SMSG_LFG_PLAYER_INFO {
    const OPCODE: u32 = 0x036f;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_available_dungeons: u8
        w.write_all(&(self.available_dungeons.len() as u8).to_le_bytes())?;

        // available_dungeons: LfgAvailableDungeon[amount_of_available_dungeons]
        for i in self.available_dungeons.iter() {
            i.write_into_vec(&mut w)?;
        }

        // amount_of_locked_dungeons: u8
        w.write_all(&(self.locked_dungeons.len() as u8).to_le_bytes())?;

        // locked_dungeons: LfgJoinLockedDungeon[amount_of_locked_dungeons]
        for i in self.locked_dungeons.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(2..=794114).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x036F, size: body_size });
        }

        // amount_of_available_dungeons: u8
        let amount_of_available_dungeons = crate::util::read_u8_le(&mut r)?;

        // available_dungeons: LfgAvailableDungeon[amount_of_available_dungeons]
        let available_dungeons = {
            let mut available_dungeons = Vec::with_capacity(amount_of_available_dungeons as usize);
            for _ in 0..amount_of_available_dungeons {
                available_dungeons.push(LfgAvailableDungeon::read(&mut r)?);
            }
            available_dungeons
        };

        // amount_of_locked_dungeons: u8
        let amount_of_locked_dungeons = crate::util::read_u8_le(&mut r)?;

        // locked_dungeons: LfgJoinLockedDungeon[amount_of_locked_dungeons]
        let locked_dungeons = {
            let mut locked_dungeons = Vec::with_capacity(amount_of_locked_dungeons as usize);
            for _ in 0..amount_of_locked_dungeons {
                locked_dungeons.push(LfgJoinLockedDungeon::read(&mut r)?);
            }
            locked_dungeons
        };

        Ok(Self {
            available_dungeons,
            locked_dungeons,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_PLAYER_INFO {}

impl SMSG_LFG_PLAYER_INFO {
    pub(crate) fn size(&self) -> usize {
        1 // amount_of_available_dungeons: u8
        + self.available_dungeons.iter().fold(0, |acc, x| acc + x.size()) // available_dungeons: LfgAvailableDungeon[amount_of_available_dungeons]
        + 1 // amount_of_locked_dungeons: u8
        + self.locked_dungeons.len() * 8 // locked_dungeons: LfgJoinLockedDungeon[amount_of_locked_dungeons]
    }
}

