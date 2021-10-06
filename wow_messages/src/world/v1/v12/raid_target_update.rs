use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::v1::v12::{RaidTargetIndex, RaidTargetIndexError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid_target.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid_target.wowm):
/// ```text
/// struct RaidTargetUpdate {
///     RaidTargetIndex index;
///     Guid guid;
/// }
/// ```
pub struct RaidTargetUpdate {
    pub index: RaidTargetIndex,
    pub guid: Guid,
}

impl ReadableAndWritable for RaidTargetUpdate {
    type Error = RaidTargetUpdateError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // index: RaidTargetIndex
        let index = RaidTargetIndex::read(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            index,
            guid,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // index: RaidTargetIndex
        self.index.write(w)?;

        // guid: Guid
        self.guid.write(w)?;

        Ok(())
    }

}

impl ConstantSized for RaidTargetUpdate {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for RaidTargetUpdate {
    fn maximum_possible_size() -> usize {
        RaidTargetIndex::size() // index: RaidTargetIndex
        + 8 // guid: Guid
    }
}

#[derive(Debug)]
pub enum RaidTargetUpdateError {
    Io(std::io::Error),
    RaidTargetIndex(RaidTargetIndexError),
}

impl std::error::Error for RaidTargetUpdateError {}
impl std::fmt::Display for RaidTargetUpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::RaidTargetIndex(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for RaidTargetUpdateError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<RaidTargetIndexError> for RaidTargetUpdateError {
    fn from(e: RaidTargetIndexError) -> Self {
        Self::RaidTargetIndex(e)
    }
}

