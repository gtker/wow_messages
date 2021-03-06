use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_periodicauralog.wowm#L3):
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
pub(crate) enum AuraType {
    NONE,
    BIND_SIGHT,
    MOD_POSSESS,
    /// vmangos: The aura should do periodic damage, the function that handles this is Aura::HandlePeriodicDamage, the amount is usually decided by the Unit::SpellDamageBonusDone or Unit::MeleeDamageBonusDone which increases/decreases the Modifier::m_amount
    ///
    PERIODIC_DAMAGE,
    /// vmangos: Used by Aura::HandleAuraDummy
    ///
    DUMMY,
    /// vmangos: Used by Aura::HandleModConfuse, will either confuse or unconfuse the target depending on whether the apply flag is set
    ///
    MOD_CONFUSE,
    MOD_CHARM,
    MOD_FEAR,
    /// vmangos: The aura will do periodic heals of a target, handled by Aura::HandlePeriodicHeal, uses Unit::SpellHealingBonusDone to calculate whether to increase or decrease Modifier::m_amount
    ///
    PERIODIC_HEAL,
    /// vmangos: Changes the attackspeed, the Modifier::m_amount decides how much we change in percent, ie, if the m_amount is 50 the attackspeed will increase by 50%
    ///
    MOD_ATTACKSPEED,
    /// vmangos: Modifies the threat that the Aura does in percent, the Modifier::m_miscvalue decides which of the SpellSchools it should affect threat for.  \see SpellSchoolMask
    ///
    MOD_THREAT,
    /// vmangos: Just applies a taunt which will change the threat a mob has Taken care of in Aura::HandleModThreat
    ///
    MOD_TAUNT,
    /// vmangos: Stuns targets in different ways, taken care of in Aura::HandleAuraModStun
    ///
    MOD_STUN,
    /// vmangos: Changes the damage done by a weapon in any hand, the Modifier::m_miscvalue will tell what school the damage is from, it's used as a bitmask \see SpellSchoolMask
    ///
    MOD_DAMAGE_DONE,
    /// vmangos: Not handled by the Aura class but instead this is implemented in Unit::MeleeDamageBonusTaken and Unit::SpellBaseDamageBonusTaken
    ///
    MOD_DAMAGE_TAKEN,
    /// vmangos: Not handled by the Aura class, implemented in Unit::DealMeleeDamage
    ///
    DAMAGE_SHIELD,
    /// vmangos: Taken care of in Aura::HandleModStealth, take note that this is not the same thing as invisibility
    ///
    MOD_STEALTH,
    /// vmangos: Not handled by the Aura class, implemented in Unit::isVisibleForOrDetect which does a lot of checks to determine whether the person is visible or not, the SPELL_AURA_MOD_STEALTH seems to determine how in/visible ie a rogue is.
    ///
    MOD_STEALTH_DETECT,
    /// vmangos: Handled by Aura::HandleInvisibility, the Modifier::m_miscvalue in the struct seems to decide what kind of invisibility it is with a bitflag. the miscvalue decides which bit is set, ie: 3 would make the 3rd bit be set.
    ///
    MOD_INVISIBILITY,
    /// vmangos: Adds one of the kinds of detections to the possible detections.  As in SPEALL_AURA_MOD_INVISIBILITY the Modifier::m_miscvalue seems to decide what kind of invisibility the Unit should be able to detect.
    ///
    MOD_INVISIBILITY_DETECTION,
    /// 20,21 unofficial
    ///
    OBS_MOD_HEALTH,
    OBS_MOD_MANA,
    /// vmangos: Handled by Aura::HandleAuraModResistance, changes the resistance for a unit the field Modifier::m_miscvalue decides which kind of resistance that should be changed, for possible values see SpellSchools.  \see SpellSchools
    ///
    MOD_RESISTANCE,
    /// vmangos: Currently just sets Aura::m_isPeriodic to apply and has a special case for Curse of the Plaguebringer.
    ///
    PERIODIC_TRIGGER_SPELL,
    /// vmangos: Just sets Aura::m_isPeriodic to apply
    ///
    PERIODIC_ENERGIZE,
    /// vmangos: Changes whether the target is pacified or not depending on the apply flag.  Pacify makes the target silenced and have all it's attack skill disabled.  See: http://classic.wowhead.com/spell=6462
    ///
    MOD_PACIFY,
    /// vmangos: Roots or unroots the target
    ///
    MOD_ROOT,
    /// vmangos: Silences the target and stops and spell casts that should be stopped, they have the flag SpellPreventionType::SPELL_PREVENTION_TYPE_SILENCE
    ///
    MOD_SILENCE,
    REFLECT_SPELLS,
    MOD_STAT,
    MOD_SKILL,
    MOD_INCREASE_SPEED,
    MOD_INCREASE_MOUNTED_SPEED,
    MOD_DECREASE_SPEED,
    MOD_INCREASE_HEALTH,
    MOD_INCREASE_ENERGY,
    MOD_SHAPESHIFT,
    EFFECT_IMMUNITY,
    STATE_IMMUNITY,
    SCHOOL_IMMUNITY,
    DAMAGE_IMMUNITY,
    DISPEL_IMMUNITY,
    PROC_TRIGGER_SPELL,
    PROC_TRIGGER_DAMAGE,
    TRACK_CREATURES,
    TRACK_RESOURCES,
    /// Ignore all Gear test spells
    ///
    UNKNOWN46,
    MOD_PARRY_PERCENT,
    /// One periodic spell
    ///
    UNKNOWN48,
    MOD_DODGE_PERCENT,
    MOD_BLOCK_SKILL,
    MOD_BLOCK_PERCENT,
    MOD_CRIT_PERCENT,
    PERIODIC_LEECH,
    MOD_HIT_CHANCE,
    MOD_SPELL_HIT_CHANCE,
    TRANSFORM,
    MOD_SPELL_CRIT_CHANCE,
    MOD_INCREASE_SWIM_SPEED,
    MOD_DAMAGE_DONE_CREATURE,
    MOD_PACIFY_SILENCE,
    MOD_SCALE,
    PERIODIC_HEALTH_FUNNEL,
    PERIODIC_MANA_FUNNEL,
    PERIODIC_MANA_LEECH,
    MOD_CASTING_SPEED_NOT_STACK,
    FEIGN_DEATH,
    MOD_DISARM,
    MOD_STALKED,
    SCHOOL_ABSORB,
    EXTRA_ATTACKS,
    MOD_SPELL_CRIT_CHANCE_SCHOOL,
    MOD_POWER_COST_SCHOOL_PCT,
    MOD_POWER_COST_SCHOOL,
    REFLECT_SPELLS_SCHOOL,
    MOD_LANGUAGE,
    FAR_SIGHT,
    MECHANIC_IMMUNITY,
    MOUNTED,
    MOD_DAMAGE_PERCENT_DONE,
    MOD_PERCENT_STAT,
    SPLIT_DAMAGE_PCT,
    WATER_BREATHING,
    MOD_BASE_RESISTANCE,
    MOD_REGEN,
    MOD_POWER_REGEN,
    CHANNEL_DEATH_ITEM,
    MOD_DAMAGE_PERCENT_TAKEN,
    MOD_HEALTH_REGEN_PERCENT,
    PERIODIC_DAMAGE_PERCENT,
    MOD_RESIST_CHANCE,
    MOD_DETECT_RANGE,
    PREVENTS_FLEEING,
    MOD_UNATTACKABLE,
    INTERRUPT_REGEN,
    GHOST,
    SPELL_MAGNET,
    MANA_SHIELD,
    MOD_SKILL_TALENT,
    MOD_ATTACK_POWER,
    AURAS_VISIBLE,
    MOD_RESISTANCE_PCT,
    MOD_MELEE_ATTACK_POWER_VERSUS,
    MOD_TOTAL_THREAT,
    WATER_WALK,
    FEATHER_FALL,
    HOVER,
    ADD_FLAT_MODIFIER,
    ADD_PCT_MODIFIER,
    ADD_TARGET_TRIGGER,
    MOD_POWER_REGEN_PERCENT,
    ADD_CASTER_HIT_TRIGGER,
    OVERRIDE_CLASS_SCRIPTS,
    MOD_RANGED_DAMAGE_TAKEN,
    MOD_RANGED_DAMAGE_TAKEN_PCT,
    MOD_HEALING,
    MOD_REGEN_DURING_COMBAT,
    MOD_MECHANIC_RESISTANCE,
    MOD_HEALING_PCT,
    SHARE_PET_TRACKING,
    UNTRACKABLE,
    EMPATHY,
    MOD_OFFHAND_DAMAGE_PCT,
    MOD_TARGET_RESISTANCE,
    MOD_RANGED_ATTACK_POWER,
    MOD_MELEE_DAMAGE_TAKEN,
    MOD_MELEE_DAMAGE_TAKEN_PCT,
    RANGED_ATTACK_POWER_ATTACKER_BONUS,
    MOD_POSSESS_PET,
    MOD_SPEED_ALWAYS,
    MOD_MOUNTED_SPEED_ALWAYS,
    MOD_RANGED_ATTACK_POWER_VERSUS,
    MOD_INCREASE_ENERGY_PERCENT,
    MOD_INCREASE_HEALTH_PERCENT,
    MOD_MANA_REGEN_INTERRUPT,
    MOD_HEALING_DONE,
    MOD_HEALING_DONE_PERCENT,
    MOD_TOTAL_STAT_PERCENTAGE,
    MOD_MELEE_HASTE,
    FORCE_REACTION,
    MOD_RANGED_HASTE,
    MOD_RANGED_AMMO_HASTE,
    MOD_BASE_RESISTANCE_PCT,
    MOD_RESISTANCE_EXCLUSIVE,
    SAFE_FALL,
    CHARISMA,
    PERSUADED,
    MECHANIC_IMMUNITY_MASK,
    RETAIN_COMBO_POINTS,
    /// Resist Pushback
    ///
    RESIST_PUSHBACK,
    MOD_SHIELD_BLOCKVALUE_PCT,
    /// Track Stealthed
    ///
    TRACK_STEALTHED,
    /// Mod Detected Range
    ///
    MOD_DETECTED_RANGE,
    /// Split Damage Flat
    ///
    SPLIT_DAMAGE_FLAT,
    /// Stealth Level Modifier
    ///
    MOD_STEALTH_LEVEL,
    /// Mod Water Breathing
    ///
    MOD_WATER_BREATHING,
    /// Mod Reputation Gain
    ///
    MOD_REPUTATION_GAIN,
    /// Mod Pet Damage
    ///
    PET_DAMAGE_MULTI,
    MOD_SHIELD_BLOCKVALUE,
    NO_PVP_CREDIT,
    MOD_AOE_AVOIDANCE,
    MOD_HEALTH_REGEN_IN_COMBAT,
    POWER_BURN_MANA,
    MOD_CRIT_DAMAGE_BONUS,
    UNKNOWN164,
    MELEE_ATTACK_POWER_ATTACKER_BONUS,
    MOD_ATTACK_POWER_PCT,
    MOD_RANGED_ATTACK_POWER_PCT,
    MOD_DAMAGE_DONE_VERSUS,
    MOD_CRIT_PERCENT_VERSUS,
    DETECT_AMORE,
    MOD_SPEED_NOT_STACK,
    MOD_MOUNTED_SPEED_NOT_STACK,
    ALLOW_CHAMPION_SPELLS,
    /// in 1.12.1 only dependent spirit case
    ///
    MOD_SPELL_DAMAGE_OF_STAT_PERCENT,
    MOD_SPELL_HEALING_OF_STAT_PERCENT,
    SPIRIT_OF_REDEMPTION,
    AOE_CHARM,
    MOD_DEBUFF_RESISTANCE,
    MOD_ATTACKER_SPELL_CRIT_CHANCE,
    MOD_FLAT_SPELL_DAMAGE_VERSUS,
    /// unused - possible flat spell crit damage versus
    ///
    MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS,
    MOD_RESISTANCE_OF_STAT_PERCENT,
    MOD_CRITICAL_THREAT,
    MOD_ATTACKER_MELEE_HIT_CHANCE,
    MOD_ATTACKER_RANGED_HIT_CHANCE,
    MOD_ATTACKER_SPELL_HIT_CHANCE,
    MOD_ATTACKER_MELEE_CRIT_CHANCE,
    MOD_ATTACKER_RANGED_CRIT_CHANCE,
    MOD_RATING,
    MOD_FACTION_REPUTATION_GAIN,
    USE_NORMAL_MOVEMENT_SPEED,
}

