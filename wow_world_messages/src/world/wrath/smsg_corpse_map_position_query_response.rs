use std::io::{Read, Write};

/// Emus just set all values to 0.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_corpse_map_position_query_response.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_corpse_map_position_query_response.wowm#L2):
/// ```text
/// smsg SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE = 0x04B7 {
///     f32 unknown1;
///     f32 unknown2;
///     f32 unknown3;
///     f32 unknown4;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE {
    pub unknown1: f32,
    pub unknown2: f32,
    pub unknown3: f32,
    pub unknown4: f32,
}

impl crate::private::Sealed for SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE {}
impl SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 16 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // unknown1: f32
        let unknown1 = crate::util::read_f32_le(&mut r)?;

        // unknown2: f32
        let unknown2 = crate::util::read_f32_le(&mut r)?;

        // unknown3: f32
        let unknown3 = crate::util::read_f32_le(&mut r)?;

        // unknown4: f32
        let unknown4 = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            unknown1,
            unknown2,
            unknown3,
            unknown4,
        })
    }

}

impl crate::Message for SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE {
    const OPCODE: u32 = 0x04b7;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    unknown1 = {};", if self.unknown1.to_string().contains('.') { self.unknown1.to_string() } else { format!("{}.0", self.unknown1) }).unwrap();
        writeln!(s, "    unknown2 = {};", if self.unknown2.to_string().contains('.') { self.unknown2.to_string() } else { format!("{}.0", self.unknown2) }).unwrap();
        writeln!(s, "    unknown3 = {};", if self.unknown3.to_string().contains('.') { self.unknown3.to_string() } else { format!("{}.0", self.unknown3) }).unwrap();
        writeln!(s, "    unknown4 = {};", if self.unknown4.to_string().contains('.') { self.unknown4.to_string() } else { format!("{}.0", self.unknown4) }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 18_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1207_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown2", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown3", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown4", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unknown1: f32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: f32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: f32
        w.write_all(&self.unknown3.to_le_bytes())?;

        // unknown4: f32
        w.write_all(&self.unknown4.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1207, "SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE {}

