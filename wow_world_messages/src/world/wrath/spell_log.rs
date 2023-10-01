use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    Power, SpellEffect,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelllogexecute.wowm:602`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelllogexecute.wowm#L602):
/// ```text
/// struct SpellLog {
///     SpellEffect effect;
///     u32 amount_of_logs = 1;
///     if (effect == POWER_DRAIN) {
///         PackedGuid target1;
///         u32 amount;
///         (u32)Power power;
///         f32 multiplier;
///     }
///     else if (effect == ADD_EXTRA_ATTACKS) {
///         PackedGuid target4;
///         u32 extra_attacks;
///     }
///     else if (effect == INTERRUPT_CAST) {
///         PackedGuid target5;
///         Spell interrupted_spell;
///     }
///     else if (effect == DURABILITY_DAMAGE) {
///         PackedGuid target6;
///         Item item_to_damage;
///         u32 unknown5;
///     }
///     else if (effect == OPEN_LOCK
///         || effect == OPEN_LOCK_ITEM) {
///         PackedGuid lock_target;
///     }
///     else if (effect == CREATE_ITEM
///         || effect == CREATE_ITEM2) {
///         Item item;
///     }
///     else if (effect == SUMMON
///         || effect == TRANS_DOOR
///         || effect == SUMMON_PET
///         || effect == SUMMON_OBJECT_WILD
///         || effect == CREATE_HOUSE
///         || effect == DUEL
///         || effect == SUMMON_OBJECT_SLOT1
///         || effect == SUMMON_OBJECT_SLOT2
///         || effect == SUMMON_OBJECT_SLOT3
///         || effect == SUMMON_OBJECT_SLOT4) {
///         PackedGuid summon_target;
///     }
///     else if (effect == FEED_PET) {
///         PackedGuid pet_feed_guid;
///     }
///     else if (effect == DISMISS_PET) {
///         PackedGuid pet_dismiss_guid;
///     }
///     else if (effect == RESURRECT
///         || effect == RESURRECT_NEW) {
///         PackedGuid resurrect_guid;
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
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // effect: SpellEffect
        w.write_all(&(self.effect.as_int().to_le_bytes()))?;

        // amount_of_logs: u32
        w.write_all(&Self::AMOUNT_OF_LOGS_VALUE.to_le_bytes())?;

        match &self.effect {
            SpellLog_SpellEffect::PowerDrain {
                amount,
                multiplier,
                power,
                target1,
            } => {
                // target1: PackedGuid
                crate::util::write_packed_guid(&target1, &mut w)?;

                // amount: u32
                w.write_all(&amount.to_le_bytes())?;

                // power: Power
                w.write_all(&u32::from(power.as_int()).to_le_bytes())?;

                // multiplier: f32
                w.write_all(&multiplier.to_le_bytes())?;

            }
            SpellLog_SpellEffect::Resurrect {
                resurrect_guid,
            } => {
                // resurrect_guid: PackedGuid
                crate::util::write_packed_guid(&resurrect_guid, &mut w)?;

            }
            SpellLog_SpellEffect::AddExtraAttacks {
                extra_attacks,
                target4,
            } => {
                // target4: PackedGuid
                crate::util::write_packed_guid(&target4, &mut w)?;

                // extra_attacks: u32
                w.write_all(&extra_attacks.to_le_bytes())?;

            }
            SpellLog_SpellEffect::CreateItem {
                item,
            } => {
                // item: Item
                w.write_all(&item.to_le_bytes())?;

            }
            SpellLog_SpellEffect::Summon {
                summon_target,
            } => {
                // summon_target: PackedGuid
                crate::util::write_packed_guid(&summon_target, &mut w)?;

            }
            SpellLog_SpellEffect::OpenLock {
                lock_target,
            } => {
                // lock_target: PackedGuid
                crate::util::write_packed_guid(&lock_target, &mut w)?;

            }
            SpellLog_SpellEffect::TransDoor {
                summon_target,
            } => {
                // summon_target: PackedGuid
                crate::util::write_packed_guid(&summon_target, &mut w)?;

            }
            SpellLog_SpellEffect::SummonPet {
                summon_target,
            } => {
                // summon_target: PackedGuid
                crate::util::write_packed_guid(&summon_target, &mut w)?;

            }
            SpellLog_SpellEffect::InterruptCast {
                interrupted_spell,
                target5,
            } => {
                // target5: PackedGuid
                crate::util::write_packed_guid(&target5, &mut w)?;

                // interrupted_spell: Spell
                w.write_all(&interrupted_spell.to_le_bytes())?;

            }
            SpellLog_SpellEffect::SummonObjectWild {
                summon_target,
            } => {
                // summon_target: PackedGuid
                crate::util::write_packed_guid(&summon_target, &mut w)?;

            }
            SpellLog_SpellEffect::CreateHouse {
                summon_target,
            } => {
                // summon_target: PackedGuid
                crate::util::write_packed_guid(&summon_target, &mut w)?;

            }
            SpellLog_SpellEffect::Duel {
                summon_target,
            } => {
                // summon_target: PackedGuid
                crate::util::write_packed_guid(&summon_target, &mut w)?;

            }
            SpellLog_SpellEffect::FeedPet {
                pet_feed_guid,
            } => {
                // pet_feed_guid: PackedGuid
                crate::util::write_packed_guid(&pet_feed_guid, &mut w)?;

            }
            SpellLog_SpellEffect::DismissPet {
                pet_dismiss_guid,
            } => {
                // pet_dismiss_guid: PackedGuid
                crate::util::write_packed_guid(&pet_dismiss_guid, &mut w)?;

            }
            SpellLog_SpellEffect::SummonObjectSlot1 {
                summon_target,
            } => {
                // summon_target: PackedGuid
                crate::util::write_packed_guid(&summon_target, &mut w)?;

            }
            SpellLog_SpellEffect::SummonObjectSlot2 {
                summon_target,
            } => {
                // summon_target: PackedGuid
                crate::util::write_packed_guid(&summon_target, &mut w)?;

            }
            SpellLog_SpellEffect::SummonObjectSlot3 {
                summon_target,
            } => {
                // summon_target: PackedGuid
                crate::util::write_packed_guid(&summon_target, &mut w)?;

            }
            SpellLog_SpellEffect::SummonObjectSlot4 {
                summon_target,
            } => {
                // summon_target: PackedGuid
                crate::util::write_packed_guid(&summon_target, &mut w)?;

            }
            SpellLog_SpellEffect::DurabilityDamage {
                item_to_damage,
                target6,
                unknown5,
            } => {
                // target6: PackedGuid
                crate::util::write_packed_guid(&target6, &mut w)?;

                // item_to_damage: Item
                w.write_all(&item_to_damage.to_le_bytes())?;

                // unknown5: u32
                w.write_all(&unknown5.to_le_bytes())?;

            }
            SpellLog_SpellEffect::ResurrectNew {
                resurrect_guid,
            } => {
                // resurrect_guid: PackedGuid
                crate::util::write_packed_guid(&resurrect_guid, &mut w)?;

            }
            _ => {}
        }

        Ok(())
    }
}

