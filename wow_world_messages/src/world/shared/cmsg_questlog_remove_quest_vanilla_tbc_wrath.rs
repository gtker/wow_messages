use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/cmsg_questlog_remove_quest.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/cmsg_questlog_remove_quest.wowm#L3):
/// ```text
/// cmsg CMSG_QUESTLOG_REMOVE_QUEST = 0x0194 {
///     u8 slot;
/// }
/// ```
pub struct CMSG_QUESTLOG_REMOVE_QUEST {
    pub slot: u8,
}

impl crate::Message for CMSG_QUESTLOG_REMOVE_QUEST {
    const OPCODE: u32 = 0x0194;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0194, size: body_size as u32 });
        }

        // slot: u8
        let slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            slot,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_QUESTLOG_REMOVE_QUEST {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_QUESTLOG_REMOVE_QUEST {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_QUESTLOG_REMOVE_QUEST {}

