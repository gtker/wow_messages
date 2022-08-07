use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::SpellEffect;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelllogexecute.wowm:134`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelllogexecute.wowm#L134):
/// ```text
/// struct SpellLog {
///     SpellEffect effect;
///     u32 amount_of_logs = 1;
///     if (effect == POWER_DRAIN) {
///         Guid target1;
///         u32 unknown1;
///         u32 unknown2;
///         f32 unknown3;
///     }
///     else if (effect == ADD_EXTRA_ATTACKS) {
///         Guid target2;
///         u32 unknown4;
///     }
///     else if (effect == INTERRUPT_CAST) {
///         Guid target3;
///         u32 interrupted_spell;
///     }
///     else if (effect == DURABILITY_DAMAGE) {
///         Guid target4;
///         u32 unknown5;
///         u32 unknown6;
///     }
///     else if (effect == CREATE_ITEM) {
///         u32 spell_effect_item_type;
///     }
///     else if (effect == FEED_PET) {
///         u32 item_target_entry;
///     }
///     else if (effect == RESURRECT
///         || effect == DISPEL
///         || effect == THREAT
///         || effect == DISTRACT
///         || effect == SANCTUARY
///         || effect == THREAT_ALL
///         || effect == DISPEL_MECHANIC
///         || effect == RESURRECT_NEW
///         || effect == ATTACK_ME
///         || effect == SKIN_PLAYER_CORPSE
///         || effect == MODIFY_THREAT_PERCENT
///         || effect == UNKNOWN126
///         || effect == DISMISS_PET
///         || effect == OPEN_LOCK
///         || effect == OPEN_LOCK_ITEM
///         || effect == INSTAKILL) {
///         Guid target5;
///     }
/// }
/// ```
pub struct SpellLog {
    pub effect: SpellLog_SpellEffect,
}

impl SpellLog {
    /// The field `amount_of_logs` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `1` |
    /// | Hex | `0x01` |
    /// | Original | `1` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const AMOUNT_OF_LOGS_VALUE: u32 = 0x01;

}

impl SpellLog {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // effect: SpellEffect
        w.write_all(&(self.effect.as_int() as u32).to_le_bytes())?;

        // amount_of_logs: u32
        w.write_all(&Self::AMOUNT_OF_LOGS_VALUE.to_le_bytes())?;

