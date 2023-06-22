/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:656`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L656):
/// ```text
/// enum SpellCastResult : u8 {
///     SUCCESS = 0x00;
///     AFFECTING_COMBAT = 0x01;
///     ALREADY_AT_FULL_HEALTH = 0x02;
///     ALREADY_AT_FULL_MANA = 0x03;
///     ALREADY_AT_FULL_POWER = 0x04;
///     ALREADY_BEING_TAMED = 0x05;
///     ALREADY_HAVE_CHARM = 0x06;
///     ALREADY_HAVE_SUMMON = 0x07;
///     ALREADY_OPEN = 0x08;
///     AURA_BOUNCED = 0x09;
///     AUTOTRACK_INTERRUPTED = 0x0A;
///     BAD_IMPLICIT_TARGETS = 0x0B;
///     BAD_TARGETS = 0x0C;
///     CANT_BE_CHARMED = 0x0D;
///     CANT_BE_DISENCHANTED = 0x0E;
///     CANT_BE_DISENCHANTED_SKILL = 0x0F;
///     CANT_BE_MILLED = 0x10;
///     CANT_BE_PROSPECTED = 0x11;
///     CANT_CAST_ON_TAPPED = 0x12;
///     CANT_DUEL_WHILE_INVISIBLE = 0x13;
///     CANT_DUEL_WHILE_STEALTHED = 0x14;
///     CANT_STEALTH = 0x15;
///     CASTER_AURASTATE = 0x16;
///     CASTER_DEAD = 0x17;
///     CHARMED = 0x18;
///     CHEST_IN_USE = 0x19;
///     CONFUSED = 0x1A;
///     DONT_REPORT = 0x1B;
///     EQUIPPED_ITEM = 0x1C;
///     EQUIPPED_ITEM_CLASS = 0x1D;
///     EQUIPPED_ITEM_CLASS_MAINHAND = 0x1E;
///     EQUIPPED_ITEM_CLASS_OFFHAND = 0x1F;
///     ERROR = 0x20;
///     FIZZLE = 0x21;
///     FLEEING = 0x22;
///     FOOD_LOWLEVEL = 0x23;
///     HIGHLEVEL = 0x24;
///     HUNGER_SATIATED = 0x25;
///     IMMUNE = 0x26;
///     INCORRECT_AREA = 0x27;
///     INTERRUPTED = 0x28;
///     INTERRUPTED_COMBAT = 0x29;
///     ITEM_ALREADY_ENCHANTED = 0x2A;
///     ITEM_GONE = 0x2B;
///     ITEM_NOT_FOUND = 0x2C;
///     ITEM_NOT_READY = 0x2D;
///     LEVEL_REQUIREMENT = 0x2E;
///     LINE_OF_SIGHT = 0x2F;
///     LOWLEVEL = 0x30;
///     LOW_CASTLEVEL = 0x31;
///     MAINHAND_EMPTY = 0x32;
///     MOVING = 0x33;
///     NEED_AMMO = 0x34;
///     NEED_AMMO_POUCH = 0x35;
///     NEED_EXOTIC_AMMO = 0x36;
///     NEED_MORE_ITEMS = 0x37;
///     NOPATH = 0x38;
///     NOT_BEHIND = 0x39;
///     NOT_FISHABLE = 0x3A;
///     NOT_FLYING = 0x3B;
///     NOT_HERE = 0x3C;
///     NOT_INFRONT = 0x3D;
///     NOT_IN_CONTROL = 0x3E;
///     NOT_KNOWN = 0x3F;
///     NOT_MOUNTED = 0x40;
///     NOT_ON_TAXI = 0x41;
///     NOT_ON_TRANSPORT = 0x42;
///     NOT_READY = 0x43;
///     NOT_SHAPESHIFT = 0x44;
///     NOT_STANDING = 0x45;
///     NOT_TRADEABLE = 0x46;
///     NOT_TRADING = 0x47;
///     NOT_UNSHEATHED = 0x48;
///     NOT_WHILE_GHOST = 0x49;
///     NOT_WHILE_LOOTING = 0x4A;
///     NO_AMMO = 0x4B;
///     NO_CHARGES_REMAIN = 0x4C;
///     NO_CHAMPION = 0x4D;
///     NO_COMBO_POINTS = 0x4E;
///     NO_DUELING = 0x4F;
///     NO_ENDURANCE = 0x50;
///     NO_FISH = 0x51;
///     NO_ITEMS_WHILE_SHAPESHIFTED = 0x52;
///     NO_MOUNTS_ALLOWED = 0x53;
///     NO_PET = 0x54;
///     NO_POWER = 0x55;
///     NOTHING_TO_DISPEL = 0x56;
///     NOTHING_TO_STEAL = 0x57;
///     ONLY_ABOVEWATER = 0x58;
///     ONLY_DAYTIME = 0x59;
///     ONLY_INDOORS = 0x5A;
///     ONLY_MOUNTED = 0x5B;
///     ONLY_NIGHTTIME = 0x5C;
///     ONLY_OUTDOORS = 0x5D;
///     ONLY_SHAPESHIFT = 0x5E;
///     ONLY_STEALTHED = 0x5F;
///     ONLY_UNDERWATER = 0x60;
///     OUT_OF_RANGE = 0x61;
///     PACIFIED = 0x62;
///     POSSESSED = 0x63;
///     REAGENTS = 0x64;
///     REQUIRES_AREA = 0x65;
///     REQUIRES_SPELL_FOCUS = 0x66;
///     ROOTED = 0x67;
///     SILENCED = 0x68;
///     SPELL_IN_PROGRESS = 0x69;
///     SPELL_LEARNED = 0x6A;
///     SPELL_UNAVAILABLE = 0x6B;
///     STUNNED = 0x6C;
///     TARGETS_DEAD = 0x6D;
///     TARGET_AFFECTING_COMBAT = 0x6E;
///     TARGET_AURASTATE = 0x6F;
///     TARGET_DUELING = 0x70;
///     TARGET_ENEMY = 0x71;
///     TARGET_ENRAGED = 0x72;
///     TARGET_FRIENDLY = 0x73;
///     TARGET_IN_COMBAT = 0x74;
///     TARGET_IS_PLAYER = 0x75;
///     TARGET_IS_PLAYER_CONTROLLED = 0x76;
///     TARGET_NOT_DEAD = 0x77;
///     TARGET_NOT_IN_PARTY = 0x78;
///     TARGET_NOT_LOOTED = 0x79;
///     TARGET_NOT_PLAYER = 0x7A;
///     TARGET_NO_POCKETS = 0x7B;
///     TARGET_NO_WEAPONS = 0x7C;
///     TARGET_NO_RANGED_WEAPONS = 0x7D;
///     TARGET_UNSKINNABLE = 0x7E;
///     THIRST_SATIATED = 0x7F;
///     TOO_CLOSE = 0x80;
///     TOO_MANY_OF_ITEM = 0x81;
///     TOTEM_CATEGORY = 0x82;
///     TOTEMS = 0x83;
///     TRY_AGAIN = 0x84;
///     UNIT_NOT_BEHIND = 0x85;
///     UNIT_NOT_INFRONT = 0x86;
///     WRONG_PET_FOOD = 0x87;
///     NOT_WHILE_FATIGUED = 0x88;
///     TARGET_NOT_IN_INSTANCE = 0x89;
///     NOT_WHILE_TRADING = 0x8A;
///     TARGET_NOT_IN_RAID = 0x8B;
///     TARGET_FREEFORALL = 0x8C;
///     NO_EDIBLE_CORPSES = 0x8D;
///     ONLY_BATTLEGROUNDS = 0x8E;
///     TARGET_NOT_GHOST = 0x8F;
///     TRANSFORM_UNUSABLE = 0x90;
///     WRONG_WEATHER = 0x91;
///     DAMAGE_IMMUNE = 0x92;
///     PREVENTED_BY_MECHANIC = 0x93;
///     PLAY_TIME = 0x94;
///     REPUTATION = 0x95;
///     MIN_SKILL = 0x96;
///     NOT_IN_ARENA = 0x97;
///     NOT_ON_SHAPESHIFT = 0x98;
///     NOT_ON_STEALTHED = 0x99;
///     NOT_ON_DAMAGE_IMMUNE = 0x9A;
///     NOT_ON_MOUNTED = 0x9B;
///     TOO_SHALLOW = 0x9C;
///     TARGET_NOT_IN_SANCTUARY = 0x9D;
///     TARGET_IS_TRIVIAL = 0x9E;
///     BM_OR_INVISGOD = 0x9F;
///     EXPERT_RIDING_REQUIREMENT = 0xA0;
///     ARTISAN_RIDING_REQUIREMENT = 0xA1;
///     NOT_IDLE = 0xA2;
///     NOT_INACTIVE = 0xA3;
///     PARTIAL_PLAYTIME = 0xA4;
///     NO_PLAYTIME = 0xA5;
///     NOT_IN_BATTLEGROUND = 0xA6;
///     NOT_IN_RAID_INSTANCE = 0xA7;
///     ONLY_IN_ARENA = 0xA8;
///     TARGET_LOCKED_TO_RAID_INSTANCE = 0xA9;
///     ON_USE_ENCHANT = 0xAA;
///     NOT_ON_GROUND = 0xAB;
///     CUSTOM_ERROR = 0xAC;
///     CANT_DO_THAT_RIGHT_NOW = 0xAD;
///     TOO_MANY_SOCKETS = 0xAE;
///     INVALID_GLYPH = 0xAF;
///     UNIQUE_GLYPH = 0xB0;
///     GLYPH_SOCKET_LOCKED = 0xB1;
///     NO_VALID_TARGETS = 0xB2;
///     ITEM_AT_MAX_CHARGES = 0xB3;
///     NOT_IN_BARBERSHOP = 0xB4;
///     FISHING_TOO_LOW = 0xB5;
///     ITEM_ENCHANT_TRADE_WINDOW = 0xB6;
///     SUMMON_PENDING = 0xB7;
///     MAX_SOCKETS = 0xB8;
///     PET_CAN_RENAME = 0xB9;
///     TARGET_CANNOT_BE_RESURRECTED = 0xBA;
///     UNKNOWN = 0xBB;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SpellCastResult {
    Success,
    AffectingCombat,
    AlreadyAtFullHealth,
    AlreadyAtFullMana,
    AlreadyAtFullPower,
    AlreadyBeingTamed,
    AlreadyHaveCharm,
    AlreadyHaveSummon,
    AlreadyOpen,
    AuraBounced,
    AutotrackInterrupted,
    BadImplicitTargets,
    BadTargets,
    CantBeCharmed,
    CantBeDisenchanted,
    CantBeDisenchantedSkill,
    CantBeMilled,
    CantBeProspected,
    CantCastOnTapped,
    CantDuelWhileInvisible,
    CantDuelWhileStealthed,
    CantStealth,
    CasterAurastate,
    CasterDead,
    Charmed,
    ChestInUse,
    Confused,
    DontReport,
    EquippedItem,
    EquippedItemClass,
    EquippedItemClassMainhand,
    EquippedItemClassOffhand,
    ErrorX,
    Fizzle,
    Fleeing,
    FoodLowlevel,
    Highlevel,
    HungerSatiated,
    Immune,
    IncorrectArea,
    Interrupted,
    InterruptedCombat,
    ItemAlreadyEnchanted,
    ItemGone,
    ItemNotFound,
    ItemNotReady,
    LevelRequirement,
    LineOfSight,
    Lowlevel,
    LowCastlevel,
    MainhandEmpty,
    Moving,
    NeedAmmo,
    NeedAmmoPouch,
    NeedExoticAmmo,
    NeedMoreItems,
    Nopath,
    NotBehind,
    NotFishable,
    NotFlying,
    NotHere,
    NotInfront,
    NotInControl,
    NotKnown,
    NotMounted,
    NotOnTaxi,
    NotOnTransport,
    NotReady,
    NotShapeshift,
    NotStanding,
    NotTradeable,
    NotTrading,
    NotUnsheathed,
    NotWhileGhost,
    NotWhileLooting,
    NoAmmo,
    NoChargesRemain,
    NoChampion,
    NoComboPoints,
    NoDueling,
    NoEndurance,
    NoFish,
    NoItemsWhileShapeshifted,
    NoMountsAllowed,
    NoPet,
    NoPower,
    NothingToDispel,
    NothingToSteal,
    OnlyAbovewater,
    OnlyDaytime,
    OnlyIndoors,
    OnlyMounted,
    OnlyNighttime,
    OnlyOutdoors,
    OnlyShapeshift,
    OnlyStealthed,
    OnlyUnderwater,
    OutOfRange,
    Pacified,
    Possessed,
    Reagents,
    RequiresArea,
    RequiresSpellFocus,
    Rooted,
    Silenced,
    SpellInProgress,
    SpellLearned,
    SpellUnavailable,
    Stunned,
    TargetsDead,
    TargetAffectingCombat,
    TargetAurastate,
    TargetDueling,
    TargetEnemy,
    TargetEnraged,
    TargetFriendly,
    TargetInCombat,
    TargetIsPlayer,
    TargetIsPlayerControlled,
    TargetNotDead,
    TargetNotInParty,
    TargetNotLooted,
    TargetNotPlayer,
    TargetNoPockets,
    TargetNoWeapons,
    TargetNoRangedWeapons,
    TargetUnskinnable,
    ThirstSatiated,
    TooClose,
    TooManyOfItem,
    TotemCategory,
    Totems,
    TryAgain,
    UnitNotBehind,
    UnitNotInfront,
    WrongPetFood,
    NotWhileFatigued,
    TargetNotInInstance,
    NotWhileTrading,
    TargetNotInRaid,
    TargetFreeforall,
    NoEdibleCorpses,
    OnlyBattlegrounds,
    TargetNotGhost,
    TransformUnusable,
    WrongWeather,
    DamageImmune,
    PreventedByMechanic,
    PlayTime,
    Reputation,
    MinSkill,
    NotInArena,
    NotOnShapeshift,
    NotOnStealthed,
    NotOnDamageImmune,
    NotOnMounted,
    TooShallow,
    TargetNotInSanctuary,
    TargetIsTrivial,
    BmOrInvisgod,
    ExpertRidingRequirement,
    ArtisanRidingRequirement,
    NotIdle,
    NotInactive,
    PartialPlaytime,
    NoPlaytime,
    NotInBattleground,
    NotInRaidInstance,
    OnlyInArena,
    TargetLockedToRaidInstance,
    OnUseEnchant,
    NotOnGround,
    CustomError,
    CantDoThatRightNow,
    TooManySockets,
    InvalidGlyph,
    UniqueGlyph,
    GlyphSocketLocked,
    NoValidTargets,
    ItemAtMaxCharges,
    NotInBarbershop,
    FishingTooLow,
    ItemEnchantTradeWindow,
    SummonPending,
    MaxSockets,
    PetCanRename,
    TargetCannotBeResurrected,
    Unknown,
}

