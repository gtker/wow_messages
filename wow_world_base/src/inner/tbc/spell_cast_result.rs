/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:402`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L402):
/// ```text
/// enum SpellCastResult : u8 {
///     AFFECTING_COMBAT = 0x00;
///     ALREADY_AT_FULL_HEALTH = 0x01;
///     ALREADY_AT_FULL_MANA = 0x02;
///     ALREADY_AT_FULL_POWER = 0x03;
///     ALREADY_BEING_TAMED = 0x04;
///     ALREADY_HAVE_CHARM = 0x05;
///     ALREADY_HAVE_SUMMON = 0x06;
///     ALREADY_OPEN = 0x07;
///     AURA_BOUNCED = 0x08;
///     AUTOTRACK_INTERRUPTED = 0x09;
///     BAD_IMPLICIT_TARGETS = 0x0A;
///     BAD_TARGETS = 0x0B;
///     CANT_BE_CHARMED = 0x0C;
///     CANT_BE_DISENCHANTED = 0x0D;
///     CANT_BE_DISENCHANTED_SKILL = 0x0E;
///     CANT_BE_PROSPECTED = 0x0F;
///     CANT_CAST_ON_TAPPED = 0x10;
///     CANT_DUEL_WHILE_INVISIBLE = 0x11;
///     CANT_DUEL_WHILE_STEALTHED = 0x12;
///     CANT_STEALTH = 0x13;
///     CASTER_AURASTATE = 0x14;
///     CASTER_DEAD = 0x15;
///     CHARMED = 0x16;
///     CHEST_IN_USE = 0x17;
///     CONFUSED = 0x18;
///     DONT_REPORT = 0x19;
///     EQUIPPED_ITEM = 0x1A;
///     EQUIPPED_ITEM_CLASS = 0x1B;
///     EQUIPPED_ITEM_CLASS_MAINHAND = 0x1C;
///     EQUIPPED_ITEM_CLASS_OFFHAND = 0x1D;
///     ERROR = 0x1E;
///     FIZZLE = 0x1F;
///     FLEEING = 0x20;
///     FOOD_LOWLEVEL = 0x21;
///     HIGHLEVEL = 0x22;
///     HUNGER_SATIATED = 0x23;
///     IMMUNE = 0x24;
///     INTERRUPTED = 0x25;
///     INTERRUPTED_COMBAT = 0x26;
///     ITEM_ALREADY_ENCHANTED = 0x27;
///     ITEM_GONE = 0x28;
///     ITEM_NOT_FOUND = 0x29;
///     ITEM_NOT_READY = 0x2A;
///     LEVEL_REQUIREMENT = 0x2B;
///     LINE_OF_SIGHT = 0x2C;
///     LOWLEVEL = 0x2D;
///     LOW_CASTLEVEL = 0x2E;
///     MAINHAND_EMPTY = 0x2F;
///     MOVING = 0x30;
///     NEED_AMMO = 0x31;
///     NEED_AMMO_POUCH = 0x32;
///     NEED_EXOTIC_AMMO = 0x33;
///     NOPATH = 0x34;
///     NOT_BEHIND = 0x35;
///     NOT_FISHABLE = 0x36;
///     NOT_FLYING = 0x37;
///     NOT_HERE = 0x38;
///     NOT_INFRONT = 0x39;
///     NOT_IN_CONTROL = 0x3A;
///     NOT_KNOWN = 0x3B;
///     NOT_MOUNTED = 0x3C;
///     NOT_ON_TAXI = 0x3D;
///     NOT_ON_TRANSPORT = 0x3E;
///     NOT_READY = 0x3F;
///     NOT_SHAPESHIFT = 0x40;
///     NOT_STANDING = 0x41;
///     NOT_TRADEABLE = 0x42;
///     NOT_TRADING = 0x43;
///     NOT_UNSHEATHED = 0x44;
///     NOT_WHILE_GHOST = 0x45;
///     NO_AMMO = 0x46;
///     NO_CHARGES_REMAIN = 0x47;
///     NO_CHAMPION = 0x48;
///     NO_COMBO_POINTS = 0x49;
///     NO_DUELING = 0x4A;
///     NO_ENDURANCE = 0x4B;
///     NO_FISH = 0x4C;
///     NO_ITEMS_WHILE_SHAPESHIFTED = 0x4D;
///     NO_MOUNTS_ALLOWED = 0x4E;
///     NO_PET = 0x4F;
///     NO_POWER = 0x50;
///     NOTHING_TO_DISPEL = 0x51;
///     NOTHING_TO_STEAL = 0x52;
///     ONLY_ABOVEWATER = 0x53;
///     ONLY_DAYTIME = 0x54;
///     ONLY_INDOORS = 0x55;
///     ONLY_MOUNTED = 0x56;
///     ONLY_NIGHTTIME = 0x57;
///     ONLY_OUTDOORS = 0x58;
///     ONLY_SHAPESHIFT = 0x59;
///     ONLY_STEALTHED = 0x5A;
///     ONLY_UNDERWATER = 0x5B;
///     OUT_OF_RANGE = 0x5C;
///     PACIFIED = 0x5D;
///     POSSESSED = 0x5E;
///     REAGENTS = 0x5F;
///     REQUIRES_AREA = 0x60;
///     REQUIRES_SPELL_FOCUS = 0x61;
///     ROOTED = 0x62;
///     SILENCED = 0x63;
///     SPELL_IN_PROGRESS = 0x64;
///     SPELL_LEARNED = 0x65;
///     SPELL_UNAVAILABLE = 0x66;
///     STUNNED = 0x67;
///     TARGETS_DEAD = 0x68;
///     TARGET_AFFECTING_COMBAT = 0x69;
///     TARGET_AURASTATE = 0x6A;
///     TARGET_DUELING = 0x6B;
///     TARGET_ENEMY = 0x6C;
///     TARGET_ENRAGED = 0x6D;
///     TARGET_FRIENDLY = 0x6E;
///     TARGET_IN_COMBAT = 0x6F;
///     TARGET_IS_PLAYER = 0x70;
///     TARGET_IS_PLAYER_CONTROLLED = 0x71;
///     TARGET_NOT_DEAD = 0x72;
///     TARGET_NOT_IN_PARTY = 0x73;
///     TARGET_NOT_LOOTED = 0x74;
///     TARGET_NOT_PLAYER = 0x75;
///     TARGET_NO_POCKETS = 0x76;
///     TARGET_NO_WEAPONS = 0x77;
///     TARGET_UNSKINNABLE = 0x78;
///     THIRST_SATIATED = 0x79;
///     TOO_CLOSE = 0x7A;
///     TOO_MANY_OF_ITEM = 0x7B;
///     TOTEM_CATEGORY = 0x7C;
///     TOTEMS = 0x7D;
///     TRAINING_POINTS = 0x7E;
///     TRY_AGAIN = 0x7F;
///     UNIT_NOT_BEHIND = 0x80;
///     UNIT_NOT_INFRONT = 0x81;
///     WRONG_PET_FOOD = 0x82;
///     NOT_WHILE_FATIGUED = 0x83;
///     TARGET_NOT_IN_INSTANCE = 0x84;
///     NOT_WHILE_TRADING = 0x85;
///     TARGET_NOT_IN_RAID = 0x86;
///     DISENCHANT_WHILE_LOOTING = 0x87;
///     PROSPECT_WHILE_LOOTING = 0x88;
///     PROSPECT_NEED_MORE = 0x89;
///     TARGET_FREEFORALL = 0x8A;
///     NO_EDIBLE_CORPSES = 0x8B;
///     ONLY_BATTLEGROUNDS = 0x8C;
///     TARGET_NOT_GHOST = 0x8D;
///     TOO_MANY_SKILLS = 0x8E;
///     TRANSFORM_UNUSABLE = 0x8F;
///     WRONG_WEATHER = 0x90;
///     DAMAGE_IMMUNE = 0x91;
///     PREVENTED_BY_MECHANIC = 0x92;
///     PLAY_TIME = 0x93;
///     REPUTATION = 0x94;
///     MIN_SKILL = 0x95;
///     NOT_IN_ARENA = 0x96;
///     NOT_ON_SHAPESHIFT = 0x97;
///     NOT_ON_STEALTHED = 0x98;
///     NOT_ON_DAMAGE_IMMUNE = 0x99;
///     NOT_ON_MOUNTED = 0x9A;
///     TOO_SHALLOW = 0x9B;
///     TARGET_NOT_IN_SANCTUARY = 0x9C;
///     TARGET_IS_TRIVIAL = 0x9D;
///     BM_OR_INVISGOD = 0x9E;
///     EXPERT_RIDING_REQUIREMENT = 0x9F;
///     ARTISAN_RIDING_REQUIREMENT = 0xA0;
///     NOT_IDLE = 0xA1;
///     NOT_INACTIVE = 0xA2;
///     PARTIAL_PLAYTIME = 0xA3;
///     NO_PLAYTIME = 0xA4;
///     NOT_IN_BATTLEGROUND = 0xA5;
///     ONLY_IN_ARENA = 0xA6;
///     TARGET_LOCKED_TO_RAID_INSTANCE = 0xA7;
///     UNKNOWN = 0xA8;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SpellCastResult {
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
    TargetUnskinnable,
    ThirstSatiated,
    TooClose,
    TooManyOfItem,
    TotemCategory,
    Totems,
    TrainingPoints,
    TryAgain,
    UnitNotBehind,
    UnitNotInfront,
    WrongPetFood,
    NotWhileFatigued,
    TargetNotInInstance,
    NotWhileTrading,
    TargetNotInRaid,
    DisenchantWhileLooting,
    ProspectWhileLooting,
    ProspectNeedMore,
    TargetFreeforall,
    NoEdibleCorpses,
    OnlyBattlegrounds,
    TargetNotGhost,
    TooManySkills,
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
    OnlyInArena,
    TargetLockedToRaidInstance,
    Unknown,
}

