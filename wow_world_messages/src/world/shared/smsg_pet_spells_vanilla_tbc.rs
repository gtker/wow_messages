use std::io::{Read, Write};

use crate::Guid;
use crate::shared::pet_spell_cooldown_vanilla_tbc::PetSpellCooldown;
use std::time::Duration;
use wow_world_base::shared::pet_command_state_vanilla_tbc_wrath::PetCommandState;
use wow_world_base::shared::pet_enabled_vanilla_tbc_wrath::PetEnabled;
use wow_world_base::shared::pet_react_state_vanilla_tbc_wrath::PetReactState;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_spells.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_spells.wowm#L12):
/// ```text
/// smsg SMSG_PET_SPELLS = 0x0179 {
///     Guid pet;
///     optional action_bars {
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

impl crate::private::Sealed for SMSG_PET_SPELLS {}
impl SMSG_PET_SPELLS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(8..=4154).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // pet: Guid
        let pet = crate::util::read_guid(&mut r)?;

        // optional action_bars
        let current_size = {
            8 // pet: Guid
        };
        let action_bars = if current_size < body_size as usize {
            // duration: u32
            let duration = crate::util::read_u32_le(&mut r)?;

            // react: PetReactState
            let react = crate::util::read_u8_le(&mut r)?.try_into()?;

            // command: PetCommandState
            let command = crate::util::read_u8_le(&mut r)?.try_into()?;

            // unknown: u8
            let unknown = crate::util::read_u8_le(&mut r)?;

            // pet_enabled: PetEnabled
            let pet_enabled = crate::util::read_u8_le(&mut r)?.try_into()?;

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
                for _ in 0..amount_of_spells {
                    spells.push(crate::util::read_u32_le(&mut r)?);
                }
                spells
            };

            // amount_of_cooldowns: u8
            let amount_of_cooldowns = crate::util::read_u8_le(&mut r)?;

            // cooldowns: PetSpellCooldown[amount_of_cooldowns]
            let cooldowns = {
                let mut cooldowns = Vec::with_capacity(amount_of_cooldowns as usize);
                for _ in 0..amount_of_cooldowns {
                    cooldowns.push(PetSpellCooldown::read(&mut r)?);
                }
                cooldowns
            };

            Some(SMSG_PET_SPELLS_action_bars {
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

impl crate::Message for SMSG_PET_SPELLS {
    const OPCODE: u32 = 0x0179;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_PET_SPELLS"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PET_SPELLS {{").unwrap();
        // Members
        writeln!(s, "    pet = {};", self.pet.guid()).unwrap();
        if let Some(action_bars) = &self.action_bars {
            writeln!(s, "    duration = {};", action_bars.duration).unwrap();
            writeln!(s, "    react = {};", action_bars.react.as_test_case_value()).unwrap();
            writeln!(s, "    command = {};", action_bars.command.as_test_case_value()).unwrap();
            writeln!(s, "    unknown = {};", action_bars.unknown).unwrap();
            writeln!(s, "    pet_enabled = {};", action_bars.pet_enabled.as_test_case_value()).unwrap();
            write!(s, "    action_bars = [").unwrap();
            for v in action_bars.action_bars.as_slice() {
                write!(s, "{v:#04X}, ").unwrap();
            }
            writeln!(s, "];").unwrap();
            writeln!(s, "    amount_of_spells = {};", action_bars.spells.len()).unwrap();
            write!(s, "    spells = [").unwrap();
            for v in action_bars.spells.as_slice() {
                write!(s, "{v:#04X}, ").unwrap();
            }
            writeln!(s, "];").unwrap();
            writeln!(s, "    amount_of_cooldowns = {};", action_bars.cooldowns.len()).unwrap();
            write!(s, "    cooldowns = [").unwrap();
            for v in action_bars.cooldowns.as_slice() {
                writeln!(s, "{{").unwrap();
                // Members
                writeln!(s, "        spell = {};", v.spell).unwrap();
                writeln!(s, "        spell_category = {};", v.spell_category).unwrap();
                writeln!(s, "        cooldown = {};", v.cooldown.as_millis()).unwrap();
                writeln!(s, "        category_cooldown = {};", v.category_cooldown.as_millis()).unwrap();

                writeln!(s, "    }},").unwrap();
            }
            writeln!(s, "];").unwrap();
        }

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 377_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "pet", "    ");
        if let Some(action_bars) = &self.action_bars {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 1, "react", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 1, "command", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 1, "pet_enabled", "    ");
            writeln!(s, "    /* action_bars: u32[10] start */").unwrap();
            for (i, v) in action_bars.action_bars.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("action_bars {i}"), "    ");
            }
            writeln!(s, "    /* action_bars: u32[10] end */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_spells", "    ");
            if !action_bars.spells.is_empty() {
                writeln!(s, "    /* spells: u32[amount_of_spells] start */").unwrap();
                for (i, v) in action_bars.spells.iter().enumerate() {
                    crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("spells {i}"), "    ");
                }
                writeln!(s, "    /* spells: u32[amount_of_spells] end */").unwrap();
            }
            crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_cooldowns", "    ");
            if !action_bars.cooldowns.is_empty() {
                writeln!(s, "    /* cooldowns: PetSpellCooldown[amount_of_cooldowns] start */").unwrap();
                for (i, v) in action_bars.cooldowns.iter().enumerate() {
                    writeln!(s, "    /* cooldowns: PetSpellCooldown[amount_of_cooldowns] {i} start */").unwrap();
                    crate::util::write_bytes(&mut s, &mut bytes, 2, "spell", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 2, "spell_category", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "cooldown", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "category_cooldown", "        ");
                    writeln!(s, "    /* cooldowns: PetSpellCooldown[amount_of_cooldowns] {i} end */").unwrap();
                }
                writeln!(s, "    /* cooldowns: PetSpellCooldown[amount_of_cooldowns] end */").unwrap();
            }
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // pet: Guid
        w.write_all(&self.pet.guid().to_le_bytes())?;

        // optional action_bars
        if let Some(v) = &self.action_bars {
            // duration: u32
            w.write_all(&v.duration.to_le_bytes())?;

            // react: PetReactState
            w.write_all(&(v.react.as_int().to_le_bytes()))?;

            // command: PetCommandState
            w.write_all(&(v.command.as_int().to_le_bytes()))?;

            // unknown: u8
            w.write_all(&v.unknown.to_le_bytes())?;

            // pet_enabled: PetEnabled
            w.write_all(&(v.pet_enabled.as_int().to_le_bytes()))?;

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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(377, "SMSG_PET_SPELLS", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PET_SPELLS {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PET_SPELLS {}

impl SMSG_PET_SPELLS {
    pub(crate) fn size(&self) -> usize {
        8 // pet: Guid
        + if let Some(action_bars) = &self.action_bars {
            4 // duration: u32
            + 1 // react: PetReactState
            + 1 // command: PetCommandState
            + 1 // unknown: u8
            + 1 // pet_enabled: PetEnabled
            + 40 // action_bars: u32[10]
            + 1 // amount_of_spells: u8
            + action_bars.spells.len() * core::mem::size_of::<u32>() // spells: u32[amount_of_spells]
            + 1 // amount_of_cooldowns: u8
            + action_bars.cooldowns.len() * 12 // cooldowns: PetSpellCooldown[amount_of_cooldowns]
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PET_SPELLS_action_bars {
    pub duration: u32,
    pub react: PetReactState,
    pub command: PetCommandState,
    pub unknown: u8,
    pub pet_enabled: PetEnabled,
    pub action_bars: [u32; 10],
    pub spells: Vec<u32>,
    pub cooldowns: Vec<PetSpellCooldown>,
}

