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
///     else if (aura_type == OBS_MOD_POWER
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
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum AuraLog {
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
    ObsModPower {
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

impl AuraLog {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // aura_type: AuraType
        w.write_all(&(self.as_int().to_le_bytes()))?;

        match &self {
            AuraLog::PeriodicDamage {
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
            AuraLog::PeriodicHeal {
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
            AuraLog::ObsModHealth {
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
            AuraLog::ObsModPower {
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
            AuraLog::PeriodicDamagePercent {
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
            AuraType::None => AuraLog::None,
            AuraType::BindSight => AuraLog::BindSight,
            AuraType::ModPossess => AuraLog::ModPossess,
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

                AuraLog::PeriodicDamage {
                    absorb1,
                    critical1,
                    damage1,
                    overkill_damage,
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

                // over_damage: u32
                let over_damage = crate::util::read_u32_le(&mut r)?;

                // absorb2: u32
                let absorb2 = crate::util::read_u32_le(&mut r)?;

                // critical2: Bool
                let critical2 = crate::util::read_bool_u8(&mut r)?;

                AuraLog::PeriodicHeal {
                    absorb2,
                    critical2,
                    damage2,
                    over_damage,
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
            AuraType::ModInvisibilityDetect => AuraLog::ModInvisibilityDetect,
            AuraType::ObsModHealth => {
                // damage2: u32
                let damage2 = crate::util::read_u32_le(&mut r)?;

                // over_damage: u32
                let over_damage = crate::util::read_u32_le(&mut r)?;

                // absorb2: u32
                let absorb2 = crate::util::read_u32_le(&mut r)?;

                // critical2: Bool
                let critical2 = crate::util::read_bool_u8(&mut r)?;

                AuraLog::ObsModHealth {
                    absorb2,
                    critical2,
                    damage2,
                    over_damage,
                }
            }
            AuraType::ObsModPower => {
                // misc_value1: u32
                let misc_value1 = crate::util::read_u32_le(&mut r)?;

                // damage3: u32
                let damage3 = crate::util::read_u32_le(&mut r)?;

                AuraLog::ObsModPower {
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
            AuraType::PeriodicTriggerSpellFromClient => AuraLog::PeriodicTriggerSpellFromClient,
            AuraType::ModDodgePercent => AuraLog::ModDodgePercent,
            AuraType::ModCriticalHealingAmount => AuraLog::ModCriticalHealingAmount,
            AuraType::ModBlockPercent => AuraLog::ModBlockPercent,
            AuraType::ModWeaponCritPercent => AuraLog::ModWeaponCritPercent,
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
            AuraType::Unknown63 => AuraLog::Unknown63,
            AuraType::PeriodicManaLeech => {
                // misc_value2: u32
                let misc_value2 = crate::util::read_u32_le(&mut r)?;

                // damage4: u32
                let damage4 = crate::util::read_u32_le(&mut r)?;

                // gain_multiplier: f32
                let gain_multiplier = crate::util::read_f32_le(&mut r)?;

                AuraLog::PeriodicManaLeech {
                    damage4,
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

                AuraLog::PeriodicDamagePercent {
                    absorb1,
                    critical1,
                    damage1,
                    overkill_damage,
                    resisted,
                    school,
                }
            }
            AuraType::Unknown90 => AuraLog::Unknown90,
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
            AuraType::Unknown119 => AuraLog::Unknown119,
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
            AuraType::ModPetTalentPoints => AuraLog::ModPetTalentPoints,
            AuraType::AllowTamePetType => AuraLog::AllowTamePetType,
            AuraType::MechanicImmunityMask => AuraLog::MechanicImmunityMask,
            AuraType::RetainComboPoints => AuraLog::RetainComboPoints,
            AuraType::ReducePushback => AuraLog::ReducePushback,
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
            AuraType::PowerBurn => AuraLog::PowerBurn,
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
            AuraType::Unknown173 => AuraLog::Unknown173,
            AuraType::ModSpellDamageOfStatPercent => AuraLog::ModSpellDamageOfStatPercent,
            AuraType::ModSpellHealingOfStatPercent => AuraLog::ModSpellHealingOfStatPercent,
            AuraType::SpiritOfRedemption => AuraLog::SpiritOfRedemption,
            AuraType::AoeCharm => AuraLog::AoeCharm,
            AuraType::ModDebuffResistance => AuraLog::ModDebuffResistance,
            AuraType::ModAttackerSpellCritChance => AuraLog::ModAttackerSpellCritChance,
            AuraType::ModFlatSpellDamageVersus => AuraLog::ModFlatSpellDamageVersus,
            AuraType::Unknown181 => AuraLog::Unknown181,
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
            AuraType::MeleeSlow => AuraLog::MeleeSlow,
            AuraType::ModTargetAbsorbSchool => AuraLog::ModTargetAbsorbSchool,
            AuraType::ModTargetAbilityAbsorbSchool => AuraLog::ModTargetAbilityAbsorbSchool,
            AuraType::ModCooldown => AuraLog::ModCooldown,
            AuraType::ModAttackerSpellAndWeaponCritChance => AuraLog::ModAttackerSpellAndWeaponCritChance,
            AuraType::Unknown198 => AuraLog::Unknown198,
            AuraType::ModIncreasesSpellPctToHit => AuraLog::ModIncreasesSpellPctToHit,
            AuraType::ModXpPct => AuraLog::ModXpPct,
            AuraType::Fly => AuraLog::Fly,
            AuraType::IgnoreCombatResult => AuraLog::IgnoreCombatResult,
            AuraType::ModAttackerMeleeCritDamage => AuraLog::ModAttackerMeleeCritDamage,
            AuraType::ModAttackerRangedCritDamage => AuraLog::ModAttackerRangedCritDamage,
            AuraType::ModSchoolCritDmgTaken => AuraLog::ModSchoolCritDmgTaken,
            AuraType::ModIncreaseVehicleFlightSpeed => AuraLog::ModIncreaseVehicleFlightSpeed,
            AuraType::ModIncreaseMountedFlightSpeed => AuraLog::ModIncreaseMountedFlightSpeed,
            AuraType::ModIncreaseFlightSpeed => AuraLog::ModIncreaseFlightSpeed,
            AuraType::ModMountedFlightSpeedAlways => AuraLog::ModMountedFlightSpeedAlways,
            AuraType::ModVehicleSpeedAlways => AuraLog::ModVehicleSpeedAlways,
            AuraType::ModFlightSpeedNotStack => AuraLog::ModFlightSpeedNotStack,
            AuraType::ModRangedAttackPowerOfStatPercent => AuraLog::ModRangedAttackPowerOfStatPercent,
            AuraType::ModRageFromDamageDealt => AuraLog::ModRageFromDamageDealt,
            AuraType::Unknown214 => AuraLog::Unknown214,
            AuraType::ArenaPreparation => AuraLog::ArenaPreparation,
            AuraType::HasteSpells => AuraLog::HasteSpells,
            AuraType::ModMeleeHaste2 => AuraLog::ModMeleeHaste2,
            AuraType::HasteRanged => AuraLog::HasteRanged,
            AuraType::ModManaRegenFromStat => AuraLog::ModManaRegenFromStat,
            AuraType::ModRatingFromStat => AuraLog::ModRatingFromStat,
            AuraType::ModDetaunt => AuraLog::ModDetaunt,
            AuraType::Unknown222 => AuraLog::Unknown222,
            AuraType::RaidProcFromCharge => AuraLog::RaidProcFromCharge,
            AuraType::Unknown224 => AuraLog::Unknown224,
            AuraType::RaidProcFromChargeWithValue => AuraLog::RaidProcFromChargeWithValue,
            AuraType::PeriodicDummy => AuraLog::PeriodicDummy,
            AuraType::PeriodicTriggerSpellWithValue => AuraLog::PeriodicTriggerSpellWithValue,
            AuraType::DetectStealth => AuraLog::DetectStealth,
            AuraType::ModAoeDamageAvoidance => AuraLog::ModAoeDamageAvoidance,
            AuraType::Unknown230 => AuraLog::Unknown230,
            AuraType::ProcTriggerSpellWithValue => AuraLog::ProcTriggerSpellWithValue,
            AuraType::MechanicDurationMod => AuraLog::MechanicDurationMod,
            AuraType::ChangeModelForAllHumanoids => AuraLog::ChangeModelForAllHumanoids,
            AuraType::MechanicDurationModNotStack => AuraLog::MechanicDurationModNotStack,
            AuraType::ModDispelResist => AuraLog::ModDispelResist,
            AuraType::ControlVehicle => AuraLog::ControlVehicle,
            AuraType::ModSpellDamageOfAttackPower => AuraLog::ModSpellDamageOfAttackPower,
            AuraType::ModSpellHealingOfAttackPower => AuraLog::ModSpellHealingOfAttackPower,
            AuraType::ModScale2 => AuraLog::ModScale2,
            AuraType::ModExpertise => AuraLog::ModExpertise,
            AuraType::ForceMoveForward => AuraLog::ForceMoveForward,
            AuraType::ModSpellDamageFromHealing => AuraLog::ModSpellDamageFromHealing,
            AuraType::ModFaction => AuraLog::ModFaction,
            AuraType::ComprehendLanguage => AuraLog::ComprehendLanguage,
            AuraType::ModAuraDurationByDispel => AuraLog::ModAuraDurationByDispel,
            AuraType::ModAuraDurationByDispelNotStack => AuraLog::ModAuraDurationByDispelNotStack,
            AuraType::CloneCaster => AuraLog::CloneCaster,
            AuraType::ModCombatResultChance => AuraLog::ModCombatResultChance,
            AuraType::ConvertRune => AuraLog::ConvertRune,
            AuraType::ModIncreaseHealth2 => AuraLog::ModIncreaseHealth2,
            AuraType::ModEnemyDodge => AuraLog::ModEnemyDodge,
            AuraType::ModSpeedSlowAll => AuraLog::ModSpeedSlowAll,
            AuraType::ModBlockCritChance => AuraLog::ModBlockCritChance,
            AuraType::ModDisarmOffhand => AuraLog::ModDisarmOffhand,
            AuraType::ModMechanicDamageTakenPercent => AuraLog::ModMechanicDamageTakenPercent,
            AuraType::NoReagentUse => AuraLog::NoReagentUse,
            AuraType::ModTargetResistBySpellClass => AuraLog::ModTargetResistBySpellClass,
            AuraType::Unknown258 => AuraLog::Unknown258,
            AuraType::ModHotPct => AuraLog::ModHotPct,
            AuraType::ScreenEffect => AuraLog::ScreenEffect,
            AuraType::Phase => AuraLog::Phase,
            AuraType::AbilityIgnoreAurastate => AuraLog::AbilityIgnoreAurastate,
            AuraType::AllowOnlyAbility => AuraLog::AllowOnlyAbility,
            AuraType::Unknown264 => AuraLog::Unknown264,
            AuraType::Unknown265 => AuraLog::Unknown265,
            AuraType::Unknown266 => AuraLog::Unknown266,
            AuraType::ModImmuneAuraApplySchool => AuraLog::ModImmuneAuraApplySchool,
            AuraType::ModAttackPowerOfStatPercent => AuraLog::ModAttackPowerOfStatPercent,
            AuraType::ModIgnoreTargetResist => AuraLog::ModIgnoreTargetResist,
            AuraType::ModAbilityIgnoreTargetResist => AuraLog::ModAbilityIgnoreTargetResist,
            AuraType::ModDamageFromCaster => AuraLog::ModDamageFromCaster,
            AuraType::IgnoreMeleeReset => AuraLog::IgnoreMeleeReset,
            AuraType::XRay => AuraLog::XRay,
            AuraType::AbilityConsumeNoAmmo => AuraLog::AbilityConsumeNoAmmo,
            AuraType::ModIgnoreShapeshift => AuraLog::ModIgnoreShapeshift,
            AuraType::ModDamageDoneForMechanic => AuraLog::ModDamageDoneForMechanic,
            AuraType::ModMaxAffectedTargets => AuraLog::ModMaxAffectedTargets,
            AuraType::ModDisarmRanged => AuraLog::ModDisarmRanged,
            AuraType::InitializeImages => AuraLog::InitializeImages,
            AuraType::ModArmorPenetrationPct => AuraLog::ModArmorPenetrationPct,
            AuraType::ModHonorGainPct => AuraLog::ModHonorGainPct,
            AuraType::ModBaseHealthPct => AuraLog::ModBaseHealthPct,
            AuraType::ModHealingReceived => AuraLog::ModHealingReceived,
            AuraType::Linked => AuraLog::Linked,
            AuraType::ModAttackPowerOfArmor => AuraLog::ModAttackPowerOfArmor,
            AuraType::AbilityPeriodicCrit => AuraLog::AbilityPeriodicCrit,
            AuraType::DeflectSpells => AuraLog::DeflectSpells,
            AuraType::IgnoreHitDirection => AuraLog::IgnoreHitDirection,
            AuraType::PreventDurabilityLoss => AuraLog::PreventDurabilityLoss,
            AuraType::ModCritPct => AuraLog::ModCritPct,
            AuraType::ModXpQuestPct => AuraLog::ModXpQuestPct,
            AuraType::OpenStable => AuraLog::OpenStable,
            AuraType::OverrideSpells => AuraLog::OverrideSpells,
            AuraType::PreventRegeneratePower => AuraLog::PreventRegeneratePower,
            AuraType::Unknown295 => AuraLog::Unknown295,
            AuraType::SetVehicleId => AuraLog::SetVehicleId,
            AuraType::BlockSpellFamily => AuraLog::BlockSpellFamily,
            AuraType::Strangulate => AuraLog::Strangulate,
            AuraType::Unknown299 => AuraLog::Unknown299,
            AuraType::ShareDamagePct => AuraLog::ShareDamagePct,
            AuraType::SchoolHealAbsorb => AuraLog::SchoolHealAbsorb,
            AuraType::Unknown302 => AuraLog::Unknown302,
            AuraType::ModDamageDoneVersusAurastate => AuraLog::ModDamageDoneVersusAurastate,
            AuraType::ModFakeInebriate => AuraLog::ModFakeInebriate,
            AuraType::ModMinimumSpeed => AuraLog::ModMinimumSpeed,
            AuraType::Unknown306 => AuraLog::Unknown306,
            AuraType::HealAbsorbTest => AuraLog::HealAbsorbTest,
            AuraType::ModCritChanceForCaster => AuraLog::ModCritChanceForCaster,
            AuraType::Unknown309 => AuraLog::Unknown309,
            AuraType::ModCreatureAoeDamageAvoidance => AuraLog::ModCreatureAoeDamageAvoidance,
            AuraType::Unknown311 => AuraLog::Unknown311,
            AuraType::Unknown312 => AuraLog::Unknown312,
            AuraType::Unknown313 => AuraLog::Unknown313,
            AuraType::PreventResurrection => AuraLog::PreventResurrection,
            AuraType::UnderwaterWalking => AuraLog::UnderwaterWalking,
            AuraType::PeriodicHaste => AuraLog::PeriodicHaste,
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
            Self::ObsModPower {
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
            Self::ModInvisibilityDetect => 19,
            Self::ObsModHealth { .. } => 20,
            Self::ObsModPower { .. } => 21,
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
            Self::ModInvisibilityDetect => f.write_str("ModInvisibilityDetect"),
            Self::ObsModHealth{ .. } => f.write_str("ObsModHealth"),
            Self::ObsModPower{ .. } => f.write_str("ObsModPower"),
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

