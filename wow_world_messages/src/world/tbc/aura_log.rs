use std::io::{Read, Write};

use crate::tbc::{
    AuraType, SpellSchool,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm:655`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm#L655):
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
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // aura_type: AuraType
        w.write_all(&u32::from(self.aura_type.as_int()).to_le_bytes())?;

        match &self.aura_type {
            AuraLog_AuraType::PeriodicDamage {
                absorbed,
                damage1,
                resisted,
                school,
            } => {
                // damage1: u32
                w.write_all(&damage1.to_le_bytes())?;

                // school: SpellSchool
                w.write_all(&u8::from(school.as_int()).to_le_bytes())?;

                // absorbed: u32
                w.write_all(&absorbed.to_le_bytes())?;

                // resisted: u32
                w.write_all(&resisted.to_le_bytes())?;

            }
            AuraLog_AuraType::PeriodicHeal {
                damage2,
            } => {
                // damage2: u32
                w.write_all(&damage2.to_le_bytes())?;

            }
            AuraLog_AuraType::ObsModHealth {
                damage2,
            } => {
                // damage2: u32
                w.write_all(&damage2.to_le_bytes())?;

            }
            AuraLog_AuraType::ObsModMana {
                damage3,
                misc_value1,
            } => {
                // misc_value1: u32
                w.write_all(&misc_value1.to_le_bytes())?;

                // damage3: u32
                w.write_all(&damage3.to_le_bytes())?;

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
            AuraLog_AuraType::PeriodicDamagePercent {
                absorbed,
                damage1,
                resisted,
                school,
            } => {
                // damage1: u32
                w.write_all(&damage1.to_le_bytes())?;

                // school: SpellSchool
                w.write_all(&u8::from(school.as_int()).to_le_bytes())?;

                // absorbed: u32
                w.write_all(&absorbed.to_le_bytes())?;

                // resisted: u32
                w.write_all(&resisted.to_le_bytes())?;

            }
            _ => {}
        }

        Ok(())
    }
}

impl AuraLog {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // aura_type: AuraType
        let aura_type: AuraType = crate::util::read_u32_le(&mut r)?.try_into()?;

        let aura_type_if = match aura_type {
            AuraType::None => AuraLog_AuraType::None,
            AuraType::BindSight => AuraLog_AuraType::BindSight,
            AuraType::ModPossess => AuraLog_AuraType::ModPossess,
            AuraType::PeriodicDamage => {
                // damage1: u32
                let damage1 = crate::util::read_u32_le(&mut r)?;

                // school: SpellSchool
                let school: SpellSchool = crate::util::read_u8_le(&mut r)?.try_into()?;

                // absorbed: u32
                let absorbed = crate::util::read_u32_le(&mut r)?;

                // resisted: u32
                let resisted = crate::util::read_u32_le(&mut r)?;

                AuraLog_AuraType::PeriodicDamage {
                    absorbed,
                    damage1,
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

                AuraLog_AuraType::PeriodicHeal {
                    damage2,
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
            AuraType::ModInvisibilityDetection => AuraLog_AuraType::ModInvisibilityDetection,
            AuraType::ObsModHealth => {
                // damage2: u32
                let damage2 = crate::util::read_u32_le(&mut r)?;

                AuraLog_AuraType::ObsModHealth {
                    damage2,
                }
            }
            AuraType::ObsModMana => {
                // misc_value1: u32
                let misc_value1 = crate::util::read_u32_le(&mut r)?;

                // damage3: u32
                let damage3 = crate::util::read_u32_le(&mut r)?;

                AuraLog_AuraType::ObsModMana {
                    damage3,
                    misc_value1,
                }
            }
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
            AuraType::Unknown48 => AuraLog_AuraType::Unknown48,
            AuraType::ModDodgePercent => AuraLog_AuraType::ModDodgePercent,
            AuraType::ModBlockSkill => AuraLog_AuraType::ModBlockSkill,
            AuraType::ModBlockPercent => AuraLog_AuraType::ModBlockPercent,
            AuraType::ModCritPercent => AuraLog_AuraType::ModCritPercent,
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
            AuraType::PeriodicManaFunnel => AuraLog_AuraType::PeriodicManaFunnel,
            AuraType::PeriodicManaLeech => {
                // misc_value2: u32
                let misc_value2 = crate::util::read_u32_le(&mut r)?;

                // damage: u32
                let damage = crate::util::read_u32_le(&mut r)?;

                // gain_multiplier: f32
                let gain_multiplier = crate::util::read_f32_le(&mut r)?;

                AuraLog_AuraType::PeriodicManaLeech {
                    damage,
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

                // school: SpellSchool
                let school: SpellSchool = crate::util::read_u8_le(&mut r)?.try_into()?;

                // absorbed: u32
                let absorbed = crate::util::read_u32_le(&mut r)?;

                // resisted: u32
                let resisted = crate::util::read_u32_le(&mut r)?;

                AuraLog_AuraType::PeriodicDamagePercent {
                    absorbed,
                    damage1,
                    resisted,
                    school,
                }
            }
            AuraType::ModResistChance => AuraLog_AuraType::ModResistChance,
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
            AuraType::SharePetTracking => AuraLog_AuraType::SharePetTracking,
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
            AuraType::Charisma => AuraLog_AuraType::Charisma,
            AuraType::Persuaded => AuraLog_AuraType::Persuaded,
            AuraType::MechanicImmunityMask => AuraLog_AuraType::MechanicImmunityMask,
            AuraType::RetainComboPoints => AuraLog_AuraType::RetainComboPoints,
            AuraType::ResistPushback => AuraLog_AuraType::ResistPushback,
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
            AuraType::PowerBurnMana => AuraLog_AuraType::PowerBurnMana,
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
            AuraType::AllowChampionSpells => AuraLog_AuraType::AllowChampionSpells,
            AuraType::ModSpellDamageOfStatPercent => AuraLog_AuraType::ModSpellDamageOfStatPercent,
            AuraType::ModSpellHealingOfStatPercent => AuraLog_AuraType::ModSpellHealingOfStatPercent,
            AuraType::SpiritOfRedemption => AuraLog_AuraType::SpiritOfRedemption,
            AuraType::AoeCharm => AuraLog_AuraType::AoeCharm,
            AuraType::ModDebuffResistance => AuraLog_AuraType::ModDebuffResistance,
            AuraType::ModAttackerSpellCritChance => AuraLog_AuraType::ModAttackerSpellCritChance,
            AuraType::ModFlatSpellDamageVersus => AuraLog_AuraType::ModFlatSpellDamageVersus,
            AuraType::ModFlatSpellCritDamageVersus => AuraLog_AuraType::ModFlatSpellCritDamageVersus,
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
            AuraType::HasteAll => AuraLog_AuraType::HasteAll,
            AuraType::ModDepricated1 => AuraLog_AuraType::ModDepricated1,
            AuraType::ModDepricated2 => AuraLog_AuraType::ModDepricated2,
            AuraType::ModCooldown => AuraLog_AuraType::ModCooldown,
            AuraType::ModAttackerSpellAndWeaponCritChance => AuraLog_AuraType::ModAttackerSpellAndWeaponCritChance,
            AuraType::ModAllWeaponSkills => AuraLog_AuraType::ModAllWeaponSkills,
            AuraType::ModIncreasesSpellPctToHit => AuraLog_AuraType::ModIncreasesSpellPctToHit,
            AuraType::ModXpPct => AuraLog_AuraType::ModXpPct,
            AuraType::Fly => AuraLog_AuraType::Fly,
            AuraType::IgnoreCombatResult => AuraLog_AuraType::IgnoreCombatResult,
            AuraType::ModAttackerMeleeCritDamage => AuraLog_AuraType::ModAttackerMeleeCritDamage,
            AuraType::ModAttackerRangedCritDamage => AuraLog_AuraType::ModAttackerRangedCritDamage,
            AuraType::ModAttackerSpellCritDamage => AuraLog_AuraType::ModAttackerSpellCritDamage,
            AuraType::ModFlightSpeed => AuraLog_AuraType::ModFlightSpeed,
            AuraType::ModFlightSpeedMounted => AuraLog_AuraType::ModFlightSpeedMounted,
            AuraType::ModFlightSpeedStacking => AuraLog_AuraType::ModFlightSpeedStacking,
            AuraType::ModFlightSpeedMountedStacking => AuraLog_AuraType::ModFlightSpeedMountedStacking,
            AuraType::ModFlightSpeedNotStacking => AuraLog_AuraType::ModFlightSpeedNotStacking,
            AuraType::ModFlightSpeedMountedNotStacking => AuraLog_AuraType::ModFlightSpeedMountedNotStacking,
            AuraType::ModRangedAttackPowerOfStatPercent => AuraLog_AuraType::ModRangedAttackPowerOfStatPercent,
            AuraType::ModRageFromDamageDealt => AuraLog_AuraType::ModRageFromDamageDealt,
            AuraType::Unknown214 => AuraLog_AuraType::Unknown214,
            AuraType::ArenaPreparation => AuraLog_AuraType::ArenaPreparation,
            AuraType::HasteSpells => AuraLog_AuraType::HasteSpells,
            AuraType::Unknown217 => AuraLog_AuraType::Unknown217,
            AuraType::HasteRanged => AuraLog_AuraType::HasteRanged,
            AuraType::ModManaRegenFromStat => AuraLog_AuraType::ModManaRegenFromStat,
            AuraType::ModRatingFromStat => AuraLog_AuraType::ModRatingFromStat,
            AuraType::Unknown221 => AuraLog_AuraType::Unknown221,
            AuraType::Unknown222 => AuraLog_AuraType::Unknown222,
            AuraType::Unknown223 => AuraLog_AuraType::Unknown223,
            AuraType::Unknown224 => AuraLog_AuraType::Unknown224,
            AuraType::PrayerOfMending => AuraLog_AuraType::PrayerOfMending,
            AuraType::PeriodicDummy => AuraLog_AuraType::PeriodicDummy,
            AuraType::PeriodicTriggerSpellWithValue => AuraLog_AuraType::PeriodicTriggerSpellWithValue,
            AuraType::DetectStealth => AuraLog_AuraType::DetectStealth,
            AuraType::ModAoeDamageAvoidance => AuraLog_AuraType::ModAoeDamageAvoidance,
            AuraType::Unknown230 => AuraLog_AuraType::Unknown230,
            AuraType::ProcTriggerSpellWithValue => AuraLog_AuraType::ProcTriggerSpellWithValue,
            AuraType::MechanicDurationMod => AuraLog_AuraType::MechanicDurationMod,
            AuraType::Unknown233 => AuraLog_AuraType::Unknown233,
            AuraType::MechanicDurationModNotStack => AuraLog_AuraType::MechanicDurationModNotStack,
            AuraType::ModDispelResist => AuraLog_AuraType::ModDispelResist,
            AuraType::Unknown236 => AuraLog_AuraType::Unknown236,
            AuraType::ModSpellDamageOfAttackPower => AuraLog_AuraType::ModSpellDamageOfAttackPower,
            AuraType::ModSpellHealingOfAttackPower => AuraLog_AuraType::ModSpellHealingOfAttackPower,
            AuraType::ModScale2 => AuraLog_AuraType::ModScale2,
            AuraType::ModExpertise => AuraLog_AuraType::ModExpertise,
            AuraType::ForceMoveForward => AuraLog_AuraType::ForceMoveForward,
            AuraType::Unknown242 => AuraLog_AuraType::Unknown242,
            AuraType::Unknown243 => AuraLog_AuraType::Unknown243,
            AuraType::ComprehendLanguage => AuraLog_AuraType::ComprehendLanguage,
            AuraType::Unknown245 => AuraLog_AuraType::Unknown245,
            AuraType::Unknown246 => AuraLog_AuraType::Unknown246,
            AuraType::MirrorImage => AuraLog_AuraType::MirrorImage,
            AuraType::ModCombatResultChance => AuraLog_AuraType::ModCombatResultChance,
            AuraType::Unknown249 => AuraLog_AuraType::Unknown249,
            AuraType::ModIncreaseHealth2 => AuraLog_AuraType::ModIncreaseHealth2,
            AuraType::ModEnemyDodge => AuraLog_AuraType::ModEnemyDodge,
            AuraType::Unknown252 => AuraLog_AuraType::Unknown252,
            AuraType::Unknown253 => AuraLog_AuraType::Unknown253,
            AuraType::Unknown254 => AuraLog_AuraType::Unknown254,
            AuraType::Unknown255 => AuraLog_AuraType::Unknown255,
            AuraType::Unknown256 => AuraLog_AuraType::Unknown256,
            AuraType::Unknown257 => AuraLog_AuraType::Unknown257,
            AuraType::Unknown258 => AuraLog_AuraType::Unknown258,
            AuraType::Unknown259 => AuraLog_AuraType::Unknown259,
            AuraType::Unknown260 => AuraLog_AuraType::Unknown260,
            AuraType::Unknown261 => AuraLog_AuraType::Unknown261,
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
        absorbed: u32,
        damage1: u32,
        resisted: u32,
        school: SpellSchool,
    },
    Dummy,
    ModConfuse,
    ModCharm,
    ModFear,
    PeriodicHeal {
        damage2: u32,
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
    ModInvisibilityDetection,
    ObsModHealth {
        damage2: u32,
    },
    ObsModMana {
        damage3: u32,
        misc_value1: u32,
    },
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
    Unknown48,
    ModDodgePercent,
    ModBlockSkill,
    ModBlockPercent,
    ModCritPercent,
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
    PeriodicManaFunnel,
    PeriodicManaLeech {
        damage: u32,
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
        absorbed: u32,
        damage1: u32,
        resisted: u32,
        school: SpellSchool,
    },
    ModResistChance,
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
    SharePetTracking,
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
    Charisma,
    Persuaded,
    MechanicImmunityMask,
    RetainComboPoints,
    ResistPushback,
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
    PowerBurnMana,
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
    AllowChampionSpells,
    ModSpellDamageOfStatPercent,
    ModSpellHealingOfStatPercent,
    SpiritOfRedemption,
    AoeCharm,
    ModDebuffResistance,
    ModAttackerSpellCritChance,
    ModFlatSpellDamageVersus,
    ModFlatSpellCritDamageVersus,
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
    HasteAll,
    ModDepricated1,
    ModDepricated2,
    ModCooldown,
    ModAttackerSpellAndWeaponCritChance,
    ModAllWeaponSkills,
    ModIncreasesSpellPctToHit,
    ModXpPct,
    Fly,
    IgnoreCombatResult,
    ModAttackerMeleeCritDamage,
    ModAttackerRangedCritDamage,
    ModAttackerSpellCritDamage,
    ModFlightSpeed,
    ModFlightSpeedMounted,
    ModFlightSpeedStacking,
    ModFlightSpeedMountedStacking,
    ModFlightSpeedNotStacking,
    ModFlightSpeedMountedNotStacking,
    ModRangedAttackPowerOfStatPercent,
    ModRageFromDamageDealt,
    Unknown214,
    ArenaPreparation,
    HasteSpells,
    Unknown217,
    HasteRanged,
    ModManaRegenFromStat,
    ModRatingFromStat,
    Unknown221,
    Unknown222,
    Unknown223,
    Unknown224,
    PrayerOfMending,
    PeriodicDummy,
    PeriodicTriggerSpellWithValue,
    DetectStealth,
    ModAoeDamageAvoidance,
    Unknown230,
    ProcTriggerSpellWithValue,
    MechanicDurationMod,
    Unknown233,
    MechanicDurationModNotStack,
    ModDispelResist,
    Unknown236,
    ModSpellDamageOfAttackPower,
    ModSpellHealingOfAttackPower,
    ModScale2,
    ModExpertise,
    ForceMoveForward,
    Unknown242,
    Unknown243,
    ComprehendLanguage,
    Unknown245,
    Unknown246,
    MirrorImage,
    ModCombatResultChance,
    Unknown249,
    ModIncreaseHealth2,
    ModEnemyDodge,
    Unknown252,
    Unknown253,
    Unknown254,
    Unknown255,
    Unknown256,
    Unknown257,
    Unknown258,
    Unknown259,
    Unknown260,
    Unknown261,
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
            Self::ModInvisibilityDetection => 19,
            Self::ObsModHealth { .. } => 20,
            Self::ObsModMana { .. } => 21,
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
            Self::Unknown48 => 48,
            Self::ModDodgePercent => 49,
            Self::ModBlockSkill => 50,
            Self::ModBlockPercent => 51,
            Self::ModCritPercent => 52,
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
            Self::PeriodicManaFunnel => 63,
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
            Self::ModResistChance => 90,
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
            Self::SharePetTracking => 119,
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
            Self::Charisma => 145,
            Self::Persuaded => 146,
            Self::MechanicImmunityMask => 147,
            Self::RetainComboPoints => 148,
            Self::ResistPushback => 149,
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
            Self::PowerBurnMana => 162,
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
            Self::AllowChampionSpells => 173,
            Self::ModSpellDamageOfStatPercent => 174,
            Self::ModSpellHealingOfStatPercent => 175,
            Self::SpiritOfRedemption => 176,
            Self::AoeCharm => 177,
            Self::ModDebuffResistance => 178,
            Self::ModAttackerSpellCritChance => 179,
            Self::ModFlatSpellDamageVersus => 180,
            Self::ModFlatSpellCritDamageVersus => 181,
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
            Self::HasteAll => 193,
            Self::ModDepricated1 => 194,
            Self::ModDepricated2 => 195,
            Self::ModCooldown => 196,
            Self::ModAttackerSpellAndWeaponCritChance => 197,
            Self::ModAllWeaponSkills => 198,
            Self::ModIncreasesSpellPctToHit => 199,
            Self::ModXpPct => 200,
            Self::Fly => 201,
            Self::IgnoreCombatResult => 202,
            Self::ModAttackerMeleeCritDamage => 203,
            Self::ModAttackerRangedCritDamage => 204,
            Self::ModAttackerSpellCritDamage => 205,
            Self::ModFlightSpeed => 206,
            Self::ModFlightSpeedMounted => 207,
            Self::ModFlightSpeedStacking => 208,
            Self::ModFlightSpeedMountedStacking => 209,
            Self::ModFlightSpeedNotStacking => 210,
            Self::ModFlightSpeedMountedNotStacking => 211,
            Self::ModRangedAttackPowerOfStatPercent => 212,
            Self::ModRageFromDamageDealt => 213,
            Self::Unknown214 => 214,
            Self::ArenaPreparation => 215,
            Self::HasteSpells => 216,
            Self::Unknown217 => 217,
            Self::HasteRanged => 218,
            Self::ModManaRegenFromStat => 219,
            Self::ModRatingFromStat => 220,
            Self::Unknown221 => 221,
            Self::Unknown222 => 222,
            Self::Unknown223 => 223,
            Self::Unknown224 => 224,
            Self::PrayerOfMending => 225,
            Self::PeriodicDummy => 226,
            Self::PeriodicTriggerSpellWithValue => 227,
            Self::DetectStealth => 228,
            Self::ModAoeDamageAvoidance => 229,
            Self::Unknown230 => 230,
            Self::ProcTriggerSpellWithValue => 231,
            Self::MechanicDurationMod => 232,
            Self::Unknown233 => 233,
            Self::MechanicDurationModNotStack => 234,
            Self::ModDispelResist => 235,
            Self::Unknown236 => 236,
            Self::ModSpellDamageOfAttackPower => 237,
            Self::ModSpellHealingOfAttackPower => 238,
            Self::ModScale2 => 239,
            Self::ModExpertise => 240,
            Self::ForceMoveForward => 241,
            Self::Unknown242 => 242,
            Self::Unknown243 => 243,
            Self::ComprehendLanguage => 244,
            Self::Unknown245 => 245,
            Self::Unknown246 => 246,
            Self::MirrorImage => 247,
            Self::ModCombatResultChance => 248,
            Self::Unknown249 => 249,
            Self::ModIncreaseHealth2 => 250,
            Self::ModEnemyDodge => 251,
            Self::Unknown252 => 252,
            Self::Unknown253 => 253,
            Self::Unknown254 => 254,
            Self::Unknown255 => 255,
            Self::Unknown256 => 256,
            Self::Unknown257 => 257,
            Self::Unknown258 => 258,
            Self::Unknown259 => 259,
            Self::Unknown260 => 260,
            Self::Unknown261 => 261,
        }
    }

}

impl AuraLog_AuraType {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::None => {
                4
            }
            Self::BindSight => {
                4
            }
            Self::ModPossess => {
                4
            }
            Self::PeriodicDamage {
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
            Self::Dummy => {
                4
            }
            Self::ModConfuse => {
                4
            }
            Self::ModCharm => {
                4
            }
            Self::ModFear => {
                4
            }
            Self::PeriodicHeal {
                damage2,
            } => {
                4
                + 4 // damage2: u32
            }
            Self::ModAttackspeed => {
                4
            }
            Self::ModThreat => {
                4
            }
            Self::ModTaunt => {
                4
            }
            Self::ModStun => {
                4
            }
            Self::ModDamageDone => {
                4
            }
            Self::ModDamageTaken => {
                4
            }
            Self::DamageShield => {
                4
            }
            Self::ModStealth => {
                4
            }
            Self::ModStealthDetect => {
                4
            }
            Self::ModInvisibility => {
                4
            }
            Self::ModInvisibilityDetection => {
                4
            }
            Self::ObsModHealth {
                damage2,
            } => {
                4
                + 4 // damage2: u32
            }
            Self::ObsModMana {
                damage3,
                misc_value1,
            } => {
                4
                + 4 // damage3: u32
                + 4 // misc_value1: u32
            }
            Self::ModResistance => {
                4
            }
            Self::PeriodicTriggerSpell => {
                4
            }
            Self::PeriodicEnergize {
                damage3,
                misc_value1,
            } => {
                4
                + 4 // damage3: u32
                + 4 // misc_value1: u32
            }
            Self::ModPacify => {
                4
            }
            Self::ModRoot => {
                4
            }
            Self::ModSilence => {
                4
            }
            Self::ReflectSpells => {
                4
            }
            Self::ModStat => {
                4
            }
            Self::ModSkill => {
                4
            }
            Self::ModIncreaseSpeed => {
                4
            }
            Self::ModIncreaseMountedSpeed => {
                4
            }
            Self::ModDecreaseSpeed => {
                4
            }
            Self::ModIncreaseHealth => {
                4
            }
            Self::ModIncreaseEnergy => {
                4
            }
            Self::ModShapeshift => {
                4
            }
            Self::EffectImmunity => {
                4
            }
            Self::StateImmunity => {
                4
            }
            Self::SchoolImmunity => {
                4
            }
            Self::DamageImmunity => {
                4
            }
            Self::DispelImmunity => {
                4
            }
            Self::ProcTriggerSpell => {
                4
            }
            Self::ProcTriggerDamage => {
                4
            }
            Self::TrackCreatures => {
                4
            }
            Self::TrackResources => {
                4
            }
            Self::Unknown46 => {
                4
            }
            Self::ModParryPercent => {
                4
            }
            Self::Unknown48 => {
                4
            }
            Self::ModDodgePercent => {
                4
            }
            Self::ModBlockSkill => {
                4
            }
            Self::ModBlockPercent => {
                4
            }
            Self::ModCritPercent => {
                4
            }
            Self::PeriodicLeech => {
                4
            }
            Self::ModHitChance => {
                4
            }
            Self::ModSpellHitChance => {
                4
            }
            Self::Transform => {
                4
            }
            Self::ModSpellCritChance => {
                4
            }
            Self::ModIncreaseSwimSpeed => {
                4
            }
            Self::ModDamageDoneCreature => {
                4
            }
            Self::ModPacifySilence => {
                4
            }
            Self::ModScale => {
                4
            }
            Self::PeriodicHealthFunnel => {
                4
            }
            Self::PeriodicManaFunnel => {
                4
            }
            Self::PeriodicManaLeech {
                damage,
                gain_multiplier,
                misc_value2,
            } => {
                4
                + 4 // damage: u32
                + 4 // gain_multiplier: f32
                + 4 // misc_value2: u32
            }
            Self::ModCastingSpeedNotStack => {
                4
            }
            Self::FeignDeath => {
                4
            }
            Self::ModDisarm => {
                4
            }
            Self::ModStalked => {
                4
            }
            Self::SchoolAbsorb => {
                4
            }
            Self::ExtraAttacks => {
                4
            }
            Self::ModSpellCritChanceSchool => {
                4
            }
            Self::ModPowerCostSchoolPct => {
                4
            }
            Self::ModPowerCostSchool => {
                4
            }
            Self::ReflectSpellsSchool => {
                4
            }
            Self::ModLanguage => {
                4
            }
            Self::FarSight => {
                4
            }
            Self::MechanicImmunity => {
                4
            }
            Self::Mounted => {
                4
            }
            Self::ModDamagePercentDone => {
                4
            }
            Self::ModPercentStat => {
                4
            }
            Self::SplitDamagePct => {
                4
            }
            Self::WaterBreathing => {
                4
            }
            Self::ModBaseResistance => {
                4
            }
            Self::ModRegen => {
                4
            }
            Self::ModPowerRegen => {
                4
            }
            Self::ChannelDeathItem => {
                4
            }
            Self::ModDamagePercentTaken => {
                4
            }
            Self::ModHealthRegenPercent => {
                4
            }
            Self::PeriodicDamagePercent {
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
            Self::ModResistChance => {
                4
            }
            Self::ModDetectRange => {
                4
            }
            Self::PreventsFleeing => {
                4
            }
            Self::ModUnattackable => {
                4
            }
            Self::InterruptRegen => {
                4
            }
            Self::Ghost => {
                4
            }
            Self::SpellMagnet => {
                4
            }
            Self::ManaShield => {
                4
            }
            Self::ModSkillTalent => {
                4
            }
            Self::ModAttackPower => {
                4
            }
            Self::AurasVisible => {
                4
            }
            Self::ModResistancePct => {
                4
            }
            Self::ModMeleeAttackPowerVersus => {
                4
            }
            Self::ModTotalThreat => {
                4
            }
            Self::WaterWalk => {
                4
            }
            Self::FeatherFall => {
                4
            }
            Self::Hover => {
                4
            }
            Self::AddFlatModifier => {
                4
            }
            Self::AddPctModifier => {
                4
            }
            Self::AddTargetTrigger => {
                4
            }
            Self::ModPowerRegenPercent => {
                4
            }
            Self::AddCasterHitTrigger => {
                4
            }
            Self::OverrideClassScripts => {
                4
            }
            Self::ModRangedDamageTaken => {
                4
            }
            Self::ModRangedDamageTakenPct => {
                4
            }
            Self::ModHealing => {
                4
            }
            Self::ModRegenDuringCombat => {
                4
            }
            Self::ModMechanicResistance => {
                4
            }
            Self::ModHealingPct => {
                4
            }
            Self::SharePetTracking => {
                4
            }
            Self::Untrackable => {
                4
            }
            Self::Empathy => {
                4
            }
            Self::ModOffhandDamagePct => {
                4
            }
            Self::ModTargetResistance => {
                4
            }
            Self::ModRangedAttackPower => {
                4
            }
            Self::ModMeleeDamageTaken => {
                4
            }
            Self::ModMeleeDamageTakenPct => {
                4
            }
            Self::RangedAttackPowerAttackerBonus => {
                4
            }
            Self::ModPossessPet => {
                4
            }
            Self::ModSpeedAlways => {
                4
            }
            Self::ModMountedSpeedAlways => {
                4
            }
            Self::ModRangedAttackPowerVersus => {
                4
            }
            Self::ModIncreaseEnergyPercent => {
                4
            }
            Self::ModIncreaseHealthPercent => {
                4
            }
            Self::ModManaRegenInterrupt => {
                4
            }
            Self::ModHealingDone => {
                4
            }
            Self::ModHealingDonePercent => {
                4
            }
            Self::ModTotalStatPercentage => {
                4
            }
            Self::ModMeleeHaste => {
                4
            }
            Self::ForceReaction => {
                4
            }
            Self::ModRangedHaste => {
                4
            }
            Self::ModRangedAmmoHaste => {
                4
            }
            Self::ModBaseResistancePct => {
                4
            }
            Self::ModResistanceExclusive => {
                4
            }
            Self::SafeFall => {
                4
            }
            Self::Charisma => {
                4
            }
            Self::Persuaded => {
                4
            }
            Self::MechanicImmunityMask => {
                4
            }
            Self::RetainComboPoints => {
                4
            }
            Self::ResistPushback => {
                4
            }
            Self::ModShieldBlockvaluePct => {
                4
            }
            Self::TrackStealthed => {
                4
            }
            Self::ModDetectedRange => {
                4
            }
            Self::SplitDamageFlat => {
                4
            }
            Self::ModStealthLevel => {
                4
            }
            Self::ModWaterBreathing => {
                4
            }
            Self::ModReputationGain => {
                4
            }
            Self::PetDamageMulti => {
                4
            }
            Self::ModShieldBlockvalue => {
                4
            }
            Self::NoPvpCredit => {
                4
            }
            Self::ModAoeAvoidance => {
                4
            }
            Self::ModHealthRegenInCombat => {
                4
            }
            Self::PowerBurnMana => {
                4
            }
            Self::ModCritDamageBonus => {
                4
            }
            Self::Unknown164 => {
                4
            }
            Self::MeleeAttackPowerAttackerBonus => {
                4
            }
            Self::ModAttackPowerPct => {
                4
            }
            Self::ModRangedAttackPowerPct => {
                4
            }
            Self::ModDamageDoneVersus => {
                4
            }
            Self::ModCritPercentVersus => {
                4
            }
            Self::DetectAmore => {
                4
            }
            Self::ModSpeedNotStack => {
                4
            }
            Self::ModMountedSpeedNotStack => {
                4
            }
            Self::AllowChampionSpells => {
                4
            }
            Self::ModSpellDamageOfStatPercent => {
                4
            }
            Self::ModSpellHealingOfStatPercent => {
                4
            }
            Self::SpiritOfRedemption => {
                4
            }
            Self::AoeCharm => {
                4
            }
            Self::ModDebuffResistance => {
                4
            }
            Self::ModAttackerSpellCritChance => {
                4
            }
            Self::ModFlatSpellDamageVersus => {
                4
            }
            Self::ModFlatSpellCritDamageVersus => {
                4
            }
            Self::ModResistanceOfStatPercent => {
                4
            }
            Self::ModCriticalThreat => {
                4
            }
            Self::ModAttackerMeleeHitChance => {
                4
            }
            Self::ModAttackerRangedHitChance => {
                4
            }
            Self::ModAttackerSpellHitChance => {
                4
            }
            Self::ModAttackerMeleeCritChance => {
                4
            }
            Self::ModAttackerRangedCritChance => {
                4
            }
            Self::ModRating => {
                4
            }
            Self::ModFactionReputationGain => {
                4
            }
            Self::UseNormalMovementSpeed => {
                4
            }
            Self::ModMeleeRangedHaste => {
                4
            }
            Self::HasteAll => {
                4
            }
            Self::ModDepricated1 => {
                4
            }
            Self::ModDepricated2 => {
                4
            }
            Self::ModCooldown => {
                4
            }
            Self::ModAttackerSpellAndWeaponCritChance => {
                4
            }
            Self::ModAllWeaponSkills => {
                4
            }
            Self::ModIncreasesSpellPctToHit => {
                4
            }
            Self::ModXpPct => {
                4
            }
            Self::Fly => {
                4
            }
            Self::IgnoreCombatResult => {
                4
            }
            Self::ModAttackerMeleeCritDamage => {
                4
            }
            Self::ModAttackerRangedCritDamage => {
                4
            }
            Self::ModAttackerSpellCritDamage => {
                4
            }
            Self::ModFlightSpeed => {
                4
            }
            Self::ModFlightSpeedMounted => {
                4
            }
            Self::ModFlightSpeedStacking => {
                4
            }
            Self::ModFlightSpeedMountedStacking => {
                4
            }
            Self::ModFlightSpeedNotStacking => {
                4
            }
            Self::ModFlightSpeedMountedNotStacking => {
                4
            }
            Self::ModRangedAttackPowerOfStatPercent => {
                4
            }
            Self::ModRageFromDamageDealt => {
                4
            }
            Self::Unknown214 => {
                4
            }
            Self::ArenaPreparation => {
                4
            }
            Self::HasteSpells => {
                4
            }
            Self::Unknown217 => {
                4
            }
            Self::HasteRanged => {
                4
            }
            Self::ModManaRegenFromStat => {
                4
            }
            Self::ModRatingFromStat => {
                4
            }
            Self::Unknown221 => {
                4
            }
            Self::Unknown222 => {
                4
            }
            Self::Unknown223 => {
                4
            }
            Self::Unknown224 => {
                4
            }
            Self::PrayerOfMending => {
                4
            }
            Self::PeriodicDummy => {
                4
            }
            Self::PeriodicTriggerSpellWithValue => {
                4
            }
            Self::DetectStealth => {
                4
            }
            Self::ModAoeDamageAvoidance => {
                4
            }
            Self::Unknown230 => {
                4
            }
            Self::ProcTriggerSpellWithValue => {
                4
            }
            Self::MechanicDurationMod => {
                4
            }
            Self::Unknown233 => {
                4
            }
            Self::MechanicDurationModNotStack => {
                4
            }
            Self::ModDispelResist => {
                4
            }
            Self::Unknown236 => {
                4
            }
            Self::ModSpellDamageOfAttackPower => {
                4
            }
            Self::ModSpellHealingOfAttackPower => {
                4
            }
            Self::ModScale2 => {
                4
            }
            Self::ModExpertise => {
                4
            }
            Self::ForceMoveForward => {
                4
            }
            Self::Unknown242 => {
                4
            }
            Self::Unknown243 => {
                4
            }
            Self::ComprehendLanguage => {
                4
            }
            Self::Unknown245 => {
                4
            }
            Self::Unknown246 => {
                4
            }
            Self::MirrorImage => {
                4
            }
            Self::ModCombatResultChance => {
                4
            }
            Self::Unknown249 => {
                4
            }
            Self::ModIncreaseHealth2 => {
                4
            }
            Self::ModEnemyDodge => {
                4
            }
            Self::Unknown252 => {
                4
            }
            Self::Unknown253 => {
                4
            }
            Self::Unknown254 => {
                4
            }
            Self::Unknown255 => {
                4
            }
            Self::Unknown256 => {
                4
            }
            Self::Unknown257 => {
                4
            }
            Self::Unknown258 => {
                4
            }
            Self::Unknown259 => {
                4
            }
            Self::Unknown260 => {
                4
            }
            Self::Unknown261 => {
                4
            }
        }
    }
}

