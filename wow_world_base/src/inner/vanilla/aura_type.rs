/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm#L11):
/// ```text
/// enum AuraType : u32 {
///     NONE = 0;
///     BIND_SIGHT = 1;
///     MOD_POSSESS = 2;
///     PERIODIC_DAMAGE = 3;
///     DUMMY = 4;
///     MOD_CONFUSE = 5;
///     MOD_CHARM = 6;
///     MOD_FEAR = 7;
///     PERIODIC_HEAL = 8;
///     MOD_ATTACKSPEED = 9;
///     MOD_THREAT = 10;
///     MOD_TAUNT = 11;
///     MOD_STUN = 12;
///     MOD_DAMAGE_DONE = 13;
///     MOD_DAMAGE_TAKEN = 14;
///     DAMAGE_SHIELD = 15;
///     MOD_STEALTH = 16;
///     MOD_STEALTH_DETECT = 17;
///     MOD_INVISIBILITY = 18;
///     MOD_INVISIBILITY_DETECTION = 19;
///     OBS_MOD_HEALTH = 20;
///     OBS_MOD_MANA = 21;
///     MOD_RESISTANCE = 22;
///     PERIODIC_TRIGGER_SPELL = 23;
///     PERIODIC_ENERGIZE = 24;
///     MOD_PACIFY = 25;
///     MOD_ROOT = 26;
///     MOD_SILENCE = 27;
///     REFLECT_SPELLS = 28;
///     MOD_STAT = 29;
///     MOD_SKILL = 30;
///     MOD_INCREASE_SPEED = 31;
///     MOD_INCREASE_MOUNTED_SPEED = 32;
///     MOD_DECREASE_SPEED = 33;
///     MOD_INCREASE_HEALTH = 34;
///     MOD_INCREASE_ENERGY = 35;
///     MOD_SHAPESHIFT = 36;
///     EFFECT_IMMUNITY = 37;
///     STATE_IMMUNITY = 38;
///     SCHOOL_IMMUNITY = 39;
///     DAMAGE_IMMUNITY = 40;
///     DISPEL_IMMUNITY = 41;
///     PROC_TRIGGER_SPELL = 42;
///     PROC_TRIGGER_DAMAGE = 43;
///     TRACK_CREATURES = 44;
///     TRACK_RESOURCES = 45;
///     UNKNOWN46 = 46;
///     MOD_PARRY_PERCENT = 47;
///     UNKNOWN48 = 48;
///     MOD_DODGE_PERCENT = 49;
///     MOD_BLOCK_SKILL = 50;
///     MOD_BLOCK_PERCENT = 51;
///     MOD_CRIT_PERCENT = 52;
///     PERIODIC_LEECH = 53;
///     MOD_HIT_CHANCE = 54;
///     MOD_SPELL_HIT_CHANCE = 55;
///     TRANSFORM = 56;
///     MOD_SPELL_CRIT_CHANCE = 57;
///     MOD_INCREASE_SWIM_SPEED = 58;
///     MOD_DAMAGE_DONE_CREATURE = 59;
///     MOD_PACIFY_SILENCE = 60;
///     MOD_SCALE = 61;
///     PERIODIC_HEALTH_FUNNEL = 62;
///     PERIODIC_MANA_FUNNEL = 63;
///     PERIODIC_MANA_LEECH = 64;
///     MOD_CASTING_SPEED_NOT_STACK = 65;
///     FEIGN_DEATH = 66;
///     MOD_DISARM = 67;
///     MOD_STALKED = 68;
///     SCHOOL_ABSORB = 69;
///     EXTRA_ATTACKS = 70;
///     MOD_SPELL_CRIT_CHANCE_SCHOOL = 71;
///     MOD_POWER_COST_SCHOOL_PCT = 72;
///     MOD_POWER_COST_SCHOOL = 73;
///     REFLECT_SPELLS_SCHOOL = 74;
///     MOD_LANGUAGE = 75;
///     FAR_SIGHT = 76;
///     MECHANIC_IMMUNITY = 77;
///     MOUNTED = 78;
///     MOD_DAMAGE_PERCENT_DONE = 79;
///     MOD_PERCENT_STAT = 80;
///     SPLIT_DAMAGE_PCT = 81;
///     WATER_BREATHING = 82;
///     MOD_BASE_RESISTANCE = 83;
///     MOD_REGEN = 84;
///     MOD_POWER_REGEN = 85;
///     CHANNEL_DEATH_ITEM = 86;
///     MOD_DAMAGE_PERCENT_TAKEN = 87;
///     MOD_HEALTH_REGEN_PERCENT = 88;
///     PERIODIC_DAMAGE_PERCENT = 89;
///     MOD_RESIST_CHANCE = 90;
///     MOD_DETECT_RANGE = 91;
///     PREVENTS_FLEEING = 92;
///     MOD_UNATTACKABLE = 93;
///     INTERRUPT_REGEN = 94;
///     GHOST = 95;
///     SPELL_MAGNET = 96;
///     MANA_SHIELD = 97;
///     MOD_SKILL_TALENT = 98;
///     MOD_ATTACK_POWER = 99;
///     AURAS_VISIBLE = 100;
///     MOD_RESISTANCE_PCT = 101;
///     MOD_MELEE_ATTACK_POWER_VERSUS = 102;
///     MOD_TOTAL_THREAT = 103;
///     WATER_WALK = 104;
///     FEATHER_FALL = 105;
///     HOVER = 106;
///     ADD_FLAT_MODIFIER = 107;
///     ADD_PCT_MODIFIER = 108;
///     ADD_TARGET_TRIGGER = 109;
///     MOD_POWER_REGEN_PERCENT = 110;
///     ADD_CASTER_HIT_TRIGGER = 111;
///     OVERRIDE_CLASS_SCRIPTS = 112;
///     MOD_RANGED_DAMAGE_TAKEN = 113;
///     MOD_RANGED_DAMAGE_TAKEN_PCT = 114;
///     MOD_HEALING = 115;
///     MOD_REGEN_DURING_COMBAT = 116;
///     MOD_MECHANIC_RESISTANCE = 117;
///     MOD_HEALING_PCT = 118;
///     SHARE_PET_TRACKING = 119;
///     UNTRACKABLE = 120;
///     EMPATHY = 121;
///     MOD_OFFHAND_DAMAGE_PCT = 122;
///     MOD_TARGET_RESISTANCE = 123;
///     MOD_RANGED_ATTACK_POWER = 124;
///     MOD_MELEE_DAMAGE_TAKEN = 125;
///     MOD_MELEE_DAMAGE_TAKEN_PCT = 126;
///     RANGED_ATTACK_POWER_ATTACKER_BONUS = 127;
///     MOD_POSSESS_PET = 128;
///     MOD_SPEED_ALWAYS = 129;
///     MOD_MOUNTED_SPEED_ALWAYS = 130;
///     MOD_RANGED_ATTACK_POWER_VERSUS = 131;
///     MOD_INCREASE_ENERGY_PERCENT = 132;
///     MOD_INCREASE_HEALTH_PERCENT = 133;
///     MOD_MANA_REGEN_INTERRUPT = 134;
///     MOD_HEALING_DONE = 135;
///     MOD_HEALING_DONE_PERCENT = 136;
///     MOD_TOTAL_STAT_PERCENTAGE = 137;
///     MOD_MELEE_HASTE = 138;
///     FORCE_REACTION = 139;
///     MOD_RANGED_HASTE = 140;
///     MOD_RANGED_AMMO_HASTE = 141;
///     MOD_BASE_RESISTANCE_PCT = 142;
///     MOD_RESISTANCE_EXCLUSIVE = 143;
///     SAFE_FALL = 144;
///     CHARISMA = 145;
///     PERSUADED = 146;
///     MECHANIC_IMMUNITY_MASK = 147;
///     RETAIN_COMBO_POINTS = 148;
///     RESIST_PUSHBACK = 149;
///     MOD_SHIELD_BLOCKVALUE_PCT = 150;
///     TRACK_STEALTHED = 151;
///     MOD_DETECTED_RANGE = 152;
///     SPLIT_DAMAGE_FLAT = 153;
///     MOD_STEALTH_LEVEL = 154;
///     MOD_WATER_BREATHING = 155;
///     MOD_REPUTATION_GAIN = 156;
///     PET_DAMAGE_MULTI = 157;
///     MOD_SHIELD_BLOCKVALUE = 158;
///     NO_PVP_CREDIT = 159;
///     MOD_AOE_AVOIDANCE = 160;
///     MOD_HEALTH_REGEN_IN_COMBAT = 161;
///     POWER_BURN_MANA = 162;
///     MOD_CRIT_DAMAGE_BONUS = 163;
///     UNKNOWN164 = 164;
///     MELEE_ATTACK_POWER_ATTACKER_BONUS = 165;
///     MOD_ATTACK_POWER_PCT = 166;
///     MOD_RANGED_ATTACK_POWER_PCT = 167;
///     MOD_DAMAGE_DONE_VERSUS = 168;
///     MOD_CRIT_PERCENT_VERSUS = 169;
///     DETECT_AMORE = 170;
///     MOD_SPEED_NOT_STACK = 171;
///     MOD_MOUNTED_SPEED_NOT_STACK = 172;
///     ALLOW_CHAMPION_SPELLS = 173;
///     MOD_SPELL_DAMAGE_OF_STAT_PERCENT = 174;
///     MOD_SPELL_HEALING_OF_STAT_PERCENT = 175;
///     SPIRIT_OF_REDEMPTION = 176;
///     AOE_CHARM = 177;
///     MOD_DEBUFF_RESISTANCE = 178;
///     MOD_ATTACKER_SPELL_CRIT_CHANCE = 179;
///     MOD_FLAT_SPELL_DAMAGE_VERSUS = 180;
///     MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS = 181;
///     MOD_RESISTANCE_OF_STAT_PERCENT = 182;
///     MOD_CRITICAL_THREAT = 183;
///     MOD_ATTACKER_MELEE_HIT_CHANCE = 184;
///     MOD_ATTACKER_RANGED_HIT_CHANCE = 185;
///     MOD_ATTACKER_SPELL_HIT_CHANCE = 186;
///     MOD_ATTACKER_MELEE_CRIT_CHANCE = 187;
///     MOD_ATTACKER_RANGED_CRIT_CHANCE = 188;
///     MOD_RATING = 189;
///     MOD_FACTION_REPUTATION_GAIN = 190;
///     USE_NORMAL_MOVEMENT_SPEED = 191;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum AuraType {
    None,
    BindSight,
    ModPossess,
    /// vmangos: The aura should do periodic damage, the function that handles this is Aura::HandlePeriodicDamage, the amount is usually decided by the Unit::SpellDamageBonusDone or Unit::MeleeDamageBonusDone which increases/decreases the Modifier::m_amount
    ///
    PeriodicDamage,
    /// vmangos: Used by Aura::HandleAuraDummy
    ///
    Dummy,
    /// vmangos: Used by Aura::HandleModConfuse, will either confuse or unconfuse the target depending on whether the apply flag is set
    ///
    ModConfuse,
    ModCharm,
    ModFear,
    /// vmangos: The aura will do periodic heals of a target, handled by Aura::HandlePeriodicHeal, uses Unit::SpellHealingBonusDone to calculate whether to increase or decrease Modifier::m_amount
    ///
    PeriodicHeal,
    /// vmangos: Changes the attackspeed, the Modifier::m_amount decides how much we change in percent, ie, if the m_amount is 50 the attackspeed will increase by 50%
    ///
    ModAttackspeed,
    /// vmangos: Modifies the threat that the Aura does in percent, the Modifier::m_miscvalue decides which of the SpellSchools it should affect threat for.  \see SpellSchoolMask
    ///
    ModThreat,
    /// vmangos: Just applies a taunt which will change the threat a mob has Taken care of in Aura::HandleModThreat
    ///
    ModTaunt,
    /// vmangos: Stuns targets in different ways, taken care of in Aura::HandleAuraModStun
    ///
    ModStun,
    /// vmangos: Changes the damage done by a weapon in any hand, the Modifier::m_miscvalue will tell what school the damage is from, it's used as a bitmask \see SpellSchoolMask
    ///
    ModDamageDone,
    /// vmangos: Not handled by the Aura class but instead this is implemented in Unit::MeleeDamageBonusTaken and Unit::SpellBaseDamageBonusTaken
    ///
    ModDamageTaken,
    /// vmangos: Not handled by the Aura class, implemented in Unit::DealMeleeDamage
    ///
    DamageShield,
    /// vmangos: Taken care of in Aura::HandleModStealth, take note that this is not the same thing as invisibility
    ///
    ModStealth,
    /// vmangos: Not handled by the Aura class, implemented in Unit::isVisibleForOrDetect which does a lot of checks to determine whether the person is visible or not, the SPELL_AURA_MOD_STEALTH seems to determine how in/visible ie a rogue is.
    ///
    ModStealthDetect,
    /// vmangos: Handled by Aura::HandleInvisibility, the Modifier::m_miscvalue in the struct seems to decide what kind of invisibility it is with a bitflag. the miscvalue decides which bit is set, ie: 3 would make the 3rd bit be set.
    ///
    ModInvisibility,
    /// vmangos: Adds one of the kinds of detections to the possible detections.  As in SPEALL_AURA_MOD_INVISIBILITY the Modifier::m_miscvalue seems to decide what kind of invisibility the Unit should be able to detect.
    ///
    ModInvisibilityDetection,
    /// 20,21 unofficial
    ///
    ObsModHealth,
    ObsModMana,
    /// vmangos: Handled by Aura::HandleAuraModResistance, changes the resistance for a unit the field Modifier::m_miscvalue decides which kind of resistance that should be changed, for possible values see SpellSchools.  \see SpellSchools
    ///
    ModResistance,
    /// vmangos: Currently just sets Aura::m_isPeriodic to apply and has a special case for Curse of the Plaguebringer.
    ///
    PeriodicTriggerSpell,
    /// vmangos: Just sets Aura::m_isPeriodic to apply
    ///
    PeriodicEnergize,
    /// vmangos: Changes whether the target is pacified or not depending on the apply flag.  Pacify makes the target silenced and have all it's attack skill disabled.  See: `http://classic.wowhead.com/spell=6462`
    ///
    ModPacify,
    /// vmangos: Roots or unroots the target
    ///
    ModRoot,
    /// vmangos: Silences the target and stops and spell casts that should be stopped, they have the flag SpellPreventionType::SPELL_PREVENTION_TYPE_SILENCE
    ///
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
    /// Ignore all Gear test spells
    ///
    Unknown46,
    ModParryPercent,
    /// One periodic spell
    ///
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
    PeriodicManaLeech,
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
    PeriodicDamagePercent,
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
    /// Resist Pushback
    ///
    ResistPushback,
    ModShieldBlockvaluePct,
    /// Track Stealthed
    ///
    TrackStealthed,
    /// Mod Detected Range
    ///
    ModDetectedRange,
    /// Split Damage Flat
    ///
    SplitDamageFlat,
    /// Stealth Level Modifier
    ///
    ModStealthLevel,
    /// Mod Water Breathing
    ///
    ModWaterBreathing,
    /// Mod Reputation Gain
    ///
    ModReputationGain,
    /// Mod Pet Damage
    ///
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
    /// in 1.12.1 only dependent spirit case
    ///
    ModSpellDamageOfStatPercent,
    ModSpellHealingOfStatPercent,
    SpiritOfRedemption,
    AoeCharm,
    ModDebuffResistance,
    ModAttackerSpellCritChance,
    ModFlatSpellDamageVersus,
    /// unused - possible flat spell crit damage versus
    ///
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

