use std::io::{Read, Write};

use crate::wrath::Map;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_update_last_instance.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_update_last_instance.wowm#L3):
/// ```text
/// smsg SMSG_UPDATE_LAST_INSTANCE = 0x0320 {
///     Map map;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_UPDATE_LAST_INSTANCE {
    pub map: Map,
}

impl crate::private::Sealed for SMSG_UPDATE_LAST_INSTANCE {}
impl SMSG_UPDATE_LAST_INSTANCE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            map,
        })
    }

}

impl crate::Message for SMSG_UPDATE_LAST_INSTANCE {
    const OPCODE: u32 = 0x0320;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_UPDATE_LAST_INSTANCE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_UPDATE_LAST_INSTANCE {{").unwrap();
        // Members
        writeln!(s, "    map = {};", self.map.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 800_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(800, "SMSG_UPDATE_LAST_INSTANCE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_UPDATE_LAST_INSTANCE {}

