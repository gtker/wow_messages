use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/cmsg_bug.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_bug.wowm#L3):
/// ```text
/// cmsg CMSG_BUG = 0x01CA {
///     u32 suggestion;
///     u32 content_length;
///     CString content;
///     u32 type_length;
///     CString bug_type;
/// }
/// ```
pub struct CMSG_BUG {
    /// cmangos/vmangos/mangoszero: If 0 received bug report, else received suggestion
    ///
    pub suggestion: u32,
    pub content_length: u32,
    pub content: String,
    pub type_length: u32,
    pub bug_type: String,
}

impl ClientMessage for CMSG_BUG {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // suggestion: u32
        w.write_all(&self.suggestion.to_le_bytes())?;

        // content_length: u32
        w.write_all(&self.content_length.to_le_bytes())?;

        // content: CString
        w.write_all(self.content.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // type_length: u32
        w.write_all(&self.type_length.to_le_bytes())?;

        // bug_type: CString
        w.write_all(self.bug_type.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x01ca;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // suggestion: u32
        let suggestion = crate::util::read_u32_le(r)?;

        // content_length: u32
        let content_length = crate::util::read_u32_le(r)?;

        // content: CString
        let content = crate::util::read_c_string_to_vec(r)?;
        let content = String::from_utf8(content)?;

        // type_length: u32
        let type_length = crate::util::read_u32_le(r)?;

        // bug_type: CString
        let bug_type = crate::util::read_c_string_to_vec(r)?;
        let bug_type = String::from_utf8(bug_type)?;

        Ok(Self {
            suggestion,
            content_length,
            content,
            type_length,
            bug_type,
        })
    }

}

impl CMSG_BUG {
    pub(crate) fn size(&self) -> usize {
        4 // suggestion: u32
        + 4 // content_length: u32
        + self.content.len() + 1 // content: CString
        + 4 // type_length: u32
        + self.bug_type.len() + 1 // bug_type: CString
    }
}

