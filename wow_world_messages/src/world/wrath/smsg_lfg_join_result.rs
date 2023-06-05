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

#[cfg(feature = "print-testcase")]
impl SMSG_LFG_JOIN_RESULT {
    pub fn to_test_case_string(&self) -> String {
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
            writeln!(s, "    player = {};", v.player.guid()).unwrap();
            writeln!(s, "    amount_of_locked_dungeons = {};", v.locked_dungeons.len()).unwrap();
            write!(s, "    locked_dungeons = [").unwrap();
            for v in v.locked_dungeons.as_slice() {
                writeln!(s, "{{").unwrap();
                // Members
                writeln!(s, "    dungeon_entry = {};", v.dungeon_entry).unwrap();
                writeln!(s, "    reason = {};", v.reason).unwrap();

                writeln!(s, "    }},").unwrap();
            }
            writeln!(s, "];").unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 868_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "result");
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

impl crate::private::Sealed for SMSG_LFG_JOIN_RESULT {}
impl crate::Message for SMSG_LFG_JOIN_RESULT {
    const OPCODE: u32 = 0x0364;

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

