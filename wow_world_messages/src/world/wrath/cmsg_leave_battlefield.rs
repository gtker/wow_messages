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

#[cfg(feature = "print-testcase")]
impl CMSG_LEAVE_BATTLEFIELD {
    pub fn to_test_case_string(&self) -> String {
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

        // Size/Opcode
        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 737_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown1");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_LEAVE_BATTLEFIELD {}
impl crate::Message for CMSG_LEAVE_BATTLEFIELD {
    const OPCODE: u32 = 0x02e1;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02E1, size: body_size });
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
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LEAVE_BATTLEFIELD {}

