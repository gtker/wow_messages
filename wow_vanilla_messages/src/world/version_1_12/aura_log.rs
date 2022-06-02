use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::AuraType;
use crate::world::version_1_12::SpellSchool;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm:266`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm#L266):
/// ```text
/// struct AuraLog {
///     AuraType aura_type;
///     if (aura_type == PERIODIC_DAMAGE
///         || aura_type == PERIODIC_DAMAGE_PERCENT) {
///         u32 damage1;
///         SpellSchool school;
///         u32 absorbed;
///         u32 resisted;
///     }
///     else if (aura_type == PERIODIC_HEAL
///         || aura_type == OBS_MOD_HEALTH) {
///         u32 damage2;
///     }
///     else if (aura_type == OBS_MOD_MANA
///         || aura_type == PERIODIC_ENERGIZE) {
///         u32 misc_value1;
///         u32 damage3;
///     }
///     else if (aura_type == PERIODIC_MANA_LEECH) {
///         u32 misc_value2;
///         u32 damage;
///         f32 gain_multiplier;
///     }
/// }
/// ```
pub struct AuraLog {
    pub aura_type: AuraLog_AuraType,
}

impl AuraLog {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // aura_type: AuraType
        w.write_all(&(self.aura_type.as_int() as u32).to_le_bytes())?;

