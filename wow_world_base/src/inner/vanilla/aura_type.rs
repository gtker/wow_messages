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
#[derive(Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum AuraType {
    #[default]
    None,
    BindSight,
    ModPossess,
    /// vmangos: The aura should do periodic damage, the function that handles this is Aura::HandlePeriodicDamage, the amount is usually decided by the Unit::SpellDamageBonusDone or Unit::MeleeDamageBonusDone which increases/decreases the Modifier::m_amount
    PeriodicDamage,
    /// vmangos: Used by Aura::HandleAuraDummy
    Dummy,
    /// vmangos: Used by Aura::HandleModConfuse, will either confuse or unconfuse the target depending on whether the apply flag is set
    ModConfuse,
    ModCharm,
    ModFear,
    /// vmangos: The aura will do periodic heals of a target, handled by Aura::HandlePeriodicHeal, uses Unit::SpellHealingBonusDone to calculate whether to increase or decrease Modifier::m_amount
    PeriodicHeal,
    /// vmangos: Changes the attackspeed, the Modifier::m_amount decides how much we change in percent, ie, if the m_amount is 50 the attackspeed will increase by 50%
    ModAttackspeed,
    /// vmangos: Modifies the threat that the Aura does in percent, the Modifier::m_miscvalue decides which of the SpellSchools it should affect threat for.  \see SpellSchoolMask
    ModThreat,
    /// vmangos: Just applies a taunt which will change the threat a mob has Taken care of in Aura::HandleModThreat
    ModTaunt,
    /// vmangos: Stuns targets in different ways, taken care of in Aura::HandleAuraModStun
    ModStun,
    /// vmangos: Changes the damage done by a weapon in any hand, the Modifier::m_miscvalue will tell what school the damage is from, it's used as a bitmask \see SpellSchoolMask
    ModDamageDone,
    /// vmangos: Not handled by the Aura class but instead this is implemented in Unit::MeleeDamageBonusTaken and Unit::SpellBaseDamageBonusTaken
    ModDamageTaken,
    /// vmangos: Not handled by the Aura class, implemented in Unit::DealMeleeDamage
    DamageShield,
    /// vmangos: Taken care of in Aura::HandleModStealth, take note that this is not the same thing as invisibility
    ModStealth,
    /// vmangos: Not handled by the Aura class, implemented in Unit::isVisibleForOrDetect which does a lot of checks to determine whether the person is visible or not, the SPELL_AURA_MOD_STEALTH seems to determine how in/visible ie a rogue is.
    ModStealthDetect,
    /// vmangos: Handled by Aura::HandleInvisibility, the Modifier::m_miscvalue in the struct seems to decide what kind of invisibility it is with a bitflag. the miscvalue decides which bit is set, ie: 3 would make the 3rd bit be set.
    ModInvisibility,
    /// vmangos: Adds one of the kinds of detections to the possible detections.  As in SPEALL_AURA_MOD_INVISIBILITY the Modifier::m_miscvalue seems to decide what kind of invisibility the Unit should be able to detect.
    ModInvisibilityDetection,
    /// 20,21 unofficial
    ObsModHealth,
    ObsModMana,
    /// vmangos: Handled by Aura::HandleAuraModResistance, changes the resistance for a unit the field Modifier::m_miscvalue decides which kind of resistance that should be changed, for possible values see SpellSchools.  \see SpellSchools
    ModResistance,
    /// vmangos: Currently just sets Aura::m_isPeriodic to apply and has a special case for Curse of the Plaguebringer.
    PeriodicTriggerSpell,
    /// vmangos: Just sets Aura::m_isPeriodic to apply
    PeriodicEnergize,
    /// vmangos: Changes whether the target is pacified or not depending on the apply flag.  Pacify makes the target silenced and have all it's attack skill disabled.  See: `http://classic.wowhead.com/spell=6462`
    ModPacify,
    /// vmangos: Roots or unroots the target
    ModRoot,
    /// vmangos: Silences the target and stops and spell casts that should be stopped, they have the flag SpellPreventionType::SPELL_PREVENTION_TYPE_SILENCE
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
    Unknown46,
    ModParryPercent,
    /// One periodic spell
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
    ResistPushback,
    ModShieldBlockvaluePct,
    /// Track Stealthed
    TrackStealthed,
    /// Mod Detected Range
    ModDetectedRange,
    /// Split Damage Flat
    SplitDamageFlat,
    /// Stealth Level Modifier
    ModStealthLevel,
    /// Mod Water Breathing
    ModWaterBreathing,
    /// Mod Reputation Gain
    ModReputationGain,
    /// Mod Pet Damage
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
    ModSpellDamageOfStatPercent,
    ModSpellHealingOfStatPercent,
    SpiritOfRedemption,
    AoeCharm,
    ModDebuffResistance,
    ModAttackerSpellCritChance,
    ModFlatSpellDamageVersus,
    /// unused - possible flat spell crit damage versus
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

    pub const fn variants() -> [Self; 192] {
        [
            Self::None,
            Self::BindSight,
            Self::ModPossess,
            Self::PeriodicDamage,
            Self::Dummy,
            Self::ModConfuse,
            Self::ModCharm,
            Self::ModFear,
            Self::PeriodicHeal,
            Self::ModAttackspeed,
            Self::ModThreat,
            Self::ModTaunt,
            Self::ModStun,
            Self::ModDamageDone,
            Self::ModDamageTaken,
            Self::DamageShield,
            Self::ModStealth,
            Self::ModStealthDetect,
            Self::ModInvisibility,
            Self::ModInvisibilityDetection,
            Self::ObsModHealth,
            Self::ObsModMana,
            Self::ModResistance,
            Self::PeriodicTriggerSpell,
            Self::PeriodicEnergize,
            Self::ModPacify,
            Self::ModRoot,
            Self::ModSilence,
            Self::ReflectSpells,
            Self::ModStat,
            Self::ModSkill,
            Self::ModIncreaseSpeed,
            Self::ModIncreaseMountedSpeed,
            Self::ModDecreaseSpeed,
            Self::ModIncreaseHealth,
            Self::ModIncreaseEnergy,
            Self::ModShapeshift,
            Self::EffectImmunity,
            Self::StateImmunity,
            Self::SchoolImmunity,
            Self::DamageImmunity,
            Self::DispelImmunity,
            Self::ProcTriggerSpell,
            Self::ProcTriggerDamage,
            Self::TrackCreatures,
            Self::TrackResources,
            Self::Unknown46,
            Self::ModParryPercent,
            Self::Unknown48,
            Self::ModDodgePercent,
            Self::ModBlockSkill,
            Self::ModBlockPercent,
            Self::ModCritPercent,
            Self::PeriodicLeech,
            Self::ModHitChance,
            Self::ModSpellHitChance,
            Self::Transform,
            Self::ModSpellCritChance,
            Self::ModIncreaseSwimSpeed,
            Self::ModDamageDoneCreature,
            Self::ModPacifySilence,
            Self::ModScale,
            Self::PeriodicHealthFunnel,
            Self::PeriodicManaFunnel,
            Self::PeriodicManaLeech,
            Self::ModCastingSpeedNotStack,
            Self::FeignDeath,
            Self::ModDisarm,
            Self::ModStalked,
            Self::SchoolAbsorb,
            Self::ExtraAttacks,
            Self::ModSpellCritChanceSchool,
            Self::ModPowerCostSchoolPct,
            Self::ModPowerCostSchool,
            Self::ReflectSpellsSchool,
            Self::ModLanguage,
            Self::FarSight,
            Self::MechanicImmunity,
            Self::Mounted,
            Self::ModDamagePercentDone,
            Self::ModPercentStat,
            Self::SplitDamagePct,
            Self::WaterBreathing,
            Self::ModBaseResistance,
            Self::ModRegen,
            Self::ModPowerRegen,
            Self::ChannelDeathItem,
            Self::ModDamagePercentTaken,
            Self::ModHealthRegenPercent,
            Self::PeriodicDamagePercent,
            Self::ModResistChance,
            Self::ModDetectRange,
            Self::PreventsFleeing,
            Self::ModUnattackable,
            Self::InterruptRegen,
            Self::Ghost,
            Self::SpellMagnet,
            Self::ManaShield,
            Self::ModSkillTalent,
            Self::ModAttackPower,
            Self::AurasVisible,
            Self::ModResistancePct,
            Self::ModMeleeAttackPowerVersus,
            Self::ModTotalThreat,
            Self::WaterWalk,
            Self::FeatherFall,
            Self::Hover,
            Self::AddFlatModifier,
            Self::AddPctModifier,
            Self::AddTargetTrigger,
            Self::ModPowerRegenPercent,
            Self::AddCasterHitTrigger,
            Self::OverrideClassScripts,
            Self::ModRangedDamageTaken,
            Self::ModRangedDamageTakenPct,
            Self::ModHealing,
            Self::ModRegenDuringCombat,
            Self::ModMechanicResistance,
            Self::ModHealingPct,
            Self::SharePetTracking,
            Self::Untrackable,
            Self::Empathy,
            Self::ModOffhandDamagePct,
            Self::ModTargetResistance,
            Self::ModRangedAttackPower,
            Self::ModMeleeDamageTaken,
            Self::ModMeleeDamageTakenPct,
            Self::RangedAttackPowerAttackerBonus,
            Self::ModPossessPet,
            Self::ModSpeedAlways,
            Self::ModMountedSpeedAlways,
            Self::ModRangedAttackPowerVersus,
            Self::ModIncreaseEnergyPercent,
            Self::ModIncreaseHealthPercent,
            Self::ModManaRegenInterrupt,
            Self::ModHealingDone,
            Self::ModHealingDonePercent,
            Self::ModTotalStatPercentage,
            Self::ModMeleeHaste,
            Self::ForceReaction,
            Self::ModRangedHaste,
            Self::ModRangedAmmoHaste,
            Self::ModBaseResistancePct,
            Self::ModResistanceExclusive,
            Self::SafeFall,
            Self::Charisma,
            Self::Persuaded,
            Self::MechanicImmunityMask,
            Self::RetainComboPoints,
            Self::ResistPushback,
            Self::ModShieldBlockvaluePct,
            Self::TrackStealthed,
            Self::ModDetectedRange,
            Self::SplitDamageFlat,
            Self::ModStealthLevel,
            Self::ModWaterBreathing,
            Self::ModReputationGain,
            Self::PetDamageMulti,
            Self::ModShieldBlockvalue,
            Self::NoPvpCredit,
            Self::ModAoeAvoidance,
            Self::ModHealthRegenInCombat,
            Self::PowerBurnMana,
            Self::ModCritDamageBonus,
            Self::Unknown164,
            Self::MeleeAttackPowerAttackerBonus,
            Self::ModAttackPowerPct,
            Self::ModRangedAttackPowerPct,
            Self::ModDamageDoneVersus,
            Self::ModCritPercentVersus,
            Self::DetectAmore,
            Self::ModSpeedNotStack,
            Self::ModMountedSpeedNotStack,
            Self::AllowChampionSpells,
            Self::ModSpellDamageOfStatPercent,
            Self::ModSpellHealingOfStatPercent,
            Self::SpiritOfRedemption,
            Self::AoeCharm,
            Self::ModDebuffResistance,
            Self::ModAttackerSpellCritChance,
            Self::ModFlatSpellDamageVersus,
            Self::ModFlatSpellCritDamageVersus,
            Self::ModResistanceOfStatPercent,
            Self::ModCriticalThreat,
            Self::ModAttackerMeleeHitChance,
            Self::ModAttackerRangedHitChance,
            Self::ModAttackerSpellHitChance,
            Self::ModAttackerMeleeCritChance,
            Self::ModAttackerRangedCritChance,
            Self::ModRating,
            Self::ModFactionReputationGain,
            Self::UseNormalMovementSpeed,
        ]
    }

    pub const fn from_int(value: u32) -> Result<Self, crate::errors::EnumError> {
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
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl AuraType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::BindSight => "BIND_SIGHT",
            Self::ModPossess => "MOD_POSSESS",
            Self::PeriodicDamage => "PERIODIC_DAMAGE",
            Self::Dummy => "DUMMY",
            Self::ModConfuse => "MOD_CONFUSE",
            Self::ModCharm => "MOD_CHARM",
            Self::ModFear => "MOD_FEAR",
            Self::PeriodicHeal => "PERIODIC_HEAL",
            Self::ModAttackspeed => "MOD_ATTACKSPEED",
            Self::ModThreat => "MOD_THREAT",
            Self::ModTaunt => "MOD_TAUNT",
            Self::ModStun => "MOD_STUN",
            Self::ModDamageDone => "MOD_DAMAGE_DONE",
            Self::ModDamageTaken => "MOD_DAMAGE_TAKEN",
            Self::DamageShield => "DAMAGE_SHIELD",
            Self::ModStealth => "MOD_STEALTH",
            Self::ModStealthDetect => "MOD_STEALTH_DETECT",
            Self::ModInvisibility => "MOD_INVISIBILITY",
            Self::ModInvisibilityDetection => "MOD_INVISIBILITY_DETECTION",
            Self::ObsModHealth => "OBS_MOD_HEALTH",
            Self::ObsModMana => "OBS_MOD_MANA",
            Self::ModResistance => "MOD_RESISTANCE",
            Self::PeriodicTriggerSpell => "PERIODIC_TRIGGER_SPELL",
            Self::PeriodicEnergize => "PERIODIC_ENERGIZE",
            Self::ModPacify => "MOD_PACIFY",
            Self::ModRoot => "MOD_ROOT",
            Self::ModSilence => "MOD_SILENCE",
            Self::ReflectSpells => "REFLECT_SPELLS",
            Self::ModStat => "MOD_STAT",
            Self::ModSkill => "MOD_SKILL",
            Self::ModIncreaseSpeed => "MOD_INCREASE_SPEED",
            Self::ModIncreaseMountedSpeed => "MOD_INCREASE_MOUNTED_SPEED",
            Self::ModDecreaseSpeed => "MOD_DECREASE_SPEED",
            Self::ModIncreaseHealth => "MOD_INCREASE_HEALTH",
            Self::ModIncreaseEnergy => "MOD_INCREASE_ENERGY",
            Self::ModShapeshift => "MOD_SHAPESHIFT",
            Self::EffectImmunity => "EFFECT_IMMUNITY",
            Self::StateImmunity => "STATE_IMMUNITY",
            Self::SchoolImmunity => "SCHOOL_IMMUNITY",
            Self::DamageImmunity => "DAMAGE_IMMUNITY",
            Self::DispelImmunity => "DISPEL_IMMUNITY",
            Self::ProcTriggerSpell => "PROC_TRIGGER_SPELL",
            Self::ProcTriggerDamage => "PROC_TRIGGER_DAMAGE",
            Self::TrackCreatures => "TRACK_CREATURES",
            Self::TrackResources => "TRACK_RESOURCES",
            Self::Unknown46 => "UNKNOWN46",
            Self::ModParryPercent => "MOD_PARRY_PERCENT",
            Self::Unknown48 => "UNKNOWN48",
            Self::ModDodgePercent => "MOD_DODGE_PERCENT",
            Self::ModBlockSkill => "MOD_BLOCK_SKILL",
            Self::ModBlockPercent => "MOD_BLOCK_PERCENT",
            Self::ModCritPercent => "MOD_CRIT_PERCENT",
            Self::PeriodicLeech => "PERIODIC_LEECH",
            Self::ModHitChance => "MOD_HIT_CHANCE",
            Self::ModSpellHitChance => "MOD_SPELL_HIT_CHANCE",
            Self::Transform => "TRANSFORM",
            Self::ModSpellCritChance => "MOD_SPELL_CRIT_CHANCE",
            Self::ModIncreaseSwimSpeed => "MOD_INCREASE_SWIM_SPEED",
            Self::ModDamageDoneCreature => "MOD_DAMAGE_DONE_CREATURE",
            Self::ModPacifySilence => "MOD_PACIFY_SILENCE",
            Self::ModScale => "MOD_SCALE",
            Self::PeriodicHealthFunnel => "PERIODIC_HEALTH_FUNNEL",
            Self::PeriodicManaFunnel => "PERIODIC_MANA_FUNNEL",
            Self::PeriodicManaLeech => "PERIODIC_MANA_LEECH",
            Self::ModCastingSpeedNotStack => "MOD_CASTING_SPEED_NOT_STACK",
            Self::FeignDeath => "FEIGN_DEATH",
            Self::ModDisarm => "MOD_DISARM",
            Self::ModStalked => "MOD_STALKED",
            Self::SchoolAbsorb => "SCHOOL_ABSORB",
            Self::ExtraAttacks => "EXTRA_ATTACKS",
            Self::ModSpellCritChanceSchool => "MOD_SPELL_CRIT_CHANCE_SCHOOL",
            Self::ModPowerCostSchoolPct => "MOD_POWER_COST_SCHOOL_PCT",
            Self::ModPowerCostSchool => "MOD_POWER_COST_SCHOOL",
            Self::ReflectSpellsSchool => "REFLECT_SPELLS_SCHOOL",
            Self::ModLanguage => "MOD_LANGUAGE",
            Self::FarSight => "FAR_SIGHT",
            Self::MechanicImmunity => "MECHANIC_IMMUNITY",
            Self::Mounted => "MOUNTED",
            Self::ModDamagePercentDone => "MOD_DAMAGE_PERCENT_DONE",
            Self::ModPercentStat => "MOD_PERCENT_STAT",
            Self::SplitDamagePct => "SPLIT_DAMAGE_PCT",
            Self::WaterBreathing => "WATER_BREATHING",
            Self::ModBaseResistance => "MOD_BASE_RESISTANCE",
            Self::ModRegen => "MOD_REGEN",
            Self::ModPowerRegen => "MOD_POWER_REGEN",
            Self::ChannelDeathItem => "CHANNEL_DEATH_ITEM",
            Self::ModDamagePercentTaken => "MOD_DAMAGE_PERCENT_TAKEN",
            Self::ModHealthRegenPercent => "MOD_HEALTH_REGEN_PERCENT",
            Self::PeriodicDamagePercent => "PERIODIC_DAMAGE_PERCENT",
            Self::ModResistChance => "MOD_RESIST_CHANCE",
            Self::ModDetectRange => "MOD_DETECT_RANGE",
            Self::PreventsFleeing => "PREVENTS_FLEEING",
            Self::ModUnattackable => "MOD_UNATTACKABLE",
            Self::InterruptRegen => "INTERRUPT_REGEN",
            Self::Ghost => "GHOST",
            Self::SpellMagnet => "SPELL_MAGNET",
            Self::ManaShield => "MANA_SHIELD",
            Self::ModSkillTalent => "MOD_SKILL_TALENT",
            Self::ModAttackPower => "MOD_ATTACK_POWER",
            Self::AurasVisible => "AURAS_VISIBLE",
            Self::ModResistancePct => "MOD_RESISTANCE_PCT",
            Self::ModMeleeAttackPowerVersus => "MOD_MELEE_ATTACK_POWER_VERSUS",
            Self::ModTotalThreat => "MOD_TOTAL_THREAT",
            Self::WaterWalk => "WATER_WALK",
            Self::FeatherFall => "FEATHER_FALL",
            Self::Hover => "HOVER",
            Self::AddFlatModifier => "ADD_FLAT_MODIFIER",
            Self::AddPctModifier => "ADD_PCT_MODIFIER",
            Self::AddTargetTrigger => "ADD_TARGET_TRIGGER",
            Self::ModPowerRegenPercent => "MOD_POWER_REGEN_PERCENT",
            Self::AddCasterHitTrigger => "ADD_CASTER_HIT_TRIGGER",
            Self::OverrideClassScripts => "OVERRIDE_CLASS_SCRIPTS",
            Self::ModRangedDamageTaken => "MOD_RANGED_DAMAGE_TAKEN",
            Self::ModRangedDamageTakenPct => "MOD_RANGED_DAMAGE_TAKEN_PCT",
            Self::ModHealing => "MOD_HEALING",
            Self::ModRegenDuringCombat => "MOD_REGEN_DURING_COMBAT",
            Self::ModMechanicResistance => "MOD_MECHANIC_RESISTANCE",
            Self::ModHealingPct => "MOD_HEALING_PCT",
            Self::SharePetTracking => "SHARE_PET_TRACKING",
            Self::Untrackable => "UNTRACKABLE",
            Self::Empathy => "EMPATHY",
            Self::ModOffhandDamagePct => "MOD_OFFHAND_DAMAGE_PCT",
            Self::ModTargetResistance => "MOD_TARGET_RESISTANCE",
            Self::ModRangedAttackPower => "MOD_RANGED_ATTACK_POWER",
            Self::ModMeleeDamageTaken => "MOD_MELEE_DAMAGE_TAKEN",
            Self::ModMeleeDamageTakenPct => "MOD_MELEE_DAMAGE_TAKEN_PCT",
            Self::RangedAttackPowerAttackerBonus => "RANGED_ATTACK_POWER_ATTACKER_BONUS",
            Self::ModPossessPet => "MOD_POSSESS_PET",
            Self::ModSpeedAlways => "MOD_SPEED_ALWAYS",
            Self::ModMountedSpeedAlways => "MOD_MOUNTED_SPEED_ALWAYS",
            Self::ModRangedAttackPowerVersus => "MOD_RANGED_ATTACK_POWER_VERSUS",
            Self::ModIncreaseEnergyPercent => "MOD_INCREASE_ENERGY_PERCENT",
            Self::ModIncreaseHealthPercent => "MOD_INCREASE_HEALTH_PERCENT",
            Self::ModManaRegenInterrupt => "MOD_MANA_REGEN_INTERRUPT",
            Self::ModHealingDone => "MOD_HEALING_DONE",
            Self::ModHealingDonePercent => "MOD_HEALING_DONE_PERCENT",
            Self::ModTotalStatPercentage => "MOD_TOTAL_STAT_PERCENTAGE",
            Self::ModMeleeHaste => "MOD_MELEE_HASTE",
            Self::ForceReaction => "FORCE_REACTION",
            Self::ModRangedHaste => "MOD_RANGED_HASTE",
            Self::ModRangedAmmoHaste => "MOD_RANGED_AMMO_HASTE",
            Self::ModBaseResistancePct => "MOD_BASE_RESISTANCE_PCT",
            Self::ModResistanceExclusive => "MOD_RESISTANCE_EXCLUSIVE",
            Self::SafeFall => "SAFE_FALL",
            Self::Charisma => "CHARISMA",
            Self::Persuaded => "PERSUADED",
            Self::MechanicImmunityMask => "MECHANIC_IMMUNITY_MASK",
            Self::RetainComboPoints => "RETAIN_COMBO_POINTS",
            Self::ResistPushback => "RESIST_PUSHBACK",
            Self::ModShieldBlockvaluePct => "MOD_SHIELD_BLOCKVALUE_PCT",
            Self::TrackStealthed => "TRACK_STEALTHED",
            Self::ModDetectedRange => "MOD_DETECTED_RANGE",
            Self::SplitDamageFlat => "SPLIT_DAMAGE_FLAT",
            Self::ModStealthLevel => "MOD_STEALTH_LEVEL",
            Self::ModWaterBreathing => "MOD_WATER_BREATHING",
            Self::ModReputationGain => "MOD_REPUTATION_GAIN",
            Self::PetDamageMulti => "PET_DAMAGE_MULTI",
            Self::ModShieldBlockvalue => "MOD_SHIELD_BLOCKVALUE",
            Self::NoPvpCredit => "NO_PVP_CREDIT",
            Self::ModAoeAvoidance => "MOD_AOE_AVOIDANCE",
            Self::ModHealthRegenInCombat => "MOD_HEALTH_REGEN_IN_COMBAT",
            Self::PowerBurnMana => "POWER_BURN_MANA",
            Self::ModCritDamageBonus => "MOD_CRIT_DAMAGE_BONUS",
            Self::Unknown164 => "UNKNOWN164",
            Self::MeleeAttackPowerAttackerBonus => "MELEE_ATTACK_POWER_ATTACKER_BONUS",
            Self::ModAttackPowerPct => "MOD_ATTACK_POWER_PCT",
            Self::ModRangedAttackPowerPct => "MOD_RANGED_ATTACK_POWER_PCT",
            Self::ModDamageDoneVersus => "MOD_DAMAGE_DONE_VERSUS",
            Self::ModCritPercentVersus => "MOD_CRIT_PERCENT_VERSUS",
            Self::DetectAmore => "DETECT_AMORE",
            Self::ModSpeedNotStack => "MOD_SPEED_NOT_STACK",
            Self::ModMountedSpeedNotStack => "MOD_MOUNTED_SPEED_NOT_STACK",
            Self::AllowChampionSpells => "ALLOW_CHAMPION_SPELLS",
            Self::ModSpellDamageOfStatPercent => "MOD_SPELL_DAMAGE_OF_STAT_PERCENT",
            Self::ModSpellHealingOfStatPercent => "MOD_SPELL_HEALING_OF_STAT_PERCENT",
            Self::SpiritOfRedemption => "SPIRIT_OF_REDEMPTION",
            Self::AoeCharm => "AOE_CHARM",
            Self::ModDebuffResistance => "MOD_DEBUFF_RESISTANCE",
            Self::ModAttackerSpellCritChance => "MOD_ATTACKER_SPELL_CRIT_CHANCE",
            Self::ModFlatSpellDamageVersus => "MOD_FLAT_SPELL_DAMAGE_VERSUS",
            Self::ModFlatSpellCritDamageVersus => "MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS",
            Self::ModResistanceOfStatPercent => "MOD_RESISTANCE_OF_STAT_PERCENT",
            Self::ModCriticalThreat => "MOD_CRITICAL_THREAT",
            Self::ModAttackerMeleeHitChance => "MOD_ATTACKER_MELEE_HIT_CHANCE",
            Self::ModAttackerRangedHitChance => "MOD_ATTACKER_RANGED_HIT_CHANCE",
            Self::ModAttackerSpellHitChance => "MOD_ATTACKER_SPELL_HIT_CHANCE",
            Self::ModAttackerMeleeCritChance => "MOD_ATTACKER_MELEE_CRIT_CHANCE",
            Self::ModAttackerRangedCritChance => "MOD_ATTACKER_RANGED_CRIT_CHANCE",
            Self::ModRating => "MOD_RATING",
            Self::ModFactionReputationGain => "MOD_FACTION_REPUTATION_GAIN",
            Self::UseNormalMovementSpeed => "USE_NORMAL_MOVEMENT_SPEED",
        }
    }

}

