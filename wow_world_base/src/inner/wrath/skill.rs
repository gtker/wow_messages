use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/skill.wowm:275`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/skill.wowm#L275):
/// ```text
/// enum Skill : u16 {
///     NONE = 0;
///     FROST = 6;
///     FIRE = 8;
///     ARMS = 26;
///     COMBAT = 38;
///     SUBTLETY = 39;
///     SWORDS = 43;
///     AXES = 44;
///     BOWS = 45;
///     GUNS = 46;
///     BEAST_MASTERY = 50;
///     SURVIVAL = 51;
///     MACES = 54;
///     TWO_HANDED_SWORDS = 55;
///     HOLY = 56;
///     SHADOW = 78;
///     DEFENSE = 95;
///     LANGUAGE_COMMON = 98;
///     RACIAL_DWARVEN = 101;
///     LANGUAGE_ORCISH = 109;
///     LANGUAGE_DWARVEN = 111;
///     LANGUAGE_DARNASSIAN = 113;
///     LANGUAGE_TAURAHE = 115;
///     DUAL_WIELD = 118;
///     RACIAL_TAUREN = 124;
///     ORC_RACIAL = 125;
///     RACIAL_NIGHT_ELF = 126;
///     FIRST_AID = 129;
///     FERAL_COMBAT = 134;
///     STAVES = 136;
///     LANGUAGE_THALASSIAN = 137;
///     LANGUAGE_DRACONIC = 138;
///     LANGUAGE_DEMON_TONGUE = 139;
///     LANGUAGE_TITAN = 140;
///     LANGUAGE_OLD_TONGUE = 141;
///     SURVIVAL2 = 142;
///     RIDING_HORSE = 148;
///     RIDING_WOLF = 149;
///     RIDING_TIGER = 150;
///     RIDING_RAM = 152;
///     SWIMING = 155;
///     TWO_HANDED_MACES = 160;
///     UNARMED = 162;
///     MARKSMANSHIP = 163;
///     BLACKSMITHING = 164;
///     LEATHERWORKING = 165;
///     ALCHEMY = 171;
///     TWO_HANDED_AXES = 172;
///     DAGGERS = 173;
///     THROWN = 176;
///     HERBALISM = 182;
///     GENERIC_DND = 183;
///     RETRIBUTION = 184;
///     COOKING = 185;
///     MINING = 186;
///     PET_IMP = 188;
///     PET_FELHUNTER = 189;
///     TAILORING = 197;
///     ENGINEERING = 202;
///     PET_SPIDER = 203;
///     PET_VOIDWALKER = 204;
///     PET_SUCCUBUS = 205;
///     PET_INFERNAL = 206;
///     PET_DOOMGUARD = 207;
///     PET_WOLF = 208;
///     PET_CAT = 209;
///     PET_BEAR = 210;
///     PET_BOAR = 211;
///     PET_CROCILISK = 212;
///     PET_CARRION_BIRD = 213;
///     PET_CRAB = 214;
///     PET_GORILLA = 215;
///     PET_RAPTOR = 217;
///     PET_TALLSTRIDER = 218;
///     RACIAL_UNDED = 220;
///     CROSSBOWS = 226;
///     WANDS = 228;
///     POLEARMS = 229;
///     PET_SCORPID = 236;
///     ARCANE = 237;
///     PET_TURTLE = 251;
///     ASSASSINATION = 253;
///     FURY = 256;
///     PROTECTION = 257;
///     PROTECTION2 = 267;
///     PET_TALENTS = 270;
///     PLATE_MAIL = 293;
///     LANGUAGE_GNOMISH = 313;
///     LANGUAGE_TROLL = 315;
///     ENCHANTING = 333;
///     DEMONOLOGY = 354;
///     AFFLICTION = 355;
///     FISHING = 356;
///     ENHANCEMENT = 373;
///     RESTORATION = 374;
///     ELEMENTAL_COMBAT = 375;
///     SKINNING = 393;
///     MAIL = 413;
///     LEATHER = 414;
///     CLOTH = 415;
///     SHIELD = 433;
///     FIST_WEAPONS = 473;
///     RIDING_RAPTOR = 533;
///     RIDING_MECHANOSTRIDER = 553;
///     RIDING_UNDEAD_HORSE = 554;
///     RESTORATION2 = 573;
///     BALANCE = 574;
///     DESTRUCTION = 593;
///     HOLY2 = 594;
///     DISCIPLINE = 613;
///     LOCKPICKING = 633;
///     PET_BAT = 653;
///     PET_HYENA = 654;
///     PET_BIRD_OF_PREY = 655;
///     PET_WIND_SERPENT = 656;
///     LANGUAGE_GUTTERSPEAK = 673;
///     RIDING_KODO = 713;
///     RACIAL_TROLL = 733;
///     RACIAL_GNOME = 753;
///     RACIAL_HUMAN = 754;
///     JEWELCRAFTING = 755;
///     RACIAL_BLOODELF = 756;
///     PET_EVENT_RC = 758;
///     LANGUAGE_DRAENEI = 759;
///     RACIAL_DRAENEI = 760;
///     PET_FELGUARD = 761;
///     RIDING = 762;
///     PET_DRAGONHAWK = 763;
///     PET_NETHER_RAY = 764;
///     PET_SPOREBAT = 765;
///     PET_WARP_STALKER = 766;
///     PET_RAVAGER = 767;
///     PET_SERPENT = 768;
///     INTERNAL = 769;
///     DK_BLOOD = 770;
///     DK_FROST = 771;
///     DK_UNHOLY = 772;
///     INSCRIPTION = 773;
///     PET_MOTH = 775;
///     RUNEFORGING = 776;
///     MOUNTS = 777;
///     COMPANIONS = 778;
///     PET_EXOTIC_CHIMAERA = 780;
///     PET_EXOTIC_DEVILSAUR = 781;
///     PET_GHOUL = 782;
///     PET_EXOTIC_SILITHID = 783;
///     PET_EXOTIC_WORM = 784;
///     PET_WASP = 785;
///     PET_EXOTIC_RHINO = 786;
///     PET_EXOTIC_CORE_HOUND = 787;
///     PET_EXOTIC_SPIRIT_BEAST = 788;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Skill {
    None,
    Frost,
    Fire,
    Arms,
    Combat,
    Subtlety,
    Swords,
    Axes,
    Bows,
    Guns,
    BeastMastery,
    Survival,
    Maces,
    TwoHandedSwords,
    Holy,
    Shadow,
    Defense,
    LanguageCommon,
    RacialDwarven,
    LanguageOrcish,
    LanguageDwarven,
    LanguageDarnassian,
    LanguageTaurahe,
    DualWield,
    RacialTauren,
    OrcRacial,
    RacialNightElf,
    FirstAid,
    FeralCombat,
    Staves,
    LanguageThalassian,
    LanguageDraconic,
    LanguageDemonTongue,
    LanguageTitan,
    LanguageOldTongue,
    Survival2,
    RidingHorse,
    RidingWolf,
    RidingTiger,
    RidingRam,
    Swiming,
    TwoHandedMaces,
    Unarmed,
    Marksmanship,
    Blacksmithing,
    Leatherworking,
    Alchemy,
    TwoHandedAxes,
    Daggers,
    Thrown,
    Herbalism,
    GenericDnd,
    Retribution,
    Cooking,
    Mining,
    PetImp,
    PetFelhunter,
    Tailoring,
    Engineering,
    PetSpider,
    PetVoidwalker,
    PetSuccubus,
    PetInfernal,
    PetDoomguard,
    PetWolf,
    PetCat,
    PetBear,
    PetBoar,
    PetCrocilisk,
    PetCarrionBird,
    PetCrab,
    PetGorilla,
    PetRaptor,
    PetTallstrider,
    RacialUnded,
    Crossbows,
    Wands,
    Polearms,
    PetScorpid,
    Arcane,
    PetTurtle,
    Assassination,
    Fury,
    Protection,
    Protection2,
    PetTalents,
    PlateMail,
    LanguageGnomish,
    LanguageTroll,
    Enchanting,
    Demonology,
    Affliction,
    Fishing,
    Enhancement,
    Restoration,
    ElementalCombat,
    Skinning,
    Mail,
    Leather,
    Cloth,
    Shield,
    FistWeapons,
    RidingRaptor,
    RidingMechanostrider,
    RidingUndeadHorse,
    Restoration2,
    Balance,
    Destruction,
    Holy2,
    Discipline,
    Lockpicking,
    PetBat,
    PetHyena,
    PetBirdOfPrey,
    PetWindSerpent,
    LanguageGutterspeak,
    RidingKodo,
    RacialTroll,
    RacialGnome,
    RacialHuman,
    Jewelcrafting,
    RacialBloodelf,
    PetEventRc,
    LanguageDraenei,
    RacialDraenei,
    PetFelguard,
    Riding,
    PetDragonhawk,
    PetNetherRay,
    PetSporebat,
    PetWarpStalker,
    PetRavager,
    PetSerpent,
    Internal,
    DkBlood,
    DkFrost,
    DkUnholy,
    Inscription,
    PetMoth,
    Runeforging,
    Mounts,
    Companions,
    PetExoticChimaera,
    PetExoticDevilsaur,
    PetGhoul,
    PetExoticSilithid,
    PetExoticWorm,
    PetWasp,
    PetExoticRhino,
    PetExoticCoreHound,
    PetExoticSpiritBeast,
}

