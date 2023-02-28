use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_fish_not_hooked.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_fish_not_hooked.wowm#L3):
/// ```text
/// smsg SMSG_FISH_NOT_HOOKED = 0x01C8 {
/// }
/// ```
pub struct SMSG_FISH_NOT_HOOKED {
}

impl crate::Message for SMSG_FISH_NOT_HOOKED {
    const OPCODE: u32 = 0x01c8;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01C8, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_FISH_NOT_HOOKED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_FISH_NOT_HOOKED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_FISH_NOT_HOOKED {}

