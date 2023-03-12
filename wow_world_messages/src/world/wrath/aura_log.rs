use std::io::{Read, Write};

use crate::wrath::{
    AuraType, SpellSchool,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm:1059`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm#L1059):
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
                w.write_all(&u8::from(school.as_int()).to_le_bytes())?;

                // absorb1: u32
                w.write_all(&absorb1.to_le_bytes())?;

                // resisted: u32
                w.write_all(&resisted.to_le_bytes())?;

                // critical1: Bool
                w.write_all(u8::from(*critical1).to_le_bytes().as_slice())?;

            }
            AuraLog_AuraType::Dummy => {}
            AuraLog_AuraType::ModConfuse => {}
            AuraLog_AuraType::ModCharm => {}
            AuraLog_AuraType::ModFear => {}
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
            AuraLog_AuraType::ModInvisibilityDetect => {}
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
            AuraLog_AuraType::ObsModPower => {}
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
            AuraLog_AuraType::PeriodicTriggerSpellFromClient => {}
            AuraLog_AuraType::ModDodgePercent => {}
            AuraLog_AuraType::ModCriticalHealingAmount => {}
            AuraLog_AuraType::ModBlockPercent => {}
            AuraLog_AuraType::ModWeaponCritPercent => {}
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
            AuraLog_AuraType::Unknown63 => {}
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
                w.write_all(&u8::from(school.as_int()).to_le_bytes())?;

                // absorb1: u32
                w.write_all(&absorb1.to_le_bytes())?;

                // resisted: u32
                w.write_all(&resisted.to_le_bytes())?;

                // critical1: Bool
                w.write_all(u8::from(*critical1).to_le_bytes().as_slice())?;

            }
            AuraLog_AuraType::Unknown90 => {}
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
            AuraLog_AuraType::Unknown119 => {}
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
            AuraLog_AuraType::ModPetTalentPoints => {}
            AuraLog_AuraType::AllowTamePetType => {}
            AuraLog_AuraType::MechanicImmunityMask => {}
            AuraLog_AuraType::RetainComboPoints => {}
            AuraLog_AuraType::ReducePushback => {}
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
            AuraLog_AuraType::PowerBurn => {}
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
            AuraLog_AuraType::Unknown173 => {}
            AuraLog_AuraType::ModSpellDamageOfStatPercent => {}
            AuraLog_AuraType::ModSpellHealingOfStatPercent => {}
            AuraLog_AuraType::SpiritOfRedemption => {}
            AuraLog_AuraType::AoeCharm => {}
            AuraLog_AuraType::ModDebuffResistance => {}
            AuraLog_AuraType::ModAttackerSpellCritChance => {}
            AuraLog_AuraType::ModFlatSpellDamageVersus => {}
            AuraLog_AuraType::Unknown181 => {}
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
            AuraLog_AuraType::ModMeleeRangedHaste => {}
            AuraLog_AuraType::MeleeSlow => {}
            AuraLog_AuraType::ModTargetAbsorbSchool => {}
            AuraLog_AuraType::ModTargetAbilityAbsorbSchool => {}
            AuraLog_AuraType::ModCooldown => {}
            AuraLog_AuraType::ModAttackerSpellAndWeaponCritChance => {}
            AuraLog_AuraType::Unknown198 => {}
            AuraLog_AuraType::ModIncreasesSpellPctToHit => {}
            AuraLog_AuraType::ModXpPct => {}
            AuraLog_AuraType::Fly => {}
            AuraLog_AuraType::IgnoreCombatResult => {}
            AuraLog_AuraType::ModAttackerMeleeCritDamage => {}
            AuraLog_AuraType::ModAttackerRangedCritDamage => {}
            AuraLog_AuraType::ModSchoolCritDmgTaken => {}
            AuraLog_AuraType::ModIncreaseVehicleFlightSpeed => {}
            AuraLog_AuraType::ModIncreaseMountedFlightSpeed => {}
            AuraLog_AuraType::ModIncreaseFlightSpeed => {}
            AuraLog_AuraType::ModMountedFlightSpeedAlways => {}
            AuraLog_AuraType::ModVehicleSpeedAlways => {}
            AuraLog_AuraType::ModFlightSpeedNotStack => {}
            AuraLog_AuraType::ModRangedAttackPowerOfStatPercent => {}
            AuraLog_AuraType::ModRageFromDamageDealt => {}
            AuraLog_AuraType::Unknown214 => {}
            AuraLog_AuraType::ArenaPreparation => {}
            AuraLog_AuraType::HasteSpells => {}
            AuraLog_AuraType::ModMeleeHaste2 => {}
            AuraLog_AuraType::HasteRanged => {}
            AuraLog_AuraType::ModManaRegenFromStat => {}
            AuraLog_AuraType::ModRatingFromStat => {}
            AuraLog_AuraType::ModDetaunt => {}
            AuraLog_AuraType::Unknown222 => {}
            AuraLog_AuraType::RaidProcFromCharge => {}
            AuraLog_AuraType::Unknown224 => {}
            AuraLog_AuraType::RaidProcFromChargeWithValue => {}
            AuraLog_AuraType::PeriodicDummy => {}
            AuraLog_AuraType::PeriodicTriggerSpellWithValue => {}
            AuraLog_AuraType::DetectStealth => {}
            AuraLog_AuraType::ModAoeDamageAvoidance => {}
            AuraLog_AuraType::Unknown230 => {}
            AuraLog_AuraType::ProcTriggerSpellWithValue => {}
            AuraLog_AuraType::MechanicDurationMod => {}
            AuraLog_AuraType::ChangeModelForAllHumanoids => {}
            AuraLog_AuraType::MechanicDurationModNotStack => {}
            AuraLog_AuraType::ModDispelResist => {}
            AuraLog_AuraType::ControlVehicle => {}
            AuraLog_AuraType::ModSpellDamageOfAttackPower => {}
            AuraLog_AuraType::ModSpellHealingOfAttackPower => {}
            AuraLog_AuraType::ModScale2 => {}
            AuraLog_AuraType::ModExpertise => {}
            AuraLog_AuraType::ForceMoveForward => {}
            AuraLog_AuraType::ModSpellDamageFromHealing => {}
            AuraLog_AuraType::ModFaction => {}
            AuraLog_AuraType::ComprehendLanguage => {}
            AuraLog_AuraType::ModAuraDurationByDispel => {}
            AuraLog_AuraType::ModAuraDurationByDispelNotStack => {}
            AuraLog_AuraType::CloneCaster => {}
            AuraLog_AuraType::ModCombatResultChance => {}
            AuraLog_AuraType::ConvertRune => {}
            AuraLog_AuraType::ModIncreaseHealth2 => {}
            AuraLog_AuraType::ModEnemyDodge => {}
            AuraLog_AuraType::ModSpeedSlowAll => {}
            AuraLog_AuraType::ModBlockCritChance => {}
            AuraLog_AuraType::ModDisarmOffhand => {}
            AuraLog_AuraType::ModMechanicDamageTakenPercent => {}
            AuraLog_AuraType::NoReagentUse => {}
            AuraLog_AuraType::ModTargetResistBySpellClass => {}
            AuraLog_AuraType::Unknown258 => {}
            AuraLog_AuraType::ModHotPct => {}
            AuraLog_AuraType::ScreenEffect => {}
            AuraLog_AuraType::Phase => {}
            AuraLog_AuraType::AbilityIgnoreAurastate => {}
            AuraLog_AuraType::AllowOnlyAbility => {}
            AuraLog_AuraType::Unknown264 => {}
            AuraLog_AuraType::Unknown265 => {}
            AuraLog_AuraType::Unknown266 => {}
            AuraLog_AuraType::ModImmuneAuraApplySchool => {}
            AuraLog_AuraType::ModAttackPowerOfStatPercent => {}
            AuraLog_AuraType::ModIgnoreTargetResist => {}
            AuraLog_AuraType::ModAbilityIgnoreTargetResist => {}
            AuraLog_AuraType::ModDamageFromCaster => {}
            AuraLog_AuraType::IgnoreMeleeReset => {}
            AuraLog_AuraType::XRay => {}
            AuraLog_AuraType::AbilityConsumeNoAmmo => {}
            AuraLog_AuraType::ModIgnoreShapeshift => {}
            AuraLog_AuraType::ModDamageDoneForMechanic => {}
            AuraLog_AuraType::ModMaxAffectedTargets => {}
            AuraLog_AuraType::ModDisarmRanged => {}
            AuraLog_AuraType::InitializeImages => {}
            AuraLog_AuraType::ModArmorPenetrationPct => {}
            AuraLog_AuraType::ModHonorGainPct => {}
            AuraLog_AuraType::ModBaseHealthPct => {}
            AuraLog_AuraType::ModHealingReceived => {}
            AuraLog_AuraType::Linked => {}
            AuraLog_AuraType::ModAttackPowerOfArmor => {}
            AuraLog_AuraType::AbilityPeriodicCrit => {}
            AuraLog_AuraType::DeflectSpells => {}
            AuraLog_AuraType::IgnoreHitDirection => {}
            AuraLog_AuraType::PreventDurabilityLoss => {}
            AuraLog_AuraType::ModCritPct => {}
            AuraLog_AuraType::ModXpQuestPct => {}
            AuraLog_AuraType::OpenStable => {}
            AuraLog_AuraType::OverrideSpells => {}
            AuraLog_AuraType::PreventRegeneratePower => {}
            AuraLog_AuraType::Unknown295 => {}
            AuraLog_AuraType::SetVehicleId => {}
            AuraLog_AuraType::BlockSpellFamily => {}
            AuraLog_AuraType::Strangulate => {}
            AuraLog_AuraType::Unknown299 => {}
            AuraLog_AuraType::ShareDamagePct => {}
            AuraLog_AuraType::SchoolHealAbsorb => {}
            AuraLog_AuraType::Unknown302 => {}
            AuraLog_AuraType::ModDamageDoneVersusAurastate => {}
            AuraLog_AuraType::ModFakeInebriate => {}
            AuraLog_AuraType::ModMinimumSpeed => {}
            AuraLog_AuraType::Unknown306 => {}
            AuraLog_AuraType::HealAbsorbTest => {}
            AuraLog_AuraType::ModCritChanceForCaster => {}
            AuraLog_AuraType::Unknown309 => {}
            AuraLog_AuraType::ModCreatureAoeDamageAvoidance => {}
            AuraLog_AuraType::Unknown311 => {}
            AuraLog_AuraType::Unknown312 => {}
            AuraLog_AuraType::Unknown313 => {}
            AuraLog_AuraType::PreventResurrection => {}
            AuraLog_AuraType::UnderwaterWalking => {}
            AuraLog_AuraType::PeriodicHaste => {}
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

                // overkill_damage: u32
                let overkill_damage = crate::util::read_u32_le(&mut r)?;

                // school: SpellSchool
                let school: SpellSchool = crate::util::read_u8_le(&mut r)?.try_into()?;

                // absorb1: u32
                let absorb1 = crate::util::read_u32_le(&mut r)?;

                // resisted: u32
                let resisted = crate::util::read_u32_le(&mut r)?;

                // critical1: Bool
                let critical1 = crate::util::read_u8_le(&mut r)? != 0;

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
                let critical2 = crate::util::read_u8_le(&mut r)? != 0;

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
                let critical2 = crate::util::read_u8_le(&mut r)? != 0;

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
                let school: SpellSchool = crate::util::read_u8_le(&mut r)?.try_into()?;

                // absorb1: u32
                let absorb1 = crate::util::read_u32_le(&mut r)?;

                // resisted: u32
                let resisted = crate::util::read_u32_le(&mut r)?;

                // critical1: Bool
                let critical1 = crate::util::read_u8_le(&mut r)? != 0;

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
                absorb1,
                critical1,
                damage1,
                overkill_damage,
                resisted,
                school,
            } => {
                4
                + 4 // absorb1: u32
                + 1 // critical1: Bool
                + 4 // damage1: u32
                + 4 // overkill_damage: u32
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
                absorb2,
                critical2,
                damage2,
                over_damage,
            } => {
                4
                + 4 // absorb2: u32
                + 1 // critical2: Bool
                + 4 // damage2: u32
                + 4 // over_damage: u32
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
            Self::ModInvisibilityDetect => {
                4
            }
            Self::ObsModHealth {
                absorb2,
                critical2,
                damage2,
                over_damage,
            } => {
                4
                + 4 // absorb2: u32
                + 1 // critical2: Bool
                + 4 // damage2: u32
                + 4 // over_damage: u32
            }
            Self::ObsModPower => {
                4
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
            Self::PeriodicTriggerSpellFromClient => {
                4
            }
            Self::ModDodgePercent => {
                4
            }
            Self::ModCriticalHealingAmount => {
                4
            }
            Self::ModBlockPercent => {
                4
            }
            Self::ModWeaponCritPercent => {
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
            Self::Unknown63 => {
                4
            }
            Self::PeriodicManaLeech {
                damage4,
                gain_multiplier,
                misc_value2,
            } => {
                4
                + 4 // damage4: u32
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
                absorb1,
                critical1,
                damage1,
                overkill_damage,
                resisted,
                school,
            } => {
                4
                + 4 // absorb1: u32
                + 1 // critical1: Bool
                + 4 // damage1: u32
                + 4 // overkill_damage: u32
                + 4 // resisted: u32
                + 1 // school: SpellSchool
            }
            Self::Unknown90 => {
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
            Self::Unknown119 => {
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
            Self::ModPetTalentPoints => {
                4
            }
            Self::AllowTamePetType => {
                4
            }
            Self::MechanicImmunityMask => {
                4
            }
            Self::RetainComboPoints => {
                4
            }
            Self::ReducePushback => {
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
            Self::PowerBurn => {
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
            Self::Unknown173 => {
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
            Self::Unknown181 => {
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
            Self::MeleeSlow => {
                4
            }
            Self::ModTargetAbsorbSchool => {
                4
            }
            Self::ModTargetAbilityAbsorbSchool => {
                4
            }
            Self::ModCooldown => {
                4
            }
            Self::ModAttackerSpellAndWeaponCritChance => {
                4
            }
            Self::Unknown198 => {
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
            Self::ModSchoolCritDmgTaken => {
                4
            }
            Self::ModIncreaseVehicleFlightSpeed => {
                4
            }
            Self::ModIncreaseMountedFlightSpeed => {
                4
            }
            Self::ModIncreaseFlightSpeed => {
                4
            }
            Self::ModMountedFlightSpeedAlways => {
                4
            }
            Self::ModVehicleSpeedAlways => {
                4
            }
            Self::ModFlightSpeedNotStack => {
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
            Self::ModMeleeHaste2 => {
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
            Self::ModDetaunt => {
                4
            }
            Self::Unknown222 => {
                4
            }
            Self::RaidProcFromCharge => {
                4
            }
            Self::Unknown224 => {
                4
            }
            Self::RaidProcFromChargeWithValue => {
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
            Self::ChangeModelForAllHumanoids => {
                4
            }
            Self::MechanicDurationModNotStack => {
                4
            }
            Self::ModDispelResist => {
                4
            }
            Self::ControlVehicle => {
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
            Self::ModSpellDamageFromHealing => {
                4
            }
            Self::ModFaction => {
                4
            }
            Self::ComprehendLanguage => {
                4
            }
            Self::ModAuraDurationByDispel => {
                4
            }
            Self::ModAuraDurationByDispelNotStack => {
                4
            }
            Self::CloneCaster => {
                4
            }
            Self::ModCombatResultChance => {
                4
            }
            Self::ConvertRune => {
                4
            }
            Self::ModIncreaseHealth2 => {
                4
            }
            Self::ModEnemyDodge => {
                4
            }
            Self::ModSpeedSlowAll => {
                4
            }
            Self::ModBlockCritChance => {
                4
            }
            Self::ModDisarmOffhand => {
                4
            }
            Self::ModMechanicDamageTakenPercent => {
                4
            }
            Self::NoReagentUse => {
                4
            }
            Self::ModTargetResistBySpellClass => {
                4
            }
            Self::Unknown258 => {
                4
            }
            Self::ModHotPct => {
                4
            }
            Self::ScreenEffect => {
                4
            }
            Self::Phase => {
                4
            }
            Self::AbilityIgnoreAurastate => {
                4
            }
            Self::AllowOnlyAbility => {
                4
            }
            Self::Unknown264 => {
                4
            }
            Self::Unknown265 => {
                4
            }
            Self::Unknown266 => {
                4
            }
            Self::ModImmuneAuraApplySchool => {
                4
            }
            Self::ModAttackPowerOfStatPercent => {
                4
            }
            Self::ModIgnoreTargetResist => {
                4
            }
            Self::ModAbilityIgnoreTargetResist => {
                4
            }
            Self::ModDamageFromCaster => {
                4
            }
            Self::IgnoreMeleeReset => {
                4
            }
            Self::XRay => {
                4
            }
            Self::AbilityConsumeNoAmmo => {
                4
            }
            Self::ModIgnoreShapeshift => {
                4
            }
            Self::ModDamageDoneForMechanic => {
                4
            }
            Self::ModMaxAffectedTargets => {
                4
            }
            Self::ModDisarmRanged => {
                4
            }
            Self::InitializeImages => {
                4
            }
            Self::ModArmorPenetrationPct => {
                4
            }
            Self::ModHonorGainPct => {
                4
            }
            Self::ModBaseHealthPct => {
                4
            }
            Self::ModHealingReceived => {
                4
            }
            Self::Linked => {
                4
            }
            Self::ModAttackPowerOfArmor => {
                4
            }
            Self::AbilityPeriodicCrit => {
                4
            }
            Self::DeflectSpells => {
                4
            }
            Self::IgnoreHitDirection => {
                4
            }
            Self::PreventDurabilityLoss => {
                4
            }
            Self::ModCritPct => {
                4
            }
            Self::ModXpQuestPct => {
                4
            }
            Self::OpenStable => {
                4
            }
            Self::OverrideSpells => {
                4
            }
            Self::PreventRegeneratePower => {
                4
            }
            Self::Unknown295 => {
                4
            }
            Self::SetVehicleId => {
                4
            }
            Self::BlockSpellFamily => {
                4
            }
            Self::Strangulate => {
                4
            }
            Self::Unknown299 => {
                4
            }
            Self::ShareDamagePct => {
                4
            }
            Self::SchoolHealAbsorb => {
                4
            }
            Self::Unknown302 => {
                4
            }
            Self::ModDamageDoneVersusAurastate => {
                4
            }
            Self::ModFakeInebriate => {
                4
            }
            Self::ModMinimumSpeed => {
                4
            }
            Self::Unknown306 => {
                4
            }
            Self::HealAbsorbTest => {
                4
            }
            Self::ModCritChanceForCaster => {
                4
            }
            Self::Unknown309 => {
                4
            }
            Self::ModCreatureAoeDamageAvoidance => {
                4
            }
            Self::Unknown311 => {
                4
            }
            Self::Unknown312 => {
                4
            }
            Self::Unknown313 => {
                4
            }
            Self::PreventResurrection => {
                4
            }
            Self::UnderwaterWalking => {
                4
            }
            Self::PeriodicHaste => {
                4
            }
        }
    }
}

