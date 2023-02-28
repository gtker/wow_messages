use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mount/smsg_mountspecial_anim.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mount/smsg_mountspecial_anim.wowm#L3):
/// ```text
/// smsg SMSG_MOUNTSPECIAL_ANIM = 0x0172 {
///     Guid guid;
/// }
/// ```
pub struct SMSG_MOUNTSPECIAL_ANIM {
    pub guid: Guid,
}

impl crate::Message for SMSG_MOUNTSPECIAL_ANIM {
    const OPCODE: u32 = 0x0172;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0172, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_MOUNTSPECIAL_ANIM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_MOUNTSPECIAL_ANIM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MOUNTSPECIAL_ANIM {}

