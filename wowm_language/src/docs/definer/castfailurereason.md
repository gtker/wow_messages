## Client Version 1.12

## Wowm Representation
```rust,ignore
enum CastFailureReason : u8 {
    AFFECTING_COMBAT = 0x00;    
    ALREADY_AT_FULL_HEALTH = 0x01;    
    ALREADY_AT_FULL_POWER = 0x02;    
    ALREADY_BEING_TAMED = 0x03;    
    ALREADY_HAVE_CHARM = 0x04;    
    ALREADY_HAVE_SUMMON = 0x05;    
    ALREADY_OPEN = 0x06;    
    AURA_BOUNCED = 0x07;    
    AUTOTRACK_INTERRUPTED = 0x08;    
    BAD_IMPLICIT_TARGETS = 0x09;    
    BAD_TARGETS = 0x0a;    
    CANT_BE_CHARMED = 0x0b;    
    CANT_BE_DISENCHANTED = 0x0c;    
    CANT_BE_PROSPECTED = 0x0d;    
    CANT_CAST_ON_TAPPED = 0x0e;    
    CANT_DUEL_WHILE_INVISIBLE = 0x0f;    
    CANT_DUEL_WHILE_STEALTHED = 0x10;    
    CANT_STEALTH = 0x11;    
    CASTER_AURASTATE = 0x12;    
    CASTER_DEAD = 0x13;    
    CHARMED = 0x14;    
    CHEST_IN_USE = 0x15;    
    CONFUSED = 0x16;    
    DONT_REPORT = 0x17;    
    EQUIPPED_ITEM = 0x18;    
    EQUIPPED_ITEM_CLASS = 0x19;    
    EQUIPPED_ITEM_CLASS_MAINHAND = 0x1a;    
    EQUIPPED_ITEM_CLASS_OFFHAND = 0x1b;    
    ERROR = 0x1c;    
    FIZZLE = 0x1d;    
    FLEEING = 0x1e;    
    FOOD_LOWLEVEL = 0x1f;    
    HIGHLEVEL = 0x20;    
    HUNGER_SATIATED = 0x21;    
    IMMUNE = 0x22;    
    INTERRUPTED = 0x23;    
    INTERRUPTED_COMBAT = 0x24;    
    ITEM_ALREADY_ENCHANTED = 0x25;    
    ITEM_GONE = 0x26;    
    ITEM_NOT_FOUND = 0x27;    
    ITEM_NOT_READY = 0x28;    
    LEVEL_REQUIREMENT = 0x29;    
    LINE_OF_SIGHT = 0x2a;    
    LOWLEVEL = 0x2b;    
    LOW_CASTLEVEL = 0x2c;    
    MAINHAND_EMPTY = 0x2d;    
    MOVING = 0x2e;    
    NEED_AMMO = 0x2f;    
    NEED_AMMO_POUCH = 0x30;    
    NEED_EXOTIC_AMMO = 0x31;    
    NOPATH = 0x32;    
    NOT_BEHIND = 0x33;    
    NOT_FISHABLE = 0x34;    
    NOT_HERE = 0x35;    
    NOT_INFRONT = 0x36;    
    NOT_IN_CONTROL = 0x37;    
    NOT_KNOWN = 0x38;    
    NOT_MOUNTED = 0x39;    
    NOT_ON_TAXI = 0x3a;    
    NOT_ON_TRANSPORT = 0x3b;    
    NOT_READY = 0x3c;    
    NOT_SHAPESHIFT = 0x3d;    
    NOT_STANDING = 0x3e;    
    NOT_TRADEABLE = 0x3f;    
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
    NO_ITEMS_WHILE_SHAPESHIFTED = 0x4a;    
    NO_MOUNTS_ALLOWED = 0x4b;    
    NO_PET = 0x4c;    
    NO_POWER = 0x4d;    
    NOTHING_TO_DISPEL = 0x4e;    
    NOTHING_TO_STEAL = 0x4f;    
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
    PACIFIED = 0x5a;    
    POSSESSED = 0x5b;    
    REAGENTS = 0x5c;    
    REQUIRES_AREA = 0x5d;    
    REQUIRES_SPELL_FOCUS = 0x5e;    
    ROOTED = 0x5f;    
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
    TARGET_ENRAGED = 0x6a;    
    TARGET_FRIENDLY = 0x6b;    
    TARGET_IN_COMBAT = 0x6c;    
    TARGET_IS_PLAYER = 0x6d;    
    TARGET_NOT_DEAD = 0x6e;    
    TARGET_NOT_IN_PARTY = 0x6f;    
    TARGET_NOT_LOOTED = 0x70;    
    TARGET_NOT_PLAYER = 0x71;    
    TARGET_NO_POCKETS = 0x72;    
    TARGET_NO_WEAPONS = 0x73;    
    TARGET_UNSKINNABLE = 0x74;    
    THIRST_SATIATED = 0x75;    
    TOO_CLOSE = 0x76;    
    TOO_MANY_OF_ITEM = 0x77;    
    TOTEMS = 0x78;    
    TRAINING_POINTS = 0x79;    
    TRY_AGAIN = 0x7a;    
    UNIT_NOT_BEHIND = 0x7b;    
    UNIT_NOT_INFRONT = 0x7c;    
    WRONG_PET_FOOD = 0x7d;    
    NOT_WHILE_FATIGUED = 0x7e;    
    TARGET_NOT_IN_INSTANCE = 0x7f;    
    NOT_WHILE_TRADING = 0x80;    
    TARGET_NOT_IN_RAID = 0x81;    
    DISENCHANT_WHILE_LOOTING = 0x82;    
    PROSPECT_WHILE_LOOTING = 0x83;    
    PROSPECT_NEED_MORE = 0x84;    
    TARGET_FREEFORALL = 0x85;    
    NO_EDIBLE_CORPSES = 0x86;    
    ONLY_BATTLEGROUNDS = 0x87;    
    TARGET_NOT_GHOST = 0x88;    
    TOO_MANY_SKILLS = 0x89;    
    TRANSFORM_UNUSABLE = 0x8a;    
    WRONG_WEATHER = 0x8b;    
    DAMAGE_IMMUNE = 0x8c;    
    PREVENTED_BY_MECHANIC = 0x8d;    
    PLAY_TIME = 0x8e;    
    REPUTATION = 0x8f;    
    MIN_SKILL = 0x90;    
    UNKNOWN = 0x91;    
}

```
## Enumerators
| Enumerator | Original | Decimal Value | Hex Value | Description | Comment |
| --------- | -------- | ------------- | --------- | ----------- | ------- |
| AFFECTING_COMBAT | 0x00 | 0 | 0x0 |  | You are in combat |
| ALREADY_AT_FULL_HEALTH | 0x01 | 1 | 0x1 |  | You are already at full Health. |
| ALREADY_AT_FULL_POWER | 0x02 | 2 | 0x2 |  | You are already at full %s. |
| ALREADY_BEING_TAMED | 0x03 | 3 | 0x3 |  | That creature is already being tamed |
| ALREADY_HAVE_CHARM | 0x04 | 4 | 0x4 |  | You already control a charmed creature |
| ALREADY_HAVE_SUMMON | 0x05 | 5 | 0x5 |  | You already control a summoned creature |
| ALREADY_OPEN | 0x06 | 6 | 0x6 |  | Already open |
| AURA_BOUNCED | 0x07 | 7 | 0x7 |  | A more powerful spell is already active |
| AUTOTRACK_INTERRUPTED | 0x08 | 8 | 0x8 |  | Message is hidden/unused |
| BAD_IMPLICIT_TARGETS | 0x09 | 9 | 0x9 |  | You have no target. |
| BAD_TARGETS | 0x0a | 10 | 0xA |  | Invalid target |
| CANT_BE_CHARMED | 0x0b | 11 | 0xB |  | Target can't be charmed |
| CANT_BE_DISENCHANTED | 0x0c | 12 | 0xC |  | Item cannot be disenchanted |
| CANT_BE_PROSPECTED | 0x0d | 13 | 0xD |  | There are no gems in this |
| CANT_CAST_ON_TAPPED | 0x0e | 14 | 0xE |  | Target is tapped |
| CANT_DUEL_WHILE_INVISIBLE | 0x0f | 15 | 0xF |  | You can't start a duel while invisible |
| CANT_DUEL_WHILE_STEALTHED | 0x10 | 16 | 0x10 |  | You can't start a duel while stealthed |
| CANT_STEALTH | 0x11 | 17 | 0x11 |  | You are too close to enemies |
| CASTER_AURASTATE | 0x12 | 18 | 0x12 |  | You can't do that yet |
| CASTER_DEAD | 0x13 | 19 | 0x13 |  | You are dead |
| CHARMED | 0x14 | 20 | 0x14 |  | Can't do that while charmed |
| CHEST_IN_USE | 0x15 | 21 | 0x15 |  | That is already being used |
| CONFUSED | 0x16 | 22 | 0x16 |  | Can't do that while confused |
| DONT_REPORT | 0x17 | 23 | 0x17 |  | Message is hidden/unused |
| EQUIPPED_ITEM | 0x18 | 24 | 0x18 |  | Must have the proper item equipped |
| EQUIPPED_ITEM_CLASS | 0x19 | 25 | 0x19 |  | Must have a %s equipped |
| EQUIPPED_ITEM_CLASS_MAINHAND | 0x1a | 26 | 0x1A |  | Must have a %s equipped in the main hand |
| EQUIPPED_ITEM_CLASS_OFFHAND | 0x1b | 27 | 0x1B |  | Must have a %s equipped in the offhand |
| ERROR | 0x1c | 28 | 0x1C |  | Internal error |
| FIZZLE | 0x1d | 29 | 0x1D |  | Fizzled |
| FLEEING | 0x1e | 30 | 0x1E |  | Can't do that while fleeing |
| FOOD_LOWLEVEL | 0x1f | 31 | 0x1F |  | That food's level is not high enough for your pet |
| HIGHLEVEL | 0x20 | 32 | 0x20 |  | Target is too high level |
| HUNGER_SATIATED | 0x21 | 33 | 0x21 |  | Message is hidden/unused |
| IMMUNE | 0x22 | 34 | 0x22 |  | Immune |
| INTERRUPTED | 0x23 | 35 | 0x23 |  | Interrupted |
| INTERRUPTED_COMBAT | 0x24 | 36 | 0x24 |  | Interrupted |
| ITEM_ALREADY_ENCHANTED | 0x25 | 37 | 0x25 |  | Item is already enchanted |
| ITEM_GONE | 0x26 | 38 | 0x26 |  | Item is gone |
| ITEM_NOT_FOUND | 0x27 | 39 | 0x27 |  | Tried to enchant an item that didn't exist |
| ITEM_NOT_READY | 0x28 | 40 | 0x28 |  | Item is not ready yet. |
| LEVEL_REQUIREMENT | 0x29 | 41 | 0x29 |  | You are not high enough level |
| LINE_OF_SIGHT | 0x2a | 42 | 0x2A |  | Target not in line of sight |
| LOWLEVEL | 0x2b | 43 | 0x2B |  | Target is too low level |
| LOW_CASTLEVEL | 0x2c | 44 | 0x2C |  | Skill not high enough |
| MAINHAND_EMPTY | 0x2d | 45 | 0x2D |  | Your weapon hand is empty |
| MOVING | 0x2e | 46 | 0x2E |  | Can't do that while moving |
| NEED_AMMO | 0x2f | 47 | 0x2F |  | Ammo needs to be in the paper doll ammo slot before it can be fired |
| NEED_AMMO_POUCH | 0x30 | 48 | 0x30 |  | Requires: %s |
| NEED_EXOTIC_AMMO | 0x31 | 49 | 0x31 |  | Requires exotic ammo: %s |
| NOPATH | 0x32 | 50 | 0x32 |  | No path available |
| NOT_BEHIND | 0x33 | 51 | 0x33 |  | You must be behind your target |
| NOT_FISHABLE | 0x34 | 52 | 0x34 |  | Your cast didn't land in fishable water |
| NOT_HERE | 0x35 | 53 | 0x35 |  | You can't use that here |
| NOT_INFRONT | 0x36 | 54 | 0x36 |  | You must be in front of your target |
| NOT_IN_CONTROL | 0x37 | 55 | 0x37 |  | You are not in control of your actions |
| NOT_KNOWN | 0x38 | 56 | 0x38 |  | Spell not learned |
| NOT_MOUNTED | 0x39 | 57 | 0x39 |  | You are mounted |
| NOT_ON_TAXI | 0x3a | 58 | 0x3A |  | You are in flight |
| NOT_ON_TRANSPORT | 0x3b | 59 | 0x3B |  | You are on a transport |
| NOT_READY | 0x3c | 60 | 0x3C |  | Spell is not ready yet. |
| NOT_SHAPESHIFT | 0x3d | 61 | 0x3D |  | You are in shapeshift form |
| NOT_STANDING | 0x3e | 62 | 0x3E |  | You must be standing to do that |
| NOT_TRADEABLE | 0x3f | 63 | 0x3F |  | You can only use this on an object you own |
| NOT_TRADING | 0x40 | 64 | 0x40 |  | Tried to enchant a trade item, but not trading |
| NOT_UNSHEATHED | 0x41 | 65 | 0x41 |  | You have to be unsheathed to do that! |
| NOT_WHILE_GHOST | 0x42 | 66 | 0x42 |  | Can't cast as ghost |
| NO_AMMO | 0x43 | 67 | 0x43 |  | Out of ammo |
| NO_CHARGES_REMAIN | 0x44 | 68 | 0x44 |  | No charges remain |
| NO_CHAMPION | 0x45 | 69 | 0x45 |  | You haven't selected a champion |
| NO_COMBO_POINTS | 0x46 | 70 | 0x46 |  | That ability requires combo points |
| NO_DUELING | 0x47 | 71 | 0x47 |  | Dueling isn't allowed here |
| NO_ENDURANCE | 0x48 | 72 | 0x48 |  | Not enough endurance |
| NO_FISH | 0x49 | 73 | 0x49 |  | There aren't any fish here |
| NO_ITEMS_WHILE_SHAPESHIFTED | 0x4a | 74 | 0x4A |  | Can't use items while shapeshifted |
| NO_MOUNTS_ALLOWED | 0x4b | 75 | 0x4B |  | You can't mount here |
| NO_PET | 0x4c | 76 | 0x4C |  | You do not have a pet |
| NO_POWER | 0x4d | 77 | 0x4D |  | Dynamic pre-defined messages, no args: Not enough mana, Not enough rage, etc |
| NOTHING_TO_DISPEL | 0x4e | 78 | 0x4E |  | Nothing to dispel |
| NOTHING_TO_STEAL | 0x4f | 79 | 0x4F |  | Nothing to steal |
| ONLY_ABOVEWATER | 0x50 | 80 | 0x50 |  | Cannot use while swimming |
| ONLY_DAYTIME | 0x51 | 81 | 0x51 |  | Can only use during the day |
| ONLY_INDOORS | 0x52 | 82 | 0x52 |  | Can only use indoors |
| ONLY_MOUNTED | 0x53 | 83 | 0x53 |  | Can only use while mounted |
| ONLY_NIGHTTIME | 0x54 | 84 | 0x54 |  | Can only use during the night |
| ONLY_OUTDOORS | 0x55 | 85 | 0x55 |  | Can only use outside |
| ONLY_SHAPESHIFT | 0x56 | 86 | 0x56 |  | Must be in %s |
| ONLY_STEALTHED | 0x57 | 87 | 0x57 |  | You must be in stealth mode |
| ONLY_UNDERWATER | 0x58 | 88 | 0x58 |  | Can only use while swimming |
| OUT_OF_RANGE | 0x59 | 89 | 0x59 |  | Out of range. |
| PACIFIED | 0x5a | 90 | 0x5A |  | Can't use that ability while pacified |
| POSSESSED | 0x5b | 91 | 0x5B |  | You are possessed |
| REAGENTS | 0x5c | 92 | 0x5C |  | Message is hidden/unused, supposedly implemented client-side only |
| REQUIRES_AREA | 0x5d | 93 | 0x5D |  | You need to be in %s |
| REQUIRES_SPELL_FOCUS | 0x5e | 94 | 0x5E |  | Requires %s |
| ROOTED | 0x5f | 95 | 0x5F |  | You are unable to move |
| SILENCED | 0x60 | 96 | 0x60 |  | Can't do that while silenced |
| SPELL_IN_PROGRESS | 0x61 | 97 | 0x61 |  | Another action is in progress |
| SPELL_LEARNED | 0x62 | 98 | 0x62 |  | You have already learned the spell |
| SPELL_UNAVAILABLE | 0x63 | 99 | 0x63 |  | The spell is not available to you |
| STUNNED | 0x64 | 100 | 0x64 |  | Can't do that while stunned |
| TARGETS_DEAD | 0x65 | 101 | 0x65 |  | Your target is dead |
| TARGET_AFFECTING_COMBAT | 0x66 | 102 | 0x66 |  | Target is in combat |
| TARGET_AURASTATE | 0x67 | 103 | 0x67 |  | You can't do that yet |
| TARGET_DUELING | 0x68 | 104 | 0x68 |  | Target is currently dueling |
| TARGET_ENEMY | 0x69 | 105 | 0x69 |  | Target is hostile |
| TARGET_ENRAGED | 0x6a | 106 | 0x6A |  | Target is too enraged to be charmed |
| TARGET_FRIENDLY | 0x6b | 107 | 0x6B |  | Target is friendly |
| TARGET_IN_COMBAT | 0x6c | 108 | 0x6C |  | The target can't be in combat |
| TARGET_IS_PLAYER | 0x6d | 109 | 0x6D |  | Can't target players |
| TARGET_NOT_DEAD | 0x6e | 110 | 0x6E |  | Target is alive |
| TARGET_NOT_IN_PARTY | 0x6f | 111 | 0x6F |  | Target is not in your party |
| TARGET_NOT_LOOTED | 0x70 | 112 | 0x70 |  | Creature must be looted first |
| TARGET_NOT_PLAYER | 0x71 | 113 | 0x71 |  | Target is not a player |
| TARGET_NO_POCKETS | 0x72 | 114 | 0x72 |  | No pockets to pick |
| TARGET_NO_WEAPONS | 0x73 | 115 | 0x73 |  | Target has no weapons equipped |
| TARGET_UNSKINNABLE | 0x74 | 116 | 0x74 |  | Creature is not skinnable |
| THIRST_SATIATED | 0x75 | 117 | 0x75 |  | Message is hidden/unused |
| TOO_CLOSE | 0x76 | 118 | 0x76 |  | Target too close |
| TOO_MANY_OF_ITEM | 0x77 | 119 | 0x77 |  | You have too many of that item already |
| TOTEMS | 0x78 | 120 | 0x78 |  | Message is hidden/unused, supposedly implemented client-side only |
| TRAINING_POINTS | 0x79 | 121 | 0x79 |  | Not enough training points |
| TRY_AGAIN | 0x7a | 122 | 0x7A |  | Failed attempt |
| UNIT_NOT_BEHIND | 0x7b | 123 | 0x7B |  | Target needs to be behind you |
| UNIT_NOT_INFRONT | 0x7c | 124 | 0x7C |  | Target needs to be in front of you |
| WRONG_PET_FOOD | 0x7d | 125 | 0x7D |  | Your pet doesn't like that food |
| NOT_WHILE_FATIGUED | 0x7e | 126 | 0x7E |  | Can't cast while fatigued |
| TARGET_NOT_IN_INSTANCE | 0x7f | 127 | 0x7F |  | Target must be in this instance |
| NOT_WHILE_TRADING | 0x80 | 128 | 0x80 |  | Can't cast while trading |
| TARGET_NOT_IN_RAID | 0x81 | 129 | 0x81 |  | Target is not in your party or raid group |
| DISENCHANT_WHILE_LOOTING | 0x82 | 130 | 0x82 |  | Cannot disenchant while looting |
| PROSPECT_WHILE_LOOTING | 0x83 | 131 | 0x83 |  | Cannot prospect while looting |
| PROSPECT_NEED_MORE | 0x84 | 132 | 0x84 |  | Message is hidden/unused, supposedly implemented client-side only |
| TARGET_FREEFORALL | 0x85 | 133 | 0x85 |  | Target is currently in free-for-all PvP combat |
| NO_EDIBLE_CORPSES | 0x86 | 134 | 0x86 |  | There are no nearby corpses to eat |
| ONLY_BATTLEGROUNDS | 0x87 | 135 | 0x87 |  | Can only use in battlegrounds |
| TARGET_NOT_GHOST | 0x88 | 136 | 0x88 |  | Target is not a ghost |
| TOO_MANY_SKILLS | 0x89 | 137 | 0x89 |  | Your pet can't learn any more skills |
| TRANSFORM_UNUSABLE | 0x8a | 138 | 0x8A |  | You can't use the new item |
| WRONG_WEATHER | 0x8b | 139 | 0x8B |  | The weather isn't right for that |
| DAMAGE_IMMUNE | 0x8c | 140 | 0x8C |  | You can't do that while you are immune |
| PREVENTED_BY_MECHANIC | 0x8d | 141 | 0x8D |  | Can't do that while %s |
| PLAY_TIME | 0x8e | 142 | 0x8E |  | Maximum play time exceeded |
| REPUTATION | 0x8f | 143 | 0x8F |  | Your reputation isn't high enough |
| MIN_SKILL | 0x90 | 144 | 0x90 |  | Your skill is not high enough.  Requires %s (%d). |
| UNKNOWN | 0x91 | 145 | 0x91 |  | Generic out of bounds response:  Unknown reason |
