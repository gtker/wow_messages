use std::io::{Read, Write};

use crate::wrath::{
    AuraType, SpellSchool,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm:956`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm#L956):
/// ```text
/// struct AuraLog {
///     AuraType aura_type;
///     if (aura_type == PERIODIC_DAMAGE
///         || aura_type == PERIODIC_DAMAGE_PERCENT) {
///         u32 damage1;
///         u32 overkill_damage;
///         SpellSchool school;
///         u32 absorb1;
///         u32 resisted;
///         Bool critical1;
///     }
///     else if (aura_type == PERIODIC_HEAL
///         || aura_type == OBS_MOD_HEALTH) {
///         u32 damage2;
///         u32 over_damage;
///         u32 absorb2;
///         Bool critical2;
///     }
///     else if (aura_type == OBS_MOD_MANA
///         || aura_type == PERIODIC_ENERGIZE) {
///         u32 misc_value1;
///         u32 damage3;
///     }
///     else if (aura_type == PERIODIC_MANA_LEECH) {
///         u32 misc_value2;
///         u32 damage4;
///         f32 gain_multiplier;
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct AuraLog {
    pub aura_type: AuraLog_AuraType,
}

impl AuraLog {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // aura_type: AuraType
        w.write_all(&(self.aura_type.as_int().to_le_bytes()))?;

        match &self.aura_type {
            AuraLog_AuraType::PeriodicDamage {
                absorb1,
                critical1,
                damage1,
                overkill_damage,
                resisted,
                school,
            } => {
                // damage1: u32
                w.write_all(&damage1.to_le_bytes())?;

                // overkill_damage: u32
                w.write_all(&overkill_damage.to_le_bytes())?;

                // school: SpellSchool
                w.write_all(&(school.as_int().to_le_bytes()))?;

                // absorb1: u32
                w.write_all(&absorb1.to_le_bytes())?;

                // resisted: u32
                w.write_all(&resisted.to_le_bytes())?;

                // critical1: Bool
                w.write_all(u8::from(*critical1).to_le_bytes().as_slice())?;

            }
            AuraLog_AuraType::PeriodicHeal {
                absorb2,
                critical2,
                damage2,
                over_damage,
            } => {
                // damage2: u32
                w.write_all(&damage2.to_le_bytes())?;

                // over_damage: u32
                w.write_all(&over_damage.to_le_bytes())?;

                // absorb2: u32
                w.write_all(&absorb2.to_le_bytes())?;

                // critical2: Bool
                w.write_all(u8::from(*critical2).to_le_bytes().as_slice())?;

            }
            AuraLog_AuraType::ObsModHealth {
                absorb2,
                critical2,
                damage2,
                over_damage,
            } => {
                // damage2: u32
                w.write_all(&damage2.to_le_bytes())?;

                // over_damage: u32
                w.write_all(&over_damage.to_le_bytes())?;

                // absorb2: u32
                w.write_all(&absorb2.to_le_bytes())?;

                // critical2: Bool
                w.write_all(u8::from(*critical2).to_le_bytes().as_slice())?;

            }
            AuraLog_AuraType::PeriodicEnergize {
                damage3,
                misc_value1,
            } => {
                // misc_value1: u32
                w.write_all(&misc_value1.to_le_bytes())?;

                // damage3: u32
                w.write_all(&damage3.to_le_bytes())?;

            }
            AuraLog_AuraType::PeriodicManaLeech {
                damage4,
                gain_multiplier,
                misc_value2,
            } => {
                // misc_value2: u32
                w.write_all(&misc_value2.to_le_bytes())?;

                // damage4: u32
                w.write_all(&damage4.to_le_bytes())?;

                // gain_multiplier: f32
                w.write_all(&gain_multiplier.to_le_bytes())?;

            }
            AuraLog_AuraType::PeriodicDamagePercent {
                absorb1,
                critical1,
                damage1,
                overkill_damage,
                resisted,
                school,
            } => {
                // damage1: u32
                w.write_all(&damage1.to_le_bytes())?;

                // overkill_damage: u32
                w.write_all(&overkill_damage.to_le_bytes())?;

                // school: SpellSchool
                w.write_all(&(school.as_int().to_le_bytes()))?;

                // absorb1: u32
                w.write_all(&absorb1.to_le_bytes())?;

                // resisted: u32
                w.write_all(&resisted.to_le_bytes())?;

                // critical1: Bool
                w.write_all(u8::from(*critical1).to_le_bytes().as_slice())?;

            }
            _ => {}
        }

        Ok(())
    }
}

impl AuraLog {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // aura_type: AuraType
        let aura_type = crate::util::read_u32_le(&mut r)?.try_into()?;

