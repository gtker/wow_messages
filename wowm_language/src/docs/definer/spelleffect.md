## Client Version 1.12

## Wowm Representation
```rust,ignore
enum SpellEffect : u32 {
    NONE = 0;    
    INSTAKILL = 1;    
    SCHOOL_DAMAGE = 2;    
    DUMMY = 3;    
    PORTAL_TELEPORT = 4;    
    TELEPORT_UNITS = 5;    
    APPLY_AURA = 6;    
    ENVIRONMENTAL_DAMAGE = 7;    
    POWER_DRAIN = 8;    
    HEALTH_LEECH = 9;    
    HEAL = 10;    
    BIND = 11;    
    PORTAL = 12;    
    RITUAL_BASE = 13;    
    RITUAL_SPECIALIZE = 14;    
    RITUAL_ACTIVATE_PORTAL = 15;    
    QUEST_COMPLETE = 16;    
    WEAPON_DAMAGE_NOSCHOOL = 17;    
    RESURRECT = 18;    
    ADD_EXTRA_ATTACKS = 19;    
    DODGE = 20;    
    EVADE = 21;    
    PARRY = 22;    
    BLOCK = 23;    
    CREATE_ITEM = 24;    
    WEAPON = 25;    
    DEFENSE = 26;    
    PERSISTENT_AREA_AURA = 27;    
    SUMMON = 28;    
    LEAP = 29;    
    ENERGIZE = 30;    
    WEAPON_PERCENT_DAMAGE = 31;    
    TRIGGER_MISSILE = 32;    
    OPEN_LOCK = 33;    
    SUMMON_CHANGE_ITEM = 34;    
    APPLY_AREA_AURA_PARTY = 35;    
    LEARN_SPELL = 36;    
    SPELL_DEFENSE = 37;    
    DISPEL = 38;    
    LANGUAGE = 39;    
    DUAL_WIELD = 40;    
    SUMMON_WILD = 41;    
    SUMMON_GUARDIAN = 42;    
    TELEPORT_UNITS_FACE_CASTER = 43;    
    SKILL_STEP = 44;    
    ADD_HONOR = 45;    
    SPAWN = 46;    
    TRADE_SKILL = 47;    
    STEALTH = 48;    
    DETECT = 49;    
    TRANS_DOOR = 50;    
    FORCE_CRITICAL_HIT = 51;    
    GUARANTEE_HIT = 52;    
    ENCHANT_ITEM = 53;    
    ENCHANT_ITEM_TEMPORARY = 54;    
    TAMECREATURE = 55;    
    SUMMON_PET = 56;    
    LEARN_PET_SPELL = 57;    
    WEAPON_DAMAGE = 58;    
    OPEN_LOCK_ITEM = 59;    
    PROFICIENCY = 60;    
    SEND_EVENT = 61;    
    POWER_BURN = 62;    
    THREAT = 63;    
    TRIGGER_SPELL = 64;    
    HEALTH_FUNNEL = 65;    
    POWER_FUNNEL = 66;    
    HEAL_MAX_HEALTH = 67;    
    INTERRUPT_CAST = 68;    
    DISTRACT = 69;    
    PULL = 70;    
    PICKPOCKET = 71;    
    ADD_FARSIGHT = 72;    
    SUMMON_POSSESSED = 73;    
    SUMMON_TOTEM = 74;    
    HEAL_MECHANICAL = 75;    
    SUMMON_OBJECT_WILD = 76;    
    SCRIPT_EFFECT = 77;    
    ATTACK = 78;    
    SANCTUARY = 79;    
    ADD_COMBO_POINTS = 80;    
    CREATE_HOUSE = 81;    
    BIND_SIGHT = 82;    
    DUEL = 83;    
    STUCK = 84;    
    SUMMON_PLAYER = 85;    
    ACTIVATE_OBJECT = 86;    
    SUMMON_TOTEM_SLOT1 = 87;    
    SUMMON_TOTEM_SLOT2 = 88;    
    SUMMON_TOTEM_SLOT3 = 89;    
    SUMMON_TOTEM_SLOT4 = 90;    
    THREAT_ALL = 91;    
    ENCHANT_HELD_ITEM = 92;    
    SUMMON_PHANTASM = 93;    
    SELF_RESURRECT = 94;    
    SKINNING = 95;    
    CHARGE = 96;    
    SUMMON_CRITTER = 97;    
    KNOCK_BACK = 98;    
    DISENCHANT = 99;    
    INEBRIATE = 100;    
    FEED_PET = 101;    
    DISMISS_PET = 102;    
    REPUTATION = 103;    
    SUMMON_OBJECT_SLOT1 = 104;    
    SUMMON_OBJECT_SLOT2 = 105;    
    SUMMON_OBJECT_SLOT3 = 106;    
    SUMMON_OBJECT_SLOT4 = 107;    
    DISPEL_MECHANIC = 108;    
    SUMMON_DEAD_PET = 109;    
    DESTROY_ALL_TOTEMS = 110;    
    DURABILITY_DAMAGE = 111;    
    SUMMON_DEMON = 112;    
    RESURRECT_NEW = 113;    
    ATTACK_ME = 114;    
    DURABILITY_DAMAGE_PCT = 115;    
    SKIN_PLAYER_CORPSE = 116;    
    SPIRIT_HEAL = 117;    
    SKILL = 118;    
    APPLY_AREA_AURA_PET = 119;    
    TELEPORT_GRAVEYARD = 120;    
    NORMALIZED_WEAPON_DMG = 121;    
    UNKNOWN122 = 122;    
    SEND_TAXI = 123;    
    PLAYER_PULL = 124;    
    MODIFY_THREAT_PERCENT = 125;    
    UNKNOWN126 = 126;    
    UNKNOWN127 = 127;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| NONE | 0 | 0 | 0x0 |  |  |
| INSTAKILL | 1 | 1 | 0x1 |  |  |
| SCHOOL_DAMAGE | 2 | 2 | 0x2 |  |  |
| DUMMY | 3 | 3 | 0x3 |  |  |
| PORTAL_TELEPORT | 4 | 4 | 0x4 |  |  |
| TELEPORT_UNITS | 5 | 5 | 0x5 |  |  |
| APPLY_AURA | 6 | 6 | 0x6 |  |  |
| ENVIRONMENTAL_DAMAGE | 7 | 7 | 0x7 |  |  |
| POWER_DRAIN | 8 | 8 | 0x8 |  |  |
| HEALTH_LEECH | 9 | 9 | 0x9 |  |  |
| HEAL | 10 | 10 | 0xA |  |  |
| BIND | 11 | 11 | 0xB |  |  |
| PORTAL | 12 | 12 | 0xC |  |  |
| RITUAL_BASE | 13 | 13 | 0xD |  |  |
| RITUAL_SPECIALIZE | 14 | 14 | 0xE |  |  |
| RITUAL_ACTIVATE_PORTAL | 15 | 15 | 0xF |  |  |
| QUEST_COMPLETE | 16 | 16 | 0x10 |  |  |
| WEAPON_DAMAGE_NOSCHOOL | 17 | 17 | 0x11 |  |  |
| RESURRECT | 18 | 18 | 0x12 |  |  |
| ADD_EXTRA_ATTACKS | 19 | 19 | 0x13 |  |  |
| DODGE | 20 | 20 | 0x14 |  |  |
| EVADE | 21 | 21 | 0x15 |  |  |
| PARRY | 22 | 22 | 0x16 |  |  |
| BLOCK | 23 | 23 | 0x17 |  |  |
| CREATE_ITEM | 24 | 24 | 0x18 |  |  |
| WEAPON | 25 | 25 | 0x19 |  |  |
| DEFENSE | 26 | 26 | 0x1A |  |  |
| PERSISTENT_AREA_AURA | 27 | 27 | 0x1B |  |  |
| SUMMON | 28 | 28 | 0x1C |  |  |
| LEAP | 29 | 29 | 0x1D |  |  |
| ENERGIZE | 30 | 30 | 0x1E |  |  |
| WEAPON_PERCENT_DAMAGE | 31 | 31 | 0x1F |  |  |
| TRIGGER_MISSILE | 32 | 32 | 0x20 |  |  |
| OPEN_LOCK | 33 | 33 | 0x21 |  |  |
| SUMMON_CHANGE_ITEM | 34 | 34 | 0x22 |  |  |
| APPLY_AREA_AURA_PARTY | 35 | 35 | 0x23 |  |  |
| LEARN_SPELL | 36 | 36 | 0x24 |  |  |
| SPELL_DEFENSE | 37 | 37 | 0x25 |  |  |
| DISPEL | 38 | 38 | 0x26 |  |  |
| LANGUAGE | 39 | 39 | 0x27 |  |  |
| DUAL_WIELD | 40 | 40 | 0x28 |  |  |
| SUMMON_WILD | 41 | 41 | 0x29 |  |  |
| SUMMON_GUARDIAN | 42 | 42 | 0x2A |  |  |
| TELEPORT_UNITS_FACE_CASTER | 43 | 43 | 0x2B |  |  |
| SKILL_STEP | 44 | 44 | 0x2C |  |  |
| ADD_HONOR | 45 | 45 | 0x2D |  |  |
| SPAWN | 46 | 46 | 0x2E |  |  |
| TRADE_SKILL | 47 | 47 | 0x2F |  |  |
| STEALTH | 48 | 48 | 0x30 |  |  |
| DETECT | 49 | 49 | 0x31 |  |  |
| TRANS_DOOR | 50 | 50 | 0x32 |  |  |
| FORCE_CRITICAL_HIT | 51 | 51 | 0x33 |  |  |
| GUARANTEE_HIT | 52 | 52 | 0x34 |  |  |
| ENCHANT_ITEM | 53 | 53 | 0x35 |  |  |
| ENCHANT_ITEM_TEMPORARY | 54 | 54 | 0x36 |  |  |
| TAMECREATURE | 55 | 55 | 0x37 |  |  |
| SUMMON_PET | 56 | 56 | 0x38 |  |  |
| LEARN_PET_SPELL | 57 | 57 | 0x39 |  |  |
| WEAPON_DAMAGE | 58 | 58 | 0x3A |  |  |
| OPEN_LOCK_ITEM | 59 | 59 | 0x3B |  |  |
| PROFICIENCY | 60 | 60 | 0x3C |  |  |
| SEND_EVENT | 61 | 61 | 0x3D |  |  |
| POWER_BURN | 62 | 62 | 0x3E |  |  |
| THREAT | 63 | 63 | 0x3F |  |  |
| TRIGGER_SPELL | 64 | 64 | 0x40 |  |  |
| HEALTH_FUNNEL | 65 | 65 | 0x41 |  |  |
| POWER_FUNNEL | 66 | 66 | 0x42 |  |  |
| HEAL_MAX_HEALTH | 67 | 67 | 0x43 |  |  |
| INTERRUPT_CAST | 68 | 68 | 0x44 |  |  |
| DISTRACT | 69 | 69 | 0x45 |  |  |
| PULL | 70 | 70 | 0x46 |  |  |
| PICKPOCKET | 71 | 71 | 0x47 |  |  |
| ADD_FARSIGHT | 72 | 72 | 0x48 |  |  |
| SUMMON_POSSESSED | 73 | 73 | 0x49 |  |  |
| SUMMON_TOTEM | 74 | 74 | 0x4A |  |  |
| HEAL_MECHANICAL | 75 | 75 | 0x4B |  |  |
| SUMMON_OBJECT_WILD | 76 | 76 | 0x4C |  |  |
| SCRIPT_EFFECT | 77 | 77 | 0x4D |  |  |
| ATTACK | 78 | 78 | 0x4E |  |  |
| SANCTUARY | 79 | 79 | 0x4F |  |  |
| ADD_COMBO_POINTS | 80 | 80 | 0x50 |  |  |
| CREATE_HOUSE | 81 | 81 | 0x51 |  |  |
| BIND_SIGHT | 82 | 82 | 0x52 |  |  |
| DUEL | 83 | 83 | 0x53 |  |  |
| STUCK | 84 | 84 | 0x54 |  |  |
| SUMMON_PLAYER | 85 | 85 | 0x55 |  |  |
| ACTIVATE_OBJECT | 86 | 86 | 0x56 |  |  |
| SUMMON_TOTEM_SLOT1 | 87 | 87 | 0x57 |  |  |
| SUMMON_TOTEM_SLOT2 | 88 | 88 | 0x58 |  |  |
| SUMMON_TOTEM_SLOT3 | 89 | 89 | 0x59 |  |  |
| SUMMON_TOTEM_SLOT4 | 90 | 90 | 0x5A |  |  |
| THREAT_ALL | 91 | 91 | 0x5B |  |  |
| ENCHANT_HELD_ITEM | 92 | 92 | 0x5C |  |  |
| SUMMON_PHANTASM | 93 | 93 | 0x5D |  |  |
| SELF_RESURRECT | 94 | 94 | 0x5E |  |  |
| SKINNING | 95 | 95 | 0x5F |  |  |
| CHARGE | 96 | 96 | 0x60 |  |  |
| SUMMON_CRITTER | 97 | 97 | 0x61 |  |  |
| KNOCK_BACK | 98 | 98 | 0x62 |  |  |
| DISENCHANT | 99 | 99 | 0x63 |  |  |
| INEBRIATE | 100 | 100 | 0x64 |  |  |
| FEED_PET | 101 | 101 | 0x65 |  |  |
| DISMISS_PET | 102 | 102 | 0x66 |  |  |
| REPUTATION | 103 | 103 | 0x67 |  |  |
| SUMMON_OBJECT_SLOT1 | 104 | 104 | 0x68 |  |  |
| SUMMON_OBJECT_SLOT2 | 105 | 105 | 0x69 |  |  |
| SUMMON_OBJECT_SLOT3 | 106 | 106 | 0x6A |  |  |
| SUMMON_OBJECT_SLOT4 | 107 | 107 | 0x6B |  |  |
| DISPEL_MECHANIC | 108 | 108 | 0x6C |  |  |
| SUMMON_DEAD_PET | 109 | 109 | 0x6D |  |  |
| DESTROY_ALL_TOTEMS | 110 | 110 | 0x6E |  |  |
| DURABILITY_DAMAGE | 111 | 111 | 0x6F |  |  |
| SUMMON_DEMON | 112 | 112 | 0x70 |  |  |
| RESURRECT_NEW | 113 | 113 | 0x71 |  |  |
| ATTACK_ME | 114 | 114 | 0x72 |  |  |
| DURABILITY_DAMAGE_PCT | 115 | 115 | 0x73 |  |  |
| SKIN_PLAYER_CORPSE | 116 | 116 | 0x74 |  |  |
| SPIRIT_HEAL | 117 | 117 | 0x75 |  |  |
| SKILL | 118 | 118 | 0x76 |  |  |
| APPLY_AREA_AURA_PET | 119 | 119 | 0x77 |  |  |
| TELEPORT_GRAVEYARD | 120 | 120 | 0x78 |  |  |
| NORMALIZED_WEAPON_DMG | 121 | 121 | 0x79 |  |  |
| UNKNOWN122 | 122 | 122 | 0x7A |  |  |
| SEND_TAXI | 123 | 123 | 0x7B |  |  |
| PLAYER_PULL | 124 | 124 | 0x7C |  |  |
| MODIFY_THREAT_PERCENT | 125 | 125 | 0x7D |  |  |
| UNKNOWN126 | 126 | 126 | 0x7E |  |  |
| UNKNOWN127 | 127 | 127 | 0x7F |  |  |