impl Skill {
    pub const fn as_int(&self) -> u16 {
        match self {
            Self::None => 0x0,
            Self::Frost => 0x6,
            Self::Fire => 0x8,
            Self::Arms => 0x1a,
            Self::Combat => 0x26,
            Self::Subtlety => 0x27,
            Self::Swords => 0x2b,
            Self::Axes => 0x2c,
            Self::Bows => 0x2d,
            Self::Guns => 0x2e,
            Self::BeastMastery => 0x32,
            Self::Survival => 0x33,
            Self::Maces => 0x36,
            Self::TwoHandedSwords => 0x37,
            Self::Holy => 0x38,
            Self::Shadow => 0x4e,
            Self::Defense => 0x5f,
            Self::LanguageCommon => 0x62,
            Self::RacialDwarven => 0x65,
            Self::LanguageOrcish => 0x6d,
            Self::LanguageDwarven => 0x6f,
            Self::LanguageDarnassian => 0x71,
            Self::LanguageTaurahe => 0x73,
            Self::DualWield => 0x76,
            Self::RacialTauren => 0x7c,
            Self::OrcRacial => 0x7d,
            Self::RacialNightElf => 0x7e,
            Self::FirstAid => 0x81,
            Self::FeralCombat => 0x86,
            Self::Staves => 0x88,
            Self::LanguageThalassian => 0x89,
            Self::LanguageDraconic => 0x8a,
            Self::LanguageDemonTongue => 0x8b,
            Self::LanguageTitan => 0x8c,
            Self::LanguageOldTongue => 0x8d,
            Self::Survival2 => 0x8e,
            Self::RidingHorse => 0x94,
            Self::RidingWolf => 0x95,
            Self::RidingTiger => 0x96,
            Self::RidingRam => 0x98,
            Self::Swiming => 0x9b,
            Self::TwoHandedMaces => 0xa0,
            Self::Unarmed => 0xa2,
            Self::Marksmanship => 0xa3,
            Self::Blacksmithing => 0xa4,
            Self::Leatherworking => 0xa5,
            Self::Alchemy => 0xab,
            Self::TwoHandedAxes => 0xac,
            Self::Daggers => 0xad,
            Self::Thrown => 0xb0,
            Self::Herbalism => 0xb6,
            Self::GenericDnd => 0xb7,
            Self::Retribution => 0xb8,
            Self::Cooking => 0xb9,
            Self::Mining => 0xba,
            Self::PetImp => 0xbc,
            Self::PetFelhunter => 0xbd,
            Self::Tailoring => 0xc5,
            Self::Engineering => 0xca,
            Self::PetSpider => 0xcb,
            Self::PetVoidwalker => 0xcc,
            Self::PetSuccubus => 0xcd,
            Self::PetInfernal => 0xce,
            Self::PetDoomguard => 0xcf,
            Self::PetWolf => 0xd0,
            Self::PetCat => 0xd1,
            Self::PetBear => 0xd2,
            Self::PetBoar => 0xd3,
            Self::PetCrocilisk => 0xd4,
            Self::PetCarrionBird => 0xd5,
            Self::PetCrab => 0xd6,
            Self::PetGorilla => 0xd7,
            Self::PetRaptor => 0xd9,
            Self::PetTallstrider => 0xda,
            Self::RacialUnded => 0xdc,
            Self::Crossbows => 0xe2,
            Self::Wands => 0xe4,
            Self::Polearms => 0xe5,
            Self::PetScorpid => 0xec,
            Self::Arcane => 0xed,
            Self::PetTurtle => 0xfb,
            Self::Assassination => 0xfd,
            Self::Fury => 0x100,
            Self::Protection => 0x101,
            Self::Protection2 => 0x10b,
            Self::PetTalents => 0x10e,
            Self::PlateMail => 0x125,
            Self::LanguageGnomish => 0x139,
            Self::LanguageTroll => 0x13b,
            Self::Enchanting => 0x14d,
            Self::Demonology => 0x162,
            Self::Affliction => 0x163,
            Self::Fishing => 0x164,
            Self::Enhancement => 0x175,
            Self::Restoration => 0x176,
            Self::ElementalCombat => 0x177,
            Self::Skinning => 0x189,
            Self::Mail => 0x19d,
            Self::Leather => 0x19e,
            Self::Cloth => 0x19f,
            Self::Shield => 0x1b1,
            Self::FistWeapons => 0x1d9,
            Self::RidingRaptor => 0x215,
            Self::RidingMechanostrider => 0x229,
            Self::RidingUndeadHorse => 0x22a,
            Self::Restoration2 => 0x23d,
            Self::Balance => 0x23e,
            Self::Destruction => 0x251,
            Self::Holy2 => 0x252,
            Self::Discipline => 0x265,
            Self::Lockpicking => 0x279,
            Self::PetBat => 0x28d,
            Self::PetHyena => 0x28e,
            Self::PetBirdOfPrey => 0x28f,
            Self::PetWindSerpent => 0x290,
            Self::LanguageGutterspeak => 0x2a1,
            Self::RidingKodo => 0x2c9,
            Self::RacialTroll => 0x2dd,
            Self::RacialGnome => 0x2f1,
            Self::RacialHuman => 0x2f2,
            Self::Jewelcrafting => 0x2f3,
            Self::RacialBloodelf => 0x2f4,
            Self::PetEventRc => 0x2f6,
            Self::LanguageDraenei => 0x2f7,
            Self::RacialDraenei => 0x2f8,
            Self::PetFelguard => 0x2f9,
            Self::Riding => 0x2fa,
            Self::PetDragonhawk => 0x2fb,
            Self::PetNetherRay => 0x2fc,
            Self::PetSporebat => 0x2fd,
            Self::PetWarpStalker => 0x2fe,
            Self::PetRavager => 0x2ff,
            Self::PetSerpent => 0x300,
            Self::Internal => 0x301,
            Self::DkBlood => 0x302,
            Self::DkFrost => 0x303,
            Self::DkUnholy => 0x304,
            Self::Inscription => 0x305,
            Self::PetMoth => 0x307,
            Self::Runeforging => 0x308,
            Self::Mounts => 0x309,
            Self::Companions => 0x30a,
            Self::PetExoticChimaera => 0x30c,
            Self::PetExoticDevilsaur => 0x30d,
            Self::PetGhoul => 0x30e,
            Self::PetExoticSilithid => 0x30f,
            Self::PetExoticWorm => 0x310,
            Self::PetWasp => 0x311,
            Self::PetExoticRhino => 0x312,
            Self::PetExoticCoreHound => 0x313,
            Self::PetExoticSpiritBeast => 0x314,
        }
    }

}

