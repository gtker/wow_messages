use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_learned_spell.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_learned_spell.wowm#L7):
/// ```text
/// smsg SMSG_LEARNED_SPELL = 0x012B {
///     u32 id;
///     u16 unknown;
/// }
/// ```
pub struct SMSG_LEARNED_SPELL {
    pub id: u32,
    /// mangostwo: 3.3.3 unk
    ///
    pub unknown: u16,
}

#[cfg(feature = "print-testcase")]
impl SMSG_LEARNED_SPELL {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LEARNED_SPELL {{").unwrap();
        // Members
        writeln!(s, "    id = {};", self.id).unwrap();
        writeln!(s, "    unknown = {};", self.unknown).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 299_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 2, "unknown", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_LEARNED_SPELL {}
impl crate::Message for SMSG_LEARNED_SPELL {
    const OPCODE: u32 = 0x012b;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_LEARNED_SPELL::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        6
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // unknown: u16
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 6 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x012B, size: body_size });
        }

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // unknown: u16
        let unknown = crate::util::read_u16_le(&mut r)?;

        Ok(Self {
            id,
            unknown,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LEARNED_SPELL {}