        let aura_type_if = match aura_type {
            AuraType::None => AuraLog_AuraType::None,
            AuraType::BindSight => AuraLog_AuraType::BindSight,
            AuraType::ModPossess => AuraLog_AuraType::ModPossess,
            AuraType::PeriodicDamage => {
                // damage1: u32
                let damage1 = crate::util::read_u32_le(&mut r)?;

                // overkill_damage: u32
                let overkill_damage = crate::util::read_u32_le(&mut r)?;

                // school: SpellSchool
                let school = crate::util::read_u8_le(&mut r)?.try_into()?;

                // absorb1: u32
                let absorb1 = crate::util::read_u32_le(&mut r)?;

                // resisted: u32
                let resisted = crate::util::read_u32_le(&mut r)?;

                // critical1: Bool
                let critical1 = crate::util::read_bool_u8(&mut r)?;

                AuraLog_AuraType::PeriodicDamage {
                    absorb1,
                    critical1,
                    damage1,
                    overkill_damage,
                    resisted,
                    school,
                }
            }
            AuraType::Dummy => AuraLog_AuraType::Dummy,
            AuraType::ModConfuse => AuraLog_AuraType::ModConfuse,
            AuraType::ModCharm => AuraLog_AuraType::ModCharm,
            AuraType::ModFear => AuraLog_AuraType::ModFear,
            AuraType::PeriodicHeal => {
                // damage2: u32
                let damage2 = crate::util::read_u32_le(&mut r)?;

                // over_damage: u32
                let over_damage = crate::util::read_u32_le(&mut r)?;

                // absorb2: u32
                let absorb2 = crate::util::read_u32_le(&mut r)?;

                // critical2: Bool
                let critical2 = crate::util::read_bool_u8(&mut r)?;

                AuraLog_AuraType::PeriodicHeal {
                    absorb2,
                    critical2,
                    damage2,
                    over_damage,
                }
            }
            AuraType::ModAttackspeed => AuraLog_AuraType::ModAttackspeed,
            AuraType::ModThreat => AuraLog_AuraType::ModThreat,
            AuraType::ModTaunt => AuraLog_AuraType::ModTaunt,
            AuraType::ModStun => AuraLog_AuraType::ModStun,
            AuraType::ModDamageDone => AuraLog_AuraType::ModDamageDone,
            AuraType::ModDamageTaken => AuraLog_AuraType::ModDamageTaken,
            AuraType::DamageShield => AuraLog_AuraType::DamageShield,
            AuraType::ModStealth => AuraLog_AuraType::ModStealth,
            AuraType::ModStealthDetect => AuraLog_AuraType::ModStealthDetect,
            AuraType::ModInvisibility => AuraLog_AuraType::ModInvisibility,
            AuraType::ModInvisibilityDetect => AuraLog_AuraType::ModInvisibilityDetect,
            AuraType::ObsModHealth => {
                // damage2: u32
                let damage2 = crate::util::read_u32_le(&mut r)?;

                // over_damage: u32
                let over_damage = crate::util::read_u32_le(&mut r)?;

                // absorb2: u32
                let absorb2 = crate::util::read_u32_le(&mut r)?;

                // critical2: Bool
                let critical2 = crate::util::read_bool_u8(&mut r)?;

                AuraLog_AuraType::ObsModHealth {
                    absorb2,
                    critical2,
                    damage2,
                    over_damage,
                }
            }
            AuraType::ObsModPower => AuraLog_AuraType::ObsModPower,
            AuraType::ModResistance => AuraLog_AuraType::ModResistance,
            AuraType::PeriodicTriggerSpell => AuraLog_AuraType::PeriodicTriggerSpell,
            AuraType::PeriodicEnergize => {
                // misc_value1: u32
                let misc_value1 = crate::util::read_u32_le(&mut r)?;

                // damage3: u32
                let damage3 = crate::util::read_u32_le(&mut r)?;

                AuraLog_AuraType::PeriodicEnergize {
                    damage3,
                    misc_value1,
                }
            }
            AuraType::ModPacify => AuraLog_AuraType::ModPacify,
            AuraType::ModRoot => AuraLog_AuraType::ModRoot,
            AuraType::ModSilence => AuraLog_AuraType::ModSilence,
            AuraType::ReflectSpells => AuraLog_AuraType::ReflectSpells,
            AuraType::ModStat => AuraLog_AuraType::ModStat,
            AuraType::ModSkill => AuraLog_AuraType::ModSkill,
            AuraType::ModIncreaseSpeed => AuraLog_AuraType::ModIncreaseSpeed,
            AuraType::ModIncreaseMountedSpeed => AuraLog_AuraType::ModIncreaseMountedSpeed,
            AuraType::ModDecreaseSpeed => AuraLog_AuraType::ModDecreaseSpeed,
            AuraType::ModIncreaseHealth => AuraLog_AuraType::ModIncreaseHealth,
            AuraType::ModIncreaseEnergy => AuraLog_AuraType::ModIncreaseEnergy,
            AuraType::ModShapeshift => AuraLog_AuraType::ModShapeshift,
            AuraType::EffectImmunity => AuraLog_AuraType::EffectImmunity,
            AuraType::StateImmunity => AuraLog_AuraType::StateImmunity,
            AuraType::SchoolImmunity => AuraLog_AuraType::SchoolImmunity,
            AuraType::DamageImmunity => AuraLog_AuraType::DamageImmunity,
            AuraType::DispelImmunity => AuraLog_AuraType::DispelImmunity,
            AuraType::ProcTriggerSpell => AuraLog_AuraType::ProcTriggerSpell,
            AuraType::ProcTriggerDamage => AuraLog_AuraType::ProcTriggerDamage,
            AuraType::TrackCreatures => AuraLog_AuraType::TrackCreatures,
            AuraType::TrackResources => AuraLog_AuraType::TrackResources,
            AuraType::Unknown46 => AuraLog_AuraType::Unknown46,
            AuraType::ModParryPercent => AuraLog_AuraType::ModParryPercent,
            AuraType::PeriodicTriggerSpellFromClient => AuraLog_AuraType::PeriodicTriggerSpellFromClient,
            AuraType::ModDodgePercent => AuraLog_AuraType::ModDodgePercent,
            AuraType::ModCriticalHealingAmount => AuraLog_AuraType::ModCriticalHealingAmount,
            AuraType::ModBlockPercent => AuraLog_AuraType::ModBlockPercent,
            AuraType::ModWeaponCritPercent => AuraLog_AuraType::ModWeaponCritPercent,
            AuraType::PeriodicLeech => AuraLog_AuraType::PeriodicLeech,
            AuraType::ModHitChance => AuraLog_AuraType::ModHitChance,
            AuraType::ModSpellHitChance => AuraLog_AuraType::ModSpellHitChance,
            AuraType::Transform => AuraLog_AuraType::Transform,
            AuraType::ModSpellCritChance => AuraLog_AuraType::ModSpellCritChance,
            AuraType::ModIncreaseSwimSpeed => AuraLog_AuraType::ModIncreaseSwimSpeed,
            AuraType::ModDamageDoneCreature => AuraLog_AuraType::ModDamageDoneCreature,
            AuraType::ModPacifySilence => AuraLog_AuraType::ModPacifySilence,
            AuraType::ModScale => AuraLog_AuraType::ModScale,
            AuraType::PeriodicHealthFunnel => AuraLog_AuraType::PeriodicHealthFunnel,
            AuraType::Unknown63 => AuraLog_AuraType::Unknown63,
            AuraType::PeriodicManaLeech => {
                // misc_value2: u32
                let misc_value2 = crate::util::read_u32_le(&mut r)?;

                // damage4: u32
                let damage4 = crate::util::read_u32_le(&mut r)?;

                // gain_multiplier: f32
                let gain_multiplier = crate::util::read_f32_le(&mut r)?;

                AuraLog_AuraType::PeriodicManaLeech {
                    damage4,
                    gain_multiplier,
                    misc_value2,
                }
            }
            AuraType::ModCastingSpeedNotStack => AuraLog_AuraType::ModCastingSpeedNotStack,
            AuraType::FeignDeath => AuraLog_AuraType::FeignDeath,
            AuraType::ModDisarm => AuraLog_AuraType::ModDisarm,
            AuraType::ModStalked => AuraLog_AuraType::ModStalked,
            AuraType::SchoolAbsorb => AuraLog_AuraType::SchoolAbsorb,
            AuraType::ExtraAttacks => AuraLog_AuraType::ExtraAttacks,
            AuraType::ModSpellCritChanceSchool => AuraLog_AuraType::ModSpellCritChanceSchool,
            AuraType::ModPowerCostSchoolPct => AuraLog_AuraType::ModPowerCostSchoolPct,
            AuraType::ModPowerCostSchool => AuraLog_AuraType::ModPowerCostSchool,
            AuraType::ReflectSpellsSchool => AuraLog_AuraType::ReflectSpellsSchool,
            AuraType::ModLanguage => AuraLog_AuraType::ModLanguage,
            AuraType::FarSight => AuraLog_AuraType::FarSight,
            AuraType::MechanicImmunity => AuraLog_AuraType::MechanicImmunity,
            AuraType::Mounted => AuraLog_AuraType::Mounted,
            AuraType::ModDamagePercentDone => AuraLog_AuraType::ModDamagePercentDone,
            AuraType::ModPercentStat => AuraLog_AuraType::ModPercentStat,
            AuraType::SplitDamagePct => AuraLog_AuraType::SplitDamagePct,
            AuraType::WaterBreathing => AuraLog_AuraType::WaterBreathing,
            AuraType::ModBaseResistance => AuraLog_AuraType::ModBaseResistance,
            AuraType::ModRegen => AuraLog_AuraType::ModRegen,
            AuraType::ModPowerRegen => AuraLog_AuraType::ModPowerRegen,
            AuraType::ChannelDeathItem => AuraLog_AuraType::ChannelDeathItem,
            AuraType::ModDamagePercentTaken => AuraLog_AuraType::ModDamagePercentTaken,
            AuraType::ModHealthRegenPercent => AuraLog_AuraType::ModHealthRegenPercent,
            AuraType::PeriodicDamagePercent => {
                // damage1: u32
                let damage1 = crate::util::read_u32_le(&mut r)?;

                // overkill_damage: u32
                let overkill_damage = crate::util::read_u32_le(&mut r)?;

                // school: SpellSchool
                let school = crate::util::read_u8_le(&mut r)?.try_into()?;

                // absorb1: u32
                let absorb1 = crate::util::read_u32_le(&mut r)?;

                // resisted: u32
                let resisted = crate::util::read_u32_le(&mut r)?;

                // critical1: Bool
                let critical1 = crate::util::read_bool_u8(&mut r)?;

                AuraLog_AuraType::PeriodicDamagePercent {
                    absorb1,
                    critical1,
                    damage1,
                    overkill_damage,
                    resisted,
                    school,
                }
            }
            AuraType::Unknown90 => AuraLog_AuraType::Unknown90,
            AuraType::ModDetectRange => AuraLog_AuraType::ModDetectRange,
            AuraType::PreventsFleeing => AuraLog_AuraType::PreventsFleeing,
            AuraType::ModUnattackable => AuraLog_AuraType::ModUnattackable,
            AuraType::InterruptRegen => AuraLog_AuraType::InterruptRegen,
            AuraType::Ghost => AuraLog_AuraType::Ghost,
            AuraType::SpellMagnet => AuraLog_AuraType::SpellMagnet,
            AuraType::ManaShield => AuraLog_AuraType::ManaShield,
            AuraType::ModSkillTalent => AuraLog_AuraType::ModSkillTalent,
            AuraType::ModAttackPower => AuraLog_AuraType::ModAttackPower,
            AuraType::AurasVisible => AuraLog_AuraType::AurasVisible,
            AuraType::ModResistancePct => AuraLog_AuraType::ModResistancePct,
            AuraType::ModMeleeAttackPowerVersus => AuraLog_AuraType::ModMeleeAttackPowerVersus,
            AuraType::ModTotalThreat => AuraLog_AuraType::ModTotalThreat,
            AuraType::WaterWalk => AuraLog_AuraType::WaterWalk,
            AuraType::FeatherFall => AuraLog_AuraType::FeatherFall,
            AuraType::Hover => AuraLog_AuraType::Hover,
            AuraType::AddFlatModifier => AuraLog_AuraType::AddFlatModifier,
            AuraType::AddPctModifier => AuraLog_AuraType::AddPctModifier,
            AuraType::AddTargetTrigger => AuraLog_AuraType::AddTargetTrigger,
            AuraType::ModPowerRegenPercent => AuraLog_AuraType::ModPowerRegenPercent,
            AuraType::AddCasterHitTrigger => AuraLog_AuraType::AddCasterHitTrigger,
            AuraType::OverrideClassScripts => AuraLog_AuraType::OverrideClassScripts,
            AuraType::ModRangedDamageTaken => AuraLog_AuraType::ModRangedDamageTaken,
            AuraType::ModRangedDamageTakenPct => AuraLog_AuraType::ModRangedDamageTakenPct,
            AuraType::ModHealing => AuraLog_AuraType::ModHealing,
            AuraType::ModRegenDuringCombat => AuraLog_AuraType::ModRegenDuringCombat,
            AuraType::ModMechanicResistance => AuraLog_AuraType::ModMechanicResistance,
            AuraType::ModHealingPct => AuraLog_AuraType::ModHealingPct,
            AuraType::Unknown119 => AuraLog_AuraType::Unknown119,
            AuraType::Untrackable => AuraLog_AuraType::Untrackable,
            AuraType::Empathy => AuraLog_AuraType::Empathy,
            AuraType::ModOffhandDamagePct => AuraLog_AuraType::ModOffhandDamagePct,
            AuraType::ModTargetResistance => AuraLog_AuraType::ModTargetResistance,
            AuraType::ModRangedAttackPower => AuraLog_AuraType::ModRangedAttackPower,
            AuraType::ModMeleeDamageTaken => AuraLog_AuraType::ModMeleeDamageTaken,
            AuraType::ModMeleeDamageTakenPct => AuraLog_AuraType::ModMeleeDamageTakenPct,
            AuraType::RangedAttackPowerAttackerBonus => AuraLog_AuraType::RangedAttackPowerAttackerBonus,
            AuraType::ModPossessPet => AuraLog_AuraType::ModPossessPet,
            AuraType::ModSpeedAlways => AuraLog_AuraType::ModSpeedAlways,
            AuraType::ModMountedSpeedAlways => AuraLog_AuraType::ModMountedSpeedAlways,
            AuraType::ModRangedAttackPowerVersus => AuraLog_AuraType::ModRangedAttackPowerVersus,
            AuraType::ModIncreaseEnergyPercent => AuraLog_AuraType::ModIncreaseEnergyPercent,
            AuraType::ModIncreaseHealthPercent => AuraLog_AuraType::ModIncreaseHealthPercent,
            AuraType::ModManaRegenInterrupt => AuraLog_AuraType::ModManaRegenInterrupt,
            AuraType::ModHealingDone => AuraLog_AuraType::ModHealingDone,
            AuraType::ModHealingDonePercent => AuraLog_AuraType::ModHealingDonePercent,
            AuraType::ModTotalStatPercentage => AuraLog_AuraType::ModTotalStatPercentage,
            AuraType::ModMeleeHaste => AuraLog_AuraType::ModMeleeHaste,
            AuraType::ForceReaction => AuraLog_AuraType::ForceReaction,
            AuraType::ModRangedHaste => AuraLog_AuraType::ModRangedHaste,
            AuraType::ModRangedAmmoHaste => AuraLog_AuraType::ModRangedAmmoHaste,
            AuraType::ModBaseResistancePct => AuraLog_AuraType::ModBaseResistancePct,
            AuraType::ModResistanceExclusive => AuraLog_AuraType::ModResistanceExclusive,
            AuraType::SafeFall => AuraLog_AuraType::SafeFall,
            AuraType::ModPetTalentPoints => AuraLog_AuraType::ModPetTalentPoints,
            AuraType::AllowTamePetType => AuraLog_AuraType::AllowTamePetType,
            AuraType::MechanicImmunityMask => AuraLog_AuraType::MechanicImmunityMask,
            AuraType::RetainComboPoints => AuraLog_AuraType::RetainComboPoints,
            AuraType::ReducePushback => AuraLog_AuraType::ReducePushback,
            AuraType::ModShieldBlockvaluePct => AuraLog_AuraType::ModShieldBlockvaluePct,
            AuraType::TrackStealthed => AuraLog_AuraType::TrackStealthed,
            AuraType::ModDetectedRange => AuraLog_AuraType::ModDetectedRange,
            AuraType::SplitDamageFlat => AuraLog_AuraType::SplitDamageFlat,
            AuraType::ModStealthLevel => AuraLog_AuraType::ModStealthLevel,
            AuraType::ModWaterBreathing => AuraLog_AuraType::ModWaterBreathing,
            AuraType::ModReputationGain => AuraLog_AuraType::ModReputationGain,
            AuraType::PetDamageMulti => AuraLog_AuraType::PetDamageMulti,
            AuraType::ModShieldBlockvalue => AuraLog_AuraType::ModShieldBlockvalue,
            AuraType::NoPvpCredit => AuraLog_AuraType::NoPvpCredit,
            AuraType::ModAoeAvoidance => AuraLog_AuraType::ModAoeAvoidance,
            AuraType::ModHealthRegenInCombat => AuraLog_AuraType::ModHealthRegenInCombat,
            AuraType::PowerBurn => AuraLog_AuraType::PowerBurn,
            AuraType::ModCritDamageBonus => AuraLog_AuraType::ModCritDamageBonus,
            AuraType::Unknown164 => AuraLog_AuraType::Unknown164,
            AuraType::MeleeAttackPowerAttackerBonus => AuraLog_AuraType::MeleeAttackPowerAttackerBonus,
            AuraType::ModAttackPowerPct => AuraLog_AuraType::ModAttackPowerPct,
            AuraType::ModRangedAttackPowerPct => AuraLog_AuraType::ModRangedAttackPowerPct,
            AuraType::ModDamageDoneVersus => AuraLog_AuraType::ModDamageDoneVersus,
            AuraType::ModCritPercentVersus => AuraLog_AuraType::ModCritPercentVersus,
            AuraType::DetectAmore => AuraLog_AuraType::DetectAmore,
            AuraType::ModSpeedNotStack => AuraLog_AuraType::ModSpeedNotStack,
            AuraType::ModMountedSpeedNotStack => AuraLog_AuraType::ModMountedSpeedNotStack,
            AuraType::Unknown173 => AuraLog_AuraType::Unknown173,
            AuraType::ModSpellDamageOfStatPercent => AuraLog_AuraType::ModSpellDamageOfStatPercent,
            AuraType::ModSpellHealingOfStatPercent => AuraLog_AuraType::ModSpellHealingOfStatPercent,
            AuraType::SpiritOfRedemption => AuraLog_AuraType::SpiritOfRedemption,
            AuraType::AoeCharm => AuraLog_AuraType::AoeCharm,
            AuraType::ModDebuffResistance => AuraLog_AuraType::ModDebuffResistance,
            AuraType::ModAttackerSpellCritChance => AuraLog_AuraType::ModAttackerSpellCritChance,
            AuraType::ModFlatSpellDamageVersus => AuraLog_AuraType::ModFlatSpellDamageVersus,
            AuraType::Unknown181 => AuraLog_AuraType::Unknown181,
            AuraType::ModResistanceOfStatPercent => AuraLog_AuraType::ModResistanceOfStatPercent,
            AuraType::ModCriticalThreat => AuraLog_AuraType::ModCriticalThreat,
            AuraType::ModAttackerMeleeHitChance => AuraLog_AuraType::ModAttackerMeleeHitChance,
            AuraType::ModAttackerRangedHitChance => AuraLog_AuraType::ModAttackerRangedHitChance,
            AuraType::ModAttackerSpellHitChance => AuraLog_AuraType::ModAttackerSpellHitChance,
            AuraType::ModAttackerMeleeCritChance => AuraLog_AuraType::ModAttackerMeleeCritChance,
            AuraType::ModAttackerRangedCritChance => AuraLog_AuraType::ModAttackerRangedCritChance,
            AuraType::ModRating => AuraLog_AuraType::ModRating,
            AuraType::ModFactionReputationGain => AuraLog_AuraType::ModFactionReputationGain,
            AuraType::UseNormalMovementSpeed => AuraLog_AuraType::UseNormalMovementSpeed,
            AuraType::ModMeleeRangedHaste => AuraLog_AuraType::ModMeleeRangedHaste,
            AuraType::MeleeSlow => AuraLog_AuraType::MeleeSlow,
            AuraType::ModTargetAbsorbSchool => AuraLog_AuraType::ModTargetAbsorbSchool,
            AuraType::ModTargetAbilityAbsorbSchool => AuraLog_AuraType::ModTargetAbilityAbsorbSchool,
            AuraType::ModCooldown => AuraLog_AuraType::ModCooldown,
            AuraType::ModAttackerSpellAndWeaponCritChance => AuraLog_AuraType::ModAttackerSpellAndWeaponCritChance,
            AuraType::Unknown198 => AuraLog_AuraType::Unknown198,
            AuraType::ModIncreasesSpellPctToHit => AuraLog_AuraType::ModIncreasesSpellPctToHit,
            AuraType::ModXpPct => AuraLog_AuraType::ModXpPct,
            AuraType::Fly => AuraLog_AuraType::Fly,
            AuraType::IgnoreCombatResult => AuraLog_AuraType::IgnoreCombatResult,
            AuraType::ModAttackerMeleeCritDamage => AuraLog_AuraType::ModAttackerMeleeCritDamage,
            AuraType::ModAttackerRangedCritDamage => AuraLog_AuraType::ModAttackerRangedCritDamage,
            AuraType::ModSchoolCritDmgTaken => AuraLog_AuraType::ModSchoolCritDmgTaken,
            AuraType::ModIncreaseVehicleFlightSpeed => AuraLog_AuraType::ModIncreaseVehicleFlightSpeed,
            AuraType::ModIncreaseMountedFlightSpeed => AuraLog_AuraType::ModIncreaseMountedFlightSpeed,
            AuraType::ModIncreaseFlightSpeed => AuraLog_AuraType::ModIncreaseFlightSpeed,
            AuraType::ModMountedFlightSpeedAlways => AuraLog_AuraType::ModMountedFlightSpeedAlways,
            AuraType::ModVehicleSpeedAlways => AuraLog_AuraType::ModVehicleSpeedAlways,
            AuraType::ModFlightSpeedNotStack => AuraLog_AuraType::ModFlightSpeedNotStack,
            AuraType::ModRangedAttackPowerOfStatPercent => AuraLog_AuraType::ModRangedAttackPowerOfStatPercent,
            AuraType::ModRageFromDamageDealt => AuraLog_AuraType::ModRageFromDamageDealt,
            AuraType::Unknown214 => AuraLog_AuraType::Unknown214,
            AuraType::ArenaPreparation => AuraLog_AuraType::ArenaPreparation,
            AuraType::HasteSpells => AuraLog_AuraType::HasteSpells,
            AuraType::ModMeleeHaste2 => AuraLog_AuraType::ModMeleeHaste2,
            AuraType::HasteRanged => AuraLog_AuraType::HasteRanged,
            AuraType::ModManaRegenFromStat => AuraLog_AuraType::ModManaRegenFromStat,
            AuraType::ModRatingFromStat => AuraLog_AuraType::ModRatingFromStat,
            AuraType::ModDetaunt => AuraLog_AuraType::ModDetaunt,
            AuraType::Unknown222 => AuraLog_AuraType::Unknown222,
            AuraType::RaidProcFromCharge => AuraLog_AuraType::RaidProcFromCharge,
            AuraType::Unknown224 => AuraLog_AuraType::Unknown224,
            AuraType::RaidProcFromChargeWithValue => AuraLog_AuraType::RaidProcFromChargeWithValue,
            AuraType::PeriodicDummy => AuraLog_AuraType::PeriodicDummy,
            AuraType::PeriodicTriggerSpellWithValue => AuraLog_AuraType::PeriodicTriggerSpellWithValue,
            AuraType::DetectStealth => AuraLog_AuraType::DetectStealth,
            AuraType::ModAoeDamageAvoidance => AuraLog_AuraType::ModAoeDamageAvoidance,
            AuraType::Unknown230 => AuraLog_AuraType::Unknown230,
            AuraType::ProcTriggerSpellWithValue => AuraLog_AuraType::ProcTriggerSpellWithValue,
            AuraType::MechanicDurationMod => AuraLog_AuraType::MechanicDurationMod,
            AuraType::ChangeModelForAllHumanoids => AuraLog_AuraType::ChangeModelForAllHumanoids,
            AuraType::MechanicDurationModNotStack => AuraLog_AuraType::MechanicDurationModNotStack,
            AuraType::ModDispelResist => AuraLog_AuraType::ModDispelResist,
            AuraType::ControlVehicle => AuraLog_AuraType::ControlVehicle,
            AuraType::ModSpellDamageOfAttackPower => AuraLog_AuraType::ModSpellDamageOfAttackPower,
            AuraType::ModSpellHealingOfAttackPower => AuraLog_AuraType::ModSpellHealingOfAttackPower,
            AuraType::ModScale2 => AuraLog_AuraType::ModScale2,
            AuraType::ModExpertise => AuraLog_AuraType::ModExpertise,
            AuraType::ForceMoveForward => AuraLog_AuraType::ForceMoveForward,
            AuraType::ModSpellDamageFromHealing => AuraLog_AuraType::ModSpellDamageFromHealing,
            AuraType::ModFaction => AuraLog_AuraType::ModFaction,
            AuraType::ComprehendLanguage => AuraLog_AuraType::ComprehendLanguage,
            AuraType::ModAuraDurationByDispel => AuraLog_AuraType::ModAuraDurationByDispel,
            AuraType::ModAuraDurationByDispelNotStack => AuraLog_AuraType::ModAuraDurationByDispelNotStack,
            AuraType::CloneCaster => AuraLog_AuraType::CloneCaster,
            AuraType::ModCombatResultChance => AuraLog_AuraType::ModCombatResultChance,
            AuraType::ConvertRune => AuraLog_AuraType::ConvertRune,
            AuraType::ModIncreaseHealth2 => AuraLog_AuraType::ModIncreaseHealth2,
            AuraType::ModEnemyDodge => AuraLog_AuraType::ModEnemyDodge,
            AuraType::ModSpeedSlowAll => AuraLog_AuraType::ModSpeedSlowAll,
            AuraType::ModBlockCritChance => AuraLog_AuraType::ModBlockCritChance,
            AuraType::ModDisarmOffhand => AuraLog_AuraType::ModDisarmOffhand,
            AuraType::ModMechanicDamageTakenPercent => AuraLog_AuraType::ModMechanicDamageTakenPercent,
            AuraType::NoReagentUse => AuraLog_AuraType::NoReagentUse,
            AuraType::ModTargetResistBySpellClass => AuraLog_AuraType::ModTargetResistBySpellClass,
            AuraType::Unknown258 => AuraLog_AuraType::Unknown258,
            AuraType::ModHotPct => AuraLog_AuraType::ModHotPct,
            AuraType::ScreenEffect => AuraLog_AuraType::ScreenEffect,
            AuraType::Phase => AuraLog_AuraType::Phase,
            AuraType::AbilityIgnoreAurastate => AuraLog_AuraType::AbilityIgnoreAurastate,
            AuraType::AllowOnlyAbility => AuraLog_AuraType::AllowOnlyAbility,
            AuraType::Unknown264 => AuraLog_AuraType::Unknown264,
            AuraType::Unknown265 => AuraLog_AuraType::Unknown265,
            AuraType::Unknown266 => AuraLog_AuraType::Unknown266,
            AuraType::ModImmuneAuraApplySchool => AuraLog_AuraType::ModImmuneAuraApplySchool,
            AuraType::ModAttackPowerOfStatPercent => AuraLog_AuraType::ModAttackPowerOfStatPercent,
            AuraType::ModIgnoreTargetResist => AuraLog_AuraType::ModIgnoreTargetResist,
            AuraType::ModAbilityIgnoreTargetResist => AuraLog_AuraType::ModAbilityIgnoreTargetResist,
            AuraType::ModDamageFromCaster => AuraLog_AuraType::ModDamageFromCaster,
            AuraType::IgnoreMeleeReset => AuraLog_AuraType::IgnoreMeleeReset,
            AuraType::XRay => AuraLog_AuraType::XRay,
            AuraType::AbilityConsumeNoAmmo => AuraLog_AuraType::AbilityConsumeNoAmmo,
            AuraType::ModIgnoreShapeshift => AuraLog_AuraType::ModIgnoreShapeshift,
            AuraType::ModDamageDoneForMechanic => AuraLog_AuraType::ModDamageDoneForMechanic,
            AuraType::ModMaxAffectedTargets => AuraLog_AuraType::ModMaxAffectedTargets,
            AuraType::ModDisarmRanged => AuraLog_AuraType::ModDisarmRanged,
            AuraType::InitializeImages => AuraLog_AuraType::InitializeImages,
            AuraType::ModArmorPenetrationPct => AuraLog_AuraType::ModArmorPenetrationPct,
            AuraType::ModHonorGainPct => AuraLog_AuraType::ModHonorGainPct,
            AuraType::ModBaseHealthPct => AuraLog_AuraType::ModBaseHealthPct,
            AuraType::ModHealingReceived => AuraLog_AuraType::ModHealingReceived,
            AuraType::Linked => AuraLog_AuraType::Linked,
            AuraType::ModAttackPowerOfArmor => AuraLog_AuraType::ModAttackPowerOfArmor,
            AuraType::AbilityPeriodicCrit => AuraLog_AuraType::AbilityPeriodicCrit,
            AuraType::DeflectSpells => AuraLog_AuraType::DeflectSpells,
            AuraType::IgnoreHitDirection => AuraLog_AuraType::IgnoreHitDirection,
            AuraType::PreventDurabilityLoss => AuraLog_AuraType::PreventDurabilityLoss,
            AuraType::ModCritPct => AuraLog_AuraType::ModCritPct,
            AuraType::ModXpQuestPct => AuraLog_AuraType::ModXpQuestPct,
            AuraType::OpenStable => AuraLog_AuraType::OpenStable,
            AuraType::OverrideSpells => AuraLog_AuraType::OverrideSpells,
            AuraType::PreventRegeneratePower => AuraLog_AuraType::PreventRegeneratePower,
            AuraType::Unknown295 => AuraLog_AuraType::Unknown295,
            AuraType::SetVehicleId => AuraLog_AuraType::SetVehicleId,
            AuraType::BlockSpellFamily => AuraLog_AuraType::BlockSpellFamily,
            AuraType::Strangulate => AuraLog_AuraType::Strangulate,
            AuraType::Unknown299 => AuraLog_AuraType::Unknown299,
            AuraType::ShareDamagePct => AuraLog_AuraType::ShareDamagePct,
            AuraType::SchoolHealAbsorb => AuraLog_AuraType::SchoolHealAbsorb,
            AuraType::Unknown302 => AuraLog_AuraType::Unknown302,
            AuraType::ModDamageDoneVersusAurastate => AuraLog_AuraType::ModDamageDoneVersusAurastate,
            AuraType::ModFakeInebriate => AuraLog_AuraType::ModFakeInebriate,
            AuraType::ModMinimumSpeed => AuraLog_AuraType::ModMinimumSpeed,
            AuraType::Unknown306 => AuraLog_AuraType::Unknown306,
            AuraType::HealAbsorbTest => AuraLog_AuraType::HealAbsorbTest,
            AuraType::ModCritChanceForCaster => AuraLog_AuraType::ModCritChanceForCaster,
            AuraType::Unknown309 => AuraLog_AuraType::Unknown309,
            AuraType::ModCreatureAoeDamageAvoidance => AuraLog_AuraType::ModCreatureAoeDamageAvoidance,
            AuraType::Unknown311 => AuraLog_AuraType::Unknown311,
            AuraType::Unknown312 => AuraLog_AuraType::Unknown312,
            AuraType::Unknown313 => AuraLog_AuraType::Unknown313,
            AuraType::PreventResurrection => AuraLog_AuraType::PreventResurrection,
            AuraType::UnderwaterWalking => AuraLog_AuraType::UnderwaterWalking,
            AuraType::PeriodicHaste => AuraLog_AuraType::PeriodicHaste,
        };