impl SpellLog {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // effect: SpellEffect
        let effect = crate::util::read_u32_le(&mut r)?.try_into()?;

        // amount_of_logs: u32
        let _amount_of_logs = crate::util::read_u32_le(&mut r)?;
        // amount_of_logs is expected to always be 1 (1)

        let effect_if = match effect {
            SpellEffect::None => SpellLog_SpellEffect::None,
            SpellEffect::Instakill => SpellLog_SpellEffect::Instakill,
            SpellEffect::SchoolDamage => SpellLog_SpellEffect::SchoolDamage,
            SpellEffect::Dummy => SpellLog_SpellEffect::Dummy,
            SpellEffect::PortalTeleport => SpellLog_SpellEffect::PortalTeleport,
            SpellEffect::TeleportUnits => SpellLog_SpellEffect::TeleportUnits,
            SpellEffect::ApplyAura => SpellLog_SpellEffect::ApplyAura,
            SpellEffect::EnvironmentalDamage => SpellLog_SpellEffect::EnvironmentalDamage,
            SpellEffect::PowerDrain => {
                // target1: PackedGuid
                let target1 = crate::util::read_packed_guid(&mut r)?;

                // amount: u32
                let amount = crate::util::read_u32_le(&mut r)?;

                // power: Power
                let power = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

                // multiplier: f32
                let multiplier = crate::util::read_f32_le(&mut r)?;

                SpellLog_SpellEffect::PowerDrain {
                    amount,
                    multiplier,
                    power,
                    target1,
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
                // resurrect_guid: PackedGuid
                let resurrect_guid = crate::util::read_packed_guid(&mut r)?;

                SpellLog_SpellEffect::Resurrect {
                    resurrect_guid,
                }
            }
            SpellEffect::AddExtraAttacks => {
                // target4: PackedGuid
                let target4 = crate::util::read_packed_guid(&mut r)?;

                // extra_attacks: u32
                let extra_attacks = crate::util::read_u32_le(&mut r)?;

                SpellLog_SpellEffect::AddExtraAttacks {
                    extra_attacks,
                    target4,
                }
            }
            SpellEffect::Dodge => SpellLog_SpellEffect::Dodge,
            SpellEffect::Evade => SpellLog_SpellEffect::Evade,
            SpellEffect::Parry => SpellLog_SpellEffect::Parry,
            SpellEffect::Block => SpellLog_SpellEffect::Block,
            SpellEffect::CreateItem => {
                // item: Item
                let item = crate::util::read_u32_le(&mut r)?;

                SpellLog_SpellEffect::CreateItem {
                    item,
                }
            }
            SpellEffect::Weapon => SpellLog_SpellEffect::Weapon,
            SpellEffect::Defense => SpellLog_SpellEffect::Defense,
            SpellEffect::PersistentAreaAura => SpellLog_SpellEffect::PersistentAreaAura,
            SpellEffect::Summon => {
                // summon_target: PackedGuid
                let summon_target = crate::util::read_packed_guid(&mut r)?;

                SpellLog_SpellEffect::Summon {
                    summon_target,
                }
            }
            SpellEffect::Leap => SpellLog_SpellEffect::Leap,
            SpellEffect::Energize => SpellLog_SpellEffect::Energize,
            SpellEffect::WeaponPercentDamage => SpellLog_SpellEffect::WeaponPercentDamage,
            SpellEffect::TriggerMissile => SpellLog_SpellEffect::TriggerMissile,
            SpellEffect::OpenLock => {
                // lock_target: PackedGuid
                let lock_target = crate::util::read_packed_guid(&mut r)?;

                SpellLog_SpellEffect::OpenLock {
                    lock_target,
                }
            }
            SpellEffect::SummonChangeItem => SpellLog_SpellEffect::SummonChangeItem,
            SpellEffect::ApplyAreaAuraParty => SpellLog_SpellEffect::ApplyAreaAuraParty,
            SpellEffect::LearnSpell => SpellLog_SpellEffect::LearnSpell,
            SpellEffect::SpellDefense => SpellLog_SpellEffect::SpellDefense,
            SpellEffect::Dispel => SpellLog_SpellEffect::Dispel,
            SpellEffect::Language => SpellLog_SpellEffect::Language,
            SpellEffect::DualWield => SpellLog_SpellEffect::DualWield,
            SpellEffect::Jump => SpellLog_SpellEffect::Jump,
            SpellEffect::Jump2 => SpellLog_SpellEffect::Jump2,
            SpellEffect::TeleportUnitsFaceCaster => SpellLog_SpellEffect::TeleportUnitsFaceCaster,
            SpellEffect::SkillStep => SpellLog_SpellEffect::SkillStep,
            SpellEffect::AddHonor => SpellLog_SpellEffect::AddHonor,
            SpellEffect::Spawn => SpellLog_SpellEffect::Spawn,
            SpellEffect::TradeSkill => SpellLog_SpellEffect::TradeSkill,
            SpellEffect::Stealth => SpellLog_SpellEffect::Stealth,
            SpellEffect::Detect => SpellLog_SpellEffect::Detect,
            SpellEffect::TransDoor => {
                // summon_target: PackedGuid
                let summon_target = crate::util::read_packed_guid(&mut r)?;

                SpellLog_SpellEffect::TransDoor {
                    summon_target,
                }
            }
            SpellEffect::ForceCriticalHit => SpellLog_SpellEffect::ForceCriticalHit,
            SpellEffect::GuaranteeHit => SpellLog_SpellEffect::GuaranteeHit,
            SpellEffect::EnchantItem => SpellLog_SpellEffect::EnchantItem,
            SpellEffect::EnchantItemTemporary => SpellLog_SpellEffect::EnchantItemTemporary,
            SpellEffect::Tamecreature => SpellLog_SpellEffect::Tamecreature,
            SpellEffect::SummonPet => {
                // summon_target: PackedGuid
                let summon_target = crate::util::read_packed_guid(&mut r)?;

                SpellLog_SpellEffect::SummonPet {
                    summon_target,
                }
            }
            SpellEffect::LearnPetSpell => SpellLog_SpellEffect::LearnPetSpell,
            SpellEffect::WeaponDamage => SpellLog_SpellEffect::WeaponDamage,
            SpellEffect::CreateRandomItem => SpellLog_SpellEffect::CreateRandomItem,
            SpellEffect::Proficiency => SpellLog_SpellEffect::Proficiency,
            SpellEffect::SendEvent => SpellLog_SpellEffect::SendEvent,
            SpellEffect::PowerBurn => SpellLog_SpellEffect::PowerBurn,
            SpellEffect::Threat => SpellLog_SpellEffect::Threat,
            SpellEffect::TriggerSpell => SpellLog_SpellEffect::TriggerSpell,
            SpellEffect::ApplyAreaAuraRaid => SpellLog_SpellEffect::ApplyAreaAuraRaid,
            SpellEffect::RestoreItemCharges => SpellLog_SpellEffect::RestoreItemCharges,
            SpellEffect::HealMaxHealth => SpellLog_SpellEffect::HealMaxHealth,
            SpellEffect::InterruptCast => {
                // target5: PackedGuid
                let target5 = crate::util::read_packed_guid(&mut r)?;

                // interrupted_spell: Spell
                let interrupted_spell = crate::util::read_u32_le(&mut r)?;

                SpellLog_SpellEffect::InterruptCast {
                    interrupted_spell,
                    target5,
                }
            }
            SpellEffect::Distract => SpellLog_SpellEffect::Distract,
            SpellEffect::Pull => SpellLog_SpellEffect::Pull,
            SpellEffect::Pickpocket => SpellLog_SpellEffect::Pickpocket,
            SpellEffect::AddFarsight => SpellLog_SpellEffect::AddFarsight,
            SpellEffect::UntrainTalents => SpellLog_SpellEffect::UntrainTalents,
            SpellEffect::ApplyGlyph => SpellLog_SpellEffect::ApplyGlyph,
            SpellEffect::HealMechanical => SpellLog_SpellEffect::HealMechanical,
            SpellEffect::SummonObjectWild => {
                // summon_target: PackedGuid
                let summon_target = crate::util::read_packed_guid(&mut r)?;

                SpellLog_SpellEffect::SummonObjectWild {
                    summon_target,
                }
            }
            SpellEffect::ScriptEffect => SpellLog_SpellEffect::ScriptEffect,
            SpellEffect::Attack => SpellLog_SpellEffect::Attack,
            SpellEffect::Sanctuary => SpellLog_SpellEffect::Sanctuary,
            SpellEffect::AddComboPoints => SpellLog_SpellEffect::AddComboPoints,
            SpellEffect::CreateHouse => {
                // summon_target: PackedGuid
                let summon_target = crate::util::read_packed_guid(&mut r)?;

                SpellLog_SpellEffect::CreateHouse {
                    summon_target,
                }
            }
            SpellEffect::BindSight => SpellLog_SpellEffect::BindSight,
            SpellEffect::Duel => {
                // summon_target: PackedGuid
                let summon_target = crate::util::read_packed_guid(&mut r)?;

                SpellLog_SpellEffect::Duel {
                    summon_target,
                }
            }
            SpellEffect::Stuck => SpellLog_SpellEffect::Stuck,
            SpellEffect::SummonPlayer => SpellLog_SpellEffect::SummonPlayer,
            SpellEffect::ActivateObject => SpellLog_SpellEffect::ActivateObject,
            SpellEffect::WmoDamage => SpellLog_SpellEffect::WmoDamage,
            SpellEffect::WmoRepair => SpellLog_SpellEffect::WmoRepair,
            SpellEffect::WmoChange => SpellLog_SpellEffect::WmoChange,
            SpellEffect::KillCreditPersonal => SpellLog_SpellEffect::KillCreditPersonal,
            SpellEffect::ThreatAll => SpellLog_SpellEffect::ThreatAll,
            SpellEffect::EnchantHeldItem => SpellLog_SpellEffect::EnchantHeldItem,
            SpellEffect::BreakPlayerTargeting => SpellLog_SpellEffect::BreakPlayerTargeting,
            SpellEffect::SelfResurrect => SpellLog_SpellEffect::SelfResurrect,
            SpellEffect::Skinning => SpellLog_SpellEffect::Skinning,
            SpellEffect::Charge => SpellLog_SpellEffect::Charge,
            SpellEffect::SummonAllTotems => SpellLog_SpellEffect::SummonAllTotems,
            SpellEffect::KnockBack => SpellLog_SpellEffect::KnockBack,
            SpellEffect::Disenchant => SpellLog_SpellEffect::Disenchant,
            SpellEffect::Inebriate => SpellLog_SpellEffect::Inebriate,
            SpellEffect::FeedPet => {
                // pet_feed_guid: PackedGuid
                let pet_feed_guid = crate::util::read_packed_guid(&mut r)?;

                SpellLog_SpellEffect::FeedPet {
                    pet_feed_guid,
                }
            }
            SpellEffect::DismissPet => {
                // pet_dismiss_guid: PackedGuid
                let pet_dismiss_guid = crate::util::read_packed_guid(&mut r)?;

                SpellLog_SpellEffect::DismissPet {
                    pet_dismiss_guid,
                }
            }
            SpellEffect::Reputation => SpellLog_SpellEffect::Reputation,
            SpellEffect::SummonObjectSlot1 => {
                // summon_target: PackedGuid
                let summon_target = crate::util::read_packed_guid(&mut r)?;

                SpellLog_SpellEffect::SummonObjectSlot1 {
                    summon_target,
                }
            }
            SpellEffect::SummonObjectSlot2 => {
                // summon_target: PackedGuid
                let summon_target = crate::util::read_packed_guid(&mut r)?;

                SpellLog_SpellEffect::SummonObjectSlot2 {
                    summon_target,
                }
            }
            SpellEffect::SummonObjectSlot3 => {
                // summon_target: PackedGuid
                let summon_target = crate::util::read_packed_guid(&mut r)?;

                SpellLog_SpellEffect::SummonObjectSlot3 {
                    summon_target,
                }
            }
            SpellEffect::SummonObjectSlot4 => {
                // summon_target: PackedGuid
                let summon_target = crate::util::read_packed_guid(&mut r)?;

                SpellLog_SpellEffect::SummonObjectSlot4 {
                    summon_target,
                }
            }
            SpellEffect::DispelMechanic => SpellLog_SpellEffect::DispelMechanic,
            SpellEffect::SummonDeadPet => SpellLog_SpellEffect::SummonDeadPet,
            SpellEffect::DestroyAllTotems => SpellLog_SpellEffect::DestroyAllTotems,
            SpellEffect::DurabilityDamage => {
                // target6: PackedGuid
                let target6 = crate::util::read_packed_guid(&mut r)?;

                // item_to_damage: Item
                let item_to_damage = crate::util::read_u32_le(&mut r)?;

                // unknown5: u32
                let unknown5 = crate::util::read_u32_le(&mut r)?;

                SpellLog_SpellEffect::DurabilityDamage {
                    item_to_damage,
                    target6,
                    unknown5,
                }
            }
            SpellEffect::Unknown112 => SpellLog_SpellEffect::Unknown112,
            SpellEffect::ResurrectNew => {
                // resurrect_guid: PackedGuid
                let resurrect_guid = crate::util::read_packed_guid(&mut r)?;

                SpellLog_SpellEffect::ResurrectNew {
                    resurrect_guid,
                }
            }
            SpellEffect::AttackMe => SpellLog_SpellEffect::AttackMe,
            SpellEffect::DurabilityDamagePct => SpellLog_SpellEffect::DurabilityDamagePct,
            SpellEffect::SkinPlayerCorpse => SpellLog_SpellEffect::SkinPlayerCorpse,
            SpellEffect::SpiritHeal => SpellLog_SpellEffect::SpiritHeal,
            SpellEffect::Skill => SpellLog_SpellEffect::Skill,
            SpellEffect::ApplyAreaAuraPet => SpellLog_SpellEffect::ApplyAreaAuraPet,
            SpellEffect::TeleportGraveyard => SpellLog_SpellEffect::TeleportGraveyard,
            SpellEffect::NormalizedWeaponDmg => SpellLog_SpellEffect::NormalizedWeaponDmg,
            SpellEffect::Unknown122 => SpellLog_SpellEffect::Unknown122,
            SpellEffect::SendTaxi => SpellLog_SpellEffect::SendTaxi,
            SpellEffect::PlayerPull => SpellLog_SpellEffect::PlayerPull,
            SpellEffect::ModifyThreatPercent => SpellLog_SpellEffect::ModifyThreatPercent,
            SpellEffect::StealBeneficialBuff => SpellLog_SpellEffect::StealBeneficialBuff,
            SpellEffect::Prospecting => SpellLog_SpellEffect::Prospecting,
            SpellEffect::ApplyAreaAuraFriend => SpellLog_SpellEffect::ApplyAreaAuraFriend,
            SpellEffect::ApplyAreaAuraEnemy => SpellLog_SpellEffect::ApplyAreaAuraEnemy,
            SpellEffect::RedirectThreat => SpellLog_SpellEffect::RedirectThreat,
            SpellEffect::PlaySound => SpellLog_SpellEffect::PlaySound,
            SpellEffect::PlayMusic => SpellLog_SpellEffect::PlayMusic,
            SpellEffect::UnlearnSpecialization => SpellLog_SpellEffect::UnlearnSpecialization,
            SpellEffect::KillCreditGroup => SpellLog_SpellEffect::KillCreditGroup,
            SpellEffect::CallPet => SpellLog_SpellEffect::CallPet,
            SpellEffect::HealPct => SpellLog_SpellEffect::HealPct,
            SpellEffect::EnergizePct => SpellLog_SpellEffect::EnergizePct,
            SpellEffect::LeapBack => SpellLog_SpellEffect::LeapBack,
            SpellEffect::ClearQuest => SpellLog_SpellEffect::ClearQuest,
            SpellEffect::ForceCast => SpellLog_SpellEffect::ForceCast,
            SpellEffect::ForceCastWithValue => SpellLog_SpellEffect::ForceCastWithValue,
            SpellEffect::TriggerSpellWithValue => SpellLog_SpellEffect::TriggerSpellWithValue,
            SpellEffect::ApplyAreaAuraOwner => SpellLog_SpellEffect::ApplyAreaAuraOwner,
            SpellEffect::KnockbackFromPosition => SpellLog_SpellEffect::KnockbackFromPosition,
            SpellEffect::GravityPull => SpellLog_SpellEffect::GravityPull,
            SpellEffect::ActivateRune => SpellLog_SpellEffect::ActivateRune,
            SpellEffect::QuestFail => SpellLog_SpellEffect::QuestFail,
            SpellEffect::Unknown148 => SpellLog_SpellEffect::Unknown148,
            SpellEffect::Charge2 => SpellLog_SpellEffect::Charge2,
            SpellEffect::QuestOffer => SpellLog_SpellEffect::QuestOffer,
            SpellEffect::TriggerSpell2 => SpellLog_SpellEffect::TriggerSpell2,
            SpellEffect::Unknown152 => SpellLog_SpellEffect::Unknown152,
            SpellEffect::CreatePet => SpellLog_SpellEffect::CreatePet,
            SpellEffect::TeachTaxiNode => SpellLog_SpellEffect::TeachTaxiNode,
            SpellEffect::TitanGrip => SpellLog_SpellEffect::TitanGrip,
            SpellEffect::EnchantItemPrismatic => SpellLog_SpellEffect::EnchantItemPrismatic,
            SpellEffect::CreateItem2 => SpellLog_SpellEffect::CreateItem2,
            SpellEffect::Milling => SpellLog_SpellEffect::Milling,
            SpellEffect::AllowRenamePet => SpellLog_SpellEffect::AllowRenamePet,
            SpellEffect::Unknown160 => SpellLog_SpellEffect::Unknown160,
            SpellEffect::TalentSpecCount => SpellLog_SpellEffect::TalentSpecCount,
            SpellEffect::TalentSpecSelect => SpellLog_SpellEffect::TalentSpecSelect,
            SpellEffect::Unknown163 => SpellLog_SpellEffect::Unknown163,
            SpellEffect::CancelAura => SpellLog_SpellEffect::CancelAura,
        };

        Ok(Self {
            effect: effect_if,
        })
    }

}