        match &self.effect {
            SpellLog_SpellEffect::None => {}
            SpellLog_SpellEffect::Instakill {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SpellLog_SpellEffect::SchoolDamage => {}
            SpellLog_SpellEffect::Dummy => {}
            SpellLog_SpellEffect::PortalTeleport => {}
            SpellLog_SpellEffect::TeleportUnits => {}
            SpellLog_SpellEffect::ApplyAura => {}
            SpellLog_SpellEffect::EnvironmentalDamage => {}
            SpellLog_SpellEffect::PowerDrain {
                target1,
                unknown1,
                unknown2,
                unknown3,
            } => {
                // target1: Guid
                w.write_all(&target1.guid().to_le_bytes())?;

                // unknown1: u32
                w.write_all(&unknown1.to_le_bytes())?;

                // unknown2: u32
                w.write_all(&unknown2.to_le_bytes())?;

                // unknown3: f32
                w.write_all(&unknown3.to_le_bytes())?;

            }
            SpellLog_SpellEffect::HealthLeech => {}
            SpellLog_SpellEffect::Heal => {}
            SpellLog_SpellEffect::Bind => {}
            SpellLog_SpellEffect::Portal => {}
            SpellLog_SpellEffect::RitualBase => {}
            SpellLog_SpellEffect::RitualSpecialize => {}
            SpellLog_SpellEffect::RitualActivatePortal => {}
            SpellLog_SpellEffect::QuestComplete => {}
            SpellLog_SpellEffect::WeaponDamageNoschool => {}
            SpellLog_SpellEffect::Resurrect {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SpellLog_SpellEffect::AddExtraAttacks {
                target2,
                unknown4,
            } => {
                // target2: Guid
                w.write_all(&target2.guid().to_le_bytes())?;

                // unknown4: u32
                w.write_all(&unknown4.to_le_bytes())?;

            }
            SpellLog_SpellEffect::Dodge => {}
            SpellLog_SpellEffect::Evade => {}
            SpellLog_SpellEffect::Parry => {}
            SpellLog_SpellEffect::Block => {}
            SpellLog_SpellEffect::CreateItem {
                spell_effect_item_type,
            } => {
                // spell_effect_item_type: u32
                w.write_all(&spell_effect_item_type.to_le_bytes())?;

            }
            SpellLog_SpellEffect::Weapon => {}
            SpellLog_SpellEffect::Defense => {}
            SpellLog_SpellEffect::PersistentAreaAura => {}
            SpellLog_SpellEffect::Summon => {}
            SpellLog_SpellEffect::Leap => {}
            SpellLog_SpellEffect::Energize => {}
            SpellLog_SpellEffect::WeaponPercentDamage => {}
            SpellLog_SpellEffect::TriggerMissile => {}
            SpellLog_SpellEffect::OpenLock {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SpellLog_SpellEffect::SummonChangeItem => {}
            SpellLog_SpellEffect::ApplyAreaAuraParty => {}
            SpellLog_SpellEffect::LearnSpell => {}
            SpellLog_SpellEffect::SpellDefense => {}
            SpellLog_SpellEffect::Dispel {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SpellLog_SpellEffect::Language => {}
            SpellLog_SpellEffect::DualWield => {}
            SpellLog_SpellEffect::SummonWild => {}
            SpellLog_SpellEffect::SummonGuardian => {}
            SpellLog_SpellEffect::TeleportUnitsFaceCaster => {}
            SpellLog_SpellEffect::SkillStep => {}
            SpellLog_SpellEffect::AddHonor => {}
            SpellLog_SpellEffect::Spawn => {}
            SpellLog_SpellEffect::TradeSkill => {}
            SpellLog_SpellEffect::Stealth => {}
            SpellLog_SpellEffect::Detect => {}
            SpellLog_SpellEffect::TransDoor => {}
            SpellLog_SpellEffect::ForceCriticalHit => {}
            SpellLog_SpellEffect::GuaranteeHit => {}
            SpellLog_SpellEffect::EnchantItem => {}
            SpellLog_SpellEffect::EnchantItemTemporary => {}
            SpellLog_SpellEffect::Tamecreature => {}
            SpellLog_SpellEffect::SummonPet => {}
            SpellLog_SpellEffect::LearnPetSpell => {}
            SpellLog_SpellEffect::WeaponDamage => {}
            SpellLog_SpellEffect::OpenLockItem {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SpellLog_SpellEffect::Proficiency => {}
            SpellLog_SpellEffect::SendEvent => {}
            SpellLog_SpellEffect::PowerBurn => {}
            SpellLog_SpellEffect::Threat {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SpellLog_SpellEffect::TriggerSpell => {}
            SpellLog_SpellEffect::HealthFunnel => {}
            SpellLog_SpellEffect::PowerFunnel => {}
            SpellLog_SpellEffect::HealMaxHealth => {}
            SpellLog_SpellEffect::InterruptCast {
                interrupted_spell,
                target3,
            } => {
                // target3: Guid
                w.write_all(&target3.guid().to_le_bytes())?;

                // interrupted_spell: u32
                w.write_all(&interrupted_spell.to_le_bytes())?;

            }
            SpellLog_SpellEffect::Distract {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SpellLog_SpellEffect::Pull => {}
            SpellLog_SpellEffect::Pickpocket => {}
            SpellLog_SpellEffect::AddFarsight => {}
            SpellLog_SpellEffect::SummonPossessed => {}
            SpellLog_SpellEffect::SummonTotem => {}
            SpellLog_SpellEffect::HealMechanical => {}
            SpellLog_SpellEffect::SummonObjectWild => {}
            SpellLog_SpellEffect::ScriptEffect => {}
            SpellLog_SpellEffect::Attack => {}
            SpellLog_SpellEffect::Sanctuary {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SpellLog_SpellEffect::AddComboPoints => {}
            SpellLog_SpellEffect::CreateHouse => {}
            SpellLog_SpellEffect::BindSight => {}
            SpellLog_SpellEffect::Duel => {}
            SpellLog_SpellEffect::Stuck => {}
            SpellLog_SpellEffect::SummonPlayer => {}
            SpellLog_SpellEffect::ActivateObject => {}
            SpellLog_SpellEffect::SummonTotemSlot1 => {}
            SpellLog_SpellEffect::SummonTotemSlot2 => {}
            SpellLog_SpellEffect::SummonTotemSlot3 => {}
            SpellLog_SpellEffect::SummonTotemSlot4 => {}
            SpellLog_SpellEffect::ThreatAll {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SpellLog_SpellEffect::EnchantHeldItem => {}
            SpellLog_SpellEffect::SummonPhantasm => {}
            SpellLog_SpellEffect::SelfResurrect => {}
            SpellLog_SpellEffect::Skinning => {}
            SpellLog_SpellEffect::Charge => {}
            SpellLog_SpellEffect::SummonCritter => {}
            SpellLog_SpellEffect::KnockBack => {}
            SpellLog_SpellEffect::Disenchant => {}
            SpellLog_SpellEffect::Inebriate => {}
            SpellLog_SpellEffect::FeedPet {
                item_target_entry,
            } => {
                // item_target_entry: u32
                w.write_all(&item_target_entry.to_le_bytes())?;

            }
            SpellLog_SpellEffect::DismissPet {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SpellLog_SpellEffect::Reputation => {}
            SpellLog_SpellEffect::SummonObjectSlot1 => {}
            SpellLog_SpellEffect::SummonObjectSlot2 => {}
            SpellLog_SpellEffect::SummonObjectSlot3 => {}
            SpellLog_SpellEffect::SummonObjectSlot4 => {}
            SpellLog_SpellEffect::DispelMechanic {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SpellLog_SpellEffect::SummonDeadPet => {}
            SpellLog_SpellEffect::DestroyAllTotems => {}
            SpellLog_SpellEffect::DurabilityDamage {
                target4,
                unknown5,
                unknown6,
            } => {
                // target4: Guid
                w.write_all(&target4.guid().to_le_bytes())?;

                // unknown5: u32
                w.write_all(&unknown5.to_le_bytes())?;

                // unknown6: u32
                w.write_all(&unknown6.to_le_bytes())?;

            }
            SpellLog_SpellEffect::SummonDemon => {}
            SpellLog_SpellEffect::ResurrectNew {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SpellLog_SpellEffect::AttackMe {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SpellLog_SpellEffect::DurabilityDamagePct => {}
            SpellLog_SpellEffect::SkinPlayerCorpse {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SpellLog_SpellEffect::SpiritHeal => {}
            SpellLog_SpellEffect::Skill => {}
            SpellLog_SpellEffect::ApplyAreaAuraPet => {}
            SpellLog_SpellEffect::TeleportGraveyard => {}
            SpellLog_SpellEffect::NormalizedWeaponDmg => {}
            SpellLog_SpellEffect::Unknown122 => {}
            SpellLog_SpellEffect::SendTaxi => {}
            SpellLog_SpellEffect::PlayerPull => {}
            SpellLog_SpellEffect::ModifyThreatPercent {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SpellLog_SpellEffect::Unknown126 {
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

            }
            SpellLog_SpellEffect::Unknown127 => {}
        }

        Ok(())
    }
}

impl SpellLog {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // effect: SpellEffect
        let effect: SpellEffect = crate::util::read_u32_le(r)?.try_into()?;

        // amount_of_logs: u32
        let _amount_of_logs = crate::util::read_u32_le(r)?;
        // amount_of_logs is expected to always be 1 (1)

        let effect_if = match effect {
            SpellEffect::None => SpellLog_SpellEffect::None,
            SpellEffect::Instakill => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLog_SpellEffect::Instakill {
                    target5,
                }
            }
            SpellEffect::SchoolDamage => SpellLog_SpellEffect::SchoolDamage,
            SpellEffect::Dummy => SpellLog_SpellEffect::Dummy,
            SpellEffect::PortalTeleport => SpellLog_SpellEffect::PortalTeleport,
            SpellEffect::TeleportUnits => SpellLog_SpellEffect::TeleportUnits,
            SpellEffect::ApplyAura => SpellLog_SpellEffect::ApplyAura,
            SpellEffect::EnvironmentalDamage => SpellLog_SpellEffect::EnvironmentalDamage,
            SpellEffect::PowerDrain => {
                // target1: Guid
                let target1 = Guid::read(r)?;

                // unknown1: u32
                let unknown1 = crate::util::read_u32_le(r)?;

                // unknown2: u32
                let unknown2 = crate::util::read_u32_le(r)?;

                // unknown3: f32
                let unknown3 = crate::util::read_f32_le(r)?;
                SpellLog_SpellEffect::PowerDrain {
                    target1,
                    unknown1,
                    unknown2,
                    unknown3,
                }
            }
            SpellEffect::HealthLeech => SpellLog_SpellEffect::HealthLeech,
            SpellEffect::Heal => SpellLog_SpellEffect::Heal,
            SpellEffect::Bind => SpellLog_SpellEffect::Bind,
            SpellEffect::Portal => SpellLog_SpellEffect::Portal,
            SpellEffect::RitualBase => SpellLog_SpellEffect::RitualBase,
            SpellEffect::RitualSpecialize => SpellLog_SpellEffect::RitualSpecialize,
            SpellEffect::RitualActivatePortal => SpellLog_SpellEffect::RitualActivatePortal,
            SpellEffect::QuestComplete => SpellLog_SpellEffect::QuestComplete,
            SpellEffect::WeaponDamageNoschool => SpellLog_SpellEffect::WeaponDamageNoschool,
            SpellEffect::Resurrect => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLog_SpellEffect::Resurrect {
                    target5,
                }
            }
            SpellEffect::AddExtraAttacks => {
                // target2: Guid
                let target2 = Guid::read(r)?;

                // unknown4: u32
                let unknown4 = crate::util::read_u32_le(r)?;

                SpellLog_SpellEffect::AddExtraAttacks {
                    target2,
                    unknown4,
                }
            }
            SpellEffect::Dodge => SpellLog_SpellEffect::Dodge,
            SpellEffect::Evade => SpellLog_SpellEffect::Evade,
            SpellEffect::Parry => SpellLog_SpellEffect::Parry,
            SpellEffect::Block => SpellLog_SpellEffect::Block,
            SpellEffect::CreateItem => {
                // spell_effect_item_type: u32
                let spell_effect_item_type = crate::util::read_u32_le(r)?;

                SpellLog_SpellEffect::CreateItem {
                    spell_effect_item_type,
                }
            }
            SpellEffect::Weapon => SpellLog_SpellEffect::Weapon,
            SpellEffect::Defense => SpellLog_SpellEffect::Defense,
            SpellEffect::PersistentAreaAura => SpellLog_SpellEffect::PersistentAreaAura,
            SpellEffect::Summon => SpellLog_SpellEffect::Summon,
            SpellEffect::Leap => SpellLog_SpellEffect::Leap,
            SpellEffect::Energize => SpellLog_SpellEffect::Energize,
            SpellEffect::WeaponPercentDamage => SpellLog_SpellEffect::WeaponPercentDamage,
            SpellEffect::TriggerMissile => SpellLog_SpellEffect::TriggerMissile,
            SpellEffect::OpenLock => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLog_SpellEffect::OpenLock {
                    target5,
                }
            }
            SpellEffect::SummonChangeItem => SpellLog_SpellEffect::SummonChangeItem,
            SpellEffect::ApplyAreaAuraParty => SpellLog_SpellEffect::ApplyAreaAuraParty,
            SpellEffect::LearnSpell => SpellLog_SpellEffect::LearnSpell,
            SpellEffect::SpellDefense => SpellLog_SpellEffect::SpellDefense,
            SpellEffect::Dispel => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLog_SpellEffect::Dispel {
                    target5,
                }
            }
            SpellEffect::Language => SpellLog_SpellEffect::Language,
            SpellEffect::DualWield => SpellLog_SpellEffect::DualWield,
            SpellEffect::SummonWild => SpellLog_SpellEffect::SummonWild,
            SpellEffect::SummonGuardian => SpellLog_SpellEffect::SummonGuardian,
            SpellEffect::TeleportUnitsFaceCaster => SpellLog_SpellEffect::TeleportUnitsFaceCaster,
            SpellEffect::SkillStep => SpellLog_SpellEffect::SkillStep,
            SpellEffect::AddHonor => SpellLog_SpellEffect::AddHonor,
            SpellEffect::Spawn => SpellLog_SpellEffect::Spawn,
            SpellEffect::TradeSkill => SpellLog_SpellEffect::TradeSkill,
            SpellEffect::Stealth => SpellLog_SpellEffect::Stealth,
            SpellEffect::Detect => SpellLog_SpellEffect::Detect,
            SpellEffect::TransDoor => SpellLog_SpellEffect::TransDoor,
            SpellEffect::ForceCriticalHit => SpellLog_SpellEffect::ForceCriticalHit,
            SpellEffect::GuaranteeHit => SpellLog_SpellEffect::GuaranteeHit,
            SpellEffect::EnchantItem => SpellLog_SpellEffect::EnchantItem,
            SpellEffect::EnchantItemTemporary => SpellLog_SpellEffect::EnchantItemTemporary,
            SpellEffect::Tamecreature => SpellLog_SpellEffect::Tamecreature,
            SpellEffect::SummonPet => SpellLog_SpellEffect::SummonPet,
            SpellEffect::LearnPetSpell => SpellLog_SpellEffect::LearnPetSpell,
            SpellEffect::WeaponDamage => SpellLog_SpellEffect::WeaponDamage,
            SpellEffect::OpenLockItem => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLog_SpellEffect::OpenLockItem {
                    target5,
                }
            }
            SpellEffect::Proficiency => SpellLog_SpellEffect::Proficiency,
            SpellEffect::SendEvent => SpellLog_SpellEffect::SendEvent,
            SpellEffect::PowerBurn => SpellLog_SpellEffect::PowerBurn,
            SpellEffect::Threat => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLog_SpellEffect::Threat {
                    target5,
                }
            }
            SpellEffect::TriggerSpell => SpellLog_SpellEffect::TriggerSpell,
            SpellEffect::HealthFunnel => SpellLog_SpellEffect::HealthFunnel,
            SpellEffect::PowerFunnel => SpellLog_SpellEffect::PowerFunnel,
            SpellEffect::HealMaxHealth => SpellLog_SpellEffect::HealMaxHealth,
            SpellEffect::InterruptCast => {
                // target3: Guid
                let target3 = Guid::read(r)?;

                // interrupted_spell: u32
                let interrupted_spell = crate::util::read_u32_le(r)?;

                SpellLog_SpellEffect::InterruptCast {
                    interrupted_spell,
                    target3,
                }
            }
            SpellEffect::Distract => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLog_SpellEffect::Distract {
                    target5,
                }
            }
            SpellEffect::Pull => SpellLog_SpellEffect::Pull,
            SpellEffect::Pickpocket => SpellLog_SpellEffect::Pickpocket,
            SpellEffect::AddFarsight => SpellLog_SpellEffect::AddFarsight,
            SpellEffect::SummonPossessed => SpellLog_SpellEffect::SummonPossessed,
            SpellEffect::SummonTotem => SpellLog_SpellEffect::SummonTotem,
            SpellEffect::HealMechanical => SpellLog_SpellEffect::HealMechanical,
            SpellEffect::SummonObjectWild => SpellLog_SpellEffect::SummonObjectWild,
            SpellEffect::ScriptEffect => SpellLog_SpellEffect::ScriptEffect,
            SpellEffect::Attack => SpellLog_SpellEffect::Attack,
            SpellEffect::Sanctuary => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLog_SpellEffect::Sanctuary {
                    target5,
                }
            }
            SpellEffect::AddComboPoints => SpellLog_SpellEffect::AddComboPoints,
            SpellEffect::CreateHouse => SpellLog_SpellEffect::CreateHouse,
            SpellEffect::BindSight => SpellLog_SpellEffect::BindSight,
            SpellEffect::Duel => SpellLog_SpellEffect::Duel,
            SpellEffect::Stuck => SpellLog_SpellEffect::Stuck,
            SpellEffect::SummonPlayer => SpellLog_SpellEffect::SummonPlayer,
            SpellEffect::ActivateObject => SpellLog_SpellEffect::ActivateObject,
            SpellEffect::SummonTotemSlot1 => SpellLog_SpellEffect::SummonTotemSlot1,
            SpellEffect::SummonTotemSlot2 => SpellLog_SpellEffect::SummonTotemSlot2,
            SpellEffect::SummonTotemSlot3 => SpellLog_SpellEffect::SummonTotemSlot3,
            SpellEffect::SummonTotemSlot4 => SpellLog_SpellEffect::SummonTotemSlot4,
            SpellEffect::ThreatAll => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLog_SpellEffect::ThreatAll {
                    target5,
                }
            }
            SpellEffect::EnchantHeldItem => SpellLog_SpellEffect::EnchantHeldItem,
            SpellEffect::SummonPhantasm => SpellLog_SpellEffect::SummonPhantasm,
            SpellEffect::SelfResurrect => SpellLog_SpellEffect::SelfResurrect,
            SpellEffect::Skinning => SpellLog_SpellEffect::Skinning,
            SpellEffect::Charge => SpellLog_SpellEffect::Charge,
            SpellEffect::SummonCritter => SpellLog_SpellEffect::SummonCritter,
            SpellEffect::KnockBack => SpellLog_SpellEffect::KnockBack,
            SpellEffect::Disenchant => SpellLog_SpellEffect::Disenchant,
            SpellEffect::Inebriate => SpellLog_SpellEffect::Inebriate,
            SpellEffect::FeedPet => {
                // item_target_entry: u32
                let item_target_entry = crate::util::read_u32_le(r)?;

                SpellLog_SpellEffect::FeedPet {
                    item_target_entry,
                }
            }
            SpellEffect::DismissPet => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLog_SpellEffect::DismissPet {
                    target5,
                }
            }
            SpellEffect::Reputation => SpellLog_SpellEffect::Reputation,
            SpellEffect::SummonObjectSlot1 => SpellLog_SpellEffect::SummonObjectSlot1,
            SpellEffect::SummonObjectSlot2 => SpellLog_SpellEffect::SummonObjectSlot2,
            SpellEffect::SummonObjectSlot3 => SpellLog_SpellEffect::SummonObjectSlot3,
            SpellEffect::SummonObjectSlot4 => SpellLog_SpellEffect::SummonObjectSlot4,
            SpellEffect::DispelMechanic => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLog_SpellEffect::DispelMechanic {
                    target5,
                }
            }
            SpellEffect::SummonDeadPet => SpellLog_SpellEffect::SummonDeadPet,
            SpellEffect::DestroyAllTotems => SpellLog_SpellEffect::DestroyAllTotems,
            SpellEffect::DurabilityDamage => {
                // target4: Guid
                let target4 = Guid::read(r)?;

                // unknown5: u32
                let unknown5 = crate::util::read_u32_le(r)?;

                // unknown6: u32
                let unknown6 = crate::util::read_u32_le(r)?;

                SpellLog_SpellEffect::DurabilityDamage {
                    target4,
                    unknown5,
                    unknown6,
                }
            }
            SpellEffect::SummonDemon => SpellLog_SpellEffect::SummonDemon,
            SpellEffect::ResurrectNew => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLog_SpellEffect::ResurrectNew {
                    target5,
                }
            }
            SpellEffect::AttackMe => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLog_SpellEffect::AttackMe {
                    target5,
                }
            }
            SpellEffect::DurabilityDamagePct => SpellLog_SpellEffect::DurabilityDamagePct,
            SpellEffect::SkinPlayerCorpse => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLog_SpellEffect::SkinPlayerCorpse {
                    target5,
                }
            }
            SpellEffect::SpiritHeal => SpellLog_SpellEffect::SpiritHeal,
            SpellEffect::Skill => SpellLog_SpellEffect::Skill,
            SpellEffect::ApplyAreaAuraPet => SpellLog_SpellEffect::ApplyAreaAuraPet,
            SpellEffect::TeleportGraveyard => SpellLog_SpellEffect::TeleportGraveyard,
            SpellEffect::NormalizedWeaponDmg => SpellLog_SpellEffect::NormalizedWeaponDmg,
            SpellEffect::Unknown122 => SpellLog_SpellEffect::Unknown122,
            SpellEffect::SendTaxi => SpellLog_SpellEffect::SendTaxi,
            SpellEffect::PlayerPull => SpellLog_SpellEffect::PlayerPull,
            SpellEffect::ModifyThreatPercent => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLog_SpellEffect::ModifyThreatPercent {
                    target5,
                }
            }
            SpellEffect::Unknown126 => {
                // target5: Guid
                let target5 = Guid::read(r)?;

                SpellLog_SpellEffect::Unknown126 {
                    target5,
                }
            }
            SpellEffect::Unknown127 => SpellLog_SpellEffect::Unknown127,
        };

        Ok(Self {
            effect: effect_if,
        })
    }

}

