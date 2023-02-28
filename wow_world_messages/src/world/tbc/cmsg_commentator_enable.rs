use crate::tbc::CommentatorEnableOption;
use std::io::{Write, Read};

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

impl crate::Message for CMSG_COMMENTATOR_ENABLE {
    const OPCODE: u32 = 0x03b4;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // option: CommentatorEnableOption
        w.write_all(&u32::from(self.option.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03B4, size: body_size as u32 });
        }

        // option: CommentatorEnableOption
        let option: CommentatorEnableOption = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            option,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_COMMENTATOR_ENABLE {}

