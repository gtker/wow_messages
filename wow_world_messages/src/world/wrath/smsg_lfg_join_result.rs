use std::io::{Read, Write};

use crate::wrath::LfgJoinPlayer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_join_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_join_result.wowm#L1):
/// ```text
/// smsg SMSG_LFG_JOIN_RESULT = 0x0364 {
///     u32 result;
///     u32 state;
///     LfgJoinPlayer[-] players;
/// }
/// ```
pub struct SMSG_LFG_JOIN_RESULT {
    pub result: u32,
    pub state: u32,
    pub players: Vec<LfgJoinPlayer>,
}

impl crate::private::Sealed for SMSG_LFG_JOIN_RESULT {}
impl crate::Message for SMSG_LFG_JOIN_RESULT {
    const OPCODE: u32 = 0x0364;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LFG_JOIN_RESULT {{").unwrap();
        // Members
        writeln!(s, "    result = {};", self.result).unwrap();
        writeln!(s, "    state = {};", self.state).unwrap();
        write!(s, "    players = [").unwrap();
        for v in self.players.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        player = {};", v.player.guid()).unwrap();
            writeln!(s, "        amount_of_locked_dungeons = {};", v.locked_dungeons.len()).unwrap();
            write!(s, "        locked_dungeons = [").unwrap();
            for v in v.locked_dungeons.as_slice() {
                writeln!(s, "{{").unwrap();
                // Members
                writeln!(s, "            dungeon_entry = {};", v.dungeon_entry).unwrap();
                writeln!(s, "            reason = {};", v.reason).unwrap();

                writeln!(s, "    }},").unwrap();
            }
            writeln!(s, "];").unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 868_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "result", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "state", "    ");
        if !self.players.is_empty() {
            writeln!(s, "    /* players: LfgJoinPlayer[-] start */").unwrap();
            for (i, v) in self.players.iter().enumerate() {
                writeln!(s, "    /* players: LfgJoinPlayer[-] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "player", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_locked_dungeons", "        ");
                if !v.locked_dungeons.is_empty() {
                    writeln!(s, "    /* locked_dungeons: LfgJoinLockedDungeon[amount_of_locked_dungeons] start */").unwrap();
                    for (i, v) in v.locked_dungeons.iter().enumerate() {
                        writeln!(s, "    /* locked_dungeons: LfgJoinLockedDungeon[amount_of_locked_dungeons] {i} start */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "dungeon_entry", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "reason", "            ");
                        writeln!(s, "    /* locked_dungeons: LfgJoinLockedDungeon[amount_of_locked_dungeons] {i} end */").unwrap();
                    }
                    writeln!(s, "    /* locked_dungeons: LfgJoinLockedDungeon[amount_of_locked_dungeons] end */").unwrap();
                }
                writeln!(s, "    /* players: LfgJoinPlayer[-] {i} end */").unwrap();
            }
            writeln!(s, "    /* players: LfgJoinPlayer[-] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // result: u32
        w.write_all(&self.result.to_le_bytes())?;

        // state: u32
        w.write_all(&self.state.to_le_bytes())?;

        // players: LfgJoinPlayer[-]
        for i in self.players.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(8..=65543).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0364, size: body_size });
        }

        // result: u32
        let result = crate::util::read_u32_le(&mut r)?;

        // state: u32
        let state = crate::util::read_u32_le(&mut r)?;

        // players: LfgJoinPlayer[-]
        let players = {
            let mut current_size = {
                4 // result: u32
                + 4 // state: u32
            };
            let mut players = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                players.push(LfgJoinPlayer::read(&mut r)?);
                current_size += 1;
            }
            players
        };

        Ok(Self {
            result,
            state,
            players,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_JOIN_RESULT {}

impl SMSG_LFG_JOIN_RESULT {
    pub(crate) fn size(&self) -> usize {
        4 // result: u32
        + 4 // state: u32
        + self.players.iter().fold(0, |acc, x| acc + x.size()) // players: LfgJoinPlayer[-]
    }
}