impl AuraType {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::NONE => 0x0,
            Self::BIND_SIGHT => 0x1,
            Self::MOD_POSSESS => 0x2,
            Self::PERIODIC_DAMAGE => 0x3,
            Self::DUMMY => 0x4,
            Self::MOD_CONFUSE => 0x5,
            Self::MOD_CHARM => 0x6,
            Self::MOD_FEAR => 0x7,
            Self::PERIODIC_HEAL => 0x8,
            Self::MOD_ATTACKSPEED => 0x9,
            Self::MOD_THREAT => 0xa,
            Self::MOD_TAUNT => 0xb,
            Self::MOD_STUN => 0xc,
            Self::MOD_DAMAGE_DONE => 0xd,
            Self::MOD_DAMAGE_TAKEN => 0xe,
            Self::DAMAGE_SHIELD => 0xf,
            Self::MOD_STEALTH => 0x10,
            Self::MOD_STEALTH_DETECT => 0x11,
            Self::MOD_INVISIBILITY => 0x12,
            Self::MOD_INVISIBILITY_DETECTION => 0x13,
            Self::OBS_MOD_HEALTH => 0x14,
            Self::OBS_MOD_MANA => 0x15,
            Self::MOD_RESISTANCE => 0x16,
            Self::PERIODIC_TRIGGER_SPELL => 0x17,
            Self::PERIODIC_ENERGIZE => 0x18,
            Self::MOD_PACIFY => 0x19,
            Self::MOD_ROOT => 0x1a,
            Self::MOD_SILENCE => 0x1b,
            Self::REFLECT_SPELLS => 0x1c,
            Self::MOD_STAT => 0x1d,
            Self::MOD_SKILL => 0x1e,
            Self::MOD_INCREASE_SPEED => 0x1f,
            Self::MOD_INCREASE_MOUNTED_SPEED => 0x20,
            Self::MOD_DECREASE_SPEED => 0x21,
            Self::MOD_INCREASE_HEALTH => 0x22,
            Self::MOD_INCREASE_ENERGY => 0x23,
            Self::MOD_SHAPESHIFT => 0x24,
            Self::EFFECT_IMMUNITY => 0x25,
            Self::STATE_IMMUNITY => 0x26,
            Self::SCHOOL_IMMUNITY => 0x27,
            Self::DAMAGE_IMMUNITY => 0x28,
            Self::DISPEL_IMMUNITY => 0x29,
            Self::PROC_TRIGGER_SPELL => 0x2a,
            Self::PROC_TRIGGER_DAMAGE => 0x2b,
            Self::TRACK_CREATURES => 0x2c,
            Self::TRACK_RESOURCES => 0x2d,
            Self::UNKNOWN46 => 0x2e,
            Self::MOD_PARRY_PERCENT => 0x2f,
            Self::UNKNOWN48 => 0x30,
            Self::MOD_DODGE_PERCENT => 0x31,
            Self::MOD_BLOCK_SKILL => 0x32,
            Self::MOD_BLOCK_PERCENT => 0x33,
            Self::MOD_CRIT_PERCENT => 0x34,
            Self::PERIODIC_LEECH => 0x35,
            Self::MOD_HIT_CHANCE => 0x36,
            Self::MOD_SPELL_HIT_CHANCE => 0x37,
            Self::TRANSFORM => 0x38,
            Self::MOD_SPELL_CRIT_CHANCE => 0x39,
            Self::MOD_INCREASE_SWIM_SPEED => 0x3a,
            Self::MOD_DAMAGE_DONE_CREATURE => 0x3b,
            Self::MOD_PACIFY_SILENCE => 0x3c,
            Self::MOD_SCALE => 0x3d,
            Self::PERIODIC_HEALTH_FUNNEL => 0x3e,
            Self::PERIODIC_MANA_FUNNEL => 0x3f,
            Self::PERIODIC_MANA_LEECH => 0x40,
            Self::MOD_CASTING_SPEED_NOT_STACK => 0x41,
            Self::FEIGN_DEATH => 0x42,
            Self::MOD_DISARM => 0x43,
            Self::MOD_STALKED => 0x44,
            Self::SCHOOL_ABSORB => 0x45,
            Self::EXTRA_ATTACKS => 0x46,
            Self::MOD_SPELL_CRIT_CHANCE_SCHOOL => 0x47,
            Self::MOD_POWER_COST_SCHOOL_PCT => 0x48,
            Self::MOD_POWER_COST_SCHOOL => 0x49,
            Self::REFLECT_SPELLS_SCHOOL => 0x4a,
            Self::MOD_LANGUAGE => 0x4b,
            Self::FAR_SIGHT => 0x4c,
            Self::MECHANIC_IMMUNITY => 0x4d,
            Self::MOUNTED => 0x4e,
            Self::MOD_DAMAGE_PERCENT_DONE => 0x4f,
            Self::MOD_PERCENT_STAT => 0x50,
            Self::SPLIT_DAMAGE_PCT => 0x51,
            Self::WATER_BREATHING => 0x52,
            Self::MOD_BASE_RESISTANCE => 0x53,
            Self::MOD_REGEN => 0x54,
            Self::MOD_POWER_REGEN => 0x55,
            Self::CHANNEL_DEATH_ITEM => 0x56,
            Self::MOD_DAMAGE_PERCENT_TAKEN => 0x57,
            Self::MOD_HEALTH_REGEN_PERCENT => 0x58,
            Self::PERIODIC_DAMAGE_PERCENT => 0x59,
            Self::MOD_RESIST_CHANCE => 0x5a,
            Self::MOD_DETECT_RANGE => 0x5b,
            Self::PREVENTS_FLEEING => 0x5c,
            Self::MOD_UNATTACKABLE => 0x5d,
            Self::INTERRUPT_REGEN => 0x5e,
            Self::GHOST => 0x5f,
            Self::SPELL_MAGNET => 0x60,
            Self::MANA_SHIELD => 0x61,
            Self::MOD_SKILL_TALENT => 0x62,
            Self::MOD_ATTACK_POWER => 0x63,
            Self::AURAS_VISIBLE => 0x64,
            Self::MOD_RESISTANCE_PCT => 0x65,
            Self::MOD_MELEE_ATTACK_POWER_VERSUS => 0x66,
            Self::MOD_TOTAL_THREAT => 0x67,
            Self::WATER_WALK => 0x68,
            Self::FEATHER_FALL => 0x69,
            Self::HOVER => 0x6a,
            Self::ADD_FLAT_MODIFIER => 0x6b,
            Self::ADD_PCT_MODIFIER => 0x6c,
            Self::ADD_TARGET_TRIGGER => 0x6d,
            Self::MOD_POWER_REGEN_PERCENT => 0x6e,
            Self::ADD_CASTER_HIT_TRIGGER => 0x6f,
            Self::OVERRIDE_CLASS_SCRIPTS => 0x70,
            Self::MOD_RANGED_DAMAGE_TAKEN => 0x71,
            Self::MOD_RANGED_DAMAGE_TAKEN_PCT => 0x72,
            Self::MOD_HEALING => 0x73,
            Self::MOD_REGEN_DURING_COMBAT => 0x74,
            Self::MOD_MECHANIC_RESISTANCE => 0x75,
            Self::MOD_HEALING_PCT => 0x76,
            Self::SHARE_PET_TRACKING => 0x77,
            Self::UNTRACKABLE => 0x78,
            Self::EMPATHY => 0x79,
            Self::MOD_OFFHAND_DAMAGE_PCT => 0x7a,
            Self::MOD_TARGET_RESISTANCE => 0x7b,
            Self::MOD_RANGED_ATTACK_POWER => 0x7c,
            Self::MOD_MELEE_DAMAGE_TAKEN => 0x7d,
            Self::MOD_MELEE_DAMAGE_TAKEN_PCT => 0x7e,
            Self::RANGED_ATTACK_POWER_ATTACKER_BONUS => 0x7f,
            Self::MOD_POSSESS_PET => 0x80,
            Self::MOD_SPEED_ALWAYS => 0x81,
            Self::MOD_MOUNTED_SPEED_ALWAYS => 0x82,
            Self::MOD_RANGED_ATTACK_POWER_VERSUS => 0x83,
            Self::MOD_INCREASE_ENERGY_PERCENT => 0x84,
            Self::MOD_INCREASE_HEALTH_PERCENT => 0x85,
            Self::MOD_MANA_REGEN_INTERRUPT => 0x86,
            Self::MOD_HEALING_DONE => 0x87,
            Self::MOD_HEALING_DONE_PERCENT => 0x88,
            Self::MOD_TOTAL_STAT_PERCENTAGE => 0x89,
            Self::MOD_MELEE_HASTE => 0x8a,
            Self::FORCE_REACTION => 0x8b,
            Self::MOD_RANGED_HASTE => 0x8c,
            Self::MOD_RANGED_AMMO_HASTE => 0x8d,
            Self::MOD_BASE_RESISTANCE_PCT => 0x8e,
            Self::MOD_RESISTANCE_EXCLUSIVE => 0x8f,
            Self::SAFE_FALL => 0x90,
            Self::CHARISMA => 0x91,
            Self::PERSUADED => 0x92,
            Self::MECHANIC_IMMUNITY_MASK => 0x93,
            Self::RETAIN_COMBO_POINTS => 0x94,
            Self::RESIST_PUSHBACK => 0x95,
            Self::MOD_SHIELD_BLOCKVALUE_PCT => 0x96,
            Self::TRACK_STEALTHED => 0x97,
            Self::MOD_DETECTED_RANGE => 0x98,
            Self::SPLIT_DAMAGE_FLAT => 0x99,
            Self::MOD_STEALTH_LEVEL => 0x9a,
            Self::MOD_WATER_BREATHING => 0x9b,
            Self::MOD_REPUTATION_GAIN => 0x9c,
            Self::PET_DAMAGE_MULTI => 0x9d,
            Self::MOD_SHIELD_BLOCKVALUE => 0x9e,
            Self::NO_PVP_CREDIT => 0x9f,
            Self::MOD_AOE_AVOIDANCE => 0xa0,
            Self::MOD_HEALTH_REGEN_IN_COMBAT => 0xa1,
            Self::POWER_BURN_MANA => 0xa2,
            Self::MOD_CRIT_DAMAGE_BONUS => 0xa3,
            Self::UNKNOWN164 => 0xa4,
            Self::MELEE_ATTACK_POWER_ATTACKER_BONUS => 0xa5,
            Self::MOD_ATTACK_POWER_PCT => 0xa6,
            Self::MOD_RANGED_ATTACK_POWER_PCT => 0xa7,
            Self::MOD_DAMAGE_DONE_VERSUS => 0xa8,
            Self::MOD_CRIT_PERCENT_VERSUS => 0xa9,
            Self::DETECT_AMORE => 0xaa,
            Self::MOD_SPEED_NOT_STACK => 0xab,
            Self::MOD_MOUNTED_SPEED_NOT_STACK => 0xac,
            Self::ALLOW_CHAMPION_SPELLS => 0xad,
            Self::MOD_SPELL_DAMAGE_OF_STAT_PERCENT => 0xae,
            Self::MOD_SPELL_HEALING_OF_STAT_PERCENT => 0xaf,
            Self::SPIRIT_OF_REDEMPTION => 0xb0,
            Self::AOE_CHARM => 0xb1,
            Self::MOD_DEBUFF_RESISTANCE => 0xb2,
            Self::MOD_ATTACKER_SPELL_CRIT_CHANCE => 0xb3,
            Self::MOD_FLAT_SPELL_DAMAGE_VERSUS => 0xb4,
            Self::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS => 0xb5,
            Self::MOD_RESISTANCE_OF_STAT_PERCENT => 0xb6,
            Self::MOD_CRITICAL_THREAT => 0xb7,
            Self::MOD_ATTACKER_MELEE_HIT_CHANCE => 0xb8,
            Self::MOD_ATTACKER_RANGED_HIT_CHANCE => 0xb9,
            Self::MOD_ATTACKER_SPELL_HIT_CHANCE => 0xba,
            Self::MOD_ATTACKER_MELEE_CRIT_CHANCE => 0xbb,
            Self::MOD_ATTACKER_RANGED_CRIT_CHANCE => 0xbc,
            Self::MOD_RATING => 0xbd,
            Self::MOD_FACTION_REPUTATION_GAIN => 0xbe,
            Self::USE_NORMAL_MOVEMENT_SPEED => 0xbf,
        }
    }

}

