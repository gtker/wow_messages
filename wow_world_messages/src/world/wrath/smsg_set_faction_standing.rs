use std::io::{Read, Write};

use crate::wrath::FactionStanding;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm:26`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm#L26):
/// ```text
/// smsg SMSG_SET_FACTION_STANDING = 0x0124 {
///     f32 refer_a_friend_bonus;
///     Bool any_rank_increased;
///     u32 amount_of_faction_standings;
///     FactionStanding[amount_of_faction_standings] faction_standings;
/// }
/// ```
pub struct SMSG_SET_FACTION_STANDING {
    /// All emus set to 0.
    pub refer_a_friend_bonus: f32,
    /// mangostwo: display visual effect
    pub any_rank_increased: bool,
    pub faction_standings: Vec<FactionStanding>,
}

impl crate::private::Sealed for SMSG_SET_FACTION_STANDING {}
impl crate::Message for SMSG_SET_FACTION_STANDING {
    const OPCODE: u32 = 0x0124;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SET_FACTION_STANDING {{").unwrap();
        // Members
        writeln!(s, "    {}", if self.refer_a_friend_bonus.to_string().contains(".") { self.refer_a_friend_bonus.to_string() } else { format!("{}.0", self.refer_a_friend_bonus) }).unwrap();
        writeln!(s, "    any_rank_increased = {};", if self.any_rank_increased { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    amount_of_faction_standings = {};", self.faction_standings.len()).unwrap();
        write!(s, "    faction_standings = [").unwrap();
        for v in self.faction_standings.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        faction = {};", v.faction.as_test_case_value()).unwrap();
            writeln!(s, "        standing = {};", v.standing).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 292_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "refer_a_friend_bonus", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "any_rank_increased", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_faction_standings", "    ");
        if !self.faction_standings.is_empty() {
            writeln!(s, "    /* faction_standings: FactionStanding[amount_of_faction_standings] start */").unwrap();
            for (i, v) in self.faction_standings.iter().enumerate() {
                writeln!(s, "    /* faction_standings: FactionStanding[amount_of_faction_standings] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 2, "faction", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "standing", "        ");
                writeln!(s, "    /* faction_standings: FactionStanding[amount_of_faction_standings] {i} end */").unwrap();
            }
            writeln!(s, "    /* faction_standings: FactionStanding[amount_of_faction_standings] end */").unwrap();
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
        // refer_a_friend_bonus: f32
        w.write_all(&self.refer_a_friend_bonus.to_le_bytes())?;

        // any_rank_increased: Bool
        w.write_all(u8::from(self.any_rank_increased).to_le_bytes().as_slice())?;

        // amount_of_faction_standings: u32
        w.write_all(&(self.faction_standings.len() as u32).to_le_bytes())?;

        // faction_standings: FactionStanding[amount_of_faction_standings]
        for i in self.faction_standings.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(9..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0124, size: body_size });
        }

        // refer_a_friend_bonus: f32
        let refer_a_friend_bonus = crate::util::read_f32_le(&mut r)?;

        // any_rank_increased: Bool
        let any_rank_increased = crate::util::read_u8_le(&mut r)? != 0;

        // amount_of_faction_standings: u32
        let amount_of_faction_standings = crate::util::read_u32_le(&mut r)?;

        // faction_standings: FactionStanding[amount_of_faction_standings]
        let faction_standings = {
            let mut faction_standings = Vec::with_capacity(amount_of_faction_standings as usize);
            for _ in 0..amount_of_faction_standings {
                faction_standings.push(FactionStanding::read(&mut r)?);
            }
            faction_standings
        };

        Ok(Self {
            refer_a_friend_bonus,
            any_rank_increased,
            faction_standings,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SET_FACTION_STANDING {}

impl SMSG_SET_FACTION_STANDING {
    pub(crate) fn size(&self) -> usize {
        4 // refer_a_friend_bonus: f32
        + 1 // any_rank_increased: Bool
        + 4 // amount_of_faction_standings: u32
        + self.faction_standings.len() * 6 // faction_standings: FactionStanding[amount_of_faction_standings]
    }
}