        Ok(Self {
            aura_type: aura_type_if,
        })
    }

}

impl AuraLog {
    pub(crate) const fn size(&self) -> usize {
        self.aura_type.size() // aura_type: AuraLog_AuraType
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum AuraLog_AuraType {
    None,
    BindSight,
    ModPossess,
    PeriodicDamage {
        absorb1: u32,
        critical1: bool,
        damage1: u32,
        overkill_damage: u32,
        resisted: u32,
        school: SpellSchool,
    },
    Dummy,
    ModConfuse,
    ModCharm,
    ModFear,
    PeriodicHeal {
        absorb2: u32,
        critical2: bool,
        damage2: u32,
        over_damage: u32,
    },
    ModAttackspeed,
    ModThreat,
    ModTaunt,
    ModStun,
    ModDamageDone,
    ModDamageTaken,
    DamageShield,
    ModStealth,
    ModStealthDetect,
    ModInvisibility,
    ModInvisibilityDetect,
    ObsModHealth {
        absorb2: u32,
        critical2: bool,
        damage2: u32,
        over_damage: u32,
    },
    ObsModPower,
    ModResistance,
    PeriodicTriggerSpell,
    PeriodicEnergize {
        damage3: u32,
        misc_value1: u32,
    },
    ModPacify,
    ModRoot,
    ModSilence,
    ReflectSpells,
    ModStat,
    ModSkill,
    ModIncreaseSpeed,
    ModIncreaseMountedSpeed,
    ModDecreaseSpeed,
    ModIncreaseHealth,
    ModIncreaseEnergy,
    ModShapeshift,
    EffectImmunity,
    StateImmunity,
    SchoolImmunity,
    DamageImmunity,
    DispelImmunity,
    ProcTriggerSpell,
    ProcTriggerDamage,
    TrackCreatures,
    TrackResources,
    Unknown46,
    ModParryPercent,
    PeriodicTriggerSpellFromClient,
    ModDodgePercent,
    ModCriticalHealingAmount,
    ModBlockPercent,
    ModWeaponCritPercent,
    PeriodicLeech,
    ModHitChance,
    ModSpellHitChance,
    Transform,
    ModSpellCritChance,
    ModIncreaseSwimSpeed,
    ModDamageDoneCreature,
    ModPacifySilence,
    ModScale,
    PeriodicHealthFunnel,
    Unknown63,
    PeriodicManaLeech {
        damage4: u32,
        gain_multiplier: f32,
        misc_value2: u32,
    },
    ModCastingSpeedNotStack,
    FeignDeath,
    ModDisarm,
    ModStalked,
    SchoolAbsorb,
    ExtraAttacks,
    ModSpellCritChanceSchool,
    ModPowerCostSchoolPct,
    ModPowerCostSchool,
    ReflectSpellsSchool,
    ModLanguage,
    FarSight,
    MechanicImmunity,
    Mounted,
    ModDamagePercentDone,
    ModPercentStat,
    SplitDamagePct,
    WaterBreathing,
    ModBaseResistance,
    ModRegen,
    ModPowerRegen,
    ChannelDeathItem,
    ModDamagePercentTaken,
    ModHealthRegenPercent,
    PeriodicDamagePercent {
        absorb1: u32,
        critical1: bool,
        damage1: u32,
        overkill_damage: u32,
        resisted: u32,
        school: SpellSchool,
    },
    Unknown90,
    ModDetectRange,
    PreventsFleeing,
    ModUnattackable,
    InterruptRegen,
    Ghost,
    SpellMagnet,
    ManaShield,
    ModSkillTalent,
    ModAttackPower,
    AurasVisible,
    ModResistancePct,
    ModMeleeAttackPowerVersus,
    ModTotalThreat,
    WaterWalk,
    FeatherFall,
    Hover,
    AddFlatModifier,
    AddPctModifier,
    AddTargetTrigger,
    ModPowerRegenPercent,
    AddCasterHitTrigger,
    OverrideClassScripts,
    ModRangedDamageTaken,
    ModRangedDamageTakenPct,
    ModHealing,
    ModRegenDuringCombat,
    ModMechanicResistance,
    ModHealingPct,
    Unknown119,
    Untrackable,
    Empathy,
    ModOffhandDamagePct,
    ModTargetResistance,
    ModRangedAttackPower,
    ModMeleeDamageTaken,
    ModMeleeDamageTakenPct,
    RangedAttackPowerAttackerBonus,
    ModPossessPet,
    ModSpeedAlways,
    ModMountedSpeedAlways,
    ModRangedAttackPowerVersus,
    ModIncreaseEnergyPercent,
    ModIncreaseHealthPercent,
    ModManaRegenInterrupt,
    ModHealingDone,
    ModHealingDonePercent,
    ModTotalStatPercentage,
    ModMeleeHaste,
    ForceReaction,
    ModRangedHaste,
    ModRangedAmmoHaste,
    ModBaseResistancePct,
    ModResistanceExclusive,
    SafeFall,
    ModPetTalentPoints,
    AllowTamePetType,
    MechanicImmunityMask,
    RetainComboPoints,
    ReducePushback,
    ModShieldBlockvaluePct,
    TrackStealthed,
    ModDetectedRange,
    SplitDamageFlat,
    ModStealthLevel,
    ModWaterBreathing,
    ModReputationGain,
    PetDamageMulti,
    ModShieldBlockvalue,
    NoPvpCredit,
    ModAoeAvoidance,
    ModHealthRegenInCombat,
    PowerBurn,
    ModCritDamageBonus,
    Unknown164,
    MeleeAttackPowerAttackerBonus,
    ModAttackPowerPct,
    ModRangedAttackPowerPct,
    ModDamageDoneVersus,
    ModCritPercentVersus,
    DetectAmore,
    ModSpeedNotStack,
    ModMountedSpeedNotStack,
    Unknown173,
    ModSpellDamageOfStatPercent,
    ModSpellHealingOfStatPercent,
    SpiritOfRedemption,
    AoeCharm,
    ModDebuffResistance,
    ModAttackerSpellCritChance,
    ModFlatSpellDamageVersus,
    Unknown181,
    ModResistanceOfStatPercent,
    ModCriticalThreat,
    ModAttackerMeleeHitChance,
    ModAttackerRangedHitChance,
    ModAttackerSpellHitChance,
    ModAttackerMeleeCritChance,
    ModAttackerRangedCritChance,
    ModRating,
    ModFactionReputationGain,
    UseNormalMovementSpeed,
    ModMeleeRangedHaste,
    MeleeSlow,
    ModTargetAbsorbSchool,
    ModTargetAbilityAbsorbSchool,
    ModCooldown,
    ModAttackerSpellAndWeaponCritChance,
    Unknown198,
    ModIncreasesSpellPctToHit,
    ModXpPct,
    Fly,
    IgnoreCombatResult,
    ModAttackerMeleeCritDamage,
    ModAttackerRangedCritDamage,
    ModSchoolCritDmgTaken,
    ModIncreaseVehicleFlightSpeed,
    ModIncreaseMountedFlightSpeed,
    ModIncreaseFlightSpeed,
    ModMountedFlightSpeedAlways,
    ModVehicleSpeedAlways,
    ModFlightSpeedNotStack,
    ModRangedAttackPowerOfStatPercent,
    ModRageFromDamageDealt,
    Unknown214,
    ArenaPreparation,
    HasteSpells,
    ModMeleeHaste2,
    HasteRanged,
    ModManaRegenFromStat,
    ModRatingFromStat,
    ModDetaunt,
    Unknown222,
    RaidProcFromCharge,
    Unknown224,
    RaidProcFromChargeWithValue,
    PeriodicDummy,
    PeriodicTriggerSpellWithValue,
    DetectStealth,
    ModAoeDamageAvoidance,
    Unknown230,
    ProcTriggerSpellWithValue,
    MechanicDurationMod,
    ChangeModelForAllHumanoids,
    MechanicDurationModNotStack,
    ModDispelResist,
    ControlVehicle,
    ModSpellDamageOfAttackPower,
    ModSpellHealingOfAttackPower,
    ModScale2,
    ModExpertise,
    ForceMoveForward,
    ModSpellDamageFromHealing,
    ModFaction,
    ComprehendLanguage,
    ModAuraDurationByDispel,
    ModAuraDurationByDispelNotStack,
    CloneCaster,
    ModCombatResultChance,
    ConvertRune,
    ModIncreaseHealth2,
    ModEnemyDodge,
    ModSpeedSlowAll,
    ModBlockCritChance,
    ModDisarmOffhand,
    ModMechanicDamageTakenPercent,
    NoReagentUse,
    ModTargetResistBySpellClass,
    Unknown258,
    ModHotPct,
    ScreenEffect,
    Phase,
    AbilityIgnoreAurastate,
    AllowOnlyAbility,
    Unknown264,
    Unknown265,
    Unknown266,
    ModImmuneAuraApplySchool,
    ModAttackPowerOfStatPercent,
    ModIgnoreTargetResist,
    ModAbilityIgnoreTargetResist,
    ModDamageFromCaster,
    IgnoreMeleeReset,
    XRay,
    AbilityConsumeNoAmmo,
    ModIgnoreShapeshift,
    ModDamageDoneForMechanic,
    ModMaxAffectedTargets,
    ModDisarmRanged,
    InitializeImages,
    ModArmorPenetrationPct,
    ModHonorGainPct,
    ModBaseHealthPct,
    ModHealingReceived,
    Linked,
    ModAttackPowerOfArmor,
    AbilityPeriodicCrit,
    DeflectSpells,
    IgnoreHitDirection,
    PreventDurabilityLoss,
    ModCritPct,
    ModXpQuestPct,
    OpenStable,
    OverrideSpells,
    PreventRegeneratePower,
    Unknown295,
    SetVehicleId,
    BlockSpellFamily,
    Strangulate,
    Unknown299,
    ShareDamagePct,
    SchoolHealAbsorb,
    Unknown302,
    ModDamageDoneVersusAurastate,
    ModFakeInebriate,
    ModMinimumSpeed,
    Unknown306,
    HealAbsorbTest,
    ModCritChanceForCaster,
    Unknown309,
    ModCreatureAoeDamageAvoidance,
    Unknown311,
    Unknown312,
    Unknown313,
    PreventResurrection,
    UnderwaterWalking,
    PeriodicHaste,
}

impl Default for AuraLog_AuraType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::None
    }
}