impl SpellLog {
    pub(crate) fn size(&self) -> usize {
        self.effect.size() // effect: SpellLog_SpellEffect
        + 4 // amount_of_logs: u32
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SpellLog_SpellEffect {
    None,
    Instakill {
        target5: Guid,
    },
    SchoolDamage,
    Dummy,
    PortalTeleport,
    TeleportUnits,
    ApplyAura,
    EnvironmentalDamage,
    PowerDrain {
        target1: Guid,
        unknown1: u32,
        unknown2: u32,
        unknown3: f32,
    },
    HealthLeech,
    Heal,
    Bind,
    Portal,
    RitualBase,
    RitualSpecialize,
    RitualActivatePortal,
    QuestComplete,
    WeaponDamageNoschool,
    Resurrect {
        target5: Guid,
    },
    AddExtraAttacks {
        target2: Guid,
        unknown4: u32,
    },
    Dodge,
    Evade,
    Parry,
    Block,
    CreateItem {
        spell_effect_item_type: u32,
    },
    Weapon,
    Defense,
    PersistentAreaAura,
    Summon,
    Leap,
    Energize,
    WeaponPercentDamage,
    TriggerMissile,
    OpenLock {
        target5: Guid,
    },
    SummonChangeItem,
    ApplyAreaAuraParty,
    LearnSpell,
    SpellDefense,
    Dispel {
        target5: Guid,
    },
    Language,
    DualWield,
    SummonWild,
    SummonGuardian,
    TeleportUnitsFaceCaster,
    SkillStep,
    AddHonor,
    Spawn,
    TradeSkill,
    Stealth,
    Detect,
    TransDoor,
    ForceCriticalHit,
    GuaranteeHit,
    EnchantItem,
    EnchantItemTemporary,
    Tamecreature,
    SummonPet,
    LearnPetSpell,
    WeaponDamage,
    OpenLockItem {
        target5: Guid,
    },
    Proficiency,
    SendEvent,
    PowerBurn,
    Threat {
        target5: Guid,
    },
    TriggerSpell,
    HealthFunnel,
    PowerFunnel,
    HealMaxHealth,
    InterruptCast {
        interrupted_spell: u32,
        target3: Guid,
    },
    Distract {
        target5: Guid,
    },
    Pull,
    Pickpocket,
    AddFarsight,
    SummonPossessed,
    SummonTotem,
    HealMechanical,
    SummonObjectWild,
    ScriptEffect,
    Attack,
    Sanctuary {
        target5: Guid,
    },
    AddComboPoints,
    CreateHouse,
    BindSight,
    Duel,
    Stuck,
    SummonPlayer,
    ActivateObject,
    SummonTotemSlot1,
    SummonTotemSlot2,
    SummonTotemSlot3,
    SummonTotemSlot4,
    ThreatAll {
        target5: Guid,
    },
    EnchantHeldItem,
    SummonPhantasm,
    SelfResurrect,
    Skinning,
    Charge,
    SummonCritter,
    KnockBack,
    Disenchant,
    Inebriate,
    FeedPet {
        item_target_entry: u32,
    },
    DismissPet {
        target5: Guid,
    },
    Reputation,
    SummonObjectSlot1,
    SummonObjectSlot2,
    SummonObjectSlot3,
    SummonObjectSlot4,
    DispelMechanic {
        target5: Guid,
    },
    SummonDeadPet,
    DestroyAllTotems,
    DurabilityDamage {
        target4: Guid,
        unknown5: u32,
        unknown6: u32,
    },
    SummonDemon,
    ResurrectNew {
        target5: Guid,
    },
    AttackMe {
        target5: Guid,
    },
    DurabilityDamagePct,
    SkinPlayerCorpse {
        target5: Guid,
    },
    SpiritHeal,
    Skill,
    ApplyAreaAuraPet,
    TeleportGraveyard,
    NormalizedWeaponDmg,
    Unknown122,
    SendTaxi,
    PlayerPull,
    ModifyThreatPercent {
        target5: Guid,
    },
    Unknown126 {
        target5: Guid,
    },
    Unknown127,
}

impl Default for SpellLog_SpellEffect {
    fn default() -> Self {
        // First enumerator without any fields
        Self::None
    }
}

impl SpellLog_SpellEffect {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Instakill { .. } => 1,
            Self::SchoolDamage => 2,
            Self::Dummy => 3,
            Self::PortalTeleport => 4,
            Self::TeleportUnits => 5,
            Self::ApplyAura => 6,
            Self::EnvironmentalDamage => 7,
            Self::PowerDrain { .. } => 8,
            Self::HealthLeech => 9,
            Self::Heal => 10,
            Self::Bind => 11,
            Self::Portal => 12,
            Self::RitualBase => 13,
            Self::RitualSpecialize => 14,
            Self::RitualActivatePortal => 15,
            Self::QuestComplete => 16,
            Self::WeaponDamageNoschool => 17,
            Self::Resurrect { .. } => 18,
            Self::AddExtraAttacks { .. } => 19,
            Self::Dodge => 20,
            Self::Evade => 21,
            Self::Parry => 22,
            Self::Block => 23,
            Self::CreateItem { .. } => 24,
            Self::Weapon => 25,
            Self::Defense => 26,
            Self::PersistentAreaAura => 27,
            Self::Summon => 28,
            Self::Leap => 29,
            Self::Energize => 30,
            Self::WeaponPercentDamage => 31,
            Self::TriggerMissile => 32,
            Self::OpenLock { .. } => 33,
            Self::SummonChangeItem => 34,
            Self::ApplyAreaAuraParty => 35,
            Self::LearnSpell => 36,
            Self::SpellDefense => 37,
            Self::Dispel { .. } => 38,
            Self::Language => 39,
            Self::DualWield => 40,
            Self::SummonWild => 41,
            Self::SummonGuardian => 42,
            Self::TeleportUnitsFaceCaster => 43,
            Self::SkillStep => 44,
            Self::AddHonor => 45,
            Self::Spawn => 46,
            Self::TradeSkill => 47,
            Self::Stealth => 48,
            Self::Detect => 49,
            Self::TransDoor => 50,
            Self::ForceCriticalHit => 51,
            Self::GuaranteeHit => 52,
            Self::EnchantItem => 53,
            Self::EnchantItemTemporary => 54,
            Self::Tamecreature => 55,
            Self::SummonPet => 56,
            Self::LearnPetSpell => 57,
            Self::WeaponDamage => 58,
            Self::OpenLockItem { .. } => 59,
            Self::Proficiency => 60,
            Self::SendEvent => 61,
            Self::PowerBurn => 62,
            Self::Threat { .. } => 63,
            Self::TriggerSpell => 64,
            Self::HealthFunnel => 65,
            Self::PowerFunnel => 66,
            Self::HealMaxHealth => 67,
            Self::InterruptCast { .. } => 68,
            Self::Distract { .. } => 69,
            Self::Pull => 70,
            Self::Pickpocket => 71,
            Self::AddFarsight => 72,
            Self::SummonPossessed => 73,
            Self::SummonTotem => 74,
            Self::HealMechanical => 75,
            Self::SummonObjectWild => 76,
            Self::ScriptEffect => 77,
            Self::Attack => 78,
            Self::Sanctuary { .. } => 79,
            Self::AddComboPoints => 80,
            Self::CreateHouse => 81,
            Self::BindSight => 82,
            Self::Duel => 83,
            Self::Stuck => 84,
            Self::SummonPlayer => 85,
            Self::ActivateObject => 86,
            Self::SummonTotemSlot1 => 87,
            Self::SummonTotemSlot2 => 88,
            Self::SummonTotemSlot3 => 89,
            Self::SummonTotemSlot4 => 90,
            Self::ThreatAll { .. } => 91,
            Self::EnchantHeldItem => 92,
            Self::SummonPhantasm => 93,
            Self::SelfResurrect => 94,
            Self::Skinning => 95,
            Self::Charge => 96,
            Self::SummonCritter => 97,
            Self::KnockBack => 98,
            Self::Disenchant => 99,
            Self::Inebriate => 100,
            Self::FeedPet { .. } => 101,
            Self::DismissPet { .. } => 102,
            Self::Reputation => 103,
            Self::SummonObjectSlot1 => 104,
            Self::SummonObjectSlot2 => 105,
            Self::SummonObjectSlot3 => 106,
            Self::SummonObjectSlot4 => 107,
            Self::DispelMechanic { .. } => 108,
            Self::SummonDeadPet => 109,
            Self::DestroyAllTotems => 110,
            Self::DurabilityDamage { .. } => 111,
            Self::SummonDemon => 112,
            Self::ResurrectNew { .. } => 113,
            Self::AttackMe { .. } => 114,
            Self::DurabilityDamagePct => 115,
            Self::SkinPlayerCorpse { .. } => 116,
            Self::SpiritHeal => 117,
            Self::Skill => 118,
            Self::ApplyAreaAuraPet => 119,
            Self::TeleportGraveyard => 120,
            Self::NormalizedWeaponDmg => 121,
            Self::Unknown122 => 122,
            Self::SendTaxi => 123,
            Self::PlayerPull => 124,
            Self::ModifyThreatPercent { .. } => 125,
            Self::Unknown126 { .. } => 126,
            Self::Unknown127 => 127,
        }
    }

}

