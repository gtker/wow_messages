use std::convert::{TryFrom, TryInto};
use crate::helper::Guid;
use crate::world::v1::v12::{PetCommandState, PetCommandStateError};
use crate::world::v1::v12::{PetReactState, PetReactStateError};
use crate::world::v1::v12::PetSpellCooldown;
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_spells.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_spells.wowm#L12):
/// ```text
/// smsg SMSG_PET_SPELLS = 0x179 {
///     Guid pet;
///     u32 unknown1;
///     PetReactState react;
///     PetCommandState command;
///     u16 unknown2;
///     u32[10] action_bars;
///     u8 amount_of_spells;
///     u32[amount_of_spells] spells;
///     u8 amount_of_cooldowns;
///     PetSpellCooldown[amount_of_cooldowns] cooldowns;
/// }
/// ```
pub struct SMSG_PET_SPELLS {
    pub pet: Guid,
    pub unknown1: u32,
    pub react: PetReactState,
    pub command: PetCommandState,
    pub unknown2: u16,
    pub action_bars: [u32; 10],
    pub amount_of_spells: u8,
    pub spells: Vec<u32>,
    pub amount_of_cooldowns: u8,
    pub cooldowns: Vec<PetSpellCooldown>,
}

impl WorldServerMessageWrite for SMSG_PET_SPELLS {
    const OPCODE: u16 = 0x179;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_PET_SPELLS {
    type Error = SMSG_PET_SPELLSError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet: Guid
        let pet = Guid::read(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // react: PetReactState
        let react = PetReactState::read(r)?;

        // command: PetCommandState
        let command = PetCommandState::read(r)?;

        // unknown2: u16
        let unknown2 = crate::util::read_u16_le(r)?;

        // action_bars: u32[10]
        let mut action_bars = Vec::with_capacity(10 as usize);
        for i in 0..10 {
            action_bars.push(crate::util::read_u32_le(r)?);
        }
        let action_bars = action_bars.try_into().unwrap();

        // amount_of_spells: u8
        let amount_of_spells = crate::util::read_u8_le(r)?;

        // spells: u32[amount_of_spells]
        let mut spells = Vec::with_capacity(amount_of_spells as usize);
        for i in 0..amount_of_spells {
            spells.push(crate::util::read_u32_le(r)?);
        }

        // amount_of_cooldowns: u8
        let amount_of_cooldowns = crate::util::read_u8_le(r)?;

        // cooldowns: PetSpellCooldown[amount_of_cooldowns]
        let mut cooldowns = Vec::with_capacity(amount_of_cooldowns as usize);
        for i in 0..amount_of_cooldowns {
            cooldowns.push(PetSpellCooldown::read(r)?);
        }

        Ok(Self {
            pet,
            unknown1,
            react,
            command,
            unknown2,
            action_bars,
            amount_of_spells,
            spells,
            amount_of_cooldowns,
            cooldowns,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // pet: Guid
        self.pet.write(w)?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // react: PetReactState
        self.react.write(w)?;

        // command: PetCommandState
        self.command.write(w)?;

        // unknown2: u16
        w.write_all(&self.unknown2.to_le_bytes())?;

        // action_bars: u32[10]
        for i in self.action_bars.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // amount_of_spells: u8
        w.write_all(&(self.spells.len() as u8).to_le_bytes())?;

        // spells: u32[amount_of_spells]
        for i in self.spells.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // amount_of_cooldowns: u8
        w.write_all(&(self.cooldowns.len() as u8).to_le_bytes())?;

        // cooldowns: PetSpellCooldown[amount_of_cooldowns]
        for i in self.cooldowns.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_PET_SPELLS {
    fn size(&self) -> usize {
        8 // pet: Guid
        + 4 // unknown1: u32
        + PetReactState::size() // react: PetReactState
        + PetCommandState::size() // command: PetCommandState
        + 2 // unknown2: u16
        + 10 * core::mem::size_of::<u32>() // action_bars: u32[10]
        + 1 // amount_of_spells: u8
        + self.spells.len() * core::mem::size_of::<u32>() // spells: u32[amount_of_spells]
        + 1 // amount_of_cooldowns: u8
        + self.cooldowns.iter().fold(0, |acc, x| acc + PetSpellCooldown::size()) // cooldowns: PetSpellCooldown[amount_of_cooldowns]
    }
}

impl MaximumPossibleSized for SMSG_PET_SPELLS {
    fn maximum_possible_size() -> usize {
        8 // pet: Guid
        + 4 // unknown1: u32
        + PetReactState::maximum_possible_size() // react: PetReactState
        + PetCommandState::maximum_possible_size() // command: PetCommandState
        + 2 // unknown2: u16
        + 10 * core::mem::size_of::<u32>() // action_bars: u32[10]
        + 1 // amount_of_spells: u8
        + 255 * core::mem::size_of::<u32>() // spells: u32[amount_of_spells]
        + 1 // amount_of_cooldowns: u8
        + 255 * PetSpellCooldown::maximum_possible_size() // cooldowns: PetSpellCooldown[amount_of_cooldowns]
    }
}

#[derive(Debug)]
pub enum SMSG_PET_SPELLSError {
    Io(std::io::Error),
    PetCommandState(PetCommandStateError),
    PetReactState(PetReactStateError),
}

impl std::error::Error for SMSG_PET_SPELLSError {}
impl std::fmt::Display for SMSG_PET_SPELLSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PetCommandState(i) => i.fmt(f),
            Self::PetReactState(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PET_SPELLSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PetCommandStateError> for SMSG_PET_SPELLSError {
    fn from(e: PetCommandStateError) -> Self {
        Self::PetCommandState(e)
    }
}

impl From<PetReactStateError> for SMSG_PET_SPELLSError {
    fn from(e: PetReactStateError) -> Self {
        Self::PetReactState(e)
    }
}