impl AuraLog_AuraType {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::BindSight => 1,
            Self::ModPossess => 2,
            Self::PeriodicDamage { .. } => 3,
            Self::Dummy => 4,
            Self::ModConfuse => 5,
            Self::ModCharm => 6,
            Self::ModFear => 7,
            Self::PeriodicHeal { .. } => 8,
            Self::ModAttackspeed => 9,
            Self::ModThreat => 10,
            Self::ModTaunt => 11,
            Self::ModStun => 12,
            Self::ModDamageDone => 13,
            Self::ModDamageTaken => 14,
            Self::DamageShield => 15,
            Self::ModStealth => 16,
            Self::ModStealthDetect => 17,
            Self::ModInvisibility => 18,
            Self::ModInvisibilityDetect => 19,
            Self::ObsModHealth { .. } => 20,
            Self::ObsModPower => 21,
            Self::ModResistance => 22,
            Self::PeriodicTriggerSpell => 23,
            Self::PeriodicEnergize { .. } => 24,
            Self::ModPacify => 25,
            Self::ModRoot => 26,
            Self::ModSilence => 27,
            Self::ReflectSpells => 28,
            Self::ModStat => 29,
            Self::ModSkill => 30,
            Self::ModIncreaseSpeed => 31,
            Self::ModIncreaseMountedSpeed => 32,
            Self::ModDecreaseSpeed => 33,
            Self::ModIncreaseHealth => 34,
            Self::ModIncreaseEnergy => 35,
            Self::ModShapeshift => 36,
            Self::EffectImmunity => 37,
            Self::StateImmunity => 38,
            Self::SchoolImmunity => 39,
            Self::DamageImmunity => 40,
            Self::DispelImmunity => 41,
            Self::ProcTriggerSpell => 42,
            Self::ProcTriggerDamage => 43,
            Self::TrackCreatures => 44,
            Self::TrackResources => 45,
            Self::Unknown46 => 46,
            Self::ModParryPercent => 47,
            Self::PeriodicTriggerSpellFromClient => 48,
            Self::ModDodgePercent => 49,
            Self::ModCriticalHealingAmount => 50,
            Self::ModBlockPercent => 51,
            Self::ModWeaponCritPercent => 52,
            Self::PeriodicLeech => 53,
            Self::ModHitChance => 54,
            Self::ModSpellHitChance => 55,
            Self::Transform => 56,
            Self::ModSpellCritChance => 57,
            Self::ModIncreaseSwimSpeed => 58,
            Self::ModDamageDoneCreature => 59,
            Self::ModPacifySilence => 60,
            Self::ModScale => 61,
            Self::PeriodicHealthFunnel => 62,
            Self::Unknown63 => 63,
            Self::PeriodicManaLeech { .. } => 64,
            Self::ModCastingSpeedNotStack => 65,
            Self::FeignDeath => 66,
            Self::ModDisarm => 67,
            Self::ModStalked => 68,
            Self::SchoolAbsorb => 69,
            Self::ExtraAttacks => 70,
            Self::ModSpellCritChanceSchool => 71,
            Self::ModPowerCostSchoolPct => 72,
            Self::ModPowerCostSchool => 73,
            Self::ReflectSpellsSchool => 74,
            Self::ModLanguage => 75,
            Self::FarSight => 76,
            Self::MechanicImmunity => 77,
            Self::Mounted => 78,
            Self::ModDamagePercentDone => 79,
            Self::ModPercentStat => 80,
            Self::SplitDamagePct => 81,
            Self::WaterBreathing => 82,
            Self::ModBaseResistance => 83,
            Self::ModRegen => 84,
            Self::ModPowerRegen => 85,
            Self::ChannelDeathItem => 86,
            Self::ModDamagePercentTaken => 87,
            Self::ModHealthRegenPercent => 88,
            Self::PeriodicDamagePercent { .. } => 89,
            Self::Unknown90 => 90,
            Self::ModDetectRange => 91,
            Self::PreventsFleeing => 92,
            Self::ModUnattackable => 93,
            Self::InterruptRegen => 94,
            Self::Ghost => 95,
            Self::SpellMagnet => 96,
            Self::ManaShield => 97,
            Self::ModSkillTalent => 98,
            Self::ModAttackPower => 99,
            Self::AurasVisible => 100,
            Self::ModResistancePct => 101,
            Self::ModMeleeAttackPowerVersus => 102,
            Self::ModTotalThreat => 103,
            Self::WaterWalk => 104,
            Self::FeatherFall => 105,
            Self::Hover => 106,
            Self::AddFlatModifier => 107,
            Self::AddPctModifier => 108,
            Self::AddTargetTrigger => 109,
            Self::ModPowerRegenPercent => 110,
            Self::AddCasterHitTrigger => 111,
            Self::OverrideClassScripts => 112,
            Self::ModRangedDamageTaken => 113,
            Self::ModRangedDamageTakenPct => 114,
            Self::ModHealing => 115,
            Self::ModRegenDuringCombat => 116,
            Self::ModMechanicResistance => 117,
            Self::ModHealingPct => 118,
            Self::Unknown119 => 119,
            Self::Untrackable => 120,
            Self::Empathy => 121,
            Self::ModOffhandDamagePct => 122,
            Self::ModTargetResistance => 123,
            Self::ModRangedAttackPower => 124,
            Self::ModMeleeDamageTaken => 125,
            Self::ModMeleeDamageTakenPct => 126,
            Self::RangedAttackPowerAttackerBonus => 127,
            Self::ModPossessPet => 128,
            Self::ModSpeedAlways => 129,
            Self::ModMountedSpeedAlways => 130,
            Self::ModRangedAttackPowerVersus => 131,
            Self::ModIncreaseEnergyPercent => 132,
            Self::ModIncreaseHealthPercent => 133,
            Self::ModManaRegenInterrupt => 134,
            Self::ModHealingDone => 135,
            Self::ModHealingDonePercent => 136,
            Self::ModTotalStatPercentage => 137,
            Self::ModMeleeHaste => 138,
            Self::ForceReaction => 139,
            Self::ModRangedHaste => 140,
            Self::ModRangedAmmoHaste => 141,
            Self::ModBaseResistancePct => 142,
            Self::ModResistanceExclusive => 143,
            Self::SafeFall => 144,
            Self::ModPetTalentPoints => 145,
            Self::AllowTamePetType => 146,
            Self::MechanicImmunityMask => 147,
            Self::RetainComboPoints => 148,
            Self::ReducePushback => 149,
            Self::ModShieldBlockvaluePct => 150,
            Self::TrackStealthed => 151,
            Self::ModDetectedRange => 152,
            Self::SplitDamageFlat => 153,
            Self::ModStealthLevel => 154,
            Self::ModWaterBreathing => 155,
            Self::ModReputationGain => 156,
            Self::PetDamageMulti => 157,
            Self::ModShieldBlockvalue => 158,
            Self::NoPvpCredit => 159,
            Self::ModAoeAvoidance => 160,
            Self::ModHealthRegenInCombat => 161,
            Self::PowerBurn => 162,
            Self::ModCritDamageBonus => 163,
            Self::Unknown164 => 164,
            Self::MeleeAttackPowerAttackerBonus => 165,
            Self::ModAttackPowerPct => 166,
            Self::ModRangedAttackPowerPct => 167,
            Self::ModDamageDoneVersus => 168,
            Self::ModCritPercentVersus => 169,
            Self::DetectAmore => 170,
            Self::ModSpeedNotStack => 171,
            Self::ModMountedSpeedNotStack => 172,
            Self::Unknown173 => 173,
            Self::ModSpellDamageOfStatPercent => 174,
            Self::ModSpellHealingOfStatPercent => 175,
            Self::SpiritOfRedemption => 176,
            Self::AoeCharm => 177,
            Self::ModDebuffResistance => 178,
            Self::ModAttackerSpellCritChance => 179,
            Self::ModFlatSpellDamageVersus => 180,
            Self::Unknown181 => 181,
            Self::ModResistanceOfStatPercent => 182,
            Self::ModCriticalThreat => 183,
            Self::ModAttackerMeleeHitChance => 184,
            Self::ModAttackerRangedHitChance => 185,
            Self::ModAttackerSpellHitChance => 186,
            Self::ModAttackerMeleeCritChance => 187,
            Self::ModAttackerRangedCritChance => 188,
            Self::ModRating => 189,
            Self::ModFactionReputationGain => 190,
            Self::UseNormalMovementSpeed => 191,
            Self::ModMeleeRangedHaste => 192,
            Self::MeleeSlow => 193,
            Self::ModTargetAbsorbSchool => 194,
            Self::ModTargetAbilityAbsorbSchool => 195,
            Self::ModCooldown => 196,
            Self::ModAttackerSpellAndWeaponCritChance => 197,
            Self::Unknown198 => 198,
            Self::ModIncreasesSpellPctToHit => 199,
            Self::ModXpPct => 200,
            Self::Fly => 201,
            Self::IgnoreCombatResult => 202,
            Self::ModAttackerMeleeCritDamage => 203,
            Self::ModAttackerRangedCritDamage => 204,
            Self::ModSchoolCritDmgTaken => 205,
            Self::ModIncreaseVehicleFlightSpeed => 206,
            Self::ModIncreaseMountedFlightSpeed => 207,
            Self::ModIncreaseFlightSpeed => 208,
            Self::ModMountedFlightSpeedAlways => 209,
            Self::ModVehicleSpeedAlways => 210,
            Self::ModFlightSpeedNotStack => 211,
            Self::ModRangedAttackPowerOfStatPercent => 212,
            Self::ModRageFromDamageDealt => 213,
            Self::Unknown214 => 214,
            Self::ArenaPreparation => 215,
            Self::HasteSpells => 216,
            Self::ModMeleeHaste2 => 217,
            Self::HasteRanged => 218,
            Self::ModManaRegenFromStat => 219,
            Self::ModRatingFromStat => 220,
            Self::ModDetaunt => 221,
            Self::Unknown222 => 222,
            Self::RaidProcFromCharge => 223,
            Self::Unknown224 => 224,
            Self::RaidProcFromChargeWithValue => 225,
            Self::PeriodicDummy => 226,
            Self::PeriodicTriggerSpellWithValue => 227,
            Self::DetectStealth => 228,
            Self::ModAoeDamageAvoidance => 229,
            Self::Unknown230 => 230,
            Self::ProcTriggerSpellWithValue => 231,
            Self::MechanicDurationMod => 232,
            Self::ChangeModelForAllHumanoids => 233,
            Self::MechanicDurationModNotStack => 234,
            Self::ModDispelResist => 235,
            Self::ControlVehicle => 236,
            Self::ModSpellDamageOfAttackPower => 237,
            Self::ModSpellHealingOfAttackPower => 238,
            Self::ModScale2 => 239,
            Self::ModExpertise => 240,
            Self::ForceMoveForward => 241,
            Self::ModSpellDamageFromHealing => 242,
            Self::ModFaction => 243,
            Self::ComprehendLanguage => 244,
            Self::ModAuraDurationByDispel => 245,
            Self::ModAuraDurationByDispelNotStack => 246,
            Self::CloneCaster => 247,
            Self::ModCombatResultChance => 248,
            Self::ConvertRune => 249,
            Self::ModIncreaseHealth2 => 250,
            Self::ModEnemyDodge => 251,
            Self::ModSpeedSlowAll => 252,
            Self::ModBlockCritChance => 253,
            Self::ModDisarmOffhand => 254,
            Self::ModMechanicDamageTakenPercent => 255,
            Self::NoReagentUse => 256,
            Self::ModTargetResistBySpellClass => 257,
            Self::Unknown258 => 258,
            Self::ModHotPct => 259,
            Self::ScreenEffect => 260,
            Self::Phase => 261,
            Self::AbilityIgnoreAurastate => 262,
            Self::AllowOnlyAbility => 263,
            Self::Unknown264 => 264,
            Self::Unknown265 => 265,
            Self::Unknown266 => 266,
            Self::ModImmuneAuraApplySchool => 267,
            Self::ModAttackPowerOfStatPercent => 268,
            Self::ModIgnoreTargetResist => 269,
            Self::ModAbilityIgnoreTargetResist => 270,
            Self::ModDamageFromCaster => 271,
            Self::IgnoreMeleeReset => 272,
            Self::XRay => 273,
            Self::AbilityConsumeNoAmmo => 274,
            Self::ModIgnoreShapeshift => 275,
            Self::ModDamageDoneForMechanic => 276,
            Self::ModMaxAffectedTargets => 277,
            Self::ModDisarmRanged => 278,
            Self::InitializeImages => 279,
            Self::ModArmorPenetrationPct => 280,
            Self::ModHonorGainPct => 281,
            Self::ModBaseHealthPct => 282,
            Self::ModHealingReceived => 283,
            Self::Linked => 284,
            Self::ModAttackPowerOfArmor => 285,
            Self::AbilityPeriodicCrit => 286,
            Self::DeflectSpells => 287,
            Self::IgnoreHitDirection => 288,
            Self::PreventDurabilityLoss => 289,
            Self::ModCritPct => 290,
            Self::ModXpQuestPct => 291,
            Self::OpenStable => 292,
            Self::OverrideSpells => 293,
            Self::PreventRegeneratePower => 294,
            Self::Unknown295 => 295,
            Self::SetVehicleId => 296,
            Self::BlockSpellFamily => 297,
            Self::Strangulate => 298,
            Self::Unknown299 => 299,
            Self::ShareDamagePct => 300,
            Self::SchoolHealAbsorb => 301,
            Self::Unknown302 => 302,
            Self::ModDamageDoneVersusAurastate => 303,
            Self::ModFakeInebriate => 304,
            Self::ModMinimumSpeed => 305,
            Self::Unknown306 => 306,
            Self::HealAbsorbTest => 307,
            Self::ModCritChanceForCaster => 308,
            Self::Unknown309 => 309,
            Self::ModCreatureAoeDamageAvoidance => 310,
            Self::Unknown311 => 311,
            Self::Unknown312 => 312,
            Self::Unknown313 => 313,
            Self::PreventResurrection => 314,
            Self::UnderwaterWalking => 315,
            Self::PeriodicHaste => 316,
        }
    }

}