impl SpellCastResult {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::AffectingCombat => 0x0,
            Self::AlreadyAtFullHealth => 0x1,
            Self::AlreadyAtFullMana => 0x2,
            Self::AlreadyAtFullPower => 0x3,
            Self::AlreadyBeingTamed => 0x4,
            Self::AlreadyHaveCharm => 0x5,
            Self::AlreadyHaveSummon => 0x6,
            Self::AlreadyOpen => 0x7,
            Self::AuraBounced => 0x8,
            Self::AutotrackInterrupted => 0x9,
            Self::BadImplicitTargets => 0xa,
            Self::BadTargets => 0xb,
            Self::CantBeCharmed => 0xc,
            Self::CantBeDisenchanted => 0xd,
            Self::CantBeDisenchantedSkill => 0xe,
            Self::CantBeProspected => 0xf,
            Self::CantCastOnTapped => 0x10,
            Self::CantDuelWhileInvisible => 0x11,
            Self::CantDuelWhileStealthed => 0x12,
            Self::CantStealth => 0x13,
            Self::CasterAurastate => 0x14,
            Self::CasterDead => 0x15,
            Self::Charmed => 0x16,
            Self::ChestInUse => 0x17,
            Self::Confused => 0x18,
            Self::DontReport => 0x19,
            Self::EquippedItem => 0x1a,
            Self::EquippedItemClass => 0x1b,
            Self::EquippedItemClassMainhand => 0x1c,
            Self::EquippedItemClassOffhand => 0x1d,
            Self::ErrorX => 0x1e,
            Self::Fizzle => 0x1f,
            Self::Fleeing => 0x20,
            Self::FoodLowlevel => 0x21,
            Self::Highlevel => 0x22,
            Self::HungerSatiated => 0x23,
            Self::Immune => 0x24,
            Self::Interrupted => 0x25,
            Self::InterruptedCombat => 0x26,
            Self::ItemAlreadyEnchanted => 0x27,
            Self::ItemGone => 0x28,
            Self::ItemNotFound => 0x29,
            Self::ItemNotReady => 0x2a,
            Self::LevelRequirement => 0x2b,
            Self::LineOfSight => 0x2c,
            Self::Lowlevel => 0x2d,
            Self::LowCastlevel => 0x2e,
            Self::MainhandEmpty => 0x2f,
            Self::Moving => 0x30,
            Self::NeedAmmo => 0x31,
            Self::NeedAmmoPouch => 0x32,
            Self::NeedExoticAmmo => 0x33,
            Self::Nopath => 0x34,
            Self::NotBehind => 0x35,
            Self::NotFishable => 0x36,
            Self::NotFlying => 0x37,
            Self::NotHere => 0x38,
            Self::NotInfront => 0x39,
            Self::NotInControl => 0x3a,
            Self::NotKnown => 0x3b,
            Self::NotMounted => 0x3c,
            Self::NotOnTaxi => 0x3d,
            Self::NotOnTransport => 0x3e,
            Self::NotReady => 0x3f,
            Self::NotShapeshift => 0x40,
            Self::NotStanding => 0x41,
            Self::NotTradeable => 0x42,
            Self::NotTrading => 0x43,
            Self::NotUnsheathed => 0x44,
            Self::NotWhileGhost => 0x45,
            Self::NoAmmo => 0x46,
            Self::NoChargesRemain => 0x47,
            Self::NoChampion => 0x48,
            Self::NoComboPoints => 0x49,
            Self::NoDueling => 0x4a,
            Self::NoEndurance => 0x4b,
            Self::NoFish => 0x4c,
            Self::NoItemsWhileShapeshifted => 0x4d,
            Self::NoMountsAllowed => 0x4e,
            Self::NoPet => 0x4f,
            Self::NoPower => 0x50,
            Self::NothingToDispel => 0x51,
            Self::NothingToSteal => 0x52,
            Self::OnlyAbovewater => 0x53,
            Self::OnlyDaytime => 0x54,
            Self::OnlyIndoors => 0x55,
            Self::OnlyMounted => 0x56,
            Self::OnlyNighttime => 0x57,
            Self::OnlyOutdoors => 0x58,
            Self::OnlyShapeshift => 0x59,
            Self::OnlyStealthed => 0x5a,
            Self::OnlyUnderwater => 0x5b,
            Self::OutOfRange => 0x5c,
            Self::Pacified => 0x5d,
            Self::Possessed => 0x5e,
            Self::Reagents => 0x5f,
            Self::RequiresArea => 0x60,
            Self::RequiresSpellFocus => 0x61,
            Self::Rooted => 0x62,
            Self::Silenced => 0x63,
            Self::SpellInProgress => 0x64,
            Self::SpellLearned => 0x65,
            Self::SpellUnavailable => 0x66,
            Self::Stunned => 0x67,
            Self::TargetsDead => 0x68,
            Self::TargetAffectingCombat => 0x69,
            Self::TargetAurastate => 0x6a,
            Self::TargetDueling => 0x6b,
            Self::TargetEnemy => 0x6c,
            Self::TargetEnraged => 0x6d,
            Self::TargetFriendly => 0x6e,
            Self::TargetInCombat => 0x6f,
            Self::TargetIsPlayer => 0x70,
            Self::TargetIsPlayerControlled => 0x71,
            Self::TargetNotDead => 0x72,
            Self::TargetNotInParty => 0x73,
            Self::TargetNotLooted => 0x74,
            Self::TargetNotPlayer => 0x75,
            Self::TargetNoPockets => 0x76,
            Self::TargetNoWeapons => 0x77,
            Self::TargetUnskinnable => 0x78,
            Self::ThirstSatiated => 0x79,
            Self::TooClose => 0x7a,
            Self::TooManyOfItem => 0x7b,
            Self::TotemCategory => 0x7c,
            Self::Totems => 0x7d,
            Self::TrainingPoints => 0x7e,
            Self::TryAgain => 0x7f,
            Self::UnitNotBehind => 0x80,
            Self::UnitNotInfront => 0x81,
            Self::WrongPetFood => 0x82,
            Self::NotWhileFatigued => 0x83,
            Self::TargetNotInInstance => 0x84,
            Self::NotWhileTrading => 0x85,
            Self::TargetNotInRaid => 0x86,
            Self::DisenchantWhileLooting => 0x87,
            Self::ProspectWhileLooting => 0x88,
            Self::ProspectNeedMore => 0x89,
            Self::TargetFreeforall => 0x8a,
            Self::NoEdibleCorpses => 0x8b,
            Self::OnlyBattlegrounds => 0x8c,
            Self::TargetNotGhost => 0x8d,
            Self::TooManySkills => 0x8e,
            Self::TransformUnusable => 0x8f,
            Self::WrongWeather => 0x90,
            Self::DamageImmune => 0x91,
            Self::PreventedByMechanic => 0x92,
            Self::PlayTime => 0x93,
            Self::Reputation => 0x94,
            Self::MinSkill => 0x95,
            Self::NotInArena => 0x96,
            Self::NotOnShapeshift => 0x97,
            Self::NotOnStealthed => 0x98,
            Self::NotOnDamageImmune => 0x99,
            Self::NotOnMounted => 0x9a,
            Self::TooShallow => 0x9b,
            Self::TargetNotInSanctuary => 0x9c,
            Self::TargetIsTrivial => 0x9d,
            Self::BmOrInvisgod => 0x9e,
            Self::ExpertRidingRequirement => 0x9f,
            Self::ArtisanRidingRequirement => 0xa0,
            Self::NotIdle => 0xa1,
            Self::NotInactive => 0xa2,
            Self::PartialPlaytime => 0xa3,
            Self::NoPlaytime => 0xa4,
            Self::NotInBattleground => 0xa5,
            Self::OnlyInArena => 0xa6,
            Self::TargetLockedToRaidInstance => 0xa7,
            Self::Unknown => 0xa8,
        }
    }

}

