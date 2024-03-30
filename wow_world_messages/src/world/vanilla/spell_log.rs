use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    Power, SpellEffect,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelllogexecute.wowm:143`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelllogexecute.wowm#L143):
/// ```text
/// struct SpellLog {
///     SpellEffect effect;
///     u32 amount_of_logs = 1;
///     if (effect == POWER_DRAIN) {
///         Guid target1;
///         u32 amount;
///         (u32)Power power;
///         f32 multiplier;
///     }
///     else if (effect == HEAL
///         || effect == HEAL_MAX_HEALTH) {
///         Guid target2;
///         u32 heal_amount;
///         u32 heal_critical;
///     }
///     else if (effect == ENERGIZE) {
///         Guid target3;
///         u32 energize_amount;
///         u32 energize_power;
///     }
///     else if (effect == ADD_EXTRA_ATTACKS) {
///         Guid target4;
///         u32 extra_attacks;
///     }
///     else if (effect == CREATE_ITEM) {
///         Item item;
///     }
///     else if (effect == INTERRUPT_CAST) {
///         Guid target5;
///         Spell interrupted_spell;
///     }
///     else if (effect == DURABILITY_DAMAGE) {
///         Guid target6;
///         Item item_to_damage;
///         u32 unknown5;
///     }
///     else if (effect == FEED_PET) {
///         Item feed_pet_item;
///     }
///     else if (effect == INSTAKILL
///         || effect == RESURRECT
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
///         || effect == OPEN_LOCK
///         || effect == OPEN_LOCK_ITEM
///         || effect == DISMISS_PET
///         || effect == TRANS_DOOR
///         || effect == SUMMON
///         || effect == SUMMON_PET
///         || effect == SUMMON_WILD
///         || effect == SUMMON_GUARDIAN
///         || effect == SUMMON_TOTEM_SLOT1
///         || effect == SUMMON_TOTEM_SLOT2
///         || effect == SUMMON_TOTEM_SLOT3
///         || effect == SUMMON_TOTEM_SLOT4
///         || effect == SUMMON_POSSESSED
///         || effect == SUMMON_TOTEM
///         || effect == SUMMON_CRITTER
///         || effect == SUMMON_OBJECT_WILD
///         || effect == SUMMON_OBJECT_SLOT1
///         || effect == SUMMON_OBJECT_SLOT2
///         || effect == SUMMON_OBJECT_SLOT3
///         || effect == SUMMON_OBJECT_SLOT4
///         || effect == SUMMON_DEMON) {
///         Guid target7;
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum SpellLog {
    None,
    Instakill {
        target7: Guid,
    },
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
    Heal {
        heal_amount: u32,
        heal_critical: u32,
        target2: Guid,
    },
    Bind,
    Portal,
    RitualBase,
    RitualSpecialize,
    RitualActivatePortal,
    QuestComplete,
    WeaponDamageNoschool,
    Resurrect {
        target7: Guid,
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
        target7: Guid,
    },
    Leap,
    Energize {
        energize_amount: u32,
        energize_power: u32,
        target3: Guid,
    },
    WeaponPercentDamage,
    TriggerMissile,
    OpenLock {
        target7: Guid,
    },
    SummonChangeItem,
    ApplyAreaAuraParty,
    LearnSpell,
    SpellDefense,
    Dispel {
        target7: Guid,
    },
    Language,
    DualWield,
    SummonWild {
        target7: Guid,
    },
    SummonGuardian {
        target7: Guid,
    },
    TeleportUnitsFaceCaster,
    SkillStep,
    AddHonor,
    Spawn,
    TradeSkill,
    Stealth,
    Detect,
    TransDoor {
        target7: Guid,
    },
    ForceCriticalHit,
    GuaranteeHit,
    EnchantItem,
    EnchantItemTemporary,
    Tamecreature,
    SummonPet {
        target7: Guid,
    },
    LearnPetSpell,
    WeaponDamage,
    OpenLockItem {
        target7: Guid,
    },
    Proficiency,
    SendEvent,
    PowerBurn,
    Threat {
        target7: Guid,
    },
    TriggerSpell,
    HealthFunnel,
    PowerFunnel,
    HealMaxHealth {
        heal_amount: u32,
        heal_critical: u32,
        target2: Guid,
    },
    InterruptCast {
        interrupted_spell: u32,
        target5: Guid,
    },
    Distract {
        target7: Guid,
    },
    Pull,
    Pickpocket,
    AddFarsight,
    SummonPossessed {
        target7: Guid,
    },
    SummonTotem {
        target7: Guid,
    },
    HealMechanical,
    SummonObjectWild {
        target7: Guid,
    },
    ScriptEffect,
    Attack,
    Sanctuary {
        target7: Guid,
    },
    AddComboPoints,
    CreateHouse,
    BindSight,
    Duel,
    Stuck,
    SummonPlayer,
    ActivateObject,
    SummonTotemSlot1 {
        target7: Guid,
    },
    SummonTotemSlot2 {
        target7: Guid,
    },
    SummonTotemSlot3 {
        target7: Guid,
    },
    SummonTotemSlot4 {
        target7: Guid,
    },
    ThreatAll {
        target7: Guid,
    },
    EnchantHeldItem,
    SummonPhantasm,
    SelfResurrect,
    Skinning,
    Charge,
    SummonCritter {
        target7: Guid,
    },
    KnockBack,
    Disenchant,
    Inebriate,
    FeedPet {
        feed_pet_item: u32,
    },
    DismissPet {
        target7: Guid,
    },
    Reputation,
    SummonObjectSlot1 {
        target7: Guid,
    },
    SummonObjectSlot2 {
        target7: Guid,
    },
    SummonObjectSlot3 {
        target7: Guid,
    },
    SummonObjectSlot4 {
        target7: Guid,
    },
    DispelMechanic {
        target7: Guid,
    },
    SummonDeadPet,
    DestroyAllTotems,
    DurabilityDamage {
        item_to_damage: u32,
        target6: Guid,
        unknown5: u32,
    },
    SummonDemon {
        target7: Guid,
    },
    ResurrectNew {
        target7: Guid,
    },
    AttackMe {
        target7: Guid,
    },
    DurabilityDamagePct,
    SkinPlayerCorpse {
        target7: Guid,
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
        target7: Guid,
    },
    Unknown126 {
        target7: Guid,
    },
    Unknown127,
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
        w.write_all(&(self.as_int().to_le_bytes()))?;

        // amount_of_logs: u32
        w.write_all(&Self::AMOUNT_OF_LOGS_VALUE.to_le_bytes())?;

        match &self {
            SpellLog::Instakill {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::PowerDrain {
                amount,
                multiplier,
                power,
                target1,
            } => {
                // target1: Guid
                w.write_all(&target1.guid().to_le_bytes())?;

                // amount: u32
                w.write_all(&amount.to_le_bytes())?;

                // power: Power
                w.write_all(&u32::from(power.as_int()).to_le_bytes())?;

                // multiplier: f32
                w.write_all(&multiplier.to_le_bytes())?;

            }
            SpellLog::Heal {
                heal_amount,
                heal_critical,
                target2,
            } => {
                // target2: Guid
                w.write_all(&target2.guid().to_le_bytes())?;

                // heal_amount: u32
                w.write_all(&heal_amount.to_le_bytes())?;

                // heal_critical: u32
                w.write_all(&heal_critical.to_le_bytes())?;

            }
            SpellLog::Resurrect {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::AddExtraAttacks {
                extra_attacks,
                target4,
            } => {
                // target4: Guid
                w.write_all(&target4.guid().to_le_bytes())?;

                // extra_attacks: u32
                w.write_all(&extra_attacks.to_le_bytes())?;

            }
            SpellLog::CreateItem {
                item,
            } => {
                // item: Item
                w.write_all(&item.to_le_bytes())?;

            }
            SpellLog::Summon {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::Energize {
                energize_amount,
                energize_power,
                target3,
            } => {
                // target3: Guid
                w.write_all(&target3.guid().to_le_bytes())?;

                // energize_amount: u32
                w.write_all(&energize_amount.to_le_bytes())?;

                // energize_power: u32
                w.write_all(&energize_power.to_le_bytes())?;

            }
            SpellLog::OpenLock {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::Dispel {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::SummonWild {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::SummonGuardian {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::TransDoor {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::SummonPet {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::OpenLockItem {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::Threat {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::HealMaxHealth {
                heal_amount,
                heal_critical,
                target2,
            } => {
                // target2: Guid
                w.write_all(&target2.guid().to_le_bytes())?;

                // heal_amount: u32
                w.write_all(&heal_amount.to_le_bytes())?;

                // heal_critical: u32
                w.write_all(&heal_critical.to_le_bytes())?;

            }
            SpellLog::InterruptCast {
                interrupted_spell,
                target5,
            } => {
                // target5: Guid
                w.write_all(&target5.guid().to_le_bytes())?;

                // interrupted_spell: Spell
                w.write_all(&interrupted_spell.to_le_bytes())?;

            }
            SpellLog::Distract {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::SummonPossessed {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::SummonTotem {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::SummonObjectWild {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::Sanctuary {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::SummonTotemSlot1 {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::SummonTotemSlot2 {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::SummonTotemSlot3 {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::SummonTotemSlot4 {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::ThreatAll {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::SummonCritter {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::FeedPet {
                feed_pet_item,
            } => {
                // feed_pet_item: Item
                w.write_all(&feed_pet_item.to_le_bytes())?;

            }
            SpellLog::DismissPet {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::SummonObjectSlot1 {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::SummonObjectSlot2 {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::SummonObjectSlot3 {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::SummonObjectSlot4 {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::DispelMechanic {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::DurabilityDamage {
                item_to_damage,
                target6,
                unknown5,
            } => {
                // target6: Guid
                w.write_all(&target6.guid().to_le_bytes())?;

                // item_to_damage: Item
                w.write_all(&item_to_damage.to_le_bytes())?;

                // unknown5: u32
                w.write_all(&unknown5.to_le_bytes())?;

            }
            SpellLog::SummonDemon {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::ResurrectNew {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::AttackMe {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::SkinPlayerCorpse {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::ModifyThreatPercent {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

            }
            SpellLog::Unknown126 {
                target7,
            } => {
                // target7: Guid
                w.write_all(&target7.guid().to_le_bytes())?;

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
            SpellEffect::None => SpellLog::None,
            SpellEffect::Instakill => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::Instakill {
                    target7,
                }
            }
            SpellEffect::SchoolDamage => SpellLog::SchoolDamage,
            SpellEffect::Dummy => SpellLog::Dummy,
            SpellEffect::PortalTeleport => SpellLog::PortalTeleport,
            SpellEffect::TeleportUnits => SpellLog::TeleportUnits,
            SpellEffect::ApplyAura => SpellLog::ApplyAura,
            SpellEffect::EnvironmentalDamage => SpellLog::EnvironmentalDamage,
            SpellEffect::PowerDrain => {
                // target1: Guid
                let target1 = crate::util::read_guid(&mut r)?;

                // amount: u32
                let amount = crate::util::read_u32_le(&mut r)?;

                // power: Power
                let power = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

                // multiplier: f32
                let multiplier = crate::util::read_f32_le(&mut r)?;

                SpellLog::PowerDrain {
                    amount,
                    multiplier,
                    power,
                    target1,
                }
            }
            SpellEffect::HealthLeech => SpellLog::HealthLeech,
            SpellEffect::Heal => {
                // target2: Guid
                let target2 = crate::util::read_guid(&mut r)?;

                // heal_amount: u32
                let heal_amount = crate::util::read_u32_le(&mut r)?;

                // heal_critical: u32
                let heal_critical = crate::util::read_u32_le(&mut r)?;

                SpellLog::Heal {
                    heal_amount,
                    heal_critical,
                    target2,
                }
            }
            SpellEffect::Bind => SpellLog::Bind,
            SpellEffect::Portal => SpellLog::Portal,
            SpellEffect::RitualBase => SpellLog::RitualBase,
            SpellEffect::RitualSpecialize => SpellLog::RitualSpecialize,
            SpellEffect::RitualActivatePortal => SpellLog::RitualActivatePortal,
            SpellEffect::QuestComplete => SpellLog::QuestComplete,
            SpellEffect::WeaponDamageNoschool => SpellLog::WeaponDamageNoschool,
            SpellEffect::Resurrect => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::Resurrect {
                    target7,
                }
            }
            SpellEffect::AddExtraAttacks => {
                // target4: Guid
                let target4 = crate::util::read_guid(&mut r)?;

                // extra_attacks: u32
                let extra_attacks = crate::util::read_u32_le(&mut r)?;

                SpellLog::AddExtraAttacks {
                    extra_attacks,
                    target4,
                }
            }
            SpellEffect::Dodge => SpellLog::Dodge,
            SpellEffect::Evade => SpellLog::Evade,
            SpellEffect::Parry => SpellLog::Parry,
            SpellEffect::Block => SpellLog::Block,
            SpellEffect::CreateItem => {
                // item: Item
                let item = crate::util::read_u32_le(&mut r)?;

                SpellLog::CreateItem {
                    item,
                }
            }
            SpellEffect::Weapon => SpellLog::Weapon,
            SpellEffect::Defense => SpellLog::Defense,
            SpellEffect::PersistentAreaAura => SpellLog::PersistentAreaAura,
            SpellEffect::Summon => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::Summon {
                    target7,
                }
            }
            SpellEffect::Leap => SpellLog::Leap,
            SpellEffect::Energize => {
                // target3: Guid
                let target3 = crate::util::read_guid(&mut r)?;

                // energize_amount: u32
                let energize_amount = crate::util::read_u32_le(&mut r)?;

                // energize_power: u32
                let energize_power = crate::util::read_u32_le(&mut r)?;

                SpellLog::Energize {
                    energize_amount,
                    energize_power,
                    target3,
                }
            }
            SpellEffect::WeaponPercentDamage => SpellLog::WeaponPercentDamage,
            SpellEffect::TriggerMissile => SpellLog::TriggerMissile,
            SpellEffect::OpenLock => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::OpenLock {
                    target7,
                }
            }
            SpellEffect::SummonChangeItem => SpellLog::SummonChangeItem,
            SpellEffect::ApplyAreaAuraParty => SpellLog::ApplyAreaAuraParty,
            SpellEffect::LearnSpell => SpellLog::LearnSpell,
            SpellEffect::SpellDefense => SpellLog::SpellDefense,
            SpellEffect::Dispel => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::Dispel {
                    target7,
                }
            }
            SpellEffect::Language => SpellLog::Language,
            SpellEffect::DualWield => SpellLog::DualWield,
            SpellEffect::SummonWild => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::SummonWild {
                    target7,
                }
            }
            SpellEffect::SummonGuardian => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::SummonGuardian {
                    target7,
                }
            }
            SpellEffect::TeleportUnitsFaceCaster => SpellLog::TeleportUnitsFaceCaster,
            SpellEffect::SkillStep => SpellLog::SkillStep,
            SpellEffect::AddHonor => SpellLog::AddHonor,
            SpellEffect::Spawn => SpellLog::Spawn,
            SpellEffect::TradeSkill => SpellLog::TradeSkill,
            SpellEffect::Stealth => SpellLog::Stealth,
            SpellEffect::Detect => SpellLog::Detect,
            SpellEffect::TransDoor => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::TransDoor {
                    target7,
                }
            }
            SpellEffect::ForceCriticalHit => SpellLog::ForceCriticalHit,
            SpellEffect::GuaranteeHit => SpellLog::GuaranteeHit,
            SpellEffect::EnchantItem => SpellLog::EnchantItem,
            SpellEffect::EnchantItemTemporary => SpellLog::EnchantItemTemporary,
            SpellEffect::Tamecreature => SpellLog::Tamecreature,
            SpellEffect::SummonPet => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::SummonPet {
                    target7,
                }
            }
            SpellEffect::LearnPetSpell => SpellLog::LearnPetSpell,
            SpellEffect::WeaponDamage => SpellLog::WeaponDamage,
            SpellEffect::OpenLockItem => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::OpenLockItem {
                    target7,
                }
            }
            SpellEffect::Proficiency => SpellLog::Proficiency,
            SpellEffect::SendEvent => SpellLog::SendEvent,
            SpellEffect::PowerBurn => SpellLog::PowerBurn,
            SpellEffect::Threat => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::Threat {
                    target7,
                }
            }
            SpellEffect::TriggerSpell => SpellLog::TriggerSpell,
            SpellEffect::HealthFunnel => SpellLog::HealthFunnel,
            SpellEffect::PowerFunnel => SpellLog::PowerFunnel,
            SpellEffect::HealMaxHealth => {
                // target2: Guid
                let target2 = crate::util::read_guid(&mut r)?;

                // heal_amount: u32
                let heal_amount = crate::util::read_u32_le(&mut r)?;

                // heal_critical: u32
                let heal_critical = crate::util::read_u32_le(&mut r)?;

                SpellLog::HealMaxHealth {
                    heal_amount,
                    heal_critical,
                    target2,
                }
            }
            SpellEffect::InterruptCast => {
                // target5: Guid
                let target5 = crate::util::read_guid(&mut r)?;

                // interrupted_spell: Spell
                let interrupted_spell = crate::util::read_u32_le(&mut r)?;

                SpellLog::InterruptCast {
                    interrupted_spell,
                    target5,
                }
            }
            SpellEffect::Distract => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::Distract {
                    target7,
                }
            }
            SpellEffect::Pull => SpellLog::Pull,
            SpellEffect::Pickpocket => SpellLog::Pickpocket,
            SpellEffect::AddFarsight => SpellLog::AddFarsight,
            SpellEffect::SummonPossessed => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::SummonPossessed {
                    target7,
                }
            }
            SpellEffect::SummonTotem => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::SummonTotem {
                    target7,
                }
            }
            SpellEffect::HealMechanical => SpellLog::HealMechanical,
            SpellEffect::SummonObjectWild => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::SummonObjectWild {
                    target7,
                }
            }
            SpellEffect::ScriptEffect => SpellLog::ScriptEffect,
            SpellEffect::Attack => SpellLog::Attack,
            SpellEffect::Sanctuary => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::Sanctuary {
                    target7,
                }
            }
            SpellEffect::AddComboPoints => SpellLog::AddComboPoints,
            SpellEffect::CreateHouse => SpellLog::CreateHouse,
            SpellEffect::BindSight => SpellLog::BindSight,
            SpellEffect::Duel => SpellLog::Duel,
            SpellEffect::Stuck => SpellLog::Stuck,
            SpellEffect::SummonPlayer => SpellLog::SummonPlayer,
            SpellEffect::ActivateObject => SpellLog::ActivateObject,
            SpellEffect::SummonTotemSlot1 => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::SummonTotemSlot1 {
                    target7,
                }
            }
            SpellEffect::SummonTotemSlot2 => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::SummonTotemSlot2 {
                    target7,
                }
            }
            SpellEffect::SummonTotemSlot3 => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::SummonTotemSlot3 {
                    target7,
                }
            }
            SpellEffect::SummonTotemSlot4 => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::SummonTotemSlot4 {
                    target7,
                }
            }
            SpellEffect::ThreatAll => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::ThreatAll {
                    target7,
                }
            }
            SpellEffect::EnchantHeldItem => SpellLog::EnchantHeldItem,
            SpellEffect::SummonPhantasm => SpellLog::SummonPhantasm,
            SpellEffect::SelfResurrect => SpellLog::SelfResurrect,
            SpellEffect::Skinning => SpellLog::Skinning,
            SpellEffect::Charge => SpellLog::Charge,
            SpellEffect::SummonCritter => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::SummonCritter {
                    target7,
                }
            }
            SpellEffect::KnockBack => SpellLog::KnockBack,
            SpellEffect::Disenchant => SpellLog::Disenchant,
            SpellEffect::Inebriate => SpellLog::Inebriate,
            SpellEffect::FeedPet => {
                // feed_pet_item: Item
                let feed_pet_item = crate::util::read_u32_le(&mut r)?;

                SpellLog::FeedPet {
                    feed_pet_item,
                }
            }
            SpellEffect::DismissPet => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::DismissPet {
                    target7,
                }
            }
            SpellEffect::Reputation => SpellLog::Reputation,
            SpellEffect::SummonObjectSlot1 => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::SummonObjectSlot1 {
                    target7,
                }
            }
            SpellEffect::SummonObjectSlot2 => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::SummonObjectSlot2 {
                    target7,
                }
            }
            SpellEffect::SummonObjectSlot3 => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::SummonObjectSlot3 {
                    target7,
                }
            }
            SpellEffect::SummonObjectSlot4 => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::SummonObjectSlot4 {
                    target7,
                }
            }
            SpellEffect::DispelMechanic => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::DispelMechanic {
                    target7,
                }
            }
            SpellEffect::SummonDeadPet => SpellLog::SummonDeadPet,
            SpellEffect::DestroyAllTotems => SpellLog::DestroyAllTotems,
            SpellEffect::DurabilityDamage => {
                // target6: Guid
                let target6 = crate::util::read_guid(&mut r)?;

                // item_to_damage: Item
                let item_to_damage = crate::util::read_u32_le(&mut r)?;

                // unknown5: u32
                let unknown5 = crate::util::read_u32_le(&mut r)?;

                SpellLog::DurabilityDamage {
                    item_to_damage,
                    target6,
                    unknown5,
                }
            }
            SpellEffect::SummonDemon => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::SummonDemon {
                    target7,
                }
            }
            SpellEffect::ResurrectNew => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::ResurrectNew {
                    target7,
                }
            }
            SpellEffect::AttackMe => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::AttackMe {
                    target7,
                }
            }
            SpellEffect::DurabilityDamagePct => SpellLog::DurabilityDamagePct,
            SpellEffect::SkinPlayerCorpse => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::SkinPlayerCorpse {
                    target7,
                }
            }
            SpellEffect::SpiritHeal => SpellLog::SpiritHeal,
            SpellEffect::Skill => SpellLog::Skill,
            SpellEffect::ApplyAreaAuraPet => SpellLog::ApplyAreaAuraPet,
            SpellEffect::TeleportGraveyard => SpellLog::TeleportGraveyard,
            SpellEffect::NormalizedWeaponDmg => SpellLog::NormalizedWeaponDmg,
            SpellEffect::Unknown122 => SpellLog::Unknown122,
            SpellEffect::SendTaxi => SpellLog::SendTaxi,
            SpellEffect::PlayerPull => SpellLog::PlayerPull,
            SpellEffect::ModifyThreatPercent => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::ModifyThreatPercent {
                    target7,
                }
            }
            SpellEffect::Unknown126 => {
                // target7: Guid
                let target7 = crate::util::read_guid(&mut r)?;

                SpellLog::Unknown126 {
                    target7,
                }
            }
            SpellEffect::Unknown127 => SpellLog::Unknown127,
        };

        Ok(effect_if)
    }

}

