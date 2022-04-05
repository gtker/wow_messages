use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new_all.wowm:1753`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new_all.wowm#L1753):
/// ```text
/// enum SpellCastResult : u8 {
///     AFFECTING_COMBAT = 0x00;
///     ALREADY_AT_FULL_HEALTH = 0x01;
///     ALREADY_AT_FULL_MANA = 0x02;
///     ALREADY_BEING_TAMED = 0x03;
///     ALREADY_HAVE_CHARM = 0x04;
///     ALREADY_HAVE_SUMMON = 0x05;
///     ALREADY_OPEN = 0x06;
///     MORE_POWERFUL_SPELL_ACTIVE = 0x07;
///     BAD_IMPLICIT_TARGETS = 0x09;
///     BAD_TARGETS = 0x0A;
///     CANT_BE_CHARMED = 0x0B;
///     CANT_BE_DISENCHANTED = 0x0C;
///     CANT_BE_PROSPECTED = 0x0D;
///     CANT_CAST_ON_TAPPED = 0x0E;
///     CANT_DUEL_WHILE_INVISIBLE = 0x0F;
///     CANT_DUEL_WHILE_STEALTHED = 0x10;
///     CANT_TOO_CLOSE_TO_ENEMY = 0x11;
///     CANT_DO_THAT_YET = 0x12;
///     CASTER_DEAD = 0x13;
///     CHARMED = 0x14;
///     CHEST_IN_USE = 0x15;
///     CONFUSED = 0x16;
///     DONT_REPORT = 0x17;
///     EQUIPPED_ITEM = 0x18;
///     EQUIPPED_ITEM_CLASS = 0x19;
///     EQUIPPED_ITEM_CLASS_MAINHAND = 0x1A;
///     EQUIPPED_ITEM_CLASS_OFFHAND = 0x1B;
///     ERROR = 0x1C;
///     FIZZLE = 0x1D;
///     FLEEING = 0x1E;
///     FOOD_LOWLEVEL = 0x1F;
///     HIGHLEVEL = 0x20;
///     IMMUNE = 0x22;
///     INTERRUPTED = 0x23;
///     INTERRUPTED_COMBAT = 0x24;
///     ITEM_ALREADY_ENCHANTED = 0x25;
///     ITEM_GONE = 0x26;
///     ENCHANT_NOT_EXISTING_ITEM = 0x27;
///     ITEM_NOT_READY = 0x28;
///     LEVEL_REQUIREMENT = 0x29;
///     LINE_OF_SIGHT = 0x2A;
///     LOWLEVEL = 0x2B;
///     SKILL_NOT_HIGH_ENOUGH = 0x2C;
///     MAINHAND_EMPTY = 0x2D;
///     MOVING = 0x2E;
///     NEED_AMMO = 0x2F;
///     NEED_REQUIRES_SOMETHING = 0x30;
///     NEED_EXOTIC_AMMO = 0x31;
///     NOPATH = 0x32;
///     NOT_BEHIND = 0x33;
///     NOT_FISHABLE = 0x34;
///     NOT_HERE = 0x35;
///     NOT_INFRONT = 0x36;
///     NOT_IN_CONTROL = 0x37;
///     NOT_KNOWN = 0x38;
///     NOT_MOUNTED = 0x39;
///     NOT_ON_TAXI = 0x3A;
///     NOT_ON_TRANSPORT = 0x3B;
///     NOT_READY = 0x3C;
///     NOT_SHAPESHIFT = 0x3D;
///     NOT_STANDING = 0x3E;
///     NOT_TRADEABLE = 0x3F;
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
///     NO_ITEMS_WHILE_SHAPESHIFTED = 0x4A;
///     NO_MOUNTS_ALLOWED = 0x4B;
///     NO_PET = 0x4C;
///     NO_POWER = 0x4D;
///     NOTHING_TO_DISPEL = 0x4E;
///     NOTHING_TO_STEAL = 0x4F;
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
///     PACIFIED = 0x5A;
///     POSSESSED = 0x5B;
///     REQUIRES_AREA = 0x5D;
///     REQUIRES_SPELL_FOCUS = 0x5E;
///     ROOTED = 0x5F;
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
///     TARGET_ENRAGED = 0x6A;
///     TARGET_FRIENDLY = 0x6B;
///     TARGET_IN_COMBAT = 0x6C;
///     TARGET_IS_PLAYER = 0x6D;
///     TARGET_NOT_DEAD = 0x6E;
///     TARGET_NOT_IN_PARTY = 0x6F;
///     TARGET_NOT_LOOTED = 0x70;
///     TARGET_NOT_PLAYER = 0x71;
///     TARGET_NO_POCKETS = 0x72;
///     TARGET_NO_WEAPONS = 0x73;
///     TARGET_UNSKINNABLE = 0x74;
///     THIRST_SATIATED = 0x75;
///     TOO_CLOSE = 0x76;
///     TOO_MANY_OF_ITEM = 0x77;
///     TRAINING_POINTS = 0x79;
///     TRY_AGAIN = 0x7A;
///     UNIT_NOT_BEHIND = 0x7B;
///     UNIT_NOT_INFRONT = 0x7C;
///     WRONG_PET_FOOD = 0x7D;
///     NOT_WHILE_FATIGUED = 0x7E;
///     TARGET_NOT_IN_INSTANCE = 0x7F;
///     NOT_WHILE_TRADING = 0x80;
///     TARGET_NOT_IN_RAID = 0x81;
///     DISENCHANT_WHILE_LOOTING = 0x82;
///     PROSPECT_WHILE_LOOTING = 0x83;
///     TARGET_FREEFORALL = 0x85;
///     NO_EDIBLE_CORPSES = 0x86;
///     ONLY_BATTLEGROUNDS = 0x87;
///     TARGET_NOT_GHOST = 0x88;
///     TOO_MANY_SKILLS = 0x89;
///     CANT_USE_NEW_ITEM = 0x8A;
///     WRONG_WEATHER = 0x8B;
///     DAMAGE_IMMUNE = 0x8C;
///     PREVENTED_BY_MECHANIC = 0x8D;
///     PLAY_TIME = 0x8E;
///     REPUTATION = 0x8F;
///     MIN_SKILL = 0x90;
///     UNKNOWN = 0x91;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum SpellCastResult {
    AFFECTING_COMBAT,
    ALREADY_AT_FULL_HEALTH,
    ALREADY_AT_FULL_MANA,
    ALREADY_BEING_TAMED,
    ALREADY_HAVE_CHARM,
    ALREADY_HAVE_SUMMON,
    ALREADY_OPEN,
    MORE_POWERFUL_SPELL_ACTIVE,
    BAD_IMPLICIT_TARGETS,
    BAD_TARGETS,
    CANT_BE_CHARMED,
    CANT_BE_DISENCHANTED,
    CANT_BE_PROSPECTED,
    CANT_CAST_ON_TAPPED,
    CANT_DUEL_WHILE_INVISIBLE,
    CANT_DUEL_WHILE_STEALTHED,
    CANT_TOO_CLOSE_TO_ENEMY,
    CANT_DO_THAT_YET,
    CASTER_DEAD,
    CHARMED,
    CHEST_IN_USE,
    CONFUSED,
    DONT_REPORT,
    EQUIPPED_ITEM,
    EQUIPPED_ITEM_CLASS,
    EQUIPPED_ITEM_CLASS_MAINHAND,
    EQUIPPED_ITEM_CLASS_OFFHAND,
    ERROR,
    FIZZLE,
    FLEEING,
    FOOD_LOWLEVEL,
    HIGHLEVEL,
    IMMUNE,
    INTERRUPTED,
    INTERRUPTED_COMBAT,
    ITEM_ALREADY_ENCHANTED,
    ITEM_GONE,
    ENCHANT_NOT_EXISTING_ITEM,
    ITEM_NOT_READY,
    LEVEL_REQUIREMENT,
    LINE_OF_SIGHT,
    LOWLEVEL,
    SKILL_NOT_HIGH_ENOUGH,
    MAINHAND_EMPTY,
    MOVING,
    NEED_AMMO,
    NEED_REQUIRES_SOMETHING,
    NEED_EXOTIC_AMMO,
    NOPATH,
    NOT_BEHIND,
    NOT_FISHABLE,
    NOT_HERE,
    NOT_INFRONT,
    NOT_IN_CONTROL,
    NOT_KNOWN,
    NOT_MOUNTED,
    NOT_ON_TAXI,
    NOT_ON_TRANSPORT,
    NOT_READY,
    NOT_SHAPESHIFT,
    NOT_STANDING,
    NOT_TRADEABLE,
    NOT_TRADING,
    NOT_UNSHEATHED,
    NOT_WHILE_GHOST,
    NO_AMMO,
    NO_CHARGES_REMAIN,
    NO_CHAMPION,
    NO_COMBO_POINTS,
    NO_DUELING,
    NO_ENDURANCE,
    NO_FISH,
    NO_ITEMS_WHILE_SHAPESHIFTED,
    NO_MOUNTS_ALLOWED,
    NO_PET,
    NO_POWER,
    NOTHING_TO_DISPEL,
    NOTHING_TO_STEAL,
    ONLY_ABOVEWATER,
    ONLY_DAYTIME,
    ONLY_INDOORS,
    ONLY_MOUNTED,
    ONLY_NIGHTTIME,
    ONLY_OUTDOORS,
    ONLY_SHAPESHIFT,
    ONLY_STEALTHED,
    ONLY_UNDERWATER,
    OUT_OF_RANGE,
    PACIFIED,
    POSSESSED,
    REQUIRES_AREA,
    REQUIRES_SPELL_FOCUS,
    ROOTED,
    SILENCED,
    SPELL_IN_PROGRESS,
    SPELL_LEARNED,
    SPELL_UNAVAILABLE,
    STUNNED,
    TARGETS_DEAD,
    TARGET_AFFECTING_COMBAT,
    TARGET_AURASTATE,
    TARGET_DUELING,
    TARGET_ENEMY,
    TARGET_ENRAGED,
    TARGET_FRIENDLY,
    TARGET_IN_COMBAT,
    TARGET_IS_PLAYER,
    TARGET_NOT_DEAD,
    TARGET_NOT_IN_PARTY,
    TARGET_NOT_LOOTED,
    TARGET_NOT_PLAYER,
    TARGET_NO_POCKETS,
    TARGET_NO_WEAPONS,
    TARGET_UNSKINNABLE,
    THIRST_SATIATED,
    TOO_CLOSE,
    TOO_MANY_OF_ITEM,
    TRAINING_POINTS,
    TRY_AGAIN,
    UNIT_NOT_BEHIND,
    UNIT_NOT_INFRONT,
    WRONG_PET_FOOD,
    NOT_WHILE_FATIGUED,
    TARGET_NOT_IN_INSTANCE,
    NOT_WHILE_TRADING,
    TARGET_NOT_IN_RAID,
    DISENCHANT_WHILE_LOOTING,
    PROSPECT_WHILE_LOOTING,
    TARGET_FREEFORALL,
    NO_EDIBLE_CORPSES,
    ONLY_BATTLEGROUNDS,
    TARGET_NOT_GHOST,
    TOO_MANY_SKILLS,
    CANT_USE_NEW_ITEM,
    WRONG_WEATHER,
    DAMAGE_IMMUNE,
    PREVENTED_BY_MECHANIC,
    PLAY_TIME,
    REPUTATION,
    MIN_SKILL,
    UNKNOWN,
}