impl std::fmt::Display for AuraLog_AuraType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::BindSight => f.write_str("BindSight"),
            Self::ModPossess => f.write_str("ModPossess"),
            Self::PeriodicDamage{ .. } => f.write_str("PeriodicDamage"),
            Self::Dummy => f.write_str("Dummy"),
            Self::ModConfuse => f.write_str("ModConfuse"),
            Self::ModCharm => f.write_str("ModCharm"),
            Self::ModFear => f.write_str("ModFear"),
            Self::PeriodicHeal{ .. } => f.write_str("PeriodicHeal"),
            Self::ModAttackspeed => f.write_str("ModAttackspeed"),
            Self::ModThreat => f.write_str("ModThreat"),
            Self::ModTaunt => f.write_str("ModTaunt"),
            Self::ModStun => f.write_str("ModStun"),
            Self::ModDamageDone => f.write_str("ModDamageDone"),
            Self::ModDamageTaken => f.write_str("ModDamageTaken"),
            Self::DamageShield => f.write_str("DamageShield"),
            Self::ModStealth => f.write_str("ModStealth"),
            Self::ModStealthDetect => f.write_str("ModStealthDetect"),
            Self::ModInvisibility => f.write_str("ModInvisibility"),
            Self::ModInvisibilityDetect => f.write_str("ModInvisibilityDetect"),
            Self::ObsModHealth{ .. } => f.write_str("ObsModHealth"),
            Self::ObsModPower => f.write_str("ObsModPower"),
            Self::ModResistance => f.write_str("ModResistance"),
            Self::PeriodicTriggerSpell => f.write_str("PeriodicTriggerSpell"),
            Self::PeriodicEnergize{ .. } => f.write_str("PeriodicEnergize"),
            Self::ModPacify => f.write_str("ModPacify"),
            Self::ModRoot => f.write_str("ModRoot"),
            Self::ModSilence => f.write_str("ModSilence"),
            Self::ReflectSpells => f.write_str("ReflectSpells"),
            Self::ModStat => f.write_str("ModStat"),
            Self::ModSkill => f.write_str("ModSkill"),
            Self::ModIncreaseSpeed => f.write_str("ModIncreaseSpeed"),
            Self::ModIncreaseMountedSpeed => f.write_str("ModIncreaseMountedSpeed"),
            Self::ModDecreaseSpeed => f.write_str("ModDecreaseSpeed"),
            Self::ModIncreaseHealth => f.write_str("ModIncreaseHealth"),
            Self::ModIncreaseEnergy => f.write_str("ModIncreaseEnergy"),
            Self::ModShapeshift => f.write_str("ModShapeshift"),
            Self::EffectImmunity => f.write_str("EffectImmunity"),
            Self::StateImmunity => f.write_str("StateImmunity"),
            Self::SchoolImmunity => f.write_str("SchoolImmunity"),
            Self::DamageImmunity => f.write_str("DamageImmunity"),
            Self::DispelImmunity => f.write_str("DispelImmunity"),
            Self::ProcTriggerSpell => f.write_str("ProcTriggerSpell"),
            Self::ProcTriggerDamage => f.write_str("ProcTriggerDamage"),
            Self::TrackCreatures => f.write_str("TrackCreatures"),
            Self::TrackResources => f.write_str("TrackResources"),
            Self::Unknown46 => f.write_str("Unknown46"),
            Self::ModParryPercent => f.write_str("ModParryPercent"),
            Self::PeriodicTriggerSpellFromClient => f.write_str("PeriodicTriggerSpellFromClient"),
            Self::ModDodgePercent => f.write_str("ModDodgePercent"),
            Self::ModCriticalHealingAmount => f.write_str("ModCriticalHealingAmount"),
            Self::ModBlockPercent => f.write_str("ModBlockPercent"),
            Self::ModWeaponCritPercent => f.write_str("ModWeaponCritPercent"),
            Self::PeriodicLeech => f.write_str("PeriodicLeech"),
            Self::ModHitChance => f.write_str("ModHitChance"),
            Self::ModSpellHitChance => f.write_str("ModSpellHitChance"),
            Self::Transform => f.write_str("Transform"),
            Self::ModSpellCritChance => f.write_str("ModSpellCritChance"),
            Self::ModIncreaseSwimSpeed => f.write_str("ModIncreaseSwimSpeed"),
            Self::ModDamageDoneCreature => f.write_str("ModDamageDoneCreature"),
            Self::ModPacifySilence => f.write_str("ModPacifySilence"),
            Self::ModScale => f.write_str("ModScale"),
            Self::PeriodicHealthFunnel => f.write_str("PeriodicHealthFunnel"),
            Self::Unknown63 => f.write_str("Unknown63"),
            Self::PeriodicManaLeech{ .. } => f.write_str("PeriodicManaLeech"),
            Self::ModCastingSpeedNotStack => f.write_str("ModCastingSpeedNotStack"),
            Self::FeignDeath => f.write_str("FeignDeath"),
            Self::ModDisarm => f.write_str("ModDisarm"),
            Self::ModStalked => f.write_str("ModStalked"),
            Self::SchoolAbsorb => f.write_str("SchoolAbsorb"),
            Self::ExtraAttacks => f.write_str("ExtraAttacks"),
            Self::ModSpellCritChanceSchool => f.write_str("ModSpellCritChanceSchool"),
            Self::ModPowerCostSchoolPct => f.write_str("ModPowerCostSchoolPct"),
            Self::ModPowerCostSchool => f.write_str("ModPowerCostSchool"),
            Self::ReflectSpellsSchool => f.write_str("ReflectSpellsSchool"),
            Self::ModLanguage => f.write_str("ModLanguage"),
            Self::FarSight => f.write_str("FarSight"),
            Self::MechanicImmunity => f.write_str("MechanicImmunity"),
            Self::Mounted => f.write_str("Mounted"),
            Self::ModDamagePercentDone => f.write_str("ModDamagePercentDone"),
            Self::ModPercentStat => f.write_str("ModPercentStat"),
            Self::SplitDamagePct => f.write_str("SplitDamagePct"),
            Self::WaterBreathing => f.write_str("WaterBreathing"),
            Self::ModBaseResistance => f.write_str("ModBaseResistance"),
            Self::ModRegen => f.write_str("ModRegen"),
            Self::ModPowerRegen => f.write_str("ModPowerRegen"),
            Self::ChannelDeathItem => f.write_str("ChannelDeathItem"),
            Self::ModDamagePercentTaken => f.write_str("ModDamagePercentTaken"),
            Self::ModHealthRegenPercent => f.write_str("ModHealthRegenPercent"),
            Self::PeriodicDamagePercent{ .. } => f.write_str("PeriodicDamagePercent"),
            Self::Unknown90 => f.write_str("Unknown90"),
            Self::ModDetectRange => f.write_str("ModDetectRange"),
            Self::PreventsFleeing => f.write_str("PreventsFleeing"),
            Self::ModUnattackable => f.write_str("ModUnattackable"),
            Self::InterruptRegen => f.write_str("InterruptRegen"),
            Self::Ghost => f.write_str("Ghost"),
            Self::SpellMagnet => f.write_str("SpellMagnet"),
            Self::ManaShield => f.write_str("ManaShield"),
            Self::ModSkillTalent => f.write_str("ModSkillTalent"),
            Self::ModAttackPower => f.write_str("ModAttackPower"),
            Self::AurasVisible => f.write_str("AurasVisible"),
            Self::ModResistancePct => f.write_str("ModResistancePct"),
            Self::ModMeleeAttackPowerVersus => f.write_str("ModMeleeAttackPowerVersus"),
            Self::ModTotalThreat => f.write_str("ModTotalThreat"),
            Self::WaterWalk => f.write_str("WaterWalk"),
            Self::FeatherFall => f.write_str("FeatherFall"),
            Self::Hover => f.write_str("Hover"),
            Self::AddFlatModifier => f.write_str("AddFlatModifier"),
            Self::AddPctModifier => f.write_str("AddPctModifier"),
            Self::AddTargetTrigger => f.write_str("AddTargetTrigger"),
            Self::ModPowerRegenPercent => f.write_str("ModPowerRegenPercent"),
            Self::AddCasterHitTrigger => f.write_str("AddCasterHitTrigger"),
            Self::OverrideClassScripts => f.write_str("OverrideClassScripts"),
            Self::ModRangedDamageTaken => f.write_str("ModRangedDamageTaken"),
            Self::ModRangedDamageTakenPct => f.write_str("ModRangedDamageTakenPct"),
            Self::ModHealing => f.write_str("ModHealing"),
            Self::ModRegenDuringCombat => f.write_str("ModRegenDuringCombat"),
            Self::ModMechanicResistance => f.write_str("ModMechanicResistance"),
            Self::ModHealingPct => f.write_str("ModHealingPct"),
            Self::Unknown119 => f.write_str("Unknown119"),
            Self::Untrackable => f.write_str("Untrackable"),
            Self::Empathy => f.write_str("Empathy"),
            Self::ModOffhandDamagePct => f.write_str("ModOffhandDamagePct"),
            Self::ModTargetResistance => f.write_str("ModTargetResistance"),
            Self::ModRangedAttackPower => f.write_str("ModRangedAttackPower"),
            Self::ModMeleeDamageTaken => f.write_str("ModMeleeDamageTaken"),
            Self::ModMeleeDamageTakenPct => f.write_str("ModMeleeDamageTakenPct"),
            Self::RangedAttackPowerAttackerBonus => f.write_str("RangedAttackPowerAttackerBonus"),
            Self::ModPossessPet => f.write_str("ModPossessPet"),
            Self::ModSpeedAlways => f.write_str("ModSpeedAlways"),
            Self::ModMountedSpeedAlways => f.write_str("ModMountedSpeedAlways"),
            Self::ModRangedAttackPowerVersus => f.write_str("ModRangedAttackPowerVersus"),
            Self::ModIncreaseEnergyPercent => f.write_str("ModIncreaseEnergyPercent"),
            Self::ModIncreaseHealthPercent => f.write_str("ModIncreaseHealthPercent"),
            Self::ModManaRegenInterrupt => f.write_str("ModManaRegenInterrupt"),
            Self::ModHealingDone => f.write_str("ModHealingDone"),
            Self::ModHealingDonePercent => f.write_str("ModHealingDonePercent"),
            Self::ModTotalStatPercentage => f.write_str("ModTotalStatPercentage"),
            Self::ModMeleeHaste => f.write_str("ModMeleeHaste"),
            Self::ForceReaction => f.write_str("ForceReaction"),
            Self::ModRangedHaste => f.write_str("ModRangedHaste"),
            Self::ModRangedAmmoHaste => f.write_str("ModRangedAmmoHaste"),
            Self::ModBaseResistancePct => f.write_str("ModBaseResistancePct"),
            Self::ModResistanceExclusive => f.write_str("ModResistanceExclusive"),
            Self::SafeFall => f.write_str("SafeFall"),
            Self::ModPetTalentPoints => f.write_str("ModPetTalentPoints"),
            Self::AllowTamePetType => f.write_str("AllowTamePetType"),
            Self::MechanicImmunityMask => f.write_str("MechanicImmunityMask"),
            Self::RetainComboPoints => f.write_str("RetainComboPoints"),
            Self::ReducePushback => f.write_str("ReducePushback"),
            Self::ModShieldBlockvaluePct => f.write_str("ModShieldBlockvaluePct"),
            Self::TrackStealthed => f.write_str("TrackStealthed"),
            Self::ModDetectedRange => f.write_str("ModDetectedRange"),
            Self::SplitDamageFlat => f.write_str("SplitDamageFlat"),
            Self::ModStealthLevel => f.write_str("ModStealthLevel"),
            Self::ModWaterBreathing => f.write_str("ModWaterBreathing"),
            Self::ModReputationGain => f.write_str("ModReputationGain"),
            Self::PetDamageMulti => f.write_str("PetDamageMulti"),
            Self::ModShieldBlockvalue => f.write_str("ModShieldBlockvalue"),
            Self::NoPvpCredit => f.write_str("NoPvpCredit"),
            Self::ModAoeAvoidance => f.write_str("ModAoeAvoidance"),
            Self::ModHealthRegenInCombat => f.write_str("ModHealthRegenInCombat"),
            Self::PowerBurn => f.write_str("PowerBurn"),
            Self::ModCritDamageBonus => f.write_str("ModCritDamageBonus"),
            Self::Unknown164 => f.write_str("Unknown164"),
            Self::MeleeAttackPowerAttackerBonus => f.write_str("MeleeAttackPowerAttackerBonus"),
            Self::ModAttackPowerPct => f.write_str("ModAttackPowerPct"),
            Self::ModRangedAttackPowerPct => f.write_str("ModRangedAttackPowerPct"),
            Self::ModDamageDoneVersus => f.write_str("ModDamageDoneVersus"),
            Self::ModCritPercentVersus => f.write_str("ModCritPercentVersus"),
            Self::DetectAmore => f.write_str("DetectAmore"),
            Self::ModSpeedNotStack => f.write_str("ModSpeedNotStack"),
            Self::ModMountedSpeedNotStack => f.write_str("ModMountedSpeedNotStack"),
            Self::Unknown173 => f.write_str("Unknown173"),
            Self::ModSpellDamageOfStatPercent => f.write_str("ModSpellDamageOfStatPercent"),
            Self::ModSpellHealingOfStatPercent => f.write_str("ModSpellHealingOfStatPercent"),
            Self::SpiritOfRedemption => f.write_str("SpiritOfRedemption"),
            Self::AoeCharm => f.write_str("AoeCharm"),
            Self::ModDebuffResistance => f.write_str("ModDebuffResistance"),
            Self::ModAttackerSpellCritChance => f.write_str("ModAttackerSpellCritChance"),
            Self::ModFlatSpellDamageVersus => f.write_str("ModFlatSpellDamageVersus"),
            Self::Unknown181 => f.write_str("Unknown181"),
            Self::ModResistanceOfStatPercent => f.write_str("ModResistanceOfStatPercent"),
            Self::ModCriticalThreat => f.write_str("ModCriticalThreat"),
            Self::ModAttackerMeleeHitChance => f.write_str("ModAttackerMeleeHitChance"),
            Self::ModAttackerRangedHitChance => f.write_str("ModAttackerRangedHitChance"),
            Self::ModAttackerSpellHitChance => f.write_str("ModAttackerSpellHitChance"),
            Self::ModAttackerMeleeCritChance => f.write_str("ModAttackerMeleeCritChance"),
            Self::ModAttackerRangedCritChance => f.write_str("ModAttackerRangedCritChance"),
            Self::ModRating => f.write_str("ModRating"),
            Self::ModFactionReputationGain => f.write_str("ModFactionReputationGain"),
            Self::UseNormalMovementSpeed => f.write_str("UseNormalMovementSpeed"),
            Self::ModMeleeRangedHaste => f.write_str("ModMeleeRangedHaste"),
            Self::MeleeSlow => f.write_str("MeleeSlow"),
            Self::ModTargetAbsorbSchool => f.write_str("ModTargetAbsorbSchool"),
            Self::ModTargetAbilityAbsorbSchool => f.write_str("ModTargetAbilityAbsorbSchool"),
            Self::ModCooldown => f.write_str("ModCooldown"),
            Self::ModAttackerSpellAndWeaponCritChance => f.write_str("ModAttackerSpellAndWeaponCritChance"),
            Self::Unknown198 => f.write_str("Unknown198"),
            Self::ModIncreasesSpellPctToHit => f.write_str("ModIncreasesSpellPctToHit"),
            Self::ModXpPct => f.write_str("ModXpPct"),
            Self::Fly => f.write_str("Fly"),
            Self::IgnoreCombatResult => f.write_str("IgnoreCombatResult"),
            Self::ModAttackerMeleeCritDamage => f.write_str("ModAttackerMeleeCritDamage"),
            Self::ModAttackerRangedCritDamage => f.write_str("ModAttackerRangedCritDamage"),
            Self::ModSchoolCritDmgTaken => f.write_str("ModSchoolCritDmgTaken"),
            Self::ModIncreaseVehicleFlightSpeed => f.write_str("ModIncreaseVehicleFlightSpeed"),
            Self::ModIncreaseMountedFlightSpeed => f.write_str("ModIncreaseMountedFlightSpeed"),
            Self::ModIncreaseFlightSpeed => f.write_str("ModIncreaseFlightSpeed"),
            Self::ModMountedFlightSpeedAlways => f.write_str("ModMountedFlightSpeedAlways"),
            Self::ModVehicleSpeedAlways => f.write_str("ModVehicleSpeedAlways"),
            Self::ModFlightSpeedNotStack => f.write_str("ModFlightSpeedNotStack"),
            Self::ModRangedAttackPowerOfStatPercent => f.write_str("ModRangedAttackPowerOfStatPercent"),
            Self::ModRageFromDamageDealt => f.write_str("ModRageFromDamageDealt"),
            Self::Unknown214 => f.write_str("Unknown214"),
            Self::ArenaPreparation => f.write_str("ArenaPreparation"),
            Self::HasteSpells => f.write_str("HasteSpells"),
            Self::ModMeleeHaste2 => f.write_str("ModMeleeHaste2"),
            Self::HasteRanged => f.write_str("HasteRanged"),
            Self::ModManaRegenFromStat => f.write_str("ModManaRegenFromStat"),
            Self::ModRatingFromStat => f.write_str("ModRatingFromStat"),
            Self::ModDetaunt => f.write_str("ModDetaunt"),
            Self::Unknown222 => f.write_str("Unknown222"),
            Self::RaidProcFromCharge => f.write_str("RaidProcFromCharge"),
            Self::Unknown224 => f.write_str("Unknown224"),
            Self::RaidProcFromChargeWithValue => f.write_str("RaidProcFromChargeWithValue"),
            Self::PeriodicDummy => f.write_str("PeriodicDummy"),
            Self::PeriodicTriggerSpellWithValue => f.write_str("PeriodicTriggerSpellWithValue"),
            Self::DetectStealth => f.write_str("DetectStealth"),
            Self::ModAoeDamageAvoidance => f.write_str("ModAoeDamageAvoidance"),
            Self::Unknown230 => f.write_str("Unknown230"),
            Self::ProcTriggerSpellWithValue => f.write_str("ProcTriggerSpellWithValue"),
            Self::MechanicDurationMod => f.write_str("MechanicDurationMod"),
            Self::ChangeModelForAllHumanoids => f.write_str("ChangeModelForAllHumanoids"),
            Self::MechanicDurationModNotStack => f.write_str("MechanicDurationModNotStack"),
            Self::ModDispelResist => f.write_str("ModDispelResist"),
            Self::ControlVehicle => f.write_str("ControlVehicle"),
            Self::ModSpellDamageOfAttackPower => f.write_str("ModSpellDamageOfAttackPower"),
            Self::ModSpellHealingOfAttackPower => f.write_str("ModSpellHealingOfAttackPower"),
            Self::ModScale2 => f.write_str("ModScale2"),
            Self::ModExpertise => f.write_str("ModExpertise"),
            Self::ForceMoveForward => f.write_str("ForceMoveForward"),
            Self::ModSpellDamageFromHealing => f.write_str("ModSpellDamageFromHealing"),
            Self::ModFaction => f.write_str("ModFaction"),
            Self::ComprehendLanguage => f.write_str("ComprehendLanguage"),
            Self::ModAuraDurationByDispel => f.write_str("ModAuraDurationByDispel"),
            Self::ModAuraDurationByDispelNotStack => f.write_str("ModAuraDurationByDispelNotStack"),
            Self::CloneCaster => f.write_str("CloneCaster"),
            Self::ModCombatResultChance => f.write_str("ModCombatResultChance"),
            Self::ConvertRune => f.write_str("ConvertRune"),
            Self::ModIncreaseHealth2 => f.write_str("ModIncreaseHealth2"),
            Self::ModEnemyDodge => f.write_str("ModEnemyDodge"),
            Self::ModSpeedSlowAll => f.write_str("ModSpeedSlowAll"),
            Self::ModBlockCritChance => f.write_str("ModBlockCritChance"),
            Self::ModDisarmOffhand => f.write_str("ModDisarmOffhand"),
            Self::ModMechanicDamageTakenPercent => f.write_str("ModMechanicDamageTakenPercent"),
            Self::NoReagentUse => f.write_str("NoReagentUse"),
            Self::ModTargetResistBySpellClass => f.write_str("ModTargetResistBySpellClass"),
            Self::Unknown258 => f.write_str("Unknown258"),
            Self::ModHotPct => f.write_str("ModHotPct"),
            Self::ScreenEffect => f.write_str("ScreenEffect"),
            Self::Phase => f.write_str("Phase"),
            Self::AbilityIgnoreAurastate => f.write_str("AbilityIgnoreAurastate"),
            Self::AllowOnlyAbility => f.write_str("AllowOnlyAbility"),
            Self::Unknown264 => f.write_str("Unknown264"),
            Self::Unknown265 => f.write_str("Unknown265"),
            Self::Unknown266 => f.write_str("Unknown266"),
            Self::ModImmuneAuraApplySchool => f.write_str("ModImmuneAuraApplySchool"),
            Self::ModAttackPowerOfStatPercent => f.write_str("ModAttackPowerOfStatPercent"),
            Self::ModIgnoreTargetResist => f.write_str("ModIgnoreTargetResist"),
            Self::ModAbilityIgnoreTargetResist => f.write_str("ModAbilityIgnoreTargetResist"),
            Self::ModDamageFromCaster => f.write_str("ModDamageFromCaster"),
            Self::IgnoreMeleeReset => f.write_str("IgnoreMeleeReset"),
            Self::XRay => f.write_str("XRay"),
            Self::AbilityConsumeNoAmmo => f.write_str("AbilityConsumeNoAmmo"),
            Self::ModIgnoreShapeshift => f.write_str("ModIgnoreShapeshift"),
            Self::ModDamageDoneForMechanic => f.write_str("ModDamageDoneForMechanic"),
            Self::ModMaxAffectedTargets => f.write_str("ModMaxAffectedTargets"),
            Self::ModDisarmRanged => f.write_str("ModDisarmRanged"),
            Self::InitializeImages => f.write_str("InitializeImages"),
            Self::ModArmorPenetrationPct => f.write_str("ModArmorPenetrationPct"),
            Self::ModHonorGainPct => f.write_str("ModHonorGainPct"),
            Self::ModBaseHealthPct => f.write_str("ModBaseHealthPct"),
            Self::ModHealingReceived => f.write_str("ModHealingReceived"),
            Self::Linked => f.write_str("Linked"),
            Self::ModAttackPowerOfArmor => f.write_str("ModAttackPowerOfArmor"),
            Self::AbilityPeriodicCrit => f.write_str("AbilityPeriodicCrit"),
            Self::DeflectSpells => f.write_str("DeflectSpells"),
            Self::IgnoreHitDirection => f.write_str("IgnoreHitDirection"),
            Self::PreventDurabilityLoss => f.write_str("PreventDurabilityLoss"),
            Self::ModCritPct => f.write_str("ModCritPct"),
            Self::ModXpQuestPct => f.write_str("ModXpQuestPct"),
            Self::OpenStable => f.write_str("OpenStable"),
            Self::OverrideSpells => f.write_str("OverrideSpells"),
            Self::PreventRegeneratePower => f.write_str("PreventRegeneratePower"),
            Self::Unknown295 => f.write_str("Unknown295"),
            Self::SetVehicleId => f.write_str("SetVehicleId"),
            Self::BlockSpellFamily => f.write_str("BlockSpellFamily"),
            Self::Strangulate => f.write_str("Strangulate"),
            Self::Unknown299 => f.write_str("Unknown299"),
            Self::ShareDamagePct => f.write_str("ShareDamagePct"),
            Self::SchoolHealAbsorb => f.write_str("SchoolHealAbsorb"),
            Self::Unknown302 => f.write_str("Unknown302"),
            Self::ModDamageDoneVersusAurastate => f.write_str("ModDamageDoneVersusAurastate"),
            Self::ModFakeInebriate => f.write_str("ModFakeInebriate"),
            Self::ModMinimumSpeed => f.write_str("ModMinimumSpeed"),
            Self::Unknown306 => f.write_str("Unknown306"),
            Self::HealAbsorbTest => f.write_str("HealAbsorbTest"),
            Self::ModCritChanceForCaster => f.write_str("ModCritChanceForCaster"),
            Self::Unknown309 => f.write_str("Unknown309"),
            Self::ModCreatureAoeDamageAvoidance => f.write_str("ModCreatureAoeDamageAvoidance"),
            Self::Unknown311 => f.write_str("Unknown311"),
            Self::Unknown312 => f.write_str("Unknown312"),
            Self::Unknown313 => f.write_str("Unknown313"),
            Self::PreventResurrection => f.write_str("PreventResurrection"),
            Self::UnderwaterWalking => f.write_str("UnderwaterWalking"),
            Self::PeriodicHaste => f.write_str("PeriodicHaste"),
        }
    }
}