impl Default for Skill {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Frost => f.write_str("Frost"),
            Self::Fire => f.write_str("Fire"),
            Self::Arms => f.write_str("Arms"),
            Self::Combat => f.write_str("Combat"),
            Self::Subtlety => f.write_str("Subtlety"),
            Self::Swords => f.write_str("Swords"),
            Self::Axes => f.write_str("Axes"),
            Self::Bows => f.write_str("Bows"),
            Self::Guns => f.write_str("Guns"),
            Self::BeastMastery => f.write_str("BeastMastery"),
            Self::Survival => f.write_str("Survival"),
            Self::Maces => f.write_str("Maces"),
            Self::TwoHandedSwords => f.write_str("TwoHandedSwords"),
            Self::Holy => f.write_str("Holy"),
            Self::Shadow => f.write_str("Shadow"),
            Self::Defense => f.write_str("Defense"),
            Self::LanguageCommon => f.write_str("LanguageCommon"),
            Self::RacialDwarven => f.write_str("RacialDwarven"),
            Self::LanguageOrcish => f.write_str("LanguageOrcish"),
            Self::LanguageDwarven => f.write_str("LanguageDwarven"),
            Self::LanguageDarnassian => f.write_str("LanguageDarnassian"),
            Self::LanguageTaurahe => f.write_str("LanguageTaurahe"),
            Self::DualWield => f.write_str("DualWield"),
            Self::RacialTauren => f.write_str("RacialTauren"),
            Self::OrcRacial => f.write_str("OrcRacial"),
            Self::RacialNightElf => f.write_str("RacialNightElf"),
            Self::FirstAid => f.write_str("FirstAid"),
            Self::FeralCombat => f.write_str("FeralCombat"),
            Self::Staves => f.write_str("Staves"),
            Self::LanguageThalassian => f.write_str("LanguageThalassian"),
            Self::LanguageDraconic => f.write_str("LanguageDraconic"),
            Self::LanguageDemonTongue => f.write_str("LanguageDemonTongue"),
            Self::LanguageTitan => f.write_str("LanguageTitan"),
            Self::LanguageOldTongue => f.write_str("LanguageOldTongue"),
            Self::Survival2 => f.write_str("Survival2"),
            Self::RidingHorse => f.write_str("RidingHorse"),
            Self::RidingWolf => f.write_str("RidingWolf"),
            Self::RidingTiger => f.write_str("RidingTiger"),
            Self::RidingRam => f.write_str("RidingRam"),
            Self::Swiming => f.write_str("Swiming"),
            Self::TwoHandedMaces => f.write_str("TwoHandedMaces"),
            Self::Unarmed => f.write_str("Unarmed"),
            Self::Marksmanship => f.write_str("Marksmanship"),
            Self::Blacksmithing => f.write_str("Blacksmithing"),
            Self::Leatherworking => f.write_str("Leatherworking"),
            Self::Alchemy => f.write_str("Alchemy"),
            Self::TwoHandedAxes => f.write_str("TwoHandedAxes"),
            Self::Daggers => f.write_str("Daggers"),
            Self::Thrown => f.write_str("Thrown"),
            Self::Herbalism => f.write_str("Herbalism"),
            Self::GenericDnd => f.write_str("GenericDnd"),
            Self::Retribution => f.write_str("Retribution"),
            Self::Cooking => f.write_str("Cooking"),
            Self::Mining => f.write_str("Mining"),
            Self::PetImp => f.write_str("PetImp"),
            Self::PetFelhunter => f.write_str("PetFelhunter"),
            Self::Tailoring => f.write_str("Tailoring"),
            Self::Engineering => f.write_str("Engineering"),
            Self::PetSpider => f.write_str("PetSpider"),
            Self::PetVoidwalker => f.write_str("PetVoidwalker"),
            Self::PetSuccubus => f.write_str("PetSuccubus"),
            Self::PetInfernal => f.write_str("PetInfernal"),
            Self::PetDoomguard => f.write_str("PetDoomguard"),
            Self::PetWolf => f.write_str("PetWolf"),
            Self::PetCat => f.write_str("PetCat"),
            Self::PetBear => f.write_str("PetBear"),
            Self::PetBoar => f.write_str("PetBoar"),
            Self::PetCrocilisk => f.write_str("PetCrocilisk"),
            Self::PetCarrionBird => f.write_str("PetCarrionBird"),
            Self::PetCrab => f.write_str("PetCrab"),
            Self::PetGorilla => f.write_str("PetGorilla"),
            Self::PetRaptor => f.write_str("PetRaptor"),
            Self::PetTallstrider => f.write_str("PetTallstrider"),
            Self::RacialUnded => f.write_str("RacialUnded"),
            Self::Crossbows => f.write_str("Crossbows"),
            Self::Wands => f.write_str("Wands"),
            Self::Polearms => f.write_str("Polearms"),
            Self::PetScorpid => f.write_str("PetScorpid"),
            Self::Arcane => f.write_str("Arcane"),
            Self::PetTurtle => f.write_str("PetTurtle"),
            Self::Assassination => f.write_str("Assassination"),
            Self::Fury => f.write_str("Fury"),
            Self::Protection => f.write_str("Protection"),
            Self::Protection2 => f.write_str("Protection2"),
            Self::PetTalents => f.write_str("PetTalents"),
            Self::PlateMail => f.write_str("PlateMail"),
            Self::LanguageGnomish => f.write_str("LanguageGnomish"),
            Self::LanguageTroll => f.write_str("LanguageTroll"),
            Self::Enchanting => f.write_str("Enchanting"),
            Self::Demonology => f.write_str("Demonology"),
            Self::Affliction => f.write_str("Affliction"),
            Self::Fishing => f.write_str("Fishing"),
            Self::Enhancement => f.write_str("Enhancement"),
            Self::Restoration => f.write_str("Restoration"),
            Self::ElementalCombat => f.write_str("ElementalCombat"),
            Self::Skinning => f.write_str("Skinning"),
            Self::Mail => f.write_str("Mail"),
            Self::Leather => f.write_str("Leather"),
            Self::Cloth => f.write_str("Cloth"),
            Self::Shield => f.write_str("Shield"),
            Self::FistWeapons => f.write_str("FistWeapons"),
            Self::RidingRaptor => f.write_str("RidingRaptor"),
            Self::RidingMechanostrider => f.write_str("RidingMechanostrider"),
            Self::RidingUndeadHorse => f.write_str("RidingUndeadHorse"),
            Self::Restoration2 => f.write_str("Restoration2"),
            Self::Balance => f.write_str("Balance"),
            Self::Destruction => f.write_str("Destruction"),
            Self::Holy2 => f.write_str("Holy2"),
            Self::Discipline => f.write_str("Discipline"),
            Self::Lockpicking => f.write_str("Lockpicking"),
            Self::PetBat => f.write_str("PetBat"),
            Self::PetHyena => f.write_str("PetHyena"),
            Self::PetBirdOfPrey => f.write_str("PetBirdOfPrey"),
            Self::PetWindSerpent => f.write_str("PetWindSerpent"),
            Self::LanguageGutterspeak => f.write_str("LanguageGutterspeak"),
            Self::RidingKodo => f.write_str("RidingKodo"),
            Self::RacialTroll => f.write_str("RacialTroll"),
            Self::RacialGnome => f.write_str("RacialGnome"),
            Self::RacialHuman => f.write_str("RacialHuman"),
            Self::Jewelcrafting => f.write_str("Jewelcrafting"),
            Self::RacialBloodelf => f.write_str("RacialBloodelf"),
            Self::PetEventRc => f.write_str("PetEventRc"),
            Self::LanguageDraenei => f.write_str("LanguageDraenei"),
            Self::RacialDraenei => f.write_str("RacialDraenei"),
            Self::PetFelguard => f.write_str("PetFelguard"),
            Self::Riding => f.write_str("Riding"),
            Self::PetDragonhawk => f.write_str("PetDragonhawk"),
            Self::PetNetherRay => f.write_str("PetNetherRay"),
            Self::PetSporebat => f.write_str("PetSporebat"),
            Self::PetWarpStalker => f.write_str("PetWarpStalker"),
            Self::PetRavager => f.write_str("PetRavager"),
            Self::PetSerpent => f.write_str("PetSerpent"),
            Self::Internal => f.write_str("Internal"),
            Self::DkBlood => f.write_str("DkBlood"),
            Self::DkFrost => f.write_str("DkFrost"),
            Self::DkUnholy => f.write_str("DkUnholy"),
            Self::Inscription => f.write_str("Inscription"),
            Self::PetMoth => f.write_str("PetMoth"),
            Self::Runeforging => f.write_str("Runeforging"),
            Self::Mounts => f.write_str("Mounts"),
            Self::Companions => f.write_str("Companions"),
            Self::PetExoticChimaera => f.write_str("PetExoticChimaera"),
            Self::PetExoticDevilsaur => f.write_str("PetExoticDevilsaur"),
            Self::PetGhoul => f.write_str("PetGhoul"),
            Self::PetExoticSilithid => f.write_str("PetExoticSilithid"),
            Self::PetExoticWorm => f.write_str("PetExoticWorm"),
            Self::PetWasp => f.write_str("PetWasp"),
            Self::PetExoticRhino => f.write_str("PetExoticRhino"),
            Self::PetExoticCoreHound => f.write_str("PetExoticCoreHound"),
            Self::PetExoticSpiritBeast => f.write_str("PetExoticSpiritBeast"),
        }
    }
}

