use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{AuraType, AuraTypeError};
use crate::world::v1::v12::{SpellSchool, SpellSchoolError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct AuraLog {
    pub aura_type: AuraLogAuraType,
}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for AuraLog {
    type Error = AuraLogError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // aura_type: AuraType
        let aura_type = AuraType::read(r)?;

        let aura_type_if = match aura_type {
            AuraType::NONE => AuraLogAuraType::NONE,
            AuraType::BIND_SIGHT => AuraLogAuraType::BIND_SIGHT,
            AuraType::MOD_POSSESS => AuraLogAuraType::MOD_POSSESS,
            AuraType::PERIODIC_DAMAGE => {
                // damage1: u32
                let damage1 = crate::util::read_u32_le(r)?;

                // school: SpellSchool
                let school = SpellSchool::read(r)?;

                // absorbed: u32
                let absorbed = crate::util::read_u32_le(r)?;

                // resisted: u32
                let resisted = crate::util::read_u32_le(r)?;

                AuraLogAuraType::PERIODIC_DAMAGE {
                    damage1,
                    school,
                    absorbed,
                    resisted,
                }
            }
            AuraType::DUMMY => AuraLogAuraType::DUMMY,
            AuraType::MOD_CONFUSE => AuraLogAuraType::MOD_CONFUSE,
            AuraType::MOD_CHARM => AuraLogAuraType::MOD_CHARM,
            AuraType::MOD_FEAR => AuraLogAuraType::MOD_FEAR,
            AuraType::PERIODIC_HEAL => {
                // damage2: u32
                let damage2 = crate::util::read_u32_le(r)?;

                AuraLogAuraType::PERIODIC_HEAL {
                    damage2,
                }
            }
            AuraType::MOD_ATTACKSPEED => AuraLogAuraType::MOD_ATTACKSPEED,
            AuraType::MOD_THREAT => AuraLogAuraType::MOD_THREAT,
            AuraType::MOD_TAUNT => AuraLogAuraType::MOD_TAUNT,
            AuraType::MOD_STUN => AuraLogAuraType::MOD_STUN,
            AuraType::MOD_DAMAGE_DONE => AuraLogAuraType::MOD_DAMAGE_DONE,
            AuraType::MOD_DAMAGE_TAKEN => AuraLogAuraType::MOD_DAMAGE_TAKEN,
            AuraType::DAMAGE_SHIELD => AuraLogAuraType::DAMAGE_SHIELD,
            AuraType::MOD_STEALTH => AuraLogAuraType::MOD_STEALTH,
            AuraType::MOD_STEALTH_DETECT => AuraLogAuraType::MOD_STEALTH_DETECT,
            AuraType::MOD_INVISIBILITY => AuraLogAuraType::MOD_INVISIBILITY,
            AuraType::MOD_INVISIBILITY_DETECTION => AuraLogAuraType::MOD_INVISIBILITY_DETECTION,
            AuraType::OBS_MOD_HEALTH => {
                // damage2: u32
                let damage2 = crate::util::read_u32_le(r)?;

                AuraLogAuraType::OBS_MOD_HEALTH {
                    damage2,
                }
            }
            AuraType::OBS_MOD_MANA => {
                // misc_value1: u32
                let misc_value1 = crate::util::read_u32_le(r)?;

                // damage3: u32
                let damage3 = crate::util::read_u32_le(r)?;

                AuraLogAuraType::OBS_MOD_MANA {
                    misc_value1,
                    damage3,
                }
            }
            AuraType::MOD_RESISTANCE => AuraLogAuraType::MOD_RESISTANCE,
            AuraType::PERIODIC_TRIGGER_SPELL => AuraLogAuraType::PERIODIC_TRIGGER_SPELL,
            AuraType::PERIODIC_ENERGIZE => {
                // misc_value1: u32
                let misc_value1 = crate::util::read_u32_le(r)?;

                // damage3: u32
                let damage3 = crate::util::read_u32_le(r)?;

                AuraLogAuraType::PERIODIC_ENERGIZE {
                    misc_value1,
                    damage3,
                }
            }
            AuraType::MOD_PACIFY => AuraLogAuraType::MOD_PACIFY,
            AuraType::MOD_ROOT => AuraLogAuraType::MOD_ROOT,
            AuraType::MOD_SILENCE => AuraLogAuraType::MOD_SILENCE,
            AuraType::REFLECT_SPELLS => AuraLogAuraType::REFLECT_SPELLS,
            AuraType::MOD_STAT => AuraLogAuraType::MOD_STAT,
            AuraType::MOD_SKILL => AuraLogAuraType::MOD_SKILL,
            AuraType::MOD_INCREASE_SPEED => AuraLogAuraType::MOD_INCREASE_SPEED,
            AuraType::MOD_INCREASE_MOUNTED_SPEED => AuraLogAuraType::MOD_INCREASE_MOUNTED_SPEED,
            AuraType::MOD_DECREASE_SPEED => AuraLogAuraType::MOD_DECREASE_SPEED,
            AuraType::MOD_INCREASE_HEALTH => AuraLogAuraType::MOD_INCREASE_HEALTH,
            AuraType::MOD_INCREASE_ENERGY => AuraLogAuraType::MOD_INCREASE_ENERGY,
            AuraType::MOD_SHAPESHIFT => AuraLogAuraType::MOD_SHAPESHIFT,
            AuraType::EFFECT_IMMUNITY => AuraLogAuraType::EFFECT_IMMUNITY,
            AuraType::STATE_IMMUNITY => AuraLogAuraType::STATE_IMMUNITY,
            AuraType::SCHOOL_IMMUNITY => AuraLogAuraType::SCHOOL_IMMUNITY,
            AuraType::DAMAGE_IMMUNITY => AuraLogAuraType::DAMAGE_IMMUNITY,
            AuraType::DISPEL_IMMUNITY => AuraLogAuraType::DISPEL_IMMUNITY,
            AuraType::PROC_TRIGGER_SPELL => AuraLogAuraType::PROC_TRIGGER_SPELL,
            AuraType::PROC_TRIGGER_DAMAGE => AuraLogAuraType::PROC_TRIGGER_DAMAGE,
            AuraType::TRACK_CREATURES => AuraLogAuraType::TRACK_CREATURES,
            AuraType::TRACK_RESOURCES => AuraLogAuraType::TRACK_RESOURCES,
            AuraType::UNKNOWN46 => AuraLogAuraType::UNKNOWN46,
            AuraType::MOD_PARRY_PERCENT => AuraLogAuraType::MOD_PARRY_PERCENT,
            AuraType::UNKNOWN48 => AuraLogAuraType::UNKNOWN48,
            AuraType::MOD_DODGE_PERCENT => AuraLogAuraType::MOD_DODGE_PERCENT,
            AuraType::MOD_BLOCK_SKILL => AuraLogAuraType::MOD_BLOCK_SKILL,
            AuraType::MOD_BLOCK_PERCENT => AuraLogAuraType::MOD_BLOCK_PERCENT,
            AuraType::MOD_CRIT_PERCENT => AuraLogAuraType::MOD_CRIT_PERCENT,
            AuraType::PERIODIC_LEECH => AuraLogAuraType::PERIODIC_LEECH,
            AuraType::MOD_HIT_CHANCE => AuraLogAuraType::MOD_HIT_CHANCE,
            AuraType::MOD_SPELL_HIT_CHANCE => AuraLogAuraType::MOD_SPELL_HIT_CHANCE,
            AuraType::TRANSFORM => AuraLogAuraType::TRANSFORM,
            AuraType::MOD_SPELL_CRIT_CHANCE => AuraLogAuraType::MOD_SPELL_CRIT_CHANCE,
            AuraType::MOD_INCREASE_SWIM_SPEED => AuraLogAuraType::MOD_INCREASE_SWIM_SPEED,
            AuraType::MOD_DAMAGE_DONE_CREATURE => AuraLogAuraType::MOD_DAMAGE_DONE_CREATURE,
            AuraType::MOD_PACIFY_SILENCE => AuraLogAuraType::MOD_PACIFY_SILENCE,
            AuraType::MOD_SCALE => AuraLogAuraType::MOD_SCALE,
            AuraType::PERIODIC_HEALTH_FUNNEL => AuraLogAuraType::PERIODIC_HEALTH_FUNNEL,
            AuraType::PERIODIC_MANA_FUNNEL => AuraLogAuraType::PERIODIC_MANA_FUNNEL,
            AuraType::PERIODIC_MANA_LEECH => {
                // misc_value2: u32
                let misc_value2 = crate::util::read_u32_le(r)?;

                // damage: u32
                let damage = crate::util::read_u32_le(r)?;

                // gain_multiplier: f32
                let gain_multiplier = crate::util::read_f32_le(r)?;
                AuraLogAuraType::PERIODIC_MANA_LEECH {
                    misc_value2,
                    damage,
                    gain_multiplier,
                }
            }
            AuraType::MOD_CASTING_SPEED_NOT_STACK => AuraLogAuraType::MOD_CASTING_SPEED_NOT_STACK,
            AuraType::FEIGN_DEATH => AuraLogAuraType::FEIGN_DEATH,
            AuraType::MOD_DISARM => AuraLogAuraType::MOD_DISARM,
            AuraType::MOD_STALKED => AuraLogAuraType::MOD_STALKED,
            AuraType::SCHOOL_ABSORB => AuraLogAuraType::SCHOOL_ABSORB,
            AuraType::EXTRA_ATTACKS => AuraLogAuraType::EXTRA_ATTACKS,
            AuraType::MOD_SPELL_CRIT_CHANCE_SCHOOL => AuraLogAuraType::MOD_SPELL_CRIT_CHANCE_SCHOOL,
            AuraType::MOD_POWER_COST_SCHOOL_PCT => AuraLogAuraType::MOD_POWER_COST_SCHOOL_PCT,
            AuraType::MOD_POWER_COST_SCHOOL => AuraLogAuraType::MOD_POWER_COST_SCHOOL,
            AuraType::REFLECT_SPELLS_SCHOOL => AuraLogAuraType::REFLECT_SPELLS_SCHOOL,
            AuraType::MOD_LANGUAGE => AuraLogAuraType::MOD_LANGUAGE,
            AuraType::FAR_SIGHT => AuraLogAuraType::FAR_SIGHT,
            AuraType::MECHANIC_IMMUNITY => AuraLogAuraType::MECHANIC_IMMUNITY,
            AuraType::MOUNTED => AuraLogAuraType::MOUNTED,
            AuraType::MOD_DAMAGE_PERCENT_DONE => AuraLogAuraType::MOD_DAMAGE_PERCENT_DONE,
            AuraType::MOD_PERCENT_STAT => AuraLogAuraType::MOD_PERCENT_STAT,
            AuraType::SPLIT_DAMAGE_PCT => AuraLogAuraType::SPLIT_DAMAGE_PCT,
            AuraType::WATER_BREATHING => AuraLogAuraType::WATER_BREATHING,
            AuraType::MOD_BASE_RESISTANCE => AuraLogAuraType::MOD_BASE_RESISTANCE,
            AuraType::MOD_REGEN => AuraLogAuraType::MOD_REGEN,
            AuraType::MOD_POWER_REGEN => AuraLogAuraType::MOD_POWER_REGEN,
            AuraType::CHANNEL_DEATH_ITEM => AuraLogAuraType::CHANNEL_DEATH_ITEM,
            AuraType::MOD_DAMAGE_PERCENT_TAKEN => AuraLogAuraType::MOD_DAMAGE_PERCENT_TAKEN,
            AuraType::MOD_HEALTH_REGEN_PERCENT => AuraLogAuraType::MOD_HEALTH_REGEN_PERCENT,
            AuraType::PERIODIC_DAMAGE_PERCENT => {
                // damage1: u32
                let damage1 = crate::util::read_u32_le(r)?;

                // school: SpellSchool
                let school = SpellSchool::read(r)?;

                // absorbed: u32
                let absorbed = crate::util::read_u32_le(r)?;

                // resisted: u32
                let resisted = crate::util::read_u32_le(r)?;

                AuraLogAuraType::PERIODIC_DAMAGE_PERCENT {
                    damage1,
                    school,
                    absorbed,
                    resisted,
                }
            }
            AuraType::MOD_RESIST_CHANCE => AuraLogAuraType::MOD_RESIST_CHANCE,
            AuraType::MOD_DETECT_RANGE => AuraLogAuraType::MOD_DETECT_RANGE,
            AuraType::PREVENTS_FLEEING => AuraLogAuraType::PREVENTS_FLEEING,
            AuraType::MOD_UNATTACKABLE => AuraLogAuraType::MOD_UNATTACKABLE,
            AuraType::INTERRUPT_REGEN => AuraLogAuraType::INTERRUPT_REGEN,
            AuraType::GHOST => AuraLogAuraType::GHOST,
            AuraType::SPELL_MAGNET => AuraLogAuraType::SPELL_MAGNET,
            AuraType::MANA_SHIELD => AuraLogAuraType::MANA_SHIELD,
            AuraType::MOD_SKILL_TALENT => AuraLogAuraType::MOD_SKILL_TALENT,
            AuraType::MOD_ATTACK_POWER => AuraLogAuraType::MOD_ATTACK_POWER,
            AuraType::AURAS_VISIBLE => AuraLogAuraType::AURAS_VISIBLE,
            AuraType::MOD_RESISTANCE_PCT => AuraLogAuraType::MOD_RESISTANCE_PCT,
            AuraType::MOD_MELEE_ATTACK_POWER_VERSUS => AuraLogAuraType::MOD_MELEE_ATTACK_POWER_VERSUS,
            AuraType::MOD_TOTAL_THREAT => AuraLogAuraType::MOD_TOTAL_THREAT,
            AuraType::WATER_WALK => AuraLogAuraType::WATER_WALK,
            AuraType::FEATHER_FALL => AuraLogAuraType::FEATHER_FALL,
            AuraType::HOVER => AuraLogAuraType::HOVER,
            AuraType::ADD_FLAT_MODIFIER => AuraLogAuraType::ADD_FLAT_MODIFIER,
            AuraType::ADD_PCT_MODIFIER => AuraLogAuraType::ADD_PCT_MODIFIER,
            AuraType::ADD_TARGET_TRIGGER => AuraLogAuraType::ADD_TARGET_TRIGGER,
            AuraType::MOD_POWER_REGEN_PERCENT => AuraLogAuraType::MOD_POWER_REGEN_PERCENT,
            AuraType::ADD_CASTER_HIT_TRIGGER => AuraLogAuraType::ADD_CASTER_HIT_TRIGGER,
            AuraType::OVERRIDE_CLASS_SCRIPTS => AuraLogAuraType::OVERRIDE_CLASS_SCRIPTS,
            AuraType::MOD_RANGED_DAMAGE_TAKEN => AuraLogAuraType::MOD_RANGED_DAMAGE_TAKEN,
            AuraType::MOD_RANGED_DAMAGE_TAKEN_PCT => AuraLogAuraType::MOD_RANGED_DAMAGE_TAKEN_PCT,
            AuraType::MOD_HEALING => AuraLogAuraType::MOD_HEALING,
            AuraType::MOD_REGEN_DURING_COMBAT => AuraLogAuraType::MOD_REGEN_DURING_COMBAT,
            AuraType::MOD_MECHANIC_RESISTANCE => AuraLogAuraType::MOD_MECHANIC_RESISTANCE,
            AuraType::MOD_HEALING_PCT => AuraLogAuraType::MOD_HEALING_PCT,
            AuraType::SHARE_PET_TRACKING => AuraLogAuraType::SHARE_PET_TRACKING,
            AuraType::UNTRACKABLE => AuraLogAuraType::UNTRACKABLE,
            AuraType::EMPATHY => AuraLogAuraType::EMPATHY,
            AuraType::MOD_OFFHAND_DAMAGE_PCT => AuraLogAuraType::MOD_OFFHAND_DAMAGE_PCT,
            AuraType::MOD_TARGET_RESISTANCE => AuraLogAuraType::MOD_TARGET_RESISTANCE,
            AuraType::MOD_RANGED_ATTACK_POWER => AuraLogAuraType::MOD_RANGED_ATTACK_POWER,
            AuraType::MOD_MELEE_DAMAGE_TAKEN => AuraLogAuraType::MOD_MELEE_DAMAGE_TAKEN,
            AuraType::MOD_MELEE_DAMAGE_TAKEN_PCT => AuraLogAuraType::MOD_MELEE_DAMAGE_TAKEN_PCT,
            AuraType::RANGED_ATTACK_POWER_ATTACKER_BONUS => AuraLogAuraType::RANGED_ATTACK_POWER_ATTACKER_BONUS,
            AuraType::MOD_POSSESS_PET => AuraLogAuraType::MOD_POSSESS_PET,
            AuraType::MOD_SPEED_ALWAYS => AuraLogAuraType::MOD_SPEED_ALWAYS,
            AuraType::MOD_MOUNTED_SPEED_ALWAYS => AuraLogAuraType::MOD_MOUNTED_SPEED_ALWAYS,
            AuraType::MOD_RANGED_ATTACK_POWER_VERSUS => AuraLogAuraType::MOD_RANGED_ATTACK_POWER_VERSUS,
            AuraType::MOD_INCREASE_ENERGY_PERCENT => AuraLogAuraType::MOD_INCREASE_ENERGY_PERCENT,
            AuraType::MOD_INCREASE_HEALTH_PERCENT => AuraLogAuraType::MOD_INCREASE_HEALTH_PERCENT,
            AuraType::MOD_MANA_REGEN_INTERRUPT => AuraLogAuraType::MOD_MANA_REGEN_INTERRUPT,
            AuraType::MOD_HEALING_DONE => AuraLogAuraType::MOD_HEALING_DONE,
            AuraType::MOD_HEALING_DONE_PERCENT => AuraLogAuraType::MOD_HEALING_DONE_PERCENT,
            AuraType::MOD_TOTAL_STAT_PERCENTAGE => AuraLogAuraType::MOD_TOTAL_STAT_PERCENTAGE,
            AuraType::MOD_MELEE_HASTE => AuraLogAuraType::MOD_MELEE_HASTE,
            AuraType::FORCE_REACTION => AuraLogAuraType::FORCE_REACTION,
            AuraType::MOD_RANGED_HASTE => AuraLogAuraType::MOD_RANGED_HASTE,
            AuraType::MOD_RANGED_AMMO_HASTE => AuraLogAuraType::MOD_RANGED_AMMO_HASTE,
            AuraType::MOD_BASE_RESISTANCE_PCT => AuraLogAuraType::MOD_BASE_RESISTANCE_PCT,
            AuraType::MOD_RESISTANCE_EXCLUSIVE => AuraLogAuraType::MOD_RESISTANCE_EXCLUSIVE,
            AuraType::SAFE_FALL => AuraLogAuraType::SAFE_FALL,
            AuraType::CHARISMA => AuraLogAuraType::CHARISMA,
            AuraType::PERSUADED => AuraLogAuraType::PERSUADED,
            AuraType::MECHANIC_IMMUNITY_MASK => AuraLogAuraType::MECHANIC_IMMUNITY_MASK,
            AuraType::RETAIN_COMBO_POINTS => AuraLogAuraType::RETAIN_COMBO_POINTS,
            AuraType::RESIST_PUSHBACK => AuraLogAuraType::RESIST_PUSHBACK,
            AuraType::MOD_SHIELD_BLOCKVALUE_PCT => AuraLogAuraType::MOD_SHIELD_BLOCKVALUE_PCT,
            AuraType::TRACK_STEALTHED => AuraLogAuraType::TRACK_STEALTHED,
            AuraType::MOD_DETECTED_RANGE => AuraLogAuraType::MOD_DETECTED_RANGE,
            AuraType::SPLIT_DAMAGE_FLAT => AuraLogAuraType::SPLIT_DAMAGE_FLAT,
            AuraType::MOD_STEALTH_LEVEL => AuraLogAuraType::MOD_STEALTH_LEVEL,
            AuraType::MOD_WATER_BREATHING => AuraLogAuraType::MOD_WATER_BREATHING,
            AuraType::MOD_REPUTATION_GAIN => AuraLogAuraType::MOD_REPUTATION_GAIN,
            AuraType::PET_DAMAGE_MULTI => AuraLogAuraType::PET_DAMAGE_MULTI,
            AuraType::MOD_SHIELD_BLOCKVALUE => AuraLogAuraType::MOD_SHIELD_BLOCKVALUE,
            AuraType::NO_PVP_CREDIT => AuraLogAuraType::NO_PVP_CREDIT,
            AuraType::MOD_AOE_AVOIDANCE => AuraLogAuraType::MOD_AOE_AVOIDANCE,
            AuraType::MOD_HEALTH_REGEN_IN_COMBAT => AuraLogAuraType::MOD_HEALTH_REGEN_IN_COMBAT,
            AuraType::POWER_BURN_MANA => AuraLogAuraType::POWER_BURN_MANA,
            AuraType::MOD_CRIT_DAMAGE_BONUS => AuraLogAuraType::MOD_CRIT_DAMAGE_BONUS,
            AuraType::UNKNOWN164 => AuraLogAuraType::UNKNOWN164,
            AuraType::MELEE_ATTACK_POWER_ATTACKER_BONUS => AuraLogAuraType::MELEE_ATTACK_POWER_ATTACKER_BONUS,
            AuraType::MOD_ATTACK_POWER_PCT => AuraLogAuraType::MOD_ATTACK_POWER_PCT,
            AuraType::MOD_RANGED_ATTACK_POWER_PCT => AuraLogAuraType::MOD_RANGED_ATTACK_POWER_PCT,
            AuraType::MOD_DAMAGE_DONE_VERSUS => AuraLogAuraType::MOD_DAMAGE_DONE_VERSUS,
            AuraType::MOD_CRIT_PERCENT_VERSUS => AuraLogAuraType::MOD_CRIT_PERCENT_VERSUS,
            AuraType::DETECT_AMORE => AuraLogAuraType::DETECT_AMORE,
            AuraType::MOD_SPEED_NOT_STACK => AuraLogAuraType::MOD_SPEED_NOT_STACK,
            AuraType::MOD_MOUNTED_SPEED_NOT_STACK => AuraLogAuraType::MOD_MOUNTED_SPEED_NOT_STACK,
            AuraType::ALLOW_CHAMPION_SPELLS => AuraLogAuraType::ALLOW_CHAMPION_SPELLS,
            AuraType::MOD_SPELL_DAMAGE_OF_STAT_PERCENT => AuraLogAuraType::MOD_SPELL_DAMAGE_OF_STAT_PERCENT,
            AuraType::MOD_SPELL_HEALING_OF_STAT_PERCENT => AuraLogAuraType::MOD_SPELL_HEALING_OF_STAT_PERCENT,
            AuraType::SPIRIT_OF_REDEMPTION => AuraLogAuraType::SPIRIT_OF_REDEMPTION,
            AuraType::AOE_CHARM => AuraLogAuraType::AOE_CHARM,
            AuraType::MOD_DEBUFF_RESISTANCE => AuraLogAuraType::MOD_DEBUFF_RESISTANCE,
            AuraType::MOD_ATTACKER_SPELL_CRIT_CHANCE => AuraLogAuraType::MOD_ATTACKER_SPELL_CRIT_CHANCE,
            AuraType::MOD_FLAT_SPELL_DAMAGE_VERSUS => AuraLogAuraType::MOD_FLAT_SPELL_DAMAGE_VERSUS,
            AuraType::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS => AuraLogAuraType::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS,
            AuraType::MOD_RESISTANCE_OF_STAT_PERCENT => AuraLogAuraType::MOD_RESISTANCE_OF_STAT_PERCENT,
            AuraType::MOD_CRITICAL_THREAT => AuraLogAuraType::MOD_CRITICAL_THREAT,
            AuraType::MOD_ATTACKER_MELEE_HIT_CHANCE => AuraLogAuraType::MOD_ATTACKER_MELEE_HIT_CHANCE,
            AuraType::MOD_ATTACKER_RANGED_HIT_CHANCE => AuraLogAuraType::MOD_ATTACKER_RANGED_HIT_CHANCE,
            AuraType::MOD_ATTACKER_SPELL_HIT_CHANCE => AuraLogAuraType::MOD_ATTACKER_SPELL_HIT_CHANCE,
            AuraType::MOD_ATTACKER_MELEE_CRIT_CHANCE => AuraLogAuraType::MOD_ATTACKER_MELEE_CRIT_CHANCE,
            AuraType::MOD_ATTACKER_RANGED_CRIT_CHANCE => AuraLogAuraType::MOD_ATTACKER_RANGED_CRIT_CHANCE,
            AuraType::MOD_RATING => AuraLogAuraType::MOD_RATING,
            AuraType::MOD_FACTION_REPUTATION_GAIN => AuraLogAuraType::MOD_FACTION_REPUTATION_GAIN,
            AuraType::USE_NORMAL_MOVEMENT_SPEED => AuraLogAuraType::USE_NORMAL_MOVEMENT_SPEED,
        };

        Ok(Self {
            aura_type: aura_type_if,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // aura_type: AuraType
        self.aura_type.write(w)?;

        match &self.aura_type {
            AuraLogAuraType::NONE => {}
            AuraLogAuraType::BIND_SIGHT => {}
            AuraLogAuraType::MOD_POSSESS => {}
            AuraLogAuraType::PERIODIC_DAMAGE {
                damage1,
                school,
                absorbed,
                resisted,
            } => {
                // damage1: u32
                w.write_all(&damage1.to_le_bytes())?;

                // school: SpellSchool
                school.write(w)?;

                // absorbed: u32
                w.write_all(&absorbed.to_le_bytes())?;

                // resisted: u32
                w.write_all(&resisted.to_le_bytes())?;

            }
            AuraLogAuraType::DUMMY => {}
            AuraLogAuraType::MOD_CONFUSE => {}
            AuraLogAuraType::MOD_CHARM => {}
            AuraLogAuraType::MOD_FEAR => {}
            AuraLogAuraType::PERIODIC_HEAL {
                damage2,
            } => {
                // damage2: u32
                w.write_all(&damage2.to_le_bytes())?;

            }
            AuraLogAuraType::MOD_ATTACKSPEED => {}
            AuraLogAuraType::MOD_THREAT => {}
            AuraLogAuraType::MOD_TAUNT => {}
            AuraLogAuraType::MOD_STUN => {}
            AuraLogAuraType::MOD_DAMAGE_DONE => {}
            AuraLogAuraType::MOD_DAMAGE_TAKEN => {}
            AuraLogAuraType::DAMAGE_SHIELD => {}
            AuraLogAuraType::MOD_STEALTH => {}
            AuraLogAuraType::MOD_STEALTH_DETECT => {}
            AuraLogAuraType::MOD_INVISIBILITY => {}
            AuraLogAuraType::MOD_INVISIBILITY_DETECTION => {}
            AuraLogAuraType::OBS_MOD_HEALTH {
                damage2,
            } => {
                // damage2: u32
                w.write_all(&damage2.to_le_bytes())?;

            }
            AuraLogAuraType::OBS_MOD_MANA {
                misc_value1,
                damage3,
            } => {
                // misc_value1: u32
                w.write_all(&misc_value1.to_le_bytes())?;

                // damage3: u32
                w.write_all(&damage3.to_le_bytes())?;

            }
            AuraLogAuraType::MOD_RESISTANCE => {}
            AuraLogAuraType::PERIODIC_TRIGGER_SPELL => {}
            AuraLogAuraType::PERIODIC_ENERGIZE {
                misc_value1,
                damage3,
            } => {
                // misc_value1: u32
                w.write_all(&misc_value1.to_le_bytes())?;

                // damage3: u32
                w.write_all(&damage3.to_le_bytes())?;

            }
            AuraLogAuraType::MOD_PACIFY => {}
            AuraLogAuraType::MOD_ROOT => {}
            AuraLogAuraType::MOD_SILENCE => {}
            AuraLogAuraType::REFLECT_SPELLS => {}
            AuraLogAuraType::MOD_STAT => {}
            AuraLogAuraType::MOD_SKILL => {}
            AuraLogAuraType::MOD_INCREASE_SPEED => {}
            AuraLogAuraType::MOD_INCREASE_MOUNTED_SPEED => {}
            AuraLogAuraType::MOD_DECREASE_SPEED => {}
            AuraLogAuraType::MOD_INCREASE_HEALTH => {}
            AuraLogAuraType::MOD_INCREASE_ENERGY => {}
            AuraLogAuraType::MOD_SHAPESHIFT => {}
            AuraLogAuraType::EFFECT_IMMUNITY => {}
            AuraLogAuraType::STATE_IMMUNITY => {}
            AuraLogAuraType::SCHOOL_IMMUNITY => {}
            AuraLogAuraType::DAMAGE_IMMUNITY => {}
            AuraLogAuraType::DISPEL_IMMUNITY => {}
            AuraLogAuraType::PROC_TRIGGER_SPELL => {}
            AuraLogAuraType::PROC_TRIGGER_DAMAGE => {}
            AuraLogAuraType::TRACK_CREATURES => {}
            AuraLogAuraType::TRACK_RESOURCES => {}
            AuraLogAuraType::UNKNOWN46 => {}
            AuraLogAuraType::MOD_PARRY_PERCENT => {}
            AuraLogAuraType::UNKNOWN48 => {}
            AuraLogAuraType::MOD_DODGE_PERCENT => {}
            AuraLogAuraType::MOD_BLOCK_SKILL => {}
            AuraLogAuraType::MOD_BLOCK_PERCENT => {}
            AuraLogAuraType::MOD_CRIT_PERCENT => {}
            AuraLogAuraType::PERIODIC_LEECH => {}
            AuraLogAuraType::MOD_HIT_CHANCE => {}
            AuraLogAuraType::MOD_SPELL_HIT_CHANCE => {}
            AuraLogAuraType::TRANSFORM => {}
            AuraLogAuraType::MOD_SPELL_CRIT_CHANCE => {}
            AuraLogAuraType::MOD_INCREASE_SWIM_SPEED => {}
            AuraLogAuraType::MOD_DAMAGE_DONE_CREATURE => {}
            AuraLogAuraType::MOD_PACIFY_SILENCE => {}
            AuraLogAuraType::MOD_SCALE => {}
            AuraLogAuraType::PERIODIC_HEALTH_FUNNEL => {}
            AuraLogAuraType::PERIODIC_MANA_FUNNEL => {}
            AuraLogAuraType::PERIODIC_MANA_LEECH {
                misc_value2,
                damage,
                gain_multiplier,
            } => {
                // misc_value2: u32
                w.write_all(&misc_value2.to_le_bytes())?;

                // damage: u32
                w.write_all(&damage.to_le_bytes())?;

                // gain_multiplier: f32
                w.write_all(&gain_multiplier.to_le_bytes())?;

            }
            AuraLogAuraType::MOD_CASTING_SPEED_NOT_STACK => {}
            AuraLogAuraType::FEIGN_DEATH => {}
            AuraLogAuraType::MOD_DISARM => {}
            AuraLogAuraType::MOD_STALKED => {}
            AuraLogAuraType::SCHOOL_ABSORB => {}
            AuraLogAuraType::EXTRA_ATTACKS => {}
            AuraLogAuraType::MOD_SPELL_CRIT_CHANCE_SCHOOL => {}
            AuraLogAuraType::MOD_POWER_COST_SCHOOL_PCT => {}
            AuraLogAuraType::MOD_POWER_COST_SCHOOL => {}
            AuraLogAuraType::REFLECT_SPELLS_SCHOOL => {}
            AuraLogAuraType::MOD_LANGUAGE => {}
            AuraLogAuraType::FAR_SIGHT => {}
            AuraLogAuraType::MECHANIC_IMMUNITY => {}
            AuraLogAuraType::MOUNTED => {}
            AuraLogAuraType::MOD_DAMAGE_PERCENT_DONE => {}
            AuraLogAuraType::MOD_PERCENT_STAT => {}
            AuraLogAuraType::SPLIT_DAMAGE_PCT => {}
            AuraLogAuraType::WATER_BREATHING => {}
            AuraLogAuraType::MOD_BASE_RESISTANCE => {}
            AuraLogAuraType::MOD_REGEN => {}
            AuraLogAuraType::MOD_POWER_REGEN => {}
            AuraLogAuraType::CHANNEL_DEATH_ITEM => {}
            AuraLogAuraType::MOD_DAMAGE_PERCENT_TAKEN => {}
            AuraLogAuraType::MOD_HEALTH_REGEN_PERCENT => {}
            AuraLogAuraType::PERIODIC_DAMAGE_PERCENT {
                damage1,
                school,
                absorbed,
                resisted,
            } => {
                // damage1: u32
                w.write_all(&damage1.to_le_bytes())?;

                // school: SpellSchool
                school.write(w)?;

                // absorbed: u32
                w.write_all(&absorbed.to_le_bytes())?;

                // resisted: u32
                w.write_all(&resisted.to_le_bytes())?;

            }
            AuraLogAuraType::MOD_RESIST_CHANCE => {}
            AuraLogAuraType::MOD_DETECT_RANGE => {}
            AuraLogAuraType::PREVENTS_FLEEING => {}
            AuraLogAuraType::MOD_UNATTACKABLE => {}
            AuraLogAuraType::INTERRUPT_REGEN => {}
            AuraLogAuraType::GHOST => {}
            AuraLogAuraType::SPELL_MAGNET => {}
            AuraLogAuraType::MANA_SHIELD => {}
            AuraLogAuraType::MOD_SKILL_TALENT => {}
            AuraLogAuraType::MOD_ATTACK_POWER => {}
            AuraLogAuraType::AURAS_VISIBLE => {}
            AuraLogAuraType::MOD_RESISTANCE_PCT => {}
            AuraLogAuraType::MOD_MELEE_ATTACK_POWER_VERSUS => {}
            AuraLogAuraType::MOD_TOTAL_THREAT => {}
            AuraLogAuraType::WATER_WALK => {}
            AuraLogAuraType::FEATHER_FALL => {}
            AuraLogAuraType::HOVER => {}
            AuraLogAuraType::ADD_FLAT_MODIFIER => {}
            AuraLogAuraType::ADD_PCT_MODIFIER => {}
            AuraLogAuraType::ADD_TARGET_TRIGGER => {}
            AuraLogAuraType::MOD_POWER_REGEN_PERCENT => {}
            AuraLogAuraType::ADD_CASTER_HIT_TRIGGER => {}
            AuraLogAuraType::OVERRIDE_CLASS_SCRIPTS => {}
            AuraLogAuraType::MOD_RANGED_DAMAGE_TAKEN => {}
            AuraLogAuraType::MOD_RANGED_DAMAGE_TAKEN_PCT => {}
            AuraLogAuraType::MOD_HEALING => {}
            AuraLogAuraType::MOD_REGEN_DURING_COMBAT => {}
            AuraLogAuraType::MOD_MECHANIC_RESISTANCE => {}
            AuraLogAuraType::MOD_HEALING_PCT => {}
            AuraLogAuraType::SHARE_PET_TRACKING => {}
            AuraLogAuraType::UNTRACKABLE => {}
            AuraLogAuraType::EMPATHY => {}
            AuraLogAuraType::MOD_OFFHAND_DAMAGE_PCT => {}
            AuraLogAuraType::MOD_TARGET_RESISTANCE => {}
            AuraLogAuraType::MOD_RANGED_ATTACK_POWER => {}
            AuraLogAuraType::MOD_MELEE_DAMAGE_TAKEN => {}
            AuraLogAuraType::MOD_MELEE_DAMAGE_TAKEN_PCT => {}
            AuraLogAuraType::RANGED_ATTACK_POWER_ATTACKER_BONUS => {}
            AuraLogAuraType::MOD_POSSESS_PET => {}
            AuraLogAuraType::MOD_SPEED_ALWAYS => {}
            AuraLogAuraType::MOD_MOUNTED_SPEED_ALWAYS => {}
            AuraLogAuraType::MOD_RANGED_ATTACK_POWER_VERSUS => {}
            AuraLogAuraType::MOD_INCREASE_ENERGY_PERCENT => {}
            AuraLogAuraType::MOD_INCREASE_HEALTH_PERCENT => {}
            AuraLogAuraType::MOD_MANA_REGEN_INTERRUPT => {}
            AuraLogAuraType::MOD_HEALING_DONE => {}
            AuraLogAuraType::MOD_HEALING_DONE_PERCENT => {}
            AuraLogAuraType::MOD_TOTAL_STAT_PERCENTAGE => {}
            AuraLogAuraType::MOD_MELEE_HASTE => {}
            AuraLogAuraType::FORCE_REACTION => {}
            AuraLogAuraType::MOD_RANGED_HASTE => {}
            AuraLogAuraType::MOD_RANGED_AMMO_HASTE => {}
            AuraLogAuraType::MOD_BASE_RESISTANCE_PCT => {}
            AuraLogAuraType::MOD_RESISTANCE_EXCLUSIVE => {}
            AuraLogAuraType::SAFE_FALL => {}
            AuraLogAuraType::CHARISMA => {}
            AuraLogAuraType::PERSUADED => {}
            AuraLogAuraType::MECHANIC_IMMUNITY_MASK => {}
            AuraLogAuraType::RETAIN_COMBO_POINTS => {}
            AuraLogAuraType::RESIST_PUSHBACK => {}
            AuraLogAuraType::MOD_SHIELD_BLOCKVALUE_PCT => {}
            AuraLogAuraType::TRACK_STEALTHED => {}
            AuraLogAuraType::MOD_DETECTED_RANGE => {}
            AuraLogAuraType::SPLIT_DAMAGE_FLAT => {}
            AuraLogAuraType::MOD_STEALTH_LEVEL => {}
            AuraLogAuraType::MOD_WATER_BREATHING => {}
            AuraLogAuraType::MOD_REPUTATION_GAIN => {}
            AuraLogAuraType::PET_DAMAGE_MULTI => {}
            AuraLogAuraType::MOD_SHIELD_BLOCKVALUE => {}
            AuraLogAuraType::NO_PVP_CREDIT => {}
            AuraLogAuraType::MOD_AOE_AVOIDANCE => {}
            AuraLogAuraType::MOD_HEALTH_REGEN_IN_COMBAT => {}
            AuraLogAuraType::POWER_BURN_MANA => {}
            AuraLogAuraType::MOD_CRIT_DAMAGE_BONUS => {}
            AuraLogAuraType::UNKNOWN164 => {}
            AuraLogAuraType::MELEE_ATTACK_POWER_ATTACKER_BONUS => {}
            AuraLogAuraType::MOD_ATTACK_POWER_PCT => {}
            AuraLogAuraType::MOD_RANGED_ATTACK_POWER_PCT => {}
            AuraLogAuraType::MOD_DAMAGE_DONE_VERSUS => {}
            AuraLogAuraType::MOD_CRIT_PERCENT_VERSUS => {}
            AuraLogAuraType::DETECT_AMORE => {}
            AuraLogAuraType::MOD_SPEED_NOT_STACK => {}
            AuraLogAuraType::MOD_MOUNTED_SPEED_NOT_STACK => {}
            AuraLogAuraType::ALLOW_CHAMPION_SPELLS => {}
            AuraLogAuraType::MOD_SPELL_DAMAGE_OF_STAT_PERCENT => {}
            AuraLogAuraType::MOD_SPELL_HEALING_OF_STAT_PERCENT => {}
            AuraLogAuraType::SPIRIT_OF_REDEMPTION => {}
            AuraLogAuraType::AOE_CHARM => {}
            AuraLogAuraType::MOD_DEBUFF_RESISTANCE => {}
            AuraLogAuraType::MOD_ATTACKER_SPELL_CRIT_CHANCE => {}
            AuraLogAuraType::MOD_FLAT_SPELL_DAMAGE_VERSUS => {}
            AuraLogAuraType::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS => {}
            AuraLogAuraType::MOD_RESISTANCE_OF_STAT_PERCENT => {}
            AuraLogAuraType::MOD_CRITICAL_THREAT => {}
            AuraLogAuraType::MOD_ATTACKER_MELEE_HIT_CHANCE => {}
            AuraLogAuraType::MOD_ATTACKER_RANGED_HIT_CHANCE => {}
            AuraLogAuraType::MOD_ATTACKER_SPELL_HIT_CHANCE => {}
            AuraLogAuraType::MOD_ATTACKER_MELEE_CRIT_CHANCE => {}
            AuraLogAuraType::MOD_ATTACKER_RANGED_CRIT_CHANCE => {}
            AuraLogAuraType::MOD_RATING => {}
            AuraLogAuraType::MOD_FACTION_REPUTATION_GAIN => {}
            AuraLogAuraType::USE_NORMAL_MOVEMENT_SPEED => {}
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // aura_type: AuraType
        let aura_type = AuraType::tokio_read(r).await?;

        let aura_type_if = match aura_type {
            AuraType::NONE => AuraLogAuraType::NONE,
            AuraType::BIND_SIGHT => AuraLogAuraType::BIND_SIGHT,
            AuraType::MOD_POSSESS => AuraLogAuraType::MOD_POSSESS,
            AuraType::PERIODIC_DAMAGE => {
                // damage1: u32
                let damage1 = crate::util::tokio_read_u32_le(r).await?;

                // school: SpellSchool
                let school = SpellSchool::tokio_read(r).await?;

                // absorbed: u32
                let absorbed = crate::util::tokio_read_u32_le(r).await?;

                // resisted: u32
                let resisted = crate::util::tokio_read_u32_le(r).await?;

                AuraLogAuraType::PERIODIC_DAMAGE {
                    damage1,
                    school,
                    absorbed,
                    resisted,
                }
            }
            AuraType::DUMMY => AuraLogAuraType::DUMMY,
            AuraType::MOD_CONFUSE => AuraLogAuraType::MOD_CONFUSE,
            AuraType::MOD_CHARM => AuraLogAuraType::MOD_CHARM,
            AuraType::MOD_FEAR => AuraLogAuraType::MOD_FEAR,
            AuraType::PERIODIC_HEAL => {
                // damage2: u32
                let damage2 = crate::util::tokio_read_u32_le(r).await?;

                AuraLogAuraType::PERIODIC_HEAL {
                    damage2,
                }
            }
            AuraType::MOD_ATTACKSPEED => AuraLogAuraType::MOD_ATTACKSPEED,
            AuraType::MOD_THREAT => AuraLogAuraType::MOD_THREAT,
            AuraType::MOD_TAUNT => AuraLogAuraType::MOD_TAUNT,
            AuraType::MOD_STUN => AuraLogAuraType::MOD_STUN,
            AuraType::MOD_DAMAGE_DONE => AuraLogAuraType::MOD_DAMAGE_DONE,
            AuraType::MOD_DAMAGE_TAKEN => AuraLogAuraType::MOD_DAMAGE_TAKEN,
            AuraType::DAMAGE_SHIELD => AuraLogAuraType::DAMAGE_SHIELD,
            AuraType::MOD_STEALTH => AuraLogAuraType::MOD_STEALTH,
            AuraType::MOD_STEALTH_DETECT => AuraLogAuraType::MOD_STEALTH_DETECT,
            AuraType::MOD_INVISIBILITY => AuraLogAuraType::MOD_INVISIBILITY,
            AuraType::MOD_INVISIBILITY_DETECTION => AuraLogAuraType::MOD_INVISIBILITY_DETECTION,
            AuraType::OBS_MOD_HEALTH => {
                // damage2: u32
                let damage2 = crate::util::tokio_read_u32_le(r).await?;

                AuraLogAuraType::OBS_MOD_HEALTH {
                    damage2,
                }
            }
            AuraType::OBS_MOD_MANA => {
                // misc_value1: u32
                let misc_value1 = crate::util::tokio_read_u32_le(r).await?;

                // damage3: u32
                let damage3 = crate::util::tokio_read_u32_le(r).await?;

                AuraLogAuraType::OBS_MOD_MANA {
                    misc_value1,
                    damage3,
                }
            }
            AuraType::MOD_RESISTANCE => AuraLogAuraType::MOD_RESISTANCE,
            AuraType::PERIODIC_TRIGGER_SPELL => AuraLogAuraType::PERIODIC_TRIGGER_SPELL,
            AuraType::PERIODIC_ENERGIZE => {
                // misc_value1: u32
                let misc_value1 = crate::util::tokio_read_u32_le(r).await?;

                // damage3: u32
                let damage3 = crate::util::tokio_read_u32_le(r).await?;

                AuraLogAuraType::PERIODIC_ENERGIZE {
                    misc_value1,
                    damage3,
                }
            }
            AuraType::MOD_PACIFY => AuraLogAuraType::MOD_PACIFY,
            AuraType::MOD_ROOT => AuraLogAuraType::MOD_ROOT,
            AuraType::MOD_SILENCE => AuraLogAuraType::MOD_SILENCE,
            AuraType::REFLECT_SPELLS => AuraLogAuraType::REFLECT_SPELLS,
            AuraType::MOD_STAT => AuraLogAuraType::MOD_STAT,
            AuraType::MOD_SKILL => AuraLogAuraType::MOD_SKILL,
            AuraType::MOD_INCREASE_SPEED => AuraLogAuraType::MOD_INCREASE_SPEED,
            AuraType::MOD_INCREASE_MOUNTED_SPEED => AuraLogAuraType::MOD_INCREASE_MOUNTED_SPEED,
            AuraType::MOD_DECREASE_SPEED => AuraLogAuraType::MOD_DECREASE_SPEED,
            AuraType::MOD_INCREASE_HEALTH => AuraLogAuraType::MOD_INCREASE_HEALTH,
            AuraType::MOD_INCREASE_ENERGY => AuraLogAuraType::MOD_INCREASE_ENERGY,
            AuraType::MOD_SHAPESHIFT => AuraLogAuraType::MOD_SHAPESHIFT,
            AuraType::EFFECT_IMMUNITY => AuraLogAuraType::EFFECT_IMMUNITY,
            AuraType::STATE_IMMUNITY => AuraLogAuraType::STATE_IMMUNITY,
            AuraType::SCHOOL_IMMUNITY => AuraLogAuraType::SCHOOL_IMMUNITY,
            AuraType::DAMAGE_IMMUNITY => AuraLogAuraType::DAMAGE_IMMUNITY,
            AuraType::DISPEL_IMMUNITY => AuraLogAuraType::DISPEL_IMMUNITY,
            AuraType::PROC_TRIGGER_SPELL => AuraLogAuraType::PROC_TRIGGER_SPELL,
            AuraType::PROC_TRIGGER_DAMAGE => AuraLogAuraType::PROC_TRIGGER_DAMAGE,
            AuraType::TRACK_CREATURES => AuraLogAuraType::TRACK_CREATURES,
            AuraType::TRACK_RESOURCES => AuraLogAuraType::TRACK_RESOURCES,
            AuraType::UNKNOWN46 => AuraLogAuraType::UNKNOWN46,
            AuraType::MOD_PARRY_PERCENT => AuraLogAuraType::MOD_PARRY_PERCENT,
            AuraType::UNKNOWN48 => AuraLogAuraType::UNKNOWN48,
            AuraType::MOD_DODGE_PERCENT => AuraLogAuraType::MOD_DODGE_PERCENT,
            AuraType::MOD_BLOCK_SKILL => AuraLogAuraType::MOD_BLOCK_SKILL,
            AuraType::MOD_BLOCK_PERCENT => AuraLogAuraType::MOD_BLOCK_PERCENT,
            AuraType::MOD_CRIT_PERCENT => AuraLogAuraType::MOD_CRIT_PERCENT,
            AuraType::PERIODIC_LEECH => AuraLogAuraType::PERIODIC_LEECH,
            AuraType::MOD_HIT_CHANCE => AuraLogAuraType::MOD_HIT_CHANCE,
            AuraType::MOD_SPELL_HIT_CHANCE => AuraLogAuraType::MOD_SPELL_HIT_CHANCE,
            AuraType::TRANSFORM => AuraLogAuraType::TRANSFORM,
            AuraType::MOD_SPELL_CRIT_CHANCE => AuraLogAuraType::MOD_SPELL_CRIT_CHANCE,
            AuraType::MOD_INCREASE_SWIM_SPEED => AuraLogAuraType::MOD_INCREASE_SWIM_SPEED,
            AuraType::MOD_DAMAGE_DONE_CREATURE => AuraLogAuraType::MOD_DAMAGE_DONE_CREATURE,
            AuraType::MOD_PACIFY_SILENCE => AuraLogAuraType::MOD_PACIFY_SILENCE,
            AuraType::MOD_SCALE => AuraLogAuraType::MOD_SCALE,
            AuraType::PERIODIC_HEALTH_FUNNEL => AuraLogAuraType::PERIODIC_HEALTH_FUNNEL,
            AuraType::PERIODIC_MANA_FUNNEL => AuraLogAuraType::PERIODIC_MANA_FUNNEL,
            AuraType::PERIODIC_MANA_LEECH => {
                // misc_value2: u32
                let misc_value2 = crate::util::tokio_read_u32_le(r).await?;

                // damage: u32
                let damage = crate::util::tokio_read_u32_le(r).await?;

                // gain_multiplier: f32
                let gain_multiplier = crate::util::tokio_read_f32_le(r).await?;
                AuraLogAuraType::PERIODIC_MANA_LEECH {
                    misc_value2,
                    damage,
                    gain_multiplier,
                }
            }
            AuraType::MOD_CASTING_SPEED_NOT_STACK => AuraLogAuraType::MOD_CASTING_SPEED_NOT_STACK,
            AuraType::FEIGN_DEATH => AuraLogAuraType::FEIGN_DEATH,
            AuraType::MOD_DISARM => AuraLogAuraType::MOD_DISARM,
            AuraType::MOD_STALKED => AuraLogAuraType::MOD_STALKED,
            AuraType::SCHOOL_ABSORB => AuraLogAuraType::SCHOOL_ABSORB,
            AuraType::EXTRA_ATTACKS => AuraLogAuraType::EXTRA_ATTACKS,
            AuraType::MOD_SPELL_CRIT_CHANCE_SCHOOL => AuraLogAuraType::MOD_SPELL_CRIT_CHANCE_SCHOOL,
            AuraType::MOD_POWER_COST_SCHOOL_PCT => AuraLogAuraType::MOD_POWER_COST_SCHOOL_PCT,
            AuraType::MOD_POWER_COST_SCHOOL => AuraLogAuraType::MOD_POWER_COST_SCHOOL,
            AuraType::REFLECT_SPELLS_SCHOOL => AuraLogAuraType::REFLECT_SPELLS_SCHOOL,
            AuraType::MOD_LANGUAGE => AuraLogAuraType::MOD_LANGUAGE,
            AuraType::FAR_SIGHT => AuraLogAuraType::FAR_SIGHT,
            AuraType::MECHANIC_IMMUNITY => AuraLogAuraType::MECHANIC_IMMUNITY,
            AuraType::MOUNTED => AuraLogAuraType::MOUNTED,
            AuraType::MOD_DAMAGE_PERCENT_DONE => AuraLogAuraType::MOD_DAMAGE_PERCENT_DONE,
            AuraType::MOD_PERCENT_STAT => AuraLogAuraType::MOD_PERCENT_STAT,
            AuraType::SPLIT_DAMAGE_PCT => AuraLogAuraType::SPLIT_DAMAGE_PCT,
            AuraType::WATER_BREATHING => AuraLogAuraType::WATER_BREATHING,
            AuraType::MOD_BASE_RESISTANCE => AuraLogAuraType::MOD_BASE_RESISTANCE,
            AuraType::MOD_REGEN => AuraLogAuraType::MOD_REGEN,
            AuraType::MOD_POWER_REGEN => AuraLogAuraType::MOD_POWER_REGEN,
            AuraType::CHANNEL_DEATH_ITEM => AuraLogAuraType::CHANNEL_DEATH_ITEM,
            AuraType::MOD_DAMAGE_PERCENT_TAKEN => AuraLogAuraType::MOD_DAMAGE_PERCENT_TAKEN,
            AuraType::MOD_HEALTH_REGEN_PERCENT => AuraLogAuraType::MOD_HEALTH_REGEN_PERCENT,
            AuraType::PERIODIC_DAMAGE_PERCENT => {
                // damage1: u32
                let damage1 = crate::util::tokio_read_u32_le(r).await?;

                // school: SpellSchool
                let school = SpellSchool::tokio_read(r).await?;

                // absorbed: u32
                let absorbed = crate::util::tokio_read_u32_le(r).await?;

                // resisted: u32
                let resisted = crate::util::tokio_read_u32_le(r).await?;

                AuraLogAuraType::PERIODIC_DAMAGE_PERCENT {
                    damage1,
                    school,
                    absorbed,
                    resisted,
                }
            }
            AuraType::MOD_RESIST_CHANCE => AuraLogAuraType::MOD_RESIST_CHANCE,
            AuraType::MOD_DETECT_RANGE => AuraLogAuraType::MOD_DETECT_RANGE,
            AuraType::PREVENTS_FLEEING => AuraLogAuraType::PREVENTS_FLEEING,
            AuraType::MOD_UNATTACKABLE => AuraLogAuraType::MOD_UNATTACKABLE,
            AuraType::INTERRUPT_REGEN => AuraLogAuraType::INTERRUPT_REGEN,
            AuraType::GHOST => AuraLogAuraType::GHOST,
            AuraType::SPELL_MAGNET => AuraLogAuraType::SPELL_MAGNET,
            AuraType::MANA_SHIELD => AuraLogAuraType::MANA_SHIELD,
            AuraType::MOD_SKILL_TALENT => AuraLogAuraType::MOD_SKILL_TALENT,
            AuraType::MOD_ATTACK_POWER => AuraLogAuraType::MOD_ATTACK_POWER,
            AuraType::AURAS_VISIBLE => AuraLogAuraType::AURAS_VISIBLE,
            AuraType::MOD_RESISTANCE_PCT => AuraLogAuraType::MOD_RESISTANCE_PCT,
            AuraType::MOD_MELEE_ATTACK_POWER_VERSUS => AuraLogAuraType::MOD_MELEE_ATTACK_POWER_VERSUS,
            AuraType::MOD_TOTAL_THREAT => AuraLogAuraType::MOD_TOTAL_THREAT,
            AuraType::WATER_WALK => AuraLogAuraType::WATER_WALK,
            AuraType::FEATHER_FALL => AuraLogAuraType::FEATHER_FALL,
            AuraType::HOVER => AuraLogAuraType::HOVER,
            AuraType::ADD_FLAT_MODIFIER => AuraLogAuraType::ADD_FLAT_MODIFIER,
            AuraType::ADD_PCT_MODIFIER => AuraLogAuraType::ADD_PCT_MODIFIER,
            AuraType::ADD_TARGET_TRIGGER => AuraLogAuraType::ADD_TARGET_TRIGGER,
            AuraType::MOD_POWER_REGEN_PERCENT => AuraLogAuraType::MOD_POWER_REGEN_PERCENT,
            AuraType::ADD_CASTER_HIT_TRIGGER => AuraLogAuraType::ADD_CASTER_HIT_TRIGGER,
            AuraType::OVERRIDE_CLASS_SCRIPTS => AuraLogAuraType::OVERRIDE_CLASS_SCRIPTS,
            AuraType::MOD_RANGED_DAMAGE_TAKEN => AuraLogAuraType::MOD_RANGED_DAMAGE_TAKEN,
            AuraType::MOD_RANGED_DAMAGE_TAKEN_PCT => AuraLogAuraType::MOD_RANGED_DAMAGE_TAKEN_PCT,
            AuraType::MOD_HEALING => AuraLogAuraType::MOD_HEALING,
            AuraType::MOD_REGEN_DURING_COMBAT => AuraLogAuraType::MOD_REGEN_DURING_COMBAT,
            AuraType::MOD_MECHANIC_RESISTANCE => AuraLogAuraType::MOD_MECHANIC_RESISTANCE,
            AuraType::MOD_HEALING_PCT => AuraLogAuraType::MOD_HEALING_PCT,
            AuraType::SHARE_PET_TRACKING => AuraLogAuraType::SHARE_PET_TRACKING,
            AuraType::UNTRACKABLE => AuraLogAuraType::UNTRACKABLE,
            AuraType::EMPATHY => AuraLogAuraType::EMPATHY,
            AuraType::MOD_OFFHAND_DAMAGE_PCT => AuraLogAuraType::MOD_OFFHAND_DAMAGE_PCT,
            AuraType::MOD_TARGET_RESISTANCE => AuraLogAuraType::MOD_TARGET_RESISTANCE,
            AuraType::MOD_RANGED_ATTACK_POWER => AuraLogAuraType::MOD_RANGED_ATTACK_POWER,
            AuraType::MOD_MELEE_DAMAGE_TAKEN => AuraLogAuraType::MOD_MELEE_DAMAGE_TAKEN,
            AuraType::MOD_MELEE_DAMAGE_TAKEN_PCT => AuraLogAuraType::MOD_MELEE_DAMAGE_TAKEN_PCT,
            AuraType::RANGED_ATTACK_POWER_ATTACKER_BONUS => AuraLogAuraType::RANGED_ATTACK_POWER_ATTACKER_BONUS,
            AuraType::MOD_POSSESS_PET => AuraLogAuraType::MOD_POSSESS_PET,
            AuraType::MOD_SPEED_ALWAYS => AuraLogAuraType::MOD_SPEED_ALWAYS,
            AuraType::MOD_MOUNTED_SPEED_ALWAYS => AuraLogAuraType::MOD_MOUNTED_SPEED_ALWAYS,
            AuraType::MOD_RANGED_ATTACK_POWER_VERSUS => AuraLogAuraType::MOD_RANGED_ATTACK_POWER_VERSUS,
            AuraType::MOD_INCREASE_ENERGY_PERCENT => AuraLogAuraType::MOD_INCREASE_ENERGY_PERCENT,
            AuraType::MOD_INCREASE_HEALTH_PERCENT => AuraLogAuraType::MOD_INCREASE_HEALTH_PERCENT,
            AuraType::MOD_MANA_REGEN_INTERRUPT => AuraLogAuraType::MOD_MANA_REGEN_INTERRUPT,
            AuraType::MOD_HEALING_DONE => AuraLogAuraType::MOD_HEALING_DONE,
            AuraType::MOD_HEALING_DONE_PERCENT => AuraLogAuraType::MOD_HEALING_DONE_PERCENT,
            AuraType::MOD_TOTAL_STAT_PERCENTAGE => AuraLogAuraType::MOD_TOTAL_STAT_PERCENTAGE,
            AuraType::MOD_MELEE_HASTE => AuraLogAuraType::MOD_MELEE_HASTE,
            AuraType::FORCE_REACTION => AuraLogAuraType::FORCE_REACTION,
            AuraType::MOD_RANGED_HASTE => AuraLogAuraType::MOD_RANGED_HASTE,
            AuraType::MOD_RANGED_AMMO_HASTE => AuraLogAuraType::MOD_RANGED_AMMO_HASTE,
            AuraType::MOD_BASE_RESISTANCE_PCT => AuraLogAuraType::MOD_BASE_RESISTANCE_PCT,
            AuraType::MOD_RESISTANCE_EXCLUSIVE => AuraLogAuraType::MOD_RESISTANCE_EXCLUSIVE,
            AuraType::SAFE_FALL => AuraLogAuraType::SAFE_FALL,
            AuraType::CHARISMA => AuraLogAuraType::CHARISMA,
            AuraType::PERSUADED => AuraLogAuraType::PERSUADED,
            AuraType::MECHANIC_IMMUNITY_MASK => AuraLogAuraType::MECHANIC_IMMUNITY_MASK,
            AuraType::RETAIN_COMBO_POINTS => AuraLogAuraType::RETAIN_COMBO_POINTS,
            AuraType::RESIST_PUSHBACK => AuraLogAuraType::RESIST_PUSHBACK,
            AuraType::MOD_SHIELD_BLOCKVALUE_PCT => AuraLogAuraType::MOD_SHIELD_BLOCKVALUE_PCT,
            AuraType::TRACK_STEALTHED => AuraLogAuraType::TRACK_STEALTHED,
            AuraType::MOD_DETECTED_RANGE => AuraLogAuraType::MOD_DETECTED_RANGE,
            AuraType::SPLIT_DAMAGE_FLAT => AuraLogAuraType::SPLIT_DAMAGE_FLAT,
            AuraType::MOD_STEALTH_LEVEL => AuraLogAuraType::MOD_STEALTH_LEVEL,
            AuraType::MOD_WATER_BREATHING => AuraLogAuraType::MOD_WATER_BREATHING,
            AuraType::MOD_REPUTATION_GAIN => AuraLogAuraType::MOD_REPUTATION_GAIN,
            AuraType::PET_DAMAGE_MULTI => AuraLogAuraType::PET_DAMAGE_MULTI,
            AuraType::MOD_SHIELD_BLOCKVALUE => AuraLogAuraType::MOD_SHIELD_BLOCKVALUE,
            AuraType::NO_PVP_CREDIT => AuraLogAuraType::NO_PVP_CREDIT,
            AuraType::MOD_AOE_AVOIDANCE => AuraLogAuraType::MOD_AOE_AVOIDANCE,
            AuraType::MOD_HEALTH_REGEN_IN_COMBAT => AuraLogAuraType::MOD_HEALTH_REGEN_IN_COMBAT,
            AuraType::POWER_BURN_MANA => AuraLogAuraType::POWER_BURN_MANA,
            AuraType::MOD_CRIT_DAMAGE_BONUS => AuraLogAuraType::MOD_CRIT_DAMAGE_BONUS,
            AuraType::UNKNOWN164 => AuraLogAuraType::UNKNOWN164,
            AuraType::MELEE_ATTACK_POWER_ATTACKER_BONUS => AuraLogAuraType::MELEE_ATTACK_POWER_ATTACKER_BONUS,
            AuraType::MOD_ATTACK_POWER_PCT => AuraLogAuraType::MOD_ATTACK_POWER_PCT,
            AuraType::MOD_RANGED_ATTACK_POWER_PCT => AuraLogAuraType::MOD_RANGED_ATTACK_POWER_PCT,
            AuraType::MOD_DAMAGE_DONE_VERSUS => AuraLogAuraType::MOD_DAMAGE_DONE_VERSUS,
            AuraType::MOD_CRIT_PERCENT_VERSUS => AuraLogAuraType::MOD_CRIT_PERCENT_VERSUS,
            AuraType::DETECT_AMORE => AuraLogAuraType::DETECT_AMORE,
            AuraType::MOD_SPEED_NOT_STACK => AuraLogAuraType::MOD_SPEED_NOT_STACK,
            AuraType::MOD_MOUNTED_SPEED_NOT_STACK => AuraLogAuraType::MOD_MOUNTED_SPEED_NOT_STACK,
            AuraType::ALLOW_CHAMPION_SPELLS => AuraLogAuraType::ALLOW_CHAMPION_SPELLS,
            AuraType::MOD_SPELL_DAMAGE_OF_STAT_PERCENT => AuraLogAuraType::MOD_SPELL_DAMAGE_OF_STAT_PERCENT,
            AuraType::MOD_SPELL_HEALING_OF_STAT_PERCENT => AuraLogAuraType::MOD_SPELL_HEALING_OF_STAT_PERCENT,
            AuraType::SPIRIT_OF_REDEMPTION => AuraLogAuraType::SPIRIT_OF_REDEMPTION,
            AuraType::AOE_CHARM => AuraLogAuraType::AOE_CHARM,
            AuraType::MOD_DEBUFF_RESISTANCE => AuraLogAuraType::MOD_DEBUFF_RESISTANCE,
            AuraType::MOD_ATTACKER_SPELL_CRIT_CHANCE => AuraLogAuraType::MOD_ATTACKER_SPELL_CRIT_CHANCE,
            AuraType::MOD_FLAT_SPELL_DAMAGE_VERSUS => AuraLogAuraType::MOD_FLAT_SPELL_DAMAGE_VERSUS,
            AuraType::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS => AuraLogAuraType::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS,
            AuraType::MOD_RESISTANCE_OF_STAT_PERCENT => AuraLogAuraType::MOD_RESISTANCE_OF_STAT_PERCENT,
            AuraType::MOD_CRITICAL_THREAT => AuraLogAuraType::MOD_CRITICAL_THREAT,
            AuraType::MOD_ATTACKER_MELEE_HIT_CHANCE => AuraLogAuraType::MOD_ATTACKER_MELEE_HIT_CHANCE,
            AuraType::MOD_ATTACKER_RANGED_HIT_CHANCE => AuraLogAuraType::MOD_ATTACKER_RANGED_HIT_CHANCE,
            AuraType::MOD_ATTACKER_SPELL_HIT_CHANCE => AuraLogAuraType::MOD_ATTACKER_SPELL_HIT_CHANCE,
            AuraType::MOD_ATTACKER_MELEE_CRIT_CHANCE => AuraLogAuraType::MOD_ATTACKER_MELEE_CRIT_CHANCE,
            AuraType::MOD_ATTACKER_RANGED_CRIT_CHANCE => AuraLogAuraType::MOD_ATTACKER_RANGED_CRIT_CHANCE,
            AuraType::MOD_RATING => AuraLogAuraType::MOD_RATING,
            AuraType::MOD_FACTION_REPUTATION_GAIN => AuraLogAuraType::MOD_FACTION_REPUTATION_GAIN,
            AuraType::USE_NORMAL_MOVEMENT_SPEED => AuraLogAuraType::USE_NORMAL_MOVEMENT_SPEED,
        };

        Ok(Self {
            aura_type: aura_type_if,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // aura_type: AuraType
        self.aura_type.tokio_write(w).await?;

        match &self.aura_type {
            AuraLogAuraType::NONE => {}
            AuraLogAuraType::BIND_SIGHT => {}
            AuraLogAuraType::MOD_POSSESS => {}
            AuraLogAuraType::PERIODIC_DAMAGE {
                damage1,
                school,
                absorbed,
                resisted,
            } => {
                // damage1: u32
                w.write_all(&damage1.to_le_bytes()).await?;

                // school: SpellSchool
                school.tokio_write(w).await?;

                // absorbed: u32
                w.write_all(&absorbed.to_le_bytes()).await?;

                // resisted: u32
                w.write_all(&resisted.to_le_bytes()).await?;

            }
            AuraLogAuraType::DUMMY => {}
            AuraLogAuraType::MOD_CONFUSE => {}
            AuraLogAuraType::MOD_CHARM => {}
            AuraLogAuraType::MOD_FEAR => {}
            AuraLogAuraType::PERIODIC_HEAL {
                damage2,
            } => {
                // damage2: u32
                w.write_all(&damage2.to_le_bytes()).await?;

            }
            AuraLogAuraType::MOD_ATTACKSPEED => {}
            AuraLogAuraType::MOD_THREAT => {}
            AuraLogAuraType::MOD_TAUNT => {}
            AuraLogAuraType::MOD_STUN => {}
            AuraLogAuraType::MOD_DAMAGE_DONE => {}
            AuraLogAuraType::MOD_DAMAGE_TAKEN => {}
            AuraLogAuraType::DAMAGE_SHIELD => {}
            AuraLogAuraType::MOD_STEALTH => {}
            AuraLogAuraType::MOD_STEALTH_DETECT => {}
            AuraLogAuraType::MOD_INVISIBILITY => {}
            AuraLogAuraType::MOD_INVISIBILITY_DETECTION => {}
            AuraLogAuraType::OBS_MOD_HEALTH {
                damage2,
            } => {
                // damage2: u32
                w.write_all(&damage2.to_le_bytes()).await?;

            }
            AuraLogAuraType::OBS_MOD_MANA {
                misc_value1,
                damage3,
            } => {
                // misc_value1: u32
                w.write_all(&misc_value1.to_le_bytes()).await?;

                // damage3: u32
                w.write_all(&damage3.to_le_bytes()).await?;

            }
            AuraLogAuraType::MOD_RESISTANCE => {}
            AuraLogAuraType::PERIODIC_TRIGGER_SPELL => {}
            AuraLogAuraType::PERIODIC_ENERGIZE {
                misc_value1,
                damage3,
            } => {
                // misc_value1: u32
                w.write_all(&misc_value1.to_le_bytes()).await?;

                // damage3: u32
                w.write_all(&damage3.to_le_bytes()).await?;

            }
            AuraLogAuraType::MOD_PACIFY => {}
            AuraLogAuraType::MOD_ROOT => {}
            AuraLogAuraType::MOD_SILENCE => {}
            AuraLogAuraType::REFLECT_SPELLS => {}
            AuraLogAuraType::MOD_STAT => {}
            AuraLogAuraType::MOD_SKILL => {}
            AuraLogAuraType::MOD_INCREASE_SPEED => {}
            AuraLogAuraType::MOD_INCREASE_MOUNTED_SPEED => {}
            AuraLogAuraType::MOD_DECREASE_SPEED => {}
            AuraLogAuraType::MOD_INCREASE_HEALTH => {}
            AuraLogAuraType::MOD_INCREASE_ENERGY => {}
            AuraLogAuraType::MOD_SHAPESHIFT => {}
            AuraLogAuraType::EFFECT_IMMUNITY => {}
            AuraLogAuraType::STATE_IMMUNITY => {}
            AuraLogAuraType::SCHOOL_IMMUNITY => {}
            AuraLogAuraType::DAMAGE_IMMUNITY => {}
            AuraLogAuraType::DISPEL_IMMUNITY => {}
            AuraLogAuraType::PROC_TRIGGER_SPELL => {}
            AuraLogAuraType::PROC_TRIGGER_DAMAGE => {}
            AuraLogAuraType::TRACK_CREATURES => {}
            AuraLogAuraType::TRACK_RESOURCES => {}
            AuraLogAuraType::UNKNOWN46 => {}
            AuraLogAuraType::MOD_PARRY_PERCENT => {}
            AuraLogAuraType::UNKNOWN48 => {}
            AuraLogAuraType::MOD_DODGE_PERCENT => {}
            AuraLogAuraType::MOD_BLOCK_SKILL => {}
            AuraLogAuraType::MOD_BLOCK_PERCENT => {}
            AuraLogAuraType::MOD_CRIT_PERCENT => {}
            AuraLogAuraType::PERIODIC_LEECH => {}
            AuraLogAuraType::MOD_HIT_CHANCE => {}
            AuraLogAuraType::MOD_SPELL_HIT_CHANCE => {}
            AuraLogAuraType::TRANSFORM => {}
            AuraLogAuraType::MOD_SPELL_CRIT_CHANCE => {}
            AuraLogAuraType::MOD_INCREASE_SWIM_SPEED => {}
            AuraLogAuraType::MOD_DAMAGE_DONE_CREATURE => {}
            AuraLogAuraType::MOD_PACIFY_SILENCE => {}
            AuraLogAuraType::MOD_SCALE => {}
            AuraLogAuraType::PERIODIC_HEALTH_FUNNEL => {}
            AuraLogAuraType::PERIODIC_MANA_FUNNEL => {}
            AuraLogAuraType::PERIODIC_MANA_LEECH {
                misc_value2,
                damage,
                gain_multiplier,
            } => {
                // misc_value2: u32
                w.write_all(&misc_value2.to_le_bytes()).await?;

                // damage: u32
                w.write_all(&damage.to_le_bytes()).await?;

                // gain_multiplier: f32
                w.write_all(&gain_multiplier.to_le_bytes()).await?;

            }
            AuraLogAuraType::MOD_CASTING_SPEED_NOT_STACK => {}
            AuraLogAuraType::FEIGN_DEATH => {}
            AuraLogAuraType::MOD_DISARM => {}
            AuraLogAuraType::MOD_STALKED => {}
            AuraLogAuraType::SCHOOL_ABSORB => {}
            AuraLogAuraType::EXTRA_ATTACKS => {}
            AuraLogAuraType::MOD_SPELL_CRIT_CHANCE_SCHOOL => {}
            AuraLogAuraType::MOD_POWER_COST_SCHOOL_PCT => {}
            AuraLogAuraType::MOD_POWER_COST_SCHOOL => {}
            AuraLogAuraType::REFLECT_SPELLS_SCHOOL => {}
            AuraLogAuraType::MOD_LANGUAGE => {}
            AuraLogAuraType::FAR_SIGHT => {}
            AuraLogAuraType::MECHANIC_IMMUNITY => {}
            AuraLogAuraType::MOUNTED => {}
            AuraLogAuraType::MOD_DAMAGE_PERCENT_DONE => {}
            AuraLogAuraType::MOD_PERCENT_STAT => {}
            AuraLogAuraType::SPLIT_DAMAGE_PCT => {}
            AuraLogAuraType::WATER_BREATHING => {}
            AuraLogAuraType::MOD_BASE_RESISTANCE => {}
            AuraLogAuraType::MOD_REGEN => {}
            AuraLogAuraType::MOD_POWER_REGEN => {}
            AuraLogAuraType::CHANNEL_DEATH_ITEM => {}
            AuraLogAuraType::MOD_DAMAGE_PERCENT_TAKEN => {}
            AuraLogAuraType::MOD_HEALTH_REGEN_PERCENT => {}
            AuraLogAuraType::PERIODIC_DAMAGE_PERCENT {
                damage1,
                school,
                absorbed,
                resisted,
            } => {
                // damage1: u32
                w.write_all(&damage1.to_le_bytes()).await?;

                // school: SpellSchool
                school.tokio_write(w).await?;

                // absorbed: u32
                w.write_all(&absorbed.to_le_bytes()).await?;

                // resisted: u32
                w.write_all(&resisted.to_le_bytes()).await?;

            }
            AuraLogAuraType::MOD_RESIST_CHANCE => {}
            AuraLogAuraType::MOD_DETECT_RANGE => {}
            AuraLogAuraType::PREVENTS_FLEEING => {}
            AuraLogAuraType::MOD_UNATTACKABLE => {}
            AuraLogAuraType::INTERRUPT_REGEN => {}
            AuraLogAuraType::GHOST => {}
            AuraLogAuraType::SPELL_MAGNET => {}
            AuraLogAuraType::MANA_SHIELD => {}
            AuraLogAuraType::MOD_SKILL_TALENT => {}
            AuraLogAuraType::MOD_ATTACK_POWER => {}
            AuraLogAuraType::AURAS_VISIBLE => {}
            AuraLogAuraType::MOD_RESISTANCE_PCT => {}
            AuraLogAuraType::MOD_MELEE_ATTACK_POWER_VERSUS => {}
            AuraLogAuraType::MOD_TOTAL_THREAT => {}
            AuraLogAuraType::WATER_WALK => {}
            AuraLogAuraType::FEATHER_FALL => {}
            AuraLogAuraType::HOVER => {}
            AuraLogAuraType::ADD_FLAT_MODIFIER => {}
            AuraLogAuraType::ADD_PCT_MODIFIER => {}
            AuraLogAuraType::ADD_TARGET_TRIGGER => {}
            AuraLogAuraType::MOD_POWER_REGEN_PERCENT => {}
            AuraLogAuraType::ADD_CASTER_HIT_TRIGGER => {}
            AuraLogAuraType::OVERRIDE_CLASS_SCRIPTS => {}
            AuraLogAuraType::MOD_RANGED_DAMAGE_TAKEN => {}
            AuraLogAuraType::MOD_RANGED_DAMAGE_TAKEN_PCT => {}
            AuraLogAuraType::MOD_HEALING => {}
            AuraLogAuraType::MOD_REGEN_DURING_COMBAT => {}
            AuraLogAuraType::MOD_MECHANIC_RESISTANCE => {}
            AuraLogAuraType::MOD_HEALING_PCT => {}
            AuraLogAuraType::SHARE_PET_TRACKING => {}
            AuraLogAuraType::UNTRACKABLE => {}
            AuraLogAuraType::EMPATHY => {}
            AuraLogAuraType::MOD_OFFHAND_DAMAGE_PCT => {}
            AuraLogAuraType::MOD_TARGET_RESISTANCE => {}
            AuraLogAuraType::MOD_RANGED_ATTACK_POWER => {}
            AuraLogAuraType::MOD_MELEE_DAMAGE_TAKEN => {}
            AuraLogAuraType::MOD_MELEE_DAMAGE_TAKEN_PCT => {}
            AuraLogAuraType::RANGED_ATTACK_POWER_ATTACKER_BONUS => {}
            AuraLogAuraType::MOD_POSSESS_PET => {}
            AuraLogAuraType::MOD_SPEED_ALWAYS => {}
            AuraLogAuraType::MOD_MOUNTED_SPEED_ALWAYS => {}
            AuraLogAuraType::MOD_RANGED_ATTACK_POWER_VERSUS => {}
            AuraLogAuraType::MOD_INCREASE_ENERGY_PERCENT => {}
            AuraLogAuraType::MOD_INCREASE_HEALTH_PERCENT => {}
            AuraLogAuraType::MOD_MANA_REGEN_INTERRUPT => {}
            AuraLogAuraType::MOD_HEALING_DONE => {}
            AuraLogAuraType::MOD_HEALING_DONE_PERCENT => {}
            AuraLogAuraType::MOD_TOTAL_STAT_PERCENTAGE => {}
            AuraLogAuraType::MOD_MELEE_HASTE => {}
            AuraLogAuraType::FORCE_REACTION => {}
            AuraLogAuraType::MOD_RANGED_HASTE => {}
            AuraLogAuraType::MOD_RANGED_AMMO_HASTE => {}
            AuraLogAuraType::MOD_BASE_RESISTANCE_PCT => {}
            AuraLogAuraType::MOD_RESISTANCE_EXCLUSIVE => {}
            AuraLogAuraType::SAFE_FALL => {}
            AuraLogAuraType::CHARISMA => {}
            AuraLogAuraType::PERSUADED => {}
            AuraLogAuraType::MECHANIC_IMMUNITY_MASK => {}
            AuraLogAuraType::RETAIN_COMBO_POINTS => {}
            AuraLogAuraType::RESIST_PUSHBACK => {}
            AuraLogAuraType::MOD_SHIELD_BLOCKVALUE_PCT => {}
            AuraLogAuraType::TRACK_STEALTHED => {}
            AuraLogAuraType::MOD_DETECTED_RANGE => {}
            AuraLogAuraType::SPLIT_DAMAGE_FLAT => {}
            AuraLogAuraType::MOD_STEALTH_LEVEL => {}
            AuraLogAuraType::MOD_WATER_BREATHING => {}
            AuraLogAuraType::MOD_REPUTATION_GAIN => {}
            AuraLogAuraType::PET_DAMAGE_MULTI => {}
            AuraLogAuraType::MOD_SHIELD_BLOCKVALUE => {}
            AuraLogAuraType::NO_PVP_CREDIT => {}
            AuraLogAuraType::MOD_AOE_AVOIDANCE => {}
            AuraLogAuraType::MOD_HEALTH_REGEN_IN_COMBAT => {}
            AuraLogAuraType::POWER_BURN_MANA => {}
            AuraLogAuraType::MOD_CRIT_DAMAGE_BONUS => {}
            AuraLogAuraType::UNKNOWN164 => {}
            AuraLogAuraType::MELEE_ATTACK_POWER_ATTACKER_BONUS => {}
            AuraLogAuraType::MOD_ATTACK_POWER_PCT => {}
            AuraLogAuraType::MOD_RANGED_ATTACK_POWER_PCT => {}
            AuraLogAuraType::MOD_DAMAGE_DONE_VERSUS => {}
            AuraLogAuraType::MOD_CRIT_PERCENT_VERSUS => {}
            AuraLogAuraType::DETECT_AMORE => {}
            AuraLogAuraType::MOD_SPEED_NOT_STACK => {}
            AuraLogAuraType::MOD_MOUNTED_SPEED_NOT_STACK => {}
            AuraLogAuraType::ALLOW_CHAMPION_SPELLS => {}
            AuraLogAuraType::MOD_SPELL_DAMAGE_OF_STAT_PERCENT => {}
            AuraLogAuraType::MOD_SPELL_HEALING_OF_STAT_PERCENT => {}
            AuraLogAuraType::SPIRIT_OF_REDEMPTION => {}
            AuraLogAuraType::AOE_CHARM => {}
            AuraLogAuraType::MOD_DEBUFF_RESISTANCE => {}
            AuraLogAuraType::MOD_ATTACKER_SPELL_CRIT_CHANCE => {}
            AuraLogAuraType::MOD_FLAT_SPELL_DAMAGE_VERSUS => {}
            AuraLogAuraType::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS => {}
            AuraLogAuraType::MOD_RESISTANCE_OF_STAT_PERCENT => {}
            AuraLogAuraType::MOD_CRITICAL_THREAT => {}
            AuraLogAuraType::MOD_ATTACKER_MELEE_HIT_CHANCE => {}
            AuraLogAuraType::MOD_ATTACKER_RANGED_HIT_CHANCE => {}
            AuraLogAuraType::MOD_ATTACKER_SPELL_HIT_CHANCE => {}
            AuraLogAuraType::MOD_ATTACKER_MELEE_CRIT_CHANCE => {}
            AuraLogAuraType::MOD_ATTACKER_RANGED_CRIT_CHANCE => {}
            AuraLogAuraType::MOD_RATING => {}
            AuraLogAuraType::MOD_FACTION_REPUTATION_GAIN => {}
            AuraLogAuraType::USE_NORMAL_MOVEMENT_SPEED => {}
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // aura_type: AuraType
        let aura_type = AuraType::astd_read(r).await?;

        let aura_type_if = match aura_type {
            AuraType::NONE => AuraLogAuraType::NONE,
            AuraType::BIND_SIGHT => AuraLogAuraType::BIND_SIGHT,
            AuraType::MOD_POSSESS => AuraLogAuraType::MOD_POSSESS,
            AuraType::PERIODIC_DAMAGE => {
                // damage1: u32
                let damage1 = crate::util::astd_read_u32_le(r).await?;

                // school: SpellSchool
                let school = SpellSchool::astd_read(r).await?;

                // absorbed: u32
                let absorbed = crate::util::astd_read_u32_le(r).await?;

                // resisted: u32
                let resisted = crate::util::astd_read_u32_le(r).await?;

                AuraLogAuraType::PERIODIC_DAMAGE {
                    damage1,
                    school,
                    absorbed,
                    resisted,
                }
            }
            AuraType::DUMMY => AuraLogAuraType::DUMMY,
            AuraType::MOD_CONFUSE => AuraLogAuraType::MOD_CONFUSE,
            AuraType::MOD_CHARM => AuraLogAuraType::MOD_CHARM,
            AuraType::MOD_FEAR => AuraLogAuraType::MOD_FEAR,
            AuraType::PERIODIC_HEAL => {
                // damage2: u32
                let damage2 = crate::util::astd_read_u32_le(r).await?;

                AuraLogAuraType::PERIODIC_HEAL {
                    damage2,
                }
            }
            AuraType::MOD_ATTACKSPEED => AuraLogAuraType::MOD_ATTACKSPEED,
            AuraType::MOD_THREAT => AuraLogAuraType::MOD_THREAT,
            AuraType::MOD_TAUNT => AuraLogAuraType::MOD_TAUNT,
            AuraType::MOD_STUN => AuraLogAuraType::MOD_STUN,
            AuraType::MOD_DAMAGE_DONE => AuraLogAuraType::MOD_DAMAGE_DONE,
            AuraType::MOD_DAMAGE_TAKEN => AuraLogAuraType::MOD_DAMAGE_TAKEN,
            AuraType::DAMAGE_SHIELD => AuraLogAuraType::DAMAGE_SHIELD,
            AuraType::MOD_STEALTH => AuraLogAuraType::MOD_STEALTH,
            AuraType::MOD_STEALTH_DETECT => AuraLogAuraType::MOD_STEALTH_DETECT,
            AuraType::MOD_INVISIBILITY => AuraLogAuraType::MOD_INVISIBILITY,
            AuraType::MOD_INVISIBILITY_DETECTION => AuraLogAuraType::MOD_INVISIBILITY_DETECTION,
            AuraType::OBS_MOD_HEALTH => {
                // damage2: u32
                let damage2 = crate::util::astd_read_u32_le(r).await?;

                AuraLogAuraType::OBS_MOD_HEALTH {
                    damage2,
                }
            }
            AuraType::OBS_MOD_MANA => {
                // misc_value1: u32
                let misc_value1 = crate::util::astd_read_u32_le(r).await?;

                // damage3: u32
                let damage3 = crate::util::astd_read_u32_le(r).await?;

                AuraLogAuraType::OBS_MOD_MANA {
                    misc_value1,
                    damage3,
                }
            }
            AuraType::MOD_RESISTANCE => AuraLogAuraType::MOD_RESISTANCE,
            AuraType::PERIODIC_TRIGGER_SPELL => AuraLogAuraType::PERIODIC_TRIGGER_SPELL,
            AuraType::PERIODIC_ENERGIZE => {
                // misc_value1: u32
                let misc_value1 = crate::util::astd_read_u32_le(r).await?;

                // damage3: u32
                let damage3 = crate::util::astd_read_u32_le(r).await?;

                AuraLogAuraType::PERIODIC_ENERGIZE {
                    misc_value1,
                    damage3,
                }
            }
            AuraType::MOD_PACIFY => AuraLogAuraType::MOD_PACIFY,
            AuraType::MOD_ROOT => AuraLogAuraType::MOD_ROOT,
            AuraType::MOD_SILENCE => AuraLogAuraType::MOD_SILENCE,
            AuraType::REFLECT_SPELLS => AuraLogAuraType::REFLECT_SPELLS,
            AuraType::MOD_STAT => AuraLogAuraType::MOD_STAT,
            AuraType::MOD_SKILL => AuraLogAuraType::MOD_SKILL,
            AuraType::MOD_INCREASE_SPEED => AuraLogAuraType::MOD_INCREASE_SPEED,
            AuraType::MOD_INCREASE_MOUNTED_SPEED => AuraLogAuraType::MOD_INCREASE_MOUNTED_SPEED,
            AuraType::MOD_DECREASE_SPEED => AuraLogAuraType::MOD_DECREASE_SPEED,
            AuraType::MOD_INCREASE_HEALTH => AuraLogAuraType::MOD_INCREASE_HEALTH,
            AuraType::MOD_INCREASE_ENERGY => AuraLogAuraType::MOD_INCREASE_ENERGY,
            AuraType::MOD_SHAPESHIFT => AuraLogAuraType::MOD_SHAPESHIFT,
            AuraType::EFFECT_IMMUNITY => AuraLogAuraType::EFFECT_IMMUNITY,
            AuraType::STATE_IMMUNITY => AuraLogAuraType::STATE_IMMUNITY,
            AuraType::SCHOOL_IMMUNITY => AuraLogAuraType::SCHOOL_IMMUNITY,
            AuraType::DAMAGE_IMMUNITY => AuraLogAuraType::DAMAGE_IMMUNITY,
            AuraType::DISPEL_IMMUNITY => AuraLogAuraType::DISPEL_IMMUNITY,
            AuraType::PROC_TRIGGER_SPELL => AuraLogAuraType::PROC_TRIGGER_SPELL,
            AuraType::PROC_TRIGGER_DAMAGE => AuraLogAuraType::PROC_TRIGGER_DAMAGE,
            AuraType::TRACK_CREATURES => AuraLogAuraType::TRACK_CREATURES,
            AuraType::TRACK_RESOURCES => AuraLogAuraType::TRACK_RESOURCES,
            AuraType::UNKNOWN46 => AuraLogAuraType::UNKNOWN46,
            AuraType::MOD_PARRY_PERCENT => AuraLogAuraType::MOD_PARRY_PERCENT,
            AuraType::UNKNOWN48 => AuraLogAuraType::UNKNOWN48,
            AuraType::MOD_DODGE_PERCENT => AuraLogAuraType::MOD_DODGE_PERCENT,
            AuraType::MOD_BLOCK_SKILL => AuraLogAuraType::MOD_BLOCK_SKILL,
            AuraType::MOD_BLOCK_PERCENT => AuraLogAuraType::MOD_BLOCK_PERCENT,
            AuraType::MOD_CRIT_PERCENT => AuraLogAuraType::MOD_CRIT_PERCENT,
            AuraType::PERIODIC_LEECH => AuraLogAuraType::PERIODIC_LEECH,
            AuraType::MOD_HIT_CHANCE => AuraLogAuraType::MOD_HIT_CHANCE,
            AuraType::MOD_SPELL_HIT_CHANCE => AuraLogAuraType::MOD_SPELL_HIT_CHANCE,
            AuraType::TRANSFORM => AuraLogAuraType::TRANSFORM,
            AuraType::MOD_SPELL_CRIT_CHANCE => AuraLogAuraType::MOD_SPELL_CRIT_CHANCE,
            AuraType::MOD_INCREASE_SWIM_SPEED => AuraLogAuraType::MOD_INCREASE_SWIM_SPEED,
            AuraType::MOD_DAMAGE_DONE_CREATURE => AuraLogAuraType::MOD_DAMAGE_DONE_CREATURE,
            AuraType::MOD_PACIFY_SILENCE => AuraLogAuraType::MOD_PACIFY_SILENCE,
            AuraType::MOD_SCALE => AuraLogAuraType::MOD_SCALE,
            AuraType::PERIODIC_HEALTH_FUNNEL => AuraLogAuraType::PERIODIC_HEALTH_FUNNEL,
            AuraType::PERIODIC_MANA_FUNNEL => AuraLogAuraType::PERIODIC_MANA_FUNNEL,
            AuraType::PERIODIC_MANA_LEECH => {
                // misc_value2: u32
                let misc_value2 = crate::util::astd_read_u32_le(r).await?;

                // damage: u32
                let damage = crate::util::astd_read_u32_le(r).await?;

                // gain_multiplier: f32
                let gain_multiplier = crate::util::astd_read_f32_le(r).await?;
                AuraLogAuraType::PERIODIC_MANA_LEECH {
                    misc_value2,
                    damage,
                    gain_multiplier,
                }
            }
            AuraType::MOD_CASTING_SPEED_NOT_STACK => AuraLogAuraType::MOD_CASTING_SPEED_NOT_STACK,
            AuraType::FEIGN_DEATH => AuraLogAuraType::FEIGN_DEATH,
            AuraType::MOD_DISARM => AuraLogAuraType::MOD_DISARM,
            AuraType::MOD_STALKED => AuraLogAuraType::MOD_STALKED,
            AuraType::SCHOOL_ABSORB => AuraLogAuraType::SCHOOL_ABSORB,
            AuraType::EXTRA_ATTACKS => AuraLogAuraType::EXTRA_ATTACKS,
            AuraType::MOD_SPELL_CRIT_CHANCE_SCHOOL => AuraLogAuraType::MOD_SPELL_CRIT_CHANCE_SCHOOL,
            AuraType::MOD_POWER_COST_SCHOOL_PCT => AuraLogAuraType::MOD_POWER_COST_SCHOOL_PCT,
            AuraType::MOD_POWER_COST_SCHOOL => AuraLogAuraType::MOD_POWER_COST_SCHOOL,
            AuraType::REFLECT_SPELLS_SCHOOL => AuraLogAuraType::REFLECT_SPELLS_SCHOOL,
            AuraType::MOD_LANGUAGE => AuraLogAuraType::MOD_LANGUAGE,
            AuraType::FAR_SIGHT => AuraLogAuraType::FAR_SIGHT,
            AuraType::MECHANIC_IMMUNITY => AuraLogAuraType::MECHANIC_IMMUNITY,
            AuraType::MOUNTED => AuraLogAuraType::MOUNTED,
            AuraType::MOD_DAMAGE_PERCENT_DONE => AuraLogAuraType::MOD_DAMAGE_PERCENT_DONE,
            AuraType::MOD_PERCENT_STAT => AuraLogAuraType::MOD_PERCENT_STAT,
            AuraType::SPLIT_DAMAGE_PCT => AuraLogAuraType::SPLIT_DAMAGE_PCT,
            AuraType::WATER_BREATHING => AuraLogAuraType::WATER_BREATHING,
            AuraType::MOD_BASE_RESISTANCE => AuraLogAuraType::MOD_BASE_RESISTANCE,
            AuraType::MOD_REGEN => AuraLogAuraType::MOD_REGEN,
            AuraType::MOD_POWER_REGEN => AuraLogAuraType::MOD_POWER_REGEN,
            AuraType::CHANNEL_DEATH_ITEM => AuraLogAuraType::CHANNEL_DEATH_ITEM,
            AuraType::MOD_DAMAGE_PERCENT_TAKEN => AuraLogAuraType::MOD_DAMAGE_PERCENT_TAKEN,
            AuraType::MOD_HEALTH_REGEN_PERCENT => AuraLogAuraType::MOD_HEALTH_REGEN_PERCENT,
            AuraType::PERIODIC_DAMAGE_PERCENT => {
                // damage1: u32
                let damage1 = crate::util::astd_read_u32_le(r).await?;

                // school: SpellSchool
                let school = SpellSchool::astd_read(r).await?;

                // absorbed: u32
                let absorbed = crate::util::astd_read_u32_le(r).await?;

                // resisted: u32
                let resisted = crate::util::astd_read_u32_le(r).await?;

                AuraLogAuraType::PERIODIC_DAMAGE_PERCENT {
                    damage1,
                    school,
                    absorbed,
                    resisted,
                }
            }
            AuraType::MOD_RESIST_CHANCE => AuraLogAuraType::MOD_RESIST_CHANCE,
            AuraType::MOD_DETECT_RANGE => AuraLogAuraType::MOD_DETECT_RANGE,
            AuraType::PREVENTS_FLEEING => AuraLogAuraType::PREVENTS_FLEEING,
            AuraType::MOD_UNATTACKABLE => AuraLogAuraType::MOD_UNATTACKABLE,
            AuraType::INTERRUPT_REGEN => AuraLogAuraType::INTERRUPT_REGEN,
            AuraType::GHOST => AuraLogAuraType::GHOST,
            AuraType::SPELL_MAGNET => AuraLogAuraType::SPELL_MAGNET,
            AuraType::MANA_SHIELD => AuraLogAuraType::MANA_SHIELD,
            AuraType::MOD_SKILL_TALENT => AuraLogAuraType::MOD_SKILL_TALENT,
            AuraType::MOD_ATTACK_POWER => AuraLogAuraType::MOD_ATTACK_POWER,
            AuraType::AURAS_VISIBLE => AuraLogAuraType::AURAS_VISIBLE,
            AuraType::MOD_RESISTANCE_PCT => AuraLogAuraType::MOD_RESISTANCE_PCT,
            AuraType::MOD_MELEE_ATTACK_POWER_VERSUS => AuraLogAuraType::MOD_MELEE_ATTACK_POWER_VERSUS,
            AuraType::MOD_TOTAL_THREAT => AuraLogAuraType::MOD_TOTAL_THREAT,
            AuraType::WATER_WALK => AuraLogAuraType::WATER_WALK,
            AuraType::FEATHER_FALL => AuraLogAuraType::FEATHER_FALL,
            AuraType::HOVER => AuraLogAuraType::HOVER,
            AuraType::ADD_FLAT_MODIFIER => AuraLogAuraType::ADD_FLAT_MODIFIER,
            AuraType::ADD_PCT_MODIFIER => AuraLogAuraType::ADD_PCT_MODIFIER,
            AuraType::ADD_TARGET_TRIGGER => AuraLogAuraType::ADD_TARGET_TRIGGER,
            AuraType::MOD_POWER_REGEN_PERCENT => AuraLogAuraType::MOD_POWER_REGEN_PERCENT,
            AuraType::ADD_CASTER_HIT_TRIGGER => AuraLogAuraType::ADD_CASTER_HIT_TRIGGER,
            AuraType::OVERRIDE_CLASS_SCRIPTS => AuraLogAuraType::OVERRIDE_CLASS_SCRIPTS,
            AuraType::MOD_RANGED_DAMAGE_TAKEN => AuraLogAuraType::MOD_RANGED_DAMAGE_TAKEN,
            AuraType::MOD_RANGED_DAMAGE_TAKEN_PCT => AuraLogAuraType::MOD_RANGED_DAMAGE_TAKEN_PCT,
            AuraType::MOD_HEALING => AuraLogAuraType::MOD_HEALING,
            AuraType::MOD_REGEN_DURING_COMBAT => AuraLogAuraType::MOD_REGEN_DURING_COMBAT,
            AuraType::MOD_MECHANIC_RESISTANCE => AuraLogAuraType::MOD_MECHANIC_RESISTANCE,
            AuraType::MOD_HEALING_PCT => AuraLogAuraType::MOD_HEALING_PCT,
            AuraType::SHARE_PET_TRACKING => AuraLogAuraType::SHARE_PET_TRACKING,
            AuraType::UNTRACKABLE => AuraLogAuraType::UNTRACKABLE,
            AuraType::EMPATHY => AuraLogAuraType::EMPATHY,
            AuraType::MOD_OFFHAND_DAMAGE_PCT => AuraLogAuraType::MOD_OFFHAND_DAMAGE_PCT,
            AuraType::MOD_TARGET_RESISTANCE => AuraLogAuraType::MOD_TARGET_RESISTANCE,
            AuraType::MOD_RANGED_ATTACK_POWER => AuraLogAuraType::MOD_RANGED_ATTACK_POWER,
            AuraType::MOD_MELEE_DAMAGE_TAKEN => AuraLogAuraType::MOD_MELEE_DAMAGE_TAKEN,
            AuraType::MOD_MELEE_DAMAGE_TAKEN_PCT => AuraLogAuraType::MOD_MELEE_DAMAGE_TAKEN_PCT,
            AuraType::RANGED_ATTACK_POWER_ATTACKER_BONUS => AuraLogAuraType::RANGED_ATTACK_POWER_ATTACKER_BONUS,
            AuraType::MOD_POSSESS_PET => AuraLogAuraType::MOD_POSSESS_PET,
            AuraType::MOD_SPEED_ALWAYS => AuraLogAuraType::MOD_SPEED_ALWAYS,
            AuraType::MOD_MOUNTED_SPEED_ALWAYS => AuraLogAuraType::MOD_MOUNTED_SPEED_ALWAYS,
            AuraType::MOD_RANGED_ATTACK_POWER_VERSUS => AuraLogAuraType::MOD_RANGED_ATTACK_POWER_VERSUS,
            AuraType::MOD_INCREASE_ENERGY_PERCENT => AuraLogAuraType::MOD_INCREASE_ENERGY_PERCENT,
            AuraType::MOD_INCREASE_HEALTH_PERCENT => AuraLogAuraType::MOD_INCREASE_HEALTH_PERCENT,
            AuraType::MOD_MANA_REGEN_INTERRUPT => AuraLogAuraType::MOD_MANA_REGEN_INTERRUPT,
            AuraType::MOD_HEALING_DONE => AuraLogAuraType::MOD_HEALING_DONE,
            AuraType::MOD_HEALING_DONE_PERCENT => AuraLogAuraType::MOD_HEALING_DONE_PERCENT,
            AuraType::MOD_TOTAL_STAT_PERCENTAGE => AuraLogAuraType::MOD_TOTAL_STAT_PERCENTAGE,
            AuraType::MOD_MELEE_HASTE => AuraLogAuraType::MOD_MELEE_HASTE,
            AuraType::FORCE_REACTION => AuraLogAuraType::FORCE_REACTION,
            AuraType::MOD_RANGED_HASTE => AuraLogAuraType::MOD_RANGED_HASTE,
            AuraType::MOD_RANGED_AMMO_HASTE => AuraLogAuraType::MOD_RANGED_AMMO_HASTE,
            AuraType::MOD_BASE_RESISTANCE_PCT => AuraLogAuraType::MOD_BASE_RESISTANCE_PCT,
            AuraType::MOD_RESISTANCE_EXCLUSIVE => AuraLogAuraType::MOD_RESISTANCE_EXCLUSIVE,
            AuraType::SAFE_FALL => AuraLogAuraType::SAFE_FALL,
            AuraType::CHARISMA => AuraLogAuraType::CHARISMA,
            AuraType::PERSUADED => AuraLogAuraType::PERSUADED,
            AuraType::MECHANIC_IMMUNITY_MASK => AuraLogAuraType::MECHANIC_IMMUNITY_MASK,
            AuraType::RETAIN_COMBO_POINTS => AuraLogAuraType::RETAIN_COMBO_POINTS,
            AuraType::RESIST_PUSHBACK => AuraLogAuraType::RESIST_PUSHBACK,
            AuraType::MOD_SHIELD_BLOCKVALUE_PCT => AuraLogAuraType::MOD_SHIELD_BLOCKVALUE_PCT,
            AuraType::TRACK_STEALTHED => AuraLogAuraType::TRACK_STEALTHED,
            AuraType::MOD_DETECTED_RANGE => AuraLogAuraType::MOD_DETECTED_RANGE,
            AuraType::SPLIT_DAMAGE_FLAT => AuraLogAuraType::SPLIT_DAMAGE_FLAT,
            AuraType::MOD_STEALTH_LEVEL => AuraLogAuraType::MOD_STEALTH_LEVEL,
            AuraType::MOD_WATER_BREATHING => AuraLogAuraType::MOD_WATER_BREATHING,
            AuraType::MOD_REPUTATION_GAIN => AuraLogAuraType::MOD_REPUTATION_GAIN,
            AuraType::PET_DAMAGE_MULTI => AuraLogAuraType::PET_DAMAGE_MULTI,
            AuraType::MOD_SHIELD_BLOCKVALUE => AuraLogAuraType::MOD_SHIELD_BLOCKVALUE,
            AuraType::NO_PVP_CREDIT => AuraLogAuraType::NO_PVP_CREDIT,
            AuraType::MOD_AOE_AVOIDANCE => AuraLogAuraType::MOD_AOE_AVOIDANCE,
            AuraType::MOD_HEALTH_REGEN_IN_COMBAT => AuraLogAuraType::MOD_HEALTH_REGEN_IN_COMBAT,
            AuraType::POWER_BURN_MANA => AuraLogAuraType::POWER_BURN_MANA,
            AuraType::MOD_CRIT_DAMAGE_BONUS => AuraLogAuraType::MOD_CRIT_DAMAGE_BONUS,
            AuraType::UNKNOWN164 => AuraLogAuraType::UNKNOWN164,
            AuraType::MELEE_ATTACK_POWER_ATTACKER_BONUS => AuraLogAuraType::MELEE_ATTACK_POWER_ATTACKER_BONUS,
            AuraType::MOD_ATTACK_POWER_PCT => AuraLogAuraType::MOD_ATTACK_POWER_PCT,
            AuraType::MOD_RANGED_ATTACK_POWER_PCT => AuraLogAuraType::MOD_RANGED_ATTACK_POWER_PCT,
            AuraType::MOD_DAMAGE_DONE_VERSUS => AuraLogAuraType::MOD_DAMAGE_DONE_VERSUS,
            AuraType::MOD_CRIT_PERCENT_VERSUS => AuraLogAuraType::MOD_CRIT_PERCENT_VERSUS,
            AuraType::DETECT_AMORE => AuraLogAuraType::DETECT_AMORE,
            AuraType::MOD_SPEED_NOT_STACK => AuraLogAuraType::MOD_SPEED_NOT_STACK,
            AuraType::MOD_MOUNTED_SPEED_NOT_STACK => AuraLogAuraType::MOD_MOUNTED_SPEED_NOT_STACK,
            AuraType::ALLOW_CHAMPION_SPELLS => AuraLogAuraType::ALLOW_CHAMPION_SPELLS,
            AuraType::MOD_SPELL_DAMAGE_OF_STAT_PERCENT => AuraLogAuraType::MOD_SPELL_DAMAGE_OF_STAT_PERCENT,
            AuraType::MOD_SPELL_HEALING_OF_STAT_PERCENT => AuraLogAuraType::MOD_SPELL_HEALING_OF_STAT_PERCENT,
            AuraType::SPIRIT_OF_REDEMPTION => AuraLogAuraType::SPIRIT_OF_REDEMPTION,
            AuraType::AOE_CHARM => AuraLogAuraType::AOE_CHARM,
            AuraType::MOD_DEBUFF_RESISTANCE => AuraLogAuraType::MOD_DEBUFF_RESISTANCE,
            AuraType::MOD_ATTACKER_SPELL_CRIT_CHANCE => AuraLogAuraType::MOD_ATTACKER_SPELL_CRIT_CHANCE,
            AuraType::MOD_FLAT_SPELL_DAMAGE_VERSUS => AuraLogAuraType::MOD_FLAT_SPELL_DAMAGE_VERSUS,
            AuraType::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS => AuraLogAuraType::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS,
            AuraType::MOD_RESISTANCE_OF_STAT_PERCENT => AuraLogAuraType::MOD_RESISTANCE_OF_STAT_PERCENT,
            AuraType::MOD_CRITICAL_THREAT => AuraLogAuraType::MOD_CRITICAL_THREAT,
            AuraType::MOD_ATTACKER_MELEE_HIT_CHANCE => AuraLogAuraType::MOD_ATTACKER_MELEE_HIT_CHANCE,
            AuraType::MOD_ATTACKER_RANGED_HIT_CHANCE => AuraLogAuraType::MOD_ATTACKER_RANGED_HIT_CHANCE,
            AuraType::MOD_ATTACKER_SPELL_HIT_CHANCE => AuraLogAuraType::MOD_ATTACKER_SPELL_HIT_CHANCE,
            AuraType::MOD_ATTACKER_MELEE_CRIT_CHANCE => AuraLogAuraType::MOD_ATTACKER_MELEE_CRIT_CHANCE,
            AuraType::MOD_ATTACKER_RANGED_CRIT_CHANCE => AuraLogAuraType::MOD_ATTACKER_RANGED_CRIT_CHANCE,
            AuraType::MOD_RATING => AuraLogAuraType::MOD_RATING,
            AuraType::MOD_FACTION_REPUTATION_GAIN => AuraLogAuraType::MOD_FACTION_REPUTATION_GAIN,
            AuraType::USE_NORMAL_MOVEMENT_SPEED => AuraLogAuraType::USE_NORMAL_MOVEMENT_SPEED,
        };

        Ok(Self {
            aura_type: aura_type_if,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // aura_type: AuraType
        self.aura_type.astd_write(w).await?;

        match &self.aura_type {
            AuraLogAuraType::NONE => {}
            AuraLogAuraType::BIND_SIGHT => {}
            AuraLogAuraType::MOD_POSSESS => {}
            AuraLogAuraType::PERIODIC_DAMAGE {
                damage1,
                school,
                absorbed,
                resisted,
            } => {
                // damage1: u32
                w.write_all(&damage1.to_le_bytes()).await?;

                // school: SpellSchool
                school.astd_write(w).await?;

                // absorbed: u32
                w.write_all(&absorbed.to_le_bytes()).await?;

                // resisted: u32
                w.write_all(&resisted.to_le_bytes()).await?;

            }
            AuraLogAuraType::DUMMY => {}
            AuraLogAuraType::MOD_CONFUSE => {}
            AuraLogAuraType::MOD_CHARM => {}
            AuraLogAuraType::MOD_FEAR => {}
            AuraLogAuraType::PERIODIC_HEAL {
                damage2,
            } => {
                // damage2: u32
                w.write_all(&damage2.to_le_bytes()).await?;

            }
            AuraLogAuraType::MOD_ATTACKSPEED => {}
            AuraLogAuraType::MOD_THREAT => {}
            AuraLogAuraType::MOD_TAUNT => {}
            AuraLogAuraType::MOD_STUN => {}
            AuraLogAuraType::MOD_DAMAGE_DONE => {}
            AuraLogAuraType::MOD_DAMAGE_TAKEN => {}
            AuraLogAuraType::DAMAGE_SHIELD => {}
            AuraLogAuraType::MOD_STEALTH => {}
            AuraLogAuraType::MOD_STEALTH_DETECT => {}
            AuraLogAuraType::MOD_INVISIBILITY => {}
            AuraLogAuraType::MOD_INVISIBILITY_DETECTION => {}
            AuraLogAuraType::OBS_MOD_HEALTH {
                damage2,
            } => {
                // damage2: u32
                w.write_all(&damage2.to_le_bytes()).await?;

            }
            AuraLogAuraType::OBS_MOD_MANA {
                misc_value1,
                damage3,
            } => {
                // misc_value1: u32
                w.write_all(&misc_value1.to_le_bytes()).await?;

                // damage3: u32
                w.write_all(&damage3.to_le_bytes()).await?;

            }
            AuraLogAuraType::MOD_RESISTANCE => {}
            AuraLogAuraType::PERIODIC_TRIGGER_SPELL => {}
            AuraLogAuraType::PERIODIC_ENERGIZE {
                misc_value1,
                damage3,
            } => {
                // misc_value1: u32
                w.write_all(&misc_value1.to_le_bytes()).await?;

                // damage3: u32
                w.write_all(&damage3.to_le_bytes()).await?;

            }
            AuraLogAuraType::MOD_PACIFY => {}
            AuraLogAuraType::MOD_ROOT => {}
            AuraLogAuraType::MOD_SILENCE => {}
            AuraLogAuraType::REFLECT_SPELLS => {}
            AuraLogAuraType::MOD_STAT => {}
            AuraLogAuraType::MOD_SKILL => {}
            AuraLogAuraType::MOD_INCREASE_SPEED => {}
            AuraLogAuraType::MOD_INCREASE_MOUNTED_SPEED => {}
            AuraLogAuraType::MOD_DECREASE_SPEED => {}
            AuraLogAuraType::MOD_INCREASE_HEALTH => {}
            AuraLogAuraType::MOD_INCREASE_ENERGY => {}
            AuraLogAuraType::MOD_SHAPESHIFT => {}
            AuraLogAuraType::EFFECT_IMMUNITY => {}
            AuraLogAuraType::STATE_IMMUNITY => {}
            AuraLogAuraType::SCHOOL_IMMUNITY => {}
            AuraLogAuraType::DAMAGE_IMMUNITY => {}
            AuraLogAuraType::DISPEL_IMMUNITY => {}
            AuraLogAuraType::PROC_TRIGGER_SPELL => {}
            AuraLogAuraType::PROC_TRIGGER_DAMAGE => {}
            AuraLogAuraType::TRACK_CREATURES => {}
            AuraLogAuraType::TRACK_RESOURCES => {}
            AuraLogAuraType::UNKNOWN46 => {}
            AuraLogAuraType::MOD_PARRY_PERCENT => {}
            AuraLogAuraType::UNKNOWN48 => {}
            AuraLogAuraType::MOD_DODGE_PERCENT => {}
            AuraLogAuraType::MOD_BLOCK_SKILL => {}
            AuraLogAuraType::MOD_BLOCK_PERCENT => {}
            AuraLogAuraType::MOD_CRIT_PERCENT => {}
            AuraLogAuraType::PERIODIC_LEECH => {}
            AuraLogAuraType::MOD_HIT_CHANCE => {}
            AuraLogAuraType::MOD_SPELL_HIT_CHANCE => {}
            AuraLogAuraType::TRANSFORM => {}
            AuraLogAuraType::MOD_SPELL_CRIT_CHANCE => {}
            AuraLogAuraType::MOD_INCREASE_SWIM_SPEED => {}
            AuraLogAuraType::MOD_DAMAGE_DONE_CREATURE => {}
            AuraLogAuraType::MOD_PACIFY_SILENCE => {}
            AuraLogAuraType::MOD_SCALE => {}
            AuraLogAuraType::PERIODIC_HEALTH_FUNNEL => {}
            AuraLogAuraType::PERIODIC_MANA_FUNNEL => {}
            AuraLogAuraType::PERIODIC_MANA_LEECH {
                misc_value2,
                damage,
                gain_multiplier,
            } => {
                // misc_value2: u32
                w.write_all(&misc_value2.to_le_bytes()).await?;

                // damage: u32
                w.write_all(&damage.to_le_bytes()).await?;

                // gain_multiplier: f32
                w.write_all(&gain_multiplier.to_le_bytes()).await?;

            }
            AuraLogAuraType::MOD_CASTING_SPEED_NOT_STACK => {}
            AuraLogAuraType::FEIGN_DEATH => {}
            AuraLogAuraType::MOD_DISARM => {}
            AuraLogAuraType::MOD_STALKED => {}
            AuraLogAuraType::SCHOOL_ABSORB => {}
            AuraLogAuraType::EXTRA_ATTACKS => {}
            AuraLogAuraType::MOD_SPELL_CRIT_CHANCE_SCHOOL => {}
            AuraLogAuraType::MOD_POWER_COST_SCHOOL_PCT => {}
            AuraLogAuraType::MOD_POWER_COST_SCHOOL => {}
            AuraLogAuraType::REFLECT_SPELLS_SCHOOL => {}
            AuraLogAuraType::MOD_LANGUAGE => {}
            AuraLogAuraType::FAR_SIGHT => {}
            AuraLogAuraType::MECHANIC_IMMUNITY => {}
            AuraLogAuraType::MOUNTED => {}
            AuraLogAuraType::MOD_DAMAGE_PERCENT_DONE => {}
            AuraLogAuraType::MOD_PERCENT_STAT => {}
            AuraLogAuraType::SPLIT_DAMAGE_PCT => {}
            AuraLogAuraType::WATER_BREATHING => {}
            AuraLogAuraType::MOD_BASE_RESISTANCE => {}
            AuraLogAuraType::MOD_REGEN => {}
            AuraLogAuraType::MOD_POWER_REGEN => {}
            AuraLogAuraType::CHANNEL_DEATH_ITEM => {}
            AuraLogAuraType::MOD_DAMAGE_PERCENT_TAKEN => {}
            AuraLogAuraType::MOD_HEALTH_REGEN_PERCENT => {}
            AuraLogAuraType::PERIODIC_DAMAGE_PERCENT {
                damage1,
                school,
                absorbed,
                resisted,
            } => {
                // damage1: u32
                w.write_all(&damage1.to_le_bytes()).await?;

                // school: SpellSchool
                school.astd_write(w).await?;

                // absorbed: u32
                w.write_all(&absorbed.to_le_bytes()).await?;

                // resisted: u32
                w.write_all(&resisted.to_le_bytes()).await?;

            }
            AuraLogAuraType::MOD_RESIST_CHANCE => {}
            AuraLogAuraType::MOD_DETECT_RANGE => {}
            AuraLogAuraType::PREVENTS_FLEEING => {}
            AuraLogAuraType::MOD_UNATTACKABLE => {}
            AuraLogAuraType::INTERRUPT_REGEN => {}
            AuraLogAuraType::GHOST => {}
            AuraLogAuraType::SPELL_MAGNET => {}
            AuraLogAuraType::MANA_SHIELD => {}
            AuraLogAuraType::MOD_SKILL_TALENT => {}
            AuraLogAuraType::MOD_ATTACK_POWER => {}
            AuraLogAuraType::AURAS_VISIBLE => {}
            AuraLogAuraType::MOD_RESISTANCE_PCT => {}
            AuraLogAuraType::MOD_MELEE_ATTACK_POWER_VERSUS => {}
            AuraLogAuraType::MOD_TOTAL_THREAT => {}
            AuraLogAuraType::WATER_WALK => {}
            AuraLogAuraType::FEATHER_FALL => {}
            AuraLogAuraType::HOVER => {}
            AuraLogAuraType::ADD_FLAT_MODIFIER => {}
            AuraLogAuraType::ADD_PCT_MODIFIER => {}
            AuraLogAuraType::ADD_TARGET_TRIGGER => {}
            AuraLogAuraType::MOD_POWER_REGEN_PERCENT => {}
            AuraLogAuraType::ADD_CASTER_HIT_TRIGGER => {}
            AuraLogAuraType::OVERRIDE_CLASS_SCRIPTS => {}
            AuraLogAuraType::MOD_RANGED_DAMAGE_TAKEN => {}
            AuraLogAuraType::MOD_RANGED_DAMAGE_TAKEN_PCT => {}
            AuraLogAuraType::MOD_HEALING => {}
            AuraLogAuraType::MOD_REGEN_DURING_COMBAT => {}
            AuraLogAuraType::MOD_MECHANIC_RESISTANCE => {}
            AuraLogAuraType::MOD_HEALING_PCT => {}
            AuraLogAuraType::SHARE_PET_TRACKING => {}
            AuraLogAuraType::UNTRACKABLE => {}
            AuraLogAuraType::EMPATHY => {}
            AuraLogAuraType::MOD_OFFHAND_DAMAGE_PCT => {}
            AuraLogAuraType::MOD_TARGET_RESISTANCE => {}
            AuraLogAuraType::MOD_RANGED_ATTACK_POWER => {}
            AuraLogAuraType::MOD_MELEE_DAMAGE_TAKEN => {}
            AuraLogAuraType::MOD_MELEE_DAMAGE_TAKEN_PCT => {}
            AuraLogAuraType::RANGED_ATTACK_POWER_ATTACKER_BONUS => {}
            AuraLogAuraType::MOD_POSSESS_PET => {}
            AuraLogAuraType::MOD_SPEED_ALWAYS => {}
            AuraLogAuraType::MOD_MOUNTED_SPEED_ALWAYS => {}
            AuraLogAuraType::MOD_RANGED_ATTACK_POWER_VERSUS => {}
            AuraLogAuraType::MOD_INCREASE_ENERGY_PERCENT => {}
            AuraLogAuraType::MOD_INCREASE_HEALTH_PERCENT => {}
            AuraLogAuraType::MOD_MANA_REGEN_INTERRUPT => {}
            AuraLogAuraType::MOD_HEALING_DONE => {}
            AuraLogAuraType::MOD_HEALING_DONE_PERCENT => {}
            AuraLogAuraType::MOD_TOTAL_STAT_PERCENTAGE => {}
            AuraLogAuraType::MOD_MELEE_HASTE => {}
            AuraLogAuraType::FORCE_REACTION => {}
            AuraLogAuraType::MOD_RANGED_HASTE => {}
            AuraLogAuraType::MOD_RANGED_AMMO_HASTE => {}
            AuraLogAuraType::MOD_BASE_RESISTANCE_PCT => {}
            AuraLogAuraType::MOD_RESISTANCE_EXCLUSIVE => {}
            AuraLogAuraType::SAFE_FALL => {}
            AuraLogAuraType::CHARISMA => {}
            AuraLogAuraType::PERSUADED => {}
            AuraLogAuraType::MECHANIC_IMMUNITY_MASK => {}
            AuraLogAuraType::RETAIN_COMBO_POINTS => {}
            AuraLogAuraType::RESIST_PUSHBACK => {}
            AuraLogAuraType::MOD_SHIELD_BLOCKVALUE_PCT => {}
            AuraLogAuraType::TRACK_STEALTHED => {}
            AuraLogAuraType::MOD_DETECTED_RANGE => {}
            AuraLogAuraType::SPLIT_DAMAGE_FLAT => {}
            AuraLogAuraType::MOD_STEALTH_LEVEL => {}
            AuraLogAuraType::MOD_WATER_BREATHING => {}
            AuraLogAuraType::MOD_REPUTATION_GAIN => {}
            AuraLogAuraType::PET_DAMAGE_MULTI => {}
            AuraLogAuraType::MOD_SHIELD_BLOCKVALUE => {}
            AuraLogAuraType::NO_PVP_CREDIT => {}
            AuraLogAuraType::MOD_AOE_AVOIDANCE => {}
            AuraLogAuraType::MOD_HEALTH_REGEN_IN_COMBAT => {}
            AuraLogAuraType::POWER_BURN_MANA => {}
            AuraLogAuraType::MOD_CRIT_DAMAGE_BONUS => {}
            AuraLogAuraType::UNKNOWN164 => {}
            AuraLogAuraType::MELEE_ATTACK_POWER_ATTACKER_BONUS => {}
            AuraLogAuraType::MOD_ATTACK_POWER_PCT => {}
            AuraLogAuraType::MOD_RANGED_ATTACK_POWER_PCT => {}
            AuraLogAuraType::MOD_DAMAGE_DONE_VERSUS => {}
            AuraLogAuraType::MOD_CRIT_PERCENT_VERSUS => {}
            AuraLogAuraType::DETECT_AMORE => {}
            AuraLogAuraType::MOD_SPEED_NOT_STACK => {}
            AuraLogAuraType::MOD_MOUNTED_SPEED_NOT_STACK => {}
            AuraLogAuraType::ALLOW_CHAMPION_SPELLS => {}
            AuraLogAuraType::MOD_SPELL_DAMAGE_OF_STAT_PERCENT => {}
            AuraLogAuraType::MOD_SPELL_HEALING_OF_STAT_PERCENT => {}
            AuraLogAuraType::SPIRIT_OF_REDEMPTION => {}
            AuraLogAuraType::AOE_CHARM => {}
            AuraLogAuraType::MOD_DEBUFF_RESISTANCE => {}
            AuraLogAuraType::MOD_ATTACKER_SPELL_CRIT_CHANCE => {}
            AuraLogAuraType::MOD_FLAT_SPELL_DAMAGE_VERSUS => {}
            AuraLogAuraType::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS => {}
            AuraLogAuraType::MOD_RESISTANCE_OF_STAT_PERCENT => {}
            AuraLogAuraType::MOD_CRITICAL_THREAT => {}
            AuraLogAuraType::MOD_ATTACKER_MELEE_HIT_CHANCE => {}
            AuraLogAuraType::MOD_ATTACKER_RANGED_HIT_CHANCE => {}
            AuraLogAuraType::MOD_ATTACKER_SPELL_HIT_CHANCE => {}
            AuraLogAuraType::MOD_ATTACKER_MELEE_CRIT_CHANCE => {}
            AuraLogAuraType::MOD_ATTACKER_RANGED_CRIT_CHANCE => {}
            AuraLogAuraType::MOD_RATING => {}
            AuraLogAuraType::MOD_FACTION_REPUTATION_GAIN => {}
            AuraLogAuraType::USE_NORMAL_MOVEMENT_SPEED => {}
        }

        Ok(())
    }

}

impl VariableSized for AuraLog {
    fn size(&self) -> usize {
        0
        + self.aura_type.size() // aura_type: AuraLogAuraType
    }
}

impl MaximumPossibleSized for AuraLog {
    fn maximum_possible_size() -> usize {
        0
        + 17 // aura_type: AuraLogAuraType
    }
}

#[derive(Debug)]
pub enum AuraLogError {
    Io(std::io::Error),
    AuraType(AuraTypeError),
    SpellSchool(SpellSchoolError),
}

impl std::error::Error for AuraLogError {}
impl std::fmt::Display for AuraLogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::AuraType(i) => i.fmt(f),
            Self::SpellSchool(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for AuraLogError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<AuraTypeError> for AuraLogError {
    fn from(e: AuraTypeError) -> Self {
        Self::AuraType(e)
    }
}

impl From<SpellSchoolError> for AuraLogError {
    fn from(e: SpellSchoolError) -> Self {
        Self::SpellSchool(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AuraLogAuraType {
    NONE,
    BIND_SIGHT,
    MOD_POSSESS,
    PERIODIC_DAMAGE {
        damage1: u32,
        school: SpellSchool,
        absorbed: u32,
        resisted: u32,
    },
    DUMMY,
    MOD_CONFUSE,
    MOD_CHARM,
    MOD_FEAR,
    PERIODIC_HEAL {
        damage2: u32,
    },
    MOD_ATTACKSPEED,
    MOD_THREAT,
    MOD_TAUNT,
    MOD_STUN,
    MOD_DAMAGE_DONE,
    MOD_DAMAGE_TAKEN,
    DAMAGE_SHIELD,
    MOD_STEALTH,
    MOD_STEALTH_DETECT,
    MOD_INVISIBILITY,
    MOD_INVISIBILITY_DETECTION,
    OBS_MOD_HEALTH {
        damage2: u32,
    },
    OBS_MOD_MANA {
        misc_value1: u32,
        damage3: u32,
    },
    MOD_RESISTANCE,
    PERIODIC_TRIGGER_SPELL,
    PERIODIC_ENERGIZE {
        misc_value1: u32,
        damage3: u32,
    },
    MOD_PACIFY,
    MOD_ROOT,
    MOD_SILENCE,
    REFLECT_SPELLS,
    MOD_STAT,
    MOD_SKILL,
    MOD_INCREASE_SPEED,
    MOD_INCREASE_MOUNTED_SPEED,
    MOD_DECREASE_SPEED,
    MOD_INCREASE_HEALTH,
    MOD_INCREASE_ENERGY,
    MOD_SHAPESHIFT,
    EFFECT_IMMUNITY,
    STATE_IMMUNITY,
    SCHOOL_IMMUNITY,
    DAMAGE_IMMUNITY,
    DISPEL_IMMUNITY,
    PROC_TRIGGER_SPELL,
    PROC_TRIGGER_DAMAGE,
    TRACK_CREATURES,
    TRACK_RESOURCES,
    UNKNOWN46,
    MOD_PARRY_PERCENT,
    UNKNOWN48,
    MOD_DODGE_PERCENT,
    MOD_BLOCK_SKILL,
    MOD_BLOCK_PERCENT,
    MOD_CRIT_PERCENT,
    PERIODIC_LEECH,
    MOD_HIT_CHANCE,
    MOD_SPELL_HIT_CHANCE,
    TRANSFORM,
    MOD_SPELL_CRIT_CHANCE,
    MOD_INCREASE_SWIM_SPEED,
    MOD_DAMAGE_DONE_CREATURE,
    MOD_PACIFY_SILENCE,
    MOD_SCALE,
    PERIODIC_HEALTH_FUNNEL,
    PERIODIC_MANA_FUNNEL,
    PERIODIC_MANA_LEECH {
        misc_value2: u32,
        damage: u32,
        gain_multiplier: f32,
    },
    MOD_CASTING_SPEED_NOT_STACK,
    FEIGN_DEATH,
    MOD_DISARM,
    MOD_STALKED,
    SCHOOL_ABSORB,
    EXTRA_ATTACKS,
    MOD_SPELL_CRIT_CHANCE_SCHOOL,
    MOD_POWER_COST_SCHOOL_PCT,
    MOD_POWER_COST_SCHOOL,
    REFLECT_SPELLS_SCHOOL,
    MOD_LANGUAGE,
    FAR_SIGHT,
    MECHANIC_IMMUNITY,
    MOUNTED,
    MOD_DAMAGE_PERCENT_DONE,
    MOD_PERCENT_STAT,
    SPLIT_DAMAGE_PCT,
    WATER_BREATHING,
    MOD_BASE_RESISTANCE,
    MOD_REGEN,
    MOD_POWER_REGEN,
    CHANNEL_DEATH_ITEM,
    MOD_DAMAGE_PERCENT_TAKEN,
    MOD_HEALTH_REGEN_PERCENT,
    PERIODIC_DAMAGE_PERCENT {
        damage1: u32,
        school: SpellSchool,
        absorbed: u32,
        resisted: u32,
    },
    MOD_RESIST_CHANCE,
    MOD_DETECT_RANGE,
    PREVENTS_FLEEING,
    MOD_UNATTACKABLE,
    INTERRUPT_REGEN,
    GHOST,
    SPELL_MAGNET,
    MANA_SHIELD,
    MOD_SKILL_TALENT,
    MOD_ATTACK_POWER,
    AURAS_VISIBLE,
    MOD_RESISTANCE_PCT,
    MOD_MELEE_ATTACK_POWER_VERSUS,
    MOD_TOTAL_THREAT,
    WATER_WALK,
    FEATHER_FALL,
    HOVER,
    ADD_FLAT_MODIFIER,
    ADD_PCT_MODIFIER,
    ADD_TARGET_TRIGGER,
    MOD_POWER_REGEN_PERCENT,
    ADD_CASTER_HIT_TRIGGER,
    OVERRIDE_CLASS_SCRIPTS,
    MOD_RANGED_DAMAGE_TAKEN,
    MOD_RANGED_DAMAGE_TAKEN_PCT,
    MOD_HEALING,
    MOD_REGEN_DURING_COMBAT,
    MOD_MECHANIC_RESISTANCE,
    MOD_HEALING_PCT,
    SHARE_PET_TRACKING,
    UNTRACKABLE,
    EMPATHY,
    MOD_OFFHAND_DAMAGE_PCT,
    MOD_TARGET_RESISTANCE,
    MOD_RANGED_ATTACK_POWER,
    MOD_MELEE_DAMAGE_TAKEN,
    MOD_MELEE_DAMAGE_TAKEN_PCT,
    RANGED_ATTACK_POWER_ATTACKER_BONUS,
    MOD_POSSESS_PET,
    MOD_SPEED_ALWAYS,
    MOD_MOUNTED_SPEED_ALWAYS,
    MOD_RANGED_ATTACK_POWER_VERSUS,
    MOD_INCREASE_ENERGY_PERCENT,
    MOD_INCREASE_HEALTH_PERCENT,
    MOD_MANA_REGEN_INTERRUPT,
    MOD_HEALING_DONE,
    MOD_HEALING_DONE_PERCENT,
    MOD_TOTAL_STAT_PERCENTAGE,
    MOD_MELEE_HASTE,
    FORCE_REACTION,
    MOD_RANGED_HASTE,
    MOD_RANGED_AMMO_HASTE,
    MOD_BASE_RESISTANCE_PCT,
    MOD_RESISTANCE_EXCLUSIVE,
    SAFE_FALL,
    CHARISMA,
    PERSUADED,
    MECHANIC_IMMUNITY_MASK,
    RETAIN_COMBO_POINTS,
    RESIST_PUSHBACK,
    MOD_SHIELD_BLOCKVALUE_PCT,
    TRACK_STEALTHED,
    MOD_DETECTED_RANGE,
    SPLIT_DAMAGE_FLAT,
    MOD_STEALTH_LEVEL,
    MOD_WATER_BREATHING,
    MOD_REPUTATION_GAIN,
    PET_DAMAGE_MULTI,
    MOD_SHIELD_BLOCKVALUE,
    NO_PVP_CREDIT,
    MOD_AOE_AVOIDANCE,
    MOD_HEALTH_REGEN_IN_COMBAT,
    POWER_BURN_MANA,
    MOD_CRIT_DAMAGE_BONUS,
    UNKNOWN164,
    MELEE_ATTACK_POWER_ATTACKER_BONUS,
    MOD_ATTACK_POWER_PCT,
    MOD_RANGED_ATTACK_POWER_PCT,
    MOD_DAMAGE_DONE_VERSUS,
    MOD_CRIT_PERCENT_VERSUS,
    DETECT_AMORE,
    MOD_SPEED_NOT_STACK,
    MOD_MOUNTED_SPEED_NOT_STACK,
    ALLOW_CHAMPION_SPELLS,
    MOD_SPELL_DAMAGE_OF_STAT_PERCENT,
    MOD_SPELL_HEALING_OF_STAT_PERCENT,
    SPIRIT_OF_REDEMPTION,
    AOE_CHARM,
    MOD_DEBUFF_RESISTANCE,
    MOD_ATTACKER_SPELL_CRIT_CHANCE,
    MOD_FLAT_SPELL_DAMAGE_VERSUS,
    MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS,
    MOD_RESISTANCE_OF_STAT_PERCENT,
    MOD_CRITICAL_THREAT,
    MOD_ATTACKER_MELEE_HIT_CHANCE,
    MOD_ATTACKER_RANGED_HIT_CHANCE,
    MOD_ATTACKER_SPELL_HIT_CHANCE,
    MOD_ATTACKER_MELEE_CRIT_CHANCE,
    MOD_ATTACKER_RANGED_CRIT_CHANCE,
    MOD_RATING,
    MOD_FACTION_REPUTATION_GAIN,
    USE_NORMAL_MOVEMENT_SPEED,
}

impl From<&AuraType> for AuraLogAuraType {
    fn from(e: &AuraType) -> Self {
        match &e {
            AuraType::NONE => Self::NONE,
            AuraType::BIND_SIGHT => Self::BIND_SIGHT,
            AuraType::MOD_POSSESS => Self::MOD_POSSESS,
            AuraType::PERIODIC_DAMAGE => Self::PERIODIC_DAMAGE {
                damage1: Default::default(),
                school: Default::default(),
                absorbed: Default::default(),
                resisted: Default::default(),
            },
            AuraType::DUMMY => Self::DUMMY,
            AuraType::MOD_CONFUSE => Self::MOD_CONFUSE,
            AuraType::MOD_CHARM => Self::MOD_CHARM,
            AuraType::MOD_FEAR => Self::MOD_FEAR,
            AuraType::PERIODIC_HEAL => Self::PERIODIC_HEAL {
                damage2: Default::default(),
            },
            AuraType::MOD_ATTACKSPEED => Self::MOD_ATTACKSPEED,
            AuraType::MOD_THREAT => Self::MOD_THREAT,
            AuraType::MOD_TAUNT => Self::MOD_TAUNT,
            AuraType::MOD_STUN => Self::MOD_STUN,
            AuraType::MOD_DAMAGE_DONE => Self::MOD_DAMAGE_DONE,
            AuraType::MOD_DAMAGE_TAKEN => Self::MOD_DAMAGE_TAKEN,
            AuraType::DAMAGE_SHIELD => Self::DAMAGE_SHIELD,
            AuraType::MOD_STEALTH => Self::MOD_STEALTH,
            AuraType::MOD_STEALTH_DETECT => Self::MOD_STEALTH_DETECT,
            AuraType::MOD_INVISIBILITY => Self::MOD_INVISIBILITY,
            AuraType::MOD_INVISIBILITY_DETECTION => Self::MOD_INVISIBILITY_DETECTION,
            AuraType::OBS_MOD_HEALTH => Self::OBS_MOD_HEALTH {
                damage2: Default::default(),
            },
            AuraType::OBS_MOD_MANA => Self::OBS_MOD_MANA {
                misc_value1: Default::default(),
                damage3: Default::default(),
            },
            AuraType::MOD_RESISTANCE => Self::MOD_RESISTANCE,
            AuraType::PERIODIC_TRIGGER_SPELL => Self::PERIODIC_TRIGGER_SPELL,
            AuraType::PERIODIC_ENERGIZE => Self::PERIODIC_ENERGIZE {
                misc_value1: Default::default(),
                damage3: Default::default(),
            },
            AuraType::MOD_PACIFY => Self::MOD_PACIFY,
            AuraType::MOD_ROOT => Self::MOD_ROOT,
            AuraType::MOD_SILENCE => Self::MOD_SILENCE,
            AuraType::REFLECT_SPELLS => Self::REFLECT_SPELLS,
            AuraType::MOD_STAT => Self::MOD_STAT,
            AuraType::MOD_SKILL => Self::MOD_SKILL,
            AuraType::MOD_INCREASE_SPEED => Self::MOD_INCREASE_SPEED,
            AuraType::MOD_INCREASE_MOUNTED_SPEED => Self::MOD_INCREASE_MOUNTED_SPEED,
            AuraType::MOD_DECREASE_SPEED => Self::MOD_DECREASE_SPEED,
            AuraType::MOD_INCREASE_HEALTH => Self::MOD_INCREASE_HEALTH,
            AuraType::MOD_INCREASE_ENERGY => Self::MOD_INCREASE_ENERGY,
            AuraType::MOD_SHAPESHIFT => Self::MOD_SHAPESHIFT,
            AuraType::EFFECT_IMMUNITY => Self::EFFECT_IMMUNITY,
            AuraType::STATE_IMMUNITY => Self::STATE_IMMUNITY,
            AuraType::SCHOOL_IMMUNITY => Self::SCHOOL_IMMUNITY,
            AuraType::DAMAGE_IMMUNITY => Self::DAMAGE_IMMUNITY,
            AuraType::DISPEL_IMMUNITY => Self::DISPEL_IMMUNITY,
            AuraType::PROC_TRIGGER_SPELL => Self::PROC_TRIGGER_SPELL,
            AuraType::PROC_TRIGGER_DAMAGE => Self::PROC_TRIGGER_DAMAGE,
            AuraType::TRACK_CREATURES => Self::TRACK_CREATURES,
            AuraType::TRACK_RESOURCES => Self::TRACK_RESOURCES,
            AuraType::UNKNOWN46 => Self::UNKNOWN46,
            AuraType::MOD_PARRY_PERCENT => Self::MOD_PARRY_PERCENT,
            AuraType::UNKNOWN48 => Self::UNKNOWN48,
            AuraType::MOD_DODGE_PERCENT => Self::MOD_DODGE_PERCENT,
            AuraType::MOD_BLOCK_SKILL => Self::MOD_BLOCK_SKILL,
            AuraType::MOD_BLOCK_PERCENT => Self::MOD_BLOCK_PERCENT,
            AuraType::MOD_CRIT_PERCENT => Self::MOD_CRIT_PERCENT,
            AuraType::PERIODIC_LEECH => Self::PERIODIC_LEECH,
            AuraType::MOD_HIT_CHANCE => Self::MOD_HIT_CHANCE,
            AuraType::MOD_SPELL_HIT_CHANCE => Self::MOD_SPELL_HIT_CHANCE,
            AuraType::TRANSFORM => Self::TRANSFORM,
            AuraType::MOD_SPELL_CRIT_CHANCE => Self::MOD_SPELL_CRIT_CHANCE,
            AuraType::MOD_INCREASE_SWIM_SPEED => Self::MOD_INCREASE_SWIM_SPEED,
            AuraType::MOD_DAMAGE_DONE_CREATURE => Self::MOD_DAMAGE_DONE_CREATURE,
            AuraType::MOD_PACIFY_SILENCE => Self::MOD_PACIFY_SILENCE,
            AuraType::MOD_SCALE => Self::MOD_SCALE,
            AuraType::PERIODIC_HEALTH_FUNNEL => Self::PERIODIC_HEALTH_FUNNEL,
            AuraType::PERIODIC_MANA_FUNNEL => Self::PERIODIC_MANA_FUNNEL,
            AuraType::PERIODIC_MANA_LEECH => Self::PERIODIC_MANA_LEECH {
                misc_value2: Default::default(),
                damage: Default::default(),
                gain_multiplier: Default::default(),
            },
            AuraType::MOD_CASTING_SPEED_NOT_STACK => Self::MOD_CASTING_SPEED_NOT_STACK,
            AuraType::FEIGN_DEATH => Self::FEIGN_DEATH,
            AuraType::MOD_DISARM => Self::MOD_DISARM,
            AuraType::MOD_STALKED => Self::MOD_STALKED,
            AuraType::SCHOOL_ABSORB => Self::SCHOOL_ABSORB,
            AuraType::EXTRA_ATTACKS => Self::EXTRA_ATTACKS,
            AuraType::MOD_SPELL_CRIT_CHANCE_SCHOOL => Self::MOD_SPELL_CRIT_CHANCE_SCHOOL,
            AuraType::MOD_POWER_COST_SCHOOL_PCT => Self::MOD_POWER_COST_SCHOOL_PCT,
            AuraType::MOD_POWER_COST_SCHOOL => Self::MOD_POWER_COST_SCHOOL,
            AuraType::REFLECT_SPELLS_SCHOOL => Self::REFLECT_SPELLS_SCHOOL,
            AuraType::MOD_LANGUAGE => Self::MOD_LANGUAGE,
            AuraType::FAR_SIGHT => Self::FAR_SIGHT,
            AuraType::MECHANIC_IMMUNITY => Self::MECHANIC_IMMUNITY,
            AuraType::MOUNTED => Self::MOUNTED,
            AuraType::MOD_DAMAGE_PERCENT_DONE => Self::MOD_DAMAGE_PERCENT_DONE,
            AuraType::MOD_PERCENT_STAT => Self::MOD_PERCENT_STAT,
            AuraType::SPLIT_DAMAGE_PCT => Self::SPLIT_DAMAGE_PCT,
            AuraType::WATER_BREATHING => Self::WATER_BREATHING,
            AuraType::MOD_BASE_RESISTANCE => Self::MOD_BASE_RESISTANCE,
            AuraType::MOD_REGEN => Self::MOD_REGEN,
            AuraType::MOD_POWER_REGEN => Self::MOD_POWER_REGEN,
            AuraType::CHANNEL_DEATH_ITEM => Self::CHANNEL_DEATH_ITEM,
            AuraType::MOD_DAMAGE_PERCENT_TAKEN => Self::MOD_DAMAGE_PERCENT_TAKEN,
            AuraType::MOD_HEALTH_REGEN_PERCENT => Self::MOD_HEALTH_REGEN_PERCENT,
            AuraType::PERIODIC_DAMAGE_PERCENT => Self::PERIODIC_DAMAGE_PERCENT {
                damage1: Default::default(),
                school: Default::default(),
                absorbed: Default::default(),
                resisted: Default::default(),
            },
            AuraType::MOD_RESIST_CHANCE => Self::MOD_RESIST_CHANCE,
            AuraType::MOD_DETECT_RANGE => Self::MOD_DETECT_RANGE,
            AuraType::PREVENTS_FLEEING => Self::PREVENTS_FLEEING,
            AuraType::MOD_UNATTACKABLE => Self::MOD_UNATTACKABLE,
            AuraType::INTERRUPT_REGEN => Self::INTERRUPT_REGEN,
            AuraType::GHOST => Self::GHOST,
            AuraType::SPELL_MAGNET => Self::SPELL_MAGNET,
            AuraType::MANA_SHIELD => Self::MANA_SHIELD,
            AuraType::MOD_SKILL_TALENT => Self::MOD_SKILL_TALENT,
            AuraType::MOD_ATTACK_POWER => Self::MOD_ATTACK_POWER,
            AuraType::AURAS_VISIBLE => Self::AURAS_VISIBLE,
            AuraType::MOD_RESISTANCE_PCT => Self::MOD_RESISTANCE_PCT,
            AuraType::MOD_MELEE_ATTACK_POWER_VERSUS => Self::MOD_MELEE_ATTACK_POWER_VERSUS,
            AuraType::MOD_TOTAL_THREAT => Self::MOD_TOTAL_THREAT,
            AuraType::WATER_WALK => Self::WATER_WALK,
            AuraType::FEATHER_FALL => Self::FEATHER_FALL,
            AuraType::HOVER => Self::HOVER,
            AuraType::ADD_FLAT_MODIFIER => Self::ADD_FLAT_MODIFIER,
            AuraType::ADD_PCT_MODIFIER => Self::ADD_PCT_MODIFIER,
            AuraType::ADD_TARGET_TRIGGER => Self::ADD_TARGET_TRIGGER,
            AuraType::MOD_POWER_REGEN_PERCENT => Self::MOD_POWER_REGEN_PERCENT,
            AuraType::ADD_CASTER_HIT_TRIGGER => Self::ADD_CASTER_HIT_TRIGGER,
            AuraType::OVERRIDE_CLASS_SCRIPTS => Self::OVERRIDE_CLASS_SCRIPTS,
            AuraType::MOD_RANGED_DAMAGE_TAKEN => Self::MOD_RANGED_DAMAGE_TAKEN,
            AuraType::MOD_RANGED_DAMAGE_TAKEN_PCT => Self::MOD_RANGED_DAMAGE_TAKEN_PCT,
            AuraType::MOD_HEALING => Self::MOD_HEALING,
            AuraType::MOD_REGEN_DURING_COMBAT => Self::MOD_REGEN_DURING_COMBAT,
            AuraType::MOD_MECHANIC_RESISTANCE => Self::MOD_MECHANIC_RESISTANCE,
            AuraType::MOD_HEALING_PCT => Self::MOD_HEALING_PCT,
            AuraType::SHARE_PET_TRACKING => Self::SHARE_PET_TRACKING,
            AuraType::UNTRACKABLE => Self::UNTRACKABLE,
            AuraType::EMPATHY => Self::EMPATHY,
            AuraType::MOD_OFFHAND_DAMAGE_PCT => Self::MOD_OFFHAND_DAMAGE_PCT,
            AuraType::MOD_TARGET_RESISTANCE => Self::MOD_TARGET_RESISTANCE,
            AuraType::MOD_RANGED_ATTACK_POWER => Self::MOD_RANGED_ATTACK_POWER,
            AuraType::MOD_MELEE_DAMAGE_TAKEN => Self::MOD_MELEE_DAMAGE_TAKEN,
            AuraType::MOD_MELEE_DAMAGE_TAKEN_PCT => Self::MOD_MELEE_DAMAGE_TAKEN_PCT,
            AuraType::RANGED_ATTACK_POWER_ATTACKER_BONUS => Self::RANGED_ATTACK_POWER_ATTACKER_BONUS,
            AuraType::MOD_POSSESS_PET => Self::MOD_POSSESS_PET,
            AuraType::MOD_SPEED_ALWAYS => Self::MOD_SPEED_ALWAYS,
            AuraType::MOD_MOUNTED_SPEED_ALWAYS => Self::MOD_MOUNTED_SPEED_ALWAYS,
            AuraType::MOD_RANGED_ATTACK_POWER_VERSUS => Self::MOD_RANGED_ATTACK_POWER_VERSUS,
            AuraType::MOD_INCREASE_ENERGY_PERCENT => Self::MOD_INCREASE_ENERGY_PERCENT,
            AuraType::MOD_INCREASE_HEALTH_PERCENT => Self::MOD_INCREASE_HEALTH_PERCENT,
            AuraType::MOD_MANA_REGEN_INTERRUPT => Self::MOD_MANA_REGEN_INTERRUPT,
            AuraType::MOD_HEALING_DONE => Self::MOD_HEALING_DONE,
            AuraType::MOD_HEALING_DONE_PERCENT => Self::MOD_HEALING_DONE_PERCENT,
            AuraType::MOD_TOTAL_STAT_PERCENTAGE => Self::MOD_TOTAL_STAT_PERCENTAGE,
            AuraType::MOD_MELEE_HASTE => Self::MOD_MELEE_HASTE,
            AuraType::FORCE_REACTION => Self::FORCE_REACTION,
            AuraType::MOD_RANGED_HASTE => Self::MOD_RANGED_HASTE,
            AuraType::MOD_RANGED_AMMO_HASTE => Self::MOD_RANGED_AMMO_HASTE,
            AuraType::MOD_BASE_RESISTANCE_PCT => Self::MOD_BASE_RESISTANCE_PCT,
            AuraType::MOD_RESISTANCE_EXCLUSIVE => Self::MOD_RESISTANCE_EXCLUSIVE,
            AuraType::SAFE_FALL => Self::SAFE_FALL,
            AuraType::CHARISMA => Self::CHARISMA,
            AuraType::PERSUADED => Self::PERSUADED,
            AuraType::MECHANIC_IMMUNITY_MASK => Self::MECHANIC_IMMUNITY_MASK,
            AuraType::RETAIN_COMBO_POINTS => Self::RETAIN_COMBO_POINTS,
            AuraType::RESIST_PUSHBACK => Self::RESIST_PUSHBACK,
            AuraType::MOD_SHIELD_BLOCKVALUE_PCT => Self::MOD_SHIELD_BLOCKVALUE_PCT,
            AuraType::TRACK_STEALTHED => Self::TRACK_STEALTHED,
            AuraType::MOD_DETECTED_RANGE => Self::MOD_DETECTED_RANGE,
            AuraType::SPLIT_DAMAGE_FLAT => Self::SPLIT_DAMAGE_FLAT,
            AuraType::MOD_STEALTH_LEVEL => Self::MOD_STEALTH_LEVEL,
            AuraType::MOD_WATER_BREATHING => Self::MOD_WATER_BREATHING,
            AuraType::MOD_REPUTATION_GAIN => Self::MOD_REPUTATION_GAIN,
            AuraType::PET_DAMAGE_MULTI => Self::PET_DAMAGE_MULTI,
            AuraType::MOD_SHIELD_BLOCKVALUE => Self::MOD_SHIELD_BLOCKVALUE,
            AuraType::NO_PVP_CREDIT => Self::NO_PVP_CREDIT,
            AuraType::MOD_AOE_AVOIDANCE => Self::MOD_AOE_AVOIDANCE,
            AuraType::MOD_HEALTH_REGEN_IN_COMBAT => Self::MOD_HEALTH_REGEN_IN_COMBAT,
            AuraType::POWER_BURN_MANA => Self::POWER_BURN_MANA,
            AuraType::MOD_CRIT_DAMAGE_BONUS => Self::MOD_CRIT_DAMAGE_BONUS,
            AuraType::UNKNOWN164 => Self::UNKNOWN164,
            AuraType::MELEE_ATTACK_POWER_ATTACKER_BONUS => Self::MELEE_ATTACK_POWER_ATTACKER_BONUS,
            AuraType::MOD_ATTACK_POWER_PCT => Self::MOD_ATTACK_POWER_PCT,
            AuraType::MOD_RANGED_ATTACK_POWER_PCT => Self::MOD_RANGED_ATTACK_POWER_PCT,
            AuraType::MOD_DAMAGE_DONE_VERSUS => Self::MOD_DAMAGE_DONE_VERSUS,
            AuraType::MOD_CRIT_PERCENT_VERSUS => Self::MOD_CRIT_PERCENT_VERSUS,
            AuraType::DETECT_AMORE => Self::DETECT_AMORE,
            AuraType::MOD_SPEED_NOT_STACK => Self::MOD_SPEED_NOT_STACK,
            AuraType::MOD_MOUNTED_SPEED_NOT_STACK => Self::MOD_MOUNTED_SPEED_NOT_STACK,
            AuraType::ALLOW_CHAMPION_SPELLS => Self::ALLOW_CHAMPION_SPELLS,
            AuraType::MOD_SPELL_DAMAGE_OF_STAT_PERCENT => Self::MOD_SPELL_DAMAGE_OF_STAT_PERCENT,
            AuraType::MOD_SPELL_HEALING_OF_STAT_PERCENT => Self::MOD_SPELL_HEALING_OF_STAT_PERCENT,
            AuraType::SPIRIT_OF_REDEMPTION => Self::SPIRIT_OF_REDEMPTION,
            AuraType::AOE_CHARM => Self::AOE_CHARM,
            AuraType::MOD_DEBUFF_RESISTANCE => Self::MOD_DEBUFF_RESISTANCE,
            AuraType::MOD_ATTACKER_SPELL_CRIT_CHANCE => Self::MOD_ATTACKER_SPELL_CRIT_CHANCE,
            AuraType::MOD_FLAT_SPELL_DAMAGE_VERSUS => Self::MOD_FLAT_SPELL_DAMAGE_VERSUS,
            AuraType::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS => Self::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS,
            AuraType::MOD_RESISTANCE_OF_STAT_PERCENT => Self::MOD_RESISTANCE_OF_STAT_PERCENT,
            AuraType::MOD_CRITICAL_THREAT => Self::MOD_CRITICAL_THREAT,
            AuraType::MOD_ATTACKER_MELEE_HIT_CHANCE => Self::MOD_ATTACKER_MELEE_HIT_CHANCE,
            AuraType::MOD_ATTACKER_RANGED_HIT_CHANCE => Self::MOD_ATTACKER_RANGED_HIT_CHANCE,
            AuraType::MOD_ATTACKER_SPELL_HIT_CHANCE => Self::MOD_ATTACKER_SPELL_HIT_CHANCE,
            AuraType::MOD_ATTACKER_MELEE_CRIT_CHANCE => Self::MOD_ATTACKER_MELEE_CRIT_CHANCE,
            AuraType::MOD_ATTACKER_RANGED_CRIT_CHANCE => Self::MOD_ATTACKER_RANGED_CRIT_CHANCE,
            AuraType::MOD_RATING => Self::MOD_RATING,
            AuraType::MOD_FACTION_REPUTATION_GAIN => Self::MOD_FACTION_REPUTATION_GAIN,
            AuraType::USE_NORMAL_MOVEMENT_SPEED => Self::USE_NORMAL_MOVEMENT_SPEED,
        }
    }
}

impl From<&AuraLogAuraType> for AuraType {
    fn from(v: &AuraLogAuraType) -> Self {
        match &v {
            AuraLogAuraType::NONE => Self::NONE,
            AuraLogAuraType::BIND_SIGHT => Self::BIND_SIGHT,
            AuraLogAuraType::MOD_POSSESS => Self::MOD_POSSESS,
            AuraLogAuraType::PERIODIC_DAMAGE { .. } => Self::PERIODIC_DAMAGE,
            AuraLogAuraType::DUMMY => Self::DUMMY,
            AuraLogAuraType::MOD_CONFUSE => Self::MOD_CONFUSE,
            AuraLogAuraType::MOD_CHARM => Self::MOD_CHARM,
            AuraLogAuraType::MOD_FEAR => Self::MOD_FEAR,
            AuraLogAuraType::PERIODIC_HEAL { .. } => Self::PERIODIC_HEAL,
            AuraLogAuraType::MOD_ATTACKSPEED => Self::MOD_ATTACKSPEED,
            AuraLogAuraType::MOD_THREAT => Self::MOD_THREAT,
            AuraLogAuraType::MOD_TAUNT => Self::MOD_TAUNT,
            AuraLogAuraType::MOD_STUN => Self::MOD_STUN,
            AuraLogAuraType::MOD_DAMAGE_DONE => Self::MOD_DAMAGE_DONE,
            AuraLogAuraType::MOD_DAMAGE_TAKEN => Self::MOD_DAMAGE_TAKEN,
            AuraLogAuraType::DAMAGE_SHIELD => Self::DAMAGE_SHIELD,
            AuraLogAuraType::MOD_STEALTH => Self::MOD_STEALTH,
            AuraLogAuraType::MOD_STEALTH_DETECT => Self::MOD_STEALTH_DETECT,
            AuraLogAuraType::MOD_INVISIBILITY => Self::MOD_INVISIBILITY,
            AuraLogAuraType::MOD_INVISIBILITY_DETECTION => Self::MOD_INVISIBILITY_DETECTION,
            AuraLogAuraType::OBS_MOD_HEALTH { .. } => Self::OBS_MOD_HEALTH,
            AuraLogAuraType::OBS_MOD_MANA { .. } => Self::OBS_MOD_MANA,
            AuraLogAuraType::MOD_RESISTANCE => Self::MOD_RESISTANCE,
            AuraLogAuraType::PERIODIC_TRIGGER_SPELL => Self::PERIODIC_TRIGGER_SPELL,
            AuraLogAuraType::PERIODIC_ENERGIZE { .. } => Self::PERIODIC_ENERGIZE,
            AuraLogAuraType::MOD_PACIFY => Self::MOD_PACIFY,
            AuraLogAuraType::MOD_ROOT => Self::MOD_ROOT,
            AuraLogAuraType::MOD_SILENCE => Self::MOD_SILENCE,
            AuraLogAuraType::REFLECT_SPELLS => Self::REFLECT_SPELLS,
            AuraLogAuraType::MOD_STAT => Self::MOD_STAT,
            AuraLogAuraType::MOD_SKILL => Self::MOD_SKILL,
            AuraLogAuraType::MOD_INCREASE_SPEED => Self::MOD_INCREASE_SPEED,
            AuraLogAuraType::MOD_INCREASE_MOUNTED_SPEED => Self::MOD_INCREASE_MOUNTED_SPEED,
            AuraLogAuraType::MOD_DECREASE_SPEED => Self::MOD_DECREASE_SPEED,
            AuraLogAuraType::MOD_INCREASE_HEALTH => Self::MOD_INCREASE_HEALTH,
            AuraLogAuraType::MOD_INCREASE_ENERGY => Self::MOD_INCREASE_ENERGY,
            AuraLogAuraType::MOD_SHAPESHIFT => Self::MOD_SHAPESHIFT,
            AuraLogAuraType::EFFECT_IMMUNITY => Self::EFFECT_IMMUNITY,
            AuraLogAuraType::STATE_IMMUNITY => Self::STATE_IMMUNITY,
            AuraLogAuraType::SCHOOL_IMMUNITY => Self::SCHOOL_IMMUNITY,
            AuraLogAuraType::DAMAGE_IMMUNITY => Self::DAMAGE_IMMUNITY,
            AuraLogAuraType::DISPEL_IMMUNITY => Self::DISPEL_IMMUNITY,
            AuraLogAuraType::PROC_TRIGGER_SPELL => Self::PROC_TRIGGER_SPELL,
            AuraLogAuraType::PROC_TRIGGER_DAMAGE => Self::PROC_TRIGGER_DAMAGE,
            AuraLogAuraType::TRACK_CREATURES => Self::TRACK_CREATURES,
            AuraLogAuraType::TRACK_RESOURCES => Self::TRACK_RESOURCES,
            AuraLogAuraType::UNKNOWN46 => Self::UNKNOWN46,
            AuraLogAuraType::MOD_PARRY_PERCENT => Self::MOD_PARRY_PERCENT,
            AuraLogAuraType::UNKNOWN48 => Self::UNKNOWN48,
            AuraLogAuraType::MOD_DODGE_PERCENT => Self::MOD_DODGE_PERCENT,
            AuraLogAuraType::MOD_BLOCK_SKILL => Self::MOD_BLOCK_SKILL,
            AuraLogAuraType::MOD_BLOCK_PERCENT => Self::MOD_BLOCK_PERCENT,
            AuraLogAuraType::MOD_CRIT_PERCENT => Self::MOD_CRIT_PERCENT,
            AuraLogAuraType::PERIODIC_LEECH => Self::PERIODIC_LEECH,
            AuraLogAuraType::MOD_HIT_CHANCE => Self::MOD_HIT_CHANCE,
            AuraLogAuraType::MOD_SPELL_HIT_CHANCE => Self::MOD_SPELL_HIT_CHANCE,
            AuraLogAuraType::TRANSFORM => Self::TRANSFORM,
            AuraLogAuraType::MOD_SPELL_CRIT_CHANCE => Self::MOD_SPELL_CRIT_CHANCE,
            AuraLogAuraType::MOD_INCREASE_SWIM_SPEED => Self::MOD_INCREASE_SWIM_SPEED,
            AuraLogAuraType::MOD_DAMAGE_DONE_CREATURE => Self::MOD_DAMAGE_DONE_CREATURE,
            AuraLogAuraType::MOD_PACIFY_SILENCE => Self::MOD_PACIFY_SILENCE,
            AuraLogAuraType::MOD_SCALE => Self::MOD_SCALE,
            AuraLogAuraType::PERIODIC_HEALTH_FUNNEL => Self::PERIODIC_HEALTH_FUNNEL,
            AuraLogAuraType::PERIODIC_MANA_FUNNEL => Self::PERIODIC_MANA_FUNNEL,
            AuraLogAuraType::PERIODIC_MANA_LEECH { .. } => Self::PERIODIC_MANA_LEECH,
            AuraLogAuraType::MOD_CASTING_SPEED_NOT_STACK => Self::MOD_CASTING_SPEED_NOT_STACK,
            AuraLogAuraType::FEIGN_DEATH => Self::FEIGN_DEATH,
            AuraLogAuraType::MOD_DISARM => Self::MOD_DISARM,
            AuraLogAuraType::MOD_STALKED => Self::MOD_STALKED,
            AuraLogAuraType::SCHOOL_ABSORB => Self::SCHOOL_ABSORB,
            AuraLogAuraType::EXTRA_ATTACKS => Self::EXTRA_ATTACKS,
            AuraLogAuraType::MOD_SPELL_CRIT_CHANCE_SCHOOL => Self::MOD_SPELL_CRIT_CHANCE_SCHOOL,
            AuraLogAuraType::MOD_POWER_COST_SCHOOL_PCT => Self::MOD_POWER_COST_SCHOOL_PCT,
            AuraLogAuraType::MOD_POWER_COST_SCHOOL => Self::MOD_POWER_COST_SCHOOL,
            AuraLogAuraType::REFLECT_SPELLS_SCHOOL => Self::REFLECT_SPELLS_SCHOOL,
            AuraLogAuraType::MOD_LANGUAGE => Self::MOD_LANGUAGE,
            AuraLogAuraType::FAR_SIGHT => Self::FAR_SIGHT,
            AuraLogAuraType::MECHANIC_IMMUNITY => Self::MECHANIC_IMMUNITY,
            AuraLogAuraType::MOUNTED => Self::MOUNTED,
            AuraLogAuraType::MOD_DAMAGE_PERCENT_DONE => Self::MOD_DAMAGE_PERCENT_DONE,
            AuraLogAuraType::MOD_PERCENT_STAT => Self::MOD_PERCENT_STAT,
            AuraLogAuraType::SPLIT_DAMAGE_PCT => Self::SPLIT_DAMAGE_PCT,
            AuraLogAuraType::WATER_BREATHING => Self::WATER_BREATHING,
            AuraLogAuraType::MOD_BASE_RESISTANCE => Self::MOD_BASE_RESISTANCE,
            AuraLogAuraType::MOD_REGEN => Self::MOD_REGEN,
            AuraLogAuraType::MOD_POWER_REGEN => Self::MOD_POWER_REGEN,
            AuraLogAuraType::CHANNEL_DEATH_ITEM => Self::CHANNEL_DEATH_ITEM,
            AuraLogAuraType::MOD_DAMAGE_PERCENT_TAKEN => Self::MOD_DAMAGE_PERCENT_TAKEN,
            AuraLogAuraType::MOD_HEALTH_REGEN_PERCENT => Self::MOD_HEALTH_REGEN_PERCENT,
            AuraLogAuraType::PERIODIC_DAMAGE_PERCENT { .. } => Self::PERIODIC_DAMAGE_PERCENT,
            AuraLogAuraType::MOD_RESIST_CHANCE => Self::MOD_RESIST_CHANCE,
            AuraLogAuraType::MOD_DETECT_RANGE => Self::MOD_DETECT_RANGE,
            AuraLogAuraType::PREVENTS_FLEEING => Self::PREVENTS_FLEEING,
            AuraLogAuraType::MOD_UNATTACKABLE => Self::MOD_UNATTACKABLE,
            AuraLogAuraType::INTERRUPT_REGEN => Self::INTERRUPT_REGEN,
            AuraLogAuraType::GHOST => Self::GHOST,
            AuraLogAuraType::SPELL_MAGNET => Self::SPELL_MAGNET,
            AuraLogAuraType::MANA_SHIELD => Self::MANA_SHIELD,
            AuraLogAuraType::MOD_SKILL_TALENT => Self::MOD_SKILL_TALENT,
            AuraLogAuraType::MOD_ATTACK_POWER => Self::MOD_ATTACK_POWER,
            AuraLogAuraType::AURAS_VISIBLE => Self::AURAS_VISIBLE,
            AuraLogAuraType::MOD_RESISTANCE_PCT => Self::MOD_RESISTANCE_PCT,
            AuraLogAuraType::MOD_MELEE_ATTACK_POWER_VERSUS => Self::MOD_MELEE_ATTACK_POWER_VERSUS,
            AuraLogAuraType::MOD_TOTAL_THREAT => Self::MOD_TOTAL_THREAT,
            AuraLogAuraType::WATER_WALK => Self::WATER_WALK,
            AuraLogAuraType::FEATHER_FALL => Self::FEATHER_FALL,
            AuraLogAuraType::HOVER => Self::HOVER,
            AuraLogAuraType::ADD_FLAT_MODIFIER => Self::ADD_FLAT_MODIFIER,
            AuraLogAuraType::ADD_PCT_MODIFIER => Self::ADD_PCT_MODIFIER,
            AuraLogAuraType::ADD_TARGET_TRIGGER => Self::ADD_TARGET_TRIGGER,
            AuraLogAuraType::MOD_POWER_REGEN_PERCENT => Self::MOD_POWER_REGEN_PERCENT,
            AuraLogAuraType::ADD_CASTER_HIT_TRIGGER => Self::ADD_CASTER_HIT_TRIGGER,
            AuraLogAuraType::OVERRIDE_CLASS_SCRIPTS => Self::OVERRIDE_CLASS_SCRIPTS,
            AuraLogAuraType::MOD_RANGED_DAMAGE_TAKEN => Self::MOD_RANGED_DAMAGE_TAKEN,
            AuraLogAuraType::MOD_RANGED_DAMAGE_TAKEN_PCT => Self::MOD_RANGED_DAMAGE_TAKEN_PCT,
            AuraLogAuraType::MOD_HEALING => Self::MOD_HEALING,
            AuraLogAuraType::MOD_REGEN_DURING_COMBAT => Self::MOD_REGEN_DURING_COMBAT,
            AuraLogAuraType::MOD_MECHANIC_RESISTANCE => Self::MOD_MECHANIC_RESISTANCE,
            AuraLogAuraType::MOD_HEALING_PCT => Self::MOD_HEALING_PCT,
            AuraLogAuraType::SHARE_PET_TRACKING => Self::SHARE_PET_TRACKING,
            AuraLogAuraType::UNTRACKABLE => Self::UNTRACKABLE,
            AuraLogAuraType::EMPATHY => Self::EMPATHY,
            AuraLogAuraType::MOD_OFFHAND_DAMAGE_PCT => Self::MOD_OFFHAND_DAMAGE_PCT,
            AuraLogAuraType::MOD_TARGET_RESISTANCE => Self::MOD_TARGET_RESISTANCE,
            AuraLogAuraType::MOD_RANGED_ATTACK_POWER => Self::MOD_RANGED_ATTACK_POWER,
            AuraLogAuraType::MOD_MELEE_DAMAGE_TAKEN => Self::MOD_MELEE_DAMAGE_TAKEN,
            AuraLogAuraType::MOD_MELEE_DAMAGE_TAKEN_PCT => Self::MOD_MELEE_DAMAGE_TAKEN_PCT,
            AuraLogAuraType::RANGED_ATTACK_POWER_ATTACKER_BONUS => Self::RANGED_ATTACK_POWER_ATTACKER_BONUS,
            AuraLogAuraType::MOD_POSSESS_PET => Self::MOD_POSSESS_PET,
            AuraLogAuraType::MOD_SPEED_ALWAYS => Self::MOD_SPEED_ALWAYS,
            AuraLogAuraType::MOD_MOUNTED_SPEED_ALWAYS => Self::MOD_MOUNTED_SPEED_ALWAYS,
            AuraLogAuraType::MOD_RANGED_ATTACK_POWER_VERSUS => Self::MOD_RANGED_ATTACK_POWER_VERSUS,
            AuraLogAuraType::MOD_INCREASE_ENERGY_PERCENT => Self::MOD_INCREASE_ENERGY_PERCENT,
            AuraLogAuraType::MOD_INCREASE_HEALTH_PERCENT => Self::MOD_INCREASE_HEALTH_PERCENT,
            AuraLogAuraType::MOD_MANA_REGEN_INTERRUPT => Self::MOD_MANA_REGEN_INTERRUPT,
            AuraLogAuraType::MOD_HEALING_DONE => Self::MOD_HEALING_DONE,
            AuraLogAuraType::MOD_HEALING_DONE_PERCENT => Self::MOD_HEALING_DONE_PERCENT,
            AuraLogAuraType::MOD_TOTAL_STAT_PERCENTAGE => Self::MOD_TOTAL_STAT_PERCENTAGE,
            AuraLogAuraType::MOD_MELEE_HASTE => Self::MOD_MELEE_HASTE,
            AuraLogAuraType::FORCE_REACTION => Self::FORCE_REACTION,
            AuraLogAuraType::MOD_RANGED_HASTE => Self::MOD_RANGED_HASTE,
            AuraLogAuraType::MOD_RANGED_AMMO_HASTE => Self::MOD_RANGED_AMMO_HASTE,
            AuraLogAuraType::MOD_BASE_RESISTANCE_PCT => Self::MOD_BASE_RESISTANCE_PCT,
            AuraLogAuraType::MOD_RESISTANCE_EXCLUSIVE => Self::MOD_RESISTANCE_EXCLUSIVE,
            AuraLogAuraType::SAFE_FALL => Self::SAFE_FALL,
            AuraLogAuraType::CHARISMA => Self::CHARISMA,
            AuraLogAuraType::PERSUADED => Self::PERSUADED,
            AuraLogAuraType::MECHANIC_IMMUNITY_MASK => Self::MECHANIC_IMMUNITY_MASK,
            AuraLogAuraType::RETAIN_COMBO_POINTS => Self::RETAIN_COMBO_POINTS,
            AuraLogAuraType::RESIST_PUSHBACK => Self::RESIST_PUSHBACK,
            AuraLogAuraType::MOD_SHIELD_BLOCKVALUE_PCT => Self::MOD_SHIELD_BLOCKVALUE_PCT,
            AuraLogAuraType::TRACK_STEALTHED => Self::TRACK_STEALTHED,
            AuraLogAuraType::MOD_DETECTED_RANGE => Self::MOD_DETECTED_RANGE,
            AuraLogAuraType::SPLIT_DAMAGE_FLAT => Self::SPLIT_DAMAGE_FLAT,
            AuraLogAuraType::MOD_STEALTH_LEVEL => Self::MOD_STEALTH_LEVEL,
            AuraLogAuraType::MOD_WATER_BREATHING => Self::MOD_WATER_BREATHING,
            AuraLogAuraType::MOD_REPUTATION_GAIN => Self::MOD_REPUTATION_GAIN,
            AuraLogAuraType::PET_DAMAGE_MULTI => Self::PET_DAMAGE_MULTI,
            AuraLogAuraType::MOD_SHIELD_BLOCKVALUE => Self::MOD_SHIELD_BLOCKVALUE,
            AuraLogAuraType::NO_PVP_CREDIT => Self::NO_PVP_CREDIT,
            AuraLogAuraType::MOD_AOE_AVOIDANCE => Self::MOD_AOE_AVOIDANCE,
            AuraLogAuraType::MOD_HEALTH_REGEN_IN_COMBAT => Self::MOD_HEALTH_REGEN_IN_COMBAT,
            AuraLogAuraType::POWER_BURN_MANA => Self::POWER_BURN_MANA,
            AuraLogAuraType::MOD_CRIT_DAMAGE_BONUS => Self::MOD_CRIT_DAMAGE_BONUS,
            AuraLogAuraType::UNKNOWN164 => Self::UNKNOWN164,
            AuraLogAuraType::MELEE_ATTACK_POWER_ATTACKER_BONUS => Self::MELEE_ATTACK_POWER_ATTACKER_BONUS,
            AuraLogAuraType::MOD_ATTACK_POWER_PCT => Self::MOD_ATTACK_POWER_PCT,
            AuraLogAuraType::MOD_RANGED_ATTACK_POWER_PCT => Self::MOD_RANGED_ATTACK_POWER_PCT,
            AuraLogAuraType::MOD_DAMAGE_DONE_VERSUS => Self::MOD_DAMAGE_DONE_VERSUS,
            AuraLogAuraType::MOD_CRIT_PERCENT_VERSUS => Self::MOD_CRIT_PERCENT_VERSUS,
            AuraLogAuraType::DETECT_AMORE => Self::DETECT_AMORE,
            AuraLogAuraType::MOD_SPEED_NOT_STACK => Self::MOD_SPEED_NOT_STACK,
            AuraLogAuraType::MOD_MOUNTED_SPEED_NOT_STACK => Self::MOD_MOUNTED_SPEED_NOT_STACK,
            AuraLogAuraType::ALLOW_CHAMPION_SPELLS => Self::ALLOW_CHAMPION_SPELLS,
            AuraLogAuraType::MOD_SPELL_DAMAGE_OF_STAT_PERCENT => Self::MOD_SPELL_DAMAGE_OF_STAT_PERCENT,
            AuraLogAuraType::MOD_SPELL_HEALING_OF_STAT_PERCENT => Self::MOD_SPELL_HEALING_OF_STAT_PERCENT,
            AuraLogAuraType::SPIRIT_OF_REDEMPTION => Self::SPIRIT_OF_REDEMPTION,
            AuraLogAuraType::AOE_CHARM => Self::AOE_CHARM,
            AuraLogAuraType::MOD_DEBUFF_RESISTANCE => Self::MOD_DEBUFF_RESISTANCE,
            AuraLogAuraType::MOD_ATTACKER_SPELL_CRIT_CHANCE => Self::MOD_ATTACKER_SPELL_CRIT_CHANCE,
            AuraLogAuraType::MOD_FLAT_SPELL_DAMAGE_VERSUS => Self::MOD_FLAT_SPELL_DAMAGE_VERSUS,
            AuraLogAuraType::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS => Self::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS,
            AuraLogAuraType::MOD_RESISTANCE_OF_STAT_PERCENT => Self::MOD_RESISTANCE_OF_STAT_PERCENT,
            AuraLogAuraType::MOD_CRITICAL_THREAT => Self::MOD_CRITICAL_THREAT,
            AuraLogAuraType::MOD_ATTACKER_MELEE_HIT_CHANCE => Self::MOD_ATTACKER_MELEE_HIT_CHANCE,
            AuraLogAuraType::MOD_ATTACKER_RANGED_HIT_CHANCE => Self::MOD_ATTACKER_RANGED_HIT_CHANCE,
            AuraLogAuraType::MOD_ATTACKER_SPELL_HIT_CHANCE => Self::MOD_ATTACKER_SPELL_HIT_CHANCE,
            AuraLogAuraType::MOD_ATTACKER_MELEE_CRIT_CHANCE => Self::MOD_ATTACKER_MELEE_CRIT_CHANCE,
            AuraLogAuraType::MOD_ATTACKER_RANGED_CRIT_CHANCE => Self::MOD_ATTACKER_RANGED_CRIT_CHANCE,
            AuraLogAuraType::MOD_RATING => Self::MOD_RATING,
            AuraLogAuraType::MOD_FACTION_REPUTATION_GAIN => Self::MOD_FACTION_REPUTATION_GAIN,
            AuraLogAuraType::USE_NORMAL_MOVEMENT_SPEED => Self::USE_NORMAL_MOVEMENT_SPEED,
        }
    }
}

impl Default for AuraLogAuraType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NONE
    }
}

impl AuraLogAuraType {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: AuraType = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: AuraType = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: AuraType = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: AuraType = self.into();
        a.write_u32_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: AuraType = self.into();
        a.tokio_write_u32_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: AuraType = self.into();
        a.astd_write_u32_be(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: AuraType = self.into();
        a.write_u64_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: AuraType = self.into();
        a.tokio_write_u64_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: AuraType = self.into();
        a.astd_write_u64_le(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: AuraType = self.into();
        a.write_u64_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: AuraType = self.into();
        a.tokio_write_u64_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: AuraType = self.into();
        a.astd_write_u64_be(w).await
    }

}

impl VariableSized for AuraLogAuraType {
    fn size(&self) -> usize {
        match self {
            Self::NONE => {
                4
            }
            Self::BIND_SIGHT => {
                4
            }
            Self::MOD_POSSESS => {
                4
            }
            Self::PERIODIC_DAMAGE {
                absorbed,
                damage1,
                resisted,
                school,
            } => {
                4
                + 4 // absorbed: u32
                + 4 // damage1: u32
                + 4 // resisted: u32
                + 1 // school: SpellSchool
            }
            Self::DUMMY => {
                4
            }
            Self::MOD_CONFUSE => {
                4
            }
            Self::MOD_CHARM => {
                4
            }
            Self::MOD_FEAR => {
                4
            }
            Self::PERIODIC_HEAL {
                damage2,
            } => {
                4
                + 4 // damage2: u32
            }
            Self::MOD_ATTACKSPEED => {
                4
            }
            Self::MOD_THREAT => {
                4
            }
            Self::MOD_TAUNT => {
                4
            }
            Self::MOD_STUN => {
                4
            }
            Self::MOD_DAMAGE_DONE => {
                4
            }
            Self::MOD_DAMAGE_TAKEN => {
                4
            }
            Self::DAMAGE_SHIELD => {
                4
            }
            Self::MOD_STEALTH => {
                4
            }
            Self::MOD_STEALTH_DETECT => {
                4
            }
            Self::MOD_INVISIBILITY => {
                4
            }
            Self::MOD_INVISIBILITY_DETECTION => {
                4
            }
            Self::OBS_MOD_HEALTH {
                damage2,
            } => {
                4
                + 4 // damage2: u32
            }
            Self::OBS_MOD_MANA {
                damage3,
                misc_value1,
            } => {
                4
                + 4 // damage3: u32
                + 4 // misc_value1: u32
            }
            Self::MOD_RESISTANCE => {
                4
            }
            Self::PERIODIC_TRIGGER_SPELL => {
                4
            }
            Self::PERIODIC_ENERGIZE {
                damage3,
                misc_value1,
            } => {
                4
                + 4 // damage3: u32
                + 4 // misc_value1: u32
            }
            Self::MOD_PACIFY => {
                4
            }
            Self::MOD_ROOT => {
                4
            }
            Self::MOD_SILENCE => {
                4
            }
            Self::REFLECT_SPELLS => {
                4
            }
            Self::MOD_STAT => {
                4
            }
            Self::MOD_SKILL => {
                4
            }
            Self::MOD_INCREASE_SPEED => {
                4
            }
            Self::MOD_INCREASE_MOUNTED_SPEED => {
                4
            }
            Self::MOD_DECREASE_SPEED => {
                4
            }
            Self::MOD_INCREASE_HEALTH => {
                4
            }
            Self::MOD_INCREASE_ENERGY => {
                4
            }
            Self::MOD_SHAPESHIFT => {
                4
            }
            Self::EFFECT_IMMUNITY => {
                4
            }
            Self::STATE_IMMUNITY => {
                4
            }
            Self::SCHOOL_IMMUNITY => {
                4
            }
            Self::DAMAGE_IMMUNITY => {
                4
            }
            Self::DISPEL_IMMUNITY => {
                4
            }
            Self::PROC_TRIGGER_SPELL => {
                4
            }
            Self::PROC_TRIGGER_DAMAGE => {
                4
            }
            Self::TRACK_CREATURES => {
                4
            }
            Self::TRACK_RESOURCES => {
                4
            }
            Self::UNKNOWN46 => {
                4
            }
            Self::MOD_PARRY_PERCENT => {
                4
            }
            Self::UNKNOWN48 => {
                4
            }
            Self::MOD_DODGE_PERCENT => {
                4
            }
            Self::MOD_BLOCK_SKILL => {
                4
            }
            Self::MOD_BLOCK_PERCENT => {
                4
            }
            Self::MOD_CRIT_PERCENT => {
                4
            }
            Self::PERIODIC_LEECH => {
                4
            }
            Self::MOD_HIT_CHANCE => {
                4
            }
            Self::MOD_SPELL_HIT_CHANCE => {
                4
            }
            Self::TRANSFORM => {
                4
            }
            Self::MOD_SPELL_CRIT_CHANCE => {
                4
            }
            Self::MOD_INCREASE_SWIM_SPEED => {
                4
            }
            Self::MOD_DAMAGE_DONE_CREATURE => {
                4
            }
            Self::MOD_PACIFY_SILENCE => {
                4
            }
            Self::MOD_SCALE => {
                4
            }
            Self::PERIODIC_HEALTH_FUNNEL => {
                4
            }
            Self::PERIODIC_MANA_FUNNEL => {
                4
            }
            Self::PERIODIC_MANA_LEECH {
                damage,
                gain_multiplier,
                misc_value2,
            } => {
                4
                + 4 // damage: u32
                + 4 // gain_multiplier: f32
                + 4 // misc_value2: u32
            }
            Self::MOD_CASTING_SPEED_NOT_STACK => {
                4
            }
            Self::FEIGN_DEATH => {
                4
            }
            Self::MOD_DISARM => {
                4
            }
            Self::MOD_STALKED => {
                4
            }
            Self::SCHOOL_ABSORB => {
                4
            }
            Self::EXTRA_ATTACKS => {
                4
            }
            Self::MOD_SPELL_CRIT_CHANCE_SCHOOL => {
                4
            }
            Self::MOD_POWER_COST_SCHOOL_PCT => {
                4
            }
            Self::MOD_POWER_COST_SCHOOL => {
                4
            }
            Self::REFLECT_SPELLS_SCHOOL => {
                4
            }
            Self::MOD_LANGUAGE => {
                4
            }
            Self::FAR_SIGHT => {
                4
            }
            Self::MECHANIC_IMMUNITY => {
                4
            }
            Self::MOUNTED => {
                4
            }
            Self::MOD_DAMAGE_PERCENT_DONE => {
                4
            }
            Self::MOD_PERCENT_STAT => {
                4
            }
            Self::SPLIT_DAMAGE_PCT => {
                4
            }
            Self::WATER_BREATHING => {
                4
            }
            Self::MOD_BASE_RESISTANCE => {
                4
            }
            Self::MOD_REGEN => {
                4
            }
            Self::MOD_POWER_REGEN => {
                4
            }
            Self::CHANNEL_DEATH_ITEM => {
                4
            }
            Self::MOD_DAMAGE_PERCENT_TAKEN => {
                4
            }
            Self::MOD_HEALTH_REGEN_PERCENT => {
                4
            }
            Self::PERIODIC_DAMAGE_PERCENT {
                absorbed,
                damage1,
                resisted,
                school,
            } => {
                4
                + 4 // absorbed: u32
                + 4 // damage1: u32
                + 4 // resisted: u32
                + 1 // school: SpellSchool
            }
            Self::MOD_RESIST_CHANCE => {
                4
            }
            Self::MOD_DETECT_RANGE => {
                4
            }
            Self::PREVENTS_FLEEING => {
                4
            }
            Self::MOD_UNATTACKABLE => {
                4
            }
            Self::INTERRUPT_REGEN => {
                4
            }
            Self::GHOST => {
                4
            }
            Self::SPELL_MAGNET => {
                4
            }
            Self::MANA_SHIELD => {
                4
            }
            Self::MOD_SKILL_TALENT => {
                4
            }
            Self::MOD_ATTACK_POWER => {
                4
            }
            Self::AURAS_VISIBLE => {
                4
            }
            Self::MOD_RESISTANCE_PCT => {
                4
            }
            Self::MOD_MELEE_ATTACK_POWER_VERSUS => {
                4
            }
            Self::MOD_TOTAL_THREAT => {
                4
            }
            Self::WATER_WALK => {
                4
            }
            Self::FEATHER_FALL => {
                4
            }
            Self::HOVER => {
                4
            }
            Self::ADD_FLAT_MODIFIER => {
                4
            }
            Self::ADD_PCT_MODIFIER => {
                4
            }
            Self::ADD_TARGET_TRIGGER => {
                4
            }
            Self::MOD_POWER_REGEN_PERCENT => {
                4
            }
            Self::ADD_CASTER_HIT_TRIGGER => {
                4
            }
            Self::OVERRIDE_CLASS_SCRIPTS => {
                4
            }
            Self::MOD_RANGED_DAMAGE_TAKEN => {
                4
            }
            Self::MOD_RANGED_DAMAGE_TAKEN_PCT => {
                4
            }
            Self::MOD_HEALING => {
                4
            }
            Self::MOD_REGEN_DURING_COMBAT => {
                4
            }
            Self::MOD_MECHANIC_RESISTANCE => {
                4
            }
            Self::MOD_HEALING_PCT => {
                4
            }
            Self::SHARE_PET_TRACKING => {
                4
            }
            Self::UNTRACKABLE => {
                4
            }
            Self::EMPATHY => {
                4
            }
            Self::MOD_OFFHAND_DAMAGE_PCT => {
                4
            }
            Self::MOD_TARGET_RESISTANCE => {
                4
            }
            Self::MOD_RANGED_ATTACK_POWER => {
                4
            }
            Self::MOD_MELEE_DAMAGE_TAKEN => {
                4
            }
            Self::MOD_MELEE_DAMAGE_TAKEN_PCT => {
                4
            }
            Self::RANGED_ATTACK_POWER_ATTACKER_BONUS => {
                4
            }
            Self::MOD_POSSESS_PET => {
                4
            }
            Self::MOD_SPEED_ALWAYS => {
                4
            }
            Self::MOD_MOUNTED_SPEED_ALWAYS => {
                4
            }
            Self::MOD_RANGED_ATTACK_POWER_VERSUS => {
                4
            }
            Self::MOD_INCREASE_ENERGY_PERCENT => {
                4
            }
            Self::MOD_INCREASE_HEALTH_PERCENT => {
                4
            }
            Self::MOD_MANA_REGEN_INTERRUPT => {
                4
            }
            Self::MOD_HEALING_DONE => {
                4
            }
            Self::MOD_HEALING_DONE_PERCENT => {
                4
            }
            Self::MOD_TOTAL_STAT_PERCENTAGE => {
                4
            }
            Self::MOD_MELEE_HASTE => {
                4
            }
            Self::FORCE_REACTION => {
                4
            }
            Self::MOD_RANGED_HASTE => {
                4
            }
            Self::MOD_RANGED_AMMO_HASTE => {
                4
            }
            Self::MOD_BASE_RESISTANCE_PCT => {
                4
            }
            Self::MOD_RESISTANCE_EXCLUSIVE => {
                4
            }
            Self::SAFE_FALL => {
                4
            }
            Self::CHARISMA => {
                4
            }
            Self::PERSUADED => {
                4
            }
            Self::MECHANIC_IMMUNITY_MASK => {
                4
            }
            Self::RETAIN_COMBO_POINTS => {
                4
            }
            Self::RESIST_PUSHBACK => {
                4
            }
            Self::MOD_SHIELD_BLOCKVALUE_PCT => {
                4
            }
            Self::TRACK_STEALTHED => {
                4
            }
            Self::MOD_DETECTED_RANGE => {
                4
            }
            Self::SPLIT_DAMAGE_FLAT => {
                4
            }
            Self::MOD_STEALTH_LEVEL => {
                4
            }
            Self::MOD_WATER_BREATHING => {
                4
            }
            Self::MOD_REPUTATION_GAIN => {
                4
            }
            Self::PET_DAMAGE_MULTI => {
                4
            }
            Self::MOD_SHIELD_BLOCKVALUE => {
                4
            }
            Self::NO_PVP_CREDIT => {
                4
            }
            Self::MOD_AOE_AVOIDANCE => {
                4
            }
            Self::MOD_HEALTH_REGEN_IN_COMBAT => {
                4
            }
            Self::POWER_BURN_MANA => {
                4
            }
            Self::MOD_CRIT_DAMAGE_BONUS => {
                4
            }
            Self::UNKNOWN164 => {
                4
            }
            Self::MELEE_ATTACK_POWER_ATTACKER_BONUS => {
                4
            }
            Self::MOD_ATTACK_POWER_PCT => {
                4
            }
            Self::MOD_RANGED_ATTACK_POWER_PCT => {
                4
            }
            Self::MOD_DAMAGE_DONE_VERSUS => {
                4
            }
            Self::MOD_CRIT_PERCENT_VERSUS => {
                4
            }
            Self::DETECT_AMORE => {
                4
            }
            Self::MOD_SPEED_NOT_STACK => {
                4
            }
            Self::MOD_MOUNTED_SPEED_NOT_STACK => {
                4
            }
            Self::ALLOW_CHAMPION_SPELLS => {
                4
            }
            Self::MOD_SPELL_DAMAGE_OF_STAT_PERCENT => {
                4
            }
            Self::MOD_SPELL_HEALING_OF_STAT_PERCENT => {
                4
            }
            Self::SPIRIT_OF_REDEMPTION => {
                4
            }
            Self::AOE_CHARM => {
                4
            }
            Self::MOD_DEBUFF_RESISTANCE => {
                4
            }
            Self::MOD_ATTACKER_SPELL_CRIT_CHANCE => {
                4
            }
            Self::MOD_FLAT_SPELL_DAMAGE_VERSUS => {
                4
            }
            Self::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS => {
                4
            }
            Self::MOD_RESISTANCE_OF_STAT_PERCENT => {
                4
            }
            Self::MOD_CRITICAL_THREAT => {
                4
            }
            Self::MOD_ATTACKER_MELEE_HIT_CHANCE => {
                4
            }
            Self::MOD_ATTACKER_RANGED_HIT_CHANCE => {
                4
            }
            Self::MOD_ATTACKER_SPELL_HIT_CHANCE => {
                4
            }
            Self::MOD_ATTACKER_MELEE_CRIT_CHANCE => {
                4
            }
            Self::MOD_ATTACKER_RANGED_CRIT_CHANCE => {
                4
            }
            Self::MOD_RATING => {
                4
            }
            Self::MOD_FACTION_REPUTATION_GAIN => {
                4
            }
            Self::USE_NORMAL_MOVEMENT_SPEED => {
                4
            }
        }
    }
}

impl MaximumPossibleSized for AuraLogAuraType {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