impl Default for SpellCastResult {
    fn default() -> Self {
        Self::AffectingCombat
    }
}

impl std::fmt::Display for SpellCastResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
            Self::TargetUnskinnable => f.write_str("TargetUnskinnable"),
            Self::ThirstSatiated => f.write_str("ThirstSatiated"),
            Self::TooClose => f.write_str("TooClose"),
            Self::TooManyOfItem => f.write_str("TooManyOfItem"),
            Self::TotemCategory => f.write_str("TotemCategory"),
            Self::Totems => f.write_str("Totems"),
            Self::TrainingPoints => f.write_str("TrainingPoints"),
            Self::TryAgain => f.write_str("TryAgain"),
            Self::UnitNotBehind => f.write_str("UnitNotBehind"),
            Self::UnitNotInfront => f.write_str("UnitNotInfront"),
            Self::WrongPetFood => f.write_str("WrongPetFood"),
            Self::NotWhileFatigued => f.write_str("NotWhileFatigued"),
            Self::TargetNotInInstance => f.write_str("TargetNotInInstance"),
            Self::NotWhileTrading => f.write_str("NotWhileTrading"),
            Self::TargetNotInRaid => f.write_str("TargetNotInRaid"),
            Self::DisenchantWhileLooting => f.write_str("DisenchantWhileLooting"),
            Self::ProspectWhileLooting => f.write_str("ProspectWhileLooting"),
            Self::ProspectNeedMore => f.write_str("ProspectNeedMore"),
            Self::TargetFreeforall => f.write_str("TargetFreeforall"),
            Self::NoEdibleCorpses => f.write_str("NoEdibleCorpses"),
            Self::OnlyBattlegrounds => f.write_str("OnlyBattlegrounds"),
            Self::TargetNotGhost => f.write_str("TargetNotGhost"),
            Self::TooManySkills => f.write_str("TooManySkills"),
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
            Self::OnlyInArena => f.write_str("OnlyInArena"),
            Self::TargetLockedToRaidInstance => f.write_str("TargetLockedToRaidInstance"),
            Self::Unknown => f.write_str("Unknown"),
        }
    }
}

