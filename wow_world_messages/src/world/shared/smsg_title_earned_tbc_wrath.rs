use wow_world_base::shared::title_earn_status_tbc_wrath::TitleEarnStatus;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_title_earned.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_title_earned.wowm#L8):
/// ```text
/// smsg SMSG_TITLE_EARNED = 0x0373 {
///     u32 title;
///     TitleEarnStatus status;
/// }
/// ```
pub struct SMSG_TITLE_EARNED {
    pub title: u32,
    pub status: TitleEarnStatus,
}

impl crate::Message for SMSG_TITLE_EARNED {
    const OPCODE: u32 = 0x0373;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // title: u32
        w.write_all(&self.title.to_le_bytes())?;

        // status: TitleEarnStatus
        w.write_all(&(self.status.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0373, size: body_size as u32 });
        }

        // title: u32
        let title = crate::util::read_u32_le(r)?;

        // status: TitleEarnStatus
        let status: TitleEarnStatus = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            title,
            status,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_TITLE_EARNED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_TITLE_EARNED {}

