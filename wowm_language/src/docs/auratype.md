## Client Version 1.12

### Wowm Representation
```rust,ignore
enum AuraType : u32 {
    NONE = 0;
    BIND_SIGHT = 1;
    MOD_POSSESS = 2;
    PERIODIC_DAMAGE = 3;
    DUMMY = 4;
    MOD_CONFUSE = 5;
    MOD_CHARM = 6;
    MOD_FEAR = 7;
    PERIODIC_HEAL = 8;
    MOD_ATTACKSPEED = 9;
    MOD_THREAT = 10;
    MOD_TAUNT = 11;
    MOD_STUN = 12;
    MOD_DAMAGE_DONE = 13;
    MOD_DAMAGE_TAKEN = 14;
    DAMAGE_SHIELD = 15;
    MOD_STEALTH = 16;
    MOD_STEALTH_DETECT = 17;
    MOD_INVISIBILITY = 18;
    MOD_INVISIBILITY_DETECTION = 19;
    OBS_MOD_HEALTH = 20;
    OBS_MOD_MANA = 21;
    MOD_RESISTANCE = 22;
    PERIODIC_TRIGGER_SPELL = 23;
    PERIODIC_ENERGIZE = 24;
    MOD_PACIFY = 25;
    MOD_ROOT = 26;
    MOD_SILENCE = 27;
    REFLECT_SPELLS = 28;
    MOD_STAT = 29;
    MOD_SKILL = 30;
    MOD_INCREASE_SPEED = 31;
    MOD_INCREASE_MOUNTED_SPEED = 32;
    MOD_DECREASE_SPEED = 33;
    MOD_INCREASE_HEALTH = 34;
    MOD_INCREASE_ENERGY = 35;
    MOD_SHAPESHIFT = 36;
    EFFECT_IMMUNITY = 37;
    STATE_IMMUNITY = 38;
    SCHOOL_IMMUNITY = 39;
    DAMAGE_IMMUNITY = 40;
    DISPEL_IMMUNITY = 41;
    PROC_TRIGGER_SPELL = 42;
    PROC_TRIGGER_DAMAGE = 43;
    TRACK_CREATURES = 44;
    TRACK_RESOURCES = 45;
    UNKNOWN46 = 46;
    MOD_PARRY_PERCENT = 47;
    UNKNOWN48 = 48;
    MOD_DODGE_PERCENT = 49;
    MOD_BLOCK_SKILL = 50;
    MOD_BLOCK_PERCENT = 51;
    MOD_CRIT_PERCENT = 52;
    PERIODIC_LEECH = 53;
    MOD_HIT_CHANCE = 54;
    MOD_SPELL_HIT_CHANCE = 55;
    TRANSFORM = 56;
    MOD_SPELL_CRIT_CHANCE = 57;
    MOD_INCREASE_SWIM_SPEED = 58;
    MOD_DAMAGE_DONE_CREATURE = 59;
    MOD_PACIFY_SILENCE = 60;
    MOD_SCALE = 61;
    PERIODIC_HEALTH_FUNNEL = 62;
    PERIODIC_MANA_FUNNEL = 63;
    PERIODIC_MANA_LEECH = 64;
    MOD_CASTING_SPEED_NOT_STACK = 65;
    FEIGN_DEATH = 66;
    MOD_DISARM = 67;
    MOD_STALKED = 68;
    SCHOOL_ABSORB = 69;
    EXTRA_ATTACKS = 70;
    MOD_SPELL_CRIT_CHANCE_SCHOOL = 71;
    MOD_POWER_COST_SCHOOL_PCT = 72;
    MOD_POWER_COST_SCHOOL = 73;
    REFLECT_SPELLS_SCHOOL = 74;
    MOD_LANGUAGE = 75;
    FAR_SIGHT = 76;
    MECHANIC_IMMUNITY = 77;
    MOUNTED = 78;
    MOD_DAMAGE_PERCENT_DONE = 79;
    MOD_PERCENT_STAT = 80;
    SPLIT_DAMAGE_PCT = 81;
    WATER_BREATHING = 82;
    MOD_BASE_RESISTANCE = 83;
    MOD_REGEN = 84;
    MOD_POWER_REGEN = 85;
    CHANNEL_DEATH_ITEM = 86;
    MOD_DAMAGE_PERCENT_TAKEN = 87;
    MOD_HEALTH_REGEN_PERCENT = 88;
    PERIODIC_DAMAGE_PERCENT = 89;
    MOD_RESIST_CHANCE = 90;
    MOD_DETECT_RANGE = 91;
    PREVENTS_FLEEING = 92;
    MOD_UNATTACKABLE = 93;
    INTERRUPT_REGEN = 94;
    GHOST = 95;
    SPELL_MAGNET = 96;
    MANA_SHIELD = 97;
    MOD_SKILL_TALENT = 98;
    MOD_ATTACK_POWER = 99;
    AURAS_VISIBLE = 100;
    MOD_RESISTANCE_PCT = 101;
    MOD_MELEE_ATTACK_POWER_VERSUS = 102;
    MOD_TOTAL_THREAT = 103;
    WATER_WALK = 104;
    FEATHER_FALL = 105;
    HOVER = 106;
    ADD_FLAT_MODIFIER = 107;
    ADD_PCT_MODIFIER = 108;
    ADD_TARGET_TRIGGER = 109;
    MOD_POWER_REGEN_PERCENT = 110;
    ADD_CASTER_HIT_TRIGGER = 111;
    OVERRIDE_CLASS_SCRIPTS = 112;
    MOD_RANGED_DAMAGE_TAKEN = 113;
    MOD_RANGED_DAMAGE_TAKEN_PCT = 114;
    MOD_HEALING = 115;
    MOD_REGEN_DURING_COMBAT = 116;
    MOD_MECHANIC_RESISTANCE = 117;
    MOD_HEALING_PCT = 118;
    SHARE_PET_TRACKING = 119;
    UNTRACKABLE = 120;
    EMPATHY = 121;
    MOD_OFFHAND_DAMAGE_PCT = 122;
    MOD_TARGET_RESISTANCE = 123;
    MOD_RANGED_ATTACK_POWER = 124;
    MOD_MELEE_DAMAGE_TAKEN = 125;
    MOD_MELEE_DAMAGE_TAKEN_PCT = 126;
    RANGED_ATTACK_POWER_ATTACKER_BONUS = 127;
    MOD_POSSESS_PET = 128;
    MOD_SPEED_ALWAYS = 129;
    MOD_MOUNTED_SPEED_ALWAYS = 130;
    MOD_RANGED_ATTACK_POWER_VERSUS = 131;
    MOD_INCREASE_ENERGY_PERCENT = 132;
    MOD_INCREASE_HEALTH_PERCENT = 133;
    MOD_MANA_REGEN_INTERRUPT = 134;
    MOD_HEALING_DONE = 135;
    MOD_HEALING_DONE_PERCENT = 136;
    MOD_TOTAL_STAT_PERCENTAGE = 137;
    MOD_MELEE_HASTE = 138;
    FORCE_REACTION = 139;
    MOD_RANGED_HASTE = 140;
    MOD_RANGED_AMMO_HASTE = 141;
    MOD_BASE_RESISTANCE_PCT = 142;
    MOD_RESISTANCE_EXCLUSIVE = 143;
    SAFE_FALL = 144;
    CHARISMA = 145;
    PERSUADED = 146;
    MECHANIC_IMMUNITY_MASK = 147;
    RETAIN_COMBO_POINTS = 148;
    RESIST_PUSHBACK = 149;
    MOD_SHIELD_BLOCKVALUE_PCT = 150;
    TRACK_STEALTHED = 151;
    MOD_DETECTED_RANGE = 152;
    SPLIT_DAMAGE_FLAT = 153;
    MOD_STEALTH_LEVEL = 154;
    MOD_WATER_BREATHING = 155;
    MOD_REPUTATION_GAIN = 156;
    PET_DAMAGE_MULTI = 157;
    MOD_SHIELD_BLOCKVALUE = 158;
    NO_PVP_CREDIT = 159;
    MOD_AOE_AVOIDANCE = 160;
    MOD_HEALTH_REGEN_IN_COMBAT = 161;
    POWER_BURN_MANA = 162;
    MOD_CRIT_DAMAGE_BONUS = 163;
    UNKNOWN164 = 164;
    MELEE_ATTACK_POWER_ATTACKER_BONUS = 165;
    MOD_ATTACK_POWER_PCT = 166;
    MOD_RANGED_ATTACK_POWER_PCT = 167;
    MOD_DAMAGE_DONE_VERSUS = 168;
    MOD_CRIT_PERCENT_VERSUS = 169;
    DETECT_AMORE = 170;
    MOD_SPEED_NOT_STACK = 171;
    MOD_MOUNTED_SPEED_NOT_STACK = 172;
    ALLOW_CHAMPION_SPELLS = 173;
    MOD_SPELL_DAMAGE_OF_STAT_PERCENT = 174;
    MOD_SPELL_HEALING_OF_STAT_PERCENT = 175;
    SPIRIT_OF_REDEMPTION = 176;
    AOE_CHARM = 177;
    MOD_DEBUFF_RESISTANCE = 178;
    MOD_ATTACKER_SPELL_CRIT_CHANCE = 179;
    MOD_FLAT_SPELL_DAMAGE_VERSUS = 180;
    MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS = 181;
    MOD_RESISTANCE_OF_STAT_PERCENT = 182;
    MOD_CRITICAL_THREAT = 183;
    MOD_ATTACKER_MELEE_HIT_CHANCE = 184;
    MOD_ATTACKER_RANGED_HIT_CHANCE = 185;
    MOD_ATTACKER_SPELL_HIT_CHANCE = 186;
    MOD_ATTACKER_MELEE_CRIT_CHANCE = 187;
    MOD_ATTACKER_RANGED_CRIT_CHANCE = 188;
    MOD_RATING = 189;
    MOD_FACTION_REPUTATION_GAIN = 190;
    USE_NORMAL_MOVEMENT_SPEED = 191;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NONE` | 0 (0x00) |  |  |
