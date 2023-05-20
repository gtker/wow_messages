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
    ///
    pub unknown: u8,
    pub window_result: ComplainResultWindow,
}

impl crate::private::Sealed for SMSG_COMPLAIN_RESULT {}
impl crate::Message for SMSG_COMPLAIN_RESULT {
    const OPCODE: u32 = 0x03c7;

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
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03C7, size: body_size });
        }

        // unknown: u8
        let unknown = crate::util::read_u8_le(&mut r)?;

        // window_result: ComplainResultWindow
        let window_result: ComplainResultWindow = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            unknown,
            window_result,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_COMPLAIN_RESULT {}

