use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/cmsg_pushquesttoparty.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/cmsg_pushquesttoparty.wowm#L3):
/// ```text
/// cmsg CMSG_PUSHQUESTTOPARTY = 0x019D {
///     u32 quest_id;
/// }
/// ```
pub struct CMSG_PUSHQUESTTOPARTY {
    pub quest_id: u32,
}

impl crate::private::Sealed for CMSG_PUSHQUESTTOPARTY {}
impl crate::Message for CMSG_PUSHQUESTTOPARTY {
    const OPCODE: u32 = 0x019d;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x019D, size: body_size });
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            quest_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PUSHQUESTTOPARTY {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_PUSHQUESTTOPARTY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PUSHQUESTTOPARTY {}

