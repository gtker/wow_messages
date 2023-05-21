use std::io::{Read, Write};

use crate::tbc::{
    Faction, FactionFlag,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/cmsg_set_faction_atwar.wowm:28`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/cmsg_set_faction_atwar.wowm#L28):
/// ```text
/// cmsg CMSG_SET_FACTION_ATWAR = 0x0125 {
///     Faction faction;
///     FactionFlag flags;
/// }
/// ```
pub struct CMSG_SET_FACTION_ATWAR {
    pub faction: Faction,
    pub flags: FactionFlag,
}

impl crate::private::Sealed for CMSG_SET_FACTION_ATWAR {}
impl CMSG_SET_FACTION_ATWAR {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 3 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // faction: Faction
        let faction = crate::util::read_u16_le(&mut r)?.try_into()?;

        // flags: FactionFlag
        let flags = FactionFlag::new(crate::util::read_u8_le(&mut r)?);

        Ok(Self {
            faction,
            flags,
        })
    }

}

impl crate::Message for CMSG_SET_FACTION_ATWAR {
    const OPCODE: u32 = 0x0125;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SET_FACTION_ATWAR {{").unwrap();
        // Members
        writeln!(s, "    faction = {};", self.faction.as_test_case_value()).unwrap();
        writeln!(s, "    flags = {};", self.flags.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 7_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 293_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 2, "faction", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "flags", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        3
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // faction: Faction
        w.write_all(&(self.faction.as_int().to_le_bytes()))?;

        // flags: FactionFlag
        w.write_all(&(self.flags.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(293, "CMSG_SET_FACTION_ATWAR", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_FACTION_ATWAR {}

