use std::io::{Read, Write};
use crate::vanilla::BgTypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm#L21):
/// ```text
/// smsg SMSG_GROUP_JOINED_BATTLEGROUND = 0x02E8 {
///     BgTypeId id;
/// }
/// ```
pub struct SMSG_GROUP_JOINED_BATTLEGROUND {
    pub id: BgTypeId,
}

impl crate::Message for SMSG_GROUP_JOINED_BATTLEGROUND {
    const OPCODE: u32 = 0x02e8;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // id: BgTypeId
        w.write_all(&u32::from(self.id.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02E8, size: body_size as u32 });
        }

        // id: BgTypeId
        let id: BgTypeId = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_GROUP_JOINED_BATTLEGROUND {}

