use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_alter_appearance.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_alter_appearance.wowm#L1):
/// ```text
/// cmsg CMSG_ALTER_APPEARANCE = 0x0426 {
///     u32 hair;
///     u32 hair_color;
///     u32 facial_hair;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_ALTER_APPEARANCE {
    pub hair: u32,
    pub hair_color: u32,
    pub facial_hair: u32,
}

impl crate::private::Sealed for CMSG_ALTER_APPEARANCE {}
impl CMSG_ALTER_APPEARANCE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // hair: u32
        let hair = crate::util::read_u32_le(&mut r)?;

        // hair_color: u32
        let hair_color = crate::util::read_u32_le(&mut r)?;

        // facial_hair: u32
        let facial_hair = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            hair,
            hair_color,
            facial_hair,
        })
    }

}

impl crate::Message for CMSG_ALTER_APPEARANCE {
    const OPCODE: u32 = 0x0426;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_ALTER_APPEARANCE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_ALTER_APPEARANCE {{").unwrap();
        // Members
        writeln!(s, "    hair = {};", self.hair).unwrap();
        writeln!(s, "    hair_color = {};", self.hair_color).unwrap();
        writeln!(s, "    facial_hair = {};", self.facial_hair).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 16_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1062_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "hair", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "hair_color", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "facial_hair", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // hair: u32
        w.write_all(&self.hair.to_le_bytes())?;

        // hair_color: u32
        w.write_all(&self.hair_color.to_le_bytes())?;

        // facial_hair: u32
        w.write_all(&self.facial_hair.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1062, "CMSG_ALTER_APPEARANCE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_ALTER_APPEARANCE {}

