## Client Version 1.12

## Wowm Representation
```rust,ignore
enum SpellCastResult : u8 {
    AFFECTING_COMBAT = 0x00;    
    ALREADY_AT_FULL_HEALTH = 0x01;    
    ALREADY_AT_FULL_MANA = 0x02;    
    ALREADY_BEING_TAMED = 0x03;    
    ALREADY_HAVE_CHARM = 0x04;    
    ALREADY_HAVE_SUMMON = 0x05;    
    ALREADY_OPEN = 0x06;    
    MORE_POWERFUL_SPELL_ACTIVE = 0x07;    
    BAD_IMPLICIT_TARGETS = 0x09;    
    BAD_TARGETS = 0x0A;    
    CANT_BE_CHARMED = 0x0B;    
    CANT_BE_DISENCHANTED = 0x0C;    
    CANT_BE_PROSPECTED = 0x0D;    
    CANT_CAST_ON_TAPPED = 0x0E;    
    CANT_DUEL_WHILE_INVISIBLE = 0x0F;    
    CANT_DUEL_WHILE_STEALTHED = 0x10;    
    CANT_TOO_CLOSE_TO_ENEMY = 0x11;    
    CANT_DO_THAT_YET = 0x12;    
    CASTER_DEAD = 0x13;    
    CHARMED = 0x14;    
    CHEST_IN_USE = 0x15;    
    CONFUSED = 0x16;    
    DONT_REPORT = 0x17;    
    EQUIPPED_ITEM = 0x18;    
    EQUIPPED_ITEM_CLASS = 0x19;    
    EQUIPPED_ITEM_CLASS_MAINHAND = 0x1A;    
    EQUIPPED_ITEM_CLASS_OFFHAND = 0x1B;    
    ERROR = 0x1C;    
    FIZZLE = 0x1D;    
    FLEEING = 0x1E;    
    FOOD_LOWLEVEL = 0x1F;    
    HIGHLEVEL = 0x20;    
    IMMUNE = 0x22;    
    INTERRUPTED = 0x23;    
    INTERRUPTED_COMBAT = 0x24;    
    ITEM_ALREADY_ENCHANTED = 0x25;    
    ITEM_GONE = 0x26;    
    ENCHANT_NOT_EXISTING_ITEM = 0x27;    
    ITEM_NOT_READY = 0x28;    
    LEVEL_REQUIREMENT = 0x29;    
    LINE_OF_SIGHT = 0x2A;    
    LOWLEVEL = 0x2B;    
    SKILL_NOT_HIGH_ENOUGH = 0x2C;    
    MAINHAND_EMPTY = 0x2D;    
    MOVING = 0x2E;    
    NEED_AMMO = 0x2F;    
    NEED_REQUIRES_SOMETHING = 0x30;    
    NEED_EXOTIC_AMMO = 0x31;    
    NOPATH = 0x32;    
    NOT_BEHIND = 0x33;    
    NOT_FISHABLE = 0x34;    
    NOT_HERE = 0x35;    
    NOT_INFRONT = 0x36;    
    NOT_IN_CONTROL = 0x37;    
    NOT_KNOWN = 0x38;    
    NOT_MOUNTED = 0x39;    
    NOT_ON_TAXI = 0x3A;    
    NOT_ON_TRANSPORT = 0x3B;    
    NOT_READY = 0x3C;    
    NOT_SHAPESHIFT = 0x3D;    
    NOT_STANDING = 0x3E;    
    NOT_TRADEABLE = 0x3F;    
    NOT_TRADING = 0x40;    
    NOT_UNSHEATHED = 0x41;    
    NOT_WHILE_GHOST = 0x42;    
    NO_AMMO = 0x43;    
    NO_CHARGES_REMAIN = 0x44;    
    NO_CHAMPION = 0x45;    
    NO_COMBO_POINTS = 0x46;    
    NO_DUELING = 0x47;    
    NO_ENDURANCE = 0x48;    
    NO_FISH = 0x49;    
    NO_ITEMS_WHILE_SHAPESHIFTED = 0x4A;    
    NO_MOUNTS_ALLOWED = 0x4B;    
    NO_PET = 0x4C;    
    NO_POWER = 0x4D;    
    NOTHING_TO_DISPEL = 0x4E;    
    NOTHING_TO_STEAL = 0x4F;    
    ONLY_ABOVEWATER = 0x50;    
    ONLY_DAYTIME = 0x51;    
    ONLY_INDOORS = 0x52;    
    ONLY_MOUNTED = 0x53;    
    ONLY_NIGHTTIME = 0x54;    
    ONLY_OUTDOORS = 0x55;    
    ONLY_SHAPESHIFT = 0x56;    
    ONLY_STEALTHED = 0x57;    
    ONLY_UNDERWATER = 0x58;    
    OUT_OF_RANGE = 0x59;    
    PACIFIED = 0x5A;    
    POSSESSED = 0x5B;    
    REQUIRES_AREA = 0x5D;    
    REQUIRES_SPELL_FOCUS = 0x5E;    
    ROOTED = 0x5F;    
    SILENCED = 0x60;    
    SPELL_IN_PROGRESS = 0x61;    
    SPELL_LEARNED = 0x62;    
    SPELL_UNAVAILABLE = 0x63;    
    STUNNED = 0x64;    
    TARGETS_DEAD = 0x65;    
    TARGET_AFFECTING_COMBAT = 0x66;    
    TARGET_AURASTATE = 0x67;    
    TARGET_DUELING = 0x68;    
    TARGET_ENEMY = 0x69;    
    TARGET_ENRAGED = 0x6A;    
    TARGET_FRIENDLY = 0x6B;    
    TARGET_IN_COMBAT = 0x6C;    
    TARGET_IS_PLAYER = 0x6D;    
    TARGET_NOT_DEAD = 0x6E;    
    TARGET_NOT_IN_PARTY = 0x6F;    
    TARGET_NOT_LOOTED = 0x70;    
    TARGET_NOT_PLAYER = 0x71;    
    TARGET_NO_POCKETS = 0x72;    
    TARGET_NO_WEAPONS = 0x73;    
    TARGET_UNSKINNABLE = 0x74;    
    THIRST_SATIATED = 0x75;    
    TOO_CLOSE = 0x76;    
    TOO_MANY_OF_ITEM = 0x77;    
    TRAINING_POINTS = 0x79;    
    TRY_AGAIN = 0x7A;    
    UNIT_NOT_BEHIND = 0x7B;    
    UNIT_NOT_INFRONT = 0x7C;    
    WRONG_PET_FOOD = 0x7D;    
    NOT_WHILE_FATIGUED = 0x7E;    
    TARGET_NOT_IN_INSTANCE = 0x7F;    
    NOT_WHILE_TRADING = 0x80;    
    TARGET_NOT_IN_RAID = 0x81;    
    DISENCHANT_WHILE_LOOTING = 0x82;    
    PROSPECT_WHILE_LOOTING = 0x83;    
    TARGET_FREEFORALL = 0x85;    
    NO_EDIBLE_CORPSES = 0x86;    
    ONLY_BATTLEGROUNDS = 0x87;    
    TARGET_NOT_GHOST = 0x88;    
    TOO_MANY_SKILLS = 0x89;    
    CANT_USE_NEW_ITEM = 0x8A;    
    WRONG_WEATHER = 0x8B;    
    DAMAGE_IMMUNE = 0x8C;    
    PREVENTED_BY_MECHANIC = 0x8D;    
    PLAY_TIME = 0x8E;    
    REPUTATION = 0x8F;    
    MIN_SKILL = 0x90;    
    UNKNOWN = 0x91;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| AFFECTING_COMBAT | 0x00 | 0 | 0x0 |  |  |
| ALREADY_AT_FULL_HEALTH | 0x01 | 1 | 0x1 |  |  |
| ALREADY_AT_FULL_MANA | 0x02 | 2 | 0x2 |  |  |
| ALREADY_BEING_TAMED | 0x03 | 3 | 0x3 |  |  |
| ALREADY_HAVE_CHARM | 0x04 | 4 | 0x4 |  |  |
| ALREADY_HAVE_SUMMON | 0x05 | 5 | 0x5 |  |  |
| ALREADY_OPEN | 0x06 | 6 | 0x6 |  |  |
| MORE_POWERFUL_SPELL_ACTIVE | 0x07 | 7 | 0x7 |  |  |
| BAD_IMPLICIT_TARGETS | 0x09 | 9 | 0x9 |  |  |
| BAD_TARGETS | 0x0A | 10 | 0xA |  |  |
| CANT_BE_CHARMED | 0x0B | 11 | 0xB |  |  |
| CANT_BE_DISENCHANTED | 0x0C | 12 | 0xC |  |  |
| CANT_BE_PROSPECTED | 0x0D | 13 | 0xD |  |  |
| CANT_CAST_ON_TAPPED | 0x0E | 14 | 0xE |  |  |
| CANT_DUEL_WHILE_INVISIBLE | 0x0F | 15 | 0xF |  |  |
| CANT_DUEL_WHILE_STEALTHED | 0x10 | 16 | 0x10 |  |  |
| CANT_TOO_CLOSE_TO_ENEMY | 0x11 | 17 | 0x11 |  |  |
| CANT_DO_THAT_YET | 0x12 | 18 | 0x12 |  |  |
| CASTER_DEAD | 0x13 | 19 | 0x13 |  |  |
| CHARMED | 0x14 | 20 | 0x14 |  |  |
| CHEST_IN_USE | 0x15 | 21 | 0x15 |  |  |
| CONFUSED | 0x16 | 22 | 0x16 |  |  |
| DONT_REPORT | 0x17 | 23 | 0x17 |  |  |
| EQUIPPED_ITEM | 0x18 | 24 | 0x18 |  |  |
| EQUIPPED_ITEM_CLASS | 0x19 | 25 | 0x19 |  |  |
| EQUIPPED_ITEM_CLASS_MAINHAND | 0x1A | 26 | 0x1A |  |  |
| EQUIPPED_ITEM_CLASS_OFFHAND | 0x1B | 27 | 0x1B |  |  |
| ERROR | 0x1C | 28 | 0x1C |  |  |
| FIZZLE | 0x1D | 29 | 0x1D |  |  |
| FLEEING | 0x1E | 30 | 0x1E |  |  |
| FOOD_LOWLEVEL | 0x1F | 31 | 0x1F |  |  |
| HIGHLEVEL | 0x20 | 32 | 0x20 |  |  |
| IMMUNE | 0x22 | 34 | 0x22 |  |  |
| INTERRUPTED | 0x23 | 35 | 0x23 |  |  |
| INTERRUPTED_COMBAT | 0x24 | 36 | 0x24 |  |  |
| ITEM_ALREADY_ENCHANTED | 0x25 | 37 | 0x25 |  |  |
| ITEM_GONE | 0x26 | 38 | 0x26 |  |  |
| ENCHANT_NOT_EXISTING_ITEM | 0x27 | 39 | 0x27 |  |  |
| ITEM_NOT_READY | 0x28 | 40 | 0x28 |  |  |
| LEVEL_REQUIREMENT | 0x29 | 41 | 0x29 |  |  |
| LINE_OF_SIGHT | 0x2A | 42 | 0x2A |  |  |
| LOWLEVEL | 0x2B | 43 | 0x2B |  |  |
| SKILL_NOT_HIGH_ENOUGH | 0x2C | 44 | 0x2C |  |  |
| MAINHAND_EMPTY | 0x2D | 45 | 0x2D |  |  |
| MOVING | 0x2E | 46 | 0x2E |  |  |
| NEED_AMMO | 0x2F | 47 | 0x2F |  |  |
| NEED_REQUIRES_SOMETHING | 0x30 | 48 | 0x30 |  |  |
| NEED_EXOTIC_AMMO | 0x31 | 49 | 0x31 |  |  |
| NOPATH | 0x32 | 50 | 0x32 |  |  |
| NOT_BEHIND | 0x33 | 51 | 0x33 |  |  |
| NOT_FISHABLE | 0x34 | 52 | 0x34 |  |  |
| NOT_HERE | 0x35 | 53 | 0x35 |  |  |
| NOT_INFRONT | 0x36 | 54 | 0x36 |  |  |
| NOT_IN_CONTROL | 0x37 | 55 | 0x37 |  |  |
| NOT_KNOWN | 0x38 | 56 | 0x38 |  |  |
| NOT_MOUNTED | 0x39 | 57 | 0x39 |  |  |
| NOT_ON_TAXI | 0x3A | 58 | 0x3A |  |  |
| NOT_ON_TRANSPORT | 0x3B | 59 | 0x3B |  |  |
| NOT_READY | 0x3C | 60 | 0x3C |  |  |
| NOT_SHAPESHIFT | 0x3D | 61 | 0x3D |  |  |
| NOT_STANDING | 0x3E | 62 | 0x3E |  |  |
| NOT_TRADEABLE | 0x3F | 63 | 0x3F |  | rogues trying 'enchant' other's weapon with poison |
| NOT_TRADING | 0x40 | 64 | 0x40 |  |  |
| NOT_UNSHEATHED | 0x41 | 65 | 0x41 |  |  |
| NOT_WHILE_GHOST | 0x42 | 66 | 0x42 |  |  |
| NO_AMMO | 0x43 | 67 | 0x43 |  |  |
| NO_CHARGES_REMAIN | 0x44 | 68 | 0x44 |  |  |
| NO_CHAMPION | 0x45 | 69 | 0x45 |  |  |
| NO_COMBO_POINTS | 0x46 | 70 | 0x46 |  |  |
| NO_DUELING | 0x47 | 71 | 0x47 |  |  |
| NO_ENDURANCE | 0x48 | 72 | 0x48 |  |  |
| NO_FISH | 0x49 | 73 | 0x49 |  |  |
| NO_ITEMS_WHILE_SHAPESHIFTED | 0x4A | 74 | 0x4A |  |  |
| NO_MOUNTS_ALLOWED | 0x4B | 75 | 0x4B |  |  |
| NO_PET | 0x4C | 76 | 0x4C |  |  |
| NO_POWER | 0x4D | 77 | 0x4D |  |  |
| NOTHING_TO_DISPEL | 0x4E | 78 | 0x4E |  |  |
| NOTHING_TO_STEAL | 0x4F | 79 | 0x4F |  |  |
| ONLY_ABOVEWATER | 0x50 | 80 | 0x50 |  |  |
| ONLY_DAYTIME | 0x51 | 81 | 0x51 |  |  |
| ONLY_INDOORS | 0x52 | 82 | 0x52 |  |  |
| ONLY_MOUNTED | 0x53 | 83 | 0x53 |  |  |
| ONLY_NIGHTTIME | 0x54 | 84 | 0x54 |  |  |
| ONLY_OUTDOORS | 0x55 | 85 | 0x55 |  |  |
| ONLY_SHAPESHIFT | 0x56 | 86 | 0x56 |  |  |
| ONLY_STEALTHED | 0x57 | 87 | 0x57 |  |  |
| ONLY_UNDERWATER | 0x58 | 88 | 0x58 |  |  |
| OUT_OF_RANGE | 0x59 | 89 | 0x59 |  |  |
| PACIFIED | 0x5A | 90 | 0x5A |  |  |
| POSSESSED | 0x5B | 91 | 0x5B |  |  |
| REQUIRES_AREA | 0x5D | 93 | 0x5D |  |  |
| REQUIRES_SPELL_FOCUS | 0x5E | 94 | 0x5E |  |  |
| ROOTED | 0x5F | 95 | 0x5F |  |  |
| SILENCED | 0x60 | 96 | 0x60 |  |  |
| SPELL_IN_PROGRESS | 0x61 | 97 | 0x61 |  |  |
| SPELL_LEARNED | 0x62 | 98 | 0x62 |  |  |
| SPELL_UNAVAILABLE | 0x63 | 99 | 0x63 |  |  |
| STUNNED | 0x64 | 100 | 0x64 |  |  |
| TARGETS_DEAD | 0x65 | 101 | 0x65 |  |  |
| TARGET_AFFECTING_COMBAT | 0x66 | 102 | 0x66 |  |  |
| TARGET_AURASTATE | 0x67 | 103 | 0x67 |  |  |
| TARGET_DUELING | 0x68 | 104 | 0x68 |  |  |
| TARGET_ENEMY | 0x69 | 105 | 0x69 |  |  |
| TARGET_ENRAGED | 0x6A | 106 | 0x6A |  |  |
| TARGET_FRIENDLY | 0x6B | 107 | 0x6B |  |  |
| TARGET_IN_COMBAT | 0x6C | 108 | 0x6C |  |  |
| TARGET_IS_PLAYER | 0x6D | 109 | 0x6D |  |  |
| TARGET_NOT_DEAD | 0x6E | 110 | 0x6E |  |  |
| TARGET_NOT_IN_PARTY | 0x6F | 111 | 0x6F |  |  |
| TARGET_NOT_LOOTED | 0x70 | 112 | 0x70 |  |  |
| TARGET_NOT_PLAYER | 0x71 | 113 | 0x71 |  |  |
| TARGET_NO_POCKETS | 0x72 | 114 | 0x72 |  |  |
| TARGET_NO_WEAPONS | 0x73 | 115 | 0x73 |  |  |
| TARGET_UNSKINNABLE | 0x74 | 116 | 0x74 |  |  |
| THIRST_SATIATED | 0x75 | 117 | 0x75 |  |  |
| TOO_CLOSE | 0x76 | 118 | 0x76 |  |  |
| TOO_MANY_OF_ITEM | 0x77 | 119 | 0x77 |  |  |
| TRAINING_POINTS | 0x79 | 121 | 0x79 |  |  |
| TRY_AGAIN | 0x7A | 122 | 0x7A |  |  |
| UNIT_NOT_BEHIND | 0x7B | 123 | 0x7B |  |  |
| UNIT_NOT_INFRONT | 0x7C | 124 | 0x7C |  |  |
| WRONG_PET_FOOD | 0x7D | 125 | 0x7D |  |  |
| NOT_WHILE_FATIGUED | 0x7E | 126 | 0x7E |  |  |
| TARGET_NOT_IN_INSTANCE | 0x7F | 127 | 0x7F |  |  |
| NOT_WHILE_TRADING | 0x80 | 128 | 0x80 |  |  |
| TARGET_NOT_IN_RAID | 0x81 | 129 | 0x81 |  |  |
| DISENCHANT_WHILE_LOOTING | 0x82 | 130 | 0x82 |  |  |
| PROSPECT_WHILE_LOOTING | 0x83 | 131 | 0x83 |  |  |
| TARGET_FREEFORALL | 0x85 | 133 | 0x85 |  |  |
| NO_EDIBLE_CORPSES | 0x86 | 134 | 0x86 |  |  |
| ONLY_BATTLEGROUNDS | 0x87 | 135 | 0x87 |  |  |
| TARGET_NOT_GHOST | 0x88 | 136 | 0x88 |  |  |
| TOO_MANY_SKILLS | 0x89 | 137 | 0x89 |  |  |
| CANT_USE_NEW_ITEM | 0x8A | 138 | 0x8A |  |  |
| WRONG_WEATHER | 0x8B | 139 | 0x8B |  |  |
| DAMAGE_IMMUNE | 0x8C | 140 | 0x8C |  |  |
| PREVENTED_BY_MECHANIC | 0x8D | 141 | 0x8D |  |  |
| PLAY_TIME | 0x8E | 142 | 0x8E |  |  |
| REPUTATION | 0x8F | 143 | 0x8F |  |  |
| MIN_SKILL | 0x90 | 144 | 0x90 |  |  |
| UNKNOWN | 0x91 | 145 | 0x91 |  |  |