impl ReadableAndWritable for SpellCastResult {
    type Error = SpellCastResultError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl SpellCastResult {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellCastResultError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellCastResultError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellCastResultError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellCastResultError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellCastResultError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellCastResultError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::AFFECTING_COMBAT => 0x0,
            Self::ALREADY_AT_FULL_HEALTH => 0x1,
            Self::ALREADY_AT_FULL_MANA => 0x2,
            Self::ALREADY_BEING_TAMED => 0x3,
            Self::ALREADY_HAVE_CHARM => 0x4,
            Self::ALREADY_HAVE_SUMMON => 0x5,
            Self::ALREADY_OPEN => 0x6,
            Self::MORE_POWERFUL_SPELL_ACTIVE => 0x7,
            Self::BAD_IMPLICIT_TARGETS => 0x9,
            Self::BAD_TARGETS => 0xa,
            Self::CANT_BE_CHARMED => 0xb,
            Self::CANT_BE_DISENCHANTED => 0xc,
            Self::CANT_BE_PROSPECTED => 0xd,
            Self::CANT_CAST_ON_TAPPED => 0xe,
            Self::CANT_DUEL_WHILE_INVISIBLE => 0xf,
            Self::CANT_DUEL_WHILE_STEALTHED => 0x10,
            Self::CANT_TOO_CLOSE_TO_ENEMY => 0x11,
            Self::CANT_DO_THAT_YET => 0x12,
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
            Self::IMMUNE => 0x22,
            Self::INTERRUPTED => 0x23,
            Self::INTERRUPTED_COMBAT => 0x24,
            Self::ITEM_ALREADY_ENCHANTED => 0x25,
            Self::ITEM_GONE => 0x26,
            Self::ENCHANT_NOT_EXISTING_ITEM => 0x27,
            Self::ITEM_NOT_READY => 0x28,
            Self::LEVEL_REQUIREMENT => 0x29,
            Self::LINE_OF_SIGHT => 0x2a,
            Self::LOWLEVEL => 0x2b,
            Self::SKILL_NOT_HIGH_ENOUGH => 0x2c,
            Self::MAINHAND_EMPTY => 0x2d,
            Self::MOVING => 0x2e,
            Self::NEED_AMMO => 0x2f,
            Self::NEED_REQUIRES_SOMETHING => 0x30,
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
            Self::TARGET_FREEFORALL => 0x85,
            Self::NO_EDIBLE_CORPSES => 0x86,
            Self::ONLY_BATTLEGROUNDS => 0x87,
            Self::TARGET_NOT_GHOST => 0x88,
            Self::TOO_MANY_SKILLS => 0x89,
            Self::CANT_USE_NEW_ITEM => 0x8a,
            Self::WRONG_WEATHER => 0x8b,
            Self::DAMAGE_IMMUNE => 0x8c,
            Self::PREVENTED_BY_MECHANIC => 0x8d,
            Self::PLAY_TIME => 0x8e,
            Self::REPUTATION => 0x8f,
            Self::MIN_SKILL => 0x90,
            Self::UNKNOWN => 0x91,
        }
    }

    pub const fn new() -> Self {
        Self::AFFECTING_COMBAT
    }

}