impl Default for AuraType {
    fn default() -> Self {
        Self::NONE
    }
}

impl std::fmt::Display for AuraType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NONE => f.write_str("NONE"),
            Self::BIND_SIGHT => f.write_str("BIND_SIGHT"),
            Self::MOD_POSSESS => f.write_str("MOD_POSSESS"),
            Self::PERIODIC_DAMAGE => f.write_str("PERIODIC_DAMAGE"),
            Self::DUMMY => f.write_str("DUMMY"),
            Self::MOD_CONFUSE => f.write_str("MOD_CONFUSE"),
            Self::MOD_CHARM => f.write_str("MOD_CHARM"),
            Self::MOD_FEAR => f.write_str("MOD_FEAR"),
            Self::PERIODIC_HEAL => f.write_str("PERIODIC_HEAL"),
            Self::MOD_ATTACKSPEED => f.write_str("MOD_ATTACKSPEED"),
            Self::MOD_THREAT => f.write_str("MOD_THREAT"),
            Self::MOD_TAUNT => f.write_str("MOD_TAUNT"),
            Self::MOD_STUN => f.write_str("MOD_STUN"),
            Self::MOD_DAMAGE_DONE => f.write_str("MOD_DAMAGE_DONE"),
            Self::MOD_DAMAGE_TAKEN => f.write_str("MOD_DAMAGE_TAKEN"),
            Self::DAMAGE_SHIELD => f.write_str("DAMAGE_SHIELD"),
            Self::MOD_STEALTH => f.write_str("MOD_STEALTH"),
            Self::MOD_STEALTH_DETECT => f.write_str("MOD_STEALTH_DETECT"),
            Self::MOD_INVISIBILITY => f.write_str("MOD_INVISIBILITY"),
            Self::MOD_INVISIBILITY_DETECTION => f.write_str("MOD_INVISIBILITY_DETECTION"),
            Self::OBS_MOD_HEALTH => f.write_str("OBS_MOD_HEALTH"),
            Self::OBS_MOD_MANA => f.write_str("OBS_MOD_MANA"),
            Self::MOD_RESISTANCE => f.write_str("MOD_RESISTANCE"),
            Self::PERIODIC_TRIGGER_SPELL => f.write_str("PERIODIC_TRIGGER_SPELL"),
            Self::PERIODIC_ENERGIZE => f.write_str("PERIODIC_ENERGIZE"),
            Self::MOD_PACIFY => f.write_str("MOD_PACIFY"),
            Self::MOD_ROOT => f.write_str("MOD_ROOT"),
            Self::MOD_SILENCE => f.write_str("MOD_SILENCE"),
            Self::REFLECT_SPELLS => f.write_str("REFLECT_SPELLS"),
            Self::MOD_STAT => f.write_str("MOD_STAT"),
            Self::MOD_SKILL => f.write_str("MOD_SKILL"),
            Self::MOD_INCREASE_SPEED => f.write_str("MOD_INCREASE_SPEED"),
            Self::MOD_INCREASE_MOUNTED_SPEED => f.write_str("MOD_INCREASE_MOUNTED_SPEED"),
            Self::MOD_DECREASE_SPEED => f.write_str("MOD_DECREASE_SPEED"),
            Self::MOD_INCREASE_HEALTH => f.write_str("MOD_INCREASE_HEALTH"),
            Self::MOD_INCREASE_ENERGY => f.write_str("MOD_INCREASE_ENERGY"),
            Self::MOD_SHAPESHIFT => f.write_str("MOD_SHAPESHIFT"),
            Self::EFFECT_IMMUNITY => f.write_str("EFFECT_IMMUNITY"),
            Self::STATE_IMMUNITY => f.write_str("STATE_IMMUNITY"),
            Self::SCHOOL_IMMUNITY => f.write_str("SCHOOL_IMMUNITY"),
            Self::DAMAGE_IMMUNITY => f.write_str("DAMAGE_IMMUNITY"),
            Self::DISPEL_IMMUNITY => f.write_str("DISPEL_IMMUNITY"),
            Self::PROC_TRIGGER_SPELL => f.write_str("PROC_TRIGGER_SPELL"),
            Self::PROC_TRIGGER_DAMAGE => f.write_str("PROC_TRIGGER_DAMAGE"),
            Self::TRACK_CREATURES => f.write_str("TRACK_CREATURES"),
            Self::TRACK_RESOURCES => f.write_str("TRACK_RESOURCES"),
            Self::UNKNOWN46 => f.write_str("UNKNOWN46"),
            Self::MOD_PARRY_PERCENT => f.write_str("MOD_PARRY_PERCENT"),
            Self::UNKNOWN48 => f.write_str("UNKNOWN48"),
            Self::MOD_DODGE_PERCENT => f.write_str("MOD_DODGE_PERCENT"),
            Self::MOD_BLOCK_SKILL => f.write_str("MOD_BLOCK_SKILL"),
            Self::MOD_BLOCK_PERCENT => f.write_str("MOD_BLOCK_PERCENT"),
            Self::MOD_CRIT_PERCENT => f.write_str("MOD_CRIT_PERCENT"),
            Self::PERIODIC_LEECH => f.write_str("PERIODIC_LEECH"),
            Self::MOD_HIT_CHANCE => f.write_str("MOD_HIT_CHANCE"),
            Self::MOD_SPELL_HIT_CHANCE => f.write_str("MOD_SPELL_HIT_CHANCE"),
            Self::TRANSFORM => f.write_str("TRANSFORM"),
            Self::MOD_SPELL_CRIT_CHANCE => f.write_str("MOD_SPELL_CRIT_CHANCE"),
            Self::MOD_INCREASE_SWIM_SPEED => f.write_str("MOD_INCREASE_SWIM_SPEED"),
            Self::MOD_DAMAGE_DONE_CREATURE => f.write_str("MOD_DAMAGE_DONE_CREATURE"),
            Self::MOD_PACIFY_SILENCE => f.write_str("MOD_PACIFY_SILENCE"),
            Self::MOD_SCALE => f.write_str("MOD_SCALE"),
            Self::PERIODIC_HEALTH_FUNNEL => f.write_str("PERIODIC_HEALTH_FUNNEL"),
            Self::PERIODIC_MANA_FUNNEL => f.write_str("PERIODIC_MANA_FUNNEL"),
            Self::PERIODIC_MANA_LEECH => f.write_str("PERIODIC_MANA_LEECH"),
            Self::MOD_CASTING_SPEED_NOT_STACK => f.write_str("MOD_CASTING_SPEED_NOT_STACK"),
            Self::FEIGN_DEATH => f.write_str("FEIGN_DEATH"),
            Self::MOD_DISARM => f.write_str("MOD_DISARM"),
            Self::MOD_STALKED => f.write_str("MOD_STALKED"),
            Self::SCHOOL_ABSORB => f.write_str("SCHOOL_ABSORB"),
            Self::EXTRA_ATTACKS => f.write_str("EXTRA_ATTACKS"),
            Self::MOD_SPELL_CRIT_CHANCE_SCHOOL => f.write_str("MOD_SPELL_CRIT_CHANCE_SCHOOL"),
            Self::MOD_POWER_COST_SCHOOL_PCT => f.write_str("MOD_POWER_COST_SCHOOL_PCT"),
            Self::MOD_POWER_COST_SCHOOL => f.write_str("MOD_POWER_COST_SCHOOL"),
            Self::REFLECT_SPELLS_SCHOOL => f.write_str("REFLECT_SPELLS_SCHOOL"),
            Self::MOD_LANGUAGE => f.write_str("MOD_LANGUAGE"),
            Self::FAR_SIGHT => f.write_str("FAR_SIGHT"),
            Self::MECHANIC_IMMUNITY => f.write_str("MECHANIC_IMMUNITY"),
            Self::MOUNTED => f.write_str("MOUNTED"),
            Self::MOD_DAMAGE_PERCENT_DONE => f.write_str("MOD_DAMAGE_PERCENT_DONE"),
            Self::MOD_PERCENT_STAT => f.write_str("MOD_PERCENT_STAT"),
            Self::SPLIT_DAMAGE_PCT => f.write_str("SPLIT_DAMAGE_PCT"),
            Self::WATER_BREATHING => f.write_str("WATER_BREATHING"),
            Self::MOD_BASE_RESISTANCE => f.write_str("MOD_BASE_RESISTANCE"),
            Self::MOD_REGEN => f.write_str("MOD_REGEN"),
            Self::MOD_POWER_REGEN => f.write_str("MOD_POWER_REGEN"),
            Self::CHANNEL_DEATH_ITEM => f.write_str("CHANNEL_DEATH_ITEM"),
            Self::MOD_DAMAGE_PERCENT_TAKEN => f.write_str("MOD_DAMAGE_PERCENT_TAKEN"),
            Self::MOD_HEALTH_REGEN_PERCENT => f.write_str("MOD_HEALTH_REGEN_PERCENT"),
            Self::PERIODIC_DAMAGE_PERCENT => f.write_str("PERIODIC_DAMAGE_PERCENT"),
            Self::MOD_RESIST_CHANCE => f.write_str("MOD_RESIST_CHANCE"),
            Self::MOD_DETECT_RANGE => f.write_str("MOD_DETECT_RANGE"),
            Self::PREVENTS_FLEEING => f.write_str("PREVENTS_FLEEING"),
            Self::MOD_UNATTACKABLE => f.write_str("MOD_UNATTACKABLE"),
            Self::INTERRUPT_REGEN => f.write_str("INTERRUPT_REGEN"),
            Self::GHOST => f.write_str("GHOST"),
            Self::SPELL_MAGNET => f.write_str("SPELL_MAGNET"),
            Self::MANA_SHIELD => f.write_str("MANA_SHIELD"),
            Self::MOD_SKILL_TALENT => f.write_str("MOD_SKILL_TALENT"),
            Self::MOD_ATTACK_POWER => f.write_str("MOD_ATTACK_POWER"),
            Self::AURAS_VISIBLE => f.write_str("AURAS_VISIBLE"),
            Self::MOD_RESISTANCE_PCT => f.write_str("MOD_RESISTANCE_PCT"),
            Self::MOD_MELEE_ATTACK_POWER_VERSUS => f.write_str("MOD_MELEE_ATTACK_POWER_VERSUS"),
            Self::MOD_TOTAL_THREAT => f.write_str("MOD_TOTAL_THREAT"),
            Self::WATER_WALK => f.write_str("WATER_WALK"),
            Self::FEATHER_FALL => f.write_str("FEATHER_FALL"),
            Self::HOVER => f.write_str("HOVER"),
            Self::ADD_FLAT_MODIFIER => f.write_str("ADD_FLAT_MODIFIER"),
            Self::ADD_PCT_MODIFIER => f.write_str("ADD_PCT_MODIFIER"),
            Self::ADD_TARGET_TRIGGER => f.write_str("ADD_TARGET_TRIGGER"),
            Self::MOD_POWER_REGEN_PERCENT => f.write_str("MOD_POWER_REGEN_PERCENT"),
            Self::ADD_CASTER_HIT_TRIGGER => f.write_str("ADD_CASTER_HIT_TRIGGER"),
            Self::OVERRIDE_CLASS_SCRIPTS => f.write_str("OVERRIDE_CLASS_SCRIPTS"),
            Self::MOD_RANGED_DAMAGE_TAKEN => f.write_str("MOD_RANGED_DAMAGE_TAKEN"),
            Self::MOD_RANGED_DAMAGE_TAKEN_PCT => f.write_str("MOD_RANGED_DAMAGE_TAKEN_PCT"),
            Self::MOD_HEALING => f.write_str("MOD_HEALING"),
            Self::MOD_REGEN_DURING_COMBAT => f.write_str("MOD_REGEN_DURING_COMBAT"),
            Self::MOD_MECHANIC_RESISTANCE => f.write_str("MOD_MECHANIC_RESISTANCE"),
            Self::MOD_HEALING_PCT => f.write_str("MOD_HEALING_PCT"),
            Self::SHARE_PET_TRACKING => f.write_str("SHARE_PET_TRACKING"),
            Self::UNTRACKABLE => f.write_str("UNTRACKABLE"),
            Self::EMPATHY => f.write_str("EMPATHY"),
            Self::MOD_OFFHAND_DAMAGE_PCT => f.write_str("MOD_OFFHAND_DAMAGE_PCT"),
            Self::MOD_TARGET_RESISTANCE => f.write_str("MOD_TARGET_RESISTANCE"),
            Self::MOD_RANGED_ATTACK_POWER => f.write_str("MOD_RANGED_ATTACK_POWER"),
            Self::MOD_MELEE_DAMAGE_TAKEN => f.write_str("MOD_MELEE_DAMAGE_TAKEN"),
            Self::MOD_MELEE_DAMAGE_TAKEN_PCT => f.write_str("MOD_MELEE_DAMAGE_TAKEN_PCT"),
            Self::RANGED_ATTACK_POWER_ATTACKER_BONUS => f.write_str("RANGED_ATTACK_POWER_ATTACKER_BONUS"),
            Self::MOD_POSSESS_PET => f.write_str("MOD_POSSESS_PET"),
            Self::MOD_SPEED_ALWAYS => f.write_str("MOD_SPEED_ALWAYS"),
            Self::MOD_MOUNTED_SPEED_ALWAYS => f.write_str("MOD_MOUNTED_SPEED_ALWAYS"),
            Self::MOD_RANGED_ATTACK_POWER_VERSUS => f.write_str("MOD_RANGED_ATTACK_POWER_VERSUS"),
            Self::MOD_INCREASE_ENERGY_PERCENT => f.write_str("MOD_INCREASE_ENERGY_PERCENT"),
            Self::MOD_INCREASE_HEALTH_PERCENT => f.write_str("MOD_INCREASE_HEALTH_PERCENT"),
            Self::MOD_MANA_REGEN_INTERRUPT => f.write_str("MOD_MANA_REGEN_INTERRUPT"),
            Self::MOD_HEALING_DONE => f.write_str("MOD_HEALING_DONE"),
            Self::MOD_HEALING_DONE_PERCENT => f.write_str("MOD_HEALING_DONE_PERCENT"),
            Self::MOD_TOTAL_STAT_PERCENTAGE => f.write_str("MOD_TOTAL_STAT_PERCENTAGE"),
            Self::MOD_MELEE_HASTE => f.write_str("MOD_MELEE_HASTE"),
            Self::FORCE_REACTION => f.write_str("FORCE_REACTION"),
            Self::MOD_RANGED_HASTE => f.write_str("MOD_RANGED_HASTE"),
            Self::MOD_RANGED_AMMO_HASTE => f.write_str("MOD_RANGED_AMMO_HASTE"),
            Self::MOD_BASE_RESISTANCE_PCT => f.write_str("MOD_BASE_RESISTANCE_PCT"),
            Self::MOD_RESISTANCE_EXCLUSIVE => f.write_str("MOD_RESISTANCE_EXCLUSIVE"),
            Self::SAFE_FALL => f.write_str("SAFE_FALL"),
            Self::CHARISMA => f.write_str("CHARISMA"),
            Self::PERSUADED => f.write_str("PERSUADED"),
            Self::MECHANIC_IMMUNITY_MASK => f.write_str("MECHANIC_IMMUNITY_MASK"),
            Self::RETAIN_COMBO_POINTS => f.write_str("RETAIN_COMBO_POINTS"),
            Self::RESIST_PUSHBACK => f.write_str("RESIST_PUSHBACK"),
            Self::MOD_SHIELD_BLOCKVALUE_PCT => f.write_str("MOD_SHIELD_BLOCKVALUE_PCT"),
            Self::TRACK_STEALTHED => f.write_str("TRACK_STEALTHED"),
            Self::MOD_DETECTED_RANGE => f.write_str("MOD_DETECTED_RANGE"),
            Self::SPLIT_DAMAGE_FLAT => f.write_str("SPLIT_DAMAGE_FLAT"),
            Self::MOD_STEALTH_LEVEL => f.write_str("MOD_STEALTH_LEVEL"),
            Self::MOD_WATER_BREATHING => f.write_str("MOD_WATER_BREATHING"),
            Self::MOD_REPUTATION_GAIN => f.write_str("MOD_REPUTATION_GAIN"),
            Self::PET_DAMAGE_MULTI => f.write_str("PET_DAMAGE_MULTI"),
            Self::MOD_SHIELD_BLOCKVALUE => f.write_str("MOD_SHIELD_BLOCKVALUE"),
            Self::NO_PVP_CREDIT => f.write_str("NO_PVP_CREDIT"),
            Self::MOD_AOE_AVOIDANCE => f.write_str("MOD_AOE_AVOIDANCE"),
            Self::MOD_HEALTH_REGEN_IN_COMBAT => f.write_str("MOD_HEALTH_REGEN_IN_COMBAT"),
            Self::POWER_BURN_MANA => f.write_str("POWER_BURN_MANA"),
            Self::MOD_CRIT_DAMAGE_BONUS => f.write_str("MOD_CRIT_DAMAGE_BONUS"),
            Self::UNKNOWN164 => f.write_str("UNKNOWN164"),
            Self::MELEE_ATTACK_POWER_ATTACKER_BONUS => f.write_str("MELEE_ATTACK_POWER_ATTACKER_BONUS"),
            Self::MOD_ATTACK_POWER_PCT => f.write_str("MOD_ATTACK_POWER_PCT"),
            Self::MOD_RANGED_ATTACK_POWER_PCT => f.write_str("MOD_RANGED_ATTACK_POWER_PCT"),
            Self::MOD_DAMAGE_DONE_VERSUS => f.write_str("MOD_DAMAGE_DONE_VERSUS"),
            Self::MOD_CRIT_PERCENT_VERSUS => f.write_str("MOD_CRIT_PERCENT_VERSUS"),
            Self::DETECT_AMORE => f.write_str("DETECT_AMORE"),
            Self::MOD_SPEED_NOT_STACK => f.write_str("MOD_SPEED_NOT_STACK"),
            Self::MOD_MOUNTED_SPEED_NOT_STACK => f.write_str("MOD_MOUNTED_SPEED_NOT_STACK"),
            Self::ALLOW_CHAMPION_SPELLS => f.write_str("ALLOW_CHAMPION_SPELLS"),
            Self::MOD_SPELL_DAMAGE_OF_STAT_PERCENT => f.write_str("MOD_SPELL_DAMAGE_OF_STAT_PERCENT"),
            Self::MOD_SPELL_HEALING_OF_STAT_PERCENT => f.write_str("MOD_SPELL_HEALING_OF_STAT_PERCENT"),
            Self::SPIRIT_OF_REDEMPTION => f.write_str("SPIRIT_OF_REDEMPTION"),
            Self::AOE_CHARM => f.write_str("AOE_CHARM"),
            Self::MOD_DEBUFF_RESISTANCE => f.write_str("MOD_DEBUFF_RESISTANCE"),
            Self::MOD_ATTACKER_SPELL_CRIT_CHANCE => f.write_str("MOD_ATTACKER_SPELL_CRIT_CHANCE"),
            Self::MOD_FLAT_SPELL_DAMAGE_VERSUS => f.write_str("MOD_FLAT_SPELL_DAMAGE_VERSUS"),
            Self::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS => f.write_str("MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS"),
            Self::MOD_RESISTANCE_OF_STAT_PERCENT => f.write_str("MOD_RESISTANCE_OF_STAT_PERCENT"),
            Self::MOD_CRITICAL_THREAT => f.write_str("MOD_CRITICAL_THREAT"),
            Self::MOD_ATTACKER_MELEE_HIT_CHANCE => f.write_str("MOD_ATTACKER_MELEE_HIT_CHANCE"),
            Self::MOD_ATTACKER_RANGED_HIT_CHANCE => f.write_str("MOD_ATTACKER_RANGED_HIT_CHANCE"),
            Self::MOD_ATTACKER_SPELL_HIT_CHANCE => f.write_str("MOD_ATTACKER_SPELL_HIT_CHANCE"),
            Self::MOD_ATTACKER_MELEE_CRIT_CHANCE => f.write_str("MOD_ATTACKER_MELEE_CRIT_CHANCE"),
            Self::MOD_ATTACKER_RANGED_CRIT_CHANCE => f.write_str("MOD_ATTACKER_RANGED_CRIT_CHANCE"),
            Self::MOD_RATING => f.write_str("MOD_RATING"),
            Self::MOD_FACTION_REPUTATION_GAIN => f.write_str("MOD_FACTION_REPUTATION_GAIN"),
            Self::USE_NORMAL_MOVEMENT_SPEED => f.write_str("USE_NORMAL_MOVEMENT_SPEED"),
        }
    }
}

