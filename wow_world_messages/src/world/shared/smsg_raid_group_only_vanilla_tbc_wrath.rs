use wow_world_base::shared::raid_group_error_vanilla_tbc_wrath::RaidGroupError;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// used when player leaves raid group inside instance
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_raid_group_only.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_raid_group_only.wowm#L8):
/// ```text
/// smsg SMSG_RAID_GROUP_ONLY = 0x0286 {
///     u32 homebind_timer;
///     RaidGroupError error;
/// }
/// ```
pub struct SMSG_RAID_GROUP_ONLY {
    pub homebind_timer: u32,
    pub error: RaidGroupError,
}

impl crate::Message for SMSG_RAID_GROUP_ONLY {
    const OPCODE: u32 = 0x0286;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // homebind_timer: u32
        w.write_all(&self.homebind_timer.to_le_bytes())?;

        // error: RaidGroupError
        w.write_all(&(self.error.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0286, size: body_size as u32 });
        }

        // homebind_timer: u32
        let homebind_timer = crate::util::read_u32_le(r)?;

        // error: RaidGroupError
        let error: RaidGroupError = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            homebind_timer,
            error,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_RAID_GROUP_ONLY {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_RAID_GROUP_ONLY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_RAID_GROUP_ONLY {}

