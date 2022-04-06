use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/smsg_set_forced_reactions.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/smsg_set_forced_reactions.wowm#L3):
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

impl ReadableAndWritable for ForcedReaction {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // faction_id: u32
        let faction_id = crate::util::read_u32_le(r)?;

        // reputation_rank: u32
        let reputation_rank = crate::util::read_u32_le(r)?;

        Ok(Self {
            faction_id,
            reputation_rank,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // faction_id: u32
        w.write_all(&self.faction_id.to_le_bytes())?;

        // reputation_rank: u32
        w.write_all(&self.reputation_rank.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for ForcedReaction {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for ForcedReaction {
    fn maximum_possible_size() -> usize {
        4 // faction_id: u32
        + 4 // reputation_rank: u32
    }
}

