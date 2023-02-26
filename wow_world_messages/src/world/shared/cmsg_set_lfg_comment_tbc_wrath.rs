use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_set_lfg_comment.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_set_lfg_comment.wowm#L1):
/// ```text
/// cmsg CMSG_SET_LFG_COMMENT = 0x0366 {
///     CString comment;
/// }
/// ```
pub struct CMSG_SET_LFG_COMMENT {
    pub comment: String,
}

impl crate::Message for CMSG_SET_LFG_COMMENT {
    const OPCODE: u32 = 0x0366;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // comment: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.comment.as_bytes().iter().rev().next(), Some(&0_u8), "String `comment` must not be null-terminated.");
        w.write_all(self.comment.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0366, size: body_size as u32 });
        }

        // comment: CString
        let comment = {
            let comment = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(comment)?
        };

        Ok(Self {
            comment,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_LFG_COMMENT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_LFG_COMMENT {}

impl CMSG_SET_LFG_COMMENT {
    pub(crate) fn size(&self) -> usize {
        self.comment.len() + 1 // comment: CString
    }
}

