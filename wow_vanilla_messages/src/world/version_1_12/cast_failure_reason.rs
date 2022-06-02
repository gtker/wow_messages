use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_cast_result.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_cast_result.wowm#L8):
/// ```text
/// enum CastFailureReason : u8 {
///     AFFECTING_COMBAT = 0x00;
///     ALREADY_AT_FULL_HEALTH = 0x01;
///     ALREADY_AT_FULL_POWER = 0x02;
///     ALREADY_BEING_TAMED = 0x03;
///     ALREADY_HAVE_CHARM = 0x04;
///     ALREADY_HAVE_SUMMON = 0x05;
///     ALREADY_OPEN = 0x06;
///     AURA_BOUNCED = 0x07;
///     AUTOTRACK_INTERRUPTED = 0x08;
///     BAD_IMPLICIT_TARGETS = 0x09;
///     BAD_TARGETS = 0x0a;
///     CANT_BE_CHARMED = 0x0b;
///     CANT_BE_DISENCHANTED = 0x0c;
///     CANT_BE_PROSPECTED = 0x0d;
///     CANT_CAST_ON_TAPPED = 0x0e;
///     CANT_DUEL_WHILE_INVISIBLE = 0x0f;
///     CANT_DUEL_WHILE_STEALTHED = 0x10;
///     CANT_STEALTH = 0x11;
///     CASTER_AURASTATE = 0x12;
///     CASTER_DEAD = 0x13;
///     CHARMED = 0x14;
///     CHEST_IN_USE = 0x15;
///     CONFUSED = 0x16;
///     DONT_REPORT = 0x17;
///     EQUIPPED_ITEM = 0x18;
///     EQUIPPED_ITEM_CLASS = 0x19;
///     EQUIPPED_ITEM_CLASS_MAINHAND = 0x1a;
///     EQUIPPED_ITEM_CLASS_OFFHAND = 0x1b;
///     ERROR = 0x1c;
///     FIZZLE = 0x1d;
///     FLEEING = 0x1e;
///     FOOD_LOWLEVEL = 0x1f;
///     HIGHLEVEL = 0x20;
///     HUNGER_SATIATED = 0x21;
///     IMMUNE = 0x22;
///     INTERRUPTED = 0x23;
///     INTERRUPTED_COMBAT = 0x24;
///     ITEM_ALREADY_ENCHANTED = 0x25;
///     ITEM_GONE = 0x26;
///     ITEM_NOT_FOUND = 0x27;
///     ITEM_NOT_READY = 0x28;
///     LEVEL_REQUIREMENT = 0x29;
///     LINE_OF_SIGHT = 0x2a;
///     LOWLEVEL = 0x2b;
///     LOW_CASTLEVEL = 0x2c;
///     MAINHAND_EMPTY = 0x2d;
///     MOVING = 0x2e;
///     NEED_AMMO = 0x2f;
///     NEED_AMMO_POUCH = 0x30;
///     NEED_EXOTIC_AMMO = 0x31;
///     NOPATH = 0x32;
///     NOT_BEHIND = 0x33;
///     NOT_FISHABLE = 0x34;
///     NOT_HERE = 0x35;
///     NOT_INFRONT = 0x36;
///     NOT_IN_CONTROL = 0x37;
///     NOT_KNOWN = 0x38;
///     NOT_MOUNTED = 0x39;
///     NOT_ON_TAXI = 0x3a;
///     NOT_ON_TRANSPORT = 0x3b;
///     NOT_READY = 0x3c;
///     NOT_SHAPESHIFT = 0x3d;
///     NOT_STANDING = 0x3e;
///     NOT_TRADEABLE = 0x3f;
///     NOT_TRADING = 0x40;
///     NOT_UNSHEATHED = 0x41;
///     NOT_WHILE_GHOST = 0x42;
///     NO_AMMO = 0x43;
///     NO_CHARGES_REMAIN = 0x44;
///     NO_CHAMPION = 0x45;
///     NO_COMBO_POINTS = 0x46;
///     NO_DUELING = 0x47;
///     NO_ENDURANCE = 0x48;
///     NO_FISH = 0x49;
///     NO_ITEMS_WHILE_SHAPESHIFTED = 0x4a;
///     NO_MOUNTS_ALLOWED = 0x4b;
///     NO_PET = 0x4c;
///     NO_POWER = 0x4d;
///     NOTHING_TO_DISPEL = 0x4e;
///     NOTHING_TO_STEAL = 0x4f;
///     ONLY_ABOVEWATER = 0x50;
///     ONLY_DAYTIME = 0x51;
///     ONLY_INDOORS = 0x52;
///     ONLY_MOUNTED = 0x53;
///     ONLY_NIGHTTIME = 0x54;
///     ONLY_OUTDOORS = 0x55;
///     ONLY_SHAPESHIFT = 0x56;
///     ONLY_STEALTHED = 0x57;
///     ONLY_UNDERWATER = 0x58;
///     OUT_OF_RANGE = 0x59;
///     PACIFIED = 0x5a;
///     POSSESSED = 0x5b;
///     REAGENTS = 0x5c;
///     REQUIRES_AREA = 0x5d;
///     REQUIRES_SPELL_FOCUS = 0x5e;
///     ROOTED = 0x5f;
///     SILENCED = 0x60;
///     SPELL_IN_PROGRESS = 0x61;
///     SPELL_LEARNED = 0x62;
///     SPELL_UNAVAILABLE = 0x63;
///     STUNNED = 0x64;
///     TARGETS_DEAD = 0x65;
///     TARGET_AFFECTING_COMBAT = 0x66;
///     TARGET_AURASTATE = 0x67;
///     TARGET_DUELING = 0x68;
///     TARGET_ENEMY = 0x69;
///     TARGET_ENRAGED = 0x6a;
///     TARGET_FRIENDLY = 0x6b;
///     TARGET_IN_COMBAT = 0x6c;
///     TARGET_IS_PLAYER = 0x6d;
///     TARGET_NOT_DEAD = 0x6e;
///     TARGET_NOT_IN_PARTY = 0x6f;
///     TARGET_NOT_LOOTED = 0x70;
///     TARGET_NOT_PLAYER = 0x71;
///     TARGET_NO_POCKETS = 0x72;
///     TARGET_NO_WEAPONS = 0x73;
///     TARGET_UNSKINNABLE = 0x74;
///     THIRST_SATIATED = 0x75;
///     TOO_CLOSE = 0x76;
///     TOO_MANY_OF_ITEM = 0x77;
///     TOTEMS = 0x78;
///     TRAINING_POINTS = 0x79;
///     TRY_AGAIN = 0x7a;
///     UNIT_NOT_BEHIND = 0x7b;
///     UNIT_NOT_INFRONT = 0x7c;
///     WRONG_PET_FOOD = 0x7d;
///     NOT_WHILE_FATIGUED = 0x7e;
///     TARGET_NOT_IN_INSTANCE = 0x7f;
///     NOT_WHILE_TRADING = 0x80;
///     TARGET_NOT_IN_RAID = 0x81;
///     DISENCHANT_WHILE_LOOTING = 0x82;
///     PROSPECT_WHILE_LOOTING = 0x83;
///     PROSPECT_NEED_MORE = 0x84;
///     TARGET_FREEFORALL = 0x85;
///     NO_EDIBLE_CORPSES = 0x86;
///     ONLY_BATTLEGROUNDS = 0x87;
///     TARGET_NOT_GHOST = 0x88;
///     TOO_MANY_SKILLS = 0x89;
///     TRANSFORM_UNUSABLE = 0x8a;
///     WRONG_WEATHER = 0x8b;
///     DAMAGE_IMMUNE = 0x8c;
///     PREVENTED_BY_MECHANIC = 0x8d;
///     PLAY_TIME = 0x8e;
///     REPUTATION = 0x8f;
///     MIN_SKILL = 0x90;
///     UNKNOWN = 0x91;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum CastFailureReason {
    /// # Comment
    ///
    /// You are in combat
    AFFECTING_COMBAT,
    /// # Comment
    ///
    /// You are already at full Health.
    ALREADY_AT_FULL_HEALTH,
    /// # Comment
    ///
    /// You are already at full %s.
    ALREADY_AT_FULL_POWER,
    /// # Comment
    ///
    /// That creature is already being tamed
    ALREADY_BEING_TAMED,
    /// # Comment
    ///
    /// You already control a charmed creature
    ALREADY_HAVE_CHARM,
    /// # Comment
    ///
    /// You already control a summoned creature
    ALREADY_HAVE_SUMMON,
    /// # Comment
    ///
    /// Already open
    ALREADY_OPEN,
    /// # Comment
    ///
    /// A more powerful spell is already active
    AURA_BOUNCED,
    /// # Comment
    ///
    /// Message is hidden/unused
    AUTOTRACK_INTERRUPTED,
    /// # Comment
    ///
    /// You have no target.
    BAD_IMPLICIT_TARGETS,
    /// # Comment
    ///
    /// Invalid target
    BAD_TARGETS,
    /// # Comment
    ///
    /// Target can't be charmed
    CANT_BE_CHARMED,
    /// # Comment
    ///
    /// Item cannot be disenchanted
    CANT_BE_DISENCHANTED,
    /// # Comment
    ///
    /// There are no gems in this
    CANT_BE_PROSPECTED,
    /// # Comment
    ///
    /// Target is tapped
    CANT_CAST_ON_TAPPED,
    /// # Comment
    ///
    /// You can't start a duel while invisible
    CANT_DUEL_WHILE_INVISIBLE,
    /// # Comment
    ///
    /// You can't start a duel while stealthed
    CANT_DUEL_WHILE_STEALTHED,
    /// # Comment
    ///
    /// You are too close to enemies
    CANT_STEALTH,
    /// # Comment
    ///
    /// You can't do that yet
    CASTER_AURASTATE,
    /// # Comment
    ///
    /// You are dead
    CASTER_DEAD,
    /// # Comment
    ///
    /// Can't do that while charmed
    CHARMED,
    /// # Comment
    ///
    /// That is already being used
    CHEST_IN_USE,
    /// # Comment
    ///
    /// Can't do that while confused
    CONFUSED,
    /// # Comment
    ///
    /// Message is hidden/unused
    DONT_REPORT,
    /// # Comment
    ///
    /// Must have the proper item equipped
    EQUIPPED_ITEM,
    /// # Comment
    ///
    /// Must have a %s equipped
    EQUIPPED_ITEM_CLASS,
    /// # Comment
    ///
    /// Must have a %s equipped in the main hand
    EQUIPPED_ITEM_CLASS_MAINHAND,
    /// # Comment
    ///
    /// Must have a %s equipped in the offhand
    EQUIPPED_ITEM_CLASS_OFFHAND,
    /// # Comment
    ///
    /// Internal error
    ERROR,
    /// # Comment
    ///
    /// Fizzled
    FIZZLE,
    /// # Comment
    ///
    /// Can't do that while fleeing
    FLEEING,
    /// # Comment
    ///
    /// That food's level is not high enough for your pet
    FOOD_LOWLEVEL,
    /// # Comment
    ///
    /// Target is too high level
    HIGHLEVEL,
    /// # Comment
    ///
    /// Message is hidden/unused
    HUNGER_SATIATED,
    /// # Comment
    ///
    /// Immune
    IMMUNE,
    /// # Comment
    ///
    /// Interrupted
    INTERRUPTED,
    /// # Comment
    ///
    /// Interrupted
    INTERRUPTED_COMBAT,
    /// # Comment
    ///
    /// Item is already enchanted
    ITEM_ALREADY_ENCHANTED,
    /// # Comment
    ///
    /// Item is gone
    ITEM_GONE,
    /// # Comment
    ///
    /// Tried to enchant an item that didn't exist
    ITEM_NOT_FOUND,
    /// # Comment
    ///
    /// Item is not ready yet.
    ITEM_NOT_READY,
    /// # Comment
    ///
    /// You are not high enough level
    LEVEL_REQUIREMENT,
    /// # Comment
    ///
    /// Target not in line of sight
    LINE_OF_SIGHT,
    /// # Comment
    ///
    /// Target is too low level
    LOWLEVEL,
    /// # Comment
    ///
    /// Skill not high enough
    LOW_CASTLEVEL,
    /// # Comment
    ///
    /// Your weapon hand is empty
    MAINHAND_EMPTY,
    /// # Comment
    ///
    /// Can't do that while moving
    MOVING,
    /// # Comment
    ///
    /// Ammo needs to be in the paper doll ammo slot before it can be fired
    NEED_AMMO,
    /// # Comment
    ///
    /// Requires: %s
    NEED_AMMO_POUCH,
    /// # Comment
    ///
    /// Requires exotic ammo: %s
    NEED_EXOTIC_AMMO,
    /// # Comment
    ///
    /// No path available
    NOPATH,
    /// # Comment
    ///
    /// You must be behind your target
    NOT_BEHIND,
    /// # Comment
    ///
    /// Your cast didn't land in fishable water
    NOT_FISHABLE,
    /// # Comment
    ///
    /// You can't use that here
    NOT_HERE,
    /// # Comment
    ///
    /// You must be in front of your target
    NOT_INFRONT,
    /// # Comment
    ///
    /// You are not in control of your actions
    NOT_IN_CONTROL,
    /// # Comment
    ///
    /// Spell not learned
    NOT_KNOWN,
    /// # Comment
    ///
    /// You are mounted
    NOT_MOUNTED,
    /// # Comment
    ///
    /// You are in flight
    NOT_ON_TAXI,
    /// # Comment
    ///
    /// You are on a transport
    NOT_ON_TRANSPORT,
    /// # Comment
    ///
    /// Spell is not ready yet.
    NOT_READY,
    /// # Comment
    ///
    /// You are in shapeshift form
    NOT_SHAPESHIFT,
    /// # Comment
    ///
    /// You must be standing to do that
    NOT_STANDING,
    /// # Comment
    ///
    /// You can only use this on an object you own
    NOT_TRADEABLE,
    /// # Comment
    ///
    /// Tried to enchant a trade item, but not trading
    NOT_TRADING,
    /// # Comment
    ///
    /// You have to be unsheathed to do that!
    NOT_UNSHEATHED,
    /// # Comment
    ///
    /// Can't cast as ghost
    NOT_WHILE_GHOST,
    /// # Comment
    ///
    /// Out of ammo
    NO_AMMO,
    /// # Comment
    ///
    /// No charges remain
    NO_CHARGES_REMAIN,
    /// # Comment
    ///
    /// You haven't selected a champion
    NO_CHAMPION,
    /// # Comment
    ///
    /// That ability requires combo points
    NO_COMBO_POINTS,
    /// # Comment
    ///
    /// Dueling isn't allowed here
    NO_DUELING,
    /// # Comment
    ///
    /// Not enough endurance
    NO_ENDURANCE,
    /// # Comment
    ///
    /// There aren't any fish here
    NO_FISH,
    /// # Comment
    ///
    /// Can't use items while shapeshifted
    NO_ITEMS_WHILE_SHAPESHIFTED,
    /// # Comment
    ///
    /// You can't mount here
    NO_MOUNTS_ALLOWED,
    /// # Comment
    ///
    /// You do not have a pet
    NO_PET,
    /// # Comment
    ///
    /// Dynamic pre-defined messages, no args: Not enough mana, Not enough rage, etc
    NO_POWER,
    /// # Comment
    ///
    /// Nothing to dispel
    NOTHING_TO_DISPEL,
    /// # Comment
    ///
    /// Nothing to steal
    NOTHING_TO_STEAL,
    /// # Comment
    ///
    /// Cannot use while swimming
    ONLY_ABOVEWATER,
    /// # Comment
    ///
    /// Can only use during the day
    ONLY_DAYTIME,
    /// # Comment
    ///
    /// Can only use indoors
    ONLY_INDOORS,
    /// # Comment
    ///
    /// Can only use while mounted
    ONLY_MOUNTED,
    /// # Comment
    ///
    /// Can only use during the night
    ONLY_NIGHTTIME,
    /// # Comment
    ///
    /// Can only use outside
    ONLY_OUTDOORS,
    /// # Comment
    ///
    /// Must be in %s
    ONLY_SHAPESHIFT,
    /// # Comment
    ///
    /// You must be in stealth mode
    ONLY_STEALTHED,
    /// # Comment
    ///
    /// Can only use while swimming
    ONLY_UNDERWATER,
    /// # Comment
    ///
    /// Out of range.
    OUT_OF_RANGE,
    /// # Comment
    ///
    /// Can't use that ability while pacified
    PACIFIED,
    /// # Comment
    ///
    /// You are possessed
    POSSESSED,
    /// # Comment
    ///
    /// Message is hidden/unused, supposedly implemented client-side only
    REAGENTS,
    /// # Comment
    ///
    /// You need to be in %s
    REQUIRES_AREA,
    /// # Comment
    ///
    /// Requires %s
    REQUIRES_SPELL_FOCUS,
    /// # Comment
    ///
    /// You are unable to move
    ROOTED,
    /// # Comment
    ///
    /// Can't do that while silenced
    SILENCED,
    /// # Comment
    ///
    /// Another action is in progress
    SPELL_IN_PROGRESS,
    /// # Comment
    ///
    /// You have already learned the spell
    SPELL_LEARNED,
    /// # Comment
    ///
    /// The spell is not available to you
    SPELL_UNAVAILABLE,
    /// # Comment
    ///
    /// Can't do that while stunned
    STUNNED,
    /// # Comment
    ///
    /// Your target is dead
    TARGETS_DEAD,
    /// # Comment
    ///
    /// Target is in combat
    TARGET_AFFECTING_COMBAT,
    /// # Comment
    ///
    /// You can't do that yet
    TARGET_AURASTATE,
    /// # Comment
    ///
    /// Target is currently dueling
    TARGET_DUELING,
    /// # Comment
    ///
    /// Target is hostile
    TARGET_ENEMY,
    /// # Comment
    ///
    /// Target is too enraged to be charmed
    TARGET_ENRAGED,
    /// # Comment
    ///
    /// Target is friendly
    TARGET_FRIENDLY,
    /// # Comment
    ///
    /// The target can't be in combat
    TARGET_IN_COMBAT,
    /// # Comment
    ///
    /// Can't target players
    TARGET_IS_PLAYER,
    /// # Comment
    ///
    /// Target is alive
    TARGET_NOT_DEAD,
    /// # Comment
    ///
    /// Target is not in your party
    TARGET_NOT_IN_PARTY,
    /// # Comment
    ///
    /// Creature must be looted first
    TARGET_NOT_LOOTED,
    /// # Comment
    ///
    /// Target is not a player
    TARGET_NOT_PLAYER,
    /// # Comment
    ///
    /// No pockets to pick
    TARGET_NO_POCKETS,
    /// # Comment
    ///
    /// Target has no weapons equipped
    TARGET_NO_WEAPONS,
    /// # Comment
    ///
    /// Creature is not skinnable
    TARGET_UNSKINNABLE,
    /// # Comment
    ///
    /// Message is hidden/unused
    THIRST_SATIATED,
    /// # Comment
    ///
    /// Target too close
    TOO_CLOSE,
    /// # Comment
    ///
    /// You have too many of that item already
    TOO_MANY_OF_ITEM,
    /// # Comment
    ///
    /// Message is hidden/unused, supposedly implemented client-side only
    TOTEMS,
    /// # Comment
    ///
    /// Not enough training points
    TRAINING_POINTS,
    /// # Comment
    ///
    /// Failed attempt
    TRY_AGAIN,
    /// # Comment
    ///
    /// Target needs to be behind you
    UNIT_NOT_BEHIND,
    /// # Comment
    ///
    /// Target needs to be in front of you
    UNIT_NOT_INFRONT,
    /// # Comment
    ///
    /// Your pet doesn't like that food
    WRONG_PET_FOOD,
    /// # Comment
    ///
    /// Can't cast while fatigued
    NOT_WHILE_FATIGUED,
    /// # Comment
    ///
    /// Target must be in this instance
    TARGET_NOT_IN_INSTANCE,
    /// # Comment
    ///
    /// Can't cast while trading
    NOT_WHILE_TRADING,
    /// # Comment
    ///
    /// Target is not in your party or raid group
    TARGET_NOT_IN_RAID,
    /// # Comment
    ///
    /// Cannot disenchant while looting
    DISENCHANT_WHILE_LOOTING,
    /// # Comment
    ///
    /// Cannot prospect while looting
    PROSPECT_WHILE_LOOTING,
    /// # Comment
    ///
    /// Message is hidden/unused, supposedly implemented client-side only
    PROSPECT_NEED_MORE,
    /// # Comment
    ///
    /// Target is currently in free-for-all PvP combat
    TARGET_FREEFORALL,
    /// # Comment
    ///
    /// There are no nearby corpses to eat
    NO_EDIBLE_CORPSES,
    /// # Comment
    ///
    /// Can only use in battlegrounds
    ONLY_BATTLEGROUNDS,
    /// # Comment
    ///
    /// Target is not a ghost
    TARGET_NOT_GHOST,
    /// # Comment
    ///
    /// Your pet can't learn any more skills
    TOO_MANY_SKILLS,
    /// # Comment
    ///
    /// You can't use the new item
    TRANSFORM_UNUSABLE,
    /// # Comment
    ///
    /// The weather isn't right for that
    WRONG_WEATHER,
    /// # Comment
    ///
    /// You can't do that while you are immune
    DAMAGE_IMMUNE,
    /// # Comment
    ///
    /// Can't do that while %s
    PREVENTED_BY_MECHANIC,
    /// # Comment
    ///
    /// Maximum play time exceeded
    PLAY_TIME,
    /// # Comment
    ///
    /// Your reputation isn't high enough
    REPUTATION,
    /// # Comment
    ///
    /// Your skill is not high enough.  Requires %s (%d).
    MIN_SKILL,
    /// # Comment
    ///
    /// Generic out of bounds response:  Unknown reason
    UNKNOWN,
}

