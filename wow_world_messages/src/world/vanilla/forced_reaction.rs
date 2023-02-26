use crate::vanilla::Faction;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_set_forced_reactions.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_set_forced_reactions.wowm#L1):
/// ```text
/// struct ForcedReaction {
///     Faction faction;
///     u32 reputation_rank;
/// }
/// ```
pub struct ForcedReaction {
    pub faction: Faction,
    pub reputation_rank: u32,
}

impl ForcedReaction {
    pub(crate) fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // faction: Faction
        w.write_all(&(self.faction.as_int() as u16).to_le_bytes())?;

        // reputation_rank: u32
        w.write_all(&self.reputation_rank.to_le_bytes())?;

        Ok(())
    }
}

impl ForcedReaction {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // faction: Faction
        let faction: Faction = crate::util::read_u16_le(r)?.try_into()?;

        // reputation_rank: u32
        let reputation_rank = crate::util::read_u32_le(r)?;

        Ok(Self {
            faction,
            reputation_rank,
        })
    }

}

