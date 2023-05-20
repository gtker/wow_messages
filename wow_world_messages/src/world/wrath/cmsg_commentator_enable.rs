use std::io::{Read, Write};

use crate::wrath::CommentatorEnableOption;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_commentator_enable.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_commentator_enable.wowm#L15):
/// ```text
/// cmsg CMSG_COMMENTATOR_ENABLE = 0x03B5 {
///     CommentatorEnableOption option;
/// }
/// ```
pub struct CMSG_COMMENTATOR_ENABLE {
    pub option: CommentatorEnableOption,
}

impl crate::private::Sealed for CMSG_COMMENTATOR_ENABLE {}
impl crate::Message for CMSG_COMMENTATOR_ENABLE {
    const OPCODE: u32 = 0x03b5;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // option: CommentatorEnableOption
        w.write_all(&(self.option.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03B5, size: body_size });
        }

        // option: CommentatorEnableOption
        let option: CommentatorEnableOption = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            option,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_COMMENTATOR_ENABLE {}

