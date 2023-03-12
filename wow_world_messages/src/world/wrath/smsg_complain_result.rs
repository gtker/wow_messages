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

impl crate::Message for SMSG_COMPLAIN_RESULT {
    const OPCODE: u32 = 0x03c8;

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        // window_result: ComplainResultWindow
        w.write_all(&u8::from(self.window_result.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03C8, size: body_size as u32 });
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
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_COMPLAIN_RESULT {}

