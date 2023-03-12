use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    CreatureFamily, PetCommandState, PetEnabled, PetReactState, PetSpellCooldown,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_spells.wowm:43`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_spells.wowm#L43):
/// ```text
/// smsg SMSG_PET_SPELLS = 0x0179 {
///     Guid pet;
///     optional action_bars {
///         (u16)CreatureFamily family;
///         u32 duration;
///         PetReactState react;
///         PetCommandState command;
///         u8 unknown;
///         PetEnabled pet_enabled;
///         u32[10] action_bars;
///         u8 amount_of_spells;
///         u32[amount_of_spells] spells;
///         u8 amount_of_cooldowns;
///         PetSpellCooldown[amount_of_cooldowns] cooldowns;
///     }
/// }
/// ```
pub struct SMSG_PET_SPELLS {
    pub pet: Guid,
    pub action_bars: Option<SMSG_PET_SPELLS_action_bars>,
}

impl crate::Message for SMSG_PET_SPELLS {
    const OPCODE: u32 = 0x0179;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // pet: Guid
        w.write_all(&self.pet.guid().to_le_bytes())?;

        // optional action_bars
        if let Some(v) = &self.action_bars {
            // family: CreatureFamily
            w.write_all(&u16::from(v.family.as_int()).to_le_bytes())?;

            // duration: u32
            w.write_all(&v.duration.to_le_bytes())?;

            // react: PetReactState
            w.write_all(&u8::from(v.react.as_int()).to_le_bytes())?;

            // command: PetCommandState
            w.write_all(&u8::from(v.command.as_int()).to_le_bytes())?;

            // unknown: u8
            w.write_all(&v.unknown.to_le_bytes())?;

            // pet_enabled: PetEnabled
            w.write_all(&u8::from(v.pet_enabled.as_int()).to_le_bytes())?;

            // action_bars: u32[10]
            for i in v.action_bars.iter() {
                w.write_all(&i.to_le_bytes())?;
            }

            // amount_of_spells: u8
            w.write_all(&(v.spells.len() as u8).to_le_bytes())?;

            // spells: u32[amount_of_spells]
            for i in v.spells.iter() {
                w.write_all(&i.to_le_bytes())?;
            }

            // amount_of_cooldowns: u8
            w.write_all(&(v.cooldowns.len() as u8).to_le_bytes())?;

            // cooldowns: PetSpellCooldown[amount_of_cooldowns]
            for i in v.cooldowns.iter() {
                i.write_into_vec(&mut w)?;
            }

        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(8..=4668).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0179, size: body_size as u32 });
        }

        // pet: Guid
        let pet = Guid::read(&mut r)?;

        // optional action_bars
        let current_size = {
            8 // pet: Guid
        };
        let action_bars = if current_size < body_size as usize {
            // family: CreatureFamily
            let family: CreatureFamily = (crate::util::read_u16_le(&mut r)? as u8).try_into()?;

            // duration: u32
            let duration = crate::util::read_u32_le(&mut r)?;

            // react: PetReactState
            let react: PetReactState = crate::util::read_u8_le(&mut r)?.try_into()?;

            // command: PetCommandState
            let command: PetCommandState = crate::util::read_u8_le(&mut r)?.try_into()?;

            // unknown: u8
            let unknown = crate::util::read_u8_le(&mut r)?;

            // pet_enabled: PetEnabled
            let pet_enabled: PetEnabled = crate::util::read_u8_le(&mut r)?.try_into()?;

            // action_bars: u32[10]
            let action_bars = {
                let mut action_bars = [u32::default(); 10];
                for i in action_bars.iter_mut() {
                    *i = crate::util::read_u32_le(&mut r)?;
                }
                action_bars
            };

            // amount_of_spells: u8
            let amount_of_spells = crate::util::read_u8_le(&mut r)?;

            // spells: u32[amount_of_spells]
            let spells = {
                let mut spells = Vec::with_capacity(amount_of_spells as usize);
                for i in 0..amount_of_spells {
                    spells.push(crate::util::read_u32_le(&mut r)?);
                }
                spells
            };

            // amount_of_cooldowns: u8
            let amount_of_cooldowns = crate::util::read_u8_le(&mut r)?;

            // cooldowns: PetSpellCooldown[amount_of_cooldowns]
            let cooldowns = {
                let mut cooldowns = Vec::with_capacity(amount_of_cooldowns as usize);
                for i in 0..amount_of_cooldowns {
                    cooldowns.push(PetSpellCooldown::read(&mut r)?);
                }
                cooldowns
            };

            Some(SMSG_PET_SPELLS_action_bars {
                family,
                duration,
                react,
                command,
                unknown,
                pet_enabled,
                action_bars,
                spells,
                cooldowns,
            })
        } else {
            None
        };

        Ok(Self {
            pet,
            action_bars,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PET_SPELLS {}

impl SMSG_PET_SPELLS {
    pub(crate) fn size(&self) -> usize {
        8 // pet: Guid
        + if let Some(action_bars) = &self.action_bars {
            2 // family: CreatureFamily
            + 4 // duration: u32
            + 1 // react: PetReactState
            + 1 // command: PetCommandState
            + 1 // unknown: u8
            + 1 // pet_enabled: PetEnabled
            + 10 * core::mem::size_of::<u32>() // action_bars: u32[10]
            + 1 // amount_of_spells: u8
            + action_bars.spells.len() * core::mem::size_of::<u32>() // spells: u32[amount_of_spells]
            + 1 // amount_of_cooldowns: u8
            + action_bars.cooldowns.len() * 14 // cooldowns: PetSpellCooldown[amount_of_cooldowns]
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PET_SPELLS_action_bars {
    pub family: CreatureFamily,
    pub duration: u32,
    pub react: PetReactState,
    pub command: PetCommandState,
    pub unknown: u8,
    pub pet_enabled: PetEnabled,
    pub action_bars: [u32; 10],
    pub spells: Vec<u32>,
    pub cooldowns: Vec<PetSpellCooldown>,
}

impl SMSG_PET_SPELLS_action_bars {
    pub(crate) fn size(&self) -> usize {
        2 // family: CreatureFamily
        + 4 // duration: u32
        + 1 // react: PetReactState
        + 1 // command: PetCommandState
        + 1 // unknown: u8
        + 1 // pet_enabled: PetEnabled
        + 10 * core::mem::size_of::<u32>() // action_bars: u32[10]
        + 1 // amount_of_spells: u8
        + self.spells.len() * core::mem::size_of::<u32>() // spells: u32[amount_of_spells]
        + 1 // amount_of_cooldowns: u8
        + self.cooldowns.len() * 14 // cooldowns: PetSpellCooldown[amount_of_cooldowns]
    }

}