impl SpellLog {
    pub(crate) const fn size(&self) -> usize {
        self.effect.size() // effect: SpellLog_SpellEffect
        + 4 // amount_of_logs: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum SpellLog_SpellEffect {
    None,
    Instakill,
    SchoolDamage,
    Dummy,
    PortalTeleport,
    TeleportUnits,
    ApplyAura,
    EnvironmentalDamage,
    PowerDrain {
        amount: u32,
        multiplier: f32,
        power: Power,
        target1: Guid,
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
        resurrect_guid: Guid,
    },
    AddExtraAttacks {
        extra_attacks: u32,
        target4: Guid,
    },
    Dodge,
    Evade,
    Parry,
    Block,
    CreateItem {
        item: u32,
    },
    Weapon,
    Defense,
    PersistentAreaAura,
    Summon {
        summon_target: Guid,
    },
    Leap,
    Energize,
    WeaponPercentDamage,
    TriggerMissile,
    OpenLock {
        lock_target: Guid,
    },
    SummonChangeItem,
    ApplyAreaAuraParty,
    LearnSpell,
    SpellDefense,
    Dispel,
    Language,
    DualWield,
    Jump,
    Jump2,
    TeleportUnitsFaceCaster,
    SkillStep,
    AddHonor,
    Spawn,
    TradeSkill,
    Stealth,
    Detect,
    TransDoor {
        summon_target: Guid,
    },
    ForceCriticalHit,
    GuaranteeHit,
    EnchantItem,
    EnchantItemTemporary,
    Tamecreature,
    SummonPet {
        summon_target: Guid,
    },
    LearnPetSpell,
    WeaponDamage,
    CreateRandomItem,
    Proficiency,
    SendEvent,
    PowerBurn,
    Threat,
    TriggerSpell,
    ApplyAreaAuraRaid,
    RestoreItemCharges,
    HealMaxHealth,
    InterruptCast {
        interrupted_spell: u32,
        target5: Guid,
    },
    Distract,
    Pull,
    Pickpocket,
    AddFarsight,
    UntrainTalents,
    ApplyGlyph,
    HealMechanical,
    SummonObjectWild {
        summon_target: Guid,
    },
    ScriptEffect,
    Attack,
    Sanctuary,
    AddComboPoints,
    CreateHouse {
        summon_target: Guid,
    },
    BindSight,
    Duel {
        summon_target: Guid,
    },
    Stuck,
    SummonPlayer,
    ActivateObject,
    WmoDamage,
    WmoRepair,
    WmoChange,
    KillCreditPersonal,
    ThreatAll,
    EnchantHeldItem,
    BreakPlayerTargeting,
    SelfResurrect,
    Skinning,
    Charge,
    SummonAllTotems,
    KnockBack,
    Disenchant,
    Inebriate,
    FeedPet {
        pet_feed_guid: Guid,
    },
    DismissPet {
        pet_dismiss_guid: Guid,
    },
    Reputation,
    SummonObjectSlot1 {
        summon_target: Guid,
    },
    SummonObjectSlot2 {
        summon_target: Guid,
    },
    SummonObjectSlot3 {
        summon_target: Guid,
    },
    SummonObjectSlot4 {
        summon_target: Guid,
    },
    DispelMechanic,
    SummonDeadPet,
    DestroyAllTotems,
    DurabilityDamage {
        item_to_damage: u32,
        target6: Guid,
        unknown5: u32,
    },
    Unknown112,
    ResurrectNew {
        resurrect_guid: Guid,
    },
    AttackMe,
    DurabilityDamagePct,
    SkinPlayerCorpse,
    SpiritHeal,
    Skill,
    ApplyAreaAuraPet,
    TeleportGraveyard,
    NormalizedWeaponDmg,
    Unknown122,
    SendTaxi,
    PlayerPull,
    ModifyThreatPercent,
    StealBeneficialBuff,
    Prospecting,
    ApplyAreaAuraFriend,
    ApplyAreaAuraEnemy,
    RedirectThreat,
    PlaySound,
    PlayMusic,
    UnlearnSpecialization,
    KillCreditGroup,
    CallPet,
    HealPct,
    EnergizePct,
    LeapBack,
    ClearQuest,
    ForceCast,
    ForceCastWithValue,
    TriggerSpellWithValue,
    ApplyAreaAuraOwner,
    KnockbackFromPosition,
    GravityPull,
    ActivateRune,
    QuestFail,
    Unknown148,
    Charge2,
    QuestOffer,
    TriggerSpell2,
    Unknown152,
    CreatePet,
    TeachTaxiNode,
    TitanGrip,
    EnchantItemPrismatic,
    CreateItem2,
    Milling,
    AllowRenamePet,
    Unknown160,
    TalentSpecCount,
    TalentSpecSelect,
    Unknown163,
    CancelAura,
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
            Self::Instakill => 1,
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
            Self::Summon { .. } => 28,
            Self::Leap => 29,
            Self::Energize => 30,
            Self::WeaponPercentDamage => 31,
            Self::TriggerMissile => 32,
            Self::OpenLock { .. } => 33,
            Self::SummonChangeItem => 34,
            Self::ApplyAreaAuraParty => 35,
            Self::LearnSpell => 36,
            Self::SpellDefense => 37,
            Self::Dispel => 38,
            Self::Language => 39,
            Self::DualWield => 40,
            Self::Jump => 41,
            Self::Jump2 => 42,
            Self::TeleportUnitsFaceCaster => 43,
            Self::SkillStep => 44,
            Self::AddHonor => 45,
            Self::Spawn => 46,
            Self::TradeSkill => 47,
            Self::Stealth => 48,
            Self::Detect => 49,
            Self::TransDoor { .. } => 50,
            Self::ForceCriticalHit => 51,
            Self::GuaranteeHit => 52,
            Self::EnchantItem => 53,
            Self::EnchantItemTemporary => 54,
            Self::Tamecreature => 55,
            Self::SummonPet { .. } => 56,
            Self::LearnPetSpell => 57,
            Self::WeaponDamage => 58,
            Self::CreateRandomItem => 59,
            Self::Proficiency => 60,
            Self::SendEvent => 61,
            Self::PowerBurn => 62,
            Self::Threat => 63,
            Self::TriggerSpell => 64,
            Self::ApplyAreaAuraRaid => 65,
            Self::RestoreItemCharges => 66,
            Self::HealMaxHealth => 67,
            Self::InterruptCast { .. } => 68,
            Self::Distract => 69,
            Self::Pull => 70,
            Self::Pickpocket => 71,
            Self::AddFarsight => 72,
            Self::UntrainTalents => 73,
            Self::ApplyGlyph => 74,
            Self::HealMechanical => 75,
            Self::SummonObjectWild { .. } => 76,
            Self::ScriptEffect => 77,
            Self::Attack => 78,
            Self::Sanctuary => 79,
            Self::AddComboPoints => 80,
            Self::CreateHouse { .. } => 81,
            Self::BindSight => 82,
            Self::Duel { .. } => 83,
            Self::Stuck => 84,
            Self::SummonPlayer => 85,
            Self::ActivateObject => 86,
            Self::WmoDamage => 87,
            Self::WmoRepair => 88,
            Self::WmoChange => 89,
            Self::KillCreditPersonal => 90,
            Self::ThreatAll => 91,
            Self::EnchantHeldItem => 92,
            Self::BreakPlayerTargeting => 93,
            Self::SelfResurrect => 94,
            Self::Skinning => 95,
            Self::Charge => 96,
            Self::SummonAllTotems => 97,
            Self::KnockBack => 98,
            Self::Disenchant => 99,
            Self::Inebriate => 100,
            Self::FeedPet { .. } => 101,
            Self::DismissPet { .. } => 102,
            Self::Reputation => 103,
            Self::SummonObjectSlot1 { .. } => 104,
            Self::SummonObjectSlot2 { .. } => 105,
            Self::SummonObjectSlot3 { .. } => 106,
            Self::SummonObjectSlot4 { .. } => 107,
            Self::DispelMechanic => 108,
            Self::SummonDeadPet => 109,
            Self::DestroyAllTotems => 110,
            Self::DurabilityDamage { .. } => 111,
            Self::Unknown112 => 112,
            Self::ResurrectNew { .. } => 113,
            Self::AttackMe => 114,
            Self::DurabilityDamagePct => 115,
            Self::SkinPlayerCorpse => 116,
            Self::SpiritHeal => 117,
            Self::Skill => 118,
            Self::ApplyAreaAuraPet => 119,
            Self::TeleportGraveyard => 120,
            Self::NormalizedWeaponDmg => 121,
            Self::Unknown122 => 122,
            Self::SendTaxi => 123,
            Self::PlayerPull => 124,
            Self::ModifyThreatPercent => 125,
            Self::StealBeneficialBuff => 126,
            Self::Prospecting => 127,
            Self::ApplyAreaAuraFriend => 128,
            Self::ApplyAreaAuraEnemy => 129,
            Self::RedirectThreat => 130,
            Self::PlaySound => 131,
            Self::PlayMusic => 132,
            Self::UnlearnSpecialization => 133,
            Self::KillCreditGroup => 134,
            Self::CallPet => 135,
            Self::HealPct => 136,
            Self::EnergizePct => 137,
            Self::LeapBack => 138,
            Self::ClearQuest => 139,
            Self::ForceCast => 140,
            Self::ForceCastWithValue => 141,
            Self::TriggerSpellWithValue => 142,
            Self::ApplyAreaAuraOwner => 143,
            Self::KnockbackFromPosition => 144,
            Self::GravityPull => 145,
            Self::ActivateRune => 146,
            Self::QuestFail => 147,
            Self::Unknown148 => 148,
            Self::Charge2 => 149,
            Self::QuestOffer => 150,
            Self::TriggerSpell2 => 151,
            Self::Unknown152 => 152,
            Self::CreatePet => 153,
            Self::TeachTaxiNode => 154,
            Self::TitanGrip => 155,
            Self::EnchantItemPrismatic => 156,
            Self::CreateItem2 => 157,
            Self::Milling => 158,
            Self::AllowRenamePet => 159,
            Self::Unknown160 => 160,
            Self::TalentSpecCount => 161,
            Self::TalentSpecSelect => 162,
            Self::Unknown163 => 163,
            Self::CancelAura => 164,
        }
    }

}