impl AuraLog_AuraType {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::PeriodicDamage {
                ..
            } => {
                4
                + 4 // absorb1: u32
                + 1 // critical1: Bool
                + 4 // damage1: u32
                + 4 // overkill_damage: u32
                + 4 // resisted: u32
                + 1 // school: SpellSchool
            }
            Self::PeriodicHeal {
                ..
            } => {
                4
                + 4 // absorb2: u32
                + 1 // critical2: Bool
                + 4 // damage2: u32
                + 4 // over_damage: u32
            }
            Self::ObsModHealth {
                ..
            } => {
                4
                + 4 // absorb2: u32
                + 1 // critical2: Bool
                + 4 // damage2: u32
                + 4 // over_damage: u32
            }
            Self::PeriodicEnergize {
                ..
            } => {
                4
                + 4 // damage3: u32
                + 4 // misc_value1: u32
            }
            Self::PeriodicManaLeech {
                ..
            } => {
                4
                + 4 // damage4: u32
                + 4 // gain_multiplier: f32
                + 4 // misc_value2: u32
            }
            Self::PeriodicDamagePercent {
                ..
            } => {
                4
                + 4 // absorb1: u32
                + 1 // critical1: Bool
                + 4 // damage1: u32
                + 4 // overkill_damage: u32
                + 4 // resisted: u32
                + 1 // school: SpellSchool
            }
            _ => 4,
        }
    }
}