        match &self.aura_type {
            AuraLog_AuraType::NONE => {}
            AuraLog_AuraType::BIND_SIGHT => {}
            AuraLog_AuraType::MOD_POSSESS => {}
            AuraLog_AuraType::PERIODIC_DAMAGE {
                absorbed,
                damage1,
                resisted,
                school,
            } => {
                // damage1: u32
                w.write_all(&damage1.to_le_bytes())?;

                // school: SpellSchool
                w.write_all(&(school.as_int() as u8).to_le_bytes())?;

                // absorbed: u32
                w.write_all(&absorbed.to_le_bytes())?;

                // resisted: u32
                w.write_all(&resisted.to_le_bytes())?;

            }
            AuraLog_AuraType::DUMMY => {}
            AuraLog_AuraType::MOD_CONFUSE => {}
            AuraLog_AuraType::MOD_CHARM => {}
            AuraLog_AuraType::MOD_FEAR => {}
            AuraLog_AuraType::PERIODIC_HEAL {
                damage2,
            } => {
                // damage2: u32
                w.write_all(&damage2.to_le_bytes())?;

            }
            AuraLog_AuraType::MOD_ATTACKSPEED => {}
            AuraLog_AuraType::MOD_THREAT => {}
            AuraLog_AuraType::MOD_TAUNT => {}
            AuraLog_AuraType::MOD_STUN => {}
            AuraLog_AuraType::MOD_DAMAGE_DONE => {}
            AuraLog_AuraType::MOD_DAMAGE_TAKEN => {}
            AuraLog_AuraType::DAMAGE_SHIELD => {}
            AuraLog_AuraType::MOD_STEALTH => {}
            AuraLog_AuraType::MOD_STEALTH_DETECT => {}
            AuraLog_AuraType::MOD_INVISIBILITY => {}
            AuraLog_AuraType::MOD_INVISIBILITY_DETECTION => {}
            AuraLog_AuraType::OBS_MOD_HEALTH {
                damage2,
            } => {
                // damage2: u32
                w.write_all(&damage2.to_le_bytes())?;

            }
            AuraLog_AuraType::OBS_MOD_MANA {
                damage3,
                misc_value1,
            } => {
                // misc_value1: u32
                w.write_all(&misc_value1.to_le_bytes())?;

                // damage3: u32
                w.write_all(&damage3.to_le_bytes())?;

            }
            AuraLog_AuraType::MOD_RESISTANCE => {}
            AuraLog_AuraType::PERIODIC_TRIGGER_SPELL => {}
            AuraLog_AuraType::PERIODIC_ENERGIZE {
                damage3,
                misc_value1,
            } => {
                // misc_value1: u32
                w.write_all(&misc_value1.to_le_bytes())?;

                // damage3: u32
                w.write_all(&damage3.to_le_bytes())?;

            }
            AuraLog_AuraType::MOD_PACIFY => {}
            AuraLog_AuraType::MOD_ROOT => {}
            AuraLog_AuraType::MOD_SILENCE => {}
            AuraLog_AuraType::REFLECT_SPELLS => {}
            AuraLog_AuraType::MOD_STAT => {}
            AuraLog_AuraType::MOD_SKILL => {}
            AuraLog_AuraType::MOD_INCREASE_SPEED => {}
            AuraLog_AuraType::MOD_INCREASE_MOUNTED_SPEED => {}
            AuraLog_AuraType::MOD_DECREASE_SPEED => {}
            AuraLog_AuraType::MOD_INCREASE_HEALTH => {}
            AuraLog_AuraType::MOD_INCREASE_ENERGY => {}
            AuraLog_AuraType::MOD_SHAPESHIFT => {}
            AuraLog_AuraType::EFFECT_IMMUNITY => {}
            AuraLog_AuraType::STATE_IMMUNITY => {}
            AuraLog_AuraType::SCHOOL_IMMUNITY => {}
            AuraLog_AuraType::DAMAGE_IMMUNITY => {}
            AuraLog_AuraType::DISPEL_IMMUNITY => {}
            AuraLog_AuraType::PROC_TRIGGER_SPELL => {}
            AuraLog_AuraType::PROC_TRIGGER_DAMAGE => {}
            AuraLog_AuraType::TRACK_CREATURES => {}
            AuraLog_AuraType::TRACK_RESOURCES => {}
            AuraLog_AuraType::UNKNOWN46 => {}
            AuraLog_AuraType::MOD_PARRY_PERCENT => {}
            AuraLog_AuraType::UNKNOWN48 => {}
            AuraLog_AuraType::MOD_DODGE_PERCENT => {}
            AuraLog_AuraType::MOD_BLOCK_SKILL => {}
            AuraLog_AuraType::MOD_BLOCK_PERCENT => {}
            AuraLog_AuraType::MOD_CRIT_PERCENT => {}
            AuraLog_AuraType::PERIODIC_LEECH => {}
            AuraLog_AuraType::MOD_HIT_CHANCE => {}
            AuraLog_AuraType::MOD_SPELL_HIT_CHANCE => {}
            AuraLog_AuraType::TRANSFORM => {}
            AuraLog_AuraType::MOD_SPELL_CRIT_CHANCE => {}
            AuraLog_AuraType::MOD_INCREASE_SWIM_SPEED => {}
            AuraLog_AuraType::MOD_DAMAGE_DONE_CREATURE => {}
            AuraLog_AuraType::MOD_PACIFY_SILENCE => {}
            AuraLog_AuraType::MOD_SCALE => {}
            AuraLog_AuraType::PERIODIC_HEALTH_FUNNEL => {}
            AuraLog_AuraType::PERIODIC_MANA_FUNNEL => {}
            AuraLog_AuraType::PERIODIC_MANA_LEECH {
                damage,
                gain_multiplier,
                misc_value2,
            } => {
                // misc_value2: u32
                w.write_all(&misc_value2.to_le_bytes())?;

                // damage: u32
                w.write_all(&damage.to_le_bytes())?;

                // gain_multiplier: f32
                w.write_all(&gain_multiplier.to_le_bytes())?;

            }
            AuraLog_AuraType::MOD_CASTING_SPEED_NOT_STACK => {}
            AuraLog_AuraType::FEIGN_DEATH => {}
            AuraLog_AuraType::MOD_DISARM => {}
            AuraLog_AuraType::MOD_STALKED => {}
            AuraLog_AuraType::SCHOOL_ABSORB => {}
            AuraLog_AuraType::EXTRA_ATTACKS => {}
            AuraLog_AuraType::MOD_SPELL_CRIT_CHANCE_SCHOOL => {}
            AuraLog_AuraType::MOD_POWER_COST_SCHOOL_PCT => {}
            AuraLog_AuraType::MOD_POWER_COST_SCHOOL => {}
            AuraLog_AuraType::REFLECT_SPELLS_SCHOOL => {}
            AuraLog_AuraType::MOD_LANGUAGE => {}
            AuraLog_AuraType::FAR_SIGHT => {}
            AuraLog_AuraType::MECHANIC_IMMUNITY => {}
            AuraLog_AuraType::MOUNTED => {}
            AuraLog_AuraType::MOD_DAMAGE_PERCENT_DONE => {}
            AuraLog_AuraType::MOD_PERCENT_STAT => {}
            AuraLog_AuraType::SPLIT_DAMAGE_PCT => {}
            AuraLog_AuraType::WATER_BREATHING => {}
            AuraLog_AuraType::MOD_BASE_RESISTANCE => {}
            AuraLog_AuraType::MOD_REGEN => {}
            AuraLog_AuraType::MOD_POWER_REGEN => {}
            AuraLog_AuraType::CHANNEL_DEATH_ITEM => {}
            AuraLog_AuraType::MOD_DAMAGE_PERCENT_TAKEN => {}
            AuraLog_AuraType::MOD_HEALTH_REGEN_PERCENT => {}
            AuraLog_AuraType::PERIODIC_DAMAGE_PERCENT {
                absorbed,
                damage1,
                resisted,
                school,
            } => {
                // damage1: u32
                w.write_all(&damage1.to_le_bytes())?;

                // school: SpellSchool
                w.write_all(&(school.as_int() as u8).to_le_bytes())?;

                // absorbed: u32
                w.write_all(&absorbed.to_le_bytes())?;

                // resisted: u32
                w.write_all(&resisted.to_le_bytes())?;

            }
            AuraLog_AuraType::MOD_RESIST_CHANCE => {}
            AuraLog_AuraType::MOD_DETECT_RANGE => {}
            AuraLog_AuraType::PREVENTS_FLEEING => {}
            AuraLog_AuraType::MOD_UNATTACKABLE => {}
            AuraLog_AuraType::INTERRUPT_REGEN => {}
            AuraLog_AuraType::GHOST => {}
            AuraLog_AuraType::SPELL_MAGNET => {}
            AuraLog_AuraType::MANA_SHIELD => {}
            AuraLog_AuraType::MOD_SKILL_TALENT => {}
            AuraLog_AuraType::MOD_ATTACK_POWER => {}
            AuraLog_AuraType::AURAS_VISIBLE => {}
            AuraLog_AuraType::MOD_RESISTANCE_PCT => {}
            AuraLog_AuraType::MOD_MELEE_ATTACK_POWER_VERSUS => {}
            AuraLog_AuraType::MOD_TOTAL_THREAT => {}
            AuraLog_AuraType::WATER_WALK => {}
            AuraLog_AuraType::FEATHER_FALL => {}
            AuraLog_AuraType::HOVER => {}
            AuraLog_AuraType::ADD_FLAT_MODIFIER => {}
            AuraLog_AuraType::ADD_PCT_MODIFIER => {}
            AuraLog_AuraType::ADD_TARGET_TRIGGER => {}
            AuraLog_AuraType::MOD_POWER_REGEN_PERCENT => {}
            AuraLog_AuraType::ADD_CASTER_HIT_TRIGGER => {}
            AuraLog_AuraType::OVERRIDE_CLASS_SCRIPTS => {}
            AuraLog_AuraType::MOD_RANGED_DAMAGE_TAKEN => {}
            AuraLog_AuraType::MOD_RANGED_DAMAGE_TAKEN_PCT => {}
            AuraLog_AuraType::MOD_HEALING => {}
            AuraLog_AuraType::MOD_REGEN_DURING_COMBAT => {}
            AuraLog_AuraType::MOD_MECHANIC_RESISTANCE => {}
            AuraLog_AuraType::MOD_HEALING_PCT => {}
            AuraLog_AuraType::SHARE_PET_TRACKING => {}
            AuraLog_AuraType::UNTRACKABLE => {}
            AuraLog_AuraType::EMPATHY => {}
            AuraLog_AuraType::MOD_OFFHAND_DAMAGE_PCT => {}
            AuraLog_AuraType::MOD_TARGET_RESISTANCE => {}
            AuraLog_AuraType::MOD_RANGED_ATTACK_POWER => {}
            AuraLog_AuraType::MOD_MELEE_DAMAGE_TAKEN => {}
            AuraLog_AuraType::MOD_MELEE_DAMAGE_TAKEN_PCT => {}
            AuraLog_AuraType::RANGED_ATTACK_POWER_ATTACKER_BONUS => {}
            AuraLog_AuraType::MOD_POSSESS_PET => {}
            AuraLog_AuraType::MOD_SPEED_ALWAYS => {}
            AuraLog_AuraType::MOD_MOUNTED_SPEED_ALWAYS => {}
            AuraLog_AuraType::MOD_RANGED_ATTACK_POWER_VERSUS => {}
            AuraLog_AuraType::MOD_INCREASE_ENERGY_PERCENT => {}
            AuraLog_AuraType::MOD_INCREASE_HEALTH_PERCENT => {}
            AuraLog_AuraType::MOD_MANA_REGEN_INTERRUPT => {}
            AuraLog_AuraType::MOD_HEALING_DONE => {}
            AuraLog_AuraType::MOD_HEALING_DONE_PERCENT => {}
            AuraLog_AuraType::MOD_TOTAL_STAT_PERCENTAGE => {}
            AuraLog_AuraType::MOD_MELEE_HASTE => {}
            AuraLog_AuraType::FORCE_REACTION => {}
            AuraLog_AuraType::MOD_RANGED_HASTE => {}
            AuraLog_AuraType::MOD_RANGED_AMMO_HASTE => {}
            AuraLog_AuraType::MOD_BASE_RESISTANCE_PCT => {}
            AuraLog_AuraType::MOD_RESISTANCE_EXCLUSIVE => {}
            AuraLog_AuraType::SAFE_FALL => {}
            AuraLog_AuraType::CHARISMA => {}
            AuraLog_AuraType::PERSUADED => {}
            AuraLog_AuraType::MECHANIC_IMMUNITY_MASK => {}
            AuraLog_AuraType::RETAIN_COMBO_POINTS => {}
            AuraLog_AuraType::RESIST_PUSHBACK => {}
            AuraLog_AuraType::MOD_SHIELD_BLOCKVALUE_PCT => {}
            AuraLog_AuraType::TRACK_STEALTHED => {}
            AuraLog_AuraType::MOD_DETECTED_RANGE => {}
            AuraLog_AuraType::SPLIT_DAMAGE_FLAT => {}
            AuraLog_AuraType::MOD_STEALTH_LEVEL => {}
            AuraLog_AuraType::MOD_WATER_BREATHING => {}
            AuraLog_AuraType::MOD_REPUTATION_GAIN => {}
            AuraLog_AuraType::PET_DAMAGE_MULTI => {}
            AuraLog_AuraType::MOD_SHIELD_BLOCKVALUE => {}
            AuraLog_AuraType::NO_PVP_CREDIT => {}
            AuraLog_AuraType::MOD_AOE_AVOIDANCE => {}
            AuraLog_AuraType::MOD_HEALTH_REGEN_IN_COMBAT => {}
            AuraLog_AuraType::POWER_BURN_MANA => {}
            AuraLog_AuraType::MOD_CRIT_DAMAGE_BONUS => {}
            AuraLog_AuraType::UNKNOWN164 => {}
            AuraLog_AuraType::MELEE_ATTACK_POWER_ATTACKER_BONUS => {}
            AuraLog_AuraType::MOD_ATTACK_POWER_PCT => {}
            AuraLog_AuraType::MOD_RANGED_ATTACK_POWER_PCT => {}
            AuraLog_AuraType::MOD_DAMAGE_DONE_VERSUS => {}
            AuraLog_AuraType::MOD_CRIT_PERCENT_VERSUS => {}
            AuraLog_AuraType::DETECT_AMORE => {}
            AuraLog_AuraType::MOD_SPEED_NOT_STACK => {}
            AuraLog_AuraType::MOD_MOUNTED_SPEED_NOT_STACK => {}
            AuraLog_AuraType::ALLOW_CHAMPION_SPELLS => {}
            AuraLog_AuraType::MOD_SPELL_DAMAGE_OF_STAT_PERCENT => {}
            AuraLog_AuraType::MOD_SPELL_HEALING_OF_STAT_PERCENT => {}
            AuraLog_AuraType::SPIRIT_OF_REDEMPTION => {}
            AuraLog_AuraType::AOE_CHARM => {}
            AuraLog_AuraType::MOD_DEBUFF_RESISTANCE => {}
            AuraLog_AuraType::MOD_ATTACKER_SPELL_CRIT_CHANCE => {}
            AuraLog_AuraType::MOD_FLAT_SPELL_DAMAGE_VERSUS => {}
            AuraLog_AuraType::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS => {}
            AuraLog_AuraType::MOD_RESISTANCE_OF_STAT_PERCENT => {}
            AuraLog_AuraType::MOD_CRITICAL_THREAT => {}
            AuraLog_AuraType::MOD_ATTACKER_MELEE_HIT_CHANCE => {}
            AuraLog_AuraType::MOD_ATTACKER_RANGED_HIT_CHANCE => {}
            AuraLog_AuraType::MOD_ATTACKER_SPELL_HIT_CHANCE => {}
            AuraLog_AuraType::MOD_ATTACKER_MELEE_CRIT_CHANCE => {}
            AuraLog_AuraType::MOD_ATTACKER_RANGED_CRIT_CHANCE => {}
            AuraLog_AuraType::MOD_RATING => {}
            AuraLog_AuraType::MOD_FACTION_REPUTATION_GAIN => {}
            AuraLog_AuraType::USE_NORMAL_MOVEMENT_SPEED => {}
        }

