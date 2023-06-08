use std::io::{Read, Write};

use crate::tbc::LfgData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_set_looking_for_more.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_set_looking_for_more.wowm#L1):
/// ```text
/// cmsg CMSG_SET_LOOKING_FOR_MORE = 0x0365 {
///     LfgData data;
/// }
/// ```
pub struct CMSG_SET_LOOKING_FOR_MORE {
    pub data: LfgData,
}

#[cfg(feature = "print-testcase")]
impl CMSG_SET_LOOKING_FOR_MORE {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SET_LOOKING_FOR_MORE {{").unwrap();
        // Members
        // data: LfgData
        writeln!(s, "    data = {{").unwrap();
        // Members
        writeln!(s, "        entry = {};", self.data.entry).unwrap();
        writeln!(s, "        lfg_type = {};", self.data.lfg_type.as_test_case_value()).unwrap();

        writeln!(s, "    }};").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 869_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        writeln!(s, "    /* data: LfgData start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 2, "entry", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 2, "lfg_type", "        ");
        writeln!(s, "    /* data: LfgData end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_SET_LOOKING_FOR_MORE {}
impl crate::Message for CMSG_SET_LOOKING_FOR_MORE {
    const OPCODE: u32 = 0x0365;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_SET_LOOKING_FOR_MORE::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // data: LfgData
        self.data.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0365, size: body_size });
        }

        // data: LfgData
        let data = LfgData::read(&mut r)?;

        Ok(Self {
            data,
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_LOOKING_FOR_MORE {}

