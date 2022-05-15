use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{SpellEffect, SpellEffectError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SpellLog {
    pub effect: SpellLogSpellEffect,
}

impl SpellLog {
    pub const AMOUNT_OF_LOGS_VALUE: u32 = 0x01;

}

impl ReadableAndWritable for SpellLog {
    type Error = SpellLogError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // effect: SpellEffect
        let effect = SpellEffect::read(r)?;

        // amount_of_logs: u32
        let _amount_of_logs = crate::util::read_u32_le(r)?;
        // amount_of_logs is expected to always be 1 (1)

        let effect_if = match effect {
            SpellEffect::NONE => SpellLogSpellEffect::NONE,
            SpellEffect::INSTAKILL => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLogSpellEffect::INSTAKILL {
                    target5,
                }
            }
            SpellEffect::SCHOOL_DAMAGE => SpellLogSpellEffect::SCHOOL_DAMAGE,
            SpellEffect::DUMMY => SpellLogSpellEffect::DUMMY,
            SpellEffect::PORTAL_TELEPORT => SpellLogSpellEffect::PORTAL_TELEPORT,
            SpellEffect::TELEPORT_UNITS => SpellLogSpellEffect::TELEPORT_UNITS,
            SpellEffect::APPLY_AURA => SpellLogSpellEffect::APPLY_AURA,
            SpellEffect::ENVIRONMENTAL_DAMAGE => SpellLogSpellEffect::ENVIRONMENTAL_DAMAGE,
            SpellEffect::POWER_DRAIN => {
                // target1: Guid
                let target1 = Guid::read(r)?;

                // unknown1: u32
                let unknown1 = crate::util::read_u32_le(r)?;

                // unknown2: u32
                let unknown2 = crate::util::read_u32_le(r)?;

                // unknown3: f32
                let unknown3 = crate::util::read_f32_le(r)?;
                SpellLogSpellEffect::POWER_DRAIN {
                    target1,
                    unknown1,
                    unknown2,
                    unknown3,
                }
            }
            SpellEffect::HEALTH_LEECH => SpellLogSpellEffect::HEALTH_LEECH,
            SpellEffect::HEAL => SpellLogSpellEffect::HEAL,
            SpellEffect::BIND => SpellLogSpellEffect::BIND,
            SpellEffect::PORTAL => SpellLogSpellEffect::PORTAL,
            SpellEffect::RITUAL_BASE => SpellLogSpellEffect::RITUAL_BASE,
            SpellEffect::RITUAL_SPECIALIZE => SpellLogSpellEffect::RITUAL_SPECIALIZE,
            SpellEffect::RITUAL_ACTIVATE_PORTAL => SpellLogSpellEffect::RITUAL_ACTIVATE_PORTAL,
            SpellEffect::QUEST_COMPLETE => SpellLogSpellEffect::QUEST_COMPLETE,
            SpellEffect::WEAPON_DAMAGE_NOSCHOOL => SpellLogSpellEffect::WEAPON_DAMAGE_NOSCHOOL,
            SpellEffect::RESURRECT => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLogSpellEffect::RESURRECT {
                    target5,
                }
            }
            SpellEffect::ADD_EXTRA_ATTACKS => {
                // target2: Guid
                let target2 = Guid::read(r)?;

                // unknown4: u32
                let unknown4 = crate::util::read_u32_le(r)?;

                SpellLogSpellEffect::ADD_EXTRA_ATTACKS {
                    target2,
                    unknown4,
                }
            }
            SpellEffect::DODGE => SpellLogSpellEffect::DODGE,
            SpellEffect::EVADE => SpellLogSpellEffect::EVADE,
            SpellEffect::PARRY => SpellLogSpellEffect::PARRY,
            SpellEffect::BLOCK => SpellLogSpellEffect::BLOCK,
            SpellEffect::CREATE_ITEM => {
                // spell_effect_item_type: u32
                let spell_effect_item_type = crate::util::read_u32_le(r)?;

                SpellLogSpellEffect::CREATE_ITEM {
                    spell_effect_item_type,
                }
            }
            SpellEffect::WEAPON => SpellLogSpellEffect::WEAPON,
            SpellEffect::DEFENSE => SpellLogSpellEffect::DEFENSE,
            SpellEffect::PERSISTENT_AREA_AURA => SpellLogSpellEffect::PERSISTENT_AREA_AURA,
            SpellEffect::SUMMON => SpellLogSpellEffect::SUMMON,
            SpellEffect::LEAP => SpellLogSpellEffect::LEAP,
            SpellEffect::ENERGIZE => SpellLogSpellEffect::ENERGIZE,
            SpellEffect::WEAPON_PERCENT_DAMAGE => SpellLogSpellEffect::WEAPON_PERCENT_DAMAGE,
            SpellEffect::TRIGGER_MISSILE => SpellLogSpellEffect::TRIGGER_MISSILE,
            SpellEffect::OPEN_LOCK => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLogSpellEffect::OPEN_LOCK {
                    target5,
                }
            }
            SpellEffect::SUMMON_CHANGE_ITEM => SpellLogSpellEffect::SUMMON_CHANGE_ITEM,
            SpellEffect::APPLY_AREA_AURA_PARTY => SpellLogSpellEffect::APPLY_AREA_AURA_PARTY,
            SpellEffect::LEARN_SPELL => SpellLogSpellEffect::LEARN_SPELL,
            SpellEffect::SPELL_DEFENSE => SpellLogSpellEffect::SPELL_DEFENSE,
            SpellEffect::DISPEL => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLogSpellEffect::DISPEL {
                    target5,
                }
            }
            SpellEffect::LANGUAGE => SpellLogSpellEffect::LANGUAGE,
            SpellEffect::DUAL_WIELD => SpellLogSpellEffect::DUAL_WIELD,
            SpellEffect::SUMMON_WILD => SpellLogSpellEffect::SUMMON_WILD,
            SpellEffect::SUMMON_GUARDIAN => SpellLogSpellEffect::SUMMON_GUARDIAN,
            SpellEffect::TELEPORT_UNITS_FACE_CASTER => SpellLogSpellEffect::TELEPORT_UNITS_FACE_CASTER,
            SpellEffect::SKILL_STEP => SpellLogSpellEffect::SKILL_STEP,
            SpellEffect::ADD_HONOR => SpellLogSpellEffect::ADD_HONOR,
            SpellEffect::SPAWN => SpellLogSpellEffect::SPAWN,
            SpellEffect::TRADE_SKILL => SpellLogSpellEffect::TRADE_SKILL,
            SpellEffect::STEALTH => SpellLogSpellEffect::STEALTH,
            SpellEffect::DETECT => SpellLogSpellEffect::DETECT,
            SpellEffect::TRANS_DOOR => SpellLogSpellEffect::TRANS_DOOR,
            SpellEffect::FORCE_CRITICAL_HIT => SpellLogSpellEffect::FORCE_CRITICAL_HIT,
            SpellEffect::GUARANTEE_HIT => SpellLogSpellEffect::GUARANTEE_HIT,
            SpellEffect::ENCHANT_ITEM => SpellLogSpellEffect::ENCHANT_ITEM,
            SpellEffect::ENCHANT_ITEM_TEMPORARY => SpellLogSpellEffect::ENCHANT_ITEM_TEMPORARY,
            SpellEffect::TAMECREATURE => SpellLogSpellEffect::TAMECREATURE,
            SpellEffect::SUMMON_PET => SpellLogSpellEffect::SUMMON_PET,
            SpellEffect::LEARN_PET_SPELL => SpellLogSpellEffect::LEARN_PET_SPELL,
            SpellEffect::WEAPON_DAMAGE => SpellLogSpellEffect::WEAPON_DAMAGE,
            SpellEffect::OPEN_LOCK_ITEM => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLogSpellEffect::OPEN_LOCK_ITEM {
                    target5,
                }
            }
            SpellEffect::PROFICIENCY => SpellLogSpellEffect::PROFICIENCY,
            SpellEffect::SEND_EVENT => SpellLogSpellEffect::SEND_EVENT,
            SpellEffect::POWER_BURN => SpellLogSpellEffect::POWER_BURN,
            SpellEffect::THREAT => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLogSpellEffect::THREAT {
                    target5,
                }
            }
            SpellEffect::TRIGGER_SPELL => SpellLogSpellEffect::TRIGGER_SPELL,
            SpellEffect::HEALTH_FUNNEL => SpellLogSpellEffect::HEALTH_FUNNEL,
            SpellEffect::POWER_FUNNEL => SpellLogSpellEffect::POWER_FUNNEL,
            SpellEffect::HEAL_MAX_HEALTH => SpellLogSpellEffect::HEAL_MAX_HEALTH,
            SpellEffect::INTERRUPT_CAST => {
                // target3: Guid
                let target3 = Guid::read(r)?;

                // interrupted_spell: u32
                let interrupted_spell = crate::util::read_u32_le(r)?;

                SpellLogSpellEffect::INTERRUPT_CAST {
                    interrupted_spell,
                    target3,
                }
            }
            SpellEffect::DISTRACT => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLogSpellEffect::DISTRACT {
                    target5,
                }
            }
            SpellEffect::PULL => SpellLogSpellEffect::PULL,
            SpellEffect::PICKPOCKET => SpellLogSpellEffect::PICKPOCKET,
            SpellEffect::ADD_FARSIGHT => SpellLogSpellEffect::ADD_FARSIGHT,
            SpellEffect::SUMMON_POSSESSED => SpellLogSpellEffect::SUMMON_POSSESSED,
            SpellEffect::SUMMON_TOTEM => SpellLogSpellEffect::SUMMON_TOTEM,
            SpellEffect::HEAL_MECHANICAL => SpellLogSpellEffect::HEAL_MECHANICAL,
            SpellEffect::SUMMON_OBJECT_WILD => SpellLogSpellEffect::SUMMON_OBJECT_WILD,
            SpellEffect::SCRIPT_EFFECT => SpellLogSpellEffect::SCRIPT_EFFECT,
            SpellEffect::ATTACK => SpellLogSpellEffect::ATTACK,
            SpellEffect::SANCTUARY => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLogSpellEffect::SANCTUARY {
                    target5,
                }
            }
            SpellEffect::ADD_COMBO_POINTS => SpellLogSpellEffect::ADD_COMBO_POINTS,
            SpellEffect::CREATE_HOUSE => SpellLogSpellEffect::CREATE_HOUSE,
            SpellEffect::BIND_SIGHT => SpellLogSpellEffect::BIND_SIGHT,
            SpellEffect::DUEL => SpellLogSpellEffect::DUEL,
            SpellEffect::STUCK => SpellLogSpellEffect::STUCK,
            SpellEffect::SUMMON_PLAYER => SpellLogSpellEffect::SUMMON_PLAYER,
            SpellEffect::ACTIVATE_OBJECT => SpellLogSpellEffect::ACTIVATE_OBJECT,
            SpellEffect::SUMMON_TOTEM_SLOT1 => SpellLogSpellEffect::SUMMON_TOTEM_SLOT1,
            SpellEffect::SUMMON_TOTEM_SLOT2 => SpellLogSpellEffect::SUMMON_TOTEM_SLOT2,
            SpellEffect::SUMMON_TOTEM_SLOT3 => SpellLogSpellEffect::SUMMON_TOTEM_SLOT3,
            SpellEffect::SUMMON_TOTEM_SLOT4 => SpellLogSpellEffect::SUMMON_TOTEM_SLOT4,
            SpellEffect::THREAT_ALL => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLogSpellEffect::THREAT_ALL {
                    target5,
                }
            }
            SpellEffect::ENCHANT_HELD_ITEM => SpellLogSpellEffect::ENCHANT_HELD_ITEM,
            SpellEffect::SUMMON_PHANTASM => SpellLogSpellEffect::SUMMON_PHANTASM,
            SpellEffect::SELF_RESURRECT => SpellLogSpellEffect::SELF_RESURRECT,
            SpellEffect::SKINNING => SpellLogSpellEffect::SKINNING,
            SpellEffect::CHARGE => SpellLogSpellEffect::CHARGE,
            SpellEffect::SUMMON_CRITTER => SpellLogSpellEffect::SUMMON_CRITTER,
            SpellEffect::KNOCK_BACK => SpellLogSpellEffect::KNOCK_BACK,
            SpellEffect::DISENCHANT => SpellLogSpellEffect::DISENCHANT,
            SpellEffect::INEBRIATE => SpellLogSpellEffect::INEBRIATE,
            SpellEffect::FEED_PET => {
                // item_target_entry: u32
                let item_target_entry = crate::util::read_u32_le(r)?;

                SpellLogSpellEffect::FEED_PET {
                    item_target_entry,
                }
            }
            SpellEffect::DISMISS_PET => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLogSpellEffect::DISMISS_PET {
                    target5,
                }
            }
            SpellEffect::REPUTATION => SpellLogSpellEffect::REPUTATION,
            SpellEffect::SUMMON_OBJECT_SLOT1 => SpellLogSpellEffect::SUMMON_OBJECT_SLOT1,
            SpellEffect::SUMMON_OBJECT_SLOT2 => SpellLogSpellEffect::SUMMON_OBJECT_SLOT2,
            SpellEffect::SUMMON_OBJECT_SLOT3 => SpellLogSpellEffect::SUMMON_OBJECT_SLOT3,
            SpellEffect::SUMMON_OBJECT_SLOT4 => SpellLogSpellEffect::SUMMON_OBJECT_SLOT4,
            SpellEffect::DISPEL_MECHANIC => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLogSpellEffect::DISPEL_MECHANIC {
                    target5,
                }
            }
            SpellEffect::SUMMON_DEAD_PET => SpellLogSpellEffect::SUMMON_DEAD_PET,
            SpellEffect::DESTROY_ALL_TOTEMS => SpellLogSpellEffect::DESTROY_ALL_TOTEMS,
            SpellEffect::DURABILITY_DAMAGE => {
                // target4: Guid
                let target4 = Guid::read(r)?;

                // unknown5: u32
                let unknown5 = crate::util::read_u32_le(r)?;

                // unknown6: u32
                let unknown6 = crate::util::read_u32_le(r)?;

                SpellLogSpellEffect::DURABILITY_DAMAGE {
                    target4,
                    unknown5,
                    unknown6,
                }
            }
            SpellEffect::SUMMON_DEMON => SpellLogSpellEffect::SUMMON_DEMON,
            SpellEffect::RESURRECT_NEW => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLogSpellEffect::RESURRECT_NEW {
                    target5,
                }
            }
            SpellEffect::ATTACK_ME => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLogSpellEffect::ATTACK_ME {
                    target5,
                }
            }
            SpellEffect::DURABILITY_DAMAGE_PCT => SpellLogSpellEffect::DURABILITY_DAMAGE_PCT,
            SpellEffect::SKIN_PLAYER_CORPSE => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLogSpellEffect::SKIN_PLAYER_CORPSE {
                    target5,
                }
            }
            SpellEffect::SPIRIT_HEAL => SpellLogSpellEffect::SPIRIT_HEAL,
            SpellEffect::SKILL => SpellLogSpellEffect::SKILL,
            SpellEffect::APPLY_AREA_AURA_PET => SpellLogSpellEffect::APPLY_AREA_AURA_PET,
            SpellEffect::TELEPORT_GRAVEYARD => SpellLogSpellEffect::TELEPORT_GRAVEYARD,
            SpellEffect::NORMALIZED_WEAPON_DMG => SpellLogSpellEffect::NORMALIZED_WEAPON_DMG,
            SpellEffect::UNKNOWN122 => SpellLogSpellEffect::UNKNOWN122,
            SpellEffect::SEND_TAXI => SpellLogSpellEffect::SEND_TAXI,
            SpellEffect::PLAYER_PULL => SpellLogSpellEffect::PLAYER_PULL,
            SpellEffect::MODIFY_THREAT_PERCENT => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLogSpellEffect::MODIFY_THREAT_PERCENT {
                    target5,
                }
            }
            SpellEffect::UNKNOWN126 => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLogSpellEffect::UNKNOWN126 {
                    target5,
                }
            }
            SpellEffect::UNKNOWN127 => SpellLogSpellEffect::UNKNOWN127,
        };

        Ok(Self {
            effect: effect_if,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // effect: SpellEffect
        self.effect.write(w)?;

        // amount_of_logs: u32
        w.write_all(&Self::AMOUNT_OF_LOGS_VALUE.to_le_bytes())?;

        match &self.effect {
            SpellLogSpellEffect::NONE => {}
            SpellLogSpellEffect::INSTAKILL {
                target5,
            } => {
                // target5: Guid
                target5.write(w)?;

            }
            SpellLogSpellEffect::SCHOOL_DAMAGE => {}
            SpellLogSpellEffect::DUMMY => {}
            SpellLogSpellEffect::PORTAL_TELEPORT => {}
            SpellLogSpellEffect::TELEPORT_UNITS => {}
            SpellLogSpellEffect::APPLY_AURA => {}
            SpellLogSpellEffect::ENVIRONMENTAL_DAMAGE => {}
            SpellLogSpellEffect::POWER_DRAIN {
                target1,
                unknown1,
                unknown2,
                unknown3,
            } => {
                // target1: Guid
                target1.write(w)?;

                // unknown1: u32
                w.write_all(&unknown1.to_le_bytes())?;

                // unknown2: u32
                w.write_all(&unknown2.to_le_bytes())?;

                // unknown3: f32
                w.write_all(&unknown3.to_le_bytes())?;

            }
            SpellLogSpellEffect::HEALTH_LEECH => {}
            SpellLogSpellEffect::HEAL => {}
            SpellLogSpellEffect::BIND => {}
            SpellLogSpellEffect::PORTAL => {}
            SpellLogSpellEffect::RITUAL_BASE => {}
            SpellLogSpellEffect::RITUAL_SPECIALIZE => {}
            SpellLogSpellEffect::RITUAL_ACTIVATE_PORTAL => {}
            SpellLogSpellEffect::QUEST_COMPLETE => {}
            SpellLogSpellEffect::WEAPON_DAMAGE_NOSCHOOL => {}
            SpellLogSpellEffect::RESURRECT {
                target5,
            } => {
                // target5: Guid
                target5.write(w)?;

            }
            SpellLogSpellEffect::ADD_EXTRA_ATTACKS {
                target2,
                unknown4,
            } => {
                // target2: Guid
                target2.write(w)?;

                // unknown4: u32
                w.write_all(&unknown4.to_le_bytes())?;

            }
            SpellLogSpellEffect::DODGE => {}
            SpellLogSpellEffect::EVADE => {}
            SpellLogSpellEffect::PARRY => {}
            SpellLogSpellEffect::BLOCK => {}
            SpellLogSpellEffect::CREATE_ITEM {
                spell_effect_item_type,
            } => {
                // spell_effect_item_type: u32
                w.write_all(&spell_effect_item_type.to_le_bytes())?;

            }
            SpellLogSpellEffect::WEAPON => {}
            SpellLogSpellEffect::DEFENSE => {}
            SpellLogSpellEffect::PERSISTENT_AREA_AURA => {}
            SpellLogSpellEffect::SUMMON => {}
            SpellLogSpellEffect::LEAP => {}
            SpellLogSpellEffect::ENERGIZE => {}
            SpellLogSpellEffect::WEAPON_PERCENT_DAMAGE => {}
            SpellLogSpellEffect::TRIGGER_MISSILE => {}
            SpellLogSpellEffect::OPEN_LOCK {
                target5,
            } => {
                // target5: Guid
                target5.write(w)?;

            }
            SpellLogSpellEffect::SUMMON_CHANGE_ITEM => {}
            SpellLogSpellEffect::APPLY_AREA_AURA_PARTY => {}
            SpellLogSpellEffect::LEARN_SPELL => {}
            SpellLogSpellEffect::SPELL_DEFENSE => {}
            SpellLogSpellEffect::DISPEL {
                target5,
            } => {
                // target5: Guid
                target5.write(w)?;

            }
            SpellLogSpellEffect::LANGUAGE => {}
            SpellLogSpellEffect::DUAL_WIELD => {}
            SpellLogSpellEffect::SUMMON_WILD => {}
            SpellLogSpellEffect::SUMMON_GUARDIAN => {}
            SpellLogSpellEffect::TELEPORT_UNITS_FACE_CASTER => {}
            SpellLogSpellEffect::SKILL_STEP => {}
            SpellLogSpellEffect::ADD_HONOR => {}
            SpellLogSpellEffect::SPAWN => {}
            SpellLogSpellEffect::TRADE_SKILL => {}
            SpellLogSpellEffect::STEALTH => {}
            SpellLogSpellEffect::DETECT => {}
            SpellLogSpellEffect::TRANS_DOOR => {}
            SpellLogSpellEffect::FORCE_CRITICAL_HIT => {}
            SpellLogSpellEffect::GUARANTEE_HIT => {}
            SpellLogSpellEffect::ENCHANT_ITEM => {}
            SpellLogSpellEffect::ENCHANT_ITEM_TEMPORARY => {}
            SpellLogSpellEffect::TAMECREATURE => {}
            SpellLogSpellEffect::SUMMON_PET => {}
            SpellLogSpellEffect::LEARN_PET_SPELL => {}
            SpellLogSpellEffect::WEAPON_DAMAGE => {}
            SpellLogSpellEffect::OPEN_LOCK_ITEM {
                target5,
            } => {
                // target5: Guid
                target5.write(w)?;

            }
            SpellLogSpellEffect::PROFICIENCY => {}
            SpellLogSpellEffect::SEND_EVENT => {}
            SpellLogSpellEffect::POWER_BURN => {}
            SpellLogSpellEffect::THREAT {
                target5,
            } => {
                // target5: Guid
                target5.write(w)?;

            }
            SpellLogSpellEffect::TRIGGER_SPELL => {}
            SpellLogSpellEffect::HEALTH_FUNNEL => {}
            SpellLogSpellEffect::POWER_FUNNEL => {}
            SpellLogSpellEffect::HEAL_MAX_HEALTH => {}
            SpellLogSpellEffect::INTERRUPT_CAST {
                interrupted_spell,
                target3,
            } => {
                // target3: Guid
                target3.write(w)?;

                // interrupted_spell: u32
                w.write_all(&interrupted_spell.to_le_bytes())?;

            }
            SpellLogSpellEffect::DISTRACT {
                target5,
            } => {
                // target5: Guid
                target5.write(w)?;

            }
            SpellLogSpellEffect::PULL => {}
            SpellLogSpellEffect::PICKPOCKET => {}
            SpellLogSpellEffect::ADD_FARSIGHT => {}
            SpellLogSpellEffect::SUMMON_POSSESSED => {}
            SpellLogSpellEffect::SUMMON_TOTEM => {}
            SpellLogSpellEffect::HEAL_MECHANICAL => {}
            SpellLogSpellEffect::SUMMON_OBJECT_WILD => {}
            SpellLogSpellEffect::SCRIPT_EFFECT => {}
            SpellLogSpellEffect::ATTACK => {}
            SpellLogSpellEffect::SANCTUARY {
                target5,
            } => {
                // target5: Guid
                target5.write(w)?;

            }
            SpellLogSpellEffect::ADD_COMBO_POINTS => {}
            SpellLogSpellEffect::CREATE_HOUSE => {}
            SpellLogSpellEffect::BIND_SIGHT => {}
            SpellLogSpellEffect::DUEL => {}
            SpellLogSpellEffect::STUCK => {}
            SpellLogSpellEffect::SUMMON_PLAYER => {}
            SpellLogSpellEffect::ACTIVATE_OBJECT => {}
            SpellLogSpellEffect::SUMMON_TOTEM_SLOT1 => {}
            SpellLogSpellEffect::SUMMON_TOTEM_SLOT2 => {}
            SpellLogSpellEffect::SUMMON_TOTEM_SLOT3 => {}
            SpellLogSpellEffect::SUMMON_TOTEM_SLOT4 => {}
            SpellLogSpellEffect::THREAT_ALL {
                target5,
            } => {
                // target5: Guid
                target5.write(w)?;

            }
            SpellLogSpellEffect::ENCHANT_HELD_ITEM => {}
            SpellLogSpellEffect::SUMMON_PHANTASM => {}
            SpellLogSpellEffect::SELF_RESURRECT => {}
            SpellLogSpellEffect::SKINNING => {}
            SpellLogSpellEffect::CHARGE => {}
            SpellLogSpellEffect::SUMMON_CRITTER => {}
            SpellLogSpellEffect::KNOCK_BACK => {}
            SpellLogSpellEffect::DISENCHANT => {}
            SpellLogSpellEffect::INEBRIATE => {}
            SpellLogSpellEffect::FEED_PET {
                item_target_entry,
            } => {
                // item_target_entry: u32
                w.write_all(&item_target_entry.to_le_bytes())?;

            }
            SpellLogSpellEffect::DISMISS_PET {
                target5,
            } => {
                // target5: Guid
                target5.write(w)?;

            }
            SpellLogSpellEffect::REPUTATION => {}
            SpellLogSpellEffect::SUMMON_OBJECT_SLOT1 => {}
            SpellLogSpellEffect::SUMMON_OBJECT_SLOT2 => {}
            SpellLogSpellEffect::SUMMON_OBJECT_SLOT3 => {}
            SpellLogSpellEffect::SUMMON_OBJECT_SLOT4 => {}
            SpellLogSpellEffect::DISPEL_MECHANIC {
                target5,
            } => {
                // target5: Guid
                target5.write(w)?;

            }
            SpellLogSpellEffect::SUMMON_DEAD_PET => {}
            SpellLogSpellEffect::DESTROY_ALL_TOTEMS => {}
            SpellLogSpellEffect::DURABILITY_DAMAGE {
                target4,
                unknown5,
                unknown6,
            } => {
                // target4: Guid
                target4.write(w)?;

                // unknown5: u32
                w.write_all(&unknown5.to_le_bytes())?;

                // unknown6: u32
                w.write_all(&unknown6.to_le_bytes())?;

            }
            SpellLogSpellEffect::SUMMON_DEMON => {}
            SpellLogSpellEffect::RESURRECT_NEW {
                target5,
            } => {
                // target5: Guid
                target5.write(w)?;

            }
            SpellLogSpellEffect::ATTACK_ME {
                target5,
            } => {
                // target5: Guid
                target5.write(w)?;

            }
            SpellLogSpellEffect::DURABILITY_DAMAGE_PCT => {}
            SpellLogSpellEffect::SKIN_PLAYER_CORPSE {
                target5,
            } => {
                // target5: Guid
                target5.write(w)?;

            }
            SpellLogSpellEffect::SPIRIT_HEAL => {}
            SpellLogSpellEffect::SKILL => {}
            SpellLogSpellEffect::APPLY_AREA_AURA_PET => {}
            SpellLogSpellEffect::TELEPORT_GRAVEYARD => {}
            SpellLogSpellEffect::NORMALIZED_WEAPON_DMG => {}
            SpellLogSpellEffect::UNKNOWN122 => {}
            SpellLogSpellEffect::SEND_TAXI => {}
            SpellLogSpellEffect::PLAYER_PULL => {}
            SpellLogSpellEffect::MODIFY_THREAT_PERCENT {
                target5,
            } => {
                // target5: Guid
                target5.write(w)?;

            }
            SpellLogSpellEffect::UNKNOWN126 {
                target5,
            } => {
                // target5: Guid
                target5.write(w)?;

            }
            SpellLogSpellEffect::UNKNOWN127 => {}
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // effect: SpellEffect
            let effect = SpellEffect::tokio_read(r).await?;

            // amount_of_logs: u32
            let _amount_of_logs = crate::util::tokio_read_u32_le(r).await?;
            // amount_of_logs is expected to always be 1 (1)

            let effect_if = match effect {
                SpellEffect::NONE => SpellLogSpellEffect::NONE,
                SpellEffect::INSTAKILL => {
                    // target5: Guid
                    let target5 = Guid::tokio_read(r).await?;

                    SpellLogSpellEffect::INSTAKILL {
                        target5,
                    }
                }
                SpellEffect::SCHOOL_DAMAGE => SpellLogSpellEffect::SCHOOL_DAMAGE,
                SpellEffect::DUMMY => SpellLogSpellEffect::DUMMY,
                SpellEffect::PORTAL_TELEPORT => SpellLogSpellEffect::PORTAL_TELEPORT,
                SpellEffect::TELEPORT_UNITS => SpellLogSpellEffect::TELEPORT_UNITS,
                SpellEffect::APPLY_AURA => SpellLogSpellEffect::APPLY_AURA,
                SpellEffect::ENVIRONMENTAL_DAMAGE => SpellLogSpellEffect::ENVIRONMENTAL_DAMAGE,
                SpellEffect::POWER_DRAIN => {
                    // target1: Guid
                    let target1 = Guid::tokio_read(r).await?;

                    // unknown1: u32
                    let unknown1 = crate::util::tokio_read_u32_le(r).await?;

                    // unknown2: u32
                    let unknown2 = crate::util::tokio_read_u32_le(r).await?;

                    // unknown3: f32
                    let unknown3 = crate::util::tokio_read_f32_le(r).await?;
                    SpellLogSpellEffect::POWER_DRAIN {
                        target1,
                        unknown1,
                        unknown2,
                        unknown3,
                    }
                }
                SpellEffect::HEALTH_LEECH => SpellLogSpellEffect::HEALTH_LEECH,
                SpellEffect::HEAL => SpellLogSpellEffect::HEAL,
                SpellEffect::BIND => SpellLogSpellEffect::BIND,
                SpellEffect::PORTAL => SpellLogSpellEffect::PORTAL,
                SpellEffect::RITUAL_BASE => SpellLogSpellEffect::RITUAL_BASE,
                SpellEffect::RITUAL_SPECIALIZE => SpellLogSpellEffect::RITUAL_SPECIALIZE,
                SpellEffect::RITUAL_ACTIVATE_PORTAL => SpellLogSpellEffect::RITUAL_ACTIVATE_PORTAL,
                SpellEffect::QUEST_COMPLETE => SpellLogSpellEffect::QUEST_COMPLETE,
                SpellEffect::WEAPON_DAMAGE_NOSCHOOL => SpellLogSpellEffect::WEAPON_DAMAGE_NOSCHOOL,
                SpellEffect::RESURRECT => {
                    // target5: Guid
                    let target5 = Guid::tokio_read(r).await?;

                    SpellLogSpellEffect::RESURRECT {
                        target5,
                    }
                }
                SpellEffect::ADD_EXTRA_ATTACKS => {
                    // target2: Guid
                    let target2 = Guid::tokio_read(r).await?;

                    // unknown4: u32
                    let unknown4 = crate::util::tokio_read_u32_le(r).await?;

                    SpellLogSpellEffect::ADD_EXTRA_ATTACKS {
                        target2,
                        unknown4,
                    }
                }
                SpellEffect::DODGE => SpellLogSpellEffect::DODGE,
                SpellEffect::EVADE => SpellLogSpellEffect::EVADE,
                SpellEffect::PARRY => SpellLogSpellEffect::PARRY,
                SpellEffect::BLOCK => SpellLogSpellEffect::BLOCK,
                SpellEffect::CREATE_ITEM => {
                    // spell_effect_item_type: u32
                    let spell_effect_item_type = crate::util::tokio_read_u32_le(r).await?;

                    SpellLogSpellEffect::CREATE_ITEM {
                        spell_effect_item_type,
                    }
                }
                SpellEffect::WEAPON => SpellLogSpellEffect::WEAPON,
                SpellEffect::DEFENSE => SpellLogSpellEffect::DEFENSE,
                SpellEffect::PERSISTENT_AREA_AURA => SpellLogSpellEffect::PERSISTENT_AREA_AURA,
                SpellEffect::SUMMON => SpellLogSpellEffect::SUMMON,
                SpellEffect::LEAP => SpellLogSpellEffect::LEAP,
                SpellEffect::ENERGIZE => SpellLogSpellEffect::ENERGIZE,
                SpellEffect::WEAPON_PERCENT_DAMAGE => SpellLogSpellEffect::WEAPON_PERCENT_DAMAGE,
                SpellEffect::TRIGGER_MISSILE => SpellLogSpellEffect::TRIGGER_MISSILE,
                SpellEffect::OPEN_LOCK => {
                    // target5: Guid
                    let target5 = Guid::tokio_read(r).await?;

                    SpellLogSpellEffect::OPEN_LOCK {
                        target5,
                    }
                }
                SpellEffect::SUMMON_CHANGE_ITEM => SpellLogSpellEffect::SUMMON_CHANGE_ITEM,
                SpellEffect::APPLY_AREA_AURA_PARTY => SpellLogSpellEffect::APPLY_AREA_AURA_PARTY,
                SpellEffect::LEARN_SPELL => SpellLogSpellEffect::LEARN_SPELL,
                SpellEffect::SPELL_DEFENSE => SpellLogSpellEffect::SPELL_DEFENSE,
                SpellEffect::DISPEL => {
                    // target5: Guid
                    let target5 = Guid::tokio_read(r).await?;

                    SpellLogSpellEffect::DISPEL {
                        target5,
                    }
                }
                SpellEffect::LANGUAGE => SpellLogSpellEffect::LANGUAGE,
                SpellEffect::DUAL_WIELD => SpellLogSpellEffect::DUAL_WIELD,
                SpellEffect::SUMMON_WILD => SpellLogSpellEffect::SUMMON_WILD,
                SpellEffect::SUMMON_GUARDIAN => SpellLogSpellEffect::SUMMON_GUARDIAN,
                SpellEffect::TELEPORT_UNITS_FACE_CASTER => SpellLogSpellEffect::TELEPORT_UNITS_FACE_CASTER,
                SpellEffect::SKILL_STEP => SpellLogSpellEffect::SKILL_STEP,
                SpellEffect::ADD_HONOR => SpellLogSpellEffect::ADD_HONOR,
                SpellEffect::SPAWN => SpellLogSpellEffect::SPAWN,
                SpellEffect::TRADE_SKILL => SpellLogSpellEffect::TRADE_SKILL,
                SpellEffect::STEALTH => SpellLogSpellEffect::STEALTH,
                SpellEffect::DETECT => SpellLogSpellEffect::DETECT,
                SpellEffect::TRANS_DOOR => SpellLogSpellEffect::TRANS_DOOR,
                SpellEffect::FORCE_CRITICAL_HIT => SpellLogSpellEffect::FORCE_CRITICAL_HIT,
                SpellEffect::GUARANTEE_HIT => SpellLogSpellEffect::GUARANTEE_HIT,
                SpellEffect::ENCHANT_ITEM => SpellLogSpellEffect::ENCHANT_ITEM,
                SpellEffect::ENCHANT_ITEM_TEMPORARY => SpellLogSpellEffect::ENCHANT_ITEM_TEMPORARY,
                SpellEffect::TAMECREATURE => SpellLogSpellEffect::TAMECREATURE,
                SpellEffect::SUMMON_PET => SpellLogSpellEffect::SUMMON_PET,
                SpellEffect::LEARN_PET_SPELL => SpellLogSpellEffect::LEARN_PET_SPELL,
                SpellEffect::WEAPON_DAMAGE => SpellLogSpellEffect::WEAPON_DAMAGE,
                SpellEffect::OPEN_LOCK_ITEM => {
                    // target5: Guid
                    let target5 = Guid::tokio_read(r).await?;

                    SpellLogSpellEffect::OPEN_LOCK_ITEM {
                        target5,
                    }
                }
                SpellEffect::PROFICIENCY => SpellLogSpellEffect::PROFICIENCY,
                SpellEffect::SEND_EVENT => SpellLogSpellEffect::SEND_EVENT,
                SpellEffect::POWER_BURN => SpellLogSpellEffect::POWER_BURN,
                SpellEffect::THREAT => {
                    // target5: Guid
                    let target5 = Guid::tokio_read(r).await?;

                    SpellLogSpellEffect::THREAT {
                        target5,
                    }
                }
                SpellEffect::TRIGGER_SPELL => SpellLogSpellEffect::TRIGGER_SPELL,
                SpellEffect::HEALTH_FUNNEL => SpellLogSpellEffect::HEALTH_FUNNEL,
                SpellEffect::POWER_FUNNEL => SpellLogSpellEffect::POWER_FUNNEL,
                SpellEffect::HEAL_MAX_HEALTH => SpellLogSpellEffect::HEAL_MAX_HEALTH,
                SpellEffect::INTERRUPT_CAST => {
                    // target3: Guid
                    let target3 = Guid::tokio_read(r).await?;

                    // interrupted_spell: u32
                    let interrupted_spell = crate::util::tokio_read_u32_le(r).await?;

                    SpellLogSpellEffect::INTERRUPT_CAST {
                        interrupted_spell,
                        target3,
                    }
                }
                SpellEffect::DISTRACT => {
                    // target5: Guid
                    let target5 = Guid::tokio_read(r).await?;

                    SpellLogSpellEffect::DISTRACT {
                        target5,
                    }
                }
                SpellEffect::PULL => SpellLogSpellEffect::PULL,
                SpellEffect::PICKPOCKET => SpellLogSpellEffect::PICKPOCKET,
                SpellEffect::ADD_FARSIGHT => SpellLogSpellEffect::ADD_FARSIGHT,
                SpellEffect::SUMMON_POSSESSED => SpellLogSpellEffect::SUMMON_POSSESSED,
                SpellEffect::SUMMON_TOTEM => SpellLogSpellEffect::SUMMON_TOTEM,
                SpellEffect::HEAL_MECHANICAL => SpellLogSpellEffect::HEAL_MECHANICAL,
                SpellEffect::SUMMON_OBJECT_WILD => SpellLogSpellEffect::SUMMON_OBJECT_WILD,
                SpellEffect::SCRIPT_EFFECT => SpellLogSpellEffect::SCRIPT_EFFECT,
                SpellEffect::ATTACK => SpellLogSpellEffect::ATTACK,
                SpellEffect::SANCTUARY => {
                    // target5: Guid
                    let target5 = Guid::tokio_read(r).await?;

                    SpellLogSpellEffect::SANCTUARY {
                        target5,
                    }
                }
                SpellEffect::ADD_COMBO_POINTS => SpellLogSpellEffect::ADD_COMBO_POINTS,
                SpellEffect::CREATE_HOUSE => SpellLogSpellEffect::CREATE_HOUSE,
                SpellEffect::BIND_SIGHT => SpellLogSpellEffect::BIND_SIGHT,
                SpellEffect::DUEL => SpellLogSpellEffect::DUEL,
                SpellEffect::STUCK => SpellLogSpellEffect::STUCK,
                SpellEffect::SUMMON_PLAYER => SpellLogSpellEffect::SUMMON_PLAYER,
                SpellEffect::ACTIVATE_OBJECT => SpellLogSpellEffect::ACTIVATE_OBJECT,
                SpellEffect::SUMMON_TOTEM_SLOT1 => SpellLogSpellEffect::SUMMON_TOTEM_SLOT1,
                SpellEffect::SUMMON_TOTEM_SLOT2 => SpellLogSpellEffect::SUMMON_TOTEM_SLOT2,
                SpellEffect::SUMMON_TOTEM_SLOT3 => SpellLogSpellEffect::SUMMON_TOTEM_SLOT3,
                SpellEffect::SUMMON_TOTEM_SLOT4 => SpellLogSpellEffect::SUMMON_TOTEM_SLOT4,
                SpellEffect::THREAT_ALL => {
                    // target5: Guid
                    let target5 = Guid::tokio_read(r).await?;

                    SpellLogSpellEffect::THREAT_ALL {
                        target5,
                    }
                }
                SpellEffect::ENCHANT_HELD_ITEM => SpellLogSpellEffect::ENCHANT_HELD_ITEM,
                SpellEffect::SUMMON_PHANTASM => SpellLogSpellEffect::SUMMON_PHANTASM,
                SpellEffect::SELF_RESURRECT => SpellLogSpellEffect::SELF_RESURRECT,
                SpellEffect::SKINNING => SpellLogSpellEffect::SKINNING,
                SpellEffect::CHARGE => SpellLogSpellEffect::CHARGE,
                SpellEffect::SUMMON_CRITTER => SpellLogSpellEffect::SUMMON_CRITTER,
                SpellEffect::KNOCK_BACK => SpellLogSpellEffect::KNOCK_BACK,
                SpellEffect::DISENCHANT => SpellLogSpellEffect::DISENCHANT,
                SpellEffect::INEBRIATE => SpellLogSpellEffect::INEBRIATE,
                SpellEffect::FEED_PET => {
                    // item_target_entry: u32
                    let item_target_entry = crate::util::tokio_read_u32_le(r).await?;

                    SpellLogSpellEffect::FEED_PET {
                        item_target_entry,
                    }
                }
                SpellEffect::DISMISS_PET => {
                    // target5: Guid
                    let target5 = Guid::tokio_read(r).await?;

                    SpellLogSpellEffect::DISMISS_PET {
                        target5,
                    }
                }
                SpellEffect::REPUTATION => SpellLogSpellEffect::REPUTATION,
                SpellEffect::SUMMON_OBJECT_SLOT1 => SpellLogSpellEffect::SUMMON_OBJECT_SLOT1,
                SpellEffect::SUMMON_OBJECT_SLOT2 => SpellLogSpellEffect::SUMMON_OBJECT_SLOT2,
                SpellEffect::SUMMON_OBJECT_SLOT3 => SpellLogSpellEffect::SUMMON_OBJECT_SLOT3,
                SpellEffect::SUMMON_OBJECT_SLOT4 => SpellLogSpellEffect::SUMMON_OBJECT_SLOT4,
                SpellEffect::DISPEL_MECHANIC => {
                    // target5: Guid
                    let target5 = Guid::tokio_read(r).await?;

                    SpellLogSpellEffect::DISPEL_MECHANIC {
                        target5,
                    }
                }
                SpellEffect::SUMMON_DEAD_PET => SpellLogSpellEffect::SUMMON_DEAD_PET,
                SpellEffect::DESTROY_ALL_TOTEMS => SpellLogSpellEffect::DESTROY_ALL_TOTEMS,
                SpellEffect::DURABILITY_DAMAGE => {
                    // target4: Guid
                    let target4 = Guid::tokio_read(r).await?;

                    // unknown5: u32
                    let unknown5 = crate::util::tokio_read_u32_le(r).await?;

                    // unknown6: u32
                    let unknown6 = crate::util::tokio_read_u32_le(r).await?;

                    SpellLogSpellEffect::DURABILITY_DAMAGE {
                        target4,
                        unknown5,
                        unknown6,
                    }
                }
                SpellEffect::SUMMON_DEMON => SpellLogSpellEffect::SUMMON_DEMON,
                SpellEffect::RESURRECT_NEW => {
                    // target5: Guid
                    let target5 = Guid::tokio_read(r).await?;

                    SpellLogSpellEffect::RESURRECT_NEW {
                        target5,
                    }
                }
                SpellEffect::ATTACK_ME => {
                    // target5: Guid
                    let target5 = Guid::tokio_read(r).await?;

                    SpellLogSpellEffect::ATTACK_ME {
                        target5,
                    }
                }
                SpellEffect::DURABILITY_DAMAGE_PCT => SpellLogSpellEffect::DURABILITY_DAMAGE_PCT,
                SpellEffect::SKIN_PLAYER_CORPSE => {
                    // target5: Guid
                    let target5 = Guid::tokio_read(r).await?;

                    SpellLogSpellEffect::SKIN_PLAYER_CORPSE {
                        target5,
                    }
                }
                SpellEffect::SPIRIT_HEAL => SpellLogSpellEffect::SPIRIT_HEAL,
                SpellEffect::SKILL => SpellLogSpellEffect::SKILL,
                SpellEffect::APPLY_AREA_AURA_PET => SpellLogSpellEffect::APPLY_AREA_AURA_PET,
                SpellEffect::TELEPORT_GRAVEYARD => SpellLogSpellEffect::TELEPORT_GRAVEYARD,
                SpellEffect::NORMALIZED_WEAPON_DMG => SpellLogSpellEffect::NORMALIZED_WEAPON_DMG,
                SpellEffect::UNKNOWN122 => SpellLogSpellEffect::UNKNOWN122,
                SpellEffect::SEND_TAXI => SpellLogSpellEffect::SEND_TAXI,
                SpellEffect::PLAYER_PULL => SpellLogSpellEffect::PLAYER_PULL,
                SpellEffect::MODIFY_THREAT_PERCENT => {
                    // target5: Guid
                    let target5 = Guid::tokio_read(r).await?;

                    SpellLogSpellEffect::MODIFY_THREAT_PERCENT {
                        target5,
                    }
                }
                SpellEffect::UNKNOWN126 => {
                    // target5: Guid
                    let target5 = Guid::tokio_read(r).await?;

                    SpellLogSpellEffect::UNKNOWN126 {
                        target5,
                    }
                }
                SpellEffect::UNKNOWN127 => SpellLogSpellEffect::UNKNOWN127,
            };

            Ok(Self {
                effect: effect_if,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // effect: SpellEffect
            self.effect.tokio_write(w).await?;

            // amount_of_logs: u32
            w.write_all(&Self::AMOUNT_OF_LOGS_VALUE.to_le_bytes()).await?;

            match &self.effect {
                SpellLogSpellEffect::NONE => {}
                SpellLogSpellEffect::INSTAKILL {
                    target5,
                } => {
                    // target5: Guid
                    target5.tokio_write(w).await?;

                }
                SpellLogSpellEffect::SCHOOL_DAMAGE => {}
                SpellLogSpellEffect::DUMMY => {}
                SpellLogSpellEffect::PORTAL_TELEPORT => {}
                SpellLogSpellEffect::TELEPORT_UNITS => {}
                SpellLogSpellEffect::APPLY_AURA => {}
                SpellLogSpellEffect::ENVIRONMENTAL_DAMAGE => {}
                SpellLogSpellEffect::POWER_DRAIN {
                    target1,
                    unknown1,
                    unknown2,
                    unknown3,
                } => {
                    // target1: Guid
                    target1.tokio_write(w).await?;

                    // unknown1: u32
                    w.write_all(&unknown1.to_le_bytes()).await?;

                    // unknown2: u32
                    w.write_all(&unknown2.to_le_bytes()).await?;

                    // unknown3: f32
                    w.write_all(&unknown3.to_le_bytes()).await?;

                }
                SpellLogSpellEffect::HEALTH_LEECH => {}
                SpellLogSpellEffect::HEAL => {}
                SpellLogSpellEffect::BIND => {}
                SpellLogSpellEffect::PORTAL => {}
                SpellLogSpellEffect::RITUAL_BASE => {}
                SpellLogSpellEffect::RITUAL_SPECIALIZE => {}
                SpellLogSpellEffect::RITUAL_ACTIVATE_PORTAL => {}
                SpellLogSpellEffect::QUEST_COMPLETE => {}
                SpellLogSpellEffect::WEAPON_DAMAGE_NOSCHOOL => {}
                SpellLogSpellEffect::RESURRECT {
                    target5,
                } => {
                    // target5: Guid
                    target5.tokio_write(w).await?;

                }
                SpellLogSpellEffect::ADD_EXTRA_ATTACKS {
                    target2,
                    unknown4,
                } => {
                    // target2: Guid
                    target2.tokio_write(w).await?;

                    // unknown4: u32
                    w.write_all(&unknown4.to_le_bytes()).await?;

                }
                SpellLogSpellEffect::DODGE => {}
                SpellLogSpellEffect::EVADE => {}
                SpellLogSpellEffect::PARRY => {}
                SpellLogSpellEffect::BLOCK => {}
                SpellLogSpellEffect::CREATE_ITEM {
                    spell_effect_item_type,
                } => {
                    // spell_effect_item_type: u32
                    w.write_all(&spell_effect_item_type.to_le_bytes()).await?;

                }
                SpellLogSpellEffect::WEAPON => {}
                SpellLogSpellEffect::DEFENSE => {}
                SpellLogSpellEffect::PERSISTENT_AREA_AURA => {}
                SpellLogSpellEffect::SUMMON => {}
                SpellLogSpellEffect::LEAP => {}
                SpellLogSpellEffect::ENERGIZE => {}
                SpellLogSpellEffect::WEAPON_PERCENT_DAMAGE => {}
                SpellLogSpellEffect::TRIGGER_MISSILE => {}
                SpellLogSpellEffect::OPEN_LOCK {
                    target5,
                } => {
                    // target5: Guid
                    target5.tokio_write(w).await?;

                }
                SpellLogSpellEffect::SUMMON_CHANGE_ITEM => {}
                SpellLogSpellEffect::APPLY_AREA_AURA_PARTY => {}
                SpellLogSpellEffect::LEARN_SPELL => {}
                SpellLogSpellEffect::SPELL_DEFENSE => {}
                SpellLogSpellEffect::DISPEL {
                    target5,
                } => {
                    // target5: Guid
                    target5.tokio_write(w).await?;

                }
                SpellLogSpellEffect::LANGUAGE => {}
                SpellLogSpellEffect::DUAL_WIELD => {}
                SpellLogSpellEffect::SUMMON_WILD => {}
                SpellLogSpellEffect::SUMMON_GUARDIAN => {}
                SpellLogSpellEffect::TELEPORT_UNITS_FACE_CASTER => {}
                SpellLogSpellEffect::SKILL_STEP => {}
                SpellLogSpellEffect::ADD_HONOR => {}
                SpellLogSpellEffect::SPAWN => {}
                SpellLogSpellEffect::TRADE_SKILL => {}
                SpellLogSpellEffect::STEALTH => {}
                SpellLogSpellEffect::DETECT => {}
                SpellLogSpellEffect::TRANS_DOOR => {}
                SpellLogSpellEffect::FORCE_CRITICAL_HIT => {}
                SpellLogSpellEffect::GUARANTEE_HIT => {}
                SpellLogSpellEffect::ENCHANT_ITEM => {}
                SpellLogSpellEffect::ENCHANT_ITEM_TEMPORARY => {}
                SpellLogSpellEffect::TAMECREATURE => {}
                SpellLogSpellEffect::SUMMON_PET => {}
                SpellLogSpellEffect::LEARN_PET_SPELL => {}
                SpellLogSpellEffect::WEAPON_DAMAGE => {}
                SpellLogSpellEffect::OPEN_LOCK_ITEM {
                    target5,
                } => {
                    // target5: Guid
                    target5.tokio_write(w).await?;

                }
                SpellLogSpellEffect::PROFICIENCY => {}
                SpellLogSpellEffect::SEND_EVENT => {}
                SpellLogSpellEffect::POWER_BURN => {}
                SpellLogSpellEffect::THREAT {
                    target5,
                } => {
                    // target5: Guid
                    target5.tokio_write(w).await?;

                }
                SpellLogSpellEffect::TRIGGER_SPELL => {}
                SpellLogSpellEffect::HEALTH_FUNNEL => {}
                SpellLogSpellEffect::POWER_FUNNEL => {}
                SpellLogSpellEffect::HEAL_MAX_HEALTH => {}
                SpellLogSpellEffect::INTERRUPT_CAST {
                    interrupted_spell,
                    target3,
                } => {
                    // target3: Guid
                    target3.tokio_write(w).await?;

                    // interrupted_spell: u32
                    w.write_all(&interrupted_spell.to_le_bytes()).await?;

                }
                SpellLogSpellEffect::DISTRACT {
                    target5,
                } => {
                    // target5: Guid
                    target5.tokio_write(w).await?;

                }
                SpellLogSpellEffect::PULL => {}
                SpellLogSpellEffect::PICKPOCKET => {}
                SpellLogSpellEffect::ADD_FARSIGHT => {}
                SpellLogSpellEffect::SUMMON_POSSESSED => {}
                SpellLogSpellEffect::SUMMON_TOTEM => {}
                SpellLogSpellEffect::HEAL_MECHANICAL => {}
                SpellLogSpellEffect::SUMMON_OBJECT_WILD => {}
                SpellLogSpellEffect::SCRIPT_EFFECT => {}
                SpellLogSpellEffect::ATTACK => {}
                SpellLogSpellEffect::SANCTUARY {
                    target5,
                } => {
                    // target5: Guid
                    target5.tokio_write(w).await?;

                }
                SpellLogSpellEffect::ADD_COMBO_POINTS => {}
                SpellLogSpellEffect::CREATE_HOUSE => {}
                SpellLogSpellEffect::BIND_SIGHT => {}
                SpellLogSpellEffect::DUEL => {}
                SpellLogSpellEffect::STUCK => {}
                SpellLogSpellEffect::SUMMON_PLAYER => {}
                SpellLogSpellEffect::ACTIVATE_OBJECT => {}
                SpellLogSpellEffect::SUMMON_TOTEM_SLOT1 => {}
                SpellLogSpellEffect::SUMMON_TOTEM_SLOT2 => {}
                SpellLogSpellEffect::SUMMON_TOTEM_SLOT3 => {}
                SpellLogSpellEffect::SUMMON_TOTEM_SLOT4 => {}
                SpellLogSpellEffect::THREAT_ALL {
                    target5,
                } => {
                    // target5: Guid
                    target5.tokio_write(w).await?;

                }
                SpellLogSpellEffect::ENCHANT_HELD_ITEM => {}
                SpellLogSpellEffect::SUMMON_PHANTASM => {}
                SpellLogSpellEffect::SELF_RESURRECT => {}
                SpellLogSpellEffect::SKINNING => {}
                SpellLogSpellEffect::CHARGE => {}
                SpellLogSpellEffect::SUMMON_CRITTER => {}
                SpellLogSpellEffect::KNOCK_BACK => {}
                SpellLogSpellEffect::DISENCHANT => {}
                SpellLogSpellEffect::INEBRIATE => {}
                SpellLogSpellEffect::FEED_PET {
                    item_target_entry,
                } => {
                    // item_target_entry: u32
                    w.write_all(&item_target_entry.to_le_bytes()).await?;

                }
                SpellLogSpellEffect::DISMISS_PET {
                    target5,
                } => {
                    // target5: Guid
                    target5.tokio_write(w).await?;

                }
                SpellLogSpellEffect::REPUTATION => {}
                SpellLogSpellEffect::SUMMON_OBJECT_SLOT1 => {}
                SpellLogSpellEffect::SUMMON_OBJECT_SLOT2 => {}
                SpellLogSpellEffect::SUMMON_OBJECT_SLOT3 => {}
                SpellLogSpellEffect::SUMMON_OBJECT_SLOT4 => {}
                SpellLogSpellEffect::DISPEL_MECHANIC {
                    target5,
                } => {
                    // target5: Guid
                    target5.tokio_write(w).await?;

                }
                SpellLogSpellEffect::SUMMON_DEAD_PET => {}
                SpellLogSpellEffect::DESTROY_ALL_TOTEMS => {}
                SpellLogSpellEffect::DURABILITY_DAMAGE {
                    target4,
                    unknown5,
                    unknown6,
                } => {
                    // target4: Guid
                    target4.tokio_write(w).await?;

                    // unknown5: u32
                    w.write_all(&unknown5.to_le_bytes()).await?;

                    // unknown6: u32
                    w.write_all(&unknown6.to_le_bytes()).await?;

                }
                SpellLogSpellEffect::SUMMON_DEMON => {}
                SpellLogSpellEffect::RESURRECT_NEW {
                    target5,
                } => {
                    // target5: Guid
                    target5.tokio_write(w).await?;

                }
                SpellLogSpellEffect::ATTACK_ME {
                    target5,
                } => {
                    // target5: Guid
                    target5.tokio_write(w).await?;

                }
                SpellLogSpellEffect::DURABILITY_DAMAGE_PCT => {}
                SpellLogSpellEffect::SKIN_PLAYER_CORPSE {
                    target5,
                } => {
                    // target5: Guid
                    target5.tokio_write(w).await?;

                }
                SpellLogSpellEffect::SPIRIT_HEAL => {}
                SpellLogSpellEffect::SKILL => {}
                SpellLogSpellEffect::APPLY_AREA_AURA_PET => {}
                SpellLogSpellEffect::TELEPORT_GRAVEYARD => {}
                SpellLogSpellEffect::NORMALIZED_WEAPON_DMG => {}
                SpellLogSpellEffect::UNKNOWN122 => {}
                SpellLogSpellEffect::SEND_TAXI => {}
                SpellLogSpellEffect::PLAYER_PULL => {}
                SpellLogSpellEffect::MODIFY_THREAT_PERCENT {
                    target5,
                } => {
                    // target5: Guid
                    target5.tokio_write(w).await?;

                }
                SpellLogSpellEffect::UNKNOWN126 {
                    target5,
                } => {
                    // target5: Guid
                    target5.tokio_write(w).await?;

                }
                SpellLogSpellEffect::UNKNOWN127 => {}
            }

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // effect: SpellEffect
            let effect = SpellEffect::astd_read(r).await?;

            // amount_of_logs: u32
            let _amount_of_logs = crate::util::astd_read_u32_le(r).await?;
            // amount_of_logs is expected to always be 1 (1)

            let effect_if = match effect {
                SpellEffect::NONE => SpellLogSpellEffect::NONE,
                SpellEffect::INSTAKILL => {
                    // target5: Guid
                    let target5 = Guid::astd_read(r).await?;

                    SpellLogSpellEffect::INSTAKILL {
                        target5,
                    }
                }
                SpellEffect::SCHOOL_DAMAGE => SpellLogSpellEffect::SCHOOL_DAMAGE,
                SpellEffect::DUMMY => SpellLogSpellEffect::DUMMY,
                SpellEffect::PORTAL_TELEPORT => SpellLogSpellEffect::PORTAL_TELEPORT,
                SpellEffect::TELEPORT_UNITS => SpellLogSpellEffect::TELEPORT_UNITS,
                SpellEffect::APPLY_AURA => SpellLogSpellEffect::APPLY_AURA,
                SpellEffect::ENVIRONMENTAL_DAMAGE => SpellLogSpellEffect::ENVIRONMENTAL_DAMAGE,
                SpellEffect::POWER_DRAIN => {
                    // target1: Guid
                    let target1 = Guid::astd_read(r).await?;

                    // unknown1: u32
                    let unknown1 = crate::util::astd_read_u32_le(r).await?;

                    // unknown2: u32
                    let unknown2 = crate::util::astd_read_u32_le(r).await?;

                    // unknown3: f32
                    let unknown3 = crate::util::astd_read_f32_le(r).await?;
                    SpellLogSpellEffect::POWER_DRAIN {
                        target1,
                        unknown1,
                        unknown2,
                        unknown3,
                    }
                }
                SpellEffect::HEALTH_LEECH => SpellLogSpellEffect::HEALTH_LEECH,
                SpellEffect::HEAL => SpellLogSpellEffect::HEAL,
                SpellEffect::BIND => SpellLogSpellEffect::BIND,
                SpellEffect::PORTAL => SpellLogSpellEffect::PORTAL,
                SpellEffect::RITUAL_BASE => SpellLogSpellEffect::RITUAL_BASE,
                SpellEffect::RITUAL_SPECIALIZE => SpellLogSpellEffect::RITUAL_SPECIALIZE,
                SpellEffect::RITUAL_ACTIVATE_PORTAL => SpellLogSpellEffect::RITUAL_ACTIVATE_PORTAL,
                SpellEffect::QUEST_COMPLETE => SpellLogSpellEffect::QUEST_COMPLETE,
                SpellEffect::WEAPON_DAMAGE_NOSCHOOL => SpellLogSpellEffect::WEAPON_DAMAGE_NOSCHOOL,
                SpellEffect::RESURRECT => {
                    // target5: Guid
                    let target5 = Guid::astd_read(r).await?;

                    SpellLogSpellEffect::RESURRECT {
                        target5,
                    }
                }
                SpellEffect::ADD_EXTRA_ATTACKS => {
                    // target2: Guid
                    let target2 = Guid::astd_read(r).await?;

                    // unknown4: u32
                    let unknown4 = crate::util::astd_read_u32_le(r).await?;

                    SpellLogSpellEffect::ADD_EXTRA_ATTACKS {
                        target2,
                        unknown4,
                    }
                }
                SpellEffect::DODGE => SpellLogSpellEffect::DODGE,
                SpellEffect::EVADE => SpellLogSpellEffect::EVADE,
                SpellEffect::PARRY => SpellLogSpellEffect::PARRY,
                SpellEffect::BLOCK => SpellLogSpellEffect::BLOCK,
                SpellEffect::CREATE_ITEM => {
                    // spell_effect_item_type: u32
                    let spell_effect_item_type = crate::util::astd_read_u32_le(r).await?;

                    SpellLogSpellEffect::CREATE_ITEM {
                        spell_effect_item_type,
                    }
                }
                SpellEffect::WEAPON => SpellLogSpellEffect::WEAPON,
                SpellEffect::DEFENSE => SpellLogSpellEffect::DEFENSE,
                SpellEffect::PERSISTENT_AREA_AURA => SpellLogSpellEffect::PERSISTENT_AREA_AURA,
                SpellEffect::SUMMON => SpellLogSpellEffect::SUMMON,
                SpellEffect::LEAP => SpellLogSpellEffect::LEAP,
                SpellEffect::ENERGIZE => SpellLogSpellEffect::ENERGIZE,
                SpellEffect::WEAPON_PERCENT_DAMAGE => SpellLogSpellEffect::WEAPON_PERCENT_DAMAGE,
                SpellEffect::TRIGGER_MISSILE => SpellLogSpellEffect::TRIGGER_MISSILE,
                SpellEffect::OPEN_LOCK => {
                    // target5: Guid
                    let target5 = Guid::astd_read(r).await?;

                    SpellLogSpellEffect::OPEN_LOCK {
                        target5,
                    }
                }
                SpellEffect::SUMMON_CHANGE_ITEM => SpellLogSpellEffect::SUMMON_CHANGE_ITEM,
                SpellEffect::APPLY_AREA_AURA_PARTY => SpellLogSpellEffect::APPLY_AREA_AURA_PARTY,
                SpellEffect::LEARN_SPELL => SpellLogSpellEffect::LEARN_SPELL,
                SpellEffect::SPELL_DEFENSE => SpellLogSpellEffect::SPELL_DEFENSE,
                SpellEffect::DISPEL => {
                    // target5: Guid
                    let target5 = Guid::astd_read(r).await?;

                    SpellLogSpellEffect::DISPEL {
                        target5,
                    }
                }
                SpellEffect::LANGUAGE => SpellLogSpellEffect::LANGUAGE,
                SpellEffect::DUAL_WIELD => SpellLogSpellEffect::DUAL_WIELD,
                SpellEffect::SUMMON_WILD => SpellLogSpellEffect::SUMMON_WILD,
                SpellEffect::SUMMON_GUARDIAN => SpellLogSpellEffect::SUMMON_GUARDIAN,
                SpellEffect::TELEPORT_UNITS_FACE_CASTER => SpellLogSpellEffect::TELEPORT_UNITS_FACE_CASTER,
                SpellEffect::SKILL_STEP => SpellLogSpellEffect::SKILL_STEP,
                SpellEffect::ADD_HONOR => SpellLogSpellEffect::ADD_HONOR,
                SpellEffect::SPAWN => SpellLogSpellEffect::SPAWN,
                SpellEffect::TRADE_SKILL => SpellLogSpellEffect::TRADE_SKILL,
                SpellEffect::STEALTH => SpellLogSpellEffect::STEALTH,
                SpellEffect::DETECT => SpellLogSpellEffect::DETECT,
                SpellEffect::TRANS_DOOR => SpellLogSpellEffect::TRANS_DOOR,
                SpellEffect::FORCE_CRITICAL_HIT => SpellLogSpellEffect::FORCE_CRITICAL_HIT,
                SpellEffect::GUARANTEE_HIT => SpellLogSpellEffect::GUARANTEE_HIT,
                SpellEffect::ENCHANT_ITEM => SpellLogSpellEffect::ENCHANT_ITEM,
                SpellEffect::ENCHANT_ITEM_TEMPORARY => SpellLogSpellEffect::ENCHANT_ITEM_TEMPORARY,
                SpellEffect::TAMECREATURE => SpellLogSpellEffect::TAMECREATURE,
                SpellEffect::SUMMON_PET => SpellLogSpellEffect::SUMMON_PET,
                SpellEffect::LEARN_PET_SPELL => SpellLogSpellEffect::LEARN_PET_SPELL,
                SpellEffect::WEAPON_DAMAGE => SpellLogSpellEffect::WEAPON_DAMAGE,
                SpellEffect::OPEN_LOCK_ITEM => {
                    // target5: Guid
                    let target5 = Guid::astd_read(r).await?;

                    SpellLogSpellEffect::OPEN_LOCK_ITEM {
                        target5,
                    }
                }
                SpellEffect::PROFICIENCY => SpellLogSpellEffect::PROFICIENCY,
                SpellEffect::SEND_EVENT => SpellLogSpellEffect::SEND_EVENT,
                SpellEffect::POWER_BURN => SpellLogSpellEffect::POWER_BURN,
                SpellEffect::THREAT => {
                    // target5: Guid
                    let target5 = Guid::astd_read(r).await?;

                    SpellLogSpellEffect::THREAT {
                        target5,
                    }
                }
                SpellEffect::TRIGGER_SPELL => SpellLogSpellEffect::TRIGGER_SPELL,
                SpellEffect::HEALTH_FUNNEL => SpellLogSpellEffect::HEALTH_FUNNEL,
                SpellEffect::POWER_FUNNEL => SpellLogSpellEffect::POWER_FUNNEL,
                SpellEffect::HEAL_MAX_HEALTH => SpellLogSpellEffect::HEAL_MAX_HEALTH,
                SpellEffect::INTERRUPT_CAST => {
                    // target3: Guid
                    let target3 = Guid::astd_read(r).await?;

                    // interrupted_spell: u32
                    let interrupted_spell = crate::util::astd_read_u32_le(r).await?;

                    SpellLogSpellEffect::INTERRUPT_CAST {
                        interrupted_spell,
                        target3,
                    }
                }
                SpellEffect::DISTRACT => {
                    // target5: Guid
                    let target5 = Guid::astd_read(r).await?;

                    SpellLogSpellEffect::DISTRACT {
                        target5,
                    }
                }
                SpellEffect::PULL => SpellLogSpellEffect::PULL,
                SpellEffect::PICKPOCKET => SpellLogSpellEffect::PICKPOCKET,
                SpellEffect::ADD_FARSIGHT => SpellLogSpellEffect::ADD_FARSIGHT,
                SpellEffect::SUMMON_POSSESSED => SpellLogSpellEffect::SUMMON_POSSESSED,
                SpellEffect::SUMMON_TOTEM => SpellLogSpellEffect::SUMMON_TOTEM,
                SpellEffect::HEAL_MECHANICAL => SpellLogSpellEffect::HEAL_MECHANICAL,
                SpellEffect::SUMMON_OBJECT_WILD => SpellLogSpellEffect::SUMMON_OBJECT_WILD,
                SpellEffect::SCRIPT_EFFECT => SpellLogSpellEffect::SCRIPT_EFFECT,
                SpellEffect::ATTACK => SpellLogSpellEffect::ATTACK,
                SpellEffect::SANCTUARY => {
                    // target5: Guid
                    let target5 = Guid::astd_read(r).await?;

                    SpellLogSpellEffect::SANCTUARY {
                        target5,
                    }
                }
                SpellEffect::ADD_COMBO_POINTS => SpellLogSpellEffect::ADD_COMBO_POINTS,
                SpellEffect::CREATE_HOUSE => SpellLogSpellEffect::CREATE_HOUSE,
                SpellEffect::BIND_SIGHT => SpellLogSpellEffect::BIND_SIGHT,
                SpellEffect::DUEL => SpellLogSpellEffect::DUEL,
                SpellEffect::STUCK => SpellLogSpellEffect::STUCK,
                SpellEffect::SUMMON_PLAYER => SpellLogSpellEffect::SUMMON_PLAYER,
                SpellEffect::ACTIVATE_OBJECT => SpellLogSpellEffect::ACTIVATE_OBJECT,
                SpellEffect::SUMMON_TOTEM_SLOT1 => SpellLogSpellEffect::SUMMON_TOTEM_SLOT1,
                SpellEffect::SUMMON_TOTEM_SLOT2 => SpellLogSpellEffect::SUMMON_TOTEM_SLOT2,
                SpellEffect::SUMMON_TOTEM_SLOT3 => SpellLogSpellEffect::SUMMON_TOTEM_SLOT3,
                SpellEffect::SUMMON_TOTEM_SLOT4 => SpellLogSpellEffect::SUMMON_TOTEM_SLOT4,
                SpellEffect::THREAT_ALL => {
                    // target5: Guid
                    let target5 = Guid::astd_read(r).await?;

                    SpellLogSpellEffect::THREAT_ALL {
                        target5,
                    }
                }
                SpellEffect::ENCHANT_HELD_ITEM => SpellLogSpellEffect::ENCHANT_HELD_ITEM,
                SpellEffect::SUMMON_PHANTASM => SpellLogSpellEffect::SUMMON_PHANTASM,
                SpellEffect::SELF_RESURRECT => SpellLogSpellEffect::SELF_RESURRECT,
                SpellEffect::SKINNING => SpellLogSpellEffect::SKINNING,
                SpellEffect::CHARGE => SpellLogSpellEffect::CHARGE,
                SpellEffect::SUMMON_CRITTER => SpellLogSpellEffect::SUMMON_CRITTER,
                SpellEffect::KNOCK_BACK => SpellLogSpellEffect::KNOCK_BACK,
                SpellEffect::DISENCHANT => SpellLogSpellEffect::DISENCHANT,
                SpellEffect::INEBRIATE => SpellLogSpellEffect::INEBRIATE,
                SpellEffect::FEED_PET => {
                    // item_target_entry: u32
                    let item_target_entry = crate::util::astd_read_u32_le(r).await?;

                    SpellLogSpellEffect::FEED_PET {
                        item_target_entry,
                    }
                }
                SpellEffect::DISMISS_PET => {
                    // target5: Guid
                    let target5 = Guid::astd_read(r).await?;

                    SpellLogSpellEffect::DISMISS_PET {
                        target5,
                    }
                }
                SpellEffect::REPUTATION => SpellLogSpellEffect::REPUTATION,
                SpellEffect::SUMMON_OBJECT_SLOT1 => SpellLogSpellEffect::SUMMON_OBJECT_SLOT1,
                SpellEffect::SUMMON_OBJECT_SLOT2 => SpellLogSpellEffect::SUMMON_OBJECT_SLOT2,
                SpellEffect::SUMMON_OBJECT_SLOT3 => SpellLogSpellEffect::SUMMON_OBJECT_SLOT3,
                SpellEffect::SUMMON_OBJECT_SLOT4 => SpellLogSpellEffect::SUMMON_OBJECT_SLOT4,
                SpellEffect::DISPEL_MECHANIC => {
                    // target5: Guid
                    let target5 = Guid::astd_read(r).await?;

                    SpellLogSpellEffect::DISPEL_MECHANIC {
                        target5,
                    }
                }
                SpellEffect::SUMMON_DEAD_PET => SpellLogSpellEffect::SUMMON_DEAD_PET,
                SpellEffect::DESTROY_ALL_TOTEMS => SpellLogSpellEffect::DESTROY_ALL_TOTEMS,
                SpellEffect::DURABILITY_DAMAGE => {
                    // target4: Guid
                    let target4 = Guid::astd_read(r).await?;

                    // unknown5: u32
                    let unknown5 = crate::util::astd_read_u32_le(r).await?;

                    // unknown6: u32
                    let unknown6 = crate::util::astd_read_u32_le(r).await?;

                    SpellLogSpellEffect::DURABILITY_DAMAGE {
                        target4,
                        unknown5,
                        unknown6,
                    }
                }
                SpellEffect::SUMMON_DEMON => SpellLogSpellEffect::SUMMON_DEMON,
                SpellEffect::RESURRECT_NEW => {
                    // target5: Guid
                    let target5 = Guid::astd_read(r).await?;

                    SpellLogSpellEffect::RESURRECT_NEW {
                        target5,
                    }
                }
                SpellEffect::ATTACK_ME => {
                    // target5: Guid
                    let target5 = Guid::astd_read(r).await?;

                    SpellLogSpellEffect::ATTACK_ME {
                        target5,
                    }
                }
                SpellEffect::DURABILITY_DAMAGE_PCT => SpellLogSpellEffect::DURABILITY_DAMAGE_PCT,
                SpellEffect::SKIN_PLAYER_CORPSE => {
                    // target5: Guid
                    let target5 = Guid::astd_read(r).await?;

                    SpellLogSpellEffect::SKIN_PLAYER_CORPSE {
                        target5,
                    }
                }
                SpellEffect::SPIRIT_HEAL => SpellLogSpellEffect::SPIRIT_HEAL,
                SpellEffect::SKILL => SpellLogSpellEffect::SKILL,
                SpellEffect::APPLY_AREA_AURA_PET => SpellLogSpellEffect::APPLY_AREA_AURA_PET,
                SpellEffect::TELEPORT_GRAVEYARD => SpellLogSpellEffect::TELEPORT_GRAVEYARD,
                SpellEffect::NORMALIZED_WEAPON_DMG => SpellLogSpellEffect::NORMALIZED_WEAPON_DMG,
                SpellEffect::UNKNOWN122 => SpellLogSpellEffect::UNKNOWN122,
                SpellEffect::SEND_TAXI => SpellLogSpellEffect::SEND_TAXI,
                SpellEffect::PLAYER_PULL => SpellLogSpellEffect::PLAYER_PULL,
                SpellEffect::MODIFY_THREAT_PERCENT => {
                    // target5: Guid
                    let target5 = Guid::astd_read(r).await?;

                    SpellLogSpellEffect::MODIFY_THREAT_PERCENT {
                        target5,
                    }
                }
                SpellEffect::UNKNOWN126 => {
                    // target5: Guid
                    let target5 = Guid::astd_read(r).await?;

                    SpellLogSpellEffect::UNKNOWN126 {
                        target5,
                    }
                }
                SpellEffect::UNKNOWN127 => SpellLogSpellEffect::UNKNOWN127,
            };

            Ok(Self {
                effect: effect_if,
            })
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // effect: SpellEffect
            self.effect.astd_write(w).await?;

            // amount_of_logs: u32
            w.write_all(&Self::AMOUNT_OF_LOGS_VALUE.to_le_bytes()).await?;

            match &self.effect {
                SpellLogSpellEffect::NONE => {}
                SpellLogSpellEffect::INSTAKILL {
                    target5,
                } => {
                    // target5: Guid
                    target5.astd_write(w).await?;

                }
                SpellLogSpellEffect::SCHOOL_DAMAGE => {}
                SpellLogSpellEffect::DUMMY => {}
                SpellLogSpellEffect::PORTAL_TELEPORT => {}
                SpellLogSpellEffect::TELEPORT_UNITS => {}
                SpellLogSpellEffect::APPLY_AURA => {}
                SpellLogSpellEffect::ENVIRONMENTAL_DAMAGE => {}
                SpellLogSpellEffect::POWER_DRAIN {
                    target1,
                    unknown1,
                    unknown2,
                    unknown3,
                } => {
                    // target1: Guid
                    target1.astd_write(w).await?;

                    // unknown1: u32
                    w.write_all(&unknown1.to_le_bytes()).await?;

                    // unknown2: u32
                    w.write_all(&unknown2.to_le_bytes()).await?;

                    // unknown3: f32
                    w.write_all(&unknown3.to_le_bytes()).await?;

                }
                SpellLogSpellEffect::HEALTH_LEECH => {}
                SpellLogSpellEffect::HEAL => {}
                SpellLogSpellEffect::BIND => {}
                SpellLogSpellEffect::PORTAL => {}
                SpellLogSpellEffect::RITUAL_BASE => {}
                SpellLogSpellEffect::RITUAL_SPECIALIZE => {}
                SpellLogSpellEffect::RITUAL_ACTIVATE_PORTAL => {}
                SpellLogSpellEffect::QUEST_COMPLETE => {}
                SpellLogSpellEffect::WEAPON_DAMAGE_NOSCHOOL => {}
                SpellLogSpellEffect::RESURRECT {
                    target5,
                } => {
                    // target5: Guid
                    target5.astd_write(w).await?;

                }
                SpellLogSpellEffect::ADD_EXTRA_ATTACKS {
                    target2,
                    unknown4,
                } => {
                    // target2: Guid
                    target2.astd_write(w).await?;

                    // unknown4: u32
                    w.write_all(&unknown4.to_le_bytes()).await?;

                }
                SpellLogSpellEffect::DODGE => {}
                SpellLogSpellEffect::EVADE => {}
                SpellLogSpellEffect::PARRY => {}
                SpellLogSpellEffect::BLOCK => {}
                SpellLogSpellEffect::CREATE_ITEM {
                    spell_effect_item_type,
                } => {
                    // spell_effect_item_type: u32
                    w.write_all(&spell_effect_item_type.to_le_bytes()).await?;

                }
                SpellLogSpellEffect::WEAPON => {}
                SpellLogSpellEffect::DEFENSE => {}
                SpellLogSpellEffect::PERSISTENT_AREA_AURA => {}
                SpellLogSpellEffect::SUMMON => {}
                SpellLogSpellEffect::LEAP => {}
                SpellLogSpellEffect::ENERGIZE => {}
                SpellLogSpellEffect::WEAPON_PERCENT_DAMAGE => {}
                SpellLogSpellEffect::TRIGGER_MISSILE => {}
                SpellLogSpellEffect::OPEN_LOCK {
                    target5,
                } => {
                    // target5: Guid
                    target5.astd_write(w).await?;

                }
                SpellLogSpellEffect::SUMMON_CHANGE_ITEM => {}
                SpellLogSpellEffect::APPLY_AREA_AURA_PARTY => {}
                SpellLogSpellEffect::LEARN_SPELL => {}
                SpellLogSpellEffect::SPELL_DEFENSE => {}
                SpellLogSpellEffect::DISPEL {
                    target5,
                } => {
                    // target5: Guid
                    target5.astd_write(w).await?;

                }
                SpellLogSpellEffect::LANGUAGE => {}
                SpellLogSpellEffect::DUAL_WIELD => {}
                SpellLogSpellEffect::SUMMON_WILD => {}
                SpellLogSpellEffect::SUMMON_GUARDIAN => {}
                SpellLogSpellEffect::TELEPORT_UNITS_FACE_CASTER => {}
                SpellLogSpellEffect::SKILL_STEP => {}
                SpellLogSpellEffect::ADD_HONOR => {}
                SpellLogSpellEffect::SPAWN => {}
                SpellLogSpellEffect::TRADE_SKILL => {}
                SpellLogSpellEffect::STEALTH => {}
                SpellLogSpellEffect::DETECT => {}
                SpellLogSpellEffect::TRANS_DOOR => {}
                SpellLogSpellEffect::FORCE_CRITICAL_HIT => {}
                SpellLogSpellEffect::GUARANTEE_HIT => {}
                SpellLogSpellEffect::ENCHANT_ITEM => {}
                SpellLogSpellEffect::ENCHANT_ITEM_TEMPORARY => {}
                SpellLogSpellEffect::TAMECREATURE => {}
                SpellLogSpellEffect::SUMMON_PET => {}
                SpellLogSpellEffect::LEARN_PET_SPELL => {}
                SpellLogSpellEffect::WEAPON_DAMAGE => {}
                SpellLogSpellEffect::OPEN_LOCK_ITEM {
                    target5,
                } => {
                    // target5: Guid
                    target5.astd_write(w).await?;

                }
                SpellLogSpellEffect::PROFICIENCY => {}
                SpellLogSpellEffect::SEND_EVENT => {}
                SpellLogSpellEffect::POWER_BURN => {}
                SpellLogSpellEffect::THREAT {
                    target5,
                } => {
                    // target5: Guid
                    target5.astd_write(w).await?;

                }
                SpellLogSpellEffect::TRIGGER_SPELL => {}
                SpellLogSpellEffect::HEALTH_FUNNEL => {}
                SpellLogSpellEffect::POWER_FUNNEL => {}
                SpellLogSpellEffect::HEAL_MAX_HEALTH => {}
                SpellLogSpellEffect::INTERRUPT_CAST {
                    interrupted_spell,
                    target3,
                } => {
                    // target3: Guid
                    target3.astd_write(w).await?;

                    // interrupted_spell: u32
                    w.write_all(&interrupted_spell.to_le_bytes()).await?;

                }
                SpellLogSpellEffect::DISTRACT {
                    target5,
                } => {
                    // target5: Guid
                    target5.astd_write(w).await?;

                }
                SpellLogSpellEffect::PULL => {}
                SpellLogSpellEffect::PICKPOCKET => {}
                SpellLogSpellEffect::ADD_FARSIGHT => {}
                SpellLogSpellEffect::SUMMON_POSSESSED => {}
                SpellLogSpellEffect::SUMMON_TOTEM => {}
                SpellLogSpellEffect::HEAL_MECHANICAL => {}
                SpellLogSpellEffect::SUMMON_OBJECT_WILD => {}
                SpellLogSpellEffect::SCRIPT_EFFECT => {}
                SpellLogSpellEffect::ATTACK => {}
                SpellLogSpellEffect::SANCTUARY {
                    target5,
                } => {
                    // target5: Guid
                    target5.astd_write(w).await?;

                }
                SpellLogSpellEffect::ADD_COMBO_POINTS => {}
                SpellLogSpellEffect::CREATE_HOUSE => {}
                SpellLogSpellEffect::BIND_SIGHT => {}
                SpellLogSpellEffect::DUEL => {}
                SpellLogSpellEffect::STUCK => {}
                SpellLogSpellEffect::SUMMON_PLAYER => {}
                SpellLogSpellEffect::ACTIVATE_OBJECT => {}
                SpellLogSpellEffect::SUMMON_TOTEM_SLOT1 => {}
                SpellLogSpellEffect::SUMMON_TOTEM_SLOT2 => {}
                SpellLogSpellEffect::SUMMON_TOTEM_SLOT3 => {}
                SpellLogSpellEffect::SUMMON_TOTEM_SLOT4 => {}
                SpellLogSpellEffect::THREAT_ALL {
                    target5,
                } => {
                    // target5: Guid
                    target5.astd_write(w).await?;

                }
                SpellLogSpellEffect::ENCHANT_HELD_ITEM => {}
                SpellLogSpellEffect::SUMMON_PHANTASM => {}
                SpellLogSpellEffect::SELF_RESURRECT => {}
                SpellLogSpellEffect::SKINNING => {}
                SpellLogSpellEffect::CHARGE => {}
                SpellLogSpellEffect::SUMMON_CRITTER => {}
                SpellLogSpellEffect::KNOCK_BACK => {}
                SpellLogSpellEffect::DISENCHANT => {}
                SpellLogSpellEffect::INEBRIATE => {}
                SpellLogSpellEffect::FEED_PET {
                    item_target_entry,
                } => {
                    // item_target_entry: u32
                    w.write_all(&item_target_entry.to_le_bytes()).await?;

                }
                SpellLogSpellEffect::DISMISS_PET {
                    target5,
                } => {
                    // target5: Guid
                    target5.astd_write(w).await?;

                }
                SpellLogSpellEffect::REPUTATION => {}
                SpellLogSpellEffect::SUMMON_OBJECT_SLOT1 => {}
                SpellLogSpellEffect::SUMMON_OBJECT_SLOT2 => {}
                SpellLogSpellEffect::SUMMON_OBJECT_SLOT3 => {}
                SpellLogSpellEffect::SUMMON_OBJECT_SLOT4 => {}
                SpellLogSpellEffect::DISPEL_MECHANIC {
                    target5,
                } => {
                    // target5: Guid
                    target5.astd_write(w).await?;

                }
                SpellLogSpellEffect::SUMMON_DEAD_PET => {}
                SpellLogSpellEffect::DESTROY_ALL_TOTEMS => {}
                SpellLogSpellEffect::DURABILITY_DAMAGE {
                    target4,
                    unknown5,
                    unknown6,
                } => {
                    // target4: Guid
                    target4.astd_write(w).await?;

                    // unknown5: u32
                    w.write_all(&unknown5.to_le_bytes()).await?;

                    // unknown6: u32
                    w.write_all(&unknown6.to_le_bytes()).await?;

                }
                SpellLogSpellEffect::SUMMON_DEMON => {}
                SpellLogSpellEffect::RESURRECT_NEW {
                    target5,
                } => {
                    // target5: Guid
                    target5.astd_write(w).await?;

                }
                SpellLogSpellEffect::ATTACK_ME {
                    target5,
                } => {
                    // target5: Guid
                    target5.astd_write(w).await?;

                }
                SpellLogSpellEffect::DURABILITY_DAMAGE_PCT => {}
                SpellLogSpellEffect::SKIN_PLAYER_CORPSE {
                    target5,
                } => {
                    // target5: Guid
                    target5.astd_write(w).await?;

                }
                SpellLogSpellEffect::SPIRIT_HEAL => {}
                SpellLogSpellEffect::SKILL => {}
                SpellLogSpellEffect::APPLY_AREA_AURA_PET => {}
                SpellLogSpellEffect::TELEPORT_GRAVEYARD => {}
                SpellLogSpellEffect::NORMALIZED_WEAPON_DMG => {}
                SpellLogSpellEffect::UNKNOWN122 => {}
                SpellLogSpellEffect::SEND_TAXI => {}
                SpellLogSpellEffect::PLAYER_PULL => {}
                SpellLogSpellEffect::MODIFY_THREAT_PERCENT {
                    target5,
                } => {
                    // target5: Guid
                    target5.astd_write(w).await?;

                }
                SpellLogSpellEffect::UNKNOWN126 {
                    target5,
                } => {
                    // target5: Guid
                    target5.astd_write(w).await?;

                }
                SpellLogSpellEffect::UNKNOWN127 => {}
            }

            Ok(())
        })
    }

}

impl VariableSized for SpellLog {
    fn size(&self) -> usize {
        0
        + self.effect.size() // effect: SpellLogSpellEffect
        + 4 // amount_of_logs: u32
    }
}

impl MaximumPossibleSized for SpellLog {
    fn maximum_possible_size() -> usize {
        0
        + 24 // effect: SpellLogSpellEffect
        + 4 // amount_of_logs: u32
    }
}

#[derive(Debug)]
pub enum SpellLogError {
    Io(std::io::Error),
    SpellEffect(SpellEffectError),
}

impl std::error::Error for SpellLogError {}
impl std::fmt::Display for SpellLogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SpellEffect(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SpellLogError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SpellEffectError> for SpellLogError {
    fn from(e: SpellEffectError) -> Self {
        Self::SpellEffect(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SpellLogSpellEffect {
    NONE,
    INSTAKILL {
        target5: Guid,
    },
    SCHOOL_DAMAGE,
    DUMMY,
    PORTAL_TELEPORT,
    TELEPORT_UNITS,
    APPLY_AURA,
    ENVIRONMENTAL_DAMAGE,
    POWER_DRAIN {
        target1: Guid,
        unknown1: u32,
        unknown2: u32,
        unknown3: f32,
    },
    HEALTH_LEECH,
    HEAL,
    BIND,
    PORTAL,
    RITUAL_BASE,
    RITUAL_SPECIALIZE,
    RITUAL_ACTIVATE_PORTAL,
    QUEST_COMPLETE,
    WEAPON_DAMAGE_NOSCHOOL,
    RESURRECT {
        target5: Guid,
    },
    ADD_EXTRA_ATTACKS {
        target2: Guid,
        unknown4: u32,
    },
    DODGE,
    EVADE,
    PARRY,
    BLOCK,
    CREATE_ITEM {
        spell_effect_item_type: u32,
    },
    WEAPON,
    DEFENSE,
    PERSISTENT_AREA_AURA,
    SUMMON,
    LEAP,
    ENERGIZE,
    WEAPON_PERCENT_DAMAGE,
    TRIGGER_MISSILE,
    OPEN_LOCK {
        target5: Guid,
    },
    SUMMON_CHANGE_ITEM,
    APPLY_AREA_AURA_PARTY,
    LEARN_SPELL,
    SPELL_DEFENSE,
    DISPEL {
        target5: Guid,
    },
    LANGUAGE,
    DUAL_WIELD,
    SUMMON_WILD,
    SUMMON_GUARDIAN,
    TELEPORT_UNITS_FACE_CASTER,
    SKILL_STEP,
    ADD_HONOR,
    SPAWN,
    TRADE_SKILL,
    STEALTH,
    DETECT,
    TRANS_DOOR,
    FORCE_CRITICAL_HIT,
    GUARANTEE_HIT,
    ENCHANT_ITEM,
    ENCHANT_ITEM_TEMPORARY,
    TAMECREATURE,
    SUMMON_PET,
    LEARN_PET_SPELL,
    WEAPON_DAMAGE,
    OPEN_LOCK_ITEM {
        target5: Guid,
    },
    PROFICIENCY,
    SEND_EVENT,
    POWER_BURN,
    THREAT {
        target5: Guid,
    },
    TRIGGER_SPELL,
    HEALTH_FUNNEL,
    POWER_FUNNEL,
    HEAL_MAX_HEALTH,
    INTERRUPT_CAST {
        interrupted_spell: u32,
        target3: Guid,
    },
    DISTRACT {
        target5: Guid,
    },
    PULL,
    PICKPOCKET,
    ADD_FARSIGHT,
    SUMMON_POSSESSED,
    SUMMON_TOTEM,
    HEAL_MECHANICAL,
    SUMMON_OBJECT_WILD,
    SCRIPT_EFFECT,
    ATTACK,
    SANCTUARY {
        target5: Guid,
    },
    ADD_COMBO_POINTS,
    CREATE_HOUSE,
    BIND_SIGHT,
    DUEL,
    STUCK,
    SUMMON_PLAYER,
    ACTIVATE_OBJECT,
    SUMMON_TOTEM_SLOT1,
    SUMMON_TOTEM_SLOT2,
    SUMMON_TOTEM_SLOT3,
    SUMMON_TOTEM_SLOT4,
    THREAT_ALL {
        target5: Guid,
    },
    ENCHANT_HELD_ITEM,
    SUMMON_PHANTASM,
    SELF_RESURRECT,
    SKINNING,
    CHARGE,
    SUMMON_CRITTER,
    KNOCK_BACK,
    DISENCHANT,
    INEBRIATE,
    FEED_PET {
        item_target_entry: u32,
    },
    DISMISS_PET {
        target5: Guid,
    },
    REPUTATION,
    SUMMON_OBJECT_SLOT1,
    SUMMON_OBJECT_SLOT2,
    SUMMON_OBJECT_SLOT3,
    SUMMON_OBJECT_SLOT4,
    DISPEL_MECHANIC {
        target5: Guid,
    },
    SUMMON_DEAD_PET,
    DESTROY_ALL_TOTEMS,
    DURABILITY_DAMAGE {
        target4: Guid,
        unknown5: u32,
        unknown6: u32,
    },
    SUMMON_DEMON,
    RESURRECT_NEW {
        target5: Guid,
    },
    ATTACK_ME {
        target5: Guid,
    },
    DURABILITY_DAMAGE_PCT,
    SKIN_PLAYER_CORPSE {
        target5: Guid,
    },
    SPIRIT_HEAL,
    SKILL,
    APPLY_AREA_AURA_PET,
    TELEPORT_GRAVEYARD,
    NORMALIZED_WEAPON_DMG,
    UNKNOWN122,
    SEND_TAXI,
    PLAYER_PULL,
    MODIFY_THREAT_PERCENT {
        target5: Guid,
    },
    UNKNOWN126 {
        target5: Guid,
    },
    UNKNOWN127,
}

impl From<&SpellEffect> for SpellLogSpellEffect {
    fn from(e: &SpellEffect) -> Self {
        match &e {
            SpellEffect::NONE => Self::NONE,
            SpellEffect::INSTAKILL => Self::INSTAKILL {
                target5: Default::default(),
            },
            SpellEffect::SCHOOL_DAMAGE => Self::SCHOOL_DAMAGE,
            SpellEffect::DUMMY => Self::DUMMY,
            SpellEffect::PORTAL_TELEPORT => Self::PORTAL_TELEPORT,
            SpellEffect::TELEPORT_UNITS => Self::TELEPORT_UNITS,
            SpellEffect::APPLY_AURA => Self::APPLY_AURA,
            SpellEffect::ENVIRONMENTAL_DAMAGE => Self::ENVIRONMENTAL_DAMAGE,
            SpellEffect::POWER_DRAIN => Self::POWER_DRAIN {
                target1: Default::default(),
                unknown1: Default::default(),
                unknown2: Default::default(),
                unknown3: Default::default(),
            },
            SpellEffect::HEALTH_LEECH => Self::HEALTH_LEECH,
            SpellEffect::HEAL => Self::HEAL,
            SpellEffect::BIND => Self::BIND,
            SpellEffect::PORTAL => Self::PORTAL,
            SpellEffect::RITUAL_BASE => Self::RITUAL_BASE,
            SpellEffect::RITUAL_SPECIALIZE => Self::RITUAL_SPECIALIZE,
            SpellEffect::RITUAL_ACTIVATE_PORTAL => Self::RITUAL_ACTIVATE_PORTAL,
            SpellEffect::QUEST_COMPLETE => Self::QUEST_COMPLETE,
            SpellEffect::WEAPON_DAMAGE_NOSCHOOL => Self::WEAPON_DAMAGE_NOSCHOOL,
            SpellEffect::RESURRECT => Self::RESURRECT {
                target5: Default::default(),
            },
            SpellEffect::ADD_EXTRA_ATTACKS => Self::ADD_EXTRA_ATTACKS {
                target2: Default::default(),
                unknown4: Default::default(),
            },
            SpellEffect::DODGE => Self::DODGE,
            SpellEffect::EVADE => Self::EVADE,
            SpellEffect::PARRY => Self::PARRY,
            SpellEffect::BLOCK => Self::BLOCK,
            SpellEffect::CREATE_ITEM => Self::CREATE_ITEM {
                spell_effect_item_type: Default::default(),
            },
            SpellEffect::WEAPON => Self::WEAPON,
            SpellEffect::DEFENSE => Self::DEFENSE,
            SpellEffect::PERSISTENT_AREA_AURA => Self::PERSISTENT_AREA_AURA,
            SpellEffect::SUMMON => Self::SUMMON,
            SpellEffect::LEAP => Self::LEAP,
            SpellEffect::ENERGIZE => Self::ENERGIZE,
            SpellEffect::WEAPON_PERCENT_DAMAGE => Self::WEAPON_PERCENT_DAMAGE,
            SpellEffect::TRIGGER_MISSILE => Self::TRIGGER_MISSILE,
            SpellEffect::OPEN_LOCK => Self::OPEN_LOCK {
                target5: Default::default(),
            },
            SpellEffect::SUMMON_CHANGE_ITEM => Self::SUMMON_CHANGE_ITEM,
            SpellEffect::APPLY_AREA_AURA_PARTY => Self::APPLY_AREA_AURA_PARTY,
            SpellEffect::LEARN_SPELL => Self::LEARN_SPELL,
            SpellEffect::SPELL_DEFENSE => Self::SPELL_DEFENSE,
            SpellEffect::DISPEL => Self::DISPEL {
                target5: Default::default(),
            },
            SpellEffect::LANGUAGE => Self::LANGUAGE,
            SpellEffect::DUAL_WIELD => Self::DUAL_WIELD,
            SpellEffect::SUMMON_WILD => Self::SUMMON_WILD,
            SpellEffect::SUMMON_GUARDIAN => Self::SUMMON_GUARDIAN,
            SpellEffect::TELEPORT_UNITS_FACE_CASTER => Self::TELEPORT_UNITS_FACE_CASTER,
            SpellEffect::SKILL_STEP => Self::SKILL_STEP,
            SpellEffect::ADD_HONOR => Self::ADD_HONOR,
            SpellEffect::SPAWN => Self::SPAWN,
            SpellEffect::TRADE_SKILL => Self::TRADE_SKILL,
            SpellEffect::STEALTH => Self::STEALTH,
            SpellEffect::DETECT => Self::DETECT,
            SpellEffect::TRANS_DOOR => Self::TRANS_DOOR,
            SpellEffect::FORCE_CRITICAL_HIT => Self::FORCE_CRITICAL_HIT,
            SpellEffect::GUARANTEE_HIT => Self::GUARANTEE_HIT,
            SpellEffect::ENCHANT_ITEM => Self::ENCHANT_ITEM,
            SpellEffect::ENCHANT_ITEM_TEMPORARY => Self::ENCHANT_ITEM_TEMPORARY,
            SpellEffect::TAMECREATURE => Self::TAMECREATURE,
            SpellEffect::SUMMON_PET => Self::SUMMON_PET,
            SpellEffect::LEARN_PET_SPELL => Self::LEARN_PET_SPELL,
            SpellEffect::WEAPON_DAMAGE => Self::WEAPON_DAMAGE,
            SpellEffect::OPEN_LOCK_ITEM => Self::OPEN_LOCK_ITEM {
                target5: Default::default(),
            },
            SpellEffect::PROFICIENCY => Self::PROFICIENCY,
            SpellEffect::SEND_EVENT => Self::SEND_EVENT,
            SpellEffect::POWER_BURN => Self::POWER_BURN,
            SpellEffect::THREAT => Self::THREAT {
                target5: Default::default(),
            },
            SpellEffect::TRIGGER_SPELL => Self::TRIGGER_SPELL,
            SpellEffect::HEALTH_FUNNEL => Self::HEALTH_FUNNEL,
            SpellEffect::POWER_FUNNEL => Self::POWER_FUNNEL,
            SpellEffect::HEAL_MAX_HEALTH => Self::HEAL_MAX_HEALTH,
            SpellEffect::INTERRUPT_CAST => Self::INTERRUPT_CAST {
                interrupted_spell: Default::default(),
                target3: Default::default(),
            },
            SpellEffect::DISTRACT => Self::DISTRACT {
                target5: Default::default(),
            },
            SpellEffect::PULL => Self::PULL,
            SpellEffect::PICKPOCKET => Self::PICKPOCKET,
            SpellEffect::ADD_FARSIGHT => Self::ADD_FARSIGHT,
            SpellEffect::SUMMON_POSSESSED => Self::SUMMON_POSSESSED,
            SpellEffect::SUMMON_TOTEM => Self::SUMMON_TOTEM,
            SpellEffect::HEAL_MECHANICAL => Self::HEAL_MECHANICAL,
            SpellEffect::SUMMON_OBJECT_WILD => Self::SUMMON_OBJECT_WILD,
            SpellEffect::SCRIPT_EFFECT => Self::SCRIPT_EFFECT,
            SpellEffect::ATTACK => Self::ATTACK,
            SpellEffect::SANCTUARY => Self::SANCTUARY {
                target5: Default::default(),
            },
            SpellEffect::ADD_COMBO_POINTS => Self::ADD_COMBO_POINTS,
            SpellEffect::CREATE_HOUSE => Self::CREATE_HOUSE,
            SpellEffect::BIND_SIGHT => Self::BIND_SIGHT,
            SpellEffect::DUEL => Self::DUEL,
            SpellEffect::STUCK => Self::STUCK,
            SpellEffect::SUMMON_PLAYER => Self::SUMMON_PLAYER,
            SpellEffect::ACTIVATE_OBJECT => Self::ACTIVATE_OBJECT,
            SpellEffect::SUMMON_TOTEM_SLOT1 => Self::SUMMON_TOTEM_SLOT1,
            SpellEffect::SUMMON_TOTEM_SLOT2 => Self::SUMMON_TOTEM_SLOT2,
            SpellEffect::SUMMON_TOTEM_SLOT3 => Self::SUMMON_TOTEM_SLOT3,
            SpellEffect::SUMMON_TOTEM_SLOT4 => Self::SUMMON_TOTEM_SLOT4,
            SpellEffect::THREAT_ALL => Self::THREAT_ALL {
                target5: Default::default(),
            },
            SpellEffect::ENCHANT_HELD_ITEM => Self::ENCHANT_HELD_ITEM,
            SpellEffect::SUMMON_PHANTASM => Self::SUMMON_PHANTASM,
            SpellEffect::SELF_RESURRECT => Self::SELF_RESURRECT,
            SpellEffect::SKINNING => Self::SKINNING,
            SpellEffect::CHARGE => Self::CHARGE,
            SpellEffect::SUMMON_CRITTER => Self::SUMMON_CRITTER,
            SpellEffect::KNOCK_BACK => Self::KNOCK_BACK,
            SpellEffect::DISENCHANT => Self::DISENCHANT,
            SpellEffect::INEBRIATE => Self::INEBRIATE,
            SpellEffect::FEED_PET => Self::FEED_PET {
                item_target_entry: Default::default(),
            },
            SpellEffect::DISMISS_PET => Self::DISMISS_PET {
                target5: Default::default(),
            },
            SpellEffect::REPUTATION => Self::REPUTATION,
            SpellEffect::SUMMON_OBJECT_SLOT1 => Self::SUMMON_OBJECT_SLOT1,
            SpellEffect::SUMMON_OBJECT_SLOT2 => Self::SUMMON_OBJECT_SLOT2,
            SpellEffect::SUMMON_OBJECT_SLOT3 => Self::SUMMON_OBJECT_SLOT3,
            SpellEffect::SUMMON_OBJECT_SLOT4 => Self::SUMMON_OBJECT_SLOT4,
            SpellEffect::DISPEL_MECHANIC => Self::DISPEL_MECHANIC {
                target5: Default::default(),
            },
            SpellEffect::SUMMON_DEAD_PET => Self::SUMMON_DEAD_PET,
            SpellEffect::DESTROY_ALL_TOTEMS => Self::DESTROY_ALL_TOTEMS,
            SpellEffect::DURABILITY_DAMAGE => Self::DURABILITY_DAMAGE {
                target4: Default::default(),
                unknown5: Default::default(),
                unknown6: Default::default(),
            },
            SpellEffect::SUMMON_DEMON => Self::SUMMON_DEMON,
            SpellEffect::RESURRECT_NEW => Self::RESURRECT_NEW {
                target5: Default::default(),
            },
            SpellEffect::ATTACK_ME => Self::ATTACK_ME {
                target5: Default::default(),
            },
            SpellEffect::DURABILITY_DAMAGE_PCT => Self::DURABILITY_DAMAGE_PCT,
            SpellEffect::SKIN_PLAYER_CORPSE => Self::SKIN_PLAYER_CORPSE {
                target5: Default::default(),
            },
            SpellEffect::SPIRIT_HEAL => Self::SPIRIT_HEAL,
            SpellEffect::SKILL => Self::SKILL,
            SpellEffect::APPLY_AREA_AURA_PET => Self::APPLY_AREA_AURA_PET,
            SpellEffect::TELEPORT_GRAVEYARD => Self::TELEPORT_GRAVEYARD,
            SpellEffect::NORMALIZED_WEAPON_DMG => Self::NORMALIZED_WEAPON_DMG,
            SpellEffect::UNKNOWN122 => Self::UNKNOWN122,
            SpellEffect::SEND_TAXI => Self::SEND_TAXI,
            SpellEffect::PLAYER_PULL => Self::PLAYER_PULL,
            SpellEffect::MODIFY_THREAT_PERCENT => Self::MODIFY_THREAT_PERCENT {
                target5: Default::default(),
            },
            SpellEffect::UNKNOWN126 => Self::UNKNOWN126 {
                target5: Default::default(),
            },
            SpellEffect::UNKNOWN127 => Self::UNKNOWN127,
        }
    }
}

impl From<&SpellLogSpellEffect> for SpellEffect {
    fn from(v: &SpellLogSpellEffect) -> Self {
        match &v {
            SpellLogSpellEffect::NONE => Self::NONE,
            SpellLogSpellEffect::INSTAKILL { .. } => Self::INSTAKILL,
            SpellLogSpellEffect::SCHOOL_DAMAGE => Self::SCHOOL_DAMAGE,
            SpellLogSpellEffect::DUMMY => Self::DUMMY,
            SpellLogSpellEffect::PORTAL_TELEPORT => Self::PORTAL_TELEPORT,
            SpellLogSpellEffect::TELEPORT_UNITS => Self::TELEPORT_UNITS,
            SpellLogSpellEffect::APPLY_AURA => Self::APPLY_AURA,
            SpellLogSpellEffect::ENVIRONMENTAL_DAMAGE => Self::ENVIRONMENTAL_DAMAGE,
            SpellLogSpellEffect::POWER_DRAIN { .. } => Self::POWER_DRAIN,
            SpellLogSpellEffect::HEALTH_LEECH => Self::HEALTH_LEECH,
            SpellLogSpellEffect::HEAL => Self::HEAL,
            SpellLogSpellEffect::BIND => Self::BIND,
            SpellLogSpellEffect::PORTAL => Self::PORTAL,
            SpellLogSpellEffect::RITUAL_BASE => Self::RITUAL_BASE,
            SpellLogSpellEffect::RITUAL_SPECIALIZE => Self::RITUAL_SPECIALIZE,
            SpellLogSpellEffect::RITUAL_ACTIVATE_PORTAL => Self::RITUAL_ACTIVATE_PORTAL,
            SpellLogSpellEffect::QUEST_COMPLETE => Self::QUEST_COMPLETE,
            SpellLogSpellEffect::WEAPON_DAMAGE_NOSCHOOL => Self::WEAPON_DAMAGE_NOSCHOOL,
            SpellLogSpellEffect::RESURRECT { .. } => Self::RESURRECT,
            SpellLogSpellEffect::ADD_EXTRA_ATTACKS { .. } => Self::ADD_EXTRA_ATTACKS,
            SpellLogSpellEffect::DODGE => Self::DODGE,
            SpellLogSpellEffect::EVADE => Self::EVADE,
            SpellLogSpellEffect::PARRY => Self::PARRY,
            SpellLogSpellEffect::BLOCK => Self::BLOCK,
            SpellLogSpellEffect::CREATE_ITEM { .. } => Self::CREATE_ITEM,
            SpellLogSpellEffect::WEAPON => Self::WEAPON,
            SpellLogSpellEffect::DEFENSE => Self::DEFENSE,
            SpellLogSpellEffect::PERSISTENT_AREA_AURA => Self::PERSISTENT_AREA_AURA,
            SpellLogSpellEffect::SUMMON => Self::SUMMON,
            SpellLogSpellEffect::LEAP => Self::LEAP,
            SpellLogSpellEffect::ENERGIZE => Self::ENERGIZE,
            SpellLogSpellEffect::WEAPON_PERCENT_DAMAGE => Self::WEAPON_PERCENT_DAMAGE,
            SpellLogSpellEffect::TRIGGER_MISSILE => Self::TRIGGER_MISSILE,
            SpellLogSpellEffect::OPEN_LOCK { .. } => Self::OPEN_LOCK,
            SpellLogSpellEffect::SUMMON_CHANGE_ITEM => Self::SUMMON_CHANGE_ITEM,
            SpellLogSpellEffect::APPLY_AREA_AURA_PARTY => Self::APPLY_AREA_AURA_PARTY,
            SpellLogSpellEffect::LEARN_SPELL => Self::LEARN_SPELL,
            SpellLogSpellEffect::SPELL_DEFENSE => Self::SPELL_DEFENSE,
            SpellLogSpellEffect::DISPEL { .. } => Self::DISPEL,
            SpellLogSpellEffect::LANGUAGE => Self::LANGUAGE,
            SpellLogSpellEffect::DUAL_WIELD => Self::DUAL_WIELD,
            SpellLogSpellEffect::SUMMON_WILD => Self::SUMMON_WILD,
            SpellLogSpellEffect::SUMMON_GUARDIAN => Self::SUMMON_GUARDIAN,
            SpellLogSpellEffect::TELEPORT_UNITS_FACE_CASTER => Self::TELEPORT_UNITS_FACE_CASTER,
            SpellLogSpellEffect::SKILL_STEP => Self::SKILL_STEP,
            SpellLogSpellEffect::ADD_HONOR => Self::ADD_HONOR,
            SpellLogSpellEffect::SPAWN => Self::SPAWN,
            SpellLogSpellEffect::TRADE_SKILL => Self::TRADE_SKILL,
            SpellLogSpellEffect::STEALTH => Self::STEALTH,
            SpellLogSpellEffect::DETECT => Self::DETECT,
            SpellLogSpellEffect::TRANS_DOOR => Self::TRANS_DOOR,
            SpellLogSpellEffect::FORCE_CRITICAL_HIT => Self::FORCE_CRITICAL_HIT,
            SpellLogSpellEffect::GUARANTEE_HIT => Self::GUARANTEE_HIT,
            SpellLogSpellEffect::ENCHANT_ITEM => Self::ENCHANT_ITEM,
            SpellLogSpellEffect::ENCHANT_ITEM_TEMPORARY => Self::ENCHANT_ITEM_TEMPORARY,
            SpellLogSpellEffect::TAMECREATURE => Self::TAMECREATURE,
            SpellLogSpellEffect::SUMMON_PET => Self::SUMMON_PET,
            SpellLogSpellEffect::LEARN_PET_SPELL => Self::LEARN_PET_SPELL,
            SpellLogSpellEffect::WEAPON_DAMAGE => Self::WEAPON_DAMAGE,
            SpellLogSpellEffect::OPEN_LOCK_ITEM { .. } => Self::OPEN_LOCK_ITEM,
            SpellLogSpellEffect::PROFICIENCY => Self::PROFICIENCY,
            SpellLogSpellEffect::SEND_EVENT => Self::SEND_EVENT,
            SpellLogSpellEffect::POWER_BURN => Self::POWER_BURN,
            SpellLogSpellEffect::THREAT { .. } => Self::THREAT,
            SpellLogSpellEffect::TRIGGER_SPELL => Self::TRIGGER_SPELL,
            SpellLogSpellEffect::HEALTH_FUNNEL => Self::HEALTH_FUNNEL,
            SpellLogSpellEffect::POWER_FUNNEL => Self::POWER_FUNNEL,
            SpellLogSpellEffect::HEAL_MAX_HEALTH => Self::HEAL_MAX_HEALTH,
            SpellLogSpellEffect::INTERRUPT_CAST { .. } => Self::INTERRUPT_CAST,
            SpellLogSpellEffect::DISTRACT { .. } => Self::DISTRACT,
            SpellLogSpellEffect::PULL => Self::PULL,
            SpellLogSpellEffect::PICKPOCKET => Self::PICKPOCKET,
            SpellLogSpellEffect::ADD_FARSIGHT => Self::ADD_FARSIGHT,
            SpellLogSpellEffect::SUMMON_POSSESSED => Self::SUMMON_POSSESSED,
            SpellLogSpellEffect::SUMMON_TOTEM => Self::SUMMON_TOTEM,
            SpellLogSpellEffect::HEAL_MECHANICAL => Self::HEAL_MECHANICAL,
            SpellLogSpellEffect::SUMMON_OBJECT_WILD => Self::SUMMON_OBJECT_WILD,
            SpellLogSpellEffect::SCRIPT_EFFECT => Self::SCRIPT_EFFECT,
            SpellLogSpellEffect::ATTACK => Self::ATTACK,
            SpellLogSpellEffect::SANCTUARY { .. } => Self::SANCTUARY,
            SpellLogSpellEffect::ADD_COMBO_POINTS => Self::ADD_COMBO_POINTS,
            SpellLogSpellEffect::CREATE_HOUSE => Self::CREATE_HOUSE,
            SpellLogSpellEffect::BIND_SIGHT => Self::BIND_SIGHT,
            SpellLogSpellEffect::DUEL => Self::DUEL,
            SpellLogSpellEffect::STUCK => Self::STUCK,
            SpellLogSpellEffect::SUMMON_PLAYER => Self::SUMMON_PLAYER,
            SpellLogSpellEffect::ACTIVATE_OBJECT => Self::ACTIVATE_OBJECT,
            SpellLogSpellEffect::SUMMON_TOTEM_SLOT1 => Self::SUMMON_TOTEM_SLOT1,
            SpellLogSpellEffect::SUMMON_TOTEM_SLOT2 => Self::SUMMON_TOTEM_SLOT2,
            SpellLogSpellEffect::SUMMON_TOTEM_SLOT3 => Self::SUMMON_TOTEM_SLOT3,
            SpellLogSpellEffect::SUMMON_TOTEM_SLOT4 => Self::SUMMON_TOTEM_SLOT4,
            SpellLogSpellEffect::THREAT_ALL { .. } => Self::THREAT_ALL,
            SpellLogSpellEffect::ENCHANT_HELD_ITEM => Self::ENCHANT_HELD_ITEM,
            SpellLogSpellEffect::SUMMON_PHANTASM => Self::SUMMON_PHANTASM,
            SpellLogSpellEffect::SELF_RESURRECT => Self::SELF_RESURRECT,
            SpellLogSpellEffect::SKINNING => Self::SKINNING,
            SpellLogSpellEffect::CHARGE => Self::CHARGE,
            SpellLogSpellEffect::SUMMON_CRITTER => Self::SUMMON_CRITTER,
            SpellLogSpellEffect::KNOCK_BACK => Self::KNOCK_BACK,
            SpellLogSpellEffect::DISENCHANT => Self::DISENCHANT,
            SpellLogSpellEffect::INEBRIATE => Self::INEBRIATE,
            SpellLogSpellEffect::FEED_PET { .. } => Self::FEED_PET,
            SpellLogSpellEffect::DISMISS_PET { .. } => Self::DISMISS_PET,
            SpellLogSpellEffect::REPUTATION => Self::REPUTATION,
            SpellLogSpellEffect::SUMMON_OBJECT_SLOT1 => Self::SUMMON_OBJECT_SLOT1,
            SpellLogSpellEffect::SUMMON_OBJECT_SLOT2 => Self::SUMMON_OBJECT_SLOT2,
            SpellLogSpellEffect::SUMMON_OBJECT_SLOT3 => Self::SUMMON_OBJECT_SLOT3,
            SpellLogSpellEffect::SUMMON_OBJECT_SLOT4 => Self::SUMMON_OBJECT_SLOT4,
            SpellLogSpellEffect::DISPEL_MECHANIC { .. } => Self::DISPEL_MECHANIC,
            SpellLogSpellEffect::SUMMON_DEAD_PET => Self::SUMMON_DEAD_PET,
            SpellLogSpellEffect::DESTROY_ALL_TOTEMS => Self::DESTROY_ALL_TOTEMS,
            SpellLogSpellEffect::DURABILITY_DAMAGE { .. } => Self::DURABILITY_DAMAGE,
            SpellLogSpellEffect::SUMMON_DEMON => Self::SUMMON_DEMON,
            SpellLogSpellEffect::RESURRECT_NEW { .. } => Self::RESURRECT_NEW,
            SpellLogSpellEffect::ATTACK_ME { .. } => Self::ATTACK_ME,
            SpellLogSpellEffect::DURABILITY_DAMAGE_PCT => Self::DURABILITY_DAMAGE_PCT,
            SpellLogSpellEffect::SKIN_PLAYER_CORPSE { .. } => Self::SKIN_PLAYER_CORPSE,
            SpellLogSpellEffect::SPIRIT_HEAL => Self::SPIRIT_HEAL,
            SpellLogSpellEffect::SKILL => Self::SKILL,
            SpellLogSpellEffect::APPLY_AREA_AURA_PET => Self::APPLY_AREA_AURA_PET,
            SpellLogSpellEffect::TELEPORT_GRAVEYARD => Self::TELEPORT_GRAVEYARD,
            SpellLogSpellEffect::NORMALIZED_WEAPON_DMG => Self::NORMALIZED_WEAPON_DMG,
            SpellLogSpellEffect::UNKNOWN122 => Self::UNKNOWN122,
            SpellLogSpellEffect::SEND_TAXI => Self::SEND_TAXI,
            SpellLogSpellEffect::PLAYER_PULL => Self::PLAYER_PULL,
            SpellLogSpellEffect::MODIFY_THREAT_PERCENT { .. } => Self::MODIFY_THREAT_PERCENT,
            SpellLogSpellEffect::UNKNOWN126 { .. } => Self::UNKNOWN126,
            SpellLogSpellEffect::UNKNOWN127 => Self::UNKNOWN127,
        }
    }
}

impl Default for SpellLogSpellEffect {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NONE
    }
}

impl SpellLogSpellEffect {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SpellEffect = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SpellEffect = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: SpellEffect = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    pub(crate) fn as_int(&self) -> u32 {
        let a: SpellEffect = self.into();
        a.as_int() as u32
    }

}

impl VariableSized for SpellLogSpellEffect {
    fn size(&self) -> usize {
        match self {
            Self::NONE => {
                4
            }
            Self::INSTAKILL {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::SCHOOL_DAMAGE => {
                4
            }
            Self::DUMMY => {
                4
            }
            Self::PORTAL_TELEPORT => {
                4
            }
            Self::TELEPORT_UNITS => {
                4
            }
            Self::APPLY_AURA => {
                4
            }
            Self::ENVIRONMENTAL_DAMAGE => {
                4
            }
            Self::POWER_DRAIN {
                target1,
                unknown1,
                unknown2,
                unknown3,
            } => {
                4
                + 8 // target1: Guid
                + 4 // unknown1: u32
                + 4 // unknown2: u32
                + 4 // unknown3: f32
            }
            Self::HEALTH_LEECH => {
                4
            }
            Self::HEAL => {
                4
            }
            Self::BIND => {
                4
            }
            Self::PORTAL => {
                4
            }
            Self::RITUAL_BASE => {
                4
            }
            Self::RITUAL_SPECIALIZE => {
                4
            }
            Self::RITUAL_ACTIVATE_PORTAL => {
                4
            }
            Self::QUEST_COMPLETE => {
                4
            }
            Self::WEAPON_DAMAGE_NOSCHOOL => {
                4
            }
            Self::RESURRECT {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::ADD_EXTRA_ATTACKS {
                target2,
                unknown4,
            } => {
                4
                + 8 // target2: Guid
                + 4 // unknown4: u32
            }
            Self::DODGE => {
                4
            }
            Self::EVADE => {
                4
            }
            Self::PARRY => {
                4
            }
            Self::BLOCK => {
                4
            }
            Self::CREATE_ITEM {
                spell_effect_item_type,
            } => {
                4
                + 4 // spell_effect_item_type: u32
            }
            Self::WEAPON => {
                4
            }
            Self::DEFENSE => {
                4
            }
            Self::PERSISTENT_AREA_AURA => {
                4
            }
            Self::SUMMON => {
                4
            }
            Self::LEAP => {
                4
            }
            Self::ENERGIZE => {
                4
            }
            Self::WEAPON_PERCENT_DAMAGE => {
                4
            }
            Self::TRIGGER_MISSILE => {
                4
            }
            Self::OPEN_LOCK {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::SUMMON_CHANGE_ITEM => {
                4
            }
            Self::APPLY_AREA_AURA_PARTY => {
                4
            }
            Self::LEARN_SPELL => {
                4
            }
            Self::SPELL_DEFENSE => {
                4
            }
            Self::DISPEL {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::LANGUAGE => {
                4
            }
            Self::DUAL_WIELD => {
                4
            }
            Self::SUMMON_WILD => {
                4
            }
            Self::SUMMON_GUARDIAN => {
                4
            }
            Self::TELEPORT_UNITS_FACE_CASTER => {
                4
            }
            Self::SKILL_STEP => {
                4
            }
            Self::ADD_HONOR => {
                4
            }
            Self::SPAWN => {
                4
            }
            Self::TRADE_SKILL => {
                4
            }
            Self::STEALTH => {
                4
            }
            Self::DETECT => {
                4
            }
            Self::TRANS_DOOR => {
                4
            }
            Self::FORCE_CRITICAL_HIT => {
                4
            }
            Self::GUARANTEE_HIT => {
                4
            }
            Self::ENCHANT_ITEM => {
                4
            }
            Self::ENCHANT_ITEM_TEMPORARY => {
                4
            }
            Self::TAMECREATURE => {
                4
            }
            Self::SUMMON_PET => {
                4
            }
            Self::LEARN_PET_SPELL => {
                4
            }
            Self::WEAPON_DAMAGE => {
                4
            }
            Self::OPEN_LOCK_ITEM {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::PROFICIENCY => {
                4
            }
            Self::SEND_EVENT => {
                4
            }
            Self::POWER_BURN => {
                4
            }
            Self::THREAT {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::TRIGGER_SPELL => {
                4
            }
            Self::HEALTH_FUNNEL => {
                4
            }
            Self::POWER_FUNNEL => {
                4
            }
            Self::HEAL_MAX_HEALTH => {
                4
            }
            Self::INTERRUPT_CAST {
                interrupted_spell,
                target3,
            } => {
                4
                + 4 // interrupted_spell: u32
                + 8 // target3: Guid
            }
            Self::DISTRACT {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::PULL => {
                4
            }
            Self::PICKPOCKET => {
                4
            }
            Self::ADD_FARSIGHT => {
                4
            }
            Self::SUMMON_POSSESSED => {
                4
            }
            Self::SUMMON_TOTEM => {
                4
            }
            Self::HEAL_MECHANICAL => {
                4
            }
            Self::SUMMON_OBJECT_WILD => {
                4
            }
            Self::SCRIPT_EFFECT => {
                4
            }
            Self::ATTACK => {
                4
            }
            Self::SANCTUARY {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::ADD_COMBO_POINTS => {
                4
            }
            Self::CREATE_HOUSE => {
                4
            }
            Self::BIND_SIGHT => {
                4
            }
            Self::DUEL => {
                4
            }
            Self::STUCK => {
                4
            }
            Self::SUMMON_PLAYER => {
                4
            }
            Self::ACTIVATE_OBJECT => {
                4
            }
            Self::SUMMON_TOTEM_SLOT1 => {
                4
            }
            Self::SUMMON_TOTEM_SLOT2 => {
                4
            }
            Self::SUMMON_TOTEM_SLOT3 => {
                4
            }
            Self::SUMMON_TOTEM_SLOT4 => {
                4
            }
            Self::THREAT_ALL {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::ENCHANT_HELD_ITEM => {
                4
            }
            Self::SUMMON_PHANTASM => {
                4
            }
            Self::SELF_RESURRECT => {
                4
            }
            Self::SKINNING => {
                4
            }
            Self::CHARGE => {
                4
            }
            Self::SUMMON_CRITTER => {
                4
            }
            Self::KNOCK_BACK => {
                4
            }
            Self::DISENCHANT => {
                4
            }
            Self::INEBRIATE => {
                4
            }
            Self::FEED_PET {
                item_target_entry,
            } => {
                4
                + 4 // item_target_entry: u32
            }
            Self::DISMISS_PET {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::REPUTATION => {
                4
            }
            Self::SUMMON_OBJECT_SLOT1 => {
                4
            }
            Self::SUMMON_OBJECT_SLOT2 => {
                4
            }
            Self::SUMMON_OBJECT_SLOT3 => {
                4
            }
            Self::SUMMON_OBJECT_SLOT4 => {
                4
            }
            Self::DISPEL_MECHANIC {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::SUMMON_DEAD_PET => {
                4
            }
            Self::DESTROY_ALL_TOTEMS => {
                4
            }
            Self::DURABILITY_DAMAGE {
                target4,
                unknown5,
                unknown6,
            } => {
                4
                + 8 // target4: Guid
                + 4 // unknown5: u32
                + 4 // unknown6: u32
            }
            Self::SUMMON_DEMON => {
                4
            }
            Self::RESURRECT_NEW {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::ATTACK_ME {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::DURABILITY_DAMAGE_PCT => {
                4
            }
            Self::SKIN_PLAYER_CORPSE {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::SPIRIT_HEAL => {
                4
            }
            Self::SKILL => {
                4
            }
            Self::APPLY_AREA_AURA_PET => {
                4
            }
            Self::TELEPORT_GRAVEYARD => {
                4
            }
            Self::NORMALIZED_WEAPON_DMG => {
                4
            }
            Self::UNKNOWN122 => {
                4
            }
            Self::SEND_TAXI => {
                4
            }
            Self::PLAYER_PULL => {
                4
            }
            Self::MODIFY_THREAT_PERCENT {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::UNKNOWN126 {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::UNKNOWN127 => {
                4
            }
        }
    }
}

impl MaximumPossibleSized for SpellLogSpellEffect {
    fn maximum_possible_size() -> usize {
        24
    }
}

