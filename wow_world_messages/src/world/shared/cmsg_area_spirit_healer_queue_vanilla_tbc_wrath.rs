use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/resurrect/cmsg_area_spirit_healer_queue.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/resurrect/cmsg_area_spirit_healer_queue.wowm#L3):
/// ```text
/// cmsg CMSG_AREA_SPIRIT_HEALER_QUEUE = 0x02E3 {
///     Guid guid;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_AREA_SPIRIT_HEALER_QUEUE {
    pub guid: Guid,
}

impl crate::private::Sealed for CMSG_AREA_SPIRIT_HEALER_QUEUE {}
impl CMSG_AREA_SPIRIT_HEALER_QUEUE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}

impl crate::Message for CMSG_AREA_SPIRIT_HEALER_QUEUE {
    const OPCODE: u32 = 0x02e3;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_AREA_SPIRIT_HEALER_QUEUE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_AREA_SPIRIT_HEALER_QUEUE {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 12_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 739_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(739, "CMSG_AREA_SPIRIT_HEALER_QUEUE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AREA_SPIRIT_HEALER_QUEUE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AREA_SPIRIT_HEALER_QUEUE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AREA_SPIRIT_HEALER_QUEUE {}