impl CastFailureReason {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::AFFECTING_COMBAT => 0x0,
            Self::ALREADY_AT_FULL_HEALTH => 0x1,
            Self::ALREADY_AT_FULL_POWER => 0x2,
            Self::ALREADY_BEING_TAMED => 0x3,
            Self::ALREADY_HAVE_CHARM => 0x4,
            Self::ALREADY_HAVE_SUMMON => 0x5,
            Self::ALREADY_OPEN => 0x6,
            Self::AURA_BOUNCED => 0x7,
            Self::AUTOTRACK_INTERRUPTED => 0x8,
            Self::BAD_IMPLICIT_TARGETS => 0x9,
            Self::BAD_TARGETS => 0xa,
            Self::CANT_BE_CHARMED => 0xb,
            Self::CANT_BE_DISENCHANTED => 0xc,
            Self::CANT_BE_PROSPECTED => 0xd,
            Self::CANT_CAST_ON_TAPPED => 0xe,
            Self::CANT_DUEL_WHILE_INVISIBLE => 0xf,
            Self::CANT_DUEL_WHILE_STEALTHED => 0x10,
            Self::CANT_STEALTH => 0x11,
            Self::CASTER_AURASTATE => 0x12,
            Self::CASTER_DEAD => 0x13,
            Self::CHARMED => 0x14,
            Self::CHEST_IN_USE => 0x15,
            Self::CONFUSED => 0x16,
            Self::DONT_REPORT => 0x17,
            Self::EQUIPPED_ITEM => 0x18,
            Self::EQUIPPED_ITEM_CLASS => 0x19,
            Self::EQUIPPED_ITEM_CLASS_MAINHAND => 0x1a,
            Self::EQUIPPED_ITEM_CLASS_OFFHAND => 0x1b,
            Self::ERROR => 0x1c,
            Self::FIZZLE => 0x1d,
            Self::FLEEING => 0x1e,
            Self::FOOD_LOWLEVEL => 0x1f,
            Self::HIGHLEVEL => 0x20,
            Self::HUNGER_SATIATED => 0x21,
            Self::IMMUNE => 0x22,
            Self::INTERRUPTED => 0x23,
            Self::INTERRUPTED_COMBAT => 0x24,
            Self::ITEM_ALREADY_ENCHANTED => 0x25,
            Self::ITEM_GONE => 0x26,
            Self::ITEM_NOT_FOUND => 0x27,
            Self::ITEM_NOT_READY => 0x28,
            Self::LEVEL_REQUIREMENT => 0x29,
            Self::LINE_OF_SIGHT => 0x2a,
            Self::LOWLEVEL => 0x2b,
            Self::LOW_CASTLEVEL => 0x2c,
            Self::MAINHAND_EMPTY => 0x2d,
            Self::MOVING => 0x2e,
            Self::NEED_AMMO => 0x2f,
            Self::NEED_AMMO_POUCH => 0x30,
            Self::NEED_EXOTIC_AMMO => 0x31,
            Self::NOPATH => 0x32,
            Self::NOT_BEHIND => 0x33,
            Self::NOT_FISHABLE => 0x34,
            Self::NOT_HERE => 0x35,
            Self::NOT_INFRONT => 0x36,
            Self::NOT_IN_CONTROL => 0x37,
            Self::NOT_KNOWN => 0x38,
            Self::NOT_MOUNTED => 0x39,
            Self::NOT_ON_TAXI => 0x3a,
            Self::NOT_ON_TRANSPORT => 0x3b,
            Self::NOT_READY => 0x3c,
            Self::NOT_SHAPESHIFT => 0x3d,
            Self::NOT_STANDING => 0x3e,
            Self::NOT_TRADEABLE => 0x3f,
            Self::NOT_TRADING => 0x40,
            Self::NOT_UNSHEATHED => 0x41,
            Self::NOT_WHILE_GHOST => 0x42,
            Self::NO_AMMO => 0x43,
            Self::NO_CHARGES_REMAIN => 0x44,
            Self::NO_CHAMPION => 0x45,
            Self::NO_COMBO_POINTS => 0x46,
            Self::NO_DUELING => 0x47,
            Self::NO_ENDURANCE => 0x48,
            Self::NO_FISH => 0x49,
            Self::NO_ITEMS_WHILE_SHAPESHIFTED => 0x4a,
            Self::NO_MOUNTS_ALLOWED => 0x4b,
            Self::NO_PET => 0x4c,
            Self::NO_POWER => 0x4d,
            Self::NOTHING_TO_DISPEL => 0x4e,
            Self::NOTHING_TO_STEAL => 0x4f,
            Self::ONLY_ABOVEWATER => 0x50,
            Self::ONLY_DAYTIME => 0x51,
            Self::ONLY_INDOORS => 0x52,
            Self::ONLY_MOUNTED => 0x53,
            Self::ONLY_NIGHTTIME => 0x54,
            Self::ONLY_OUTDOORS => 0x55,
            Self::ONLY_SHAPESHIFT => 0x56,
            Self::ONLY_STEALTHED => 0x57,
            Self::ONLY_UNDERWATER => 0x58,
            Self::OUT_OF_RANGE => 0x59,
            Self::PACIFIED => 0x5a,
            Self::POSSESSED => 0x5b,
            Self::REAGENTS => 0x5c,
            Self::REQUIRES_AREA => 0x5d,
            Self::REQUIRES_SPELL_FOCUS => 0x5e,
            Self::ROOTED => 0x5f,
            Self::SILENCED => 0x60,
            Self::SPELL_IN_PROGRESS => 0x61,
            Self::SPELL_LEARNED => 0x62,
            Self::SPELL_UNAVAILABLE => 0x63,
            Self::STUNNED => 0x64,
            Self::TARGETS_DEAD => 0x65,
            Self::TARGET_AFFECTING_COMBAT => 0x66,
            Self::TARGET_AURASTATE => 0x67,
            Self::TARGET_DUELING => 0x68,
            Self::TARGET_ENEMY => 0x69,
            Self::TARGET_ENRAGED => 0x6a,
            Self::TARGET_FRIENDLY => 0x6b,
            Self::TARGET_IN_COMBAT => 0x6c,
            Self::TARGET_IS_PLAYER => 0x6d,
            Self::TARGET_NOT_DEAD => 0x6e,
            Self::TARGET_NOT_IN_PARTY => 0x6f,
            Self::TARGET_NOT_LOOTED => 0x70,
            Self::TARGET_NOT_PLAYER => 0x71,
            Self::TARGET_NO_POCKETS => 0x72,
            Self::TARGET_NO_WEAPONS => 0x73,
            Self::TARGET_UNSKINNABLE => 0x74,
            Self::THIRST_SATIATED => 0x75,
            Self::TOO_CLOSE => 0x76,
            Self::TOO_MANY_OF_ITEM => 0x77,
            Self::TOTEMS => 0x78,
            Self::TRAINING_POINTS => 0x79,
            Self::TRY_AGAIN => 0x7a,
            Self::UNIT_NOT_BEHIND => 0x7b,
            Self::UNIT_NOT_INFRONT => 0x7c,
            Self::WRONG_PET_FOOD => 0x7d,
            Self::NOT_WHILE_FATIGUED => 0x7e,
            Self::TARGET_NOT_IN_INSTANCE => 0x7f,
            Self::NOT_WHILE_TRADING => 0x80,
            Self::TARGET_NOT_IN_RAID => 0x81,
            Self::DISENCHANT_WHILE_LOOTING => 0x82,
            Self::PROSPECT_WHILE_LOOTING => 0x83,
            Self::PROSPECT_NEED_MORE => 0x84,
            Self::TARGET_FREEFORALL => 0x85,
            Self::NO_EDIBLE_CORPSES => 0x86,
            Self::ONLY_BATTLEGROUNDS => 0x87,
            Self::TARGET_NOT_GHOST => 0x88,
            Self::TOO_MANY_SKILLS => 0x89,
            Self::TRANSFORM_UNUSABLE => 0x8a,
            Self::WRONG_WEATHER => 0x8b,
            Self::DAMAGE_IMMUNE => 0x8c,
            Self::PREVENTED_BY_MECHANIC => 0x8d,
            Self::PLAY_TIME => 0x8e,
            Self::REPUTATION => 0x8f,
            Self::MIN_SKILL => 0x90,
            Self::UNKNOWN => 0x91,
        }
    }

}