const NAME: &str = "AuraType";

impl std::fmt::Display for AuraType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::None => "None",
            Self::BindSight => "BindSight",
            Self::ModPossess => "ModPossess",
            Self::PeriodicDamage => "PeriodicDamage",
            Self::Dummy => "Dummy",
            Self::ModConfuse => "ModConfuse",
            Self::ModCharm => "ModCharm",
            Self::ModFear => "ModFear",
            Self::PeriodicHeal => "PeriodicHeal",
            Self::ModAttackspeed => "ModAttackspeed",
            Self::ModThreat => "ModThreat",
            Self::ModTaunt => "ModTaunt",
            Self::ModStun => "ModStun",
            Self::ModDamageDone => "ModDamageDone",
            Self::ModDamageTaken => "ModDamageTaken",
            Self::DamageShield => "DamageShield",
            Self::ModStealth => "ModStealth",
            Self::ModStealthDetect => "ModStealthDetect",
            Self::ModInvisibility => "ModInvisibility",
            Self::ModInvisibilityDetection => "ModInvisibilityDetection",
            Self::ObsModHealth => "ObsModHealth",
            Self::ObsModMana => "ObsModMana",
            Self::ModResistance => "ModResistance",
            Self::PeriodicTriggerSpell => "PeriodicTriggerSpell",
            Self::PeriodicEnergize => "PeriodicEnergize",
            Self::ModPacify => "ModPacify",
            Self::ModRoot => "ModRoot",
            Self::ModSilence => "ModSilence",
            Self::ReflectSpells => "ReflectSpells",
            Self::ModStat => "ModStat",
            Self::ModSkill => "ModSkill",
            Self::ModIncreaseSpeed => "ModIncreaseSpeed",
            Self::ModIncreaseMountedSpeed => "ModIncreaseMountedSpeed",
            Self::ModDecreaseSpeed => "ModDecreaseSpeed",
            Self::ModIncreaseHealth => "ModIncreaseHealth",
            Self::ModIncreaseEnergy => "ModIncreaseEnergy",
            Self::ModShapeshift => "ModShapeshift",
            Self::EffectImmunity => "EffectImmunity",
            Self::StateImmunity => "StateImmunity",
            Self::SchoolImmunity => "SchoolImmunity",
            Self::DamageImmunity => "DamageImmunity",
            Self::DispelImmunity => "DispelImmunity",
            Self::ProcTriggerSpell => "ProcTriggerSpell",
            Self::ProcTriggerDamage => "ProcTriggerDamage",
            Self::TrackCreatures => "TrackCreatures",
            Self::TrackResources => "TrackResources",
            Self::Unknown46 => "Unknown46",
            Self::ModParryPercent => "ModParryPercent",
            Self::Unknown48 => "Unknown48",
            Self::ModDodgePercent => "ModDodgePercent",
            Self::ModBlockSkill => "ModBlockSkill",
            Self::ModBlockPercent => "ModBlockPercent",
            Self::ModCritPercent => "ModCritPercent",
            Self::PeriodicLeech => "PeriodicLeech",
            Self::ModHitChance => "ModHitChance",
            Self::ModSpellHitChance => "ModSpellHitChance",
            Self::Transform => "Transform",
            Self::ModSpellCritChance => "ModSpellCritChance",
            Self::ModIncreaseSwimSpeed => "ModIncreaseSwimSpeed",
            Self::ModDamageDoneCreature => "ModDamageDoneCreature",
            Self::ModPacifySilence => "ModPacifySilence",
            Self::ModScale => "ModScale",
            Self::PeriodicHealthFunnel => "PeriodicHealthFunnel",
            Self::PeriodicManaFunnel => "PeriodicManaFunnel",
            Self::PeriodicManaLeech => "PeriodicManaLeech",
            Self::ModCastingSpeedNotStack => "ModCastingSpeedNotStack",
            Self::FeignDeath => "FeignDeath",
            Self::ModDisarm => "ModDisarm",
            Self::ModStalked => "ModStalked",
            Self::SchoolAbsorb => "SchoolAbsorb",
            Self::ExtraAttacks => "ExtraAttacks",
            Self::ModSpellCritChanceSchool => "ModSpellCritChanceSchool",
            Self::ModPowerCostSchoolPct => "ModPowerCostSchoolPct",
            Self::ModPowerCostSchool => "ModPowerCostSchool",
            Self::ReflectSpellsSchool => "ReflectSpellsSchool",
            Self::ModLanguage => "ModLanguage",
            Self::FarSight => "FarSight",
            Self::MechanicImmunity => "MechanicImmunity",
            Self::Mounted => "Mounted",
            Self::ModDamagePercentDone => "ModDamagePercentDone",
            Self::ModPercentStat => "ModPercentStat",
            Self::SplitDamagePct => "SplitDamagePct",
            Self::WaterBreathing => "WaterBreathing",
            Self::ModBaseResistance => "ModBaseResistance",
            Self::ModRegen => "ModRegen",
            Self::ModPowerRegen => "ModPowerRegen",
            Self::ChannelDeathItem => "ChannelDeathItem",
            Self::ModDamagePercentTaken => "ModDamagePercentTaken",
            Self::ModHealthRegenPercent => "ModHealthRegenPercent",
            Self::PeriodicDamagePercent => "PeriodicDamagePercent",
            Self::ModResistChance => "ModResistChance",
            Self::ModDetectRange => "ModDetectRange",
            Self::PreventsFleeing => "PreventsFleeing",
            Self::ModUnattackable => "ModUnattackable",
            Self::InterruptRegen => "InterruptRegen",
            Self::Ghost => "Ghost",
            Self::SpellMagnet => "SpellMagnet",
            Self::ManaShield => "ManaShield",
            Self::ModSkillTalent => "ModSkillTalent",
            Self::ModAttackPower => "ModAttackPower",
            Self::AurasVisible => "AurasVisible",
            Self::ModResistancePct => "ModResistancePct",
            Self::ModMeleeAttackPowerVersus => "ModMeleeAttackPowerVersus",
            Self::ModTotalThreat => "ModTotalThreat",
            Self::WaterWalk => "WaterWalk",
            Self::FeatherFall => "FeatherFall",
            Self::Hover => "Hover",
            Self::AddFlatModifier => "AddFlatModifier",
            Self::AddPctModifier => "AddPctModifier",
            Self::AddTargetTrigger => "AddTargetTrigger",
            Self::ModPowerRegenPercent => "ModPowerRegenPercent",
            Self::AddCasterHitTrigger => "AddCasterHitTrigger",
            Self::OverrideClassScripts => "OverrideClassScripts",
            Self::ModRangedDamageTaken => "ModRangedDamageTaken",
            Self::ModRangedDamageTakenPct => "ModRangedDamageTakenPct",
            Self::ModHealing => "ModHealing",
            Self::ModRegenDuringCombat => "ModRegenDuringCombat",
            Self::ModMechanicResistance => "ModMechanicResistance",
            Self::ModHealingPct => "ModHealingPct",
            Self::SharePetTracking => "SharePetTracking",
            Self::Untrackable => "Untrackable",
            Self::Empathy => "Empathy",
            Self::ModOffhandDamagePct => "ModOffhandDamagePct",
            Self::ModTargetResistance => "ModTargetResistance",
            Self::ModRangedAttackPower => "ModRangedAttackPower",
            Self::ModMeleeDamageTaken => "ModMeleeDamageTaken",
            Self::ModMeleeDamageTakenPct => "ModMeleeDamageTakenPct",
            Self::RangedAttackPowerAttackerBonus => "RangedAttackPowerAttackerBonus",
            Self::ModPossessPet => "ModPossessPet",
            Self::ModSpeedAlways => "ModSpeedAlways",
            Self::ModMountedSpeedAlways => "ModMountedSpeedAlways",
            Self::ModRangedAttackPowerVersus => "ModRangedAttackPowerVersus",
            Self::ModIncreaseEnergyPercent => "ModIncreaseEnergyPercent",
            Self::ModIncreaseHealthPercent => "ModIncreaseHealthPercent",
            Self::ModManaRegenInterrupt => "ModManaRegenInterrupt",
            Self::ModHealingDone => "ModHealingDone",
            Self::ModHealingDonePercent => "ModHealingDonePercent",
            Self::ModTotalStatPercentage => "ModTotalStatPercentage",
            Self::ModMeleeHaste => "ModMeleeHaste",
            Self::ForceReaction => "ForceReaction",
            Self::ModRangedHaste => "ModRangedHaste",
            Self::ModRangedAmmoHaste => "ModRangedAmmoHaste",
            Self::ModBaseResistancePct => "ModBaseResistancePct",
            Self::ModResistanceExclusive => "ModResistanceExclusive",
            Self::SafeFall => "SafeFall",
            Self::Charisma => "Charisma",
            Self::Persuaded => "Persuaded",
            Self::MechanicImmunityMask => "MechanicImmunityMask",
            Self::RetainComboPoints => "RetainComboPoints",
            Self::ResistPushback => "ResistPushback",
            Self::ModShieldBlockvaluePct => "ModShieldBlockvaluePct",
            Self::TrackStealthed => "TrackStealthed",
            Self::ModDetectedRange => "ModDetectedRange",
            Self::SplitDamageFlat => "SplitDamageFlat",
            Self::ModStealthLevel => "ModStealthLevel",
            Self::ModWaterBreathing => "ModWaterBreathing",
            Self::ModReputationGain => "ModReputationGain",
            Self::PetDamageMulti => "PetDamageMulti",
            Self::ModShieldBlockvalue => "ModShieldBlockvalue",
            Self::NoPvpCredit => "NoPvpCredit",
            Self::ModAoeAvoidance => "ModAoeAvoidance",
            Self::ModHealthRegenInCombat => "ModHealthRegenInCombat",
            Self::PowerBurnMana => "PowerBurnMana",
            Self::ModCritDamageBonus => "ModCritDamageBonus",
            Self::Unknown164 => "Unknown164",
            Self::MeleeAttackPowerAttackerBonus => "MeleeAttackPowerAttackerBonus",
            Self::ModAttackPowerPct => "ModAttackPowerPct",
            Self::ModRangedAttackPowerPct => "ModRangedAttackPowerPct",
            Self::ModDamageDoneVersus => "ModDamageDoneVersus",
            Self::ModCritPercentVersus => "ModCritPercentVersus",
            Self::DetectAmore => "DetectAmore",
            Self::ModSpeedNotStack => "ModSpeedNotStack",
            Self::ModMountedSpeedNotStack => "ModMountedSpeedNotStack",
            Self::AllowChampionSpells => "AllowChampionSpells",
            Self::ModSpellDamageOfStatPercent => "ModSpellDamageOfStatPercent",
            Self::ModSpellHealingOfStatPercent => "ModSpellHealingOfStatPercent",
            Self::SpiritOfRedemption => "SpiritOfRedemption",
            Self::AoeCharm => "AoeCharm",
            Self::ModDebuffResistance => "ModDebuffResistance",
            Self::ModAttackerSpellCritChance => "ModAttackerSpellCritChance",
            Self::ModFlatSpellDamageVersus => "ModFlatSpellDamageVersus",
            Self::ModFlatSpellCritDamageVersus => "ModFlatSpellCritDamageVersus",
            Self::ModResistanceOfStatPercent => "ModResistanceOfStatPercent",
            Self::ModCriticalThreat => "ModCriticalThreat",
            Self::ModAttackerMeleeHitChance => "ModAttackerMeleeHitChance",
            Self::ModAttackerRangedHitChance => "ModAttackerRangedHitChance",
            Self::ModAttackerSpellHitChance => "ModAttackerSpellHitChance",
            Self::ModAttackerMeleeCritChance => "ModAttackerMeleeCritChance",
            Self::ModAttackerRangedCritChance => "ModAttackerRangedCritChance",
            Self::ModRating => "ModRating",
            Self::ModFactionReputationGain => "ModFactionReputationGain",
            Self::UseNormalMovementSpeed => "UseNormalMovementSpeed",
        })
    }
}

impl TryFrom<u32> for AuraType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for AuraType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u16> for AuraType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u64> for AuraType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for AuraType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for AuraType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for AuraType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i64> for AuraType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for AuraType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

