use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{TrainerSpellState, TrainerSpellStateError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct TrainerSpell {
    pub spell: u32,
    pub state: TrainerSpellState,
    pub spell_cost: u32,
    pub talent_point_cost: u32,
    pub first_rank: u32,
    pub required_level: u8,
    pub required_skill: u32,
    pub required_skill_value: u32,
    pub spell_chain_required: u32,
    pub spell_chain_previous: u32,
    pub unknown1: u32,
}

impl ReadableAndWritable for TrainerSpell {
    type Error = TrainerSpellError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // state: TrainerSpellState
        let state = TrainerSpellState::read(r)?;

        // spell_cost: u32
        let spell_cost = crate::util::read_u32_le(r)?;

        // talent_point_cost: u32
        let talent_point_cost = crate::util::read_u32_le(r)?;

        // first_rank: u32
        let first_rank = crate::util::read_u32_le(r)?;

        // required_level: u8
        let required_level = crate::util::read_u8_le(r)?;

        // required_skill: u32
        let required_skill = crate::util::read_u32_le(r)?;

        // required_skill_value: u32
        let required_skill_value = crate::util::read_u32_le(r)?;

        // spell_chain_required: u32
        let spell_chain_required = crate::util::read_u32_le(r)?;

        // spell_chain_previous: u32
        let spell_chain_previous = crate::util::read_u32_le(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        Ok(Self {
            spell,
            state,
            spell_cost,
            talent_point_cost,
            first_rank,
            required_level,
            required_skill,
            required_skill_value,
            spell_chain_required,
            spell_chain_previous,
            unknown1,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // state: TrainerSpellState
        self.state.write(w)?;

        // spell_cost: u32
        w.write_all(&self.spell_cost.to_le_bytes())?;

        // talent_point_cost: u32
        w.write_all(&self.talent_point_cost.to_le_bytes())?;

        // first_rank: u32
        w.write_all(&self.first_rank.to_le_bytes())?;

        // required_level: u8
        w.write_all(&self.required_level.to_le_bytes())?;

        // required_skill: u32
        w.write_all(&self.required_skill.to_le_bytes())?;

        // required_skill_value: u32
        w.write_all(&self.required_skill_value.to_le_bytes())?;

        // spell_chain_required: u32
        w.write_all(&self.spell_chain_required.to_le_bytes())?;

        // spell_chain_previous: u32
        w.write_all(&self.spell_chain_previous.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }

}

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
#[async_trait]
impl AsyncReadWrite for TrainerSpell {
    type Error = TrainerSpellError;
    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> Result<Self, Self::Error> {
        // spell: u32
        let spell = crate::util::tokio_read_u32_le(r).await?;

        // state: TrainerSpellState
        let state = TrainerSpellState::tokio_read(r).await?;

        // spell_cost: u32
        let spell_cost = crate::util::tokio_read_u32_le(r).await?;

        // talent_point_cost: u32
        let talent_point_cost = crate::util::tokio_read_u32_le(r).await?;

        // first_rank: u32
        let first_rank = crate::util::tokio_read_u32_le(r).await?;

        // required_level: u8
        let required_level = crate::util::tokio_read_u8_le(r).await?;

        // required_skill: u32
        let required_skill = crate::util::tokio_read_u32_le(r).await?;

        // required_skill_value: u32
        let required_skill_value = crate::util::tokio_read_u32_le(r).await?;

        // spell_chain_required: u32
        let spell_chain_required = crate::util::tokio_read_u32_le(r).await?;

        // spell_chain_previous: u32
        let spell_chain_previous = crate::util::tokio_read_u32_le(r).await?;

        // unknown1: u32
        let unknown1 = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            spell,
            state,
            spell_cost,
            talent_point_cost,
            first_rank,
            required_level,
            required_skill,
            required_skill_value,
            spell_chain_required,
            spell_chain_previous,
            unknown1,
        })
    }
    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes()).await?;

        // state: TrainerSpellState
        self.state.tokio_write(w).await?;

        // spell_cost: u32
        w.write_all(&self.spell_cost.to_le_bytes()).await?;

        // talent_point_cost: u32
        w.write_all(&self.talent_point_cost.to_le_bytes()).await?;

        // first_rank: u32
        w.write_all(&self.first_rank.to_le_bytes()).await?;

        // required_level: u8
        w.write_all(&self.required_level.to_le_bytes()).await?;

        // required_skill: u32
        w.write_all(&self.required_skill.to_le_bytes()).await?;

        // required_skill_value: u32
        w.write_all(&self.required_skill_value.to_le_bytes()).await?;

        // spell_chain_required: u32
        w.write_all(&self.spell_chain_required.to_le_bytes()).await?;

        // spell_chain_previous: u32
        w.write_all(&self.spell_chain_previous.to_le_bytes()).await?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes()).await?;

        Ok(())
    }
}
impl ConstantSized for TrainerSpell {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for TrainerSpell {
    fn maximum_possible_size() -> usize {
        4 // spell: u32
        + TrainerSpellState::size() // state: TrainerSpellState
        + 4 // spell_cost: u32
        + 4 // talent_point_cost: u32
        + 4 // first_rank: u32
        + 1 // required_level: u8
        + 4 // required_skill: u32
        + 4 // required_skill_value: u32
        + 4 // spell_chain_required: u32
        + 4 // spell_chain_previous: u32
        + 4 // unknown1: u32
    }
}

#[derive(Debug)]
pub enum TrainerSpellError {
    Io(std::io::Error),
    TrainerSpellState(TrainerSpellStateError),
}

impl std::error::Error for TrainerSpellError {}
impl std::fmt::Display for TrainerSpellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::TrainerSpellState(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for TrainerSpellError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<TrainerSpellStateError> for TrainerSpellError {
    fn from(e: TrainerSpellStateError) -> Self {
        Self::TrainerSpellState(e)
    }
}

