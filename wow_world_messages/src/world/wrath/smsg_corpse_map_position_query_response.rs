use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Emus just set all values to 0.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_corpse_map_position_query_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_corpse_map_position_query_response.wowm#L1):
/// ```text
/// smsg SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE = 0x04B7 {
///     f32 unknown1;
///     f32 unknown2;
///     f32 unknown3;
///     f32 unknown4;
/// }
/// ```
pub struct SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE {
    pub unknown1: f32,
    pub unknown2: f32,
    pub unknown3: f32,
    pub unknown4: f32,
}

#[cfg(feature = "print-testcase")]
impl SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    {}", if self.unknown1.to_string().contains(".") { self.unknown1.to_string() } else { format!("{}.0", self.unknown1) }).unwrap();
        writeln!(s, "    {}", if self.unknown2.to_string().contains(".") { self.unknown2.to_string() } else { format!("{}.0", self.unknown2) }).unwrap();
        writeln!(s, "    {}", if self.unknown3.to_string().contains(".") { self.unknown3.to_string() } else { format!("{}.0", self.unknown3) }).unwrap();
        writeln!(s, "    {}", if self.unknown4.to_string().contains(".") { self.unknown4.to_string() } else { format!("{}.0", self.unknown4) }).unwrap();

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
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE {}
impl crate::Message for SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE {
    const OPCODE: u32 = 0x04b7;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE::to_test_case_string(self)
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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04B7, size: body_size });
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
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CORPSE_MAP_POSITION_QUERY_RESPONSE {}

