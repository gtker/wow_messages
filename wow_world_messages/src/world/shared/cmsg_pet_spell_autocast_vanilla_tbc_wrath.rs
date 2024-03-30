use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_pet_spell_autocast.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_spell_autocast.wowm#L3):
/// ```text
/// cmsg CMSG_PET_SPELL_AUTOCAST = 0x02F3 {
///     Guid guid;
///     Spell id;
///     Bool autocast_enabled;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_PET_SPELL_AUTOCAST {
    pub guid: Guid,
    pub id: u32,
    pub autocast_enabled: bool,
}

impl crate::private::Sealed for CMSG_PET_SPELL_AUTOCAST {}
impl CMSG_PET_SPELL_AUTOCAST {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 13 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // id: Spell
        let id = crate::util::read_u32_le(&mut r)?;

        // autocast_enabled: Bool
        let autocast_enabled = crate::util::read_bool_u8(&mut r)?;

        Ok(Self {
            guid,
            id,
            autocast_enabled,
        })
    }

}

impl crate::Message for CMSG_PET_SPELL_AUTOCAST {
    const OPCODE: u32 = 0x02f3;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_PET_SPELL_AUTOCAST"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_PET_SPELL_AUTOCAST {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    id = {};", self.id).unwrap();
        writeln!(s, "    autocast_enabled = {};", if self.autocast_enabled { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 17_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 755_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "autocast_enabled", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        13
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // id: Spell
        w.write_all(&self.id.to_le_bytes())?;

        // autocast_enabled: Bool
        w.write_all(u8::from(self.autocast_enabled).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(755, "CMSG_PET_SPELL_AUTOCAST", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PET_SPELL_AUTOCAST {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_PET_SPELL_AUTOCAST {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PET_SPELL_AUTOCAST {}

