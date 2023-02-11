use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pvp/msg_inspect_honor_stats_server.wowm:28`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pvp/msg_inspect_honor_stats_server.wowm#L28):
/// ```text
/// smsg MSG_INSPECT_HONOR_STATS_Server = 0x02D6 {
///     Guid guid;
///     u8 amount_of_honor;
///     u32 kills;
///     u32 honor_today;
///     u32 honor_yesterday;
///     u32 lifetime_honorable_kills;
/// }
/// ```
pub struct MSG_INSPECT_HONOR_STATS_Server {
    pub guid: Guid,
    pub amount_of_honor: u8,
    pub kills: u32,
    pub honor_today: u32,
    pub honor_yesterday: u32,
    pub lifetime_honorable_kills: u32,
}

impl crate::Message for MSG_INSPECT_HONOR_STATS_Server {
    const OPCODE: u32 = 0x02d6;

    fn size_without_header(&self) -> u32 {
        25
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // amount_of_honor: u8
        w.write_all(&self.amount_of_honor.to_le_bytes())?;

        // kills: u32
        w.write_all(&self.kills.to_le_bytes())?;

        // honor_today: u32
        w.write_all(&self.honor_today.to_le_bytes())?;

        // honor_yesterday: u32
        w.write_all(&self.honor_yesterday.to_le_bytes())?;

        // lifetime_honorable_kills: u32
        w.write_all(&self.lifetime_honorable_kills.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 25 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02D6, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // amount_of_honor: u8
        let amount_of_honor = crate::util::read_u8_le(r)?;

        // kills: u32
        let kills = crate::util::read_u32_le(r)?;

        // honor_today: u32
        let honor_today = crate::util::read_u32_le(r)?;

        // honor_yesterday: u32
        let honor_yesterday = crate::util::read_u32_le(r)?;

        // lifetime_honorable_kills: u32
        let lifetime_honorable_kills = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            amount_of_honor,
            kills,
            honor_today,
            honor_yesterday,
            lifetime_honorable_kills,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_INSPECT_HONOR_STATS_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_INSPECT_HONOR_STATS_Server {}