impl SpellCastResult {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Success => 0x0,
            Self::AffectingCombat => 0x1,
            Self::AlreadyAtFullHealth => 0x2,
            Self::AlreadyAtFullMana => 0x3,
            Self::AlreadyAtFullPower => 0x4,
            Self::AlreadyBeingTamed => 0x5,
            Self::AlreadyHaveCharm => 0x6,
            Self::AlreadyHaveSummon => 0x7,
            Self::AlreadyOpen => 0x8,
            Self::AuraBounced => 0x9,
            Self::AutotrackInterrupted => 0xa,
            Self::BadImplicitTargets => 0xb,
            Self::BadTargets => 0xc,
            Self::CantBeCharmed => 0xd,
            Self::CantBeDisenchanted => 0xe,
            Self::CantBeDisenchantedSkill => 0xf,
            Self::CantBeMilled => 0x10,
            Self::CantBeProspected => 0x11,
            Self::CantCastOnTapped => 0x12,
            Self::CantDuelWhileInvisible => 0x13,
            Self::CantDuelWhileStealthed => 0x14,
            Self::CantStealth => 0x15,
            Self::CasterAurastate => 0x16,
            Self::CasterDead => 0x17,
            Self::Charmed => 0x18,
            Self::ChestInUse => 0x19,
            Self::Confused => 0x1a,
            Self::DontReport => 0x1b,
            Self::EquippedItem => 0x1c,
            Self::EquippedItemClass => 0x1d,
            Self::EquippedItemClassMainhand => 0x1e,
            Self::EquippedItemClassOffhand => 0x1f,
            Self::ErrorX => 0x20,
            Self::Fizzle => 0x21,
            Self::Fleeing => 0x22,
            Self::FoodLowlevel => 0x23,
            Self::Highlevel => 0x24,
            Self::HungerSatiated => 0x25,
            Self::Immune => 0x26,
            Self::IncorrectArea => 0x27,
            Self::Interrupted => 0x28,
            Self::InterruptedCombat => 0x29,
            Self::ItemAlreadyEnchanted => 0x2a,
            Self::ItemGone => 0x2b,
            Self::ItemNotFound => 0x2c,
            Self::ItemNotReady => 0x2d,
            Self::LevelRequirement => 0x2e,
            Self::LineOfSight => 0x2f,
            Self::Lowlevel => 0x30,
            Self::LowCastlevel => 0x31,
            Self::MainhandEmpty => 0x32,
            Self::Moving => 0x33,
            Self::NeedAmmo => 0x34,
            Self::NeedAmmoPouch => 0x35,
            Self::NeedExoticAmmo => 0x36,
            Self::NeedMoreItems => 0x37,
            Self::Nopath => 0x38,
            Self::NotBehind => 0x39,
            Self::NotFishable => 0x3a,
            Self::NotFlying => 0x3b,
            Self::NotHere => 0x3c,
            Self::NotInfront => 0x3d,
            Self::NotInControl => 0x3e,
            Self::NotKnown => 0x3f,
            Self::NotMounted => 0x40,
            Self::NotOnTaxi => 0x41,
            Self::NotOnTransport => 0x42,
            Self::NotReady => 0x43,
            Self::NotShapeshift => 0x44,
            Self::NotStanding => 0x45,
            Self::NotTradeable => 0x46,
            Self::NotTrading => 0x47,
            Self::NotUnsheathed => 0x48,
            Self::NotWhileGhost => 0x49,
            Self::NotWhileLooting => 0x4a,
            Self::NoAmmo => 0x4b,
            Self::NoChargesRemain => 0x4c,
            Self::NoChampion => 0x4d,
            Self::NoComboPoints => 0x4e,
            Self::NoDueling => 0x4f,
            Self::NoEndurance => 0x50,
            Self::NoFish => 0x51,
            Self::NoItemsWhileShapeshifted => 0x52,
            Self::NoMountsAllowed => 0x53,
            Self::NoPet => 0x54,
            Self::NoPower => 0x55,
            Self::NothingToDispel => 0x56,
            Self::NothingToSteal => 0x57,
            Self::OnlyAbovewater => 0x58,
            Self::OnlyDaytime => 0x59,
            Self::OnlyIndoors => 0x5a,
            Self::OnlyMounted => 0x5b,
            Self::OnlyNighttime => 0x5c,
            Self::OnlyOutdoors => 0x5d,
            Self::OnlyShapeshift => 0x5e,
            Self::OnlyStealthed => 0x5f,
            Self::OnlyUnderwater => 0x60,
            Self::OutOfRange => 0x61,
            Self::Pacified => 0x62,
            Self::Possessed => 0x63,
            Self::Reagents => 0x64,
            Self::RequiresArea => 0x65,
            Self::RequiresSpellFocus => 0x66,
            Self::Rooted => 0x67,
            Self::Silenced => 0x68,
            Self::SpellInProgress => 0x69,
            Self::SpellLearned => 0x6a,
            Self::SpellUnavailable => 0x6b,
            Self::Stunned => 0x6c,
            Self::TargetsDead => 0x6d,
            Self::TargetAffectingCombat => 0x6e,
            Self::TargetAurastate => 0x6f,
            Self::TargetDueling => 0x70,
            Self::TargetEnemy => 0x71,
            Self::TargetEnraged => 0x72,
            Self::TargetFriendly => 0x73,
            Self::TargetInCombat => 0x74,
            Self::TargetIsPlayer => 0x75,
            Self::TargetIsPlayerControlled => 0x76,
            Self::TargetNotDead => 0x77,
            Self::TargetNotInParty => 0x78,
            Self::TargetNotLooted => 0x79,
            Self::TargetNotPlayer => 0x7a,
            Self::TargetNoPockets => 0x7b,
            Self::TargetNoWeapons => 0x7c,
            Self::TargetNoRangedWeapons => 0x7d,
            Self::TargetUnskinnable => 0x7e,
            Self::ThirstSatiated => 0x7f,
            Self::TooClose => 0x80,
            Self::TooManyOfItem => 0x81,
            Self::TotemCategory => 0x82,
            Self::Totems => 0x83,
            Self::TryAgain => 0x84,
            Self::UnitNotBehind => 0x85,
            Self::UnitNotInfront => 0x86,
            Self::WrongPetFood => 0x87,
            Self::NotWhileFatigued => 0x88,
            Self::TargetNotInInstance => 0x89,
            Self::NotWhileTrading => 0x8a,
            Self::TargetNotInRaid => 0x8b,
            Self::TargetFreeforall => 0x8c,
            Self::NoEdibleCorpses => 0x8d,
            Self::OnlyBattlegrounds => 0x8e,
            Self::TargetNotGhost => 0x8f,
            Self::TransformUnusable => 0x90,
            Self::WrongWeather => 0x91,
            Self::DamageImmune => 0x92,
            Self::PreventedByMechanic => 0x93,
            Self::PlayTime => 0x94,
            Self::Reputation => 0x95,
            Self::MinSkill => 0x96,
            Self::NotInArena => 0x97,
            Self::NotOnShapeshift => 0x98,
            Self::NotOnStealthed => 0x99,
            Self::NotOnDamageImmune => 0x9a,
            Self::NotOnMounted => 0x9b,
            Self::TooShallow => 0x9c,
            Self::TargetNotInSanctuary => 0x9d,
            Self::TargetIsTrivial => 0x9e,
            Self::BmOrInvisgod => 0x9f,
            Self::ExpertRidingRequirement => 0xa0,
            Self::ArtisanRidingRequirement => 0xa1,
            Self::NotIdle => 0xa2,
            Self::NotInactive => 0xa3,
            Self::PartialPlaytime => 0xa4,
            Self::NoPlaytime => 0xa5,
            Self::NotInBattleground => 0xa6,
            Self::NotInRaidInstance => 0xa7,
            Self::OnlyInArena => 0xa8,
            Self::TargetLockedToRaidInstance => 0xa9,
            Self::OnUseEnchant => 0xaa,
            Self::NotOnGround => 0xab,
            Self::CustomError => 0xac,
            Self::CantDoThatRightNow => 0xad,
            Self::TooManySockets => 0xae,
            Self::InvalidGlyph => 0xaf,
            Self::UniqueGlyph => 0xb0,
            Self::GlyphSocketLocked => 0xb1,
            Self::NoValidTargets => 0xb2,
            Self::ItemAtMaxCharges => 0xb3,
            Self::NotInBarbershop => 0xb4,
            Self::FishingTooLow => 0xb5,
            Self::ItemEnchantTradeWindow => 0xb6,
            Self::SummonPending => 0xb7,
            Self::MaxSockets => 0xb8,
            Self::PetCanRename => 0xb9,
            Self::TargetCannotBeResurrected => 0xba,
            Self::Unknown => 0xbb,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl SpellCastResult {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Success => "SUCCESS",
            Self::AffectingCombat => "AFFECTING_COMBAT",
            Self::AlreadyAtFullHealth => "ALREADY_AT_FULL_HEALTH",
            Self::AlreadyAtFullMana => "ALREADY_AT_FULL_MANA",
            Self::AlreadyAtFullPower => "ALREADY_AT_FULL_POWER",
            Self::AlreadyBeingTamed => "ALREADY_BEING_TAMED",
            Self::AlreadyHaveCharm => "ALREADY_HAVE_CHARM",
            Self::AlreadyHaveSummon => "ALREADY_HAVE_SUMMON",
            Self::AlreadyOpen => "ALREADY_OPEN",
            Self::AuraBounced => "AURA_BOUNCED",
            Self::AutotrackInterrupted => "AUTOTRACK_INTERRUPTED",
            Self::BadImplicitTargets => "BAD_IMPLICIT_TARGETS",
            Self::BadTargets => "BAD_TARGETS",
            Self::CantBeCharmed => "CANT_BE_CHARMED",
            Self::CantBeDisenchanted => "CANT_BE_DISENCHANTED",
            Self::CantBeDisenchantedSkill => "CANT_BE_DISENCHANTED_SKILL",
            Self::CantBeMilled => "CANT_BE_MILLED",
            Self::CantBeProspected => "CANT_BE_PROSPECTED",
            Self::CantCastOnTapped => "CANT_CAST_ON_TAPPED",
            Self::CantDuelWhileInvisible => "CANT_DUEL_WHILE_INVISIBLE",
            Self::CantDuelWhileStealthed => "CANT_DUEL_WHILE_STEALTHED",
            Self::CantStealth => "CANT_STEALTH",
            Self::CasterAurastate => "CASTER_AURASTATE",
            Self::CasterDead => "CASTER_DEAD",
            Self::Charmed => "CHARMED",
            Self::ChestInUse => "CHEST_IN_USE",
            Self::Confused => "CONFUSED",
            Self::DontReport => "DONT_REPORT",
            Self::EquippedItem => "EQUIPPED_ITEM",
            Self::EquippedItemClass => "EQUIPPED_ITEM_CLASS",
            Self::EquippedItemClassMainhand => "EQUIPPED_ITEM_CLASS_MAINHAND",
            Self::EquippedItemClassOffhand => "EQUIPPED_ITEM_CLASS_OFFHAND",
            Self::ErrorX => "ERROR",
            Self::Fizzle => "FIZZLE",
            Self::Fleeing => "FLEEING",
            Self::FoodLowlevel => "FOOD_LOWLEVEL",
            Self::Highlevel => "HIGHLEVEL",
            Self::HungerSatiated => "HUNGER_SATIATED",
            Self::Immune => "IMMUNE",
            Self::IncorrectArea => "INCORRECT_AREA",
            Self::Interrupted => "INTERRUPTED",
            Self::InterruptedCombat => "INTERRUPTED_COMBAT",
            Self::ItemAlreadyEnchanted => "ITEM_ALREADY_ENCHANTED",
            Self::ItemGone => "ITEM_GONE",
            Self::ItemNotFound => "ITEM_NOT_FOUND",
            Self::ItemNotReady => "ITEM_NOT_READY",
            Self::LevelRequirement => "LEVEL_REQUIREMENT",
            Self::LineOfSight => "LINE_OF_SIGHT",
            Self::Lowlevel => "LOWLEVEL",
            Self::LowCastlevel => "LOW_CASTLEVEL",
            Self::MainhandEmpty => "MAINHAND_EMPTY",
            Self::Moving => "MOVING",
            Self::NeedAmmo => "NEED_AMMO",
            Self::NeedAmmoPouch => "NEED_AMMO_POUCH",
            Self::NeedExoticAmmo => "NEED_EXOTIC_AMMO",
            Self::NeedMoreItems => "NEED_MORE_ITEMS",
            Self::Nopath => "NOPATH",
            Self::NotBehind => "NOT_BEHIND",
            Self::NotFishable => "NOT_FISHABLE",
            Self::NotFlying => "NOT_FLYING",
            Self::NotHere => "NOT_HERE",
            Self::NotInfront => "NOT_INFRONT",
            Self::NotInControl => "NOT_IN_CONTROL",
            Self::NotKnown => "NOT_KNOWN",
            Self::NotMounted => "NOT_MOUNTED",
            Self::NotOnTaxi => "NOT_ON_TAXI",
            Self::NotOnTransport => "NOT_ON_TRANSPORT",
            Self::NotReady => "NOT_READY",
            Self::NotShapeshift => "NOT_SHAPESHIFT",
            Self::NotStanding => "NOT_STANDING",
            Self::NotTradeable => "NOT_TRADEABLE",
            Self::NotTrading => "NOT_TRADING",
            Self::NotUnsheathed => "NOT_UNSHEATHED",
            Self::NotWhileGhost => "NOT_WHILE_GHOST",
            Self::NotWhileLooting => "NOT_WHILE_LOOTING",
            Self::NoAmmo => "NO_AMMO",
            Self::NoChargesRemain => "NO_CHARGES_REMAIN",
            Self::NoChampion => "NO_CHAMPION",
            Self::NoComboPoints => "NO_COMBO_POINTS",
            Self::NoDueling => "NO_DUELING",
            Self::NoEndurance => "NO_ENDURANCE",
            Self::NoFish => "NO_FISH",
            Self::NoItemsWhileShapeshifted => "NO_ITEMS_WHILE_SHAPESHIFTED",
            Self::NoMountsAllowed => "NO_MOUNTS_ALLOWED",
            Self::NoPet => "NO_PET",
            Self::NoPower => "NO_POWER",
            Self::NothingToDispel => "NOTHING_TO_DISPEL",
            Self::NothingToSteal => "NOTHING_TO_STEAL",
            Self::OnlyAbovewater => "ONLY_ABOVEWATER",
            Self::OnlyDaytime => "ONLY_DAYTIME",
            Self::OnlyIndoors => "ONLY_INDOORS",
            Self::OnlyMounted => "ONLY_MOUNTED",
            Self::OnlyNighttime => "ONLY_NIGHTTIME",
            Self::OnlyOutdoors => "ONLY_OUTDOORS",
            Self::OnlyShapeshift => "ONLY_SHAPESHIFT",
            Self::OnlyStealthed => "ONLY_STEALTHED",
            Self::OnlyUnderwater => "ONLY_UNDERWATER",
            Self::OutOfRange => "OUT_OF_RANGE",
            Self::Pacified => "PACIFIED",
            Self::Possessed => "POSSESSED",
            Self::Reagents => "REAGENTS",
            Self::RequiresArea => "REQUIRES_AREA",
            Self::RequiresSpellFocus => "REQUIRES_SPELL_FOCUS",
            Self::Rooted => "ROOTED",
            Self::Silenced => "SILENCED",
            Self::SpellInProgress => "SPELL_IN_PROGRESS",
            Self::SpellLearned => "SPELL_LEARNED",
            Self::SpellUnavailable => "SPELL_UNAVAILABLE",
            Self::Stunned => "STUNNED",
            Self::TargetsDead => "TARGETS_DEAD",
            Self::TargetAffectingCombat => "TARGET_AFFECTING_COMBAT",
            Self::TargetAurastate => "TARGET_AURASTATE",
            Self::TargetDueling => "TARGET_DUELING",
            Self::TargetEnemy => "TARGET_ENEMY",
            Self::TargetEnraged => "TARGET_ENRAGED",
            Self::TargetFriendly => "TARGET_FRIENDLY",
            Self::TargetInCombat => "TARGET_IN_COMBAT",
            Self::TargetIsPlayer => "TARGET_IS_PLAYER",
            Self::TargetIsPlayerControlled => "TARGET_IS_PLAYER_CONTROLLED",
            Self::TargetNotDead => "TARGET_NOT_DEAD",
            Self::TargetNotInParty => "TARGET_NOT_IN_PARTY",
            Self::TargetNotLooted => "TARGET_NOT_LOOTED",
            Self::TargetNotPlayer => "TARGET_NOT_PLAYER",
            Self::TargetNoPockets => "TARGET_NO_POCKETS",
            Self::TargetNoWeapons => "TARGET_NO_WEAPONS",
            Self::TargetNoRangedWeapons => "TARGET_NO_RANGED_WEAPONS",
            Self::TargetUnskinnable => "TARGET_UNSKINNABLE",
            Self::ThirstSatiated => "THIRST_SATIATED",
            Self::TooClose => "TOO_CLOSE",
            Self::TooManyOfItem => "TOO_MANY_OF_ITEM",
            Self::TotemCategory => "TOTEM_CATEGORY",
            Self::Totems => "TOTEMS",
            Self::TryAgain => "TRY_AGAIN",
            Self::UnitNotBehind => "UNIT_NOT_BEHIND",
            Self::UnitNotInfront => "UNIT_NOT_INFRONT",
            Self::WrongPetFood => "WRONG_PET_FOOD",
            Self::NotWhileFatigued => "NOT_WHILE_FATIGUED",
            Self::TargetNotInInstance => "TARGET_NOT_IN_INSTANCE",
            Self::NotWhileTrading => "NOT_WHILE_TRADING",
            Self::TargetNotInRaid => "TARGET_NOT_IN_RAID",
            Self::TargetFreeforall => "TARGET_FREEFORALL",
            Self::NoEdibleCorpses => "NO_EDIBLE_CORPSES",
            Self::OnlyBattlegrounds => "ONLY_BATTLEGROUNDS",
            Self::TargetNotGhost => "TARGET_NOT_GHOST",
            Self::TransformUnusable => "TRANSFORM_UNUSABLE",
            Self::WrongWeather => "WRONG_WEATHER",
            Self::DamageImmune => "DAMAGE_IMMUNE",
            Self::PreventedByMechanic => "PREVENTED_BY_MECHANIC",
            Self::PlayTime => "PLAY_TIME",
            Self::Reputation => "REPUTATION",
            Self::MinSkill => "MIN_SKILL",
            Self::NotInArena => "NOT_IN_ARENA",
            Self::NotOnShapeshift => "NOT_ON_SHAPESHIFT",
            Self::NotOnStealthed => "NOT_ON_STEALTHED",
            Self::NotOnDamageImmune => "NOT_ON_DAMAGE_IMMUNE",
            Self::NotOnMounted => "NOT_ON_MOUNTED",
            Self::TooShallow => "TOO_SHALLOW",
            Self::TargetNotInSanctuary => "TARGET_NOT_IN_SANCTUARY",
            Self::TargetIsTrivial => "TARGET_IS_TRIVIAL",
            Self::BmOrInvisgod => "BM_OR_INVISGOD",
            Self::ExpertRidingRequirement => "EXPERT_RIDING_REQUIREMENT",
            Self::ArtisanRidingRequirement => "ARTISAN_RIDING_REQUIREMENT",
            Self::NotIdle => "NOT_IDLE",
            Self::NotInactive => "NOT_INACTIVE",
            Self::PartialPlaytime => "PARTIAL_PLAYTIME",
            Self::NoPlaytime => "NO_PLAYTIME",
            Self::NotInBattleground => "NOT_IN_BATTLEGROUND",
            Self::NotInRaidInstance => "NOT_IN_RAID_INSTANCE",
            Self::OnlyInArena => "ONLY_IN_ARENA",
            Self::TargetLockedToRaidInstance => "TARGET_LOCKED_TO_RAID_INSTANCE",
            Self::OnUseEnchant => "ON_USE_ENCHANT",
            Self::NotOnGround => "NOT_ON_GROUND",
            Self::CustomError => "CUSTOM_ERROR",
            Self::CantDoThatRightNow => "CANT_DO_THAT_RIGHT_NOW",
            Self::TooManySockets => "TOO_MANY_SOCKETS",
            Self::InvalidGlyph => "INVALID_GLYPH",
            Self::UniqueGlyph => "UNIQUE_GLYPH",
            Self::GlyphSocketLocked => "GLYPH_SOCKET_LOCKED",
            Self::NoValidTargets => "NO_VALID_TARGETS",
            Self::ItemAtMaxCharges => "ITEM_AT_MAX_CHARGES",
            Self::NotInBarbershop => "NOT_IN_BARBERSHOP",
            Self::FishingTooLow => "FISHING_TOO_LOW",
            Self::ItemEnchantTradeWindow => "ITEM_ENCHANT_TRADE_WINDOW",
            Self::SummonPending => "SUMMON_PENDING",
            Self::MaxSockets => "MAX_SOCKETS",
            Self::PetCanRename => "PET_CAN_RENAME",
            Self::TargetCannotBeResurrected => "TARGET_CANNOT_BE_RESURRECTED",
            Self::Unknown => "UNKNOWN",
        }
    }

}

