use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::TrainerSpellState;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

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

impl TrainerSpell {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 38], std::io::Error> {
        let mut array_w = [0u8; 38];
        let mut w = array_w.as_mut_slice();
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // state: TrainerSpellState
        w.write_all(&(self.state.as_int() as u8).to_le_bytes())?;

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

        Ok(array_w)
    }
}

impl TrainerSpell {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TrainerSpellError> {
        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // state: TrainerSpellState
        let state: TrainerSpellState = crate::util::read_u8_le(r)?.try_into()?;

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

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, TrainerSpellError> {
        // spell: u32
        let spell = crate::util::tokio_read_u32_le(r).await?;

        // state: TrainerSpellState
        let state: TrainerSpellState = crate::util::tokio_read_u8_le(r).await?.try_into()?;

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

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, TrainerSpellError> {
        // spell: u32
        let spell = crate::util::astd_read_u32_le(r).await?;

        // state: TrainerSpellState
        let state: TrainerSpellState = crate::util::astd_read_u8_le(r).await?.try_into()?;

        // spell_cost: u32
        let spell_cost = crate::util::astd_read_u32_le(r).await?;

        // talent_point_cost: u32
        let talent_point_cost = crate::util::astd_read_u32_le(r).await?;

        // first_rank: u32
        let first_rank = crate::util::astd_read_u32_le(r).await?;

        // required_level: u8
        let required_level = crate::util::astd_read_u8_le(r).await?;

        // required_skill: u32
        let required_skill = crate::util::astd_read_u32_le(r).await?;

        // required_skill_value: u32
        let required_skill_value = crate::util::astd_read_u32_le(r).await?;

        // spell_chain_required: u32
        let spell_chain_required = crate::util::astd_read_u32_le(r).await?;

        // spell_chain_previous: u32
        let spell_chain_previous = crate::util::astd_read_u32_le(r).await?;

        // unknown1: u32
        let unknown1 = crate::util::astd_read_u32_le(r).await?;

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

}

#[derive(Debug)]
pub enum TrainerSpellError {
    Io(std::io::Error),
    Enum(crate::errors::EnumError),
}

impl std::error::Error for TrainerSpellError {}
impl std::fmt::Display for TrainerSpellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Enum(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for TrainerSpellError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::errors::EnumError> for TrainerSpellError {
    fn from(e: crate::errors::EnumError) -> Self {
        Self::Enum(e)
    }
}

