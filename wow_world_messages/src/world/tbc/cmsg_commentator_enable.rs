use std::io::{Read, Write};

use crate::tbc::CommentatorEnableOption;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_commentator_enable.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_commentator_enable.wowm#L9):
/// ```text
/// cmsg CMSG_COMMENTATOR_ENABLE = 0x03B4 {
///     CommentatorEnableOption option;
/// }
/// ```
pub struct CMSG_COMMENTATOR_ENABLE {
    pub option: CommentatorEnableOption,
}

impl crate::private::Sealed for CMSG_COMMENTATOR_ENABLE {}
impl CMSG_COMMENTATOR_ENABLE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03B4, size: body_size });
        }

        // option: CommentatorEnableOption
        let option = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            option,
        })
    }

}

impl crate::Message for CMSG_COMMENTATOR_ENABLE {
    const OPCODE: u32 = 0x03b4;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_COMMENTATOR_ENABLE {{").unwrap();
        // Members
        writeln!(s, "    option = {};", self.option.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 948_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "option", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // option: CommentatorEnableOption
        w.write_all(&(self.option.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_COMMENTATOR_ENABLE {}