impl TryFrom<u16> for Skill {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            6 => Ok(Self::Frost),
            8 => Ok(Self::Fire),
            26 => Ok(Self::Arms),
            38 => Ok(Self::Combat),
            39 => Ok(Self::Subtlety),
            43 => Ok(Self::Swords),
            44 => Ok(Self::Axes),
            45 => Ok(Self::Bows),
            46 => Ok(Self::Guns),
            50 => Ok(Self::BeastMastery),
            51 => Ok(Self::Survival),
            54 => Ok(Self::Maces),
            55 => Ok(Self::TwoHandedSwords),
            56 => Ok(Self::Holy),
            78 => Ok(Self::Shadow),
            95 => Ok(Self::Defense),
            98 => Ok(Self::LanguageCommon),
            101 => Ok(Self::RacialDwarven),
            109 => Ok(Self::LanguageOrcish),
            111 => Ok(Self::LanguageDwarven),
            113 => Ok(Self::LanguageDarnassian),
            115 => Ok(Self::LanguageTaurahe),
            118 => Ok(Self::DualWield),
            124 => Ok(Self::RacialTauren),
            125 => Ok(Self::OrcRacial),
            126 => Ok(Self::RacialNightElf),
            129 => Ok(Self::FirstAid),
            134 => Ok(Self::FeralCombat),
            136 => Ok(Self::Staves),
            137 => Ok(Self::LanguageThalassian),
            138 => Ok(Self::LanguageDraconic),
            139 => Ok(Self::LanguageDemonTongue),
            140 => Ok(Self::LanguageTitan),
            141 => Ok(Self::LanguageOldTongue),
            142 => Ok(Self::Survival2),
            148 => Ok(Self::RidingHorse),
            149 => Ok(Self::RidingWolf),
            150 => Ok(Self::RidingTiger),
            152 => Ok(Self::RidingRam),
            155 => Ok(Self::Swiming),
            160 => Ok(Self::TwoHandedMaces),
            162 => Ok(Self::Unarmed),
            163 => Ok(Self::Marksmanship),
            164 => Ok(Self::Blacksmithing),
            165 => Ok(Self::Leatherworking),
            171 => Ok(Self::Alchemy),
            172 => Ok(Self::TwoHandedAxes),
            173 => Ok(Self::Daggers),
            176 => Ok(Self::Thrown),
            182 => Ok(Self::Herbalism),
            183 => Ok(Self::GenericDnd),
            184 => Ok(Self::Retribution),
            185 => Ok(Self::Cooking),
            186 => Ok(Self::Mining),
            188 => Ok(Self::PetImp),
            189 => Ok(Self::PetFelhunter),
            197 => Ok(Self::Tailoring),
            202 => Ok(Self::Engineering),
            203 => Ok(Self::PetSpider),
            204 => Ok(Self::PetVoidwalker),
            205 => Ok(Self::PetSuccubus),
            206 => Ok(Self::PetInfernal),
            207 => Ok(Self::PetDoomguard),
            208 => Ok(Self::PetWolf),
            209 => Ok(Self::PetCat),
            210 => Ok(Self::PetBear),
            211 => Ok(Self::PetBoar),
            212 => Ok(Self::PetCrocilisk),
            213 => Ok(Self::PetCarrionBird),
            214 => Ok(Self::PetCrab),
            215 => Ok(Self::PetGorilla),
            217 => Ok(Self::PetRaptor),
            218 => Ok(Self::PetTallstrider),
            220 => Ok(Self::RacialUnded),
            226 => Ok(Self::Crossbows),
            228 => Ok(Self::Wands),
            229 => Ok(Self::Polearms),
            236 => Ok(Self::PetScorpid),
            237 => Ok(Self::Arcane),
            251 => Ok(Self::PetTurtle),
            253 => Ok(Self::Assassination),
            256 => Ok(Self::Fury),
            257 => Ok(Self::Protection),
            267 => Ok(Self::Protection2),
            270 => Ok(Self::PetTalents),
            293 => Ok(Self::PlateMail),
            313 => Ok(Self::LanguageGnomish),
            315 => Ok(Self::LanguageTroll),
            333 => Ok(Self::Enchanting),
            354 => Ok(Self::Demonology),
            355 => Ok(Self::Affliction),
            356 => Ok(Self::Fishing),
            373 => Ok(Self::Enhancement),
            374 => Ok(Self::Restoration),
            375 => Ok(Self::ElementalCombat),
            393 => Ok(Self::Skinning),
            413 => Ok(Self::Mail),
            414 => Ok(Self::Leather),
            415 => Ok(Self::Cloth),
            433 => Ok(Self::Shield),
            473 => Ok(Self::FistWeapons),
            533 => Ok(Self::RidingRaptor),
            553 => Ok(Self::RidingMechanostrider),
            554 => Ok(Self::RidingUndeadHorse),
            573 => Ok(Self::Restoration2),
            574 => Ok(Self::Balance),
            593 => Ok(Self::Destruction),
            594 => Ok(Self::Holy2),
            613 => Ok(Self::Discipline),
            633 => Ok(Self::Lockpicking),
            653 => Ok(Self::PetBat),
            654 => Ok(Self::PetHyena),
            655 => Ok(Self::PetBirdOfPrey),
            656 => Ok(Self::PetWindSerpent),
            673 => Ok(Self::LanguageGutterspeak),
            713 => Ok(Self::RidingKodo),
            733 => Ok(Self::RacialTroll),
            753 => Ok(Self::RacialGnome),
            754 => Ok(Self::RacialHuman),
            755 => Ok(Self::Jewelcrafting),
            756 => Ok(Self::RacialBloodelf),
            758 => Ok(Self::PetEventRc),
            759 => Ok(Self::LanguageDraenei),
            760 => Ok(Self::RacialDraenei),
            761 => Ok(Self::PetFelguard),
            762 => Ok(Self::Riding),
            763 => Ok(Self::PetDragonhawk),
            764 => Ok(Self::PetNetherRay),
            765 => Ok(Self::PetSporebat),
            766 => Ok(Self::PetWarpStalker),
            767 => Ok(Self::PetRavager),
            768 => Ok(Self::PetSerpent),
            769 => Ok(Self::Internal),
            770 => Ok(Self::DkBlood),
            771 => Ok(Self::DkFrost),
            772 => Ok(Self::DkUnholy),
            773 => Ok(Self::Inscription),
            775 => Ok(Self::PetMoth),
            776 => Ok(Self::Runeforging),
            777 => Ok(Self::Mounts),
            778 => Ok(Self::Companions),
            780 => Ok(Self::PetExoticChimaera),
            781 => Ok(Self::PetExoticDevilsaur),
            782 => Ok(Self::PetGhoul),
            783 => Ok(Self::PetExoticSilithid),
            784 => Ok(Self::PetExoticWorm),
            785 => Ok(Self::PetWasp),
            786 => Ok(Self::PetExoticRhino),
            787 => Ok(Self::PetExoticCoreHound),
            788 => Ok(Self::PetExoticSpiritBeast),
            v => Err(crate::errors::EnumError::new("Skill", v as u32),)
        }
    }
}