impl AuraType {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::None => 0x0,
            Self::BindSight => 0x1,
            Self::ModPossess => 0x2,
            Self::PeriodicDamage => 0x3,
            Self::Dummy => 0x4,
            Self::ModConfuse => 0x5,
            Self::ModCharm => 0x6,
            Self::ModFear => 0x7,
            Self::PeriodicHeal => 0x8,
            Self::ModAttackspeed => 0x9,
            Self::ModThreat => 0xa,
            Self::ModTaunt => 0xb,
            Self::ModStun => 0xc,
            Self::ModDamageDone => 0xd,
            Self::ModDamageTaken => 0xe,
            Self::DamageShield => 0xf,
            Self::ModStealth => 0x10,
            Self::ModStealthDetect => 0x11,
            Self::ModInvisibility => 0x12,
            Self::ModInvisibilityDetection => 0x13,
            Self::ObsModHealth => 0x14,
            Self::ObsModMana => 0x15,
            Self::ModResistance => 0x16,
            Self::PeriodicTriggerSpell => 0x17,
            Self::PeriodicEnergize => 0x18,
            Self::ModPacify => 0x19,
            Self::ModRoot => 0x1a,
            Self::ModSilence => 0x1b,
            Self::ReflectSpells => 0x1c,
            Self::ModStat => 0x1d,
            Self::ModSkill => 0x1e,
            Self::ModIncreaseSpeed => 0x1f,
            Self::ModIncreaseMountedSpeed => 0x20,
            Self::ModDecreaseSpeed => 0x21,
            Self::ModIncreaseHealth => 0x22,
            Self::ModIncreaseEnergy => 0x23,
            Self::ModShapeshift => 0x24,
            Self::EffectImmunity => 0x25,
            Self::StateImmunity => 0x26,
            Self::SchoolImmunity => 0x27,
            Self::DamageImmunity => 0x28,
            Self::DispelImmunity => 0x29,
            Self::ProcTriggerSpell => 0x2a,
            Self::ProcTriggerDamage => 0x2b,
            Self::TrackCreatures => 0x2c,
            Self::TrackResources => 0x2d,
            Self::Unknown46 => 0x2e,
            Self::ModParryPercent => 0x2f,
            Self::Unknown48 => 0x30,
            Self::ModDodgePercent => 0x31,
            Self::ModBlockSkill => 0x32,
            Self::ModBlockPercent => 0x33,
            Self::ModCritPercent => 0x34,
            Self::PeriodicLeech => 0x35,
            Self::ModHitChance => 0x36,
            Self::ModSpellHitChance => 0x37,
            Self::Transform => 0x38,
            Self::ModSpellCritChance => 0x39,
            Self::ModIncreaseSwimSpeed => 0x3a,
            Self::ModDamageDoneCreature => 0x3b,
            Self::ModPacifySilence => 0x3c,
            Self::ModScale => 0x3d,
            Self::PeriodicHealthFunnel => 0x3e,
            Self::PeriodicManaFunnel => 0x3f,
            Self::PeriodicManaLeech => 0x40,
            Self::ModCastingSpeedNotStack => 0x41,
            Self::FeignDeath => 0x42,
            Self::ModDisarm => 0x43,
            Self::ModStalked => 0x44,
            Self::SchoolAbsorb => 0x45,
            Self::ExtraAttacks => 0x46,
            Self::ModSpellCritChanceSchool => 0x47,
            Self::ModPowerCostSchoolPct => 0x48,
            Self::ModPowerCostSchool => 0x49,
            Self::ReflectSpellsSchool => 0x4a,
            Self::ModLanguage => 0x4b,
            Self::FarSight => 0x4c,
            Self::MechanicImmunity => 0x4d,
            Self::Mounted => 0x4e,
            Self::ModDamagePercentDone => 0x4f,
            Self::ModPercentStat => 0x50,
            Self::SplitDamagePct => 0x51,
            Self::WaterBreathing => 0x52,
            Self::ModBaseResistance => 0x53,
            Self::ModRegen => 0x54,
            Self::ModPowerRegen => 0x55,
            Self::ChannelDeathItem => 0x56,
            Self::ModDamagePercentTaken => 0x57,
            Self::ModHealthRegenPercent => 0x58,
            Self::PeriodicDamagePercent => 0x59,
            Self::ModResistChance => 0x5a,
            Self::ModDetectRange => 0x5b,
            Self::PreventsFleeing => 0x5c,
            Self::ModUnattackable => 0x5d,
            Self::InterruptRegen => 0x5e,
            Self::Ghost => 0x5f,
            Self::SpellMagnet => 0x60,
            Self::ManaShield => 0x61,
            Self::ModSkillTalent => 0x62,
            Self::ModAttackPower => 0x63,
            Self::AurasVisible => 0x64,
            Self::ModResistancePct => 0x65,
            Self::ModMeleeAttackPowerVersus => 0x66,
            Self::ModTotalThreat => 0x67,
            Self::WaterWalk => 0x68,
            Self::FeatherFall => 0x69,
            Self::Hover => 0x6a,
            Self::AddFlatModifier => 0x6b,
            Self::AddPctModifier => 0x6c,
            Self::AddTargetTrigger => 0x6d,
            Self::ModPowerRegenPercent => 0x6e,
            Self::AddCasterHitTrigger => 0x6f,
            Self::OverrideClassScripts => 0x70,
            Self::ModRangedDamageTaken => 0x71,
            Self::ModRangedDamageTakenPct => 0x72,
            Self::ModHealing => 0x73,
            Self::ModRegenDuringCombat => 0x74,
            Self::ModMechanicResistance => 0x75,
            Self::ModHealingPct => 0x76,
            Self::SharePetTracking => 0x77,
            Self::Untrackable => 0x78,
            Self::Empathy => 0x79,
            Self::ModOffhandDamagePct => 0x7a,
            Self::ModTargetResistance => 0x7b,
            Self::ModRangedAttackPower => 0x7c,
            Self::ModMeleeDamageTaken => 0x7d,
            Self::ModMeleeDamageTakenPct => 0x7e,
            Self::RangedAttackPowerAttackerBonus => 0x7f,
            Self::ModPossessPet => 0x80,
            Self::ModSpeedAlways => 0x81,
            Self::ModMountedSpeedAlways => 0x82,
            Self::ModRangedAttackPowerVersus => 0x83,
            Self::ModIncreaseEnergyPercent => 0x84,
            Self::ModIncreaseHealthPercent => 0x85,
            Self::ModManaRegenInterrupt => 0x86,
            Self::ModHealingDone => 0x87,
            Self::ModHealingDonePercent => 0x88,
            Self::ModTotalStatPercentage => 0x89,
            Self::ModMeleeHaste => 0x8a,
            Self::ForceReaction => 0x8b,
            Self::ModRangedHaste => 0x8c,
            Self::ModRangedAmmoHaste => 0x8d,
            Self::ModBaseResistancePct => 0x8e,
            Self::ModResistanceExclusive => 0x8f,
            Self::SafeFall => 0x90,
            Self::Charisma => 0x91,
            Self::Persuaded => 0x92,
            Self::MechanicImmunityMask => 0x93,
            Self::RetainComboPoints => 0x94,
            Self::ResistPushback => 0x95,
            Self::ModShieldBlockvaluePct => 0x96,
            Self::TrackStealthed => 0x97,
            Self::ModDetectedRange => 0x98,
            Self::SplitDamageFlat => 0x99,
            Self::ModStealthLevel => 0x9a,
            Self::ModWaterBreathing => 0x9b,
            Self::ModReputationGain => 0x9c,
            Self::PetDamageMulti => 0x9d,
            Self::ModShieldBlockvalue => 0x9e,
            Self::NoPvpCredit => 0x9f,
            Self::ModAoeAvoidance => 0xa0,
            Self::ModHealthRegenInCombat => 0xa1,
            Self::PowerBurnMana => 0xa2,
            Self::ModCritDamageBonus => 0xa3,
            Self::Unknown164 => 0xa4,
            Self::MeleeAttackPowerAttackerBonus => 0xa5,
            Self::ModAttackPowerPct => 0xa6,
            Self::ModRangedAttackPowerPct => 0xa7,
            Self::ModDamageDoneVersus => 0xa8,
            Self::ModCritPercentVersus => 0xa9,
            Self::DetectAmore => 0xaa,
            Self::ModSpeedNotStack => 0xab,
            Self::ModMountedSpeedNotStack => 0xac,
            Self::AllowChampionSpells => 0xad,
            Self::ModSpellDamageOfStatPercent => 0xae,
            Self::ModSpellHealingOfStatPercent => 0xaf,
            Self::SpiritOfRedemption => 0xb0,
            Self::AoeCharm => 0xb1,
            Self::ModDebuffResistance => 0xb2,
            Self::ModAttackerSpellCritChance => 0xb3,
            Self::ModFlatSpellDamageVersus => 0xb4,
            Self::ModFlatSpellCritDamageVersus => 0xb5,
            Self::ModResistanceOfStatPercent => 0xb6,
            Self::ModCriticalThreat => 0xb7,
            Self::ModAttackerMeleeHitChance => 0xb8,
            Self::ModAttackerRangedHitChance => 0xb9,
            Self::ModAttackerSpellHitChance => 0xba,
            Self::ModAttackerMeleeCritChance => 0xbb,
            Self::ModAttackerRangedCritChance => 0xbc,
            Self::ModRating => 0xbd,
            Self::ModFactionReputationGain => 0xbe,
            Self::UseNormalMovementSpeed => 0xbf,
        }
    }

}