impl ConstantSized for SpellCastResult {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SpellCastResult {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for SpellCastResult {
    fn default() -> Self {
        Self::AFFECTING_COMBAT
    }
}

impl std::fmt::Display for SpellCastResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AFFECTING_COMBAT => f.write_str("AFFECTING_COMBAT"),
            Self::ALREADY_AT_FULL_HEALTH => f.write_str("ALREADY_AT_FULL_HEALTH"),
            Self::ALREADY_AT_FULL_MANA => f.write_str("ALREADY_AT_FULL_MANA"),
            Self::ALREADY_BEING_TAMED => f.write_str("ALREADY_BEING_TAMED"),
            Self::ALREADY_HAVE_CHARM => f.write_str("ALREADY_HAVE_CHARM"),
            Self::ALREADY_HAVE_SUMMON => f.write_str("ALREADY_HAVE_SUMMON"),
            Self::ALREADY_OPEN => f.write_str("ALREADY_OPEN"),
            Self::MORE_POWERFUL_SPELL_ACTIVE => f.write_str("MORE_POWERFUL_SPELL_ACTIVE"),
            Self::BAD_IMPLICIT_TARGETS => f.write_str("BAD_IMPLICIT_TARGETS"),
            Self::BAD_TARGETS => f.write_str("BAD_TARGETS"),
            Self::CANT_BE_CHARMED => f.write_str("CANT_BE_CHARMED"),
            Self::CANT_BE_DISENCHANTED => f.write_str("CANT_BE_DISENCHANTED"),
            Self::CANT_BE_PROSPECTED => f.write_str("CANT_BE_PROSPECTED"),
            Self::CANT_CAST_ON_TAPPED => f.write_str("CANT_CAST_ON_TAPPED"),
            Self::CANT_DUEL_WHILE_INVISIBLE => f.write_str("CANT_DUEL_WHILE_INVISIBLE"),
            Self::CANT_DUEL_WHILE_STEALTHED => f.write_str("CANT_DUEL_WHILE_STEALTHED"),
            Self::CANT_TOO_CLOSE_TO_ENEMY => f.write_str("CANT_TOO_CLOSE_TO_ENEMY"),
            Self::CANT_DO_THAT_YET => f.write_str("CANT_DO_THAT_YET"),
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
            Self::IMMUNE => f.write_str("IMMUNE"),
            Self::INTERRUPTED => f.write_str("INTERRUPTED"),
            Self::INTERRUPTED_COMBAT => f.write_str("INTERRUPTED_COMBAT"),
            Self::ITEM_ALREADY_ENCHANTED => f.write_str("ITEM_ALREADY_ENCHANTED"),
            Self::ITEM_GONE => f.write_str("ITEM_GONE"),
            Self::ENCHANT_NOT_EXISTING_ITEM => f.write_str("ENCHANT_NOT_EXISTING_ITEM"),
            Self::ITEM_NOT_READY => f.write_str("ITEM_NOT_READY"),
            Self::LEVEL_REQUIREMENT => f.write_str("LEVEL_REQUIREMENT"),
            Self::LINE_OF_SIGHT => f.write_str("LINE_OF_SIGHT"),
            Self::LOWLEVEL => f.write_str("LOWLEVEL"),
            Self::SKILL_NOT_HIGH_ENOUGH => f.write_str("SKILL_NOT_HIGH_ENOUGH"),
            Self::MAINHAND_EMPTY => f.write_str("MAINHAND_EMPTY"),
            Self::MOVING => f.write_str("MOVING"),
            Self::NEED_AMMO => f.write_str("NEED_AMMO"),
            Self::NEED_REQUIRES_SOMETHING => f.write_str("NEED_REQUIRES_SOMETHING"),
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
            Self::TARGET_FREEFORALL => f.write_str("TARGET_FREEFORALL"),
            Self::NO_EDIBLE_CORPSES => f.write_str("NO_EDIBLE_CORPSES"),
            Self::ONLY_BATTLEGROUNDS => f.write_str("ONLY_BATTLEGROUNDS"),
            Self::TARGET_NOT_GHOST => f.write_str("TARGET_NOT_GHOST"),
            Self::TOO_MANY_SKILLS => f.write_str("TOO_MANY_SKILLS"),
            Self::CANT_USE_NEW_ITEM => f.write_str("CANT_USE_NEW_ITEM"),
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

impl TryFrom<u8> for SpellCastResult {
    type Error = TryFromSpellCastResultError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AFFECTING_COMBAT),
            1 => Ok(Self::ALREADY_AT_FULL_HEALTH),
            2 => Ok(Self::ALREADY_AT_FULL_MANA),
            3 => Ok(Self::ALREADY_BEING_TAMED),
            4 => Ok(Self::ALREADY_HAVE_CHARM),
            5 => Ok(Self::ALREADY_HAVE_SUMMON),
            6 => Ok(Self::ALREADY_OPEN),
            7 => Ok(Self::MORE_POWERFUL_SPELL_ACTIVE),
            9 => Ok(Self::BAD_IMPLICIT_TARGETS),
            10 => Ok(Self::BAD_TARGETS),
            11 => Ok(Self::CANT_BE_CHARMED),
            12 => Ok(Self::CANT_BE_DISENCHANTED),
            13 => Ok(Self::CANT_BE_PROSPECTED),
            14 => Ok(Self::CANT_CAST_ON_TAPPED),
            15 => Ok(Self::CANT_DUEL_WHILE_INVISIBLE),
            16 => Ok(Self::CANT_DUEL_WHILE_STEALTHED),
            17 => Ok(Self::CANT_TOO_CLOSE_TO_ENEMY),
            18 => Ok(Self::CANT_DO_THAT_YET),
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
            34 => Ok(Self::IMMUNE),
            35 => Ok(Self::INTERRUPTED),
            36 => Ok(Self::INTERRUPTED_COMBAT),
            37 => Ok(Self::ITEM_ALREADY_ENCHANTED),
            38 => Ok(Self::ITEM_GONE),
            39 => Ok(Self::ENCHANT_NOT_EXISTING_ITEM),
            40 => Ok(Self::ITEM_NOT_READY),
            41 => Ok(Self::LEVEL_REQUIREMENT),
            42 => Ok(Self::LINE_OF_SIGHT),
            43 => Ok(Self::LOWLEVEL),
            44 => Ok(Self::SKILL_NOT_HIGH_ENOUGH),
            45 => Ok(Self::MAINHAND_EMPTY),
            46 => Ok(Self::MOVING),
            47 => Ok(Self::NEED_AMMO),
            48 => Ok(Self::NEED_REQUIRES_SOMETHING),
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
            133 => Ok(Self::TARGET_FREEFORALL),
            134 => Ok(Self::NO_EDIBLE_CORPSES),
            135 => Ok(Self::ONLY_BATTLEGROUNDS),
            136 => Ok(Self::TARGET_NOT_GHOST),
            137 => Ok(Self::TOO_MANY_SKILLS),
            138 => Ok(Self::CANT_USE_NEW_ITEM),
            139 => Ok(Self::WRONG_WEATHER),
            140 => Ok(Self::DAMAGE_IMMUNE),
            141 => Ok(Self::PREVENTED_BY_MECHANIC),
            142 => Ok(Self::PLAY_TIME),
            143 => Ok(Self::REPUTATION),
            144 => Ok(Self::MIN_SKILL),
            145 => Ok(Self::UNKNOWN),
            _ => Err(TryFromSpellCastResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromSpellCastResultError {
    value: u8,
}

impl TryFromSpellCastResultError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum SpellCastResultError {
    Read(std::io::Error),
    TryFrom(TryFromSpellCastResultError),
}

impl std::error::Error for SpellCastResultError {}
impl std::fmt::Display for TryFromSpellCastResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'SpellCastResult': '{}'", self.value))
    }
}

impl std::fmt::Display for SpellCastResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for SpellCastResultError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromSpellCastResultError> for SpellCastResultError {
    fn from(value: TryFromSpellCastResultError) -> Self {
        Self::TryFrom(value)
    }
}

