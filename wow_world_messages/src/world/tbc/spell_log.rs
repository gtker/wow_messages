use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::{
    Power, SpellEffect,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelllogexecute.wowm:397`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelllogexecute.wowm#L397):
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
///         u32 interrupted_spell;
///     }
///     else if (effect == DURABILITY_DAMAGE) {
///         PackedGuid target6;
///         u32 item_to_damage;
///         u32 unknown5;
///     }
///     else if (effect == OPEN_LOCK
///         || effect == OPEN_LOCK_ITEM) {
///         PackedGuid lock_target;
///     }
///     else if (effect == CREATE_ITEM) {
///         u32 item;
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
                // item: u32
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
            SpellLog_SpellEffect::OpenLockItem {
                lock_target,
            } => {
                // lock_target: PackedGuid
                crate::util::write_packed_guid(&lock_target, &mut w)?;

            }
            SpellLog_SpellEffect::InterruptCast {
                interrupted_spell,
                target5,
            } => {
                // target5: PackedGuid
                crate::util::write_packed_guid(&target5, &mut w)?;

                // interrupted_spell: u32
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

                // item_to_damage: u32
                w.write_all(&item_to_damage.to_le_bytes())?;

                // unknown5: u32
                w.write_all(&unknown5.to_le_bytes())?;

            }
            _ => {}
        }

        Ok(())
    }
}

impl SpellLog {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseError> {
        // effect: SpellEffect
        let effect: SpellEffect = crate::util::read_u32_le(&mut r)?.try_into()?;

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
                let power: Power = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

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
            SpellEffect::Resurrect => SpellLog_SpellEffect::Resurrect,
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
                // item: u32
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
            SpellEffect::Unknown41 => SpellLog_SpellEffect::Unknown41,
            SpellEffect::Unknown42 => SpellLog_SpellEffect::Unknown42,
            SpellEffect::TeleportUnitsFaceCaster => SpellLog_SpellEffect::TeleportUnitsFaceCaster,
            SpellEffect::SkillStep => SpellLog_SpellEffect::SkillStep,
            SpellEffect::Undefined45 => SpellLog_SpellEffect::Undefined45,
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
            SpellEffect::OpenLockItem => {
                // lock_target: PackedGuid
                let lock_target = crate::util::read_packed_guid(&mut r)?;

                SpellLog_SpellEffect::OpenLockItem {
                    lock_target,
                }
            }
            SpellEffect::Proficiency => SpellLog_SpellEffect::Proficiency,
            SpellEffect::SendEvent => SpellLog_SpellEffect::SendEvent,
            SpellEffect::PowerBurn => SpellLog_SpellEffect::PowerBurn,
            SpellEffect::Threat => SpellLog_SpellEffect::Threat,
            SpellEffect::TriggerSpell => SpellLog_SpellEffect::TriggerSpell,
            SpellEffect::HealthFunnel => SpellLog_SpellEffect::HealthFunnel,
            SpellEffect::PowerFunnel => SpellLog_SpellEffect::PowerFunnel,
            SpellEffect::HealMaxHealth => SpellLog_SpellEffect::HealMaxHealth,
            SpellEffect::InterruptCast => {
                // target5: PackedGuid
                let target5 = crate::util::read_packed_guid(&mut r)?;

                // interrupted_spell: u32
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
            SpellEffect::Unknown73 => SpellLog_SpellEffect::Unknown73,
            SpellEffect::Unknown74 => SpellLog_SpellEffect::Unknown74,
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
            SpellEffect::Unknown87 => SpellLog_SpellEffect::Unknown87,
            SpellEffect::Unknown88 => SpellLog_SpellEffect::Unknown88,
            SpellEffect::Unknown89 => SpellLog_SpellEffect::Unknown89,
            SpellEffect::Unknown90 => SpellLog_SpellEffect::Unknown90,
            SpellEffect::ThreatAll => SpellLog_SpellEffect::ThreatAll,
            SpellEffect::EnchantHeldItem => SpellLog_SpellEffect::EnchantHeldItem,
            SpellEffect::Unknown93 => SpellLog_SpellEffect::Unknown93,
            SpellEffect::SelfResurrect => SpellLog_SpellEffect::SelfResurrect,
            SpellEffect::Skinning => SpellLog_SpellEffect::Skinning,
            SpellEffect::Charge => SpellLog_SpellEffect::Charge,
            SpellEffect::Unknown97 => SpellLog_SpellEffect::Unknown97,
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

                // item_to_damage: u32
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
            SpellEffect::ResurrectNew => SpellLog_SpellEffect::ResurrectNew,
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
            SpellEffect::Unknown141 => SpellLog_SpellEffect::Unknown141,
            SpellEffect::TriggerSpellWithValue => SpellLog_SpellEffect::TriggerSpellWithValue,
            SpellEffect::ApplyAreaAuraOwner => SpellLog_SpellEffect::ApplyAreaAuraOwner,
            SpellEffect::KnockbackFromPosition => SpellLog_SpellEffect::KnockbackFromPosition,
            SpellEffect::Unknown145 => SpellLog_SpellEffect::Unknown145,
            SpellEffect::Unknown146 => SpellLog_SpellEffect::Unknown146,
            SpellEffect::QuestFail => SpellLog_SpellEffect::QuestFail,
            SpellEffect::Unknown148 => SpellLog_SpellEffect::Unknown148,
            SpellEffect::Charge2 => SpellLog_SpellEffect::Charge2,
            SpellEffect::Unknown150 => SpellLog_SpellEffect::Unknown150,
            SpellEffect::TriggerSpell2 => SpellLog_SpellEffect::TriggerSpell2,
            SpellEffect::Unknown152 => SpellLog_SpellEffect::Unknown152,
            SpellEffect::Unknown153 => SpellLog_SpellEffect::Unknown153,
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
    Resurrect,
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
    Unknown41,
    Unknown42,
    TeleportUnitsFaceCaster,
    SkillStep,
    Undefined45,
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
    OpenLockItem {
        lock_target: Guid,
    },
    Proficiency,
    SendEvent,
    PowerBurn,
    Threat,
    TriggerSpell,
    HealthFunnel,
    PowerFunnel,
    HealMaxHealth,
    InterruptCast {
        interrupted_spell: u32,
        target5: Guid,
    },
    Distract,
    Pull,
    Pickpocket,
    AddFarsight,
    Unknown73,
    Unknown74,
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
    Unknown87,
    Unknown88,
    Unknown89,
    Unknown90,
    ThreatAll,
    EnchantHeldItem,
    Unknown93,
    SelfResurrect,
    Skinning,
    Charge,
    Unknown97,
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
    ResurrectNew,
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
    Unknown141,
    TriggerSpellWithValue,
    ApplyAreaAuraOwner,
    KnockbackFromPosition,
    Unknown145,
    Unknown146,
    QuestFail,
    Unknown148,
    Charge2,
    Unknown150,
    TriggerSpell2,
    Unknown152,
    Unknown153,
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
            Self::Resurrect => 18,
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
            Self::Unknown41 => 41,
            Self::Unknown42 => 42,
            Self::TeleportUnitsFaceCaster => 43,
            Self::SkillStep => 44,
            Self::Undefined45 => 45,
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
            Self::OpenLockItem { .. } => 59,
            Self::Proficiency => 60,
            Self::SendEvent => 61,
            Self::PowerBurn => 62,
            Self::Threat => 63,
            Self::TriggerSpell => 64,
            Self::HealthFunnel => 65,
            Self::PowerFunnel => 66,
            Self::HealMaxHealth => 67,
            Self::InterruptCast { .. } => 68,
            Self::Distract => 69,
            Self::Pull => 70,
            Self::Pickpocket => 71,
            Self::AddFarsight => 72,
            Self::Unknown73 => 73,
            Self::Unknown74 => 74,
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
            Self::Unknown87 => 87,
            Self::Unknown88 => 88,
            Self::Unknown89 => 89,
            Self::Unknown90 => 90,
            Self::ThreatAll => 91,
            Self::EnchantHeldItem => 92,
            Self::Unknown93 => 93,
            Self::SelfResurrect => 94,
            Self::Skinning => 95,
            Self::Charge => 96,
            Self::Unknown97 => 97,
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
            Self::ResurrectNew => 113,
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
            Self::Unknown141 => 141,
            Self::TriggerSpellWithValue => 142,
            Self::ApplyAreaAuraOwner => 143,
            Self::KnockbackFromPosition => 144,
            Self::Unknown145 => 145,
            Self::Unknown146 => 146,
            Self::QuestFail => 147,
            Self::Unknown148 => 148,
            Self::Charge2 => 149,
            Self::Unknown150 => 150,
            Self::TriggerSpell2 => 151,
            Self::Unknown152 => 152,
            Self::Unknown153 => 153,
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
                + 4 // item: u32
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
            Self::OpenLockItem {
                lock_target,
            } => {
                4
                + crate::util::packed_guid_size(&lock_target) // lock_target: PackedGuid
            }
            Self::InterruptCast {
                target5,
                ..
            } => {
                4
                + 4 // interrupted_spell: u32
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
                + 4 // item_to_damage: u32
                + crate::util::packed_guid_size(&target6) // target6: PackedGuid
                + 4 // unknown5: u32
            }
            _ => 4,
        }
    }
}

