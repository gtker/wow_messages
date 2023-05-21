use std::io::{Read, Write};

use crate::wrath::Map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_leave_battlefield.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_leave_battlefield.wowm#L7):
/// ```text
/// cmsg CMSG_LEAVE_BATTLEFIELD = 0x02E1 {
///     u8 unknown1;
///     u8 unknown2;
///     Map map;
///     u16 unknown3;
/// }
/// ```
pub struct CMSG_LEAVE_BATTLEFIELD {
    pub unknown1: u8,
    pub unknown2: u8,
    pub map: Map,
    pub unknown3: u16,
}

impl crate::private::Sealed for CMSG_LEAVE_BATTLEFIELD {}
impl CMSG_LEAVE_BATTLEFIELD {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(&mut r)?;

        // unknown2: u8
        let unknown2 = crate::util::read_u8_le(&mut r)?;

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // unknown3: u16
        let unknown3 = crate::util::read_u16_le(&mut r)?;

        Ok(Self {
            unknown1,
            unknown2,
            map,
            unknown3,
        })
    }

}

impl crate::Message for CMSG_LEAVE_BATTLEFIELD {
    const OPCODE: u32 = 0x02e1;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_LEAVE_BATTLEFIELD {{").unwrap();
        // Members
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    unknown2 = {};", self.unknown2).unwrap();
        writeln!(s, "    map = {};", self.map.as_test_case_value()).unwrap();
        writeln!(s, "    unknown3 = {};", self.unknown3).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 12_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 737_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown2", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 2, "unknown3", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u8
        w.write_all(&self.unknown2.to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // unknown3: u16
        w.write_all(&self.unknown3.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(737, "CMSG_LEAVE_BATTLEFIELD", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LEAVE_BATTLEFIELD {}

