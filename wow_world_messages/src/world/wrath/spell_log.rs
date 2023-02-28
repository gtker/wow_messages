use crate::Guid;
use crate::wrath::Power;
use crate::wrath::SpellEffect;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelllogexecute.wowm:616`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelllogexecute.wowm#L616):
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
///     else if (effect == CREATE_ITEM
///         || effect == CREATE_ITEM2) {
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
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // effect: SpellEffect
        w.write_all(&u32::from(self.effect.as_int()).to_le_bytes())?;

        // amount_of_logs: u32
        w.write_all(&Self::AMOUNT_OF_LOGS_VALUE.to_le_bytes())?;

        match &self.effect {
            SpellLog_SpellEffect::None => {}
            SpellLog_SpellEffect::Instakill => {}
            SpellLog_SpellEffect::SchoolDamage => {}
            SpellLog_SpellEffect::Dummy => {}
            SpellLog_SpellEffect::PortalTeleport => {}
            SpellLog_SpellEffect::TeleportUnits => {}
            SpellLog_SpellEffect::ApplyAura => {}
            SpellLog_SpellEffect::EnvironmentalDamage => {}
            SpellLog_SpellEffect::PowerDrain {
                amount,
                multiplier,
                power,
                target1,
            } => {
                // target1: PackedGuid
                target1.write_packed_guid_into_vec(&mut w)?;

                // amount: u32
                w.write_all(&amount.to_le_bytes())?;

                // power: Power
                w.write_all(&u32::from(power.as_int()).to_le_bytes())?;

                // multiplier: f32
                w.write_all(&multiplier.to_le_bytes())?;

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
                resurrect_guid,
            } => {
                // resurrect_guid: PackedGuid
                resurrect_guid.write_packed_guid_into_vec(&mut w)?;

            }
            SpellLog_SpellEffect::AddExtraAttacks {
                extra_attacks,
                target4,
            } => {
                // target4: PackedGuid
                target4.write_packed_guid_into_vec(&mut w)?;

                // extra_attacks: u32
                w.write_all(&extra_attacks.to_le_bytes())?;

            }
            SpellLog_SpellEffect::Dodge => {}
            SpellLog_SpellEffect::Evade => {}
            SpellLog_SpellEffect::Parry => {}
            SpellLog_SpellEffect::Block => {}
            SpellLog_SpellEffect::CreateItem {
                item,
            } => {
                // item: u32
                w.write_all(&item.to_le_bytes())?;

            }
            SpellLog_SpellEffect::Weapon => {}
            SpellLog_SpellEffect::Defense => {}
            SpellLog_SpellEffect::PersistentAreaAura => {}
            SpellLog_SpellEffect::Summon {
                summon_target,
            } => {
                // summon_target: PackedGuid
                summon_target.write_packed_guid_into_vec(&mut w)?;

            }
            SpellLog_SpellEffect::Leap => {}
            SpellLog_SpellEffect::Energize => {}
            SpellLog_SpellEffect::WeaponPercentDamage => {}
            SpellLog_SpellEffect::TriggerMissile => {}
            SpellLog_SpellEffect::OpenLock {
                lock_target,
            } => {
                // lock_target: PackedGuid
                lock_target.write_packed_guid_into_vec(&mut w)?;

            }
            SpellLog_SpellEffect::SummonChangeItem => {}
            SpellLog_SpellEffect::ApplyAreaAuraParty => {}
            SpellLog_SpellEffect::LearnSpell => {}
            SpellLog_SpellEffect::SpellDefense => {}
            SpellLog_SpellEffect::Dispel => {}
            SpellLog_SpellEffect::Language => {}
            SpellLog_SpellEffect::DualWield => {}
            SpellLog_SpellEffect::Jump => {}
            SpellLog_SpellEffect::Jump2 => {}
            SpellLog_SpellEffect::TeleportUnitsFaceCaster => {}
            SpellLog_SpellEffect::SkillStep => {}
            SpellLog_SpellEffect::AddHonor => {}
            SpellLog_SpellEffect::Spawn => {}
            SpellLog_SpellEffect::TradeSkill => {}
            SpellLog_SpellEffect::Stealth => {}
            SpellLog_SpellEffect::Detect => {}
            SpellLog_SpellEffect::TransDoor {
                summon_target,
            } => {
                // summon_target: PackedGuid
                summon_target.write_packed_guid_into_vec(&mut w)?;

            }
            SpellLog_SpellEffect::ForceCriticalHit => {}
            SpellLog_SpellEffect::GuaranteeHit => {}
            SpellLog_SpellEffect::EnchantItem => {}
            SpellLog_SpellEffect::EnchantItemTemporary => {}
            SpellLog_SpellEffect::Tamecreature => {}
            SpellLog_SpellEffect::SummonPet {
                summon_target,
            } => {
                // summon_target: PackedGuid
                summon_target.write_packed_guid_into_vec(&mut w)?;

            }
            SpellLog_SpellEffect::LearnPetSpell => {}
            SpellLog_SpellEffect::WeaponDamage => {}
            SpellLog_SpellEffect::CreateRandomItem => {}
            SpellLog_SpellEffect::Proficiency => {}
            SpellLog_SpellEffect::SendEvent => {}
            SpellLog_SpellEffect::PowerBurn => {}
            SpellLog_SpellEffect::Threat => {}
            SpellLog_SpellEffect::TriggerSpell => {}
            SpellLog_SpellEffect::ApplyAreaAuraRaid => {}
            SpellLog_SpellEffect::RestoreItemCharges => {}
            SpellLog_SpellEffect::HealMaxHealth => {}
            SpellLog_SpellEffect::InterruptCast {
                interrupted_spell,
                target5,
            } => {
                // target5: PackedGuid
                target5.write_packed_guid_into_vec(&mut w)?;

                // interrupted_spell: u32
                w.write_all(&interrupted_spell.to_le_bytes())?;

            }
            SpellLog_SpellEffect::Distract => {}
            SpellLog_SpellEffect::Pull => {}
            SpellLog_SpellEffect::Pickpocket => {}
            SpellLog_SpellEffect::AddFarsight => {}
            SpellLog_SpellEffect::UntrainTalents => {}
            SpellLog_SpellEffect::ApplyGlyph => {}
            SpellLog_SpellEffect::HealMechanical => {}
            SpellLog_SpellEffect::SummonObjectWild {
                summon_target,
            } => {
                // summon_target: PackedGuid
                summon_target.write_packed_guid_into_vec(&mut w)?;

            }
            SpellLog_SpellEffect::ScriptEffect => {}
            SpellLog_SpellEffect::Attack => {}
            SpellLog_SpellEffect::Sanctuary => {}
            SpellLog_SpellEffect::AddComboPoints => {}
            SpellLog_SpellEffect::CreateHouse {
                summon_target,
            } => {
                // summon_target: PackedGuid
                summon_target.write_packed_guid_into_vec(&mut w)?;

            }
            SpellLog_SpellEffect::BindSight => {}
            SpellLog_SpellEffect::Duel {
                summon_target,
            } => {
                // summon_target: PackedGuid
                summon_target.write_packed_guid_into_vec(&mut w)?;

            }
            SpellLog_SpellEffect::Stuck => {}
            SpellLog_SpellEffect::SummonPlayer => {}
            SpellLog_SpellEffect::ActivateObject => {}
            SpellLog_SpellEffect::WmoDamage => {}
            SpellLog_SpellEffect::WmoRepair => {}
            SpellLog_SpellEffect::WmoChange => {}
            SpellLog_SpellEffect::KillCreditPersonal => {}
            SpellLog_SpellEffect::ThreatAll => {}
            SpellLog_SpellEffect::EnchantHeldItem => {}
            SpellLog_SpellEffect::BreakPlayerTargeting => {}
            SpellLog_SpellEffect::SelfResurrect => {}
            SpellLog_SpellEffect::Skinning => {}
            SpellLog_SpellEffect::Charge => {}
            SpellLog_SpellEffect::SummonAllTotems => {}
            SpellLog_SpellEffect::KnockBack => {}
            SpellLog_SpellEffect::Disenchant => {}
            SpellLog_SpellEffect::Inebriate => {}
            SpellLog_SpellEffect::FeedPet {
                pet_feed_guid,
            } => {
                // pet_feed_guid: PackedGuid
                pet_feed_guid.write_packed_guid_into_vec(&mut w)?;

            }
            SpellLog_SpellEffect::DismissPet {
                pet_dismiss_guid,
            } => {
                // pet_dismiss_guid: PackedGuid
                pet_dismiss_guid.write_packed_guid_into_vec(&mut w)?;

            }
            SpellLog_SpellEffect::Reputation => {}
            SpellLog_SpellEffect::SummonObjectSlot1 {
                summon_target,
            } => {
                // summon_target: PackedGuid
                summon_target.write_packed_guid_into_vec(&mut w)?;

            }
            SpellLog_SpellEffect::SummonObjectSlot2 {
                summon_target,
            } => {
                // summon_target: PackedGuid
                summon_target.write_packed_guid_into_vec(&mut w)?;

            }
            SpellLog_SpellEffect::SummonObjectSlot3 {
                summon_target,
            } => {
                // summon_target: PackedGuid
                summon_target.write_packed_guid_into_vec(&mut w)?;

            }
            SpellLog_SpellEffect::SummonObjectSlot4 {
                summon_target,
            } => {
                // summon_target: PackedGuid
                summon_target.write_packed_guid_into_vec(&mut w)?;

            }
            SpellLog_SpellEffect::DispelMechanic => {}
            SpellLog_SpellEffect::SummonDeadPet => {}
            SpellLog_SpellEffect::DestroyAllTotems => {}
            SpellLog_SpellEffect::DurabilityDamage {
                item_to_damage,
                target6,
                unknown5,
            } => {
                // target6: PackedGuid
                target6.write_packed_guid_into_vec(&mut w)?;

                // item_to_damage: u32
                w.write_all(&item_to_damage.to_le_bytes())?;

                // unknown5: u32
                w.write_all(&unknown5.to_le_bytes())?;

            }
            SpellLog_SpellEffect::Unknown112 => {}
            SpellLog_SpellEffect::ResurrectNew {
                resurrect_guid,
            } => {
                // resurrect_guid: PackedGuid
                resurrect_guid.write_packed_guid_into_vec(&mut w)?;

            }
            SpellLog_SpellEffect::AttackMe => {}
            SpellLog_SpellEffect::DurabilityDamagePct => {}
            SpellLog_SpellEffect::SkinPlayerCorpse => {}
            SpellLog_SpellEffect::SpiritHeal => {}
            SpellLog_SpellEffect::Skill => {}
            SpellLog_SpellEffect::ApplyAreaAuraPet => {}
            SpellLog_SpellEffect::TeleportGraveyard => {}
            SpellLog_SpellEffect::NormalizedWeaponDmg => {}
            SpellLog_SpellEffect::Unknown122 => {}
            SpellLog_SpellEffect::SendTaxi => {}
            SpellLog_SpellEffect::PlayerPull => {}
            SpellLog_SpellEffect::ModifyThreatPercent => {}
            SpellLog_SpellEffect::StealBeneficialBuff => {}
            SpellLog_SpellEffect::Prospecting => {}
            SpellLog_SpellEffect::ApplyAreaAuraFriend => {}
            SpellLog_SpellEffect::ApplyAreaAuraEnemy => {}
            SpellLog_SpellEffect::RedirectThreat => {}
            SpellLog_SpellEffect::PlaySound => {}
            SpellLog_SpellEffect::PlayMusic => {}
            SpellLog_SpellEffect::UnlearnSpecialization => {}
            SpellLog_SpellEffect::KillCreditGroup => {}
            SpellLog_SpellEffect::CallPet => {}
            SpellLog_SpellEffect::HealPct => {}
            SpellLog_SpellEffect::EnergizePct => {}
            SpellLog_SpellEffect::LeapBack => {}
            SpellLog_SpellEffect::ClearQuest => {}
            SpellLog_SpellEffect::ForceCast => {}
            SpellLog_SpellEffect::ForceCastWithValue => {}
            SpellLog_SpellEffect::TriggerSpellWithValue => {}
            SpellLog_SpellEffect::ApplyAreaAuraOwner => {}
            SpellLog_SpellEffect::KnockbackFromPosition => {}
            SpellLog_SpellEffect::GravityPull => {}
            SpellLog_SpellEffect::ActivateRune => {}
            SpellLog_SpellEffect::QuestFail => {}
            SpellLog_SpellEffect::Unknown148 => {}
            SpellLog_SpellEffect::Charge2 => {}
            SpellLog_SpellEffect::QuestOffer => {}
            SpellLog_SpellEffect::TriggerSpell2 => {}
            SpellLog_SpellEffect::Unknown152 => {}
            SpellLog_SpellEffect::CreatePet => {}
            SpellLog_SpellEffect::TeachTaxiNode => {}
            SpellLog_SpellEffect::TitanGrip => {}
            SpellLog_SpellEffect::EnchantItemPrismatic => {}
            SpellLog_SpellEffect::CreateItem2 => {}
            SpellLog_SpellEffect::Milling => {}
            SpellLog_SpellEffect::AllowRenamePet => {}
            SpellLog_SpellEffect::Unknown160 => {}
            SpellLog_SpellEffect::TalentSpecCount => {}
            SpellLog_SpellEffect::TalentSpecSelect => {}
            SpellLog_SpellEffect::Unknown163 => {}
            SpellLog_SpellEffect::CancelAura => {}
        }

        Ok(())
    }
}

impl SpellLog {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
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
                let target1 = Guid::read_packed(&mut r)?;

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
            SpellEffect::Resurrect => {
                // resurrect_guid: PackedGuid
                let resurrect_guid = Guid::read_packed(&mut r)?;

                SpellLog_SpellEffect::Resurrect {
                    resurrect_guid,
                }
            }
            SpellEffect::AddExtraAttacks => {
                // target4: PackedGuid
                let target4 = Guid::read_packed(&mut r)?;

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
                let summon_target = Guid::read_packed(&mut r)?;

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
                let lock_target = Guid::read_packed(&mut r)?;

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
                let summon_target = Guid::read_packed(&mut r)?;

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
                let summon_target = Guid::read_packed(&mut r)?;

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
                let target5 = Guid::read_packed(&mut r)?;

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
            SpellEffect::UntrainTalents => SpellLog_SpellEffect::UntrainTalents,
            SpellEffect::ApplyGlyph => SpellLog_SpellEffect::ApplyGlyph,
            SpellEffect::HealMechanical => SpellLog_SpellEffect::HealMechanical,
            SpellEffect::SummonObjectWild => {
                // summon_target: PackedGuid
                let summon_target = Guid::read_packed(&mut r)?;

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
                let summon_target = Guid::read_packed(&mut r)?;

                SpellLog_SpellEffect::CreateHouse {
                    summon_target,
                }
            }
            SpellEffect::BindSight => SpellLog_SpellEffect::BindSight,
            SpellEffect::Duel => {
                // summon_target: PackedGuid
                let summon_target = Guid::read_packed(&mut r)?;

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
                let pet_feed_guid = Guid::read_packed(&mut r)?;

                SpellLog_SpellEffect::FeedPet {
                    pet_feed_guid,
                }
            }
            SpellEffect::DismissPet => {
                // pet_dismiss_guid: PackedGuid
                let pet_dismiss_guid = Guid::read_packed(&mut r)?;

                SpellLog_SpellEffect::DismissPet {
                    pet_dismiss_guid,
                }
            }
            SpellEffect::Reputation => SpellLog_SpellEffect::Reputation,
            SpellEffect::SummonObjectSlot1 => {
                // summon_target: PackedGuid
                let summon_target = Guid::read_packed(&mut r)?;

                SpellLog_SpellEffect::SummonObjectSlot1 {
                    summon_target,
                }
            }
            SpellEffect::SummonObjectSlot2 => {
                // summon_target: PackedGuid
                let summon_target = Guid::read_packed(&mut r)?;

                SpellLog_SpellEffect::SummonObjectSlot2 {
                    summon_target,
                }
            }
            SpellEffect::SummonObjectSlot3 => {
                // summon_target: PackedGuid
                let summon_target = Guid::read_packed(&mut r)?;

                SpellLog_SpellEffect::SummonObjectSlot3 {
                    summon_target,
                }
            }
            SpellEffect::SummonObjectSlot4 => {
                // summon_target: PackedGuid
                let summon_target = Guid::read_packed(&mut r)?;

                SpellLog_SpellEffect::SummonObjectSlot4 {
                    summon_target,
                }
            }
            SpellEffect::DispelMechanic => SpellLog_SpellEffect::DispelMechanic,
            SpellEffect::SummonDeadPet => SpellLog_SpellEffect::SummonDeadPet,
            SpellEffect::DestroyAllTotems => SpellLog_SpellEffect::DestroyAllTotems,
            SpellEffect::DurabilityDamage => {
                // target6: PackedGuid
                let target6 = Guid::read_packed(&mut r)?;

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
            SpellEffect::ResurrectNew => {
                // resurrect_guid: PackedGuid
                let resurrect_guid = Guid::read_packed(&mut r)?;

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
    pub(crate) fn size(&self) -> usize {
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

impl SpellLog_SpellEffect {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::None => {
                4
            }
            Self::Instakill => {
                4
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
                amount,
                multiplier,
                power,
                target1,
            } => {
                4
                + 4 // amount: u32
                + 4 // multiplier: f32
                + 4 // power: Power
                + target1.size() // target1: Guid
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
                resurrect_guid,
            } => {
                4
                + resurrect_guid.size() // resurrect_guid: Guid
            }
            Self::AddExtraAttacks {
                extra_attacks,
                target4,
            } => {
                4
                + 4 // extra_attacks: u32
                + target4.size() // target4: Guid
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
                item,
            } => {
                4
                + 4 // item: u32
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
            Self::Summon {
                summon_target,
            } => {
                4
                + summon_target.size() // summon_target: Guid
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
                lock_target,
            } => {
                4
                + lock_target.size() // lock_target: Guid
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
            Self::Dispel => {
                4
            }
            Self::Language => {
                4
            }
            Self::DualWield => {
                4
            }
            Self::Jump => {
                4
            }
            Self::Jump2 => {
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
            Self::TransDoor {
                summon_target,
            } => {
                4
                + summon_target.size() // summon_target: Guid
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
            Self::SummonPet {
                summon_target,
            } => {
                4
                + summon_target.size() // summon_target: Guid
            }
            Self::LearnPetSpell => {
                4
            }
            Self::WeaponDamage => {
                4
            }
            Self::CreateRandomItem => {
                4
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
            Self::Threat => {
                4
            }
            Self::TriggerSpell => {
                4
            }
            Self::ApplyAreaAuraRaid => {
                4
            }
            Self::RestoreItemCharges => {
                4
            }
            Self::HealMaxHealth => {
                4
            }
            Self::InterruptCast {
                interrupted_spell,
                target5,
            } => {
                4
                + 4 // interrupted_spell: u32
                + target5.size() // target5: Guid
            }
            Self::Distract => {
                4
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
            Self::UntrainTalents => {
                4
            }
            Self::ApplyGlyph => {
                4
            }
            Self::HealMechanical => {
                4
            }
            Self::SummonObjectWild {
                summon_target,
            } => {
                4
                + summon_target.size() // summon_target: Guid
            }
            Self::ScriptEffect => {
                4
            }
            Self::Attack => {
                4
            }
            Self::Sanctuary => {
                4
            }
            Self::AddComboPoints => {
                4
            }
            Self::CreateHouse {
                summon_target,
            } => {
                4
                + summon_target.size() // summon_target: Guid
            }
            Self::BindSight => {
                4
            }
            Self::Duel {
                summon_target,
            } => {
                4
                + summon_target.size() // summon_target: Guid
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
            Self::WmoDamage => {
                4
            }
            Self::WmoRepair => {
                4
            }
            Self::WmoChange => {
                4
            }
            Self::KillCreditPersonal => {
                4
            }
            Self::ThreatAll => {
                4
            }
            Self::EnchantHeldItem => {
                4
            }
            Self::BreakPlayerTargeting => {
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
            Self::SummonAllTotems => {
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
                pet_feed_guid,
            } => {
                4
                + pet_feed_guid.size() // pet_feed_guid: Guid
            }
            Self::DismissPet {
                pet_dismiss_guid,
            } => {
                4
                + pet_dismiss_guid.size() // pet_dismiss_guid: Guid
            }
            Self::Reputation => {
                4
            }
            Self::SummonObjectSlot1 {
                summon_target,
            } => {
                4
                + summon_target.size() // summon_target: Guid
            }
            Self::SummonObjectSlot2 {
                summon_target,
            } => {
                4
                + summon_target.size() // summon_target: Guid
            }
            Self::SummonObjectSlot3 {
                summon_target,
            } => {
                4
                + summon_target.size() // summon_target: Guid
            }
            Self::SummonObjectSlot4 {
                summon_target,
            } => {
                4
                + summon_target.size() // summon_target: Guid
            }
            Self::DispelMechanic => {
                4
            }
            Self::SummonDeadPet => {
                4
            }
            Self::DestroyAllTotems => {
                4
            }
            Self::DurabilityDamage {
                item_to_damage,
                target6,
                unknown5,
            } => {
                4
                + 4 // item_to_damage: u32
                + target6.size() // target6: Guid
                + 4 // unknown5: u32
            }
            Self::Unknown112 => {
                4
            }
            Self::ResurrectNew {
                resurrect_guid,
            } => {
                4
                + resurrect_guid.size() // resurrect_guid: Guid
            }
            Self::AttackMe => {
                4
            }
            Self::DurabilityDamagePct => {
                4
            }
            Self::SkinPlayerCorpse => {
                4
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
            Self::ModifyThreatPercent => {
                4
            }
            Self::StealBeneficialBuff => {
                4
            }
            Self::Prospecting => {
                4
            }
            Self::ApplyAreaAuraFriend => {
                4
            }
            Self::ApplyAreaAuraEnemy => {
                4
            }
            Self::RedirectThreat => {
                4
            }
            Self::PlaySound => {
                4
            }
            Self::PlayMusic => {
                4
            }
            Self::UnlearnSpecialization => {
                4
            }
            Self::KillCreditGroup => {
                4
            }
            Self::CallPet => {
                4
            }
            Self::HealPct => {
                4
            }
            Self::EnergizePct => {
                4
            }
            Self::LeapBack => {
                4
            }
            Self::ClearQuest => {
                4
            }
            Self::ForceCast => {
                4
            }
            Self::ForceCastWithValue => {
                4
            }
            Self::TriggerSpellWithValue => {
                4
            }
            Self::ApplyAreaAuraOwner => {
                4
            }
            Self::KnockbackFromPosition => {
                4
            }
            Self::GravityPull => {
                4
            }
            Self::ActivateRune => {
                4
            }
            Self::QuestFail => {
                4
            }
            Self::Unknown148 => {
                4
            }
            Self::Charge2 => {
                4
            }
            Self::QuestOffer => {
                4
            }
            Self::TriggerSpell2 => {
                4
            }
            Self::Unknown152 => {
                4
            }
            Self::CreatePet => {
                4
            }
            Self::TeachTaxiNode => {
                4
            }
            Self::TitanGrip => {
                4
            }
            Self::EnchantItemPrismatic => {
                4
            }
            Self::CreateItem2 => {
                4
            }
            Self::Milling => {
                4
            }
            Self::AllowRenamePet => {
                4
            }
            Self::Unknown160 => {
                4
            }
            Self::TalentSpecCount => {
                4
            }
            Self::TalentSpecSelect => {
                4
            }
            Self::Unknown163 => {
                4
            }
            Self::CancelAura => {
                4
            }
        }
    }
}