impl std::fmt::Display for SpellLog_SpellEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Instakill => f.write_str("Instakill"),
            Self::SchoolDamage => f.write_str("SchoolDamage"),
            Self::Dummy => f.write_str("Dummy"),
            Self::PortalTeleport => f.write_str("PortalTeleport"),
            Self::TeleportUnits => f.write_str("TeleportUnits"),
            Self::ApplyAura => f.write_str("ApplyAura"),
            Self::EnvironmentalDamage => f.write_str("EnvironmentalDamage"),
            Self::PowerDrain{ .. } => f.write_str("PowerDrain"),
            Self::HealthLeech => f.write_str("HealthLeech"),
            Self::Heal => f.write_str("Heal"),
            Self::Bind => f.write_str("Bind"),
            Self::Portal => f.write_str("Portal"),
            Self::RitualBase => f.write_str("RitualBase"),
            Self::RitualSpecialize => f.write_str("RitualSpecialize"),
            Self::RitualActivatePortal => f.write_str("RitualActivatePortal"),
            Self::QuestComplete => f.write_str("QuestComplete"),
            Self::WeaponDamageNoschool => f.write_str("WeaponDamageNoschool"),
            Self::Resurrect{ .. } => f.write_str("Resurrect"),
            Self::AddExtraAttacks{ .. } => f.write_str("AddExtraAttacks"),
            Self::Dodge => f.write_str("Dodge"),
            Self::Evade => f.write_str("Evade"),
            Self::Parry => f.write_str("Parry"),
            Self::Block => f.write_str("Block"),
            Self::CreateItem{ .. } => f.write_str("CreateItem"),
            Self::Weapon => f.write_str("Weapon"),
            Self::Defense => f.write_str("Defense"),
            Self::PersistentAreaAura => f.write_str("PersistentAreaAura"),
            Self::Summon{ .. } => f.write_str("Summon"),
            Self::Leap => f.write_str("Leap"),
            Self::Energize => f.write_str("Energize"),
            Self::WeaponPercentDamage => f.write_str("WeaponPercentDamage"),
            Self::TriggerMissile => f.write_str("TriggerMissile"),
            Self::OpenLock{ .. } => f.write_str("OpenLock"),
            Self::SummonChangeItem => f.write_str("SummonChangeItem"),
            Self::ApplyAreaAuraParty => f.write_str("ApplyAreaAuraParty"),
            Self::LearnSpell => f.write_str("LearnSpell"),
            Self::SpellDefense => f.write_str("SpellDefense"),
            Self::Dispel => f.write_str("Dispel"),
            Self::Language => f.write_str("Language"),
            Self::DualWield => f.write_str("DualWield"),
            Self::Jump => f.write_str("Jump"),
            Self::Jump2 => f.write_str("Jump2"),
            Self::TeleportUnitsFaceCaster => f.write_str("TeleportUnitsFaceCaster"),
            Self::SkillStep => f.write_str("SkillStep"),
            Self::AddHonor => f.write_str("AddHonor"),
            Self::Spawn => f.write_str("Spawn"),
            Self::TradeSkill => f.write_str("TradeSkill"),
            Self::Stealth => f.write_str("Stealth"),
            Self::Detect => f.write_str("Detect"),
            Self::TransDoor{ .. } => f.write_str("TransDoor"),
            Self::ForceCriticalHit => f.write_str("ForceCriticalHit"),
            Self::GuaranteeHit => f.write_str("GuaranteeHit"),
            Self::EnchantItem => f.write_str("EnchantItem"),
            Self::EnchantItemTemporary => f.write_str("EnchantItemTemporary"),
            Self::Tamecreature => f.write_str("Tamecreature"),
            Self::SummonPet{ .. } => f.write_str("SummonPet"),
            Self::LearnPetSpell => f.write_str("LearnPetSpell"),
            Self::WeaponDamage => f.write_str("WeaponDamage"),
            Self::CreateRandomItem => f.write_str("CreateRandomItem"),
            Self::Proficiency => f.write_str("Proficiency"),
            Self::SendEvent => f.write_str("SendEvent"),
            Self::PowerBurn => f.write_str("PowerBurn"),
            Self::Threat => f.write_str("Threat"),
            Self::TriggerSpell => f.write_str("TriggerSpell"),
            Self::ApplyAreaAuraRaid => f.write_str("ApplyAreaAuraRaid"),
            Self::RestoreItemCharges => f.write_str("RestoreItemCharges"),
            Self::HealMaxHealth => f.write_str("HealMaxHealth"),
            Self::InterruptCast{ .. } => f.write_str("InterruptCast"),
            Self::Distract => f.write_str("Distract"),
            Self::Pull => f.write_str("Pull"),
            Self::Pickpocket => f.write_str("Pickpocket"),
            Self::AddFarsight => f.write_str("AddFarsight"),
            Self::UntrainTalents => f.write_str("UntrainTalents"),
            Self::ApplyGlyph => f.write_str("ApplyGlyph"),
            Self::HealMechanical => f.write_str("HealMechanical"),
            Self::SummonObjectWild{ .. } => f.write_str("SummonObjectWild"),
            Self::ScriptEffect => f.write_str("ScriptEffect"),
            Self::Attack => f.write_str("Attack"),
            Self::Sanctuary => f.write_str("Sanctuary"),
            Self::AddComboPoints => f.write_str("AddComboPoints"),
            Self::CreateHouse{ .. } => f.write_str("CreateHouse"),
            Self::BindSight => f.write_str("BindSight"),
            Self::Duel{ .. } => f.write_str("Duel"),
            Self::Stuck => f.write_str("Stuck"),
            Self::SummonPlayer => f.write_str("SummonPlayer"),
            Self::ActivateObject => f.write_str("ActivateObject"),
            Self::WmoDamage => f.write_str("WmoDamage"),
            Self::WmoRepair => f.write_str("WmoRepair"),
            Self::WmoChange => f.write_str("WmoChange"),
            Self::KillCreditPersonal => f.write_str("KillCreditPersonal"),
            Self::ThreatAll => f.write_str("ThreatAll"),
            Self::EnchantHeldItem => f.write_str("EnchantHeldItem"),
            Self::BreakPlayerTargeting => f.write_str("BreakPlayerTargeting"),
            Self::SelfResurrect => f.write_str("SelfResurrect"),
            Self::Skinning => f.write_str("Skinning"),
            Self::Charge => f.write_str("Charge"),
            Self::SummonAllTotems => f.write_str("SummonAllTotems"),
            Self::KnockBack => f.write_str("KnockBack"),
            Self::Disenchant => f.write_str("Disenchant"),
            Self::Inebriate => f.write_str("Inebriate"),
            Self::FeedPet{ .. } => f.write_str("FeedPet"),
            Self::DismissPet{ .. } => f.write_str("DismissPet"),
            Self::Reputation => f.write_str("Reputation"),
            Self::SummonObjectSlot1{ .. } => f.write_str("SummonObjectSlot1"),
            Self::SummonObjectSlot2{ .. } => f.write_str("SummonObjectSlot2"),
            Self::SummonObjectSlot3{ .. } => f.write_str("SummonObjectSlot3"),
            Self::SummonObjectSlot4{ .. } => f.write_str("SummonObjectSlot4"),
            Self::DispelMechanic => f.write_str("DispelMechanic"),
            Self::SummonDeadPet => f.write_str("SummonDeadPet"),
            Self::DestroyAllTotems => f.write_str("DestroyAllTotems"),
            Self::DurabilityDamage{ .. } => f.write_str("DurabilityDamage"),
            Self::Unknown112 => f.write_str("Unknown112"),
            Self::ResurrectNew{ .. } => f.write_str("ResurrectNew"),
            Self::AttackMe => f.write_str("AttackMe"),
            Self::DurabilityDamagePct => f.write_str("DurabilityDamagePct"),
            Self::SkinPlayerCorpse => f.write_str("SkinPlayerCorpse"),
            Self::SpiritHeal => f.write_str("SpiritHeal"),
            Self::Skill => f.write_str("Skill"),
            Self::ApplyAreaAuraPet => f.write_str("ApplyAreaAuraPet"),
            Self::TeleportGraveyard => f.write_str("TeleportGraveyard"),
            Self::NormalizedWeaponDmg => f.write_str("NormalizedWeaponDmg"),
            Self::Unknown122 => f.write_str("Unknown122"),
            Self::SendTaxi => f.write_str("SendTaxi"),
            Self::PlayerPull => f.write_str("PlayerPull"),
            Self::ModifyThreatPercent => f.write_str("ModifyThreatPercent"),
            Self::StealBeneficialBuff => f.write_str("StealBeneficialBuff"),
            Self::Prospecting => f.write_str("Prospecting"),
            Self::ApplyAreaAuraFriend => f.write_str("ApplyAreaAuraFriend"),
            Self::ApplyAreaAuraEnemy => f.write_str("ApplyAreaAuraEnemy"),
            Self::RedirectThreat => f.write_str("RedirectThreat"),
            Self::PlaySound => f.write_str("PlaySound"),
            Self::PlayMusic => f.write_str("PlayMusic"),
            Self::UnlearnSpecialization => f.write_str("UnlearnSpecialization"),
            Self::KillCreditGroup => f.write_str("KillCreditGroup"),
            Self::CallPet => f.write_str("CallPet"),
            Self::HealPct => f.write_str("HealPct"),
            Self::EnergizePct => f.write_str("EnergizePct"),
            Self::LeapBack => f.write_str("LeapBack"),
            Self::ClearQuest => f.write_str("ClearQuest"),
            Self::ForceCast => f.write_str("ForceCast"),
            Self::ForceCastWithValue => f.write_str("ForceCastWithValue"),
            Self::TriggerSpellWithValue => f.write_str("TriggerSpellWithValue"),
            Self::ApplyAreaAuraOwner => f.write_str("ApplyAreaAuraOwner"),
            Self::KnockbackFromPosition => f.write_str("KnockbackFromPosition"),
            Self::GravityPull => f.write_str("GravityPull"),
            Self::ActivateRune => f.write_str("ActivateRune"),
            Self::QuestFail => f.write_str("QuestFail"),
            Self::Unknown148 => f.write_str("Unknown148"),
            Self::Charge2 => f.write_str("Charge2"),
            Self::QuestOffer => f.write_str("QuestOffer"),
            Self::TriggerSpell2 => f.write_str("TriggerSpell2"),
            Self::Unknown152 => f.write_str("Unknown152"),
            Self::CreatePet => f.write_str("CreatePet"),
            Self::TeachTaxiNode => f.write_str("TeachTaxiNode"),
            Self::TitanGrip => f.write_str("TitanGrip"),
            Self::EnchantItemPrismatic => f.write_str("EnchantItemPrismatic"),
            Self::CreateItem2 => f.write_str("CreateItem2"),
            Self::Milling => f.write_str("Milling"),
            Self::AllowRenamePet => f.write_str("AllowRenamePet"),
            Self::Unknown160 => f.write_str("Unknown160"),
            Self::TalentSpecCount => f.write_str("TalentSpecCount"),
            Self::TalentSpecSelect => f.write_str("TalentSpecSelect"),
            Self::Unknown163 => f.write_str("Unknown163"),
            Self::CancelAura => f.write_str("CancelAura"),
        }
    }
}

