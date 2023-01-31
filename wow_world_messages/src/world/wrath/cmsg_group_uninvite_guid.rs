use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_group_uninvite_guid.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_group_uninvite_guid.wowm#L7):
/// ```text
/// cmsg CMSG_GROUP_UNINVITE_GUID = 0x0076 {
///     Guid guid;
///     CString reason;
/// }
/// ```
pub struct CMSG_GROUP_UNINVITE_GUID {
    pub guid: Guid,
    pub reason: String,
}

impl crate::Message for CMSG_GROUP_UNINVITE_GUID {
    const OPCODE: u32 = 0x0076;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // reason: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.reason.as_bytes().iter().rev().next(), Some(&0_u8), "String `reason` must not be null-terminated.");
        w.write_all(self.reason.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(9..=264).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0076, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // reason: CString
        let reason = crate::util::read_c_string_to_vec(r)?;
        let reason = String::from_utf8(reason)?;

        Ok(Self {
            guid,
            reason,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GROUP_UNINVITE_GUID {}

impl CMSG_GROUP_UNINVITE_GUID {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + self.reason.len() + 1 // reason: CString
    }
}