| `BIND_SIGHT` | 1 (0x01) |  |  |
| `MOD_POSSESS` | 2 (0x02) |  |  |
| `PERIODIC_DAMAGE` | 3 (0x03) |  | vmangos: The aura should do periodic damage, the function that handles this is Aura::HandlePeriodicDamage, the amount is usually decided by the Unit::SpellDamageBonusDone or Unit::MeleeDamageBonusDone which increases/decreases the Modifier::m_amount |
| `DUMMY` | 4 (0x04) |  | vmangos: Used by Aura::HandleAuraDummy |
| `MOD_CONFUSE` | 5 (0x05) |  | vmangos: Used by Aura::HandleModConfuse, will either confuse or unconfuse the target depending on whether the apply flag is set |
| `MOD_CHARM` | 6 (0x06) |  |  |
| `MOD_FEAR` | 7 (0x07) |  |  |
| `PERIODIC_HEAL` | 8 (0x08) |  | vmangos: The aura will do periodic heals of a target, handled by Aura::HandlePeriodicHeal, uses Unit::SpellHealingBonusDone to calculate whether to increase or decrease Modifier::m_amount |
| `MOD_ATTACKSPEED` | 9 (0x09) |  | vmangos: Changes the attackspeed, the Modifier::m_amount decides how much we change in percent, ie, if the m_amount is 50 the attackspeed will increase by 50% |
| `MOD_THREAT` | 10 (0x0A) |  | vmangos: Modifies the threat that the Aura does in percent, the Modifier::m_miscvalue decides which of the SpellSchools it should affect threat for.  \see SpellSchoolMask |
| `MOD_TAUNT` | 11 (0x0B) |  | vmangos: Just applies a taunt which will change the threat a mob has Taken care of in Aura::HandleModThreat |
| `MOD_STUN` | 12 (0x0C) |  | vmangos: Stuns targets in different ways, taken care of in Aura::HandleAuraModStun |
| `MOD_DAMAGE_DONE` | 13 (0x0D) |  | vmangos: Changes the damage done by a weapon in any hand, the Modifier::m_miscvalue will tell what school the damage is from, it's used as a bitmask \see SpellSchoolMask |
| `MOD_DAMAGE_TAKEN` | 14 (0x0E) |  | vmangos: Not handled by the Aura class but instead this is implemented in Unit::MeleeDamageBonusTaken and Unit::SpellBaseDamageBonusTaken |
| `DAMAGE_SHIELD` | 15 (0x0F) |  | vmangos: Not handled by the Aura class, implemented in Unit::DealMeleeDamage |
| `MOD_STEALTH` | 16 (0x10) |  | vmangos: Taken care of in Aura::HandleModStealth, take note that this is not the same thing as invisibility |
| `MOD_STEALTH_DETECT` | 17 (0x11) |  | vmangos: Not handled by the Aura class, implemented in Unit::isVisibleForOrDetect which does a lot of checks to determine whether the person is visible or not, the SPELL_AURA_MOD_STEALTH seems to determine how in/visible ie a rogue is. |
| `MOD_INVISIBILITY` | 18 (0x12) |  | vmangos: Handled by Aura::HandleInvisibility, the Modifier::m_miscvalue in the struct seems to decide what kind of invisibility it is with a bitflag. the miscvalue decides which bit is set, ie: 3 would make the 3rd bit be set. |
| `MOD_INVISIBILITY_DETECTION` | 19 (0x13) |  | vmangos: Adds one of the kinds of detections to the possible detections.  As in SPEALL_AURA_MOD_INVISIBILITY the Modifier::m_miscvalue seems to decide what kind of invisibility the Unit should be able to detect. |
| `OBS_MOD_HEALTH` | 20 (0x14) |  | 20,21 unofficial |
| `OBS_MOD_MANA` | 21 (0x15) |  |  |
| `MOD_RESISTANCE` | 22 (0x16) |  | vmangos: Handled by Aura::HandleAuraModResistance, changes the resistance for a unit the field Modifier::m_miscvalue decides which kind of resistance that should be changed, for possible values see SpellSchools.  \see SpellSchools |
| `PERIODIC_TRIGGER_SPELL` | 23 (0x17) |  | vmangos: Currently just sets Aura::m_isPeriodic to apply and has a special case for Curse of the Plaguebringer. |
| `PERIODIC_ENERGIZE` | 24 (0x18) |  | vmangos: Just sets Aura::m_isPeriodic to apply |
| `MOD_PACIFY` | 25 (0x19) |  | vmangos: Changes whether the target is pacified or not depending on the apply flag.  Pacify makes the target silenced and have all it's attack skill disabled.  See: http://classic.wowhead.com/spell=6462 |
| `MOD_ROOT` | 26 (0x1A) |  | vmangos: Roots or unroots the target |
| `MOD_SILENCE` | 27 (0x1B) |  | vmangos: Silences the target and stops and spell casts that should be stopped, they have the flag SpellPreventionType::SPELL_PREVENTION_TYPE_SILENCE |
| `REFLECT_SPELLS` | 28 (0x1C) |  |  |
| `MOD_STAT` | 29 (0x1D) |  |  |
| `MOD_SKILL` | 30 (0x1E) |  |  |
| `MOD_INCREASE_SPEED` | 31 (0x1F) |  |  |
| `MOD_INCREASE_MOUNTED_SPEED` | 32 (0x20) |  |  |
| `MOD_DECREASE_SPEED` | 33 (0x21) |  |  |
| `MOD_INCREASE_HEALTH` | 34 (0x22) |  |  |
| `MOD_INCREASE_ENERGY` | 35 (0x23) |  |  |
| `MOD_SHAPESHIFT` | 36 (0x24) |  |  |
| `EFFECT_IMMUNITY` | 37 (0x25) |  |  |
| `STATE_IMMUNITY` | 38 (0x26) |  |  |
| `SCHOOL_IMMUNITY` | 39 (0x27) |  |  |
| `DAMAGE_IMMUNITY` | 40 (0x28) |  |  |
| `DISPEL_IMMUNITY` | 41 (0x29) |  |  |
| `PROC_TRIGGER_SPELL` | 42 (0x2A) |  |  |
| `PROC_TRIGGER_DAMAGE` | 43 (0x2B) |  |  |
| `TRACK_CREATURES` | 44 (0x2C) |  |  |
| `TRACK_RESOURCES` | 45 (0x2D) |  |  |
| `UNKNOWN46` | 46 (0x2E) |  | Ignore all Gear test spells |
| `MOD_PARRY_PERCENT` | 47 (0x2F) |  |  |
| `UNKNOWN48` | 48 (0x30) |  | One periodic spell |
| `MOD_DODGE_PERCENT` | 49 (0x31) |  |  |
| `MOD_BLOCK_SKILL` | 50 (0x32) |  |  |
| `MOD_BLOCK_PERCENT` | 51 (0x33) |  |  |
| `MOD_CRIT_PERCENT` | 52 (0x34) |  |  |
| `PERIODIC_LEECH` | 53 (0x35) |  |  |
| `MOD_HIT_CHANCE` | 54 (0x36) |  |  |
| `MOD_SPELL_HIT_CHANCE` | 55 (0x37) |  |  |
| `TRANSFORM` | 56 (0x38) |  |  |
| `MOD_SPELL_CRIT_CHANCE` | 57 (0x39) |  |  |
| `MOD_INCREASE_SWIM_SPEED` | 58 (0x3A) |  |  |
| `MOD_DAMAGE_DONE_CREATURE` | 59 (0x3B) |  |  |
| `MOD_PACIFY_SILENCE` | 60 (0x3C) |  |  |
| `MOD_SCALE` | 61 (0x3D) |  |  |
| `PERIODIC_HEALTH_FUNNEL` | 62 (0x3E) |  |  |
| `PERIODIC_MANA_FUNNEL` | 63 (0x3F) |  |  |
| `PERIODIC_MANA_LEECH` | 64 (0x40) |  |  |
| `MOD_CASTING_SPEED_NOT_STACK` | 65 (0x41) |  |  |
| `FEIGN_DEATH` | 66 (0x42) |  |  |
| `MOD_DISARM` | 67 (0x43) |  |  |
| `MOD_STALKED` | 68 (0x44) |  |  |
| `SCHOOL_ABSORB` | 69 (0x45) |  |  |
| `EXTRA_ATTACKS` | 70 (0x46) |  |  |
| `MOD_SPELL_CRIT_CHANCE_SCHOOL` | 71 (0x47) |  |  |
| `MOD_POWER_COST_SCHOOL_PCT` | 72 (0x48) |  |  |
| `MOD_POWER_COST_SCHOOL` | 73 (0x49) |  |  |
| `REFLECT_SPELLS_SCHOOL` | 74 (0x4A) |  |  |
| `MOD_LANGUAGE` | 75 (0x4B) |  |  |
| `FAR_SIGHT` | 76 (0x4C) |  |  |
| `MECHANIC_IMMUNITY` | 77 (0x4D) |  |  |
| `MOUNTED` | 78 (0x4E) |  |  |
| `MOD_DAMAGE_PERCENT_DONE` | 79 (0x4F) |  |  |
| `MOD_PERCENT_STAT` | 80 (0x50) |  |  |
| `SPLIT_DAMAGE_PCT` | 81 (0x51) |  |  |
| `WATER_BREATHING` | 82 (0x52) |  |  |
| `MOD_BASE_RESISTANCE` | 83 (0x53) |  |  |
| `MOD_REGEN` | 84 (0x54) |  |  |
| `MOD_POWER_REGEN` | 85 (0x55) |  |  |
| `CHANNEL_DEATH_ITEM` | 86 (0x56) |  |  |
| `MOD_DAMAGE_PERCENT_TAKEN` | 87 (0x57) |  |  |
| `MOD_HEALTH_REGEN_PERCENT` | 88 (0x58) |  |  |
| `PERIODIC_DAMAGE_PERCENT` | 89 (0x59) |  |  |
| `MOD_RESIST_CHANCE` | 90 (0x5A) |  |  |
| `MOD_DETECT_RANGE` | 91 (0x5B) |  |  |
| `PREVENTS_FLEEING` | 92 (0x5C) |  |  |
| `MOD_UNATTACKABLE` | 93 (0x5D) |  |  |
| `INTERRUPT_REGEN` | 94 (0x5E) |  |  |
| `GHOST` | 95 (0x5F) |  |  |
| `SPELL_MAGNET` | 96 (0x60) |  |  |
| `MANA_SHIELD` | 97 (0x61) |  |  |
| `MOD_SKILL_TALENT` | 98 (0x62) |  |  |
| `MOD_ATTACK_POWER` | 99 (0x63) |  |  |
| `AURAS_VISIBLE` | 100 (0x64) |  |  |
| `MOD_RESISTANCE_PCT` | 101 (0x65) |  |  |
| `MOD_MELEE_ATTACK_POWER_VERSUS` | 102 (0x66) |  |  |
| `MOD_TOTAL_THREAT` | 103 (0x67) |  |  |
| `WATER_WALK` | 104 (0x68) |  |  |
| `FEATHER_FALL` | 105 (0x69) |  |  |
| `HOVER` | 106 (0x6A) |  |  |
| `ADD_FLAT_MODIFIER` | 107 (0x6B) |  |  |
| `ADD_PCT_MODIFIER` | 108 (0x6C) |  |  |
| `ADD_TARGET_TRIGGER` | 109 (0x6D) |  |  |
| `MOD_POWER_REGEN_PERCENT` | 110 (0x6E) |  |  |
| `ADD_CASTER_HIT_TRIGGER` | 111 (0x6F) |  |  |
| `OVERRIDE_CLASS_SCRIPTS` | 112 (0x70) |  |  |
| `MOD_RANGED_DAMAGE_TAKEN` | 113 (0x71) |  |  |
| `MOD_RANGED_DAMAGE_TAKEN_PCT` | 114 (0x72) |  |  |
| `MOD_HEALING` | 115 (0x73) |  |  |
| `MOD_REGEN_DURING_COMBAT` | 116 (0x74) |  |  |
| `MOD_MECHANIC_RESISTANCE` | 117 (0x75) |  |  |
| `MOD_HEALING_PCT` | 118 (0x76) |  |  |
| `SHARE_PET_TRACKING` | 119 (0x77) |  |  |
| `UNTRACKABLE` | 120 (0x78) |  |  |
| `EMPATHY` | 121 (0x79) |  |  |
| `MOD_OFFHAND_DAMAGE_PCT` | 122 (0x7A) |  |  |
| `MOD_TARGET_RESISTANCE` | 123 (0x7B) |  |  |
| `MOD_RANGED_ATTACK_POWER` | 124 (0x7C) |  |  |
| `MOD_MELEE_DAMAGE_TAKEN` | 125 (0x7D) |  |  |
| `MOD_MELEE_DAMAGE_TAKEN_PCT` | 126 (0x7E) |  |  |
| `RANGED_ATTACK_POWER_ATTACKER_BONUS` | 127 (0x7F) |  |  |
| `MOD_POSSESS_PET` | 128 (0x80) |  |  |
| `MOD_SPEED_ALWAYS` | 129 (0x81) |  |  |
| `MOD_MOUNTED_SPEED_ALWAYS` | 130 (0x82) |  |  |
| `MOD_RANGED_ATTACK_POWER_VERSUS` | 131 (0x83) |  |  |
| `MOD_INCREASE_ENERGY_PERCENT` | 132 (0x84) |  |  |
| `MOD_INCREASE_HEALTH_PERCENT` | 133 (0x85) |  |  |
| `MOD_MANA_REGEN_INTERRUPT` | 134 (0x86) |  |  |
| `MOD_HEALING_DONE` | 135 (0x87) |  |  |
| `MOD_HEALING_DONE_PERCENT` | 136 (0x88) |  |  |
| `MOD_TOTAL_STAT_PERCENTAGE` | 137 (0x89) |  |  |
| `MOD_MELEE_HASTE` | 138 (0x8A) |  |  |
| `FORCE_REACTION` | 139 (0x8B) |  |  |
| `MOD_RANGED_HASTE` | 140 (0x8C) |  |  |
| `MOD_RANGED_AMMO_HASTE` | 141 (0x8D) |  |  |
| `MOD_BASE_RESISTANCE_PCT` | 142 (0x8E) |  |  |
| `MOD_RESISTANCE_EXCLUSIVE` | 143 (0x8F) |  |  |
| `SAFE_FALL` | 144 (0x90) |  |  |
| `CHARISMA` | 145 (0x91) |  |  |
| `PERSUADED` | 146 (0x92) |  |  |
| `MECHANIC_IMMUNITY_MASK` | 147 (0x93) |  |  |
| `RETAIN_COMBO_POINTS` | 148 (0x94) |  |  |
| `RESIST_PUSHBACK` | 149 (0x95) |  | Resist Pushback |
| `MOD_SHIELD_BLOCKVALUE_PCT` | 150 (0x96) |  |  |
| `TRACK_STEALTHED` | 151 (0x97) |  | Track Stealthed |
| `MOD_DETECTED_RANGE` | 152 (0x98) |  | Mod Detected Range |
| `SPLIT_DAMAGE_FLAT` | 153 (0x99) |  | Split Damage Flat |
| `MOD_STEALTH_LEVEL` | 154 (0x9A) |  | Stealth Level Modifier |
| `MOD_WATER_BREATHING` | 155 (0x9B) |  | Mod Water Breathing |
| `MOD_REPUTATION_GAIN` | 156 (0x9C) |  | Mod Reputation Gain |
| `PET_DAMAGE_MULTI` | 157 (0x9D) |  | Mod Pet Damage |
| `MOD_SHIELD_BLOCKVALUE` | 158 (0x9E) |  |  |
| `NO_PVP_CREDIT` | 159 (0x9F) |  |  |
| `MOD_AOE_AVOIDANCE` | 160 (0xA0) |  |  |
| `MOD_HEALTH_REGEN_IN_COMBAT` | 161 (0xA1) |  |  |
| `POWER_BURN_MANA` | 162 (0xA2) |  |  |
| `MOD_CRIT_DAMAGE_BONUS` | 163 (0xA3) |  |  |
| `UNKNOWN164` | 164 (0xA4) |  |  |
| `MELEE_ATTACK_POWER_ATTACKER_BONUS` | 165 (0xA5) |  |  |
| `MOD_ATTACK_POWER_PCT` | 166 (0xA6) |  |  |
| `MOD_RANGED_ATTACK_POWER_PCT` | 167 (0xA7) |  |  |
| `MOD_DAMAGE_DONE_VERSUS` | 168 (0xA8) |  |  |
| `MOD_CRIT_PERCENT_VERSUS` | 169 (0xA9) |  |  |
| `DETECT_AMORE` | 170 (0xAA) |  |  |
| `MOD_SPEED_NOT_STACK` | 171 (0xAB) |  |  |
| `MOD_MOUNTED_SPEED_NOT_STACK` | 172 (0xAC) |  |  |
| `ALLOW_CHAMPION_SPELLS` | 173 (0xAD) |  |  |
| `MOD_SPELL_DAMAGE_OF_STAT_PERCENT` | 174 (0xAE) |  | in 1.12.1 only dependent spirit case |
| `MOD_SPELL_HEALING_OF_STAT_PERCENT` | 175 (0xAF) |  |  |
| `SPIRIT_OF_REDEMPTION` | 176 (0xB0) |  |  |
| `AOE_CHARM` | 177 (0xB1) |  |  |
| `MOD_DEBUFF_RESISTANCE` | 178 (0xB2) |  |  |
| `MOD_ATTACKER_SPELL_CRIT_CHANCE` | 179 (0xB3) |  |  |
| `MOD_FLAT_SPELL_DAMAGE_VERSUS` | 180 (0xB4) |  |  |
| `MOD_FLAT_SPELL_CRIT_DAMAGE_VERSUS` | 181 (0xB5) |  | unused - possible flat spell crit damage versus |
| `MOD_RESISTANCE_OF_STAT_PERCENT` | 182 (0xB6) |  |  |
| `MOD_CRITICAL_THREAT` | 183 (0xB7) |  |  |
| `MOD_ATTACKER_MELEE_HIT_CHANCE` | 184 (0xB8) |  |  |
| `MOD_ATTACKER_RANGED_HIT_CHANCE` | 185 (0xB9) |  |  |
| `MOD_ATTACKER_SPELL_HIT_CHANCE` | 186 (0xBA) |  |  |
| `MOD_ATTACKER_MELEE_CRIT_CHANCE` | 187 (0xBB) |  |  |
| `MOD_ATTACKER_RANGED_CRIT_CHANCE` | 188 (0xBC) |  |  |
| `MOD_RATING` | 189 (0xBD) |  |  |
| `MOD_FACTION_REPUTATION_GAIN` | 190 (0xBE) |  |  |
| `USE_NORMAL_MOVEMENT_SPEED` | 191 (0xBF) |  |  |
