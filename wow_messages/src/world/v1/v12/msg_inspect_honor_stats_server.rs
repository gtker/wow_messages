use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::v1::v12::{PvpRank, PvpRankError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/msg_inspect_honor_stats_server.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/msg_inspect_honor_stats_server.wowm#L3):
/// ```text
/// smsg MSG_INSPECT_HONOR_STATS_Server = 0x2D6 {
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
///     PvpRank last_week_standing;
///     u8 rank_progress_bar;
/// }
/// ```
pub struct MSG_INSPECT_HONOR_STATS_Server {
    pub guid: Guid,
    pub highest_rank: PvpRank,
    pub today_honorable_and_dishonorable: u32,
    pub yesterday_honorable: u16,
    pub unknown1: u16,
    pub last_week_honorable: u16,
    pub unknown2: u16,
    pub this_week_honorable: u16,
    pub unknown3: u16,
    pub lifetime_honorable: u32,
    pub lifetime_dishonorable: u32,
    pub yesterday_honor: u32,
    pub last_week_honor: u32,
    pub this_week_honor: u32,
    pub last_week_standing: PvpRank,
    pub rank_progress_bar: u8,
}

impl WorldServerMessageWrite for MSG_INSPECT_HONOR_STATS_Server {
    const OPCODE: u16 = 0x2d6;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for MSG_INSPECT_HONOR_STATS_Server {
    type Error = MSG_INSPECT_HONOR_STATS_ServerError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // highest_rank: PvpRank
        let highest_rank = PvpRank::read(r)?;

        // today_honorable_and_dishonorable: u32
        let today_honorable_and_dishonorable = crate::util::read_u32_le(r)?;

        // yesterday_honorable: u16
        let yesterday_honorable = crate::util::read_u16_le(r)?;

        // unknown1: u16
        let unknown1 = crate::util::read_u16_le(r)?;

        // last_week_honorable: u16
        let last_week_honorable = crate::util::read_u16_le(r)?;

        // unknown2: u16
        let unknown2 = crate::util::read_u16_le(r)?;

        // this_week_honorable: u16
        let this_week_honorable = crate::util::read_u16_le(r)?;

        // unknown3: u16
        let unknown3 = crate::util::read_u16_le(r)?;

        // lifetime_honorable: u32
        let lifetime_honorable = crate::util::read_u32_le(r)?;

        // lifetime_dishonorable: u32
        let lifetime_dishonorable = crate::util::read_u32_le(r)?;

        // yesterday_honor: u32
        let yesterday_honor = crate::util::read_u32_le(r)?;

        // last_week_honor: u32
        let last_week_honor = crate::util::read_u32_le(r)?;

        // this_week_honor: u32
        let this_week_honor = crate::util::read_u32_le(r)?;

        // last_week_standing: PvpRank
        let last_week_standing = PvpRank::read_u32_le(r)?;

        // rank_progress_bar: u8
        let rank_progress_bar = crate::util::read_u8_le(r)?;

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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // highest_rank: PvpRank
        self.highest_rank.write(w)?;

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
        self.last_week_standing.write_u32_le(w)?;

        // rank_progress_bar: u8
        w.write_all(&self.rank_progress_bar.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for MSG_INSPECT_HONOR_STATS_Server {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for MSG_INSPECT_HONOR_STATS_Server {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + PvpRank::size() // highest_rank: PvpRank
        + 4 // today_honorable_and_dishonorable: u32
        + 2 // yesterday_honorable: u16
        + 2 // unknown1: u16
        + 2 // last_week_honorable: u16
        + 2 // unknown2: u16
        + 2 // this_week_honorable: u16
        + 2 // unknown3: u16
        + 4 // lifetime_honorable: u32
        + 4 // lifetime_dishonorable: u32
        + 4 // yesterday_honor: u32
        + 4 // last_week_honor: u32
        + 4 // this_week_honor: u32
        + 4 // last_week_standing: PvpRank upcasted to u32
        + 1 // rank_progress_bar: u8
    }
}

#[derive(Debug)]
pub enum MSG_INSPECT_HONOR_STATS_ServerError {
    Io(std::io::Error),
    PvpRank(PvpRankError),
}

impl std::error::Error for MSG_INSPECT_HONOR_STATS_ServerError {}
impl std::fmt::Display for MSG_INSPECT_HONOR_STATS_ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PvpRank(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for MSG_INSPECT_HONOR_STATS_ServerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PvpRankError> for MSG_INSPECT_HONOR_STATS_ServerError {
    fn from(e: PvpRankError) -> Self {
        Self::PvpRank(e)
    }
}

