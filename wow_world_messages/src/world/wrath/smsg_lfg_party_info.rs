use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    LfgJoinLockedDungeon, LfgPartyInfo,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_party_info.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_party_info.wowm#L9):
/// ```text
/// smsg SMSG_LFG_PARTY_INFO = 0x0372 {
///     u8 amount_of_infos;
///     LfgPartyInfo[amount_of_infos] infos;
/// }
/// ```
pub struct SMSG_LFG_PARTY_INFO {
    pub infos: Vec<LfgPartyInfo>,
}

impl crate::private::Sealed for SMSG_LFG_PARTY_INFO {}
impl crate::Message for SMSG_LFG_PARTY_INFO {
    const OPCODE: u32 = 0x0372;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LFG_PARTY_INFO {{").unwrap();
        // Members
        writeln!(s, "    amount_of_infos = {};", self.infos.len()).unwrap();
        write!(s, "    infos = [").unwrap();
        for v in self.infos.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        player = {};", v.player.guid()).unwrap();
            writeln!(s, "        amount_of_dungeons = {};", v.dungeons.len()).unwrap();
            write!(s, "        dungeons = [").unwrap();
            for v in v.dungeons.as_slice() {
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
        let [a, b] = 882_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_infos", "    ");
        if !self.infos.is_empty() {
            writeln!(s, "    /* infos: LfgPartyInfo[amount_of_infos] start */").unwrap();
            for (i, v) in self.infos.iter().enumerate() {
                writeln!(s, "    /* infos: LfgPartyInfo[amount_of_infos] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "player", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_dungeons", "        ");
                if !v.dungeons.is_empty() {
                    writeln!(s, "    /* dungeons: LfgJoinLockedDungeon[amount_of_dungeons] start */").unwrap();
                    for (i, v) in v.dungeons.iter().enumerate() {
                        writeln!(s, "    /* dungeons: LfgJoinLockedDungeon[amount_of_dungeons] {i} start */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "dungeon_entry", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "reason", "            ");
                        writeln!(s, "    /* dungeons: LfgJoinLockedDungeon[amount_of_dungeons] {i} end */").unwrap();
                    }
                    writeln!(s, "    /* dungeons: LfgJoinLockedDungeon[amount_of_dungeons] end */").unwrap();
                }
                writeln!(s, "    /* infos: LfgPartyInfo[amount_of_infos] {i} end */").unwrap();
            }
            writeln!(s, "    /* infos: LfgPartyInfo[amount_of_infos] end */").unwrap();
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
        // amount_of_infos: u8
        w.write_all(&(self.infos.len() as u8).to_le_bytes())?;

        // infos: LfgPartyInfo[amount_of_infos]
        for i in self.infos.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(1..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0372, size: body_size });
        }

        // amount_of_infos: u8
        let amount_of_infos = crate::util::read_u8_le(&mut r)?;

        // infos: LfgPartyInfo[amount_of_infos]
        let infos = {
            let mut infos = Vec::with_capacity(amount_of_infos as usize);
            for _ in 0..amount_of_infos {
                infos.push(LfgPartyInfo::read(&mut r)?);
            }
            infos
        };

        Ok(Self {
            infos,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_PARTY_INFO {}

impl SMSG_LFG_PARTY_INFO {
    pub(crate) fn size(&self) -> usize {
        1 // amount_of_infos: u8
        + self.infos.iter().fold(0, |acc, x| acc + x.size()) // infos: LfgPartyInfo[amount_of_infos]
    }
}