impl Default for AuraType {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for AuraType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::BindSight => f.write_str("BindSight"),
            Self::ModPossess => f.write_str("ModPossess"),
            Self::PeriodicDamage => f.write_str("PeriodicDamage"),
            Self::Dummy => f.write_str("Dummy"),
            Self::ModConfuse => f.write_str("ModConfuse"),
            Self::ModCharm => f.write_str("ModCharm"),
            Self::ModFear => f.write_str("ModFear"),
            Self::PeriodicHeal => f.write_str("PeriodicHeal"),
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
            Self::ObsModHealth => f.write_str("ObsModHealth"),
            Self::ObsModMana => f.write_str("ObsModMana"),
            Self::ModResistance => f.write_str("ModResistance"),
            Self::PeriodicTriggerSpell => f.write_str("PeriodicTriggerSpell"),
            Self::PeriodicEnergize => f.write_str("PeriodicEnergize"),
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
            Self::PeriodicManaLeech => f.write_str("PeriodicManaLeech"),
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
            Self::PeriodicDamagePercent => f.write_str("PeriodicDamagePercent"),
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
        }
    }
}

impl TryFrom<u32> for AuraType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::BindSight),
            2 => Ok(Self::ModPossess),
            3 => Ok(Self::PeriodicDamage),
            4 => Ok(Self::Dummy),
            5 => Ok(Self::ModConfuse),
            6 => Ok(Self::ModCharm),
            7 => Ok(Self::ModFear),
            8 => Ok(Self::PeriodicHeal),
            9 => Ok(Self::ModAttackspeed),
            10 => Ok(Self::ModThreat),
            11 => Ok(Self::ModTaunt),
            12 => Ok(Self::ModStun),
            13 => Ok(Self::ModDamageDone),
            14 => Ok(Self::ModDamageTaken),
            15 => Ok(Self::DamageShield),
            16 => Ok(Self::ModStealth),
            17 => Ok(Self::ModStealthDetect),
            18 => Ok(Self::ModInvisibility),
            19 => Ok(Self::ModInvisibilityDetection),
            20 => Ok(Self::ObsModHealth),
            21 => Ok(Self::ObsModMana),
            22 => Ok(Self::ModResistance),
            23 => Ok(Self::PeriodicTriggerSpell),
            24 => Ok(Self::PeriodicEnergize),
            25 => Ok(Self::ModPacify),
            26 => Ok(Self::ModRoot),
            27 => Ok(Self::ModSilence),
            28 => Ok(Self::ReflectSpells),
            29 => Ok(Self::ModStat),
            30 => Ok(Self::ModSkill),
            31 => Ok(Self::ModIncreaseSpeed),
            32 => Ok(Self::ModIncreaseMountedSpeed),
            33 => Ok(Self::ModDecreaseSpeed),
            34 => Ok(Self::ModIncreaseHealth),
            35 => Ok(Self::ModIncreaseEnergy),
            36 => Ok(Self::ModShapeshift),
            37 => Ok(Self::EffectImmunity),
            38 => Ok(Self::StateImmunity),
            39 => Ok(Self::SchoolImmunity),
            40 => Ok(Self::DamageImmunity),
            41 => Ok(Self::DispelImmunity),
            42 => Ok(Self::ProcTriggerSpell),
            43 => Ok(Self::ProcTriggerDamage),
            44 => Ok(Self::TrackCreatures),
            45 => Ok(Self::TrackResources),
            46 => Ok(Self::Unknown46),
            47 => Ok(Self::ModParryPercent),
            48 => Ok(Self::Unknown48),
            49 => Ok(Self::ModDodgePercent),
            50 => Ok(Self::ModBlockSkill),
            51 => Ok(Self::ModBlockPercent),
            52 => Ok(Self::ModCritPercent),
            53 => Ok(Self::PeriodicLeech),
            54 => Ok(Self::ModHitChance),
            55 => Ok(Self::ModSpellHitChance),
            56 => Ok(Self::Transform),
            57 => Ok(Self::ModSpellCritChance),
            58 => Ok(Self::ModIncreaseSwimSpeed),
            59 => Ok(Self::ModDamageDoneCreature),
            60 => Ok(Self::ModPacifySilence),
            61 => Ok(Self::ModScale),
            62 => Ok(Self::PeriodicHealthFunnel),
            63 => Ok(Self::PeriodicManaFunnel),
            64 => Ok(Self::PeriodicManaLeech),
            65 => Ok(Self::ModCastingSpeedNotStack),
            66 => Ok(Self::FeignDeath),
            67 => Ok(Self::ModDisarm),
            68 => Ok(Self::ModStalked),
            69 => Ok(Self::SchoolAbsorb),
            70 => Ok(Self::ExtraAttacks),
            71 => Ok(Self::ModSpellCritChanceSchool),
            72 => Ok(Self::ModPowerCostSchoolPct),
            73 => Ok(Self::ModPowerCostSchool),
            74 => Ok(Self::ReflectSpellsSchool),
            75 => Ok(Self::ModLanguage),
            76 => Ok(Self::FarSight),
            77 => Ok(Self::MechanicImmunity),
            78 => Ok(Self::Mounted),
            79 => Ok(Self::ModDamagePercentDone),
            80 => Ok(Self::ModPercentStat),
            81 => Ok(Self::SplitDamagePct),
            82 => Ok(Self::WaterBreathing),
            83 => Ok(Self::ModBaseResistance),
            84 => Ok(Self::ModRegen),
            85 => Ok(Self::ModPowerRegen),
            86 => Ok(Self::ChannelDeathItem),
            87 => Ok(Self::ModDamagePercentTaken),
            88 => Ok(Self::ModHealthRegenPercent),
            89 => Ok(Self::PeriodicDamagePercent),
            90 => Ok(Self::ModResistChance),
            91 => Ok(Self::ModDetectRange),
            92 => Ok(Self::PreventsFleeing),
            93 => Ok(Self::ModUnattackable),
            94 => Ok(Self::InterruptRegen),
            95 => Ok(Self::Ghost),
            96 => Ok(Self::SpellMagnet),
            97 => Ok(Self::ManaShield),
            98 => Ok(Self::ModSkillTalent),
            99 => Ok(Self::ModAttackPower),
            100 => Ok(Self::AurasVisible),
            101 => Ok(Self::ModResistancePct),
            102 => Ok(Self::ModMeleeAttackPowerVersus),
            103 => Ok(Self::ModTotalThreat),
            104 => Ok(Self::WaterWalk),
            105 => Ok(Self::FeatherFall),
            106 => Ok(Self::Hover),
            107 => Ok(Self::AddFlatModifier),
            108 => Ok(Self::AddPctModifier),
            109 => Ok(Self::AddTargetTrigger),
            110 => Ok(Self::ModPowerRegenPercent),
            111 => Ok(Self::AddCasterHitTrigger),
            112 => Ok(Self::OverrideClassScripts),
            113 => Ok(Self::ModRangedDamageTaken),
            114 => Ok(Self::ModRangedDamageTakenPct),
            115 => Ok(Self::ModHealing),
            116 => Ok(Self::ModRegenDuringCombat),
            117 => Ok(Self::ModMechanicResistance),
            118 => Ok(Self::ModHealingPct),
            119 => Ok(Self::SharePetTracking),
            120 => Ok(Self::Untrackable),
            121 => Ok(Self::Empathy),
            122 => Ok(Self::ModOffhandDamagePct),
            123 => Ok(Self::ModTargetResistance),
            124 => Ok(Self::ModRangedAttackPower),
            125 => Ok(Self::ModMeleeDamageTaken),
            126 => Ok(Self::ModMeleeDamageTakenPct),
            127 => Ok(Self::RangedAttackPowerAttackerBonus),
            128 => Ok(Self::ModPossessPet),
            129 => Ok(Self::ModSpeedAlways),
            130 => Ok(Self::ModMountedSpeedAlways),
            131 => Ok(Self::ModRangedAttackPowerVersus),
            132 => Ok(Self::ModIncreaseEnergyPercent),
            133 => Ok(Self::ModIncreaseHealthPercent),
            134 => Ok(Self::ModManaRegenInterrupt),
            135 => Ok(Self::ModHealingDone),
            136 => Ok(Self::ModHealingDonePercent),
            137 => Ok(Self::ModTotalStatPercentage),
            138 => Ok(Self::ModMeleeHaste),
            139 => Ok(Self::ForceReaction),
            140 => Ok(Self::ModRangedHaste),
            141 => Ok(Self::ModRangedAmmoHaste),
            142 => Ok(Self::ModBaseResistancePct),
            143 => Ok(Self::ModResistanceExclusive),
            144 => Ok(Self::SafeFall),
            145 => Ok(Self::Charisma),
            146 => Ok(Self::Persuaded),
            147 => Ok(Self::MechanicImmunityMask),
            148 => Ok(Self::RetainComboPoints),
            149 => Ok(Self::ResistPushback),
            150 => Ok(Self::ModShieldBlockvaluePct),
            151 => Ok(Self::TrackStealthed),
            152 => Ok(Self::ModDetectedRange),
            153 => Ok(Self::SplitDamageFlat),
            154 => Ok(Self::ModStealthLevel),
            155 => Ok(Self::ModWaterBreathing),
            156 => Ok(Self::ModReputationGain),
            157 => Ok(Self::PetDamageMulti),
            158 => Ok(Self::ModShieldBlockvalue),
            159 => Ok(Self::NoPvpCredit),
            160 => Ok(Self::ModAoeAvoidance),
            161 => Ok(Self::ModHealthRegenInCombat),
            162 => Ok(Self::PowerBurnMana),
            163 => Ok(Self::ModCritDamageBonus),
            164 => Ok(Self::Unknown164),
            165 => Ok(Self::MeleeAttackPowerAttackerBonus),
            166 => Ok(Self::ModAttackPowerPct),
            167 => Ok(Self::ModRangedAttackPowerPct),
            168 => Ok(Self::ModDamageDoneVersus),
            169 => Ok(Self::ModCritPercentVersus),
            170 => Ok(Self::DetectAmore),
            171 => Ok(Self::ModSpeedNotStack),
            172 => Ok(Self::ModMountedSpeedNotStack),
            173 => Ok(Self::AllowChampionSpells),
            174 => Ok(Self::ModSpellDamageOfStatPercent),
            175 => Ok(Self::ModSpellHealingOfStatPercent),
            176 => Ok(Self::SpiritOfRedemption),
            177 => Ok(Self::AoeCharm),
            178 => Ok(Self::ModDebuffResistance),
            179 => Ok(Self::ModAttackerSpellCritChance),
            180 => Ok(Self::ModFlatSpellDamageVersus),
            181 => Ok(Self::ModFlatSpellCritDamageVersus),
            182 => Ok(Self::ModResistanceOfStatPercent),
            183 => Ok(Self::ModCriticalThreat),
            184 => Ok(Self::ModAttackerMeleeHitChance),
            185 => Ok(Self::ModAttackerRangedHitChance),
            186 => Ok(Self::ModAttackerSpellHitChance),
            187 => Ok(Self::ModAttackerMeleeCritChance),
            188 => Ok(Self::ModAttackerRangedCritChance),
            189 => Ok(Self::ModRating),
            190 => Ok(Self::ModFactionReputationGain),
            191 => Ok(Self::UseNormalMovementSpeed),
            v => Err(crate::errors::EnumError::new("AuraType", v as u64),)
        }
    }
}