impl SpellLog {
    pub(crate) const fn size(&self) -> usize {
        (match self {
            Self::Instakill {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::PowerDrain {
                ..
            } => {
                4
                + 4 // amount: u32
                + 4 // multiplier: f32
                + 4 // power: Power
                + 8 // target1: Guid
            }
            Self::Heal {
                ..
            } => {
                4
                + 4 // heal_amount: u32
                + 4 // heal_critical: u32
                + 8 // target2: Guid
            }
            Self::Resurrect {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::AddExtraAttacks {
                ..
            } => {
                4
                + 4 // extra_attacks: u32
                + 8 // target4: Guid
            }
            Self::CreateItem {
                ..
            } => {
                4
                + 4 // item: Item
            }
            Self::Summon {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::Energize {
                ..
            } => {
                4
                + 4 // energize_amount: u32
                + 4 // energize_power: u32
                + 8 // target3: Guid
            }
            Self::OpenLock {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::Dispel {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::SummonWild {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::SummonGuardian {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::TransDoor {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::SummonPet {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::OpenLockItem {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::Threat {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::HealMaxHealth {
                ..
            } => {
                4
                + 4 // heal_amount: u32
                + 4 // heal_critical: u32
                + 8 // target2: Guid
            }
            Self::InterruptCast {
                ..
            } => {
                4
                + 4 // interrupted_spell: Spell
                + 8 // target5: Guid
            }
            Self::Distract {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::SummonPossessed {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::SummonTotem {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::SummonObjectWild {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::Sanctuary {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::SummonTotemSlot1 {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::SummonTotemSlot2 {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::SummonTotemSlot3 {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::SummonTotemSlot4 {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::ThreatAll {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::SummonCritter {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::FeedPet {
                ..
            } => {
                4
                + 4 // feed_pet_item: Item
            }
            Self::DismissPet {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::SummonObjectSlot1 {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::SummonObjectSlot2 {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::SummonObjectSlot3 {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::SummonObjectSlot4 {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::DispelMechanic {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::DurabilityDamage {
                ..
            } => {
                4
                + 4 // item_to_damage: Item
                + 8 // target6: Guid
                + 4 // unknown5: u32
            }
            Self::SummonDemon {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::ResurrectNew {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::AttackMe {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::SkinPlayerCorpse {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::ModifyThreatPercent {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            Self::Unknown126 {
                ..
            } => {
                4
                + 8 // target7: Guid
            }
            _ => 4,
        }) // effect: SpellLog
        + 4 // amount_of_logs: u32
    }
}

impl Default for SpellLog {
    fn default() -> Self {
        // First enumerator without any fields
        Self::None
    }
}

impl SpellLog {
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
            Self::Heal { .. } => 10,
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
            Self::Energize { .. } => 30,
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
            Self::SummonWild { .. } => 41,
            Self::SummonGuardian { .. } => 42,
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
            Self::OpenLockItem { .. } => 59,
            Self::Proficiency => 60,
            Self::SendEvent => 61,
            Self::PowerBurn => 62,
            Self::Threat { .. } => 63,
            Self::TriggerSpell => 64,
            Self::HealthFunnel => 65,
            Self::PowerFunnel => 66,
            Self::HealMaxHealth { .. } => 67,
            Self::InterruptCast { .. } => 68,
            Self::Distract { .. } => 69,
            Self::Pull => 70,
            Self::Pickpocket => 71,
            Self::AddFarsight => 72,
            Self::SummonPossessed { .. } => 73,
            Self::SummonTotem { .. } => 74,
            Self::HealMechanical => 75,
            Self::SummonObjectWild { .. } => 76,
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
            Self::SummonTotemSlot1 { .. } => 87,
            Self::SummonTotemSlot2 { .. } => 88,
            Self::SummonTotemSlot3 { .. } => 89,
            Self::SummonTotemSlot4 { .. } => 90,
            Self::ThreatAll { .. } => 91,
            Self::EnchantHeldItem => 92,
            Self::SummonPhantasm => 93,
            Self::SelfResurrect => 94,
            Self::Skinning => 95,
            Self::Charge => 96,
            Self::SummonCritter { .. } => 97,
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
            Self::DispelMechanic { .. } => 108,
            Self::SummonDeadPet => 109,
            Self::DestroyAllTotems => 110,
            Self::DurabilityDamage { .. } => 111,
            Self::SummonDemon { .. } => 112,
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

impl std::fmt::Display for SpellLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Instakill{ .. } => f.write_str("Instakill"),
            Self::SchoolDamage => f.write_str("SchoolDamage"),
            Self::Dummy => f.write_str("Dummy"),
            Self::PortalTeleport => f.write_str("PortalTeleport"),
            Self::TeleportUnits => f.write_str("TeleportUnits"),
            Self::ApplyAura => f.write_str("ApplyAura"),
            Self::EnvironmentalDamage => f.write_str("EnvironmentalDamage"),
            Self::PowerDrain{ .. } => f.write_str("PowerDrain"),
            Self::HealthLeech => f.write_str("HealthLeech"),
            Self::Heal{ .. } => f.write_str("Heal"),
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
            Self::Energize{ .. } => f.write_str("Energize"),
            Self::WeaponPercentDamage => f.write_str("WeaponPercentDamage"),
            Self::TriggerMissile => f.write_str("TriggerMissile"),
            Self::OpenLock{ .. } => f.write_str("OpenLock"),
            Self::SummonChangeItem => f.write_str("SummonChangeItem"),
            Self::ApplyAreaAuraParty => f.write_str("ApplyAreaAuraParty"),
            Self::LearnSpell => f.write_str("LearnSpell"),
            Self::SpellDefense => f.write_str("SpellDefense"),
            Self::Dispel{ .. } => f.write_str("Dispel"),
            Self::Language => f.write_str("Language"),
            Self::DualWield => f.write_str("DualWield"),
            Self::SummonWild{ .. } => f.write_str("SummonWild"),
            Self::SummonGuardian{ .. } => f.write_str("SummonGuardian"),
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
            Self::OpenLockItem{ .. } => f.write_str("OpenLockItem"),
            Self::Proficiency => f.write_str("Proficiency"),
            Self::SendEvent => f.write_str("SendEvent"),
            Self::PowerBurn => f.write_str("PowerBurn"),
            Self::Threat{ .. } => f.write_str("Threat"),
            Self::TriggerSpell => f.write_str("TriggerSpell"),
            Self::HealthFunnel => f.write_str("HealthFunnel"),
            Self::PowerFunnel => f.write_str("PowerFunnel"),
            Self::HealMaxHealth{ .. } => f.write_str("HealMaxHealth"),
            Self::InterruptCast{ .. } => f.write_str("InterruptCast"),
            Self::Distract{ .. } => f.write_str("Distract"),
            Self::Pull => f.write_str("Pull"),
            Self::Pickpocket => f.write_str("Pickpocket"),
            Self::AddFarsight => f.write_str("AddFarsight"),
            Self::SummonPossessed{ .. } => f.write_str("SummonPossessed"),
            Self::SummonTotem{ .. } => f.write_str("SummonTotem"),
            Self::HealMechanical => f.write_str("HealMechanical"),
            Self::SummonObjectWild{ .. } => f.write_str("SummonObjectWild"),
            Self::ScriptEffect => f.write_str("ScriptEffect"),
            Self::Attack => f.write_str("Attack"),
            Self::Sanctuary{ .. } => f.write_str("Sanctuary"),
            Self::AddComboPoints => f.write_str("AddComboPoints"),
            Self::CreateHouse => f.write_str("CreateHouse"),
            Self::BindSight => f.write_str("BindSight"),
            Self::Duel => f.write_str("Duel"),
            Self::Stuck => f.write_str("Stuck"),
            Self::SummonPlayer => f.write_str("SummonPlayer"),
            Self::ActivateObject => f.write_str("ActivateObject"),
            Self::SummonTotemSlot1{ .. } => f.write_str("SummonTotemSlot1"),
            Self::SummonTotemSlot2{ .. } => f.write_str("SummonTotemSlot2"),
            Self::SummonTotemSlot3{ .. } => f.write_str("SummonTotemSlot3"),
            Self::SummonTotemSlot4{ .. } => f.write_str("SummonTotemSlot4"),
            Self::ThreatAll{ .. } => f.write_str("ThreatAll"),
            Self::EnchantHeldItem => f.write_str("EnchantHeldItem"),
            Self::SummonPhantasm => f.write_str("SummonPhantasm"),
            Self::SelfResurrect => f.write_str("SelfResurrect"),
            Self::Skinning => f.write_str("Skinning"),
            Self::Charge => f.write_str("Charge"),
            Self::SummonCritter{ .. } => f.write_str("SummonCritter"),
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
            Self::DispelMechanic{ .. } => f.write_str("DispelMechanic"),
            Self::SummonDeadPet => f.write_str("SummonDeadPet"),
            Self::DestroyAllTotems => f.write_str("DestroyAllTotems"),
            Self::DurabilityDamage{ .. } => f.write_str("DurabilityDamage"),
            Self::SummonDemon{ .. } => f.write_str("SummonDemon"),
            Self::ResurrectNew{ .. } => f.write_str("ResurrectNew"),
            Self::AttackMe{ .. } => f.write_str("AttackMe"),
            Self::DurabilityDamagePct => f.write_str("DurabilityDamagePct"),
            Self::SkinPlayerCorpse{ .. } => f.write_str("SkinPlayerCorpse"),
            Self::SpiritHeal => f.write_str("SpiritHeal"),
            Self::Skill => f.write_str("Skill"),
            Self::ApplyAreaAuraPet => f.write_str("ApplyAreaAuraPet"),
            Self::TeleportGraveyard => f.write_str("TeleportGraveyard"),
            Self::NormalizedWeaponDmg => f.write_str("NormalizedWeaponDmg"),
            Self::Unknown122 => f.write_str("Unknown122"),
            Self::SendTaxi => f.write_str("SendTaxi"),
            Self::PlayerPull => f.write_str("PlayerPull"),
            Self::ModifyThreatPercent{ .. } => f.write_str("ModifyThreatPercent"),
            Self::Unknown126{ .. } => f.write_str("Unknown126"),
            Self::Unknown127 => f.write_str("Unknown127"),
        }
    }
}

