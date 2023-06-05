use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::PvpRank;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pvp/msg_inspect_honor_stats_server.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pvp/msg_inspect_honor_stats_server.wowm#L1):
/// ```text
/// smsg MSG_INSPECT_HONOR_STATS_Server = 0x02D6 {
///     Guid guid;
///     PvpRank highest_rank;
///     u32 today_honorable_and_dishonorable;
///     u16 yesterday_honorable;
///     u16 unknown1;
///     u16 last_week_honorable;
///     u16 unknown2;
///     u16 this_week_honorable;
///     u16 unknown3;
///     u32 lifetime_honorable;
///     u32 lifetime_dishonorable;
///     u32 yesterday_honor;
///     u32 last_week_honor;
///     u32 this_week_honor;
///     (u32)PvpRank last_week_standing;
///     u8 rank_progress_bar;
/// }
/// ```
pub struct MSG_INSPECT_HONOR_STATS_Server {
    pub guid: Guid,
    pub highest_rank: PvpRank,
    pub today_honorable_and_dishonorable: u32,
    pub yesterday_honorable: u16,
    /// vmangos: Unknown (deprecated, yesterday dishonourable?)
    ///
    pub unknown1: u16,
    pub last_week_honorable: u16,
    /// vmangos: Unknown (deprecated, last week dishonourable?)
    ///
    pub unknown2: u16,
    pub this_week_honorable: u16,
    /// vmangos: Unknown (deprecated, this week dishonourable?)
    ///
    pub unknown3: u16,
    pub lifetime_honorable: u32,
    pub lifetime_dishonorable: u32,
    pub yesterday_honor: u32,
    pub last_week_honor: u32,
    pub this_week_honor: u32,
    pub last_week_standing: PvpRank,
    pub rank_progress_bar: u8,
}

impl crate::private::Sealed for MSG_INSPECT_HONOR_STATS_Server {}
impl crate::Message for MSG_INSPECT_HONOR_STATS_Server {
    const OPCODE: u32 = 0x02d6;

    fn size_without_header(&self) -> u32 {
        50
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // highest_rank: PvpRank
        w.write_all(&(self.highest_rank.as_int().to_le_bytes()))?;

        // today_honorable_and_dishonorable: u32
        w.write_all(&self.today_honorable_and_dishonorable.to_le_bytes())?;

        // yesterday_honorable: u16
        w.write_all(&self.yesterday_honorable.to_le_bytes())?;

        // unknown1: u16
        w.write_all(&self.unknown1.to_le_bytes())?;

        // last_week_honorable: u16
        w.write_all(&self.last_week_honorable.to_le_bytes())?;

        // unknown2: u16
        w.write_all(&self.unknown2.to_le_bytes())?;

        // this_week_honorable: u16
        w.write_all(&self.this_week_honorable.to_le_bytes())?;

        // unknown3: u16
        w.write_all(&self.unknown3.to_le_bytes())?;

        // lifetime_honorable: u32
        w.write_all(&self.lifetime_honorable.to_le_bytes())?;

        // lifetime_dishonorable: u32
        w.write_all(&self.lifetime_dishonorable.to_le_bytes())?;

        // yesterday_honor: u32
        w.write_all(&self.yesterday_honor.to_le_bytes())?;

        // last_week_honor: u32
        w.write_all(&self.last_week_honor.to_le_bytes())?;

        // this_week_honor: u32
        w.write_all(&self.this_week_honor.to_le_bytes())?;

        // last_week_standing: PvpRank
        w.write_all(&u32::from(self.last_week_standing.as_int()).to_le_bytes())?;

        // rank_progress_bar: u8
        w.write_all(&self.rank_progress_bar.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 50 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02D6, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // highest_rank: PvpRank
        let highest_rank: PvpRank = crate::util::read_u8_le(&mut r)?.try_into()?;

        // today_honorable_and_dishonorable: u32
        let today_honorable_and_dishonorable = crate::util::read_u32_le(&mut r)?;

        // yesterday_honorable: u16
        let yesterday_honorable = crate::util::read_u16_le(&mut r)?;

        // unknown1: u16
        let unknown1 = crate::util::read_u16_le(&mut r)?;

        // last_week_honorable: u16
        let last_week_honorable = crate::util::read_u16_le(&mut r)?;

        // unknown2: u16
        let unknown2 = crate::util::read_u16_le(&mut r)?;

        // this_week_honorable: u16
        let this_week_honorable = crate::util::read_u16_le(&mut r)?;

        // unknown3: u16
        let unknown3 = crate::util::read_u16_le(&mut r)?;

        // lifetime_honorable: u32
        let lifetime_honorable = crate::util::read_u32_le(&mut r)?;

        // lifetime_dishonorable: u32
        let lifetime_dishonorable = crate::util::read_u32_le(&mut r)?;

        // yesterday_honor: u32
        let yesterday_honor = crate::util::read_u32_le(&mut r)?;

        // last_week_honor: u32
        let last_week_honor = crate::util::read_u32_le(&mut r)?;

        // this_week_honor: u32
        let this_week_honor = crate::util::read_u32_le(&mut r)?;

        // last_week_standing: PvpRank
        let last_week_standing: PvpRank = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // rank_progress_bar: u8
        let rank_progress_bar = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            guid,
            highest_rank,
            today_honorable_and_dishonorable,
            yesterday_honorable,
            unknown1,
            last_week_honorable,
            unknown2,
            this_week_honorable,
            unknown3,
            lifetime_honorable,
            lifetime_dishonorable,
            yesterday_honor,
            last_week_honor,
            this_week_honor,
            last_week_standing,
            rank_progress_bar,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_INSPECT_HONOR_STATS_Server {}

