use std::io::{Read, Write};

use crate::wrath::ComplainResultWindow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_complain_result.wowm:17`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_complain_result.wowm#L17):
/// ```text
/// smsg SMSG_COMPLAIN_RESULT = 0x03C8 {
///     u8 unknown;
///     ComplainResultWindow window_result;
/// }
/// ```
pub struct SMSG_COMPLAIN_RESULT {
    /// All emulators set to 0.
    ///
    pub unknown: u8,
    pub window_result: ComplainResultWindow,
}

#[cfg(feature = "print-testcase")]
impl SMSG_COMPLAIN_RESULT {
    pub fn to_test_case_string(&self) -> Option<String> {
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
        let [a, b] = 968_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "window_result", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_COMPLAIN_RESULT {}
impl crate::Message for SMSG_COMPLAIN_RESULT {
    const OPCODE: u32 = 0x03c8;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_COMPLAIN_RESULT::to_test_case_string(self)
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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03C8, size: body_size });
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
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_COMPLAIN_RESULT {}