impl SpellLog_SpellEffect {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::PowerDrain {
                target1,
                ..
            } => {
                4
                + 4 // amount: u32
                + 4 // multiplier: f32
                + 4 // power: Power
                + crate::util::packed_guid_size(&target1) // target1: PackedGuid
            }
            Self::Resurrect {
                resurrect_guid,
            } => {
                4
                + crate::util::packed_guid_size(&resurrect_guid) // resurrect_guid: PackedGuid
            }
            Self::AddExtraAttacks {
                target4,
                ..
            } => {
                4
                + 4 // extra_attacks: u32
                + crate::util::packed_guid_size(&target4) // target4: PackedGuid
            }
            Self::CreateItem {
                ..
            } => {
                4
                + 4 // item: Item
            }
            Self::Summon {
                summon_target,
            } => {
                4
                + crate::util::packed_guid_size(&summon_target) // summon_target: PackedGuid
            }
            Self::OpenLock {
                lock_target,
            } => {
                4
                + crate::util::packed_guid_size(&lock_target) // lock_target: PackedGuid
            }
            Self::TransDoor {
                summon_target,
            } => {
                4
                + crate::util::packed_guid_size(&summon_target) // summon_target: PackedGuid
            }
            Self::SummonPet {
                summon_target,
            } => {
                4
                + crate::util::packed_guid_size(&summon_target) // summon_target: PackedGuid
            }
            Self::InterruptCast {
                target5,
                ..
            } => {
                4
                + 4 // interrupted_spell: Spell
                + crate::util::packed_guid_size(&target5) // target5: PackedGuid
            }
            Self::SummonObjectWild {
                summon_target,
            } => {
                4
                + crate::util::packed_guid_size(&summon_target) // summon_target: PackedGuid
            }
            Self::CreateHouse {
                summon_target,
            } => {
                4
                + crate::util::packed_guid_size(&summon_target) // summon_target: PackedGuid
            }
            Self::Duel {
                summon_target,
            } => {
                4
                + crate::util::packed_guid_size(&summon_target) // summon_target: PackedGuid
            }
            Self::FeedPet {
                pet_feed_guid,
            } => {
                4
                + crate::util::packed_guid_size(&pet_feed_guid) // pet_feed_guid: PackedGuid
            }
            Self::DismissPet {
                pet_dismiss_guid,
            } => {
                4
                + crate::util::packed_guid_size(&pet_dismiss_guid) // pet_dismiss_guid: PackedGuid
            }
            Self::SummonObjectSlot1 {
                summon_target,
            } => {
                4
                + crate::util::packed_guid_size(&summon_target) // summon_target: PackedGuid
            }
            Self::SummonObjectSlot2 {
                summon_target,
            } => {
                4
                + crate::util::packed_guid_size(&summon_target) // summon_target: PackedGuid
            }
            Self::SummonObjectSlot3 {
                summon_target,
            } => {
                4
                + crate::util::packed_guid_size(&summon_target) // summon_target: PackedGuid
            }
            Self::SummonObjectSlot4 {
                summon_target,
            } => {
                4
                + crate::util::packed_guid_size(&summon_target) // summon_target: PackedGuid
            }
            Self::DurabilityDamage {
                target6,
                ..
            } => {
                4
                + 4 // item_to_damage: Item
                + crate::util::packed_guid_size(&target6) // target6: PackedGuid
                + 4 // unknown5: u32
            }
            Self::ResurrectNew {
                resurrect_guid,
            } => {
                4
                + crate::util::packed_guid_size(&resurrect_guid) // resurrect_guid: PackedGuid
            }
            _ => 4,
        }
    }
}

