use std::io::{Read, Write};

use crate::tbc::{
    AuraType, SpellSchool,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm:579`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm#L579):
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
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum AuraLog {
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

impl AuraLog {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // aura_type: AuraType
        w.write_all(&(self.as_int().to_le_bytes()))?;

        match &self {
            AuraLog::PeriodicDamage {
                absorbed,
                damage1,
                resisted,
                school,
            } => {
                // damage1: u32
                w.write_all(&damage1.to_le_bytes())?;

                // school: SpellSchool
                w.write_all(&(school.as_int().to_le_bytes()))?;

                // absorbed: u32
                w.write_all(&absorbed.to_le_bytes())?;

                // resisted: u32
                w.write_all(&resisted.to_le_bytes())?;

            }
            AuraLog::PeriodicHeal {
                damage2,
            } => {
                // damage2: u32
                w.write_all(&damage2.to_le_bytes())?;

            }
            AuraLog::ObsModHealth {
                damage2,
            } => {
                // damage2: u32
                w.write_all(&damage2.to_le_bytes())?;

            }
            AuraLog::ObsModMana {
                damage3,
                misc_value1,
            } => {
                // misc_value1: u32
                w.write_all(&misc_value1.to_le_bytes())?;

                // damage3: u32
                w.write_all(&damage3.to_le_bytes())?;

            }
            AuraLog::PeriodicEnergize {
                damage3,
                misc_value1,
            } => {
                // misc_value1: u32
                w.write_all(&misc_value1.to_le_bytes())?;

                // damage3: u32
                w.write_all(&damage3.to_le_bytes())?;

            }
            AuraLog::PeriodicManaLeech {
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
            AuraLog::PeriodicDamagePercent {
                absorbed,
                damage1,
                resisted,
                school,
            } => {
                // damage1: u32
                w.write_all(&damage1.to_le_bytes())?;

                // school: SpellSchool
                w.write_all(&(school.as_int().to_le_bytes()))?;

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
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // aura_type: AuraType
        let aura_type = crate::util::read_u32_le(&mut r)?.try_into()?;

        let aura_type_if = match aura_type {
            AuraType::None => AuraLog::None,
            AuraType::BindSight => AuraLog::BindSight,
            AuraType::ModPossess => AuraLog::ModPossess,
            AuraType::PeriodicDamage => {
                // damage1: u32
                let damage1 = crate::util::read_u32_le(&mut r)?;

                // school: SpellSchool
                let school = crate::util::read_u8_le(&mut r)?.try_into()?;

                // absorbed: u32
                let absorbed = crate::util::read_u32_le(&mut r)?;

                // resisted: u32
                let resisted = crate::util::read_u32_le(&mut r)?;

                AuraLog::PeriodicDamage {
                    absorbed,
                    damage1,
                    resisted,
                    school,
                }
            }
            AuraType::Dummy => AuraLog::Dummy,
            AuraType::ModConfuse => AuraLog::ModConfuse,
            AuraType::ModCharm => AuraLog::ModCharm,
            AuraType::ModFear => AuraLog::ModFear,
            AuraType::PeriodicHeal => {
                // damage2: u32
                let damage2 = crate::util::read_u32_le(&mut r)?;

                AuraLog::PeriodicHeal {
                    damage2,
                }
            }
            AuraType::ModAttackspeed => AuraLog::ModAttackspeed,
            AuraType::ModThreat => AuraLog::ModThreat,
            AuraType::ModTaunt => AuraLog::ModTaunt,
            AuraType::ModStun => AuraLog::ModStun,
            AuraType::ModDamageDone => AuraLog::ModDamageDone,
            AuraType::ModDamageTaken => AuraLog::ModDamageTaken,
            AuraType::DamageShield => AuraLog::DamageShield,
            AuraType::ModStealth => AuraLog::ModStealth,
            AuraType::ModStealthDetect => AuraLog::ModStealthDetect,
            AuraType::ModInvisibility => AuraLog::ModInvisibility,
            AuraType::ModInvisibilityDetection => AuraLog::ModInvisibilityDetection,
            AuraType::ObsModHealth => {
                // damage2: u32
                let damage2 = crate::util::read_u32_le(&mut r)?;

                AuraLog::ObsModHealth {
                    damage2,
                }
            }
            AuraType::ObsModMana => {
                // misc_value1: u32
                let misc_value1 = crate::util::read_u32_le(&mut r)?;

                // damage3: u32
                let damage3 = crate::util::read_u32_le(&mut r)?;

                AuraLog::ObsModMana {
                    damage3,
                    misc_value1,
                }
            }
            AuraType::ModResistance => AuraLog::ModResistance,
            AuraType::PeriodicTriggerSpell => AuraLog::PeriodicTriggerSpell,
            AuraType::PeriodicEnergize => {
                // misc_value1: u32
                let misc_value1 = crate::util::read_u32_le(&mut r)?;

                // damage3: u32
                let damage3 = crate::util::read_u32_le(&mut r)?;

                AuraLog::PeriodicEnergize {
                    damage3,
                    misc_value1,
                }
            }
            AuraType::ModPacify => AuraLog::ModPacify,
            AuraType::ModRoot => AuraLog::ModRoot,
            AuraType::ModSilence => AuraLog::ModSilence,
            AuraType::ReflectSpells => AuraLog::ReflectSpells,
            AuraType::ModStat => AuraLog::ModStat,
            AuraType::ModSkill => AuraLog::ModSkill,
            AuraType::ModIncreaseSpeed => AuraLog::ModIncreaseSpeed,
            AuraType::ModIncreaseMountedSpeed => AuraLog::ModIncreaseMountedSpeed,
            AuraType::ModDecreaseSpeed => AuraLog::ModDecreaseSpeed,
            AuraType::ModIncreaseHealth => AuraLog::ModIncreaseHealth,
            AuraType::ModIncreaseEnergy => AuraLog::ModIncreaseEnergy,
            AuraType::ModShapeshift => AuraLog::ModShapeshift,
            AuraType::EffectImmunity => AuraLog::EffectImmunity,
            AuraType::StateImmunity => AuraLog::StateImmunity,
            AuraType::SchoolImmunity => AuraLog::SchoolImmunity,
            AuraType::DamageImmunity => AuraLog::DamageImmunity,
            AuraType::DispelImmunity => AuraLog::DispelImmunity,
            AuraType::ProcTriggerSpell => AuraLog::ProcTriggerSpell,
            AuraType::ProcTriggerDamage => AuraLog::ProcTriggerDamage,
            AuraType::TrackCreatures => AuraLog::TrackCreatures,
            AuraType::TrackResources => AuraLog::TrackResources,
            AuraType::Unknown46 => AuraLog::Unknown46,
            AuraType::ModParryPercent => AuraLog::ModParryPercent,
            AuraType::Unknown48 => AuraLog::Unknown48,
            AuraType::ModDodgePercent => AuraLog::ModDodgePercent,
            AuraType::ModBlockSkill => AuraLog::ModBlockSkill,
            AuraType::ModBlockPercent => AuraLog::ModBlockPercent,
            AuraType::ModCritPercent => AuraLog::ModCritPercent,
            AuraType::PeriodicLeech => AuraLog::PeriodicLeech,
            AuraType::ModHitChance => AuraLog::ModHitChance,
            AuraType::ModSpellHitChance => AuraLog::ModSpellHitChance,
            AuraType::Transform => AuraLog::Transform,
            AuraType::ModSpellCritChance => AuraLog::ModSpellCritChance,
            AuraType::ModIncreaseSwimSpeed => AuraLog::ModIncreaseSwimSpeed,
            AuraType::ModDamageDoneCreature => AuraLog::ModDamageDoneCreature,
            AuraType::ModPacifySilence => AuraLog::ModPacifySilence,
            AuraType::ModScale => AuraLog::ModScale,
            AuraType::PeriodicHealthFunnel => AuraLog::PeriodicHealthFunnel,
            AuraType::PeriodicManaFunnel => AuraLog::PeriodicManaFunnel,
            AuraType::PeriodicManaLeech => {
                // misc_value2: u32
                let misc_value2 = crate::util::read_u32_le(&mut r)?;

                // damage: u32
                let damage = crate::util::read_u32_le(&mut r)?;

                // gain_multiplier: f32
                let gain_multiplier = crate::util::read_f32_le(&mut r)?;

                AuraLog::PeriodicManaLeech {
                    damage,
                    gain_multiplier,
                    misc_value2,
                }
            }
            AuraType::ModCastingSpeedNotStack => AuraLog::ModCastingSpeedNotStack,
            AuraType::FeignDeath => AuraLog::FeignDeath,
            AuraType::ModDisarm => AuraLog::ModDisarm,
            AuraType::ModStalked => AuraLog::ModStalked,
            AuraType::SchoolAbsorb => AuraLog::SchoolAbsorb,
            AuraType::ExtraAttacks => AuraLog::ExtraAttacks,
            AuraType::ModSpellCritChanceSchool => AuraLog::ModSpellCritChanceSchool,
            AuraType::ModPowerCostSchoolPct => AuraLog::ModPowerCostSchoolPct,
            AuraType::ModPowerCostSchool => AuraLog::ModPowerCostSchool,
            AuraType::ReflectSpellsSchool => AuraLog::ReflectSpellsSchool,
            AuraType::ModLanguage => AuraLog::ModLanguage,
            AuraType::FarSight => AuraLog::FarSight,
            AuraType::MechanicImmunity => AuraLog::MechanicImmunity,
            AuraType::Mounted => AuraLog::Mounted,
            AuraType::ModDamagePercentDone => AuraLog::ModDamagePercentDone,
            AuraType::ModPercentStat => AuraLog::ModPercentStat,
            AuraType::SplitDamagePct => AuraLog::SplitDamagePct,
            AuraType::WaterBreathing => AuraLog::WaterBreathing,
            AuraType::ModBaseResistance => AuraLog::ModBaseResistance,
            AuraType::ModRegen => AuraLog::ModRegen,
            AuraType::ModPowerRegen => AuraLog::ModPowerRegen,
            AuraType::ChannelDeathItem => AuraLog::ChannelDeathItem,
            AuraType::ModDamagePercentTaken => AuraLog::ModDamagePercentTaken,
            AuraType::ModHealthRegenPercent => AuraLog::ModHealthRegenPercent,
            AuraType::PeriodicDamagePercent => {
                // damage1: u32
                let damage1 = crate::util::read_u32_le(&mut r)?;

                // school: SpellSchool
                let school = crate::util::read_u8_le(&mut r)?.try_into()?;

                // absorbed: u32
                let absorbed = crate::util::read_u32_le(&mut r)?;

                // resisted: u32
                let resisted = crate::util::read_u32_le(&mut r)?;

                AuraLog::PeriodicDamagePercent {
                    absorbed,
                    damage1,
                    resisted,
                    school,
                }
            }
            AuraType::ModResistChance => AuraLog::ModResistChance,
            AuraType::ModDetectRange => AuraLog::ModDetectRange,
            AuraType::PreventsFleeing => AuraLog::PreventsFleeing,
            AuraType::ModUnattackable => AuraLog::ModUnattackable,
            AuraType::InterruptRegen => AuraLog::InterruptRegen,
            AuraType::Ghost => AuraLog::Ghost,
            AuraType::SpellMagnet => AuraLog::SpellMagnet,
            AuraType::ManaShield => AuraLog::ManaShield,
            AuraType::ModSkillTalent => AuraLog::ModSkillTalent,
            AuraType::ModAttackPower => AuraLog::ModAttackPower,
            AuraType::AurasVisible => AuraLog::AurasVisible,
            AuraType::ModResistancePct => AuraLog::ModResistancePct,
            AuraType::ModMeleeAttackPowerVersus => AuraLog::ModMeleeAttackPowerVersus,
            AuraType::ModTotalThreat => AuraLog::ModTotalThreat,
            AuraType::WaterWalk => AuraLog::WaterWalk,
            AuraType::FeatherFall => AuraLog::FeatherFall,
            AuraType::Hover => AuraLog::Hover,
            AuraType::AddFlatModifier => AuraLog::AddFlatModifier,
            AuraType::AddPctModifier => AuraLog::AddPctModifier,
            AuraType::AddTargetTrigger => AuraLog::AddTargetTrigger,
            AuraType::ModPowerRegenPercent => AuraLog::ModPowerRegenPercent,
            AuraType::AddCasterHitTrigger => AuraLog::AddCasterHitTrigger,
            AuraType::OverrideClassScripts => AuraLog::OverrideClassScripts,
            AuraType::ModRangedDamageTaken => AuraLog::ModRangedDamageTaken,
            AuraType::ModRangedDamageTakenPct => AuraLog::ModRangedDamageTakenPct,
            AuraType::ModHealing => AuraLog::ModHealing,
            AuraType::ModRegenDuringCombat => AuraLog::ModRegenDuringCombat,
            AuraType::ModMechanicResistance => AuraLog::ModMechanicResistance,
            AuraType::ModHealingPct => AuraLog::ModHealingPct,
            AuraType::SharePetTracking => AuraLog::SharePetTracking,
            AuraType::Untrackable => AuraLog::Untrackable,
            AuraType::Empathy => AuraLog::Empathy,
            AuraType::ModOffhandDamagePct => AuraLog::ModOffhandDamagePct,
            AuraType::ModTargetResistance => AuraLog::ModTargetResistance,
            AuraType::ModRangedAttackPower => AuraLog::ModRangedAttackPower,
            AuraType::ModMeleeDamageTaken => AuraLog::ModMeleeDamageTaken,
            AuraType::ModMeleeDamageTakenPct => AuraLog::ModMeleeDamageTakenPct,
            AuraType::RangedAttackPowerAttackerBonus => AuraLog::RangedAttackPowerAttackerBonus,
            AuraType::ModPossessPet => AuraLog::ModPossessPet,
            AuraType::ModSpeedAlways => AuraLog::ModSpeedAlways,
            AuraType::ModMountedSpeedAlways => AuraLog::ModMountedSpeedAlways,
            AuraType::ModRangedAttackPowerVersus => AuraLog::ModRangedAttackPowerVersus,
            AuraType::ModIncreaseEnergyPercent => AuraLog::ModIncreaseEnergyPercent,
            AuraType::ModIncreaseHealthPercent => AuraLog::ModIncreaseHealthPercent,
            AuraType::ModManaRegenInterrupt => AuraLog::ModManaRegenInterrupt,
            AuraType::ModHealingDone => AuraLog::ModHealingDone,
            AuraType::ModHealingDonePercent => AuraLog::ModHealingDonePercent,
            AuraType::ModTotalStatPercentage => AuraLog::ModTotalStatPercentage,
            AuraType::ModMeleeHaste => AuraLog::ModMeleeHaste,
            AuraType::ForceReaction => AuraLog::ForceReaction,
            AuraType::ModRangedHaste => AuraLog::ModRangedHaste,
            AuraType::ModRangedAmmoHaste => AuraLog::ModRangedAmmoHaste,
            AuraType::ModBaseResistancePct => AuraLog::ModBaseResistancePct,
            AuraType::ModResistanceExclusive => AuraLog::ModResistanceExclusive,
            AuraType::SafeFall => AuraLog::SafeFall,
            AuraType::Charisma => AuraLog::Charisma,
            AuraType::Persuaded => AuraLog::Persuaded,
            AuraType::MechanicImmunityMask => AuraLog::MechanicImmunityMask,
            AuraType::RetainComboPoints => AuraLog::RetainComboPoints,
            AuraType::ResistPushback => AuraLog::ResistPushback,
            AuraType::ModShieldBlockvaluePct => AuraLog::ModShieldBlockvaluePct,
            AuraType::TrackStealthed => AuraLog::TrackStealthed,
            AuraType::ModDetectedRange => AuraLog::ModDetectedRange,
            AuraType::SplitDamageFlat => AuraLog::SplitDamageFlat,
            AuraType::ModStealthLevel => AuraLog::ModStealthLevel,
            AuraType::ModWaterBreathing => AuraLog::ModWaterBreathing,
            AuraType::ModReputationGain => AuraLog::ModReputationGain,
            AuraType::PetDamageMulti => AuraLog::PetDamageMulti,
            AuraType::ModShieldBlockvalue => AuraLog::ModShieldBlockvalue,
            AuraType::NoPvpCredit => AuraLog::NoPvpCredit,
            AuraType::ModAoeAvoidance => AuraLog::ModAoeAvoidance,
            AuraType::ModHealthRegenInCombat => AuraLog::ModHealthRegenInCombat,
            AuraType::PowerBurnMana => AuraLog::PowerBurnMana,
            AuraType::ModCritDamageBonus => AuraLog::ModCritDamageBonus,
            AuraType::Unknown164 => AuraLog::Unknown164,
            AuraType::MeleeAttackPowerAttackerBonus => AuraLog::MeleeAttackPowerAttackerBonus,
            AuraType::ModAttackPowerPct => AuraLog::ModAttackPowerPct,
            AuraType::ModRangedAttackPowerPct => AuraLog::ModRangedAttackPowerPct,
            AuraType::ModDamageDoneVersus => AuraLog::ModDamageDoneVersus,
            AuraType::ModCritPercentVersus => AuraLog::ModCritPercentVersus,
            AuraType::DetectAmore => AuraLog::DetectAmore,
            AuraType::ModSpeedNotStack => AuraLog::ModSpeedNotStack,
            AuraType::ModMountedSpeedNotStack => AuraLog::ModMountedSpeedNotStack,
            AuraType::AllowChampionSpells => AuraLog::AllowChampionSpells,
            AuraType::ModSpellDamageOfStatPercent => AuraLog::ModSpellDamageOfStatPercent,
            AuraType::ModSpellHealingOfStatPercent => AuraLog::ModSpellHealingOfStatPercent,
            AuraType::SpiritOfRedemption => AuraLog::SpiritOfRedemption,
            AuraType::AoeCharm => AuraLog::AoeCharm,
            AuraType::ModDebuffResistance => AuraLog::ModDebuffResistance,
            AuraType::ModAttackerSpellCritChance => AuraLog::ModAttackerSpellCritChance,
            AuraType::ModFlatSpellDamageVersus => AuraLog::ModFlatSpellDamageVersus,
            AuraType::ModFlatSpellCritDamageVersus => AuraLog::ModFlatSpellCritDamageVersus,
            AuraType::ModResistanceOfStatPercent => AuraLog::ModResistanceOfStatPercent,
            AuraType::ModCriticalThreat => AuraLog::ModCriticalThreat,
            AuraType::ModAttackerMeleeHitChance => AuraLog::ModAttackerMeleeHitChance,
            AuraType::ModAttackerRangedHitChance => AuraLog::ModAttackerRangedHitChance,
            AuraType::ModAttackerSpellHitChance => AuraLog::ModAttackerSpellHitChance,
            AuraType::ModAttackerMeleeCritChance => AuraLog::ModAttackerMeleeCritChance,
            AuraType::ModAttackerRangedCritChance => AuraLog::ModAttackerRangedCritChance,
            AuraType::ModRating => AuraLog::ModRating,
            AuraType::ModFactionReputationGain => AuraLog::ModFactionReputationGain,
            AuraType::UseNormalMovementSpeed => AuraLog::UseNormalMovementSpeed,
            AuraType::ModMeleeRangedHaste => AuraLog::ModMeleeRangedHaste,
            AuraType::HasteAll => AuraLog::HasteAll,
            AuraType::ModDepricated1 => AuraLog::ModDepricated1,
            AuraType::ModDepricated2 => AuraLog::ModDepricated2,
            AuraType::ModCooldown => AuraLog::ModCooldown,
            AuraType::ModAttackerSpellAndWeaponCritChance => AuraLog::ModAttackerSpellAndWeaponCritChance,
            AuraType::ModAllWeaponSkills => AuraLog::ModAllWeaponSkills,
            AuraType::ModIncreasesSpellPctToHit => AuraLog::ModIncreasesSpellPctToHit,
            AuraType::ModXpPct => AuraLog::ModXpPct,
            AuraType::Fly => AuraLog::Fly,
            AuraType::IgnoreCombatResult => AuraLog::IgnoreCombatResult,
            AuraType::ModAttackerMeleeCritDamage => AuraLog::ModAttackerMeleeCritDamage,
            AuraType::ModAttackerRangedCritDamage => AuraLog::ModAttackerRangedCritDamage,
            AuraType::ModAttackerSpellCritDamage => AuraLog::ModAttackerSpellCritDamage,
            AuraType::ModFlightSpeed => AuraLog::ModFlightSpeed,
            AuraType::ModFlightSpeedMounted => AuraLog::ModFlightSpeedMounted,
            AuraType::ModFlightSpeedStacking => AuraLog::ModFlightSpeedStacking,
            AuraType::ModFlightSpeedMountedStacking => AuraLog::ModFlightSpeedMountedStacking,
            AuraType::ModFlightSpeedNotStacking => AuraLog::ModFlightSpeedNotStacking,
            AuraType::ModFlightSpeedMountedNotStacking => AuraLog::ModFlightSpeedMountedNotStacking,
            AuraType::ModRangedAttackPowerOfStatPercent => AuraLog::ModRangedAttackPowerOfStatPercent,
            AuraType::ModRageFromDamageDealt => AuraLog::ModRageFromDamageDealt,
            AuraType::Unknown214 => AuraLog::Unknown214,
            AuraType::ArenaPreparation => AuraLog::ArenaPreparation,
            AuraType::HasteSpells => AuraLog::HasteSpells,
            AuraType::Unknown217 => AuraLog::Unknown217,
            AuraType::HasteRanged => AuraLog::HasteRanged,
            AuraType::ModManaRegenFromStat => AuraLog::ModManaRegenFromStat,
            AuraType::ModRatingFromStat => AuraLog::ModRatingFromStat,
            AuraType::Unknown221 => AuraLog::Unknown221,
            AuraType::Unknown222 => AuraLog::Unknown222,
            AuraType::Unknown223 => AuraLog::Unknown223,
            AuraType::Unknown224 => AuraLog::Unknown224,
            AuraType::PrayerOfMending => AuraLog::PrayerOfMending,
            AuraType::PeriodicDummy => AuraLog::PeriodicDummy,
            AuraType::PeriodicTriggerSpellWithValue => AuraLog::PeriodicTriggerSpellWithValue,
            AuraType::DetectStealth => AuraLog::DetectStealth,
            AuraType::ModAoeDamageAvoidance => AuraLog::ModAoeDamageAvoidance,
            AuraType::Unknown230 => AuraLog::Unknown230,
            AuraType::ProcTriggerSpellWithValue => AuraLog::ProcTriggerSpellWithValue,
            AuraType::MechanicDurationMod => AuraLog::MechanicDurationMod,
            AuraType::Unknown233 => AuraLog::Unknown233,
            AuraType::MechanicDurationModNotStack => AuraLog::MechanicDurationModNotStack,
            AuraType::ModDispelResist => AuraLog::ModDispelResist,
            AuraType::Unknown236 => AuraLog::Unknown236,
            AuraType::ModSpellDamageOfAttackPower => AuraLog::ModSpellDamageOfAttackPower,
            AuraType::ModSpellHealingOfAttackPower => AuraLog::ModSpellHealingOfAttackPower,
            AuraType::ModScale2 => AuraLog::ModScale2,
            AuraType::ModExpertise => AuraLog::ModExpertise,
            AuraType::ForceMoveForward => AuraLog::ForceMoveForward,
            AuraType::Unknown242 => AuraLog::Unknown242,
            AuraType::Unknown243 => AuraLog::Unknown243,
            AuraType::ComprehendLanguage => AuraLog::ComprehendLanguage,
            AuraType::Unknown245 => AuraLog::Unknown245,
            AuraType::Unknown246 => AuraLog::Unknown246,
            AuraType::MirrorImage => AuraLog::MirrorImage,
            AuraType::ModCombatResultChance => AuraLog::ModCombatResultChance,
            AuraType::Unknown249 => AuraLog::Unknown249,
            AuraType::ModIncreaseHealth2 => AuraLog::ModIncreaseHealth2,
            AuraType::ModEnemyDodge => AuraLog::ModEnemyDodge,
            AuraType::Unknown252 => AuraLog::Unknown252,
            AuraType::Unknown253 => AuraLog::Unknown253,
            AuraType::Unknown254 => AuraLog::Unknown254,
            AuraType::Unknown255 => AuraLog::Unknown255,
            AuraType::Unknown256 => AuraLog::Unknown256,
            AuraType::Unknown257 => AuraLog::Unknown257,
            AuraType::Unknown258 => AuraLog::Unknown258,
            AuraType::Unknown259 => AuraLog::Unknown259,
            AuraType::Unknown260 => AuraLog::Unknown260,
            AuraType::Unknown261 => AuraLog::Unknown261,
        };

        Ok(aura_type_if)
    }

}

impl AuraLog {
    pub(crate) const fn size(&self) -> usize {
        (match self {
            Self::PeriodicDamage {
                ..
            } => {
                4
                + 4 // absorbed: u32
                + 4 // damage1: u32
                + 4 // resisted: u32
                + 1 // school: SpellSchool
            }
            Self::PeriodicHeal {
                ..
            } => {
                4
                + 4 // damage2: u32
            }
            Self::ObsModHealth {
                ..
            } => {
                4
                + 4 // damage2: u32
            }
            Self::ObsModMana {
                ..
            } => {
                4
                + 4 // damage3: u32
                + 4 // misc_value1: u32
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
                + 4 // damage: u32
                + 4 // gain_multiplier: f32
                + 4 // misc_value2: u32
            }
            Self::PeriodicDamagePercent {
                ..
            } => {
                4
                + 4 // absorbed: u32
                + 4 // damage1: u32
                + 4 // resisted: u32
                + 1 // school: SpellSchool
            }
            _ => 4,
        }) // aura_type: AuraLog
    }
}

impl Default for AuraLog {
    fn default() -> Self {
        // First enumerator without any fields
        Self::None
    }
}

impl AuraLog {
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

impl std::fmt::Display for AuraLog {
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
            Self::ModInvisibilityDetection => f.write_str("ModInvisibilityDetection"),
            Self::ObsModHealth{ .. } => f.write_str("ObsModHealth"),
            Self::ObsModMana{ .. } => f.write_str("ObsModMana"),
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
            Self::Unknown48 => f.write_str("Unknown48"),
            Self::ModDodgePercent => f.write_str("ModDodgePercent"),
            Self::ModBlockSkill => f.write_str("ModBlockSkill"),
            Self::ModBlockPercent => f.write_str("ModBlockPercent"),
            Self::ModCritPercent => f.write_str("ModCritPercent"),
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
            Self::PeriodicManaFunnel => f.write_str("PeriodicManaFunnel"),
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
            Self::ModResistChance => f.write_str("ModResistChance"),
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
            Self::SharePetTracking => f.write_str("SharePetTracking"),
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
            Self::Charisma => f.write_str("Charisma"),
            Self::Persuaded => f.write_str("Persuaded"),
            Self::MechanicImmunityMask => f.write_str("MechanicImmunityMask"),
            Self::RetainComboPoints => f.write_str("RetainComboPoints"),
            Self::ResistPushback => f.write_str("ResistPushback"),
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
            Self::PowerBurnMana => f.write_str("PowerBurnMana"),
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
            Self::AllowChampionSpells => f.write_str("AllowChampionSpells"),
            Self::ModSpellDamageOfStatPercent => f.write_str("ModSpellDamageOfStatPercent"),
            Self::ModSpellHealingOfStatPercent => f.write_str("ModSpellHealingOfStatPercent"),
            Self::SpiritOfRedemption => f.write_str("SpiritOfRedemption"),
            Self::AoeCharm => f.write_str("AoeCharm"),
            Self::ModDebuffResistance => f.write_str("ModDebuffResistance"),
            Self::ModAttackerSpellCritChance => f.write_str("ModAttackerSpellCritChance"),
            Self::ModFlatSpellDamageVersus => f.write_str("ModFlatSpellDamageVersus"),
            Self::ModFlatSpellCritDamageVersus => f.write_str("ModFlatSpellCritDamageVersus"),
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
            Self::HasteAll => f.write_str("HasteAll"),
            Self::ModDepricated1 => f.write_str("ModDepricated1"),
            Self::ModDepricated2 => f.write_str("ModDepricated2"),
            Self::ModCooldown => f.write_str("ModCooldown"),
            Self::ModAttackerSpellAndWeaponCritChance => f.write_str("ModAttackerSpellAndWeaponCritChance"),
            Self::ModAllWeaponSkills => f.write_str("ModAllWeaponSkills"),
            Self::ModIncreasesSpellPctToHit => f.write_str("ModIncreasesSpellPctToHit"),
            Self::ModXpPct => f.write_str("ModXpPct"),
            Self::Fly => f.write_str("Fly"),
            Self::IgnoreCombatResult => f.write_str("IgnoreCombatResult"),
            Self::ModAttackerMeleeCritDamage => f.write_str("ModAttackerMeleeCritDamage"),
            Self::ModAttackerRangedCritDamage => f.write_str("ModAttackerRangedCritDamage"),
            Self::ModAttackerSpellCritDamage => f.write_str("ModAttackerSpellCritDamage"),
            Self::ModFlightSpeed => f.write_str("ModFlightSpeed"),
            Self::ModFlightSpeedMounted => f.write_str("ModFlightSpeedMounted"),
            Self::ModFlightSpeedStacking => f.write_str("ModFlightSpeedStacking"),
            Self::ModFlightSpeedMountedStacking => f.write_str("ModFlightSpeedMountedStacking"),
            Self::ModFlightSpeedNotStacking => f.write_str("ModFlightSpeedNotStacking"),
            Self::ModFlightSpeedMountedNotStacking => f.write_str("ModFlightSpeedMountedNotStacking"),
            Self::ModRangedAttackPowerOfStatPercent => f.write_str("ModRangedAttackPowerOfStatPercent"),
            Self::ModRageFromDamageDealt => f.write_str("ModRageFromDamageDealt"),
            Self::Unknown214 => f.write_str("Unknown214"),
            Self::ArenaPreparation => f.write_str("ArenaPreparation"),
            Self::HasteSpells => f.write_str("HasteSpells"),
            Self::Unknown217 => f.write_str("Unknown217"),
            Self::HasteRanged => f.write_str("HasteRanged"),
            Self::ModManaRegenFromStat => f.write_str("ModManaRegenFromStat"),
            Self::ModRatingFromStat => f.write_str("ModRatingFromStat"),
            Self::Unknown221 => f.write_str("Unknown221"),
            Self::Unknown222 => f.write_str("Unknown222"),
            Self::Unknown223 => f.write_str("Unknown223"),
            Self::Unknown224 => f.write_str("Unknown224"),
            Self::PrayerOfMending => f.write_str("PrayerOfMending"),
            Self::PeriodicDummy => f.write_str("PeriodicDummy"),
            Self::PeriodicTriggerSpellWithValue => f.write_str("PeriodicTriggerSpellWithValue"),
            Self::DetectStealth => f.write_str("DetectStealth"),
            Self::ModAoeDamageAvoidance => f.write_str("ModAoeDamageAvoidance"),
            Self::Unknown230 => f.write_str("Unknown230"),
            Self::ProcTriggerSpellWithValue => f.write_str("ProcTriggerSpellWithValue"),
            Self::MechanicDurationMod => f.write_str("MechanicDurationMod"),
            Self::Unknown233 => f.write_str("Unknown233"),
            Self::MechanicDurationModNotStack => f.write_str("MechanicDurationModNotStack"),
            Self::ModDispelResist => f.write_str("ModDispelResist"),
            Self::Unknown236 => f.write_str("Unknown236"),
            Self::ModSpellDamageOfAttackPower => f.write_str("ModSpellDamageOfAttackPower"),
            Self::ModSpellHealingOfAttackPower => f.write_str("ModSpellHealingOfAttackPower"),
            Self::ModScale2 => f.write_str("ModScale2"),
            Self::ModExpertise => f.write_str("ModExpertise"),
            Self::ForceMoveForward => f.write_str("ForceMoveForward"),
            Self::Unknown242 => f.write_str("Unknown242"),
            Self::Unknown243 => f.write_str("Unknown243"),
            Self::ComprehendLanguage => f.write_str("ComprehendLanguage"),
            Self::Unknown245 => f.write_str("Unknown245"),
            Self::Unknown246 => f.write_str("Unknown246"),
            Self::MirrorImage => f.write_str("MirrorImage"),
            Self::ModCombatResultChance => f.write_str("ModCombatResultChance"),
            Self::Unknown249 => f.write_str("Unknown249"),
            Self::ModIncreaseHealth2 => f.write_str("ModIncreaseHealth2"),
            Self::ModEnemyDodge => f.write_str("ModEnemyDodge"),
            Self::Unknown252 => f.write_str("Unknown252"),
            Self::Unknown253 => f.write_str("Unknown253"),
            Self::Unknown254 => f.write_str("Unknown254"),
            Self::Unknown255 => f.write_str("Unknown255"),
            Self::Unknown256 => f.write_str("Unknown256"),
            Self::Unknown257 => f.write_str("Unknown257"),
            Self::Unknown258 => f.write_str("Unknown258"),
            Self::Unknown259 => f.write_str("Unknown259"),
            Self::Unknown260 => f.write_str("Unknown260"),
            Self::Unknown261 => f.write_str("Unknown261"),
        }
    }
}

