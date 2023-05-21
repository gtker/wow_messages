use std::io::{Read, Write};

use crate::tbc::ComplainResultWindow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_complain_result.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_complain_result.wowm#L8):
/// ```text
/// smsg SMSG_COMPLAIN_RESULT = 0x03C7 {
///     u8 unknown;
///     ComplainResultWindow window_result;
/// }
/// ```
pub struct SMSG_COMPLAIN_RESULT {
    /// All emulators set to 0.
    pub unknown: u8,
    pub window_result: ComplainResultWindow,
}

impl crate::private::Sealed for SMSG_COMPLAIN_RESULT {}
impl SMSG_COMPLAIN_RESULT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03C7, size: body_size });
        }

        // unknown: u8
        let unknown = crate::util::read_u8_le(&mut r)?;

        // window_result: ComplainResultWindow
        let window_result = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            unknown,
            window_result,
        })
    }

}

impl crate::Message for SMSG_COMPLAIN_RESULT {
    const OPCODE: u32 = 0x03c7;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_COMPLAIN_RESULT {{").unwrap();
        // Members
        writeln!(s, "    unknown = {};", self.unknown).unwrap();
        writeln!(s, "    window_result = {};", self.window_result.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 4_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 967_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "window_result", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        // window_result: ComplainResultWindow
        w.write_all(&(self.window_result.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_COMPLAIN_RESULT {}