impl TryFrom<u32> for AuraType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NONE),
            1 => Ok(Self::BIND_SIGHT),
            2 => Ok(Self::MOD_POSSESS),
            3 => Ok(Self::PERIODIC_DAMAGE),
            4 => Ok(Self::DUMMY),
            5 => Ok(Self::MOD_CONFUSE),
            6 => Ok(Self::MOD_CHARM),
            7 => Ok(Self::MOD_FEAR),
            8 => Ok(Self::PERIODIC_HEAL),
            9 => Ok(Self::MOD_ATTACKSPEED),
            10 => Ok(Self::MOD_THREAT),
            11 => Ok(Self::MOD_TAUNT),
            12 => Ok(Self::MOD_STUN),
            13 => Ok(Self::MOD_DAMAGE_DONE),
            14 => Ok(Self::MOD_DAMAGE_TAKEN),
            15 => Ok(Self::DAMAGE_SHIELD),
            16 => Ok(Self::MOD_STEALTH),
            17 => Ok(Self::MOD_STEALTH_DETECT),
            18 => Ok(Self::MOD_INVISIBILITY),
            19 => Ok(Self::MOD_INVISIBILITY_DETECTION),
            20 => Ok(Self::OBS_MOD_HEALTH),
            21 => Ok(Self::OBS_MOD_MANA),
            22 => Ok(Self::MOD_RESISTANCE),
            23 => Ok(Self::PERIODIC_TRIGGER_SPELL),
            24 => Ok(Self::PERIODIC_ENERGIZE),
            25 => Ok(Self::MOD_PACIFY),
            26 => Ok(Self::MOD_ROOT),
            27 => Ok(Self::MOD_SILENCE),
            28 => Ok(Self::REFLECT_SPELLS),
            29 => Ok(Self::MOD_STAT),
            30 => Ok(Self::MOD_SKILL),
            31 => Ok(Self::MOD_INCREASE_SPEED),
            32 => Ok(Self::MOD_INCREASE_MOUNTED_SPEED),
            33 => Ok(Self::MOD_DECREASE_SPEED),
            34 => Ok(Self::MOD_INCREASE_HEALTH),
            35 => Ok(Self::MOD_INCREASE_ENERGY),
            36 => Ok(Self::MOD_SHAPESHIFT),
            37 => Ok(Self::EFFECT_IMMUNITY),
            38 => Ok(Self::STATE_IMMUNITY),
            39 => Ok(Self::SCHOOL_IMMUNITY),
            40 => Ok(Self::DAMAGE_IMMUNITY),
            41 => Ok(Self::DISPEL_IMMUNITY),
            42 => Ok(Self::PROC_TRIGGER_SPELL),
            43 => Ok(Self::PROC_TRIGGER_DAMAGE),
            44 => Ok(Self::TRACK_CREATURES),
            45 => Ok(Self::TRACK_RESOURCES),
            46 => Ok(Self::UNKNOWN46),
            47 => Ok(Self::MOD_PARRY_PERCENT),
            48 => Ok(Self::UNKNOWN48),
            49 => Ok(Self::MOD_DODGE_PERCENT),
            50 => Ok(Self::MOD_BLOCK_SKILL),
            51 => Ok(Self::MOD_BLOCK_PERCENT),
            52 => Ok(Self::MOD_CRIT_PERCENT),
            53 => Ok(Self::PERIODIC_LEECH),
            54 => Ok(Self::MOD_HIT_CHANCE),
            55 => Ok(Self::MOD_SPELL_HIT_CHANCE),
            56 => Ok(Self::TRANSFORM),
            57 => Ok(Self::MOD_SPELL_CRIT_CHANCE),
            58 => Ok(Self::MOD_INCREASE_SWIM_SPEED),
            59 => Ok(Self::MOD_DAMAGE_DONE_CREATURE),
            60 => Ok(Self::MOD_PACIFY_SILENCE),
            61 => Ok(Self::MOD_SCALE),
            62 => Ok(Self::PERIODIC_HEALTH_FUNNEL),
            63 => Ok(Self::PERIODIC_MANA_FUNNEL),
            64 => Ok(Self::PERIODIC_MANA_LEECH),
            65 => Ok(Self::MOD_CASTING_SPEED_NOT_STACK),
            66 => Ok(Self::FEIGN_DEATH),
            67 => Ok(Self::MOD_DISARM),
            68 => Ok(Self::MOD_STALKED),
            69 => Ok(Self::SCHOOL_ABSORB),
            70 => Ok(Self::EXTRA_ATTACKS),
            71 => Ok(Self::MOD_SPELL_CRIT_CHANCE_SCHOOL),
            72 => Ok(Self::MOD_POWER_COST_SCHOOL_PCT),
            73 => Ok(Self::MOD_POWER_COST_SCHOOL),
            74 => Ok(Self::REFLECT_SPELLS_SCHOOL),
            75 => Ok(Self::MOD_LANGUAGE),
            76 => Ok(Self::FAR_SIGHT),
            77 => Ok(Self::MECHANIC_IMMUNITY),
            78 => Ok(Self::MOUNTED),
            79 => Ok(Self::MOD_DAMAGE_PERCENT_DONE),
            80 => Ok(Self::MOD_PERCENT_STAT),
            81 => Ok(Self::SPLIT_DAMAGE_PCT),
            82 => Ok(Self::WATER_BREATHING),
            83 => Ok(Self::MOD_BASE_RESISTANCE),
            84 => Ok(Self::MOD_REGEN),
            85 => Ok(Self::MOD_POWER_REGEN),
            86 => Ok(Self::CHANNEL_DEATH_ITEM),
            87 => Ok(Self::MOD_DAMAGE_PERCENT_TAKEN),
            88 => Ok(Self::MOD_HEALTH_REGEN_PERCENT),
            89 => Ok(Self::PERIODIC_DAMAGE_PERCENT),
            90 => Ok(Self::MOD_RESIST_CHANCE),
            91 => Ok(Self::MOD_DETECT_RANGE),
            92 => Ok(Self::PREVENTS_FLEEING),
            93 => Ok(Self::MOD_UNATTACKABLE),
            94 => Ok(Self::INTERRUPT_REGEN),
            95 => Ok(Self::GHOST),
            96 => Ok(Self::SPELL_MAGNET),
            97 => Ok(Self::MANA_SHIELD),
            98 => Ok(Self::MOD_SKILL_TALENT),
            99 => Ok(Self::MOD_ATTACK_POWER),
            100 => Ok(Self::AURAS_VISIBLE),
            101 => Ok(Self::MOD_RESISTANCE_PCT),
            102 => Ok(Self::MOD_MELEE_ATTACK_POWER_VERSUS),
            103 => Ok(Self::MOD_TOTAL_THREAT),
            104 => Ok(Self::WATER_WALK),
            105 => Ok(Self::FEATHER_FALL),
            106 => Ok(Self::HOVER),
            107 => Ok(Self::ADD_FLAT_MODIFIER),
            108 => Ok(Self::ADD_PCT_MODIFIER),
            109 => Ok(Self::ADD_TARGET_TRIGGER),
            110 => Ok(Self::MOD_POWER_REGEN_PERCENT),
            111 => Ok(Self::ADD_CASTER_HIT_TRIGGER),
            112 => Ok(Self::OVERRIDE_CLASS_SCRIPTS),
            113 => Ok(Self::MOD_RANGED_DAMAGE_TAKEN),
            114 => Ok(Self::MOD_RANGED_DAMAGE_TAKEN_PCT),
            115 => Ok(Self::MOD_HEALING),
            116 => Ok(Self::MOD_REGEN_DURING_COMBAT),
            117 => Ok(Self::MOD_MECHANIC_RESISTANCE),
            118 => Ok(Self::MOD_HEALING_PCT),
            119 => Ok(Self::SHARE_PET_TRACKING),
            120 => Ok(Self::UNTRACKABLE),
            121 => Ok(Self::EMPATHY),
            122 => Ok(Self::MOD_OFFHAND_DAMAGE_PCT),
            123 => Ok(Self::MOD_TARGET_RESISTANCE),
            124 => Ok(Self::MOD_RANGED_ATTACK_POWER),
            125 => Ok(Self::MOD_MELEE_DAMAGE_TAKEN),
            126 => Ok(Self::MOD_MELEE_DAMAGE_TAKEN_PCT),
            127 => Ok(Self::RANGED_ATTACK_POWER_ATTACKER_BONUS),
            128 => Ok(Self::MOD_POSSESS_PET),
            129 => Ok(Self::MOD_SPEED_ALWAYS),
            130 => Ok(Self::MOD_MOUNTED_SPEED_ALWAYS),
            131 => Ok(Self::MOD_RANGED_ATTACK_POWER_VERSUS),
            132 => Ok(Self::MOD_INCREASE_ENERGY_PERCENT),
            133 => Ok(Self::MOD_INCREASE_HEALTH_PERCENT),
            134 => Ok(Self::MOD_MANA_REGEN_INTERRUPT),
            135 => Ok(Self::MOD_HEALING_DONE),
            136 => Ok(Self::MOD_HEALING_DONE_PERCENT),
            137 => Ok(Self::MOD_TOTAL_STAT_PERCENTAGE),
            138 => Ok(Self::MOD_MELEE_HASTE),
            139 => Ok(Self::FORCE_REACTION),
            140 => Ok(Self::MOD_RANGED_HASTE),
            141 => Ok(Self::MOD_RANGED_AMMO_HASTE),
            142 => Ok(Self::MOD_BASE_RESISTANCE_PCT),
            143 => Ok(Self::MOD_RESISTANCE_EXCLUSIVE),
            144 => Ok(Self::SAFE_FALL),
            145 => Ok(Self::CHARISMA),
            146 => Ok(Self::PERSUADED),
            147 => Ok(Self::MECHANIC_IMMUNITY_MASK),
            148 => Ok(Self::RETAIN_COMBO_POINTS),
            149 => Ok(Self::RESIST_PUSHBACK),
            150 => Ok(Self::MOD_SHIELD_BLOCKVALUE_PCT),
            151 => Ok(Self::TRACK_STEALTHED),
            152 => Ok(Self::MOD_DETECTED_RANGE),
            153 => Ok(Self::SPLIT_DAMAGE_FLAT),
            154 => Ok(Self::MOD_STEALTH_LEVEL),
            155 => Ok(Self::MOD_WATER_BREATHING),
            156 => Ok(Self::MOD_REPUTATION_GAIN),
            157 => Ok(Self::PET_DAMAGE_MULTI),
            158 => Ok(Self::MOD_SHIELD_BLOCKVALUE),
            159 => Ok(Self::NO_PVP_CREDIT),
            160 => Ok(Self::MOD_AOE_AVOIDANCE),
            161 => Ok(Self::MOD_HEALTH_REGEN_IN_COMBAT),
            162 => Ok(Self::POWER_BURN_MANA),
            163 => Ok(Self::MOD_CRIT_DAMAGE_BONUS),
            164 => Ok(Self::UNKNOWN164),
            165 => Ok(Self::MELEE_ATTACK_POWER_ATTACKER_BONUS),
            166 => Ok(Self::MOD_ATTACK_POWER_PCT),
            167 => Ok(Self::MOD_RANGED_ATTACK_POWER_PCT),
            168 => Ok(Self::MOD_DAMAGE_DONE_VERSUS),
            169 => Ok(Self::MOD_CRIT_PERCENT_VERSUS),
            170 => Ok(Self::DETECT_AMORE),
            171 => Ok(Self::MOD_SPEED_NOT_STACK),
            172 => Ok(Self::MOD_MOUNTED_SPEED_NOT_STACK),
            173 => Ok(Self::ALLOW_CHAMPION_SPELLS),
            174 => Ok(Self::MOD_SPELL_DAMAGE_OF_STAT_PERCENT),
            175 => Ok(Self::MOD_SPELL_HEALING_OF_STAT_PERCENT),
            176 => Ok(Self::SPIRIT_OF_REDEMPTION),
            177 => Ok(Self::AOE_CHARM),
            178 => Ok(Self::MOD_DEBUFF_RESISTANCE),
            179 => Ok(Self::MOD_ATTACKER_SPELL_CRIT_CHANCE),
            180 => Ok(Self::MOD_FLAT_SPELL_DAMAGE_VERSUS),
            181 => Ok(Self::MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS),
            182 => Ok(Self::MOD_RESISTANCE_OF_STAT_PERCENT),
            183 => Ok(Self::MOD_CRITICAL_THREAT),
            184 => Ok(Self::MOD_ATTACKER_MELEE_HIT_CHANCE),
            185 => Ok(Self::MOD_ATTACKER_RANGED_HIT_CHANCE),
            186 => Ok(Self::MOD_ATTACKER_SPELL_HIT_CHANCE),
            187 => Ok(Self::MOD_ATTACKER_MELEE_CRIT_CHANCE),
            188 => Ok(Self::MOD_ATTACKER_RANGED_CRIT_CHANCE),
            189 => Ok(Self::MOD_RATING),
            190 => Ok(Self::MOD_FACTION_REPUTATION_GAIN),
            191 => Ok(Self::USE_NORMAL_MOVEMENT_SPEED),
            v => Err(crate::errors::EnumError::new("AuraType", v as u32),)
        }
    }
}

