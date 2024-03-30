use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_activatetaxi.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_activatetaxi.wowm#L3):
/// ```text
/// cmsg CMSG_ACTIVATETAXI = 0x01AD {
///     Guid guid;
///     u32 source_node;
///     u32 destination_node;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_ACTIVATETAXI {
    pub guid: Guid,
    pub source_node: u32,
    pub destination_node: u32,
}

impl crate::private::Sealed for CMSG_ACTIVATETAXI {}
impl CMSG_ACTIVATETAXI {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 16 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // source_node: u32
        let source_node = crate::util::read_u32_le(&mut r)?;

        // destination_node: u32
        let destination_node = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            source_node,
            destination_node,
        })
    }

}

impl crate::Message for CMSG_ACTIVATETAXI {
    const OPCODE: u32 = 0x01ad;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_ACTIVATETAXI"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_ACTIVATETAXI {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    source_node = {};", self.source_node).unwrap();
        writeln!(s, "    destination_node = {};", self.destination_node).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 20_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 429_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "source_node", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "destination_node", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // source_node: u32
        w.write_all(&self.source_node.to_le_bytes())?;

        // destination_node: u32
        w.write_all(&self.destination_node.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(429, "CMSG_ACTIVATETAXI", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_ACTIVATETAXI {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_ACTIVATETAXI {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_ACTIVATETAXI {}