        Ok(())
    }
}

impl AuraLog {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // aura_type: AuraType
        let aura_type: AuraType = crate::util::read_u32_le(r)?.try_into()?;

        let aura_type_if = match aura_type {
            AuraType::NONE => AuraLog_AuraType::NONE,
            AuraType::BIND_SIGHT => AuraLog_AuraType::BIND_SIGHT,
            AuraType::MOD_POSSESS => AuraLog_AuraType::MOD_POSSESS,
            AuraType::PERIODIC_DAMAGE => {
                // damage1: u32
                let damage1 = crate::util::read_u32_le(r)?;

                // school: SpellSchool
                let school: SpellSchool = crate::util::read_u8_le(r)?.try_into()?;

                // absorbed: u32
                let absorbed = crate::util::read_u32_le(r)?;

                // resisted: u32
                let resisted = crate::util::read_u32_le(r)?;

                AuraLog_AuraType::PERIODIC_DAMAGE {
                    absorbed,
                    damage1,
                    resisted,
                    school,
                }
            }
            AuraType::DUMMY => AuraLog_AuraType::DUMMY,
            AuraType::MOD_CONFUSE => AuraLog_AuraType::MOD_CONFUSE,
            AuraType::MOD_CHARM => AuraLog_AuraType::MOD_CHARM,
            AuraType::MOD_FEAR => AuraLog_AuraType::MOD_FEAR,
            AuraType::PERIODIC_HEAL => {
                // damage2: u32
                let damage2 = crate::util::read_u32_le(r)?;

                AuraLog_AuraType::PERIODIC_HEAL {
                    damage2,
                }
            }
            AuraType::MOD_ATTACKSPEED => AuraLog_AuraType::MOD_ATTACKSPEED,
            AuraType::MOD_THREAT => AuraLog_AuraType::MOD_THREAT,
            AuraType::MOD_TAUNT => AuraLog_AuraType::MOD_TAUNT,
            AuraType::MOD_STUN => AuraLog_AuraType::MOD_STUN,
            AuraType::MOD_DAMAGE_DONE => AuraLog_AuraType::MOD_DAMAGE_DONE,
            AuraType::MOD_DAMAGE_TAKEN => AuraLog_AuraType::MOD_DAMAGE_TAKEN,
            AuraType::DAMAGE_SHIELD => AuraLog_AuraType::DAMAGE_SHIELD,
            AuraType::MOD_STEALTH => AuraLog_AuraType::MOD_STEALTH,
            AuraType::MOD_STEALTH_DETECT => AuraLog_AuraType::MOD_STEALTH_DETECT,
            AuraType::MOD_INVISIBILITY => AuraLog_AuraType::MOD_INVISIBILITY,
            AuraType::MOD_INVISIBILITY_DETECTION => AuraLog_AuraType::MOD_INVISIBILITY_DETECTION,
            AuraType::OBS_MOD_HEALTH => {
                // damage2: u32
                let damage2 = crate::util::read_u32_le(r)?;

                AuraLog_AuraType::OBS_MOD_HEALTH {
                    damage2,
                }
            }
            AuraType::OBS_MOD_MANA => {
                // misc_value1: u32
                let misc_value1 = crate::util::read_u32_le(r)?;

                // damage3: u32
                let damage3 = crate::util::read_u32_le(r)?;

                AuraLog_AuraType::OBS_MOD_MANA {
                    damage3,
                    misc_value1,
                }
            }
            AuraType::MOD_RESISTANCE => AuraLog_AuraType::MOD_RESISTANCE,
            AuraType::PERIODIC_TRIGGER_SPELL => AuraLog_AuraType::PERIODIC_TRIGGER_SPELL,
            AuraType::PERIODIC_ENERGIZE => {
                // misc_value1: u32
                let misc_value1 = crate::util::read_u32_le(r)?;

                // damage3: u32
                let damage3 = crate::util::read_u32_le(r)?;

                AuraLog_AuraType::PERIODIC_ENERGIZE {
                    damage3,
                    misc_value1,
                }
            }
            AuraType::MOD_PACIFY => AuraLog_AuraType::MOD_PACIFY,
            AuraType::MOD_ROOT => AuraLog_AuraType::MOD_ROOT,
            AuraType::MOD_SILENCE => AuraLog_AuraType::MOD_SILENCE,
            AuraType::REFLECT_SPELLS => AuraLog_AuraType::REFLECT_SPELLS,
            AuraType::MOD_STAT => AuraLog_AuraType::MOD_STAT,
            AuraType::MOD_SKILL => AuraLog_AuraType::MOD_SKILL,
            AuraType::MOD_INCREASE_SPEED => AuraLog_AuraType::MOD_INCREASE_SPEED,
            AuraType::MOD_INCREASE_MOUNTED_SPEED => AuraLog_AuraType::MOD_INCREASE_MOUNTED_SPEED,
            AuraType::MOD_DECREASE_SPEED => AuraLog_AuraType::MOD_DECREASE_SPEED,
            AuraType::MOD_INCREASE_HEALTH => AuraLog_AuraType::MOD_INCREASE_HEALTH,
            AuraType::MOD_INCREASE_ENERGY => AuraLog_AuraType::MOD_INCREASE_ENERGY,
            AuraType::MOD_SHAPESHIFT => AuraLog_AuraType::MOD_SHAPESHIFT,
            AuraType::EFFECT_IMMUNITY => AuraLog_AuraType::EFFECT_IMMUNITY,
            AuraType::STATE_IMMUNITY => AuraLog_AuraType::STATE_IMMUNITY,
            AuraType::SCHOOL_IMMUNITY => AuraLog_AuraType::SCHOOL_IMMUNITY,
            AuraType::DAMAGE_IMMUNITY => AuraLog_AuraType::DAMAGE_IMMUNITY,
            AuraType::DISPEL_IMMUNITY => AuraLog_AuraType::DISPEL_IMMUNITY,
            AuraType::PROC_TRIGGER_SPELL => AuraLog_AuraType::PROC_TRIGGER_SPELL,
            AuraType::PROC_TRIGGER_DAMAGE => AuraLog_AuraType::PROC_TRIGGER_DAMAGE,
            AuraType::TRACK_CREATURES => AuraLog_AuraType::TRACK_CREATURES,
            AuraType::TRACK_RESOURCES => AuraLog_AuraType::TRACK_RESOURCES,
            AuraType::UNKNOWN46 => AuraLog_AuraType::UNKNOWN46,
            AuraType::MOD_PARRY_PERCENT => AuraLog_AuraType::MOD_PARRY_PERCENT,
            AuraType::UNKNOWN48 => AuraLog_AuraType::UNKNOWN48,
            AuraType::MOD_DODGE_PERCENT => AuraLog_AuraType::MOD_DODGE_PERCENT,
            AuraType::MOD_BLOCK_SKILL => AuraLog_AuraType::MOD_BLOCK_SKILL,
            AuraType::MOD_BLOCK_PERCENT => AuraLog_AuraType::MOD_BLOCK_PERCENT,
            AuraType::MOD_CRIT_PERCENT => AuraLog_AuraType::MOD_CRIT_PERCENT,
            AuraType::PERIODIC_LEECH => AuraLog_AuraType::PERIODIC_LEECH,
            AuraType::MOD_HIT_CHANCE => AuraLog_AuraType::MOD_HIT_CHANCE,
            AuraType::MOD_SPELL_HIT_CHANCE => AuraLog_AuraType::MOD_SPELL_HIT_CHANCE,
            AuraType::TRANSFORM => AuraLog_AuraType::TRANSFORM,
            AuraType::MOD_SPELL_CRIT_CHANCE => AuraLog_AuraType::MOD_SPELL_CRIT_CHANCE,
            AuraType::MOD_INCREASE_SWIM_SPEED => AuraLog_AuraType::MOD_INCREASE_SWIM_SPEED,
            AuraType::MOD_DAMAGE_DONE_CREATURE => AuraLog_AuraType::MOD_DAMAGE_DONE_CREATURE,
            AuraType::MOD_PACIFY_SILENCE => AuraLog_AuraType::MOD_PACIFY_SILENCE,
            AuraType::MOD_SCALE => AuraLog_AuraType::MOD_SCALE,
            AuraType::PERIODIC_HEALTH_FUNNEL => AuraLog_AuraType::PERIODIC_HEALTH_FUNNEL,
            AuraType::PERIODIC_MANA_FUNNEL => AuraLog_AuraType::PERIODIC_MANA_FUNNEL,
            AuraType::PERIODIC_MANA_LEECH => {
                // misc_value2: u32
                let misc_value2 = crate::util::read_u32_le(r)?;

                // damage: u32
                let damage = crate::util::read_u32_le(r)?;

                // gain_multiplier: f32
                let gain_multiplier = crate::util::read_f32_le(r)?;
                AuraLog_AuraType::PERIODIC_MANA_LEECH {
                    damage,
                    gain_multiplier,
                    misc_value2,
                }
            }
            AuraType::MOD_CASTING_SPEED_NOT_STACK => AuraLog_AuraType::MOD_CASTING_SPEED_NOT_STACK,
            AuraType::FEIGN_DEATH => AuraLog_AuraType::FEIGN_DEATH,
            AuraType::MOD_DISARM => AuraLog_AuraType::MOD_DISARM,
            AuraType::MOD_STALKED => AuraLog_AuraType::MOD_STALKED,
            AuraType::SCHOOL_ABSORB => AuraLog_AuraType::SCHOOL_ABSORB,
            AuraType::EXTRA_ATTACKS => AuraLog_AuraType::EXTRA_ATTACKS,
            AuraType::MOD_SPELL_CRIT_CHANCE_SCHOOL => AuraLog_AuraType::MOD_SPELL_CRIT_CHANCE_SCHOOL,
            AuraType::MOD_POWER_COST_SCHOOL_PCT => AuraLog_AuraType::MOD_POWER_COST_SCHOOL_PCT,
            AuraType::MOD_POWER_COST_SCHOOL => AuraLog_AuraType::MOD_POWER_COST_SCHOOL,
            AuraType::REFLECT_SPELLS_SCHOOL => AuraLog_AuraType::REFLECT_SPELLS_SCHOOL,
            AuraType::MOD_LANGUAGE => AuraLog_AuraType::MOD_LANGUAGE,
            AuraType::FAR_SIGHT => AuraLog_AuraType::FAR_SIGHT,
            AuraType::MECHANIC_IMMUNITY => AuraLog_AuraType::MECHANIC_IMMUNITY,
            AuraType::MOUNTED => AuraLog_AuraType::MOUNTED,
            AuraType::MOD_DAMAGE_PERCENT_DONE => AuraLog_AuraType::MOD_DAMAGE_PERCENT_DONE,
            AuraType::MOD_PERCENT_STAT => AuraLog_AuraType::MOD_PERCENT_STAT,
            AuraType::SPLIT_DAMAGE_PCT => AuraLog_AuraType::SPLIT_DAMAGE_PCT,
            AuraType::WATER_BREATHING => AuraLog_AuraType::WATER_BREATHING,
            AuraType::MOD_BASE_RESISTANCE => AuraLog_AuraType::MOD_BASE_RESISTANCE,
            AuraType::MOD_REGEN => AuraLog_AuraType::MOD_REGEN,
            AuraType::MOD_POWER_REGEN => AuraLog_AuraType::MOD_POWER_REGEN,
            AuraType::CHANNEL_DEATH_ITEM => AuraLog_AuraType::CHANNEL_DEATH_ITEM,
            AuraType::MOD_DAMAGE_PERCENT_TAKEN => AuraLog_AuraType::MOD_DAMAGE_PERCENT_TAKEN,
            AuraType::MOD_HEALTH_REGEN_PERCENT => AuraLog_AuraType::MOD_HEALTH_REGEN_PERCENT,
            AuraType::PERIODIC_DAMAGE_PERCENT => {
                // damage1: u32
                let damage1 = crate::util::read_u32_le(r)?;

                // school: SpellSchool
                let school: SpellSchool = crate::util::read_u8_le(r)?.try_into()?;

                // absorbed: u32
                let absorbed = crate::util::read_u32_le(r)?;

                // resisted: u32
                let resisted = crate::util::read_u32_le(r)?;

                AuraLog_AuraType::PERIODIC_DAMAGE_PERCENT {
                    absorbed,
                    damage1,
                    resisted,
                    school,
                }
            }
            AuraType::MOD_RESIST_CHANCE => AuraLog_AuraType::MOD_RESIST_CHANCE,
            AuraType::MOD_DETECT_RANGE => AuraLog_AuraType::MOD_DETECT_RANGE,
            AuraType::PREVENTS_FLEEING => AuraLog_AuraType::PREVENTS_FLEEING,
            AuraType::MOD_UNATTACKABLE => AuraLog_AuraType::MOD_UNATTACKABLE,
            AuraType::INTERRUPT_REGEN => AuraLog_AuraType::INTERRUPT_REGEN,
            AuraType::GHOST => AuraLog_AuraType::GHOST,
            AuraType::SPELL_MAGNET => AuraLog_AuraType::SPELL_MAGNET,
            AuraType::MANA_SHIELD => AuraLog_AuraType::MANA_SHIELD,
            AuraType::MOD_SKILL_TALENT => AuraLog_AuraType::MOD_SKILL_TALENT,
            AuraType::MOD_ATTACK_POWER => AuraLog_AuraType::MOD_ATTACK_POWER,
            AuraType::AURAS_VISIBLE => AuraLog_AuraType::AURAS_VISIBLE,
            AuraType::MOD_RESISTANCE_PCT => AuraLog_AuraType::MOD_RESISTANCE_PCT,
            AuraType::MOD_MELEE_ATTACK_POWER_VERSUS => AuraLog_AuraType::MOD_MELEE_ATTACK_POWER_VERSUS,
            AuraType::MOD_TOTAL_THREAT => AuraLog_AuraType::MOD_TOTAL_THREAT,
            AuraType::WATER_WALK => AuraLog_AuraType::WATER_WALK,
            AuraType::FEATHER_FALL => AuraLog_AuraType::FEATHER_FALL,
            AuraType::HOVER => AuraLog_AuraType::HOVER,
            AuraType::ADD_FLAT_MODIFIER => AuraLog_AuraType::ADD_FLAT_MODIFIER,
            AuraType::ADD_PCT_MODIFIER => AuraLog_AuraType::ADD_PCT_MODIFIER,
            AuraType::ADD_TARGET_TRIGGER => AuraLog_AuraType::ADD_TARGET_TRIGGER,
            AuraType::MOD_POWER_REGEN_PERCENT => AuraLog_AuraType::MOD_POWER_REGEN_PERCENT,
            AuraType::ADD_CASTER_HIT_TRIGGER => AuraLog_AuraType::ADD_CASTER_HIT_TRIGGER,
            AuraType::OVERRIDE_CLASS_SCRIPTS => AuraLog_AuraType::OVERRIDE_CLASS_SCRIPTS,
            AuraType::MOD_RANGED_DAMAGE_TAKEN => AuraLog_AuraType::MOD_RANGED_DAMAGE_TAKEN,
            AuraType::MOD_RANGED_DAMAGE_TAKEN_PCT => AuraLog_AuraType::MOD_RANGED_DAMAGE_TAKEN_PCT,
            AuraType::MOD_HEALING => AuraLog_AuraType::MOD_HEALING,
            AuraType::MOD_REGEN_DURING_COMBAT => AuraLog_AuraType::MOD_REGEN_DURING_COMBAT,
            AuraType::MOD_MECHANIC_RESISTANCE => AuraLog_AuraType::MOD_MECHANIC_RESISTANCE,
            AuraType::MOD_HEALING_PCT => AuraLog_AuraType::MOD_HEALING_PCT,
            AuraType::SHARE_PET_TRACKING => AuraLog_AuraType::SHARE_PET_TRACKING,
            AuraType::UNTRACKABLE => AuraLog_AuraType::UNTRACKABLE,
            AuraType::EMPATHY => AuraLog_AuraType::EMPATHY,
            AuraType::MOD_OFFHAND_DAMAGE_PCT => AuraLog_AuraType::MOD_OFFHAND_DAMAGE_PCT,
            AuraType::MOD_TARGET_RESISTANCE => AuraLog_AuraType::MOD_TARGET_RESISTANCE,
            AuraType::MOD_RANGED_ATTACK_POWER => AuraLog_AuraType::MOD_RANGED_ATTACK_POWER,
            AuraType::MOD_MELEE_DAMAGE_TAKEN => AuraLog_AuraType::MOD_MELEE_DAMAGE_TAKEN,
            AuraType::MOD_MELEE_DAMAGE_TAKEN_PCT => AuraLog_AuraType::MOD_MELEE_DAMAGE_TAKEN_PCT,
            AuraType::RANGED_ATTACK_POWER_ATTACKER_BONUS => AuraLog_AuraType::RANGED_ATTACK_POWER_ATTACKER_BONUS,
            AuraType::MOD_POSSESS_PET => AuraLog_AuraType::MOD_POSSESS_PET,
            AuraType::MOD_SPEED_ALWAYS => AuraLog_AuraType::MOD_SPEED_ALWAYS,
            AuraType::MOD_MOUNTED_SPEED_ALWAYS => AuraLog_AuraType::MOD_MOUNTED_SPEED_ALWAYS,
            AuraType::MOD_RANGED_ATTACK_POWER_VERSUS => AuraLog_AuraType::MOD_RANGED_ATTACK_POWER_VERSUS,
            AuraType::MOD_INCREASE_ENERGY_PERCENT => AuraLog_AuraType::MOD_INCREASE_ENERGY_PERCENT,
            AuraType::MOD_INCREASE_HEALTH_PERCENT => AuraLog_AuraType::MOD_INCREASE_HEALTH_PERCENT,
            AuraType::MOD_MANA_REGEN_INTERRUPT => AuraLog_AuraType::MOD_MANA_REGEN_INTERRUPT,
            AuraType::MOD_HEALING_DONE => AuraLog_AuraType::MOD_HEALING_DONE,
            AuraType::MOD_HEALING_DONE_PERCENT => AuraLog_AuraType::MOD_HEALING_DONE_PERCENT,
            AuraType::MOD_TOTAL_STAT_PERCENTAGE => AuraLog_AuraType::MOD_TOTAL_STAT_PERCENTAGE,
            AuraType::MOD_MELEE_HASTE => AuraLog_AuraType::MOD_MELEE_HASTE,
            AuraType::FORCE_REACTION => AuraLog_AuraType::FORCE_REACTION,
            AuraType::MOD_RANGED_HASTE => AuraLog_AuraType::MOD_RANGED_HASTE,
            AuraType::MOD_RANGED_AMMO_HASTE => AuraLog_AuraType::MOD_RANGED_AMMO_HASTE,
            AuraType::MOD_BASE_RESISTANCE_PCT => AuraLog_AuraType::MOD_BASE_RESISTANCE_PCT,
            AuraType::MOD_RESISTANCE_EXCLUSIVE => AuraLog_AuraType::MOD_RESISTANCE_EXCLUSIVE,
            AuraType::SAFE_FALL => AuraLog_AuraType::SAFE_FALL,
            AuraType::CHARISMA => AuraLog_AuraType::CHARISMA,
            AuraType::PERSUADED => AuraLog_AuraType::PERSUADED,
            AuraType::MECHANIC_IMMUNITY_MASK => AuraLog_AuraType::MECHANIC_IMMUNITY_MASK,
            AuraType::RETAIN_COMBO_POINTS => AuraLog_AuraType::RETAIN_COMBO_POINTS,
            AuraType::RESIST_PUSHBACK => AuraLog_AuraType::RESIST_PUSHBACK,
            AuraType::MOD_SHIELD_BLOCKVALUE_PCT => AuraLog_AuraType::MOD_SHIELD_BLOCKVALUE_PCT,
            AuraType::TRACK_STEALTHED => AuraLog_AuraType::TRACK_STEALTHED,
            AuraType::MOD_DETECTED_RANGE => AuraLog_AuraType::MOD_DETECTED_RANGE,
            AuraType::SPLIT_DAMAGE_FLAT => AuraLog_AuraType::SPLIT_DAMAGE_FLAT,
            AuraType::MOD_STEALTH_LEVEL => AuraLog_AuraType::MOD_STEALTH_LEVEL,
            AuraType::MOD_WATER_BREATHING => AuraLog_AuraType::MOD_WATER_BREATHING,
            AuraType::MOD_REPUTATION_GAIN => AuraLog_AuraType::MOD_REPUTATION_GAIN,
            AuraType::PET_DAMAGE_MULTI => AuraLog_AuraType::PET_DAMAGE_MULTI,
            AuraType::MOD_SHIELD_BLOCKVALUE => AuraLog_AuraType::MOD_SHIELD_BLOCKVALUE,
            AuraType::NO_PVP_CREDIT => AuraLog_AuraType::NO_PVP_CREDIT,
            AuraType::MOD_AOE_AVOIDANCE => AuraLog_AuraType::MOD_AOE_AVOIDANCE,
            AuraType::MOD_HEALTH_REGEN_IN_COMBAT => AuraLog_AuraType::MOD_HEALTH_REGEN_IN_COMBAT,
            AuraType::POWER_BURN_MANA => AuraLog_AuraType::POWER_BURN_MANA,
            AuraType::MOD_CRIT_DAMAGE_BONUS => AuraLog_AuraType::MOD_CRIT_DAMAGE_BONUS,
            AuraType::UNKNOWN164 => AuraLog_AuraType::UNKNOWN164,
            AuraType::MELEE_ATTACK_POWER_ATTACKER_BONUS => AuraLog_AuraType::MELEE_ATTACK_POWER_ATTACKER_BONUS,
            AuraType::MOD_ATTACK_POWER_PCT => AuraLog_AuraType::MOD_ATTACK_POWER_PCT,
            AuraType::MOD_RANGED_ATTACK_POWER_PCT => AuraLog_AuraType::MOD_RANGED_ATTACK_POWER_PCT,
            AuraType::MOD_DAMAGE_DONE_VERSUS => AuraLog_AuraType::MOD_DAMAGE_DONE_VERSUS,
            AuraType::MOD_CRIT_PERCENT_VERSUS => AuraLog_AuraType::MOD_CRIT_PERCENT_VERSUS,
            AuraType::DETECT_AMORE => AuraLog_AuraType::DETECT_AMORE,
            AuraType::MOD_SPEED_NOT_STACK => AuraLog_AuraType::MOD_SPEED_NOT_STACK,
            AuraType::MOD_MOUNTED_SPEED_NOT_STACK => AuraLog_AuraType::MOD_MOUNTED_SPEED_NOT_STACK,
            AuraType::ALLOW_CHAMPION_SPELLS => AuraLog_AuraType::ALLOW_CHAMPION_SPELLS,
            AuraType::MOD_SPELL_DAMAGE_OF_STAT_PERCENT => AuraLog_AuraType::MOD_SPELL_DAMAGE_OF_STAT_PERCENT,
            AuraType::MOD_SPELL_HEALING_OF_STAT_PERCENT => AuraLog_AuraType::MOD_SPELL_HEALING_OF_STAT_PERCENT,
            AuraType::SPIRIT_OF_REDEMPTION => AuraLog_AuraType::SPIRIT_OF_REDEMPTION,
            AuraType::AOE_CHARM => AuraLog_AuraType::AOE_CHARM,
            AuraType::MOD_DEBUFF_RESISTANCE => AuraLog_AuraType::MOD_DEBUFF_RESISTANCE,
            AuraType::MOD_ATTACKER_SPELL_CRIT_CHANCE => AuraLog_AuraType::MOD_ATTACKER_SPELL_CRIT_CHANCE,
            AuraType::MOD_FLAT_SPELL_DAMAGE_VERSUS => AuraLog_AuraType::MOD_FLAT_SPELL_DAMAGE_VERSUS,
            AuraType::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS => AuraLog_AuraType::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS,
            AuraType::MOD_RESISTANCE_OF_STAT_PERCENT => AuraLog_AuraType::MOD_RESISTANCE_OF_STAT_PERCENT,
            AuraType::MOD_CRITICAL_THREAT => AuraLog_AuraType::MOD_CRITICAL_THREAT,
            AuraType::MOD_ATTACKER_MELEE_HIT_CHANCE => AuraLog_AuraType::MOD_ATTACKER_MELEE_HIT_CHANCE,
            AuraType::MOD_ATTACKER_RANGED_HIT_CHANCE => AuraLog_AuraType::MOD_ATTACKER_RANGED_HIT_CHANCE,
            AuraType::MOD_ATTACKER_SPELL_HIT_CHANCE => AuraLog_AuraType::MOD_ATTACKER_SPELL_HIT_CHANCE,
            AuraType::MOD_ATTACKER_MELEE_CRIT_CHANCE => AuraLog_AuraType::MOD_ATTACKER_MELEE_CRIT_CHANCE,
            AuraType::MOD_ATTACKER_RANGED_CRIT_CHANCE => AuraLog_AuraType::MOD_ATTACKER_RANGED_CRIT_CHANCE,
            AuraType::MOD_RATING => AuraLog_AuraType::MOD_RATING,
            AuraType::MOD_FACTION_REPUTATION_GAIN => AuraLog_AuraType::MOD_FACTION_REPUTATION_GAIN,
            AuraType::USE_NORMAL_MOVEMENT_SPEED => AuraLog_AuraType::USE_NORMAL_MOVEMENT_SPEED,
        };