const NAME: &str = "SpellCastResult";

impl Default for SpellCastResult {
    fn default() -> Self {
        Self::Success
    }
}

impl std::fmt::Display for SpellCastResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => f.write_str("Success"),
            Self::AffectingCombat => f.write_str("AffectingCombat"),
            Self::AlreadyAtFullHealth => f.write_str("AlreadyAtFullHealth"),
            Self::AlreadyAtFullMana => f.write_str("AlreadyAtFullMana"),
            Self::AlreadyAtFullPower => f.write_str("AlreadyAtFullPower"),
            Self::AlreadyBeingTamed => f.write_str("AlreadyBeingTamed"),
            Self::AlreadyHaveCharm => f.write_str("AlreadyHaveCharm"),
            Self::AlreadyHaveSummon => f.write_str("AlreadyHaveSummon"),
            Self::AlreadyOpen => f.write_str("AlreadyOpen"),
            Self::AuraBounced => f.write_str("AuraBounced"),
            Self::AutotrackInterrupted => f.write_str("AutotrackInterrupted"),
            Self::BadImplicitTargets => f.write_str("BadImplicitTargets"),
            Self::BadTargets => f.write_str("BadTargets"),
            Self::CantBeCharmed => f.write_str("CantBeCharmed"),
            Self::CantBeDisenchanted => f.write_str("CantBeDisenchanted"),
            Self::CantBeDisenchantedSkill => f.write_str("CantBeDisenchantedSkill"),
            Self::CantBeMilled => f.write_str("CantBeMilled"),
            Self::CantBeProspected => f.write_str("CantBeProspected"),
            Self::CantCastOnTapped => f.write_str("CantCastOnTapped"),
            Self::CantDuelWhileInvisible => f.write_str("CantDuelWhileInvisible"),
            Self::CantDuelWhileStealthed => f.write_str("CantDuelWhileStealthed"),
            Self::CantStealth => f.write_str("CantStealth"),
            Self::CasterAurastate => f.write_str("CasterAurastate"),
            Self::CasterDead => f.write_str("CasterDead"),
            Self::Charmed => f.write_str("Charmed"),
            Self::ChestInUse => f.write_str("ChestInUse"),
            Self::Confused => f.write_str("Confused"),
            Self::DontReport => f.write_str("DontReport"),
            Self::EquippedItem => f.write_str("EquippedItem"),
            Self::EquippedItemClass => f.write_str("EquippedItemClass"),
            Self::EquippedItemClassMainhand => f.write_str("EquippedItemClassMainhand"),
            Self::EquippedItemClassOffhand => f.write_str("EquippedItemClassOffhand"),
            Self::ErrorX => f.write_str("ErrorX"),
            Self::Fizzle => f.write_str("Fizzle"),
            Self::Fleeing => f.write_str("Fleeing"),
            Self::FoodLowlevel => f.write_str("FoodLowlevel"),
            Self::Highlevel => f.write_str("Highlevel"),
            Self::HungerSatiated => f.write_str("HungerSatiated"),
            Self::Immune => f.write_str("Immune"),
            Self::IncorrectArea => f.write_str("IncorrectArea"),
            Self::Interrupted => f.write_str("Interrupted"),
            Self::InterruptedCombat => f.write_str("InterruptedCombat"),
            Self::ItemAlreadyEnchanted => f.write_str("ItemAlreadyEnchanted"),
            Self::ItemGone => f.write_str("ItemGone"),
            Self::ItemNotFound => f.write_str("ItemNotFound"),
            Self::ItemNotReady => f.write_str("ItemNotReady"),
            Self::LevelRequirement => f.write_str("LevelRequirement"),
            Self::LineOfSight => f.write_str("LineOfSight"),
            Self::Lowlevel => f.write_str("Lowlevel"),
            Self::LowCastlevel => f.write_str("LowCastlevel"),
            Self::MainhandEmpty => f.write_str("MainhandEmpty"),
            Self::Moving => f.write_str("Moving"),
            Self::NeedAmmo => f.write_str("NeedAmmo"),
            Self::NeedAmmoPouch => f.write_str("NeedAmmoPouch"),
            Self::NeedExoticAmmo => f.write_str("NeedExoticAmmo"),
            Self::NeedMoreItems => f.write_str("NeedMoreItems"),
            Self::Nopath => f.write_str("Nopath"),
            Self::NotBehind => f.write_str("NotBehind"),
            Self::NotFishable => f.write_str("NotFishable"),
            Self::NotFlying => f.write_str("NotFlying"),
            Self::NotHere => f.write_str("NotHere"),
            Self::NotInfront => f.write_str("NotInfront"),
            Self::NotInControl => f.write_str("NotInControl"),
            Self::NotKnown => f.write_str("NotKnown"),
            Self::NotMounted => f.write_str("NotMounted"),
            Self::NotOnTaxi => f.write_str("NotOnTaxi"),
            Self::NotOnTransport => f.write_str("NotOnTransport"),
            Self::NotReady => f.write_str("NotReady"),
            Self::NotShapeshift => f.write_str("NotShapeshift"),
            Self::NotStanding => f.write_str("NotStanding"),
            Self::NotTradeable => f.write_str("NotTradeable"),
            Self::NotTrading => f.write_str("NotTrading"),
            Self::NotUnsheathed => f.write_str("NotUnsheathed"),
            Self::NotWhileGhost => f.write_str("NotWhileGhost"),
            Self::NotWhileLooting => f.write_str("NotWhileLooting"),
            Self::NoAmmo => f.write_str("NoAmmo"),
            Self::NoChargesRemain => f.write_str("NoChargesRemain"),
            Self::NoChampion => f.write_str("NoChampion"),
            Self::NoComboPoints => f.write_str("NoComboPoints"),
            Self::NoDueling => f.write_str("NoDueling"),
            Self::NoEndurance => f.write_str("NoEndurance"),
            Self::NoFish => f.write_str("NoFish"),
            Self::NoItemsWhileShapeshifted => f.write_str("NoItemsWhileShapeshifted"),
            Self::NoMountsAllowed => f.write_str("NoMountsAllowed"),
            Self::NoPet => f.write_str("NoPet"),
            Self::NoPower => f.write_str("NoPower"),
            Self::NothingToDispel => f.write_str("NothingToDispel"),
            Self::NothingToSteal => f.write_str("NothingToSteal"),
            Self::OnlyAbovewater => f.write_str("OnlyAbovewater"),
            Self::OnlyDaytime => f.write_str("OnlyDaytime"),
            Self::OnlyIndoors => f.write_str("OnlyIndoors"),
            Self::OnlyMounted => f.write_str("OnlyMounted"),
            Self::OnlyNighttime => f.write_str("OnlyNighttime"),
            Self::OnlyOutdoors => f.write_str("OnlyOutdoors"),
            Self::OnlyShapeshift => f.write_str("OnlyShapeshift"),
            Self::OnlyStealthed => f.write_str("OnlyStealthed"),
            Self::OnlyUnderwater => f.write_str("OnlyUnderwater"),
            Self::OutOfRange => f.write_str("OutOfRange"),
            Self::Pacified => f.write_str("Pacified"),
            Self::Possessed => f.write_str("Possessed"),
            Self::Reagents => f.write_str("Reagents"),
            Self::RequiresArea => f.write_str("RequiresArea"),
            Self::RequiresSpellFocus => f.write_str("RequiresSpellFocus"),
            Self::Rooted => f.write_str("Rooted"),
            Self::Silenced => f.write_str("Silenced"),
            Self::SpellInProgress => f.write_str("SpellInProgress"),
            Self::SpellLearned => f.write_str("SpellLearned"),
            Self::SpellUnavailable => f.write_str("SpellUnavailable"),
            Self::Stunned => f.write_str("Stunned"),
            Self::TargetsDead => f.write_str("TargetsDead"),
            Self::TargetAffectingCombat => f.write_str("TargetAffectingCombat"),
            Self::TargetAurastate => f.write_str("TargetAurastate"),
            Self::TargetDueling => f.write_str("TargetDueling"),
            Self::TargetEnemy => f.write_str("TargetEnemy"),
            Self::TargetEnraged => f.write_str("TargetEnraged"),
            Self::TargetFriendly => f.write_str("TargetFriendly"),
            Self::TargetInCombat => f.write_str("TargetInCombat"),
            Self::TargetIsPlayer => f.write_str("TargetIsPlayer"),
            Self::TargetIsPlayerControlled => f.write_str("TargetIsPlayerControlled"),
            Self::TargetNotDead => f.write_str("TargetNotDead"),
            Self::TargetNotInParty => f.write_str("TargetNotInParty"),
            Self::TargetNotLooted => f.write_str("TargetNotLooted"),
            Self::TargetNotPlayer => f.write_str("TargetNotPlayer"),
            Self::TargetNoPockets => f.write_str("TargetNoPockets"),
            Self::TargetNoWeapons => f.write_str("TargetNoWeapons"),
            Self::TargetNoRangedWeapons => f.write_str("TargetNoRangedWeapons"),
            Self::TargetUnskinnable => f.write_str("TargetUnskinnable"),
            Self::ThirstSatiated => f.write_str("ThirstSatiated"),
            Self::TooClose => f.write_str("TooClose"),
            Self::TooManyOfItem => f.write_str("TooManyOfItem"),
            Self::TotemCategory => f.write_str("TotemCategory"),
            Self::Totems => f.write_str("Totems"),
            Self::TryAgain => f.write_str("TryAgain"),
            Self::UnitNotBehind => f.write_str("UnitNotBehind"),
            Self::UnitNotInfront => f.write_str("UnitNotInfront"),
            Self::WrongPetFood => f.write_str("WrongPetFood"),
            Self::NotWhileFatigued => f.write_str("NotWhileFatigued"),
            Self::TargetNotInInstance => f.write_str("TargetNotInInstance"),
            Self::NotWhileTrading => f.write_str("NotWhileTrading"),
            Self::TargetNotInRaid => f.write_str("TargetNotInRaid"),
            Self::TargetFreeforall => f.write_str("TargetFreeforall"),
            Self::NoEdibleCorpses => f.write_str("NoEdibleCorpses"),
            Self::OnlyBattlegrounds => f.write_str("OnlyBattlegrounds"),
            Self::TargetNotGhost => f.write_str("TargetNotGhost"),
            Self::TransformUnusable => f.write_str("TransformUnusable"),
            Self::WrongWeather => f.write_str("WrongWeather"),
            Self::DamageImmune => f.write_str("DamageImmune"),
            Self::PreventedByMechanic => f.write_str("PreventedByMechanic"),
            Self::PlayTime => f.write_str("PlayTime"),
            Self::Reputation => f.write_str("Reputation"),
            Self::MinSkill => f.write_str("MinSkill"),
            Self::NotInArena => f.write_str("NotInArena"),
            Self::NotOnShapeshift => f.write_str("NotOnShapeshift"),
            Self::NotOnStealthed => f.write_str("NotOnStealthed"),
            Self::NotOnDamageImmune => f.write_str("NotOnDamageImmune"),
            Self::NotOnMounted => f.write_str("NotOnMounted"),
            Self::TooShallow => f.write_str("TooShallow"),
            Self::TargetNotInSanctuary => f.write_str("TargetNotInSanctuary"),
            Self::TargetIsTrivial => f.write_str("TargetIsTrivial"),
            Self::BmOrInvisgod => f.write_str("BmOrInvisgod"),
            Self::ExpertRidingRequirement => f.write_str("ExpertRidingRequirement"),
            Self::ArtisanRidingRequirement => f.write_str("ArtisanRidingRequirement"),
            Self::NotIdle => f.write_str("NotIdle"),
            Self::NotInactive => f.write_str("NotInactive"),
            Self::PartialPlaytime => f.write_str("PartialPlaytime"),
            Self::NoPlaytime => f.write_str("NoPlaytime"),
            Self::NotInBattleground => f.write_str("NotInBattleground"),
            Self::NotInRaidInstance => f.write_str("NotInRaidInstance"),
            Self::OnlyInArena => f.write_str("OnlyInArena"),
            Self::TargetLockedToRaidInstance => f.write_str("TargetLockedToRaidInstance"),
            Self::OnUseEnchant => f.write_str("OnUseEnchant"),
            Self::NotOnGround => f.write_str("NotOnGround"),
            Self::CustomError => f.write_str("CustomError"),
            Self::CantDoThatRightNow => f.write_str("CantDoThatRightNow"),
            Self::TooManySockets => f.write_str("TooManySockets"),
            Self::InvalidGlyph => f.write_str("InvalidGlyph"),
            Self::UniqueGlyph => f.write_str("UniqueGlyph"),
            Self::GlyphSocketLocked => f.write_str("GlyphSocketLocked"),
            Self::NoValidTargets => f.write_str("NoValidTargets"),
            Self::ItemAtMaxCharges => f.write_str("ItemAtMaxCharges"),
            Self::NotInBarbershop => f.write_str("NotInBarbershop"),
            Self::FishingTooLow => f.write_str("FishingTooLow"),
            Self::ItemEnchantTradeWindow => f.write_str("ItemEnchantTradeWindow"),
            Self::SummonPending => f.write_str("SummonPending"),
            Self::MaxSockets => f.write_str("MaxSockets"),
            Self::PetCanRename => f.write_str("PetCanRename"),
            Self::TargetCannotBeResurrected => f.write_str("TargetCannotBeResurrected"),
            Self::Unknown => f.write_str("Unknown"),
        }
    }
}

