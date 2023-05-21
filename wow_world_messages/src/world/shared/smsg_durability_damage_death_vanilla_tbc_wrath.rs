use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Signals to the client that the death caused 10% durability loss.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_durability_damage_death.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_durability_damage_death.wowm#L3):
/// ```text
/// smsg SMSG_DURABILITY_DAMAGE_DEATH = 0x02BD {
/// }
/// ```
pub struct SMSG_DURABILITY_DAMAGE_DEATH {
}

impl crate::private::Sealed for SMSG_DURABILITY_DAMAGE_DEATH {}
impl SMSG_DURABILITY_DAMAGE_DEATH {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 0 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x02BD, size: body_size });
        }

        Ok(Self {
        })
    }

}

impl crate::Message for SMSG_DURABILITY_DAMAGE_DEATH {
    const OPCODE: u32 = 0x02bd;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_DURABILITY_DAMAGE_DEATH {{").unwrap();
        // Members

        writeln!(s, "}} [").unwrap();

        let [a, b] = 2_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 701_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_DURABILITY_DAMAGE_DEATH {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_DURABILITY_DAMAGE_DEATH {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_DURABILITY_DAMAGE_DEATH {}

