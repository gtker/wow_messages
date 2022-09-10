use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/cmsg_bug.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_bug.wowm#L3):
/// ```text
/// cmsg CMSG_BUG = 0x01CA {
///     u32 suggestion;
///     SizedCString content;
///     SizedCString bug_type;
/// }
/// ```
pub struct CMSG_BUG {
    /// cmangos/vmangos/mangoszero: If 0 received bug report, else received suggestion
    ///
    pub suggestion: u32,
    pub content: String,
    pub bug_type: String,
}

impl crate::Message for CMSG_BUG {
    const OPCODE: u32 = 0x01ca;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // suggestion: u32
        w.write_all(&self.suggestion.to_le_bytes())?;

        // content: SizedCString
        w.write_all(&((self.content.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.content.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // bug_type: SizedCString
        w.write_all(&((self.bug_type.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.bug_type.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // suggestion: u32
        let suggestion = crate::util::read_u32_le(r)?;

        // content: SizedCString
        let content = crate::util::read_u32_le(r)?;
        let content = crate::util::read_sized_c_string_to_vec(r, content)?;
        let content = String::from_utf8(content)?;;
        // bug_type: SizedCString
        let bug_type = crate::util::read_u32_le(r)?;
        let bug_type = crate::util::read_sized_c_string_to_vec(r, bug_type)?;
        let bug_type = String::from_utf8(bug_type)?;;
        Ok(Self {
            suggestion,
            content,
            bug_type,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ClientMessage for CMSG_BUG {}

impl CMSG_BUG {
    pub(crate) fn size(&self) -> usize {
        4 // suggestion: u32
        + self.content.len() + 5 // content: SizedCString
        + self.bug_type.len() + 5 // bug_type: SizedCString
    }
}

