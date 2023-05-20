use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/cmsg_quest_confirm_accept.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/cmsg_quest_confirm_accept.wowm#L3):
/// ```text
/// cmsg CMSG_QUEST_CONFIRM_ACCEPT = 0x019B {
///     u32 quest_id;
/// }
/// ```
pub struct CMSG_QUEST_CONFIRM_ACCEPT {
    pub quest_id: u32,
}

impl crate::private::Sealed for CMSG_QUEST_CONFIRM_ACCEPT {}
impl crate::Message for CMSG_QUEST_CONFIRM_ACCEPT {
    const OPCODE: u32 = 0x019b;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x019B, size: body_size });
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            quest_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_QUEST_CONFIRM_ACCEPT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_QUEST_CONFIRM_ACCEPT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_QUEST_CONFIRM_ACCEPT {}

