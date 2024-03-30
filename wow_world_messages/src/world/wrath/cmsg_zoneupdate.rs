use std::io::{Read, Write};

use crate::wrath::Area;

/// Sent by the client whenever it reaches a new area.
/// The client does not send an accurate area. For example when going to Sen'jin Village, the client will send `DUROTAR` (0x0E) and not `SENJIN_VILLAGE` (0x16F).
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/client_set/cmsg_zoneupdate.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/client_set/cmsg_zoneupdate.wowm#L3):
/// ```text
/// cmsg CMSG_ZONEUPDATE = 0x01F4 {
///     Area area;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_ZONEUPDATE {
    pub area: Area,
}

impl crate::private::Sealed for CMSG_ZONEUPDATE {}
impl CMSG_ZONEUPDATE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // area: Area
        let area = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            area,
        })
    }

}

impl crate::Message for CMSG_ZONEUPDATE {
    const OPCODE: u32 = 0x01f4;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_ZONEUPDATE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_ZONEUPDATE {{").unwrap();
        // Members
        writeln!(s, "    area = {};", self.area.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 500_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "area", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // area: Area
        w.write_all(&(self.area.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(500, "CMSG_ZONEUPDATE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_ZONEUPDATE {}

