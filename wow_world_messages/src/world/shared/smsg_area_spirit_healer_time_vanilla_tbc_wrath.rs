use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/resurrect/smsg_area_spirit_healer_time.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/resurrect/smsg_area_spirit_healer_time.wowm#L3):
/// ```text
/// smsg SMSG_AREA_SPIRIT_HEALER_TIME = 0x02E4 {
///     Guid guid;
///     u32 next_resurrect_time;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_AREA_SPIRIT_HEALER_TIME {
    pub guid: Guid,
    pub next_resurrect_time: u32,
}

impl crate::private::Sealed for SMSG_AREA_SPIRIT_HEALER_TIME {}
impl SMSG_AREA_SPIRIT_HEALER_TIME {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // next_resurrect_time: u32
        let next_resurrect_time = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            next_resurrect_time,
        })
    }

}

impl crate::Message for SMSG_AREA_SPIRIT_HEALER_TIME {
    const OPCODE: u32 = 0x02e4;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_AREA_SPIRIT_HEALER_TIME"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_AREA_SPIRIT_HEALER_TIME {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    next_resurrect_time = {};", self.next_resurrect_time).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 740_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "next_resurrect_time", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // next_resurrect_time: u32
        w.write_all(&self.next_resurrect_time.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(740, "SMSG_AREA_SPIRIT_HEALER_TIME", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_AREA_SPIRIT_HEALER_TIME {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_AREA_SPIRIT_HEALER_TIME {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_AREA_SPIRIT_HEALER_TIME {}