impl Default for CastFailureReason {
    fn default() -> Self {
        Self::AFFECTING_COMBAT
    }
}

impl std::fmt::Display for CastFailureReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AFFECTING_COMBAT => f.write_str("AFFECTING_COMBAT"),
            Self::ALREADY_AT_FULL_HEALTH => f.write_str("ALREADY_AT_FULL_HEALTH"),
            Self::ALREADY_AT_FULL_POWER => f.write_str("ALREADY_AT_FULL_POWER"),
            Self::ALREADY_BEING_TAMED => f.write_str("ALREADY_BEING_TAMED"),
            Self::ALREADY_HAVE_CHARM => f.write_str("ALREADY_HAVE_CHARM"),
            Self::ALREADY_HAVE_SUMMON => f.write_str("ALREADY_HAVE_SUMMON"),
            Self::ALREADY_OPEN => f.write_str("ALREADY_OPEN"),
            Self::AURA_BOUNCED => f.write_str("AURA_BOUNCED"),
            Self::AUTOTRACK_INTERRUPTED => f.write_str("AUTOTRACK_INTERRUPTED"),
            Self::BAD_IMPLICIT_TARGETS => f.write_str("BAD_IMPLICIT_TARGETS"),
            Self::BAD_TARGETS => f.write_str("BAD_TARGETS"),
            Self::CANT_BE_CHARMED => f.write_str("CANT_BE_CHARMED"),
            Self::CANT_BE_DISENCHANTED => f.write_str("CANT_BE_DISENCHANTED"),
            Self::CANT_BE_PROSPECTED => f.write_str("CANT_BE_PROSPECTED"),
            Self::CANT_CAST_ON_TAPPED => f.write_str("CANT_CAST_ON_TAPPED"),
            Self::CANT_DUEL_WHILE_INVISIBLE => f.write_str("CANT_DUEL_WHILE_INVISIBLE"),
            Self::CANT_DUEL_WHILE_STEALTHED => f.write_str("CANT_DUEL_WHILE_STEALTHED"),
            Self::CANT_STEALTH => f.write_str("CANT_STEALTH"),
            Self::CASTER_AURASTATE => f.write_str("CASTER_AURASTATE"),
            Self::CASTER_DEAD => f.write_str("CASTER_DEAD"),
            Self::CHARMED => f.write_str("CHARMED"),
            Self::CHEST_IN_USE => f.write_str("CHEST_IN_USE"),
            Self::CONFUSED => f.write_str("CONFUSED"),
            Self::DONT_REPORT => f.write_str("DONT_REPORT"),
            Self::EQUIPPED_ITEM => f.write_str("EQUIPPED_ITEM"),
            Self::EQUIPPED_ITEM_CLASS => f.write_str("EQUIPPED_ITEM_CLASS"),
            Self::EQUIPPED_ITEM_CLASS_MAINHAND => f.write_str("EQUIPPED_ITEM_CLASS_MAINHAND"),
            Self::EQUIPPED_ITEM_CLASS_OFFHAND => f.write_str("EQUIPPED_ITEM_CLASS_OFFHAND"),
            Self::ERROR => f.write_str("ERROR"),
            Self::FIZZLE => f.write_str("FIZZLE"),
            Self::FLEEING => f.write_str("FLEEING"),
            Self::FOOD_LOWLEVEL => f.write_str("FOOD_LOWLEVEL"),
            Self::HIGHLEVEL => f.write_str("HIGHLEVEL"),
            Self::HUNGER_SATIATED => f.write_str("HUNGER_SATIATED"),
            Self::IMMUNE => f.write_str("IMMUNE"),
            Self::INTERRUPTED => f.write_str("INTERRUPTED"),
            Self::INTERRUPTED_COMBAT => f.write_str("INTERRUPTED_COMBAT"),
            Self::ITEM_ALREADY_ENCHANTED => f.write_str("ITEM_ALREADY_ENCHANTED"),
            Self::ITEM_GONE => f.write_str("ITEM_GONE"),
            Self::ITEM_NOT_FOUND => f.write_str("ITEM_NOT_FOUND"),
            Self::ITEM_NOT_READY => f.write_str("ITEM_NOT_READY"),
            Self::LEVEL_REQUIREMENT => f.write_str("LEVEL_REQUIREMENT"),
            Self::LINE_OF_SIGHT => f.write_str("LINE_OF_SIGHT"),
            Self::LOWLEVEL => f.write_str("LOWLEVEL"),
            Self::LOW_CASTLEVEL => f.write_str("LOW_CASTLEVEL"),
            Self::MAINHAND_EMPTY => f.write_str("MAINHAND_EMPTY"),
            Self::MOVING => f.write_str("MOVING"),
            Self::NEED_AMMO => f.write_str("NEED_AMMO"),
            Self::NEED_AMMO_POUCH => f.write_str("NEED_AMMO_POUCH"),
            Self::NEED_EXOTIC_AMMO => f.write_str("NEED_EXOTIC_AMMO"),
            Self::NOPATH => f.write_str("NOPATH"),
            Self::NOT_BEHIND => f.write_str("NOT_BEHIND"),
            Self::NOT_FISHABLE => f.write_str("NOT_FISHABLE"),
            Self::NOT_HERE => f.write_str("NOT_HERE"),
            Self::NOT_INFRONT => f.write_str("NOT_INFRONT"),
            Self::NOT_IN_CONTROL => f.write_str("NOT_IN_CONTROL"),
            Self::NOT_KNOWN => f.write_str("NOT_KNOWN"),
            Self::NOT_MOUNTED => f.write_str("NOT_MOUNTED"),
            Self::NOT_ON_TAXI => f.write_str("NOT_ON_TAXI"),
            Self::NOT_ON_TRANSPORT => f.write_str("NOT_ON_TRANSPORT"),
            Self::NOT_READY => f.write_str("NOT_READY"),
            Self::NOT_SHAPESHIFT => f.write_str("NOT_SHAPESHIFT"),
            Self::NOT_STANDING => f.write_str("NOT_STANDING"),
            Self::NOT_TRADEABLE => f.write_str("NOT_TRADEABLE"),
            Self::NOT_TRADING => f.write_str("NOT_TRADING"),
            Self::NOT_UNSHEATHED => f.write_str("NOT_UNSHEATHED"),
            Self::NOT_WHILE_GHOST => f.write_str("NOT_WHILE_GHOST"),
            Self::NO_AMMO => f.write_str("NO_AMMO"),
            Self::NO_CHARGES_REMAIN => f.write_str("NO_CHARGES_REMAIN"),
            Self::NO_CHAMPION => f.write_str("NO_CHAMPION"),
            Self::NO_COMBO_POINTS => f.write_str("NO_COMBO_POINTS"),
            Self::NO_DUELING => f.write_str("NO_DUELING"),
            Self::NO_ENDURANCE => f.write_str("NO_ENDURANCE"),
            Self::NO_FISH => f.write_str("NO_FISH"),
            Self::NO_ITEMS_WHILE_SHAPESHIFTED => f.write_str("NO_ITEMS_WHILE_SHAPESHIFTED"),
            Self::NO_MOUNTS_ALLOWED => f.write_str("NO_MOUNTS_ALLOWED"),
            Self::NO_PET => f.write_str("NO_PET"),
            Self::NO_POWER => f.write_str("NO_POWER"),
            Self::NOTHING_TO_DISPEL => f.write_str("NOTHING_TO_DISPEL"),
            Self::NOTHING_TO_STEAL => f.write_str("NOTHING_TO_STEAL"),
            Self::ONLY_ABOVEWATER => f.write_str("ONLY_ABOVEWATER"),
            Self::ONLY_DAYTIME => f.write_str("ONLY_DAYTIME"),
            Self::ONLY_INDOORS => f.write_str("ONLY_INDOORS"),
            Self::ONLY_MOUNTED => f.write_str("ONLY_MOUNTED"),
            Self::ONLY_NIGHTTIME => f.write_str("ONLY_NIGHTTIME"),
            Self::ONLY_OUTDOORS => f.write_str("ONLY_OUTDOORS"),
            Self::ONLY_SHAPESHIFT => f.write_str("ONLY_SHAPESHIFT"),
            Self::ONLY_STEALTHED => f.write_str("ONLY_STEALTHED"),
            Self::ONLY_UNDERWATER => f.write_str("ONLY_UNDERWATER"),
            Self::OUT_OF_RANGE => f.write_str("OUT_OF_RANGE"),
            Self::PACIFIED => f.write_str("PACIFIED"),
            Self::POSSESSED => f.write_str("POSSESSED"),
            Self::REAGENTS => f.write_str("REAGENTS"),
            Self::REQUIRES_AREA => f.write_str("REQUIRES_AREA"),
            Self::REQUIRES_SPELL_FOCUS => f.write_str("REQUIRES_SPELL_FOCUS"),
            Self::ROOTED => f.write_str("ROOTED"),
            Self::SILENCED => f.write_str("SILENCED"),
            Self::SPELL_IN_PROGRESS => f.write_str("SPELL_IN_PROGRESS"),
            Self::SPELL_LEARNED => f.write_str("SPELL_LEARNED"),
            Self::SPELL_UNAVAILABLE => f.write_str("SPELL_UNAVAILABLE"),
            Self::STUNNED => f.write_str("STUNNED"),
            Self::TARGETS_DEAD => f.write_str("TARGETS_DEAD"),
            Self::TARGET_AFFECTING_COMBAT => f.write_str("TARGET_AFFECTING_COMBAT"),
            Self::TARGET_AURASTATE => f.write_str("TARGET_AURASTATE"),
            Self::TARGET_DUELING => f.write_str("TARGET_DUELING"),
            Self::TARGET_ENEMY => f.write_str("TARGET_ENEMY"),
            Self::TARGET_ENRAGED => f.write_str("TARGET_ENRAGED"),
            Self::TARGET_FRIENDLY => f.write_str("TARGET_FRIENDLY"),
            Self::TARGET_IN_COMBAT => f.write_str("TARGET_IN_COMBAT"),
            Self::TARGET_IS_PLAYER => f.write_str("TARGET_IS_PLAYER"),
            Self::TARGET_NOT_DEAD => f.write_str("TARGET_NOT_DEAD"),
            Self::TARGET_NOT_IN_PARTY => f.write_str("TARGET_NOT_IN_PARTY"),
            Self::TARGET_NOT_LOOTED => f.write_str("TARGET_NOT_LOOTED"),
            Self::TARGET_NOT_PLAYER => f.write_str("TARGET_NOT_PLAYER"),
            Self::TARGET_NO_POCKETS => f.write_str("TARGET_NO_POCKETS"),
            Self::TARGET_NO_WEAPONS => f.write_str("TARGET_NO_WEAPONS"),
            Self::TARGET_UNSKINNABLE => f.write_str("TARGET_UNSKINNABLE"),
            Self::THIRST_SATIATED => f.write_str("THIRST_SATIATED"),
            Self::TOO_CLOSE => f.write_str("TOO_CLOSE"),
            Self::TOO_MANY_OF_ITEM => f.write_str("TOO_MANY_OF_ITEM"),
            Self::TOTEMS => f.write_str("TOTEMS"),
            Self::TRAINING_POINTS => f.write_str("TRAINING_POINTS"),
            Self::TRY_AGAIN => f.write_str("TRY_AGAIN"),
            Self::UNIT_NOT_BEHIND => f.write_str("UNIT_NOT_BEHIND"),
            Self::UNIT_NOT_INFRONT => f.write_str("UNIT_NOT_INFRONT"),
            Self::WRONG_PET_FOOD => f.write_str("WRONG_PET_FOOD"),
            Self::NOT_WHILE_FATIGUED => f.write_str("NOT_WHILE_FATIGUED"),
            Self::TARGET_NOT_IN_INSTANCE => f.write_str("TARGET_NOT_IN_INSTANCE"),
            Self::NOT_WHILE_TRADING => f.write_str("NOT_WHILE_TRADING"),
            Self::TARGET_NOT_IN_RAID => f.write_str("TARGET_NOT_IN_RAID"),
            Self::DISENCHANT_WHILE_LOOTING => f.write_str("DISENCHANT_WHILE_LOOTING"),
            Self::PROSPECT_WHILE_LOOTING => f.write_str("PROSPECT_WHILE_LOOTING"),
            Self::PROSPECT_NEED_MORE => f.write_str("PROSPECT_NEED_MORE"),
            Self::TARGET_FREEFORALL => f.write_str("TARGET_FREEFORALL"),
            Self::NO_EDIBLE_CORPSES => f.write_str("NO_EDIBLE_CORPSES"),
            Self::ONLY_BATTLEGROUNDS => f.write_str("ONLY_BATTLEGROUNDS"),
            Self::TARGET_NOT_GHOST => f.write_str("TARGET_NOT_GHOST"),
            Self::TOO_MANY_SKILLS => f.write_str("TOO_MANY_SKILLS"),
            Self::TRANSFORM_UNUSABLE => f.write_str("TRANSFORM_UNUSABLE"),
            Self::WRONG_WEATHER => f.write_str("WRONG_WEATHER"),
            Self::DAMAGE_IMMUNE => f.write_str("DAMAGE_IMMUNE"),
            Self::PREVENTED_BY_MECHANIC => f.write_str("PREVENTED_BY_MECHANIC"),
            Self::PLAY_TIME => f.write_str("PLAY_TIME"),
            Self::REPUTATION => f.write_str("REPUTATION"),
            Self::MIN_SKILL => f.write_str("MIN_SKILL"),
            Self::UNKNOWN => f.write_str("UNKNOWN"),
        }
    }
}

