use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::BattlegroundType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm:48`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_list.wowm#L48):
/// ```text
/// smsg SMSG_BATTLEFIELD_LIST = 0x023D {
///     Guid battlemaster;
///     BattlegroundType battleground_type;
///     u32 number_of_battlegrounds;
///     u32[number_of_battlegrounds] battlegrounds;
/// }
/// ```
pub struct SMSG_BATTLEFIELD_LIST {
    pub battlemaster: Guid,
    pub battleground_type: BattlegroundType,
    pub battlegrounds: Vec<u32>,
}

impl crate::private::Sealed for SMSG_BATTLEFIELD_LIST {}
impl SMSG_BATTLEFIELD_LIST {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(16..=65535).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // battlemaster: Guid
        let battlemaster = crate::util::read_guid(&mut r)?;

        // battleground_type: BattlegroundType
        let battleground_type = crate::util::read_u32_le(&mut r)?.try_into()?;

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
            battleground_type,
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
        writeln!(s, "    battleground_type = {};", self.battleground_type.as_test_case_value()).unwrap();
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
        crate::util::write_bytes(&mut s, &mut bytes, 4, "battleground_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "number_of_battlegrounds", "    ");
        if !self.battlegrounds.is_empty() {
            writeln!(s, "    /* battlegrounds: u32[number_of_battlegrounds] start */").unwrap();
            for (i, v) in self.battlegrounds.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("battlegrounds {i}"), "    ");
            }
            writeln!(s, "    /* battlegrounds: u32[number_of_battlegrounds] end */").unwrap();
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
        // battlemaster: Guid
        w.write_all(&self.battlemaster.guid().to_le_bytes())?;

        // battleground_type: BattlegroundType
        w.write_all(&(self.battleground_type.as_int().to_le_bytes()))?;

        // number_of_battlegrounds: u32
        w.write_all(&(self.battlegrounds.len() as u32).to_le_bytes())?;

        // battlegrounds: u32[number_of_battlegrounds]
        for i in self.battlegrounds.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(573, "SMSG_BATTLEFIELD_LIST", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_BATTLEFIELD_LIST {}

impl SMSG_BATTLEFIELD_LIST {
    pub(crate) fn size(&self) -> usize {
        8 // battlemaster: Guid
        + 4 // battleground_type: BattlegroundType
        + 4 // number_of_battlegrounds: u32
        + self.battlegrounds.len() * core::mem::size_of::<u32>() // battlegrounds: u32[number_of_battlegrounds]
    }
}