impl TryFrom<u8> for SpellCastResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AffectingCombat),
            1 => Ok(Self::AlreadyAtFullHealth),
            2 => Ok(Self::AlreadyAtFullMana),
            3 => Ok(Self::AlreadyAtFullPower),
            4 => Ok(Self::AlreadyBeingTamed),
            5 => Ok(Self::AlreadyHaveCharm),
            6 => Ok(Self::AlreadyHaveSummon),
            7 => Ok(Self::AlreadyOpen),
            8 => Ok(Self::AuraBounced),
            9 => Ok(Self::AutotrackInterrupted),
            10 => Ok(Self::BadImplicitTargets),
            11 => Ok(Self::BadTargets),
            12 => Ok(Self::CantBeCharmed),
            13 => Ok(Self::CantBeDisenchanted),
            14 => Ok(Self::CantBeDisenchantedSkill),
            15 => Ok(Self::CantBeProspected),
            16 => Ok(Self::CantCastOnTapped),
            17 => Ok(Self::CantDuelWhileInvisible),
            18 => Ok(Self::CantDuelWhileStealthed),
            19 => Ok(Self::CantStealth),
            20 => Ok(Self::CasterAurastate),
            21 => Ok(Self::CasterDead),
            22 => Ok(Self::Charmed),
            23 => Ok(Self::ChestInUse),
            24 => Ok(Self::Confused),
            25 => Ok(Self::DontReport),
            26 => Ok(Self::EquippedItem),
            27 => Ok(Self::EquippedItemClass),
            28 => Ok(Self::EquippedItemClassMainhand),
            29 => Ok(Self::EquippedItemClassOffhand),
            30 => Ok(Self::ErrorX),
            31 => Ok(Self::Fizzle),
            32 => Ok(Self::Fleeing),
            33 => Ok(Self::FoodLowlevel),
            34 => Ok(Self::Highlevel),
            35 => Ok(Self::HungerSatiated),
            36 => Ok(Self::Immune),
            37 => Ok(Self::Interrupted),
            38 => Ok(Self::InterruptedCombat),
            39 => Ok(Self::ItemAlreadyEnchanted),
            40 => Ok(Self::ItemGone),
            41 => Ok(Self::ItemNotFound),
            42 => Ok(Self::ItemNotReady),
            43 => Ok(Self::LevelRequirement),
            44 => Ok(Self::LineOfSight),
            45 => Ok(Self::Lowlevel),
            46 => Ok(Self::LowCastlevel),
            47 => Ok(Self::MainhandEmpty),
            48 => Ok(Self::Moving),
            49 => Ok(Self::NeedAmmo),
            50 => Ok(Self::NeedAmmoPouch),
            51 => Ok(Self::NeedExoticAmmo),
            52 => Ok(Self::Nopath),
            53 => Ok(Self::NotBehind),
            54 => Ok(Self::NotFishable),
            55 => Ok(Self::NotFlying),
            56 => Ok(Self::NotHere),
            57 => Ok(Self::NotInfront),
            58 => Ok(Self::NotInControl),
            59 => Ok(Self::NotKnown),
            60 => Ok(Self::NotMounted),
            61 => Ok(Self::NotOnTaxi),
            62 => Ok(Self::NotOnTransport),
            63 => Ok(Self::NotReady),
            64 => Ok(Self::NotShapeshift),
            65 => Ok(Self::NotStanding),
            66 => Ok(Self::NotTradeable),
            67 => Ok(Self::NotTrading),
            68 => Ok(Self::NotUnsheathed),
            69 => Ok(Self::NotWhileGhost),
            70 => Ok(Self::NoAmmo),
            71 => Ok(Self::NoChargesRemain),
            72 => Ok(Self::NoChampion),
            73 => Ok(Self::NoComboPoints),
            74 => Ok(Self::NoDueling),
            75 => Ok(Self::NoEndurance),
            76 => Ok(Self::NoFish),
            77 => Ok(Self::NoItemsWhileShapeshifted),
            78 => Ok(Self::NoMountsAllowed),
            79 => Ok(Self::NoPet),
            80 => Ok(Self::NoPower),
            81 => Ok(Self::NothingToDispel),
            82 => Ok(Self::NothingToSteal),
            83 => Ok(Self::OnlyAbovewater),
            84 => Ok(Self::OnlyDaytime),
            85 => Ok(Self::OnlyIndoors),
            86 => Ok(Self::OnlyMounted),
            87 => Ok(Self::OnlyNighttime),
            88 => Ok(Self::OnlyOutdoors),
            89 => Ok(Self::OnlyShapeshift),
            90 => Ok(Self::OnlyStealthed),
            91 => Ok(Self::OnlyUnderwater),
            92 => Ok(Self::OutOfRange),
            93 => Ok(Self::Pacified),
            94 => Ok(Self::Possessed),
            95 => Ok(Self::Reagents),
            96 => Ok(Self::RequiresArea),
            97 => Ok(Self::RequiresSpellFocus),
            98 => Ok(Self::Rooted),
            99 => Ok(Self::Silenced),
            100 => Ok(Self::SpellInProgress),
            101 => Ok(Self::SpellLearned),
            102 => Ok(Self::SpellUnavailable),
            103 => Ok(Self::Stunned),
            104 => Ok(Self::TargetsDead),
            105 => Ok(Self::TargetAffectingCombat),
            106 => Ok(Self::TargetAurastate),
            107 => Ok(Self::TargetDueling),
            108 => Ok(Self::TargetEnemy),
            109 => Ok(Self::TargetEnraged),
            110 => Ok(Self::TargetFriendly),
            111 => Ok(Self::TargetInCombat),
            112 => Ok(Self::TargetIsPlayer),
            113 => Ok(Self::TargetIsPlayerControlled),
            114 => Ok(Self::TargetNotDead),
            115 => Ok(Self::TargetNotInParty),
            116 => Ok(Self::TargetNotLooted),
            117 => Ok(Self::TargetNotPlayer),
            118 => Ok(Self::TargetNoPockets),
            119 => Ok(Self::TargetNoWeapons),
            120 => Ok(Self::TargetUnskinnable),
            121 => Ok(Self::ThirstSatiated),
            122 => Ok(Self::TooClose),
            123 => Ok(Self::TooManyOfItem),
            124 => Ok(Self::TotemCategory),
            125 => Ok(Self::Totems),
            126 => Ok(Self::TrainingPoints),
            127 => Ok(Self::TryAgain),
            128 => Ok(Self::UnitNotBehind),
            129 => Ok(Self::UnitNotInfront),
            130 => Ok(Self::WrongPetFood),
            131 => Ok(Self::NotWhileFatigued),
            132 => Ok(Self::TargetNotInInstance),
            133 => Ok(Self::NotWhileTrading),
            134 => Ok(Self::TargetNotInRaid),
            135 => Ok(Self::DisenchantWhileLooting),
            136 => Ok(Self::ProspectWhileLooting),
            137 => Ok(Self::ProspectNeedMore),
            138 => Ok(Self::TargetFreeforall),
            139 => Ok(Self::NoEdibleCorpses),
            140 => Ok(Self::OnlyBattlegrounds),
            141 => Ok(Self::TargetNotGhost),
            142 => Ok(Self::TooManySkills),
            143 => Ok(Self::TransformUnusable),
            144 => Ok(Self::WrongWeather),
            145 => Ok(Self::DamageImmune),
            146 => Ok(Self::PreventedByMechanic),
            147 => Ok(Self::PlayTime),
            148 => Ok(Self::Reputation),
            149 => Ok(Self::MinSkill),
            150 => Ok(Self::NotInArena),
            151 => Ok(Self::NotOnShapeshift),
            152 => Ok(Self::NotOnStealthed),
            153 => Ok(Self::NotOnDamageImmune),
            154 => Ok(Self::NotOnMounted),
            155 => Ok(Self::TooShallow),
            156 => Ok(Self::TargetNotInSanctuary),
            157 => Ok(Self::TargetIsTrivial),
            158 => Ok(Self::BmOrInvisgod),
            159 => Ok(Self::ExpertRidingRequirement),
            160 => Ok(Self::ArtisanRidingRequirement),
            161 => Ok(Self::NotIdle),
            162 => Ok(Self::NotInactive),
            163 => Ok(Self::PartialPlaytime),
            164 => Ok(Self::NoPlaytime),
            165 => Ok(Self::NotInBattleground),
            166 => Ok(Self::OnlyInArena),
            167 => Ok(Self::TargetLockedToRaidInstance),
            168 => Ok(Self::Unknown),
            v => Err(crate::errors::EnumError::new("SpellCastResult", v as u64),)
        }
    }
}

