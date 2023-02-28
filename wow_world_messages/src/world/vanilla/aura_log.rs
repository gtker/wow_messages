use crate::vanilla::AuraType;
use crate::vanilla::SpellSchool;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm:276`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm#L276):
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
            AuraLog_AuraType::None => {}
            AuraLog_AuraType::BindSight => {}
            AuraLog_AuraType::ModPossess => {}
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
            AuraLog_AuraType::Dummy => {}
            AuraLog_AuraType::ModConfuse => {}
            AuraLog_AuraType::ModCharm => {}
            AuraLog_AuraType::ModFear => {}
            AuraLog_AuraType::PeriodicHeal {
                damage2,
            } => {
                // damage2: u32
                w.write_all(&damage2.to_le_bytes())?;

            }
            AuraLog_AuraType::ModAttackspeed => {}
            AuraLog_AuraType::ModThreat => {}
            AuraLog_AuraType::ModTaunt => {}
            AuraLog_AuraType::ModStun => {}
            AuraLog_AuraType::ModDamageDone => {}
            AuraLog_AuraType::ModDamageTaken => {}
            AuraLog_AuraType::DamageShield => {}
            AuraLog_AuraType::ModStealth => {}
            AuraLog_AuraType::ModStealthDetect => {}
            AuraLog_AuraType::ModInvisibility => {}
            AuraLog_AuraType::ModInvisibilityDetection => {}
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
            AuraLog_AuraType::ModResistance => {}
            AuraLog_AuraType::PeriodicTriggerSpell => {}
            AuraLog_AuraType::PeriodicEnergize {
                damage3,
                misc_value1,
            } => {
                // misc_value1: u32
                w.write_all(&misc_value1.to_le_bytes())?;

                // damage3: u32
                w.write_all(&damage3.to_le_bytes())?;

            }
            AuraLog_AuraType::ModPacify => {}
            AuraLog_AuraType::ModRoot => {}
            AuraLog_AuraType::ModSilence => {}
            AuraLog_AuraType::ReflectSpells => {}
            AuraLog_AuraType::ModStat => {}
            AuraLog_AuraType::ModSkill => {}
            AuraLog_AuraType::ModIncreaseSpeed => {}
            AuraLog_AuraType::ModIncreaseMountedSpeed => {}
            AuraLog_AuraType::ModDecreaseSpeed => {}
            AuraLog_AuraType::ModIncreaseHealth => {}
            AuraLog_AuraType::ModIncreaseEnergy => {}
            AuraLog_AuraType::ModShapeshift => {}
            AuraLog_AuraType::EffectImmunity => {}
            AuraLog_AuraType::StateImmunity => {}
            AuraLog_AuraType::SchoolImmunity => {}
            AuraLog_AuraType::DamageImmunity => {}
            AuraLog_AuraType::DispelImmunity => {}
            AuraLog_AuraType::ProcTriggerSpell => {}
            AuraLog_AuraType::ProcTriggerDamage => {}
            AuraLog_AuraType::TrackCreatures => {}
            AuraLog_AuraType::TrackResources => {}
            AuraLog_AuraType::Unknown46 => {}
            AuraLog_AuraType::ModParryPercent => {}
            AuraLog_AuraType::Unknown48 => {}
            AuraLog_AuraType::ModDodgePercent => {}
            AuraLog_AuraType::ModBlockSkill => {}
            AuraLog_AuraType::ModBlockPercent => {}
            AuraLog_AuraType::ModCritPercent => {}
            AuraLog_AuraType::PeriodicLeech => {}
            AuraLog_AuraType::ModHitChance => {}
            AuraLog_AuraType::ModSpellHitChance => {}
            AuraLog_AuraType::Transform => {}
            AuraLog_AuraType::ModSpellCritChance => {}
            AuraLog_AuraType::ModIncreaseSwimSpeed => {}
            AuraLog_AuraType::ModDamageDoneCreature => {}
            AuraLog_AuraType::ModPacifySilence => {}
            AuraLog_AuraType::ModScale => {}
            AuraLog_AuraType::PeriodicHealthFunnel => {}
            AuraLog_AuraType::PeriodicManaFunnel => {}
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
            AuraLog_AuraType::ModCastingSpeedNotStack => {}
            AuraLog_AuraType::FeignDeath => {}
            AuraLog_AuraType::ModDisarm => {}
            AuraLog_AuraType::ModStalked => {}
            AuraLog_AuraType::SchoolAbsorb => {}
            AuraLog_AuraType::ExtraAttacks => {}
            AuraLog_AuraType::ModSpellCritChanceSchool => {}
            AuraLog_AuraType::ModPowerCostSchoolPct => {}
            AuraLog_AuraType::ModPowerCostSchool => {}
            AuraLog_AuraType::ReflectSpellsSchool => {}
            AuraLog_AuraType::ModLanguage => {}
            AuraLog_AuraType::FarSight => {}
            AuraLog_AuraType::MechanicImmunity => {}
            AuraLog_AuraType::Mounted => {}
            AuraLog_AuraType::ModDamagePercentDone => {}
            AuraLog_AuraType::ModPercentStat => {}
            AuraLog_AuraType::SplitDamagePct => {}
            AuraLog_AuraType::WaterBreathing => {}
            AuraLog_AuraType::ModBaseResistance => {}
            AuraLog_AuraType::ModRegen => {}
            AuraLog_AuraType::ModPowerRegen => {}
            AuraLog_AuraType::ChannelDeathItem => {}
            AuraLog_AuraType::ModDamagePercentTaken => {}
            AuraLog_AuraType::ModHealthRegenPercent => {}
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
            AuraLog_AuraType::ModResistChance => {}
            AuraLog_AuraType::ModDetectRange => {}
            AuraLog_AuraType::PreventsFleeing => {}
            AuraLog_AuraType::ModUnattackable => {}
            AuraLog_AuraType::InterruptRegen => {}
            AuraLog_AuraType::Ghost => {}
            AuraLog_AuraType::SpellMagnet => {}
            AuraLog_AuraType::ManaShield => {}
            AuraLog_AuraType::ModSkillTalent => {}
            AuraLog_AuraType::ModAttackPower => {}
            AuraLog_AuraType::AurasVisible => {}
            AuraLog_AuraType::ModResistancePct => {}
            AuraLog_AuraType::ModMeleeAttackPowerVersus => {}
            AuraLog_AuraType::ModTotalThreat => {}
            AuraLog_AuraType::WaterWalk => {}
            AuraLog_AuraType::FeatherFall => {}
            AuraLog_AuraType::Hover => {}
            AuraLog_AuraType::AddFlatModifier => {}
            AuraLog_AuraType::AddPctModifier => {}
            AuraLog_AuraType::AddTargetTrigger => {}
            AuraLog_AuraType::ModPowerRegenPercent => {}
            AuraLog_AuraType::AddCasterHitTrigger => {}
            AuraLog_AuraType::OverrideClassScripts => {}
            AuraLog_AuraType::ModRangedDamageTaken => {}
            AuraLog_AuraType::ModRangedDamageTakenPct => {}
            AuraLog_AuraType::ModHealing => {}
            AuraLog_AuraType::ModRegenDuringCombat => {}
            AuraLog_AuraType::ModMechanicResistance => {}
            AuraLog_AuraType::ModHealingPct => {}
            AuraLog_AuraType::SharePetTracking => {}
            AuraLog_AuraType::Untrackable => {}
            AuraLog_AuraType::Empathy => {}
            AuraLog_AuraType::ModOffhandDamagePct => {}
            AuraLog_AuraType::ModTargetResistance => {}
            AuraLog_AuraType::ModRangedAttackPower => {}
            AuraLog_AuraType::ModMeleeDamageTaken => {}
            AuraLog_AuraType::ModMeleeDamageTakenPct => {}
            AuraLog_AuraType::RangedAttackPowerAttackerBonus => {}
            AuraLog_AuraType::ModPossessPet => {}
            AuraLog_AuraType::ModSpeedAlways => {}
            AuraLog_AuraType::ModMountedSpeedAlways => {}
            AuraLog_AuraType::ModRangedAttackPowerVersus => {}
            AuraLog_AuraType::ModIncreaseEnergyPercent => {}
            AuraLog_AuraType::ModIncreaseHealthPercent => {}
            AuraLog_AuraType::ModManaRegenInterrupt => {}
            AuraLog_AuraType::ModHealingDone => {}
            AuraLog_AuraType::ModHealingDonePercent => {}
            AuraLog_AuraType::ModTotalStatPercentage => {}
            AuraLog_AuraType::ModMeleeHaste => {}
            AuraLog_AuraType::ForceReaction => {}
            AuraLog_AuraType::ModRangedHaste => {}
            AuraLog_AuraType::ModRangedAmmoHaste => {}
            AuraLog_AuraType::ModBaseResistancePct => {}
            AuraLog_AuraType::ModResistanceExclusive => {}
            AuraLog_AuraType::SafeFall => {}
            AuraLog_AuraType::Charisma => {}
            AuraLog_AuraType::Persuaded => {}
            AuraLog_AuraType::MechanicImmunityMask => {}
            AuraLog_AuraType::RetainComboPoints => {}
            AuraLog_AuraType::ResistPushback => {}
            AuraLog_AuraType::ModShieldBlockvaluePct => {}
            AuraLog_AuraType::TrackStealthed => {}
            AuraLog_AuraType::ModDetectedRange => {}
            AuraLog_AuraType::SplitDamageFlat => {}
            AuraLog_AuraType::ModStealthLevel => {}
            AuraLog_AuraType::ModWaterBreathing => {}
            AuraLog_AuraType::ModReputationGain => {}
            AuraLog_AuraType::PetDamageMulti => {}
            AuraLog_AuraType::ModShieldBlockvalue => {}
            AuraLog_AuraType::NoPvpCredit => {}
            AuraLog_AuraType::ModAoeAvoidance => {}
            AuraLog_AuraType::ModHealthRegenInCombat => {}
            AuraLog_AuraType::PowerBurnMana => {}
            AuraLog_AuraType::ModCritDamageBonus => {}
            AuraLog_AuraType::Unknown164 => {}
            AuraLog_AuraType::MeleeAttackPowerAttackerBonus => {}
            AuraLog_AuraType::ModAttackPowerPct => {}
            AuraLog_AuraType::ModRangedAttackPowerPct => {}
            AuraLog_AuraType::ModDamageDoneVersus => {}
            AuraLog_AuraType::ModCritPercentVersus => {}
            AuraLog_AuraType::DetectAmore => {}
            AuraLog_AuraType::ModSpeedNotStack => {}
            AuraLog_AuraType::ModMountedSpeedNotStack => {}
            AuraLog_AuraType::AllowChampionSpells => {}
            AuraLog_AuraType::ModSpellDamageOfStatPercent => {}
            AuraLog_AuraType::ModSpellHealingOfStatPercent => {}
            AuraLog_AuraType::SpiritOfRedemption => {}
            AuraLog_AuraType::AoeCharm => {}
            AuraLog_AuraType::ModDebuffResistance => {}
            AuraLog_AuraType::ModAttackerSpellCritChance => {}
            AuraLog_AuraType::ModFlatSpellDamageVersus => {}
            AuraLog_AuraType::ModFlatSpellCritDamageVersus => {}
            AuraLog_AuraType::ModResistanceOfStatPercent => {}
            AuraLog_AuraType::ModCriticalThreat => {}
            AuraLog_AuraType::ModAttackerMeleeHitChance => {}
            AuraLog_AuraType::ModAttackerRangedHitChance => {}
            AuraLog_AuraType::ModAttackerSpellHitChance => {}
            AuraLog_AuraType::ModAttackerMeleeCritChance => {}
            AuraLog_AuraType::ModAttackerRangedCritChance => {}
            AuraLog_AuraType::ModRating => {}
            AuraLog_AuraType::ModFactionReputationGain => {}
            AuraLog_AuraType::UseNormalMovementSpeed => {}
        }

        Ok(())
    }
}

impl AuraLog {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // aura_type: AuraType
        let aura_type: AuraType = crate::util::read_u32_le(r)?.try_into()?;

        let aura_type_if = match aura_type {
            AuraType::None => AuraLog_AuraType::None,
            AuraType::BindSight => AuraLog_AuraType::BindSight,
            AuraType::ModPossess => AuraLog_AuraType::ModPossess,
            AuraType::PeriodicDamage => {
                // damage1: u32
                let damage1 = crate::util::read_u32_le(r)?;

                // school: SpellSchool
                let school: SpellSchool = crate::util::read_u8_le(r)?.try_into()?;

                // absorbed: u32
                let absorbed = crate::util::read_u32_le(r)?;

                // resisted: u32
                let resisted = crate::util::read_u32_le(r)?;

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
                let damage2 = crate::util::read_u32_le(r)?;

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
                let damage2 = crate::util::read_u32_le(r)?;

                AuraLog_AuraType::ObsModHealth {
                    damage2,
                }
            }
            AuraType::ObsModMana => {
                // misc_value1: u32
                let misc_value1 = crate::util::read_u32_le(r)?;

                // damage3: u32
                let damage3 = crate::util::read_u32_le(r)?;

                AuraLog_AuraType::ObsModMana {
                    damage3,
                    misc_value1,
                }
            }
            AuraType::ModResistance => AuraLog_AuraType::ModResistance,
            AuraType::PeriodicTriggerSpell => AuraLog_AuraType::PeriodicTriggerSpell,
            AuraType::PeriodicEnergize => {
                // misc_value1: u32
                let misc_value1 = crate::util::read_u32_le(r)?;

                // damage3: u32
                let damage3 = crate::util::read_u32_le(r)?;

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
                let misc_value2 = crate::util::read_u32_le(r)?;

                // damage: u32
                let damage = crate::util::read_u32_le(r)?;

                // gain_multiplier: f32
                let gain_multiplier = crate::util::read_f32_le(r)?;

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
                let damage1 = crate::util::read_u32_le(r)?;

                // school: SpellSchool
                let school: SpellSchool = crate::util::read_u8_le(r)?.try_into()?;

                // absorbed: u32
                let absorbed = crate::util::read_u32_le(r)?;

                // resisted: u32
                let resisted = crate::util::read_u32_le(r)?;

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
        };

        Ok(Self {
            aura_type: aura_type_if,
        })
    }

}

impl AuraLog {
    pub(crate) fn size(&self) -> usize {
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
        }
    }

}

impl AuraLog_AuraType {
    pub(crate) fn size(&self) -> usize {
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
        }
    }
}

