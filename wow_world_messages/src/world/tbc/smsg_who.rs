use std::io::{Read, Write};

use crate::tbc::{
    Area, Class, Gender, Race, WhoPlayer,
};
use wow_world_base::shared::level_vanilla_tbc_wrath::Level;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_who.wowm:25`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_who.wowm#L25):
/// ```text
/// smsg SMSG_WHO = 0x0063 {
///     u32 listed_players;
///     u32 online_players;
///     WhoPlayer[listed_players] players;
/// }
/// ```
pub struct SMSG_WHO {
    pub online_players: u32,
    pub players: Vec<WhoPlayer>,
}

impl crate::private::Sealed for SMSG_WHO {}
impl SMSG_WHO {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(8..=65535).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0063, size: body_size });
        }

        // listed_players: u32
        let listed_players = crate::util::read_u32_le(&mut r)?;

        // online_players: u32
        let online_players = crate::util::read_u32_le(&mut r)?;

        // players: WhoPlayer[listed_players]
        let players = {
            let mut players = Vec::with_capacity(listed_players as usize);
            for _ in 0..listed_players {
                players.push(WhoPlayer::read(&mut r)?);
            }
            players
        };

        Ok(Self {
            online_players,
            players,
        })
    }

}

impl crate::Message for SMSG_WHO {
    const OPCODE: u32 = 0x0063;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_WHO {{").unwrap();
        // Members
        writeln!(s, "    listed_players = {};", self.players.len()).unwrap();
        writeln!(s, "    online_players = {};", self.online_players).unwrap();
        write!(s, "    players = [").unwrap();
        for v in self.players.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        name = \"{}\";", v.name).unwrap();
            writeln!(s, "        guild = \"{}\";", v.guild).unwrap();
            writeln!(s, "        level = {};", v.level.as_int()).unwrap();
            writeln!(s, "        class = {};", v.class.as_test_case_value()).unwrap();
            writeln!(s, "        race = {};", v.race.as_test_case_value()).unwrap();
            writeln!(s, "        gender = {};", v.gender.as_test_case_value()).unwrap();
            writeln!(s, "        area = {};", v.area.as_test_case_value()).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 99_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "listed_players", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "online_players", "    ");
        if !self.players.is_empty() {
            writeln!(s, "    /* players: WhoPlayer[listed_players] start */").unwrap();
            for (i, v) in self.players.iter().enumerate() {
                writeln!(s, "    /* players: WhoPlayer[listed_players] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, v.name.len() + 1, "name", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.guild.len() + 1, "guild", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "level", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "class", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "race", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "gender", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "area", "        ");
                writeln!(s, "    /* players: WhoPlayer[listed_players] {i} end */").unwrap();
            }
            writeln!(s, "    /* players: WhoPlayer[listed_players] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // listed_players: u32
        w.write_all(&(self.players.len() as u32).to_le_bytes())?;

        // online_players: u32
        w.write_all(&self.online_players.to_le_bytes())?;

        // players: WhoPlayer[listed_players]
        for i in self.players.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_WHO {}

impl SMSG_WHO {
    pub(crate) fn size(&self) -> usize {
        4 // listed_players: u32
        + 4 // online_players: u32
        + self.players.iter().fold(0, |acc, x| acc + x.size()) // players: WhoPlayer[listed_players]
    }
}

