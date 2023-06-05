use std::io::{Read, Write};

use crate::vanilla::Area;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_zone_under_attack.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_zone_under_attack.wowm#L1):
/// ```text
/// smsg SMSG_ZONE_UNDER_ATTACK = 0x0254 {
///     Area zone_id;
/// }
/// ```
pub struct SMSG_ZONE_UNDER_ATTACK {
    pub zone_id: Area,
}

#[cfg(feature = "print-testcase")]
impl SMSG_ZONE_UNDER_ATTACK {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ZONE_UNDER_ATTACK {{").unwrap();
        // Members
        writeln!(s, "    zone_id = {};", self.zone_id.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 596_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "zone_id");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1.12\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_ZONE_UNDER_ATTACK {}
impl crate::Message for SMSG_ZONE_UNDER_ATTACK {
    const OPCODE: u32 = 0x0254;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // zone_id: Area
        w.write_all(&(self.zone_id.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0254, size: body_size });
        }

        // zone_id: Area
        let zone_id = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            zone_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ZONE_UNDER_ATTACK {}

