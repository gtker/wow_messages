use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::PetCommandState;
use crate::world::version_1_12::PetReactState;
use crate::world::version_1_12::PetSpellCooldown;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_spells.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_spells.wowm#L12):
/// ```text
/// smsg SMSG_PET_SPELLS = 0x0179 {
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
    /// # Comment
    ///
    /// mangoszero: set to 0
    pub unknown1: u32,
    pub react: PetReactState,
    pub command: PetCommandState,
    /// # Comment
    ///
    /// mangoszero: set to 0
    pub unknown2: u16,
    pub action_bars: [u32; 10],
    pub spells: Vec<u32>,
    pub cooldowns: Vec<PetSpellCooldown>,
}

impl ServerMessage for SMSG_PET_SPELLS {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // pet: Guid
        w.write_all(&self.pet.guid().to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // react: PetReactState
        w.write_all(&(self.react.as_int() as u8).to_le_bytes())?;

        // command: PetCommandState
        w.write_all(&(self.command.as_int() as u8).to_le_bytes())?;

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
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0179;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // pet: Guid
        let pet = Guid::read(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // react: PetReactState
        let react: PetReactState = crate::util::read_u8_le(r)?.try_into()?;

        // command: PetCommandState
        let command: PetCommandState = crate::util::read_u8_le(r)?.try_into()?;

        // unknown2: u16
        let unknown2 = crate::util::read_u16_le(r)?;

        // action_bars: u32[10]
        let mut action_bars = [u32::default(); 10];
        for i in action_bars.iter_mut() {
            *i = crate::util::read_u32_le(r)?;
        }

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
            spells,
            cooldowns,
        })
    }

}

impl SMSG_PET_SPELLS {
    pub(crate) fn size(&self) -> usize {
        8 // pet: Guid
        + 4 // unknown1: u32
        + 1 // react: PetReactState
        + 1 // command: PetCommandState
        + 2 // unknown2: u16
        + 10 * core::mem::size_of::<u32>() // action_bars: u32[10]
        + 1 // amount_of_spells: u8
        + self.spells.len() * core::mem::size_of::<u32>() // spells: u32[amount_of_spells]
        + 1 // amount_of_cooldowns: u8
        + self.cooldowns.len() * 12 // cooldowns: PetSpellCooldown[amount_of_cooldowns]
    }
}

