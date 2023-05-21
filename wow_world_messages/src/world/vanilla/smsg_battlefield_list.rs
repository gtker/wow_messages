use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    BattlegroundBracket, Map,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm:24`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm#L24):
/// ```text
/// smsg SMSG_BATTLEFIELD_LIST = 0x023D {
///     Guid battlemaster;
///     Map map;
///     BattlegroundBracket bracket;
///     u32 number_of_battlegrounds;
///     u32[number_of_battlegrounds] battlegrounds;
/// }
/// ```
pub struct SMSG_BATTLEFIELD_LIST {
    pub battlemaster: Guid,
    pub map: Map,
    pub bracket: BattlegroundBracket,
    pub battlegrounds: Vec<u32>,
}

impl crate::private::Sealed for SMSG_BATTLEFIELD_LIST {}
impl SMSG_BATTLEFIELD_LIST {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(17..=65535).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x023D, size: body_size });
        }

        // battlemaster: Guid
        let battlemaster = crate::util::read_guid(&mut r)?;

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // bracket: BattlegroundBracket
        let bracket = crate::util::read_u8_le(&mut r)?.try_into()?;

        // number_of_battlegrounds: u32
        let number_of_battlegrounds = crate::util::read_u32_le(&mut r)?;

        // battlegrounds: u32[number_of_battlegrounds]
        let battlegrounds = {
            let mut battlegrounds = Vec::with_capacity(number_of_battlegrounds as usize);
            for _ in 0..number_of_battlegrounds {
                battlegrounds.push(crate::util::read_u32_le(&mut r)?);
            }
            battlegrounds
        };

        Ok(Self {
            battlemaster,
            map,
            bracket,
            battlegrounds,
        })
    }

}

impl crate::Message for SMSG_BATTLEFIELD_LIST {
    const OPCODE: u32 = 0x023d;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_BATTLEFIELD_LIST {{").unwrap();
        // Members
        writeln!(s, "    battlemaster = {};", self.battlemaster.guid()).unwrap();
        writeln!(s, "    map = {};", self.map.as_test_case_value()).unwrap();
        writeln!(s, "    bracket = {};", self.bracket.as_test_case_value()).unwrap();
        writeln!(s, "    number_of_battlegrounds = {};", self.battlegrounds.len()).unwrap();
        write!(s, "    battlegrounds = [").unwrap();
        for v in self.battlegrounds.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 573_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "battlemaster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "bracket", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "number_of_battlegrounds", "    ");
        if !self.battlegrounds.is_empty() {
            writeln!(s, "    /* battlegrounds: u32[number_of_battlegrounds] start */").unwrap();
            for (i, v) in self.battlegrounds.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("battlegrounds {i}"), "    ");
            }
            writeln!(s, "    /* battlegrounds: u32[number_of_battlegrounds] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // battlemaster: Guid
        w.write_all(&self.battlemaster.guid().to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // bracket: BattlegroundBracket
        w.write_all(&(self.bracket.as_int().to_le_bytes()))?;

        // number_of_battlegrounds: u32
        w.write_all(&(self.battlegrounds.len() as u32).to_le_bytes())?;

        // battlegrounds: u32[number_of_battlegrounds]
        for i in self.battlegrounds.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_BATTLEFIELD_LIST {}

impl SMSG_BATTLEFIELD_LIST {
    pub(crate) fn size(&self) -> usize {
        8 // battlemaster: Guid
        + 4 // map: Map
        + 1 // bracket: BattlegroundBracket
        + 4 // number_of_battlegrounds: u32
        + self.battlegrounds.len() * core::mem::size_of::<u32>() // battlegrounds: u32[number_of_battlegrounds]
    }
}

