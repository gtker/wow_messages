use std::io::{Read, Write};

use wow_world_base::shared::level_vanilla_tbc_wrath::Level;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_guild_filter.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_guild_filter.wowm#L1):
/// ```text
/// cmsg CMSG_CALENDAR_GUILD_FILTER = 0x042B {
///     Level32 minimum_level;
///     Level32 maximum_level;
///     u32 minimum_rank;
/// }
/// ```
pub struct CMSG_CALENDAR_GUILD_FILTER {
    pub minimum_level: Level,
    pub maximum_level: Level,
    pub minimum_rank: u32,
}

impl crate::private::Sealed for CMSG_CALENDAR_GUILD_FILTER {}
impl crate::Message for CMSG_CALENDAR_GUILD_FILTER {
    const OPCODE: u32 = 0x042b;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // minimum_level: Level32
        w.write_all(&u32::from(self.minimum_level.as_int()).to_le_bytes())?;

        // maximum_level: Level32
        w.write_all(&u32::from(self.maximum_level.as_int()).to_le_bytes())?;

        // minimum_rank: u32
        w.write_all(&self.minimum_rank.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x042B, size: body_size });
        }

        // minimum_level: Level32
        let minimum_level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

        // maximum_level: Level32
        let maximum_level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

        // minimum_rank: u32
        let minimum_rank = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            minimum_level,
            maximum_level,
            minimum_rank,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CALENDAR_GUILD_FILTER {}

