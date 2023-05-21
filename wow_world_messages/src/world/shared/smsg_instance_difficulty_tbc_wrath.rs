use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_instance_difficulty.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_instance_difficulty.wowm#L1):
/// ```text
/// smsg SMSG_INSTANCE_DIFFICULTY = 0x033B {
///     u32 difficulty;
///     Bool32 dynamic_difficulty;
/// }
/// ```
pub struct SMSG_INSTANCE_DIFFICULTY {
    pub difficulty: u32,
    pub dynamic_difficulty: bool,
}

impl crate::private::Sealed for SMSG_INSTANCE_DIFFICULTY {}
impl SMSG_INSTANCE_DIFFICULTY {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // difficulty: u32
        let difficulty = crate::util::read_u32_le(&mut r)?;

        // dynamic_difficulty: Bool32
        let dynamic_difficulty = crate::util::read_u32_le(&mut r)? != 0;

        Ok(Self {
            difficulty,
            dynamic_difficulty,
        })
    }

}

impl crate::Message for SMSG_INSTANCE_DIFFICULTY {
    const OPCODE: u32 = 0x033b;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_INSTANCE_DIFFICULTY {{").unwrap();
        // Members
        writeln!(s, "    difficulty = {};", self.difficulty).unwrap();
        writeln!(s, "    dynamic_difficulty = {};", if self.dynamic_difficulty { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 10_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 827_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "difficulty", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "dynamic_difficulty", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // difficulty: u32
        w.write_all(&self.difficulty.to_le_bytes())?;

        // dynamic_difficulty: Bool32
        w.write_all(u32::from(self.dynamic_difficulty).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(827, "SMSG_INSTANCE_DIFFICULTY", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_INSTANCE_DIFFICULTY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_INSTANCE_DIFFICULTY {}

