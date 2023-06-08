use std::io::{Read, Write};

use crate::tbc::Faction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/cmsg_set_faction_inactive.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/cmsg_set_faction_inactive.wowm#L1):
/// ```text
/// cmsg CMSG_SET_FACTION_INACTIVE = 0x0317 {
///     Faction faction;
///     Bool inactive;
/// }
/// ```
pub struct CMSG_SET_FACTION_INACTIVE {
    pub faction: Faction,
    pub inactive: bool,
}

impl crate::private::Sealed for CMSG_SET_FACTION_INACTIVE {}
impl crate::Message for CMSG_SET_FACTION_INACTIVE {
    const OPCODE: u32 = 0x0317;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SET_FACTION_INACTIVE {{").unwrap();
        // Members
        writeln!(s, "    faction = {};", self.faction.as_test_case_value()).unwrap();
        writeln!(s, "    inactive = {};", if self.inactive { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 7_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 791_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 2, "faction", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "inactive", "    ");


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

        // inactive: Bool
        w.write_all(u8::from(self.inactive).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 3 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0317, size: body_size });
        }

        // faction: Faction
        let faction = crate::util::read_u16_le(&mut r)?.try_into()?;

        // inactive: Bool
        let inactive = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            faction,
            inactive,
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_FACTION_INACTIVE {}

