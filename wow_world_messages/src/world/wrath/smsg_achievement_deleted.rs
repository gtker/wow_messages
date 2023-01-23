use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_achievement_deleted.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_achievement_deleted.wowm#L1):
/// ```text
/// smsg SMSG_ACHIEVEMENT_DELETED = 0x049F {
///     u32 achievement;
/// }
/// ```
pub struct SMSG_ACHIEVEMENT_DELETED {
    pub achievement: u32,
}

impl crate::Message for SMSG_ACHIEVEMENT_DELETED {
    const OPCODE: u32 = 0x049f;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // achievement: u32
        w.write_all(&self.achievement.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x049F, size: body_size as u32 });
        }

        // achievement: u32
        let achievement = crate::util::read_u32_le(r)?;

        Ok(Self {
            achievement,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_ACHIEVEMENT_DELETED {}

