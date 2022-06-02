use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_set_forced_reactions.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_set_forced_reactions.wowm#L3):
/// ```text
/// struct ForcedReaction {
///     u32 faction_id;
///     u32 reputation_rank;
/// }
/// ```
pub struct ForcedReaction {
    pub faction_id: u32,
    pub reputation_rank: u32,
}

impl ForcedReaction {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // faction_id: u32
        w.write_all(&self.faction_id.to_le_bytes())?;

        // reputation_rank: u32
        w.write_all(&self.reputation_rank.to_le_bytes())?;

        Ok(())
    }
}

impl ForcedReaction {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // faction_id: u32
        let faction_id = crate::util::read_u32_le(r)?;

        // reputation_rank: u32
        let reputation_rank = crate::util::read_u32_le(r)?;

        Ok(Self {
            faction_id,
            reputation_rank,
        })
    }

}