        Ok(Self {
            aura_type: aura_type_if,
        })
    }

}

impl AuraLog {
    pub(crate) fn size(&self) -> usize {
        self.aura_type.size() // aura_type: AuraLog_AuraType
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AuraLog_AuraType {
    NONE,
    BIND_SIGHT,
    MOD_POSSESS,
    PERIODIC_DAMAGE {
        absorbed: u32,
        damage1: u32,
        resisted: u32,
        school: SpellSchool,
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
        damage3: u32,
        misc_value1: u32,
    },
    MOD_RESISTANCE,
    PERIODIC_TRIGGER_SPELL,
    PERIODIC_ENERGIZE {
        damage3: u32,
        misc_value1: u32,
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
        damage: u32,
        gain_multiplier: f32,
        misc_value2: u32,
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
        absorbed: u32,
        damage1: u32,
        resisted: u32,
        school: SpellSchool,
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

impl Default for AuraLog_AuraType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NONE
    }
}

impl AuraLog_AuraType {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::NONE => 0,
            Self::BIND_SIGHT => 1,
            Self::MOD_POSSESS => 2,
            Self::PERIODIC_DAMAGE { .. } => 3,
            Self::DUMMY => 4,
            Self::MOD_CONFUSE => 5,
            Self::MOD_CHARM => 6,
            Self::MOD_FEAR => 7,
            Self::PERIODIC_HEAL { .. } => 8,
            Self::MOD_ATTACKSPEED => 9,
            Self::MOD_THREAT => 10,
            Self::MOD_TAUNT => 11,
            Self::MOD_STUN => 12,
            Self::MOD_DAMAGE_DONE => 13,
            Self::MOD_DAMAGE_TAKEN => 14,
            Self::DAMAGE_SHIELD => 15,
            Self::MOD_STEALTH => 16,
            Self::MOD_STEALTH_DETECT => 17,
            Self::MOD_INVISIBILITY => 18,
            Self::MOD_INVISIBILITY_DETECTION => 19,
            Self::OBS_MOD_HEALTH { .. } => 20,
            Self::OBS_MOD_MANA { .. } => 21,
            Self::MOD_RESISTANCE => 22,
            Self::PERIODIC_TRIGGER_SPELL => 23,
            Self::PERIODIC_ENERGIZE { .. } => 24,
            Self::MOD_PACIFY => 25,
            Self::MOD_ROOT => 26,
            Self::MOD_SILENCE => 27,
            Self::REFLECT_SPELLS => 28,
            Self::MOD_STAT => 29,
            Self::MOD_SKILL => 30,
            Self::MOD_INCREASE_SPEED => 31,
            Self::MOD_INCREASE_MOUNTED_SPEED => 32,
            Self::MOD_DECREASE_SPEED => 33,
            Self::MOD_INCREASE_HEALTH => 34,
            Self::MOD_INCREASE_ENERGY => 35,
            Self::MOD_SHAPESHIFT => 36,
            Self::EFFECT_IMMUNITY => 37,
            Self::STATE_IMMUNITY => 38,
            Self::SCHOOL_IMMUNITY => 39,
            Self::DAMAGE_IMMUNITY => 40,
            Self::DISPEL_IMMUNITY => 41,
            Self::PROC_TRIGGER_SPELL => 42,
            Self::PROC_TRIGGER_DAMAGE => 43,
            Self::TRACK_CREATURES => 44,
            Self::TRACK_RESOURCES => 45,
            Self::UNKNOWN46 => 46,
            Self::MOD_PARRY_PERCENT => 47,
            Self::UNKNOWN48 => 48,
            Self::MOD_DODGE_PERCENT => 49,
            Self::MOD_BLOCK_SKILL => 50,
            Self::MOD_BLOCK_PERCENT => 51,
            Self::MOD_CRIT_PERCENT => 52,
            Self::PERIODIC_LEECH => 53,
            Self::MOD_HIT_CHANCE => 54,
            Self::MOD_SPELL_HIT_CHANCE => 55,
            Self::TRANSFORM => 56,
            Self::MOD_SPELL_CRIT_CHANCE => 57,
            Self::MOD_INCREASE_SWIM_SPEED => 58,
            Self::MOD_DAMAGE_DONE_CREATURE => 59,
            Self::MOD_PACIFY_SILENCE => 60,
            Self::MOD_SCALE => 61,
            Self::PERIODIC_HEALTH_FUNNEL => 62,
            Self::PERIODIC_MANA_FUNNEL => 63,
            Self::PERIODIC_MANA_LEECH { .. } => 64,
            Self::MOD_CASTING_SPEED_NOT_STACK => 65,
            Self::FEIGN_DEATH => 66,
            Self::MOD_DISARM => 67,
            Self::MOD_STALKED => 68,
            Self::SCHOOL_ABSORB => 69,
            Self::EXTRA_ATTACKS => 70,
            Self::MOD_SPELL_CRIT_CHANCE_SCHOOL => 71,
            Self::MOD_POWER_COST_SCHOOL_PCT => 72,
            Self::MOD_POWER_COST_SCHOOL => 73,
            Self::REFLECT_SPELLS_SCHOOL => 74,
            Self::MOD_LANGUAGE => 75,
            Self::FAR_SIGHT => 76,
            Self::MECHANIC_IMMUNITY => 77,
            Self::MOUNTED => 78,
            Self::MOD_DAMAGE_PERCENT_DONE => 79,
            Self::MOD_PERCENT_STAT => 80,
            Self::SPLIT_DAMAGE_PCT => 81,
            Self::WATER_BREATHING => 82,
            Self::MOD_BASE_RESISTANCE => 83,
            Self::MOD_REGEN => 84,
            Self::MOD_POWER_REGEN => 85,
            Self::CHANNEL_DEATH_ITEM => 86,
            Self::MOD_DAMAGE_PERCENT_TAKEN => 87,
            Self::MOD_HEALTH_REGEN_PERCENT => 88,
            Self::PERIODIC_DAMAGE_PERCENT { .. } => 89,
            Self::MOD_RESIST_CHANCE => 90,
            Self::MOD_DETECT_RANGE => 91,
            Self::PREVENTS_FLEEING => 92,
            Self::MOD_UNATTACKABLE => 93,
            Self::INTERRUPT_REGEN => 94,
            Self::GHOST => 95,
            Self::SPELL_MAGNET => 96,
            Self::MANA_SHIELD => 97,
            Self::MOD_SKILL_TALENT => 98,
            Self::MOD_ATTACK_POWER => 99,
            Self::AURAS_VISIBLE => 100,
            Self::MOD_RESISTANCE_PCT => 101,
            Self::MOD_MELEE_ATTACK_POWER_VERSUS => 102,
            Self::MOD_TOTAL_THREAT => 103,
            Self::WATER_WALK => 104,
            Self::FEATHER_FALL => 105,
            Self::HOVER => 106,
            Self::ADD_FLAT_MODIFIER => 107,
            Self::ADD_PCT_MODIFIER => 108,
            Self::ADD_TARGET_TRIGGER => 109,
            Self::MOD_POWER_REGEN_PERCENT => 110,
            Self::ADD_CASTER_HIT_TRIGGER => 111,
            Self::OVERRIDE_CLASS_SCRIPTS => 112,
            Self::MOD_RANGED_DAMAGE_TAKEN => 113,
            Self::MOD_RANGED_DAMAGE_TAKEN_PCT => 114,
            Self::MOD_HEALING => 115,
            Self::MOD_REGEN_DURING_COMBAT => 116,
            Self::MOD_MECHANIC_RESISTANCE => 117,
            Self::MOD_HEALING_PCT => 118,
            Self::SHARE_PET_TRACKING => 119,
            Self::UNTRACKABLE => 120,
            Self::EMPATHY => 121,
            Self::MOD_OFFHAND_DAMAGE_PCT => 122,
            Self::MOD_TARGET_RESISTANCE => 123,
            Self::MOD_RANGED_ATTACK_POWER => 124,
            Self::MOD_MELEE_DAMAGE_TAKEN => 125,
            Self::MOD_MELEE_DAMAGE_TAKEN_PCT => 126,
            Self::RANGED_ATTACK_POWER_ATTACKER_BONUS => 127,
            Self::MOD_POSSESS_PET => 128,
            Self::MOD_SPEED_ALWAYS => 129,
            Self::MOD_MOUNTED_SPEED_ALWAYS => 130,
            Self::MOD_RANGED_ATTACK_POWER_VERSUS => 131,
            Self::MOD_INCREASE_ENERGY_PERCENT => 132,
            Self::MOD_INCREASE_HEALTH_PERCENT => 133,
            Self::MOD_MANA_REGEN_INTERRUPT => 134,
            Self::MOD_HEALING_DONE => 135,
            Self::MOD_HEALING_DONE_PERCENT => 136,
            Self::MOD_TOTAL_STAT_PERCENTAGE => 137,
            Self::MOD_MELEE_HASTE => 138,
            Self::FORCE_REACTION => 139,
            Self::MOD_RANGED_HASTE => 140,
            Self::MOD_RANGED_AMMO_HASTE => 141,
            Self::MOD_BASE_RESISTANCE_PCT => 142,
            Self::MOD_RESISTANCE_EXCLUSIVE => 143,
            Self::SAFE_FALL => 144,
            Self::CHARISMA => 145,
            Self::PERSUADED => 146,
            Self::MECHANIC_IMMUNITY_MASK => 147,
            Self::RETAIN_COMBO_POINTS => 148,
            Self::RESIST_PUSHBACK => 149,
            Self::MOD_SHIELD_BLOCKVALUE_PCT => 150,
            Self::TRACK_STEALTHED => 151,
            Self::MOD_DETECTED_RANGE => 152,
            Self::SPLIT_DAMAGE_FLAT => 153,
            Self::MOD_STEALTH_LEVEL => 154,
            Self::MOD_WATER_BREATHING => 155,
            Self::MOD_REPUTATION_GAIN => 156,
            Self::PET_DAMAGE_MULTI => 157,
            Self::MOD_SHIELD_BLOCKVALUE => 158,
            Self::NO_PVP_CREDIT => 159,
            Self::MOD_AOE_AVOIDANCE => 160,
            Self::MOD_HEALTH_REGEN_IN_COMBAT => 161,
            Self::POWER_BURN_MANA => 162,
            Self::MOD_CRIT_DAMAGE_BONUS => 163,
            Self::UNKNOWN164 => 164,
            Self::MELEE_ATTACK_POWER_ATTACKER_BONUS => 165,
            Self::MOD_ATTACK_POWER_PCT => 166,
            Self::MOD_RANGED_ATTACK_POWER_PCT => 167,
            Self::MOD_DAMAGE_DONE_VERSUS => 168,
            Self::MOD_CRIT_PERCENT_VERSUS => 169,
            Self::DETECT_AMORE => 170,
            Self::MOD_SPEED_NOT_STACK => 171,
            Self::MOD_MOUNTED_SPEED_NOT_STACK => 172,
            Self::ALLOW_CHAMPION_SPELLS => 173,
            Self::MOD_SPELL_DAMAGE_OF_STAT_PERCENT => 174,
            Self::MOD_SPELL_HEALING_OF_STAT_PERCENT => 175,
            Self::SPIRIT_OF_REDEMPTION => 176,
            Self::AOE_CHARM => 177,
            Self::MOD_DEBUFF_RESISTANCE => 178,
            Self::MOD_ATTACKER_SPELL_CRIT_CHANCE => 179,
            Self::MOD_FLAT_SPELL_DAMAGE_VERSUS => 180,
            Self::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS => 181,
            Self::MOD_RESISTANCE_OF_STAT_PERCENT => 182,
            Self::MOD_CRITICAL_THREAT => 183,
            Self::MOD_ATTACKER_MELEE_HIT_CHANCE => 184,
            Self::MOD_ATTACKER_RANGED_HIT_CHANCE => 185,
            Self::MOD_ATTACKER_SPELL_HIT_CHANCE => 186,
            Self::MOD_ATTACKER_MELEE_CRIT_CHANCE => 187,
            Self::MOD_ATTACKER_RANGED_CRIT_CHANCE => 188,
            Self::MOD_RATING => 189,
            Self::MOD_FACTION_REPUTATION_GAIN => 190,
            Self::USE_NORMAL_MOVEMENT_SPEED => 191,
        }
    }

}

impl AuraLog_AuraType {
    pub(crate) fn size(&self) -> usize {
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