impl SpellLog_SpellEffect {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::None => {
                4
            }
            Self::Instakill {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::SchoolDamage => {
                4
            }
            Self::Dummy => {
                4
            }
            Self::PortalTeleport => {
                4
            }
            Self::TeleportUnits => {
                4
            }
            Self::ApplyAura => {
                4
            }
            Self::EnvironmentalDamage => {
                4
            }
            Self::PowerDrain {
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
            Self::HealthLeech => {
                4
            }
            Self::Heal => {
                4
            }
            Self::Bind => {
                4
            }
            Self::Portal => {
                4
            }
            Self::RitualBase => {
                4
            }
            Self::RitualSpecialize => {
                4
            }
            Self::RitualActivatePortal => {
                4
            }
            Self::QuestComplete => {
                4
            }
            Self::WeaponDamageNoschool => {
                4
            }
            Self::Resurrect {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::AddExtraAttacks {
                target2,
                unknown4,
            } => {
                4
                + 8 // target2: Guid
                + 4 // unknown4: u32
            }
            Self::Dodge => {
                4
            }
            Self::Evade => {
                4
            }
            Self::Parry => {
                4
            }
            Self::Block => {
                4
            }
            Self::CreateItem {
                spell_effect_item_type,
            } => {
                4
                + 4 // spell_effect_item_type: u32
            }
            Self::Weapon => {
                4
            }
            Self::Defense => {
                4
            }
            Self::PersistentAreaAura => {
                4
            }
            Self::Summon => {
                4
            }
            Self::Leap => {
                4
            }
            Self::Energize => {
                4
            }
            Self::WeaponPercentDamage => {
                4
            }
            Self::TriggerMissile => {
                4
            }
            Self::OpenLock {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::SummonChangeItem => {
                4
            }
            Self::ApplyAreaAuraParty => {
                4
            }
            Self::LearnSpell => {
                4
            }
            Self::SpellDefense => {
                4
            }
            Self::Dispel {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::Language => {
                4
            }
            Self::DualWield => {
                4
            }
            Self::SummonWild => {
                4
            }
            Self::SummonGuardian => {
                4
            }
            Self::TeleportUnitsFaceCaster => {
                4
            }
            Self::SkillStep => {
                4
            }
            Self::AddHonor => {
                4
            }
            Self::Spawn => {
                4
            }
            Self::TradeSkill => {
                4
            }
            Self::Stealth => {
                4
            }
            Self::Detect => {
                4
            }
            Self::TransDoor => {
                4
            }
            Self::ForceCriticalHit => {
                4
            }
            Self::GuaranteeHit => {
                4
            }
            Self::EnchantItem => {
                4
            }
            Self::EnchantItemTemporary => {
                4
            }
            Self::Tamecreature => {
                4
            }
            Self::SummonPet => {
                4
            }
            Self::LearnPetSpell => {
                4
            }
            Self::WeaponDamage => {
                4
            }
            Self::OpenLockItem {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::Proficiency => {
                4
            }
            Self::SendEvent => {
                4
            }
            Self::PowerBurn => {
                4
            }
            Self::Threat {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::TriggerSpell => {
                4
            }
            Self::HealthFunnel => {
                4
            }
            Self::PowerFunnel => {
                4
            }
            Self::HealMaxHealth => {
                4
            }
            Self::InterruptCast {
                interrupted_spell,
                target3,
            } => {
                4
                + 4 // interrupted_spell: u32
                + 8 // target3: Guid
            }
            Self::Distract {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::Pull => {
                4
            }
            Self::Pickpocket => {
                4
            }
            Self::AddFarsight => {
                4
            }
            Self::SummonPossessed => {
                4
            }
            Self::SummonTotem => {
                4
            }
            Self::HealMechanical => {
                4
            }
            Self::SummonObjectWild => {
                4
            }
            Self::ScriptEffect => {
                4
            }
            Self::Attack => {
                4
            }
            Self::Sanctuary {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::AddComboPoints => {
                4
            }
            Self::CreateHouse => {
                4
            }
            Self::BindSight => {
                4
            }
            Self::Duel => {
                4
            }
            Self::Stuck => {
                4
            }
            Self::SummonPlayer => {
                4
            }
            Self::ActivateObject => {
                4
            }
            Self::SummonTotemSlot1 => {
                4
            }
            Self::SummonTotemSlot2 => {
                4
            }
            Self::SummonTotemSlot3 => {
                4
            }
            Self::SummonTotemSlot4 => {
                4
            }
            Self::ThreatAll {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::EnchantHeldItem => {
                4
            }
            Self::SummonPhantasm => {
                4
            }
            Self::SelfResurrect => {
                4
            }
            Self::Skinning => {
                4
            }
            Self::Charge => {
                4
            }
            Self::SummonCritter => {
                4
            }
            Self::KnockBack => {
                4
            }
            Self::Disenchant => {
                4
            }
            Self::Inebriate => {
                4
            }
            Self::FeedPet {
                item_target_entry,
            } => {
                4
                + 4 // item_target_entry: u32
            }
            Self::DismissPet {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::Reputation => {
                4
            }
            Self::SummonObjectSlot1 => {
                4
            }
            Self::SummonObjectSlot2 => {
                4
            }
            Self::SummonObjectSlot3 => {
                4
            }
            Self::SummonObjectSlot4 => {
                4
            }
            Self::DispelMechanic {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::SummonDeadPet => {
                4
            }
            Self::DestroyAllTotems => {
                4
            }
            Self::DurabilityDamage {
                target4,
                unknown5,
                unknown6,
            } => {
                4
                + 8 // target4: Guid
                + 4 // unknown5: u32
                + 4 // unknown6: u32
            }
            Self::SummonDemon => {
                4
            }
            Self::ResurrectNew {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::AttackMe {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::DurabilityDamagePct => {
                4
            }
            Self::SkinPlayerCorpse {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::SpiritHeal => {
                4
            }
            Self::Skill => {
                4
            }
            Self::ApplyAreaAuraPet => {
                4
            }
            Self::TeleportGraveyard => {
                4
            }
            Self::NormalizedWeaponDmg => {
                4
            }
            Self::Unknown122 => {
                4
            }
            Self::SendTaxi => {
                4
            }
            Self::PlayerPull => {
                4
            }
            Self::ModifyThreatPercent {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::Unknown126 {
                target5,
            } => {
                4
                + 8 // target5: Guid
            }
            Self::Unknown127 => {
                4
            }
        }
    }
}

