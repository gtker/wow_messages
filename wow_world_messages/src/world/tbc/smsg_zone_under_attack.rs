use std::io::{Read, Write};

use crate::tbc::Area;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_zone_under_attack.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_zone_under_attack.wowm#L7):
/// ```text
/// smsg SMSG_ZONE_UNDER_ATTACK = 0x0254 {
///     Area zone_id;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_ZONE_UNDER_ATTACK {
    pub zone_id: Area,
}

impl crate::private::Sealed for SMSG_ZONE_UNDER_ATTACK {}
impl SMSG_ZONE_UNDER_ATTACK {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // zone_id: Area
        let zone_id = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            zone_id,
        })
    }

}

impl crate::Message for SMSG_ZONE_UNDER_ATTACK {
    const OPCODE: u32 = 0x0254;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ZONE_UNDER_ATTACK"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ZONE_UNDER_ATTACK {{").unwrap();
        // Members
        writeln!(s, "    zone_id = {};", self.zone_id.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 596_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "zone_id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // zone_id: Area
        w.write_all(&(self.zone_id.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(596, "SMSG_ZONE_UNDER_ATTACK", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ZONE_UNDER_ATTACK {}