impl TryFrom<u8> for CastFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AFFECTING_COMBAT),
            1 => Ok(Self::ALREADY_AT_FULL_HEALTH),
            2 => Ok(Self::ALREADY_AT_FULL_POWER),
            3 => Ok(Self::ALREADY_BEING_TAMED),
            4 => Ok(Self::ALREADY_HAVE_CHARM),
            5 => Ok(Self::ALREADY_HAVE_SUMMON),
            6 => Ok(Self::ALREADY_OPEN),
            7 => Ok(Self::AURA_BOUNCED),
            8 => Ok(Self::AUTOTRACK_INTERRUPTED),
            9 => Ok(Self::BAD_IMPLICIT_TARGETS),
            10 => Ok(Self::BAD_TARGETS),
            11 => Ok(Self::CANT_BE_CHARMED),
            12 => Ok(Self::CANT_BE_DISENCHANTED),
            13 => Ok(Self::CANT_BE_PROSPECTED),
            14 => Ok(Self::CANT_CAST_ON_TAPPED),
            15 => Ok(Self::CANT_DUEL_WHILE_INVISIBLE),
            16 => Ok(Self::CANT_DUEL_WHILE_STEALTHED),
            17 => Ok(Self::CANT_STEALTH),
            18 => Ok(Self::CASTER_AURASTATE),
            19 => Ok(Self::CASTER_DEAD),
            20 => Ok(Self::CHARMED),
            21 => Ok(Self::CHEST_IN_USE),
            22 => Ok(Self::CONFUSED),
            23 => Ok(Self::DONT_REPORT),
            24 => Ok(Self::EQUIPPED_ITEM),
            25 => Ok(Self::EQUIPPED_ITEM_CLASS),
            26 => Ok(Self::EQUIPPED_ITEM_CLASS_MAINHAND),
            27 => Ok(Self::EQUIPPED_ITEM_CLASS_OFFHAND),
            28 => Ok(Self::ERROR),
            29 => Ok(Self::FIZZLE),
            30 => Ok(Self::FLEEING),
            31 => Ok(Self::FOOD_LOWLEVEL),
            32 => Ok(Self::HIGHLEVEL),
            33 => Ok(Self::HUNGER_SATIATED),
            34 => Ok(Self::IMMUNE),
            35 => Ok(Self::INTERRUPTED),
            36 => Ok(Self::INTERRUPTED_COMBAT),
            37 => Ok(Self::ITEM_ALREADY_ENCHANTED),
            38 => Ok(Self::ITEM_GONE),
            39 => Ok(Self::ITEM_NOT_FOUND),
            40 => Ok(Self::ITEM_NOT_READY),
            41 => Ok(Self::LEVEL_REQUIREMENT),
            42 => Ok(Self::LINE_OF_SIGHT),
            43 => Ok(Self::LOWLEVEL),
            44 => Ok(Self::LOW_CASTLEVEL),
            45 => Ok(Self::MAINHAND_EMPTY),
            46 => Ok(Self::MOVING),
            47 => Ok(Self::NEED_AMMO),
            48 => Ok(Self::NEED_AMMO_POUCH),
            49 => Ok(Self::NEED_EXOTIC_AMMO),
            50 => Ok(Self::NOPATH),
            51 => Ok(Self::NOT_BEHIND),
            52 => Ok(Self::NOT_FISHABLE),
            53 => Ok(Self::NOT_HERE),
            54 => Ok(Self::NOT_INFRONT),
            55 => Ok(Self::NOT_IN_CONTROL),
            56 => Ok(Self::NOT_KNOWN),
            57 => Ok(Self::NOT_MOUNTED),
            58 => Ok(Self::NOT_ON_TAXI),
            59 => Ok(Self::NOT_ON_TRANSPORT),
            60 => Ok(Self::NOT_READY),
            61 => Ok(Self::NOT_SHAPESHIFT),
            62 => Ok(Self::NOT_STANDING),
            63 => Ok(Self::NOT_TRADEABLE),
            64 => Ok(Self::NOT_TRADING),
            65 => Ok(Self::NOT_UNSHEATHED),
            66 => Ok(Self::NOT_WHILE_GHOST),
            67 => Ok(Self::NO_AMMO),
            68 => Ok(Self::NO_CHARGES_REMAIN),
            69 => Ok(Self::NO_CHAMPION),
            70 => Ok(Self::NO_COMBO_POINTS),
            71 => Ok(Self::NO_DUELING),
            72 => Ok(Self::NO_ENDURANCE),
            73 => Ok(Self::NO_FISH),
            74 => Ok(Self::NO_ITEMS_WHILE_SHAPESHIFTED),
            75 => Ok(Self::NO_MOUNTS_ALLOWED),
            76 => Ok(Self::NO_PET),
            77 => Ok(Self::NO_POWER),
            78 => Ok(Self::NOTHING_TO_DISPEL),
            79 => Ok(Self::NOTHING_TO_STEAL),
            80 => Ok(Self::ONLY_ABOVEWATER),
            81 => Ok(Self::ONLY_DAYTIME),
            82 => Ok(Self::ONLY_INDOORS),
            83 => Ok(Self::ONLY_MOUNTED),
            84 => Ok(Self::ONLY_NIGHTTIME),
            85 => Ok(Self::ONLY_OUTDOORS),
            86 => Ok(Self::ONLY_SHAPESHIFT),
            87 => Ok(Self::ONLY_STEALTHED),
            88 => Ok(Self::ONLY_UNDERWATER),
            89 => Ok(Self::OUT_OF_RANGE),
            90 => Ok(Self::PACIFIED),
            91 => Ok(Self::POSSESSED),
            92 => Ok(Self::REAGENTS),
            93 => Ok(Self::REQUIRES_AREA),
            94 => Ok(Self::REQUIRES_SPELL_FOCUS),
            95 => Ok(Self::ROOTED),
            96 => Ok(Self::SILENCED),
            97 => Ok(Self::SPELL_IN_PROGRESS),
            98 => Ok(Self::SPELL_LEARNED),
            99 => Ok(Self::SPELL_UNAVAILABLE),
            100 => Ok(Self::STUNNED),
            101 => Ok(Self::TARGETS_DEAD),
            102 => Ok(Self::TARGET_AFFECTING_COMBAT),
            103 => Ok(Self::TARGET_AURASTATE),
            104 => Ok(Self::TARGET_DUELING),
            105 => Ok(Self::TARGET_ENEMY),
            106 => Ok(Self::TARGET_ENRAGED),
            107 => Ok(Self::TARGET_FRIENDLY),
            108 => Ok(Self::TARGET_IN_COMBAT),
            109 => Ok(Self::TARGET_IS_PLAYER),
            110 => Ok(Self::TARGET_NOT_DEAD),
            111 => Ok(Self::TARGET_NOT_IN_PARTY),
            112 => Ok(Self::TARGET_NOT_LOOTED),
            113 => Ok(Self::TARGET_NOT_PLAYER),
            114 => Ok(Self::TARGET_NO_POCKETS),
            115 => Ok(Self::TARGET_NO_WEAPONS),
            116 => Ok(Self::TARGET_UNSKINNABLE),
            117 => Ok(Self::THIRST_SATIATED),
            118 => Ok(Self::TOO_CLOSE),
            119 => Ok(Self::TOO_MANY_OF_ITEM),
            120 => Ok(Self::TOTEMS),
            121 => Ok(Self::TRAINING_POINTS),
            122 => Ok(Self::TRY_AGAIN),
            123 => Ok(Self::UNIT_NOT_BEHIND),
            124 => Ok(Self::UNIT_NOT_INFRONT),
            125 => Ok(Self::WRONG_PET_FOOD),
            126 => Ok(Self::NOT_WHILE_FATIGUED),
            127 => Ok(Self::TARGET_NOT_IN_INSTANCE),
            128 => Ok(Self::NOT_WHILE_TRADING),
            129 => Ok(Self::TARGET_NOT_IN_RAID),
            130 => Ok(Self::DISENCHANT_WHILE_LOOTING),
            131 => Ok(Self::PROSPECT_WHILE_LOOTING),
            132 => Ok(Self::PROSPECT_NEED_MORE),
            133 => Ok(Self::TARGET_FREEFORALL),
            134 => Ok(Self::NO_EDIBLE_CORPSES),
            135 => Ok(Self::ONLY_BATTLEGROUNDS),
            136 => Ok(Self::TARGET_NOT_GHOST),
            137 => Ok(Self::TOO_MANY_SKILLS),
            138 => Ok(Self::TRANSFORM_UNUSABLE),
            139 => Ok(Self::WRONG_WEATHER),
            140 => Ok(Self::DAMAGE_IMMUNE),
            141 => Ok(Self::PREVENTED_BY_MECHANIC),
            142 => Ok(Self::PLAY_TIME),
            143 => Ok(Self::REPUTATION),
            144 => Ok(Self::MIN_SKILL),
            145 => Ok(Self::UNKNOWN),
            v => Err(crate::errors::EnumError::new("CastFailureReason", v as u32),)
        }
    }
}

