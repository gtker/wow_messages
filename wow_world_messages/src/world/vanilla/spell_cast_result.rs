/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:220`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L220):
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
    AffectingCombat,
    AlreadyAtFullHealth,
    AlreadyAtFullMana,
    AlreadyBeingTamed,
    AlreadyHaveCharm,
    AlreadyHaveSummon,
    AlreadyOpen,
    MorePowerfulSpellActive,
    BadImplicitTargets,
    BadTargets,
    CantBeCharmed,
    CantBeDisenchanted,
    CantBeProspected,
    CantCastOnTapped,
    CantDuelWhileInvisible,
    CantDuelWhileStealthed,
    CantTooCloseToEnemy,
    CantDoThatYet,
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
    Immune,
    Interrupted,
    InterruptedCombat,
    ItemAlreadyEnchanted,
    ItemGone,
    EnchantNotExistingItem,
    ItemNotReady,
    LevelRequirement,
    LineOfSight,
    Lowlevel,
    SkillNotHighEnough,
    MainhandEmpty,
    Moving,
    NeedAmmo,
    NeedRequiresSomething,
    NeedExoticAmmo,
    Nopath,
    NotBehind,
    NotFishable,
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
    /// rogues trying 'enchant' other's weapon with poison
    ///
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
    TargetFreeforall,
    NoEdibleCorpses,
    OnlyBattlegrounds,
    TargetNotGhost,
    TooManySkills,
    CantUseNewItem,
    WrongWeather,
    DamageImmune,
    PreventedByMechanic,
    PlayTime,
    Reputation,
    MinSkill,
    Unknown,
}

impl SpellCastResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::AffectingCombat => 0x0,
            Self::AlreadyAtFullHealth => 0x1,
            Self::AlreadyAtFullMana => 0x2,
            Self::AlreadyBeingTamed => 0x3,
            Self::AlreadyHaveCharm => 0x4,
            Self::AlreadyHaveSummon => 0x5,
            Self::AlreadyOpen => 0x6,
            Self::MorePowerfulSpellActive => 0x7,
            Self::BadImplicitTargets => 0x9,
            Self::BadTargets => 0xa,
            Self::CantBeCharmed => 0xb,
            Self::CantBeDisenchanted => 0xc,
            Self::CantBeProspected => 0xd,
            Self::CantCastOnTapped => 0xe,
            Self::CantDuelWhileInvisible => 0xf,
            Self::CantDuelWhileStealthed => 0x10,
            Self::CantTooCloseToEnemy => 0x11,
            Self::CantDoThatYet => 0x12,
            Self::CasterDead => 0x13,
            Self::Charmed => 0x14,
            Self::ChestInUse => 0x15,
            Self::Confused => 0x16,
            Self::DontReport => 0x17,
            Self::EquippedItem => 0x18,
            Self::EquippedItemClass => 0x19,
            Self::EquippedItemClassMainhand => 0x1a,
            Self::EquippedItemClassOffhand => 0x1b,
            Self::ErrorX => 0x1c,
            Self::Fizzle => 0x1d,
            Self::Fleeing => 0x1e,
            Self::FoodLowlevel => 0x1f,
            Self::Highlevel => 0x20,
            Self::Immune => 0x22,
            Self::Interrupted => 0x23,
            Self::InterruptedCombat => 0x24,
            Self::ItemAlreadyEnchanted => 0x25,
            Self::ItemGone => 0x26,
            Self::EnchantNotExistingItem => 0x27,
            Self::ItemNotReady => 0x28,
            Self::LevelRequirement => 0x29,
            Self::LineOfSight => 0x2a,
            Self::Lowlevel => 0x2b,
            Self::SkillNotHighEnough => 0x2c,
            Self::MainhandEmpty => 0x2d,
            Self::Moving => 0x2e,
            Self::NeedAmmo => 0x2f,
            Self::NeedRequiresSomething => 0x30,
            Self::NeedExoticAmmo => 0x31,
            Self::Nopath => 0x32,
            Self::NotBehind => 0x33,
            Self::NotFishable => 0x34,
            Self::NotHere => 0x35,
            Self::NotInfront => 0x36,
            Self::NotInControl => 0x37,
            Self::NotKnown => 0x38,
            Self::NotMounted => 0x39,
            Self::NotOnTaxi => 0x3a,
            Self::NotOnTransport => 0x3b,
            Self::NotReady => 0x3c,
            Self::NotShapeshift => 0x3d,
            Self::NotStanding => 0x3e,
            Self::NotTradeable => 0x3f,
            Self::NotTrading => 0x40,
            Self::NotUnsheathed => 0x41,
            Self::NotWhileGhost => 0x42,
            Self::NoAmmo => 0x43,
            Self::NoChargesRemain => 0x44,
            Self::NoChampion => 0x45,
            Self::NoComboPoints => 0x46,
            Self::NoDueling => 0x47,
            Self::NoEndurance => 0x48,
            Self::NoFish => 0x49,
            Self::NoItemsWhileShapeshifted => 0x4a,
            Self::NoMountsAllowed => 0x4b,
            Self::NoPet => 0x4c,
            Self::NoPower => 0x4d,
            Self::NothingToDispel => 0x4e,
            Self::NothingToSteal => 0x4f,
            Self::OnlyAbovewater => 0x50,
            Self::OnlyDaytime => 0x51,
            Self::OnlyIndoors => 0x52,
            Self::OnlyMounted => 0x53,
            Self::OnlyNighttime => 0x54,
            Self::OnlyOutdoors => 0x55,
            Self::OnlyShapeshift => 0x56,
            Self::OnlyStealthed => 0x57,
            Self::OnlyUnderwater => 0x58,
            Self::OutOfRange => 0x59,
            Self::Pacified => 0x5a,
            Self::Possessed => 0x5b,
            Self::RequiresArea => 0x5d,
            Self::RequiresSpellFocus => 0x5e,
            Self::Rooted => 0x5f,
            Self::Silenced => 0x60,
            Self::SpellInProgress => 0x61,
            Self::SpellLearned => 0x62,
            Self::SpellUnavailable => 0x63,
            Self::Stunned => 0x64,
            Self::TargetsDead => 0x65,
            Self::TargetAffectingCombat => 0x66,
            Self::TargetAurastate => 0x67,
            Self::TargetDueling => 0x68,
            Self::TargetEnemy => 0x69,
            Self::TargetEnraged => 0x6a,
            Self::TargetFriendly => 0x6b,
            Self::TargetInCombat => 0x6c,
            Self::TargetIsPlayer => 0x6d,
            Self::TargetNotDead => 0x6e,
            Self::TargetNotInParty => 0x6f,
            Self::TargetNotLooted => 0x70,
            Self::TargetNotPlayer => 0x71,
            Self::TargetNoPockets => 0x72,
            Self::TargetNoWeapons => 0x73,
            Self::TargetUnskinnable => 0x74,
            Self::ThirstSatiated => 0x75,
            Self::TooClose => 0x76,
            Self::TooManyOfItem => 0x77,
            Self::TrainingPoints => 0x79,
            Self::TryAgain => 0x7a,
            Self::UnitNotBehind => 0x7b,
            Self::UnitNotInfront => 0x7c,
            Self::WrongPetFood => 0x7d,
            Self::NotWhileFatigued => 0x7e,
            Self::TargetNotInInstance => 0x7f,
            Self::NotWhileTrading => 0x80,
            Self::TargetNotInRaid => 0x81,
            Self::DisenchantWhileLooting => 0x82,
            Self::ProspectWhileLooting => 0x83,
            Self::TargetFreeforall => 0x85,
            Self::NoEdibleCorpses => 0x86,
            Self::OnlyBattlegrounds => 0x87,
            Self::TargetNotGhost => 0x88,
            Self::TooManySkills => 0x89,
            Self::CantUseNewItem => 0x8a,
            Self::WrongWeather => 0x8b,
            Self::DamageImmune => 0x8c,
            Self::PreventedByMechanic => 0x8d,
            Self::PlayTime => 0x8e,
            Self::Reputation => 0x8f,
            Self::MinSkill => 0x90,
            Self::Unknown => 0x91,
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
            Self::AlreadyBeingTamed => f.write_str("AlreadyBeingTamed"),
            Self::AlreadyHaveCharm => f.write_str("AlreadyHaveCharm"),
            Self::AlreadyHaveSummon => f.write_str("AlreadyHaveSummon"),
            Self::AlreadyOpen => f.write_str("AlreadyOpen"),
            Self::MorePowerfulSpellActive => f.write_str("MorePowerfulSpellActive"),
            Self::BadImplicitTargets => f.write_str("BadImplicitTargets"),
            Self::BadTargets => f.write_str("BadTargets"),
            Self::CantBeCharmed => f.write_str("CantBeCharmed"),
            Self::CantBeDisenchanted => f.write_str("CantBeDisenchanted"),
            Self::CantBeProspected => f.write_str("CantBeProspected"),
            Self::CantCastOnTapped => f.write_str("CantCastOnTapped"),
            Self::CantDuelWhileInvisible => f.write_str("CantDuelWhileInvisible"),
            Self::CantDuelWhileStealthed => f.write_str("CantDuelWhileStealthed"),
            Self::CantTooCloseToEnemy => f.write_str("CantTooCloseToEnemy"),
            Self::CantDoThatYet => f.write_str("CantDoThatYet"),
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
            Self::Immune => f.write_str("Immune"),
            Self::Interrupted => f.write_str("Interrupted"),
            Self::InterruptedCombat => f.write_str("InterruptedCombat"),
            Self::ItemAlreadyEnchanted => f.write_str("ItemAlreadyEnchanted"),
            Self::ItemGone => f.write_str("ItemGone"),
            Self::EnchantNotExistingItem => f.write_str("EnchantNotExistingItem"),
            Self::ItemNotReady => f.write_str("ItemNotReady"),
            Self::LevelRequirement => f.write_str("LevelRequirement"),
            Self::LineOfSight => f.write_str("LineOfSight"),
            Self::Lowlevel => f.write_str("Lowlevel"),
            Self::SkillNotHighEnough => f.write_str("SkillNotHighEnough"),
            Self::MainhandEmpty => f.write_str("MainhandEmpty"),
            Self::Moving => f.write_str("Moving"),
            Self::NeedAmmo => f.write_str("NeedAmmo"),
            Self::NeedRequiresSomething => f.write_str("NeedRequiresSomething"),
            Self::NeedExoticAmmo => f.write_str("NeedExoticAmmo"),
            Self::Nopath => f.write_str("Nopath"),
            Self::NotBehind => f.write_str("NotBehind"),
            Self::NotFishable => f.write_str("NotFishable"),
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
            Self::TargetFreeforall => f.write_str("TargetFreeforall"),
            Self::NoEdibleCorpses => f.write_str("NoEdibleCorpses"),
            Self::OnlyBattlegrounds => f.write_str("OnlyBattlegrounds"),
            Self::TargetNotGhost => f.write_str("TargetNotGhost"),
            Self::TooManySkills => f.write_str("TooManySkills"),
            Self::CantUseNewItem => f.write_str("CantUseNewItem"),
            Self::WrongWeather => f.write_str("WrongWeather"),
            Self::DamageImmune => f.write_str("DamageImmune"),
            Self::PreventedByMechanic => f.write_str("PreventedByMechanic"),
            Self::PlayTime => f.write_str("PlayTime"),
            Self::Reputation => f.write_str("Reputation"),
            Self::MinSkill => f.write_str("MinSkill"),
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
            3 => Ok(Self::AlreadyBeingTamed),
            4 => Ok(Self::AlreadyHaveCharm),
            5 => Ok(Self::AlreadyHaveSummon),
            6 => Ok(Self::AlreadyOpen),
            7 => Ok(Self::MorePowerfulSpellActive),
            9 => Ok(Self::BadImplicitTargets),
            10 => Ok(Self::BadTargets),
            11 => Ok(Self::CantBeCharmed),
            12 => Ok(Self::CantBeDisenchanted),
            13 => Ok(Self::CantBeProspected),
            14 => Ok(Self::CantCastOnTapped),
            15 => Ok(Self::CantDuelWhileInvisible),
            16 => Ok(Self::CantDuelWhileStealthed),
            17 => Ok(Self::CantTooCloseToEnemy),
            18 => Ok(Self::CantDoThatYet),
            19 => Ok(Self::CasterDead),
            20 => Ok(Self::Charmed),
            21 => Ok(Self::ChestInUse),
            22 => Ok(Self::Confused),
            23 => Ok(Self::DontReport),
            24 => Ok(Self::EquippedItem),
            25 => Ok(Self::EquippedItemClass),
            26 => Ok(Self::EquippedItemClassMainhand),
            27 => Ok(Self::EquippedItemClassOffhand),
            28 => Ok(Self::ErrorX),
            29 => Ok(Self::Fizzle),
            30 => Ok(Self::Fleeing),
            31 => Ok(Self::FoodLowlevel),
            32 => Ok(Self::Highlevel),
            34 => Ok(Self::Immune),
            35 => Ok(Self::Interrupted),
            36 => Ok(Self::InterruptedCombat),
            37 => Ok(Self::ItemAlreadyEnchanted),
            38 => Ok(Self::ItemGone),
            39 => Ok(Self::EnchantNotExistingItem),
            40 => Ok(Self::ItemNotReady),
            41 => Ok(Self::LevelRequirement),
            42 => Ok(Self::LineOfSight),
            43 => Ok(Self::Lowlevel),
            44 => Ok(Self::SkillNotHighEnough),
            45 => Ok(Self::MainhandEmpty),
            46 => Ok(Self::Moving),
            47 => Ok(Self::NeedAmmo),
            48 => Ok(Self::NeedRequiresSomething),
            49 => Ok(Self::NeedExoticAmmo),
            50 => Ok(Self::Nopath),
            51 => Ok(Self::NotBehind),
            52 => Ok(Self::NotFishable),
            53 => Ok(Self::NotHere),
            54 => Ok(Self::NotInfront),
            55 => Ok(Self::NotInControl),
            56 => Ok(Self::NotKnown),
            57 => Ok(Self::NotMounted),
            58 => Ok(Self::NotOnTaxi),
            59 => Ok(Self::NotOnTransport),
            60 => Ok(Self::NotReady),
            61 => Ok(Self::NotShapeshift),
            62 => Ok(Self::NotStanding),
            63 => Ok(Self::NotTradeable),
            64 => Ok(Self::NotTrading),
            65 => Ok(Self::NotUnsheathed),
            66 => Ok(Self::NotWhileGhost),
            67 => Ok(Self::NoAmmo),
            68 => Ok(Self::NoChargesRemain),
            69 => Ok(Self::NoChampion),
            70 => Ok(Self::NoComboPoints),
            71 => Ok(Self::NoDueling),
            72 => Ok(Self::NoEndurance),
            73 => Ok(Self::NoFish),
            74 => Ok(Self::NoItemsWhileShapeshifted),
            75 => Ok(Self::NoMountsAllowed),
            76 => Ok(Self::NoPet),
            77 => Ok(Self::NoPower),
            78 => Ok(Self::NothingToDispel),
            79 => Ok(Self::NothingToSteal),
            80 => Ok(Self::OnlyAbovewater),
            81 => Ok(Self::OnlyDaytime),
            82 => Ok(Self::OnlyIndoors),
            83 => Ok(Self::OnlyMounted),
            84 => Ok(Self::OnlyNighttime),
            85 => Ok(Self::OnlyOutdoors),
            86 => Ok(Self::OnlyShapeshift),
            87 => Ok(Self::OnlyStealthed),
            88 => Ok(Self::OnlyUnderwater),
            89 => Ok(Self::OutOfRange),
            90 => Ok(Self::Pacified),
            91 => Ok(Self::Possessed),
            93 => Ok(Self::RequiresArea),
            94 => Ok(Self::RequiresSpellFocus),
            95 => Ok(Self::Rooted),
            96 => Ok(Self::Silenced),
            97 => Ok(Self::SpellInProgress),
            98 => Ok(Self::SpellLearned),
            99 => Ok(Self::SpellUnavailable),
            100 => Ok(Self::Stunned),
            101 => Ok(Self::TargetsDead),
            102 => Ok(Self::TargetAffectingCombat),
            103 => Ok(Self::TargetAurastate),
            104 => Ok(Self::TargetDueling),
            105 => Ok(Self::TargetEnemy),
            106 => Ok(Self::TargetEnraged),
            107 => Ok(Self::TargetFriendly),
            108 => Ok(Self::TargetInCombat),
            109 => Ok(Self::TargetIsPlayer),
            110 => Ok(Self::TargetNotDead),
            111 => Ok(Self::TargetNotInParty),
            112 => Ok(Self::TargetNotLooted),
            113 => Ok(Self::TargetNotPlayer),
            114 => Ok(Self::TargetNoPockets),
            115 => Ok(Self::TargetNoWeapons),
            116 => Ok(Self::TargetUnskinnable),
            117 => Ok(Self::ThirstSatiated),
            118 => Ok(Self::TooClose),
            119 => Ok(Self::TooManyOfItem),
            121 => Ok(Self::TrainingPoints),
            122 => Ok(Self::TryAgain),
            123 => Ok(Self::UnitNotBehind),
            124 => Ok(Self::UnitNotInfront),
            125 => Ok(Self::WrongPetFood),
            126 => Ok(Self::NotWhileFatigued),
            127 => Ok(Self::TargetNotInInstance),
            128 => Ok(Self::NotWhileTrading),
            129 => Ok(Self::TargetNotInRaid),
            130 => Ok(Self::DisenchantWhileLooting),
            131 => Ok(Self::ProspectWhileLooting),
            133 => Ok(Self::TargetFreeforall),
            134 => Ok(Self::NoEdibleCorpses),
            135 => Ok(Self::OnlyBattlegrounds),
            136 => Ok(Self::TargetNotGhost),
            137 => Ok(Self::TooManySkills),
            138 => Ok(Self::CantUseNewItem),
            139 => Ok(Self::WrongWeather),
            140 => Ok(Self::DamageImmune),
            141 => Ok(Self::PreventedByMechanic),
            142 => Ok(Self::PlayTime),
            143 => Ok(Self::Reputation),
            144 => Ok(Self::MinSkill),
            145 => Ok(Self::Unknown),
            v => Err(crate::errors::EnumError::new("SpellCastResult", v as u64),)
        }
    }
}