impl TryFrom<u8> for SpellCastResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Success),
            1 => Ok(Self::AffectingCombat),
            2 => Ok(Self::AlreadyAtFullHealth),
            3 => Ok(Self::AlreadyAtFullMana),
            4 => Ok(Self::AlreadyAtFullPower),
            5 => Ok(Self::AlreadyBeingTamed),
            6 => Ok(Self::AlreadyHaveCharm),
            7 => Ok(Self::AlreadyHaveSummon),
            8 => Ok(Self::AlreadyOpen),
            9 => Ok(Self::AuraBounced),
            10 => Ok(Self::AutotrackInterrupted),
            11 => Ok(Self::BadImplicitTargets),
            12 => Ok(Self::BadTargets),
            13 => Ok(Self::CantBeCharmed),
            14 => Ok(Self::CantBeDisenchanted),
            15 => Ok(Self::CantBeDisenchantedSkill),
            16 => Ok(Self::CantBeMilled),
            17 => Ok(Self::CantBeProspected),
            18 => Ok(Self::CantCastOnTapped),
            19 => Ok(Self::CantDuelWhileInvisible),
            20 => Ok(Self::CantDuelWhileStealthed),
            21 => Ok(Self::CantStealth),
            22 => Ok(Self::CasterAurastate),
            23 => Ok(Self::CasterDead),
            24 => Ok(Self::Charmed),
            25 => Ok(Self::ChestInUse),
            26 => Ok(Self::Confused),
            27 => Ok(Self::DontReport),
            28 => Ok(Self::EquippedItem),
            29 => Ok(Self::EquippedItemClass),
            30 => Ok(Self::EquippedItemClassMainhand),
            31 => Ok(Self::EquippedItemClassOffhand),
            32 => Ok(Self::ErrorX),
            33 => Ok(Self::Fizzle),
            34 => Ok(Self::Fleeing),
            35 => Ok(Self::FoodLowlevel),
            36 => Ok(Self::Highlevel),
            37 => Ok(Self::HungerSatiated),
            38 => Ok(Self::Immune),
            39 => Ok(Self::IncorrectArea),
            40 => Ok(Self::Interrupted),
            41 => Ok(Self::InterruptedCombat),
            42 => Ok(Self::ItemAlreadyEnchanted),
            43 => Ok(Self::ItemGone),
            44 => Ok(Self::ItemNotFound),
            45 => Ok(Self::ItemNotReady),
            46 => Ok(Self::LevelRequirement),
            47 => Ok(Self::LineOfSight),
            48 => Ok(Self::Lowlevel),
            49 => Ok(Self::LowCastlevel),
            50 => Ok(Self::MainhandEmpty),
            51 => Ok(Self::Moving),
            52 => Ok(Self::NeedAmmo),
            53 => Ok(Self::NeedAmmoPouch),
            54 => Ok(Self::NeedExoticAmmo),
            55 => Ok(Self::NeedMoreItems),
            56 => Ok(Self::Nopath),
            57 => Ok(Self::NotBehind),
            58 => Ok(Self::NotFishable),
            59 => Ok(Self::NotFlying),
            60 => Ok(Self::NotHere),
            61 => Ok(Self::NotInfront),
            62 => Ok(Self::NotInControl),
            63 => Ok(Self::NotKnown),
            64 => Ok(Self::NotMounted),
            65 => Ok(Self::NotOnTaxi),
            66 => Ok(Self::NotOnTransport),
            67 => Ok(Self::NotReady),
            68 => Ok(Self::NotShapeshift),
            69 => Ok(Self::NotStanding),
            70 => Ok(Self::NotTradeable),
            71 => Ok(Self::NotTrading),
            72 => Ok(Self::NotUnsheathed),
            73 => Ok(Self::NotWhileGhost),
            74 => Ok(Self::NotWhileLooting),
            75 => Ok(Self::NoAmmo),
            76 => Ok(Self::NoChargesRemain),
            77 => Ok(Self::NoChampion),
            78 => Ok(Self::NoComboPoints),
            79 => Ok(Self::NoDueling),
            80 => Ok(Self::NoEndurance),
            81 => Ok(Self::NoFish),
            82 => Ok(Self::NoItemsWhileShapeshifted),
            83 => Ok(Self::NoMountsAllowed),
            84 => Ok(Self::NoPet),
            85 => Ok(Self::NoPower),
            86 => Ok(Self::NothingToDispel),
            87 => Ok(Self::NothingToSteal),
            88 => Ok(Self::OnlyAbovewater),
            89 => Ok(Self::OnlyDaytime),
            90 => Ok(Self::OnlyIndoors),
            91 => Ok(Self::OnlyMounted),
            92 => Ok(Self::OnlyNighttime),
            93 => Ok(Self::OnlyOutdoors),
            94 => Ok(Self::OnlyShapeshift),
            95 => Ok(Self::OnlyStealthed),
            96 => Ok(Self::OnlyUnderwater),
            97 => Ok(Self::OutOfRange),
            98 => Ok(Self::Pacified),
            99 => Ok(Self::Possessed),
            100 => Ok(Self::Reagents),
            101 => Ok(Self::RequiresArea),
            102 => Ok(Self::RequiresSpellFocus),
            103 => Ok(Self::Rooted),
            104 => Ok(Self::Silenced),
            105 => Ok(Self::SpellInProgress),
            106 => Ok(Self::SpellLearned),
            107 => Ok(Self::SpellUnavailable),
            108 => Ok(Self::Stunned),
            109 => Ok(Self::TargetsDead),
            110 => Ok(Self::TargetAffectingCombat),
            111 => Ok(Self::TargetAurastate),
            112 => Ok(Self::TargetDueling),
            113 => Ok(Self::TargetEnemy),
            114 => Ok(Self::TargetEnraged),
            115 => Ok(Self::TargetFriendly),
            116 => Ok(Self::TargetInCombat),
            117 => Ok(Self::TargetIsPlayer),
            118 => Ok(Self::TargetIsPlayerControlled),
            119 => Ok(Self::TargetNotDead),
            120 => Ok(Self::TargetNotInParty),
            121 => Ok(Self::TargetNotLooted),
            122 => Ok(Self::TargetNotPlayer),
            123 => Ok(Self::TargetNoPockets),
            124 => Ok(Self::TargetNoWeapons),
            125 => Ok(Self::TargetNoRangedWeapons),
            126 => Ok(Self::TargetUnskinnable),
            127 => Ok(Self::ThirstSatiated),
            128 => Ok(Self::TooClose),
            129 => Ok(Self::TooManyOfItem),
            130 => Ok(Self::TotemCategory),
            131 => Ok(Self::Totems),
            132 => Ok(Self::TryAgain),
            133 => Ok(Self::UnitNotBehind),
            134 => Ok(Self::UnitNotInfront),
            135 => Ok(Self::WrongPetFood),
            136 => Ok(Self::NotWhileFatigued),
            137 => Ok(Self::TargetNotInInstance),
            138 => Ok(Self::NotWhileTrading),
            139 => Ok(Self::TargetNotInRaid),
            140 => Ok(Self::TargetFreeforall),
            141 => Ok(Self::NoEdibleCorpses),
            142 => Ok(Self::OnlyBattlegrounds),
            143 => Ok(Self::TargetNotGhost),
            144 => Ok(Self::TransformUnusable),
            145 => Ok(Self::WrongWeather),
            146 => Ok(Self::DamageImmune),
            147 => Ok(Self::PreventedByMechanic),
            148 => Ok(Self::PlayTime),
            149 => Ok(Self::Reputation),
            150 => Ok(Self::MinSkill),
            151 => Ok(Self::NotInArena),
            152 => Ok(Self::NotOnShapeshift),
            153 => Ok(Self::NotOnStealthed),
            154 => Ok(Self::NotOnDamageImmune),
            155 => Ok(Self::NotOnMounted),
            156 => Ok(Self::TooShallow),
            157 => Ok(Self::TargetNotInSanctuary),
            158 => Ok(Self::TargetIsTrivial),
            159 => Ok(Self::BmOrInvisgod),
            160 => Ok(Self::ExpertRidingRequirement),
            161 => Ok(Self::ArtisanRidingRequirement),
            162 => Ok(Self::NotIdle),
            163 => Ok(Self::NotInactive),
            164 => Ok(Self::PartialPlaytime),
            165 => Ok(Self::NoPlaytime),
            166 => Ok(Self::NotInBattleground),
            167 => Ok(Self::NotInRaidInstance),
            168 => Ok(Self::OnlyInArena),
            169 => Ok(Self::TargetLockedToRaidInstance),
            170 => Ok(Self::OnUseEnchant),
            171 => Ok(Self::NotOnGround),
            172 => Ok(Self::CustomError),
            173 => Ok(Self::CantDoThatRightNow),
            174 => Ok(Self::TooManySockets),
            175 => Ok(Self::InvalidGlyph),
            176 => Ok(Self::UniqueGlyph),
            177 => Ok(Self::GlyphSocketLocked),
            178 => Ok(Self::NoValidTargets),
            179 => Ok(Self::ItemAtMaxCharges),
            180 => Ok(Self::NotInBarbershop),
            181 => Ok(Self::FishingTooLow),
            182 => Ok(Self::ItemEnchantTradeWindow),
            183 => Ok(Self::SummonPending),
            184 => Ok(Self::MaxSockets),
            185 => Ok(Self::PetCanRename),
            186 => Ok(Self::TargetCannotBeResurrected),
            187 => Ok(Self::Unknown),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for SpellCastResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for SpellCastResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for SpellCastResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for SpellCastResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for SpellCastResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for SpellCastResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for SpellCastResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for SpellCastResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

