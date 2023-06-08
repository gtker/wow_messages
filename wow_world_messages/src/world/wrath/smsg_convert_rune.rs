use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_convert_rune.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_convert_rune.wowm#L1):
/// ```text
/// smsg SMSG_CONVERT_RUNE = 0x0486 {
///     u8 index;
///     u8 new_type;
/// }
/// ```
pub struct SMSG_CONVERT_RUNE {
    pub index: u8,
    pub new_type: u8,
}

impl crate::private::Sealed for SMSG_CONVERT_RUNE {}
impl crate::Message for SMSG_CONVERT_RUNE {
    const OPCODE: u32 = 0x0486;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CONVERT_RUNE {{").unwrap();
        // Members
        writeln!(s, "    index = {};", self.index).unwrap();
        writeln!(s, "    new_type = {};", self.new_type).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 4_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1158_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "index", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "new_type", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // index: u8
        w.write_all(&self.index.to_le_bytes())?;

        // new_type: u8
        w.write_all(&self.new_type.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0486, size: body_size });
        }

        // index: u8
        let index = crate::util::read_u8_le(&mut r)?;

        // new_type: u8
        let new_type = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            index,
            new_type,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CONVERT_RUNE {}

