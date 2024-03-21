use crate::wrath::{Skill, SkillCategory};

impl Skill {
    pub const fn category(&self) -> Option<SkillCategory> {
        Some(match self {
            Skill::GenericDnd => SkillCategory::Generic,

            Skill::LanguageCommon
            | Skill::LanguageOrcish
            | Skill::LanguageDwarven
            | Skill::LanguageDarnassian
            | Skill::LanguageTaurahe
            | Skill::LanguageThalassian
            | Skill::LanguageDraconic
            | Skill::LanguageDemonTongue
            | Skill::LanguageTitan
            | Skill::LanguageOldTongue
            | Skill::LanguageGnomish
            | Skill::LanguageTroll
            | Skill::LanguageGutterspeak
            | Skill::LanguageDraenei => SkillCategory::Language,

            Skill::Swords
            | Skill::Axes
            | Skill::Bows
            | Skill::Guns
            | Skill::Maces
            | Skill::TwoHandedSwords
            | Skill::Defense
            | Skill::DualWield
            | Skill::Staves
            | Skill::TwoHandedMaces
            | Skill::Unarmed
            | Skill::TwoHandedAxes
            | Skill::Daggers
            | Skill::Thrown
            | Skill::Crossbows
            | Skill::Wands
            | Skill::Polearms
            | Skill::FistWeapons => SkillCategory::Weapon,

            Skill::Frost
            | Skill::Fire
            | Skill::Arms
            | Skill::Combat
            | Skill::Subtlety
            | Skill::BeastMastery
            | Skill::Survival
            | Skill::Holy
            | Skill::Shadow
            | Skill::FeralCombat
            | Skill::Marksmanship
            | Skill::Retribution
            | Skill::PetImp
            | Skill::PetFelhunter
            | Skill::PetSpider
            | Skill::PetVoidwalker
            | Skill::PetSuccubus
            | Skill::PetInfernal
            | Skill::PetDoomguard
            | Skill::PetWolf
            | Skill::PetCat
            | Skill::PetBear
            | Skill::PetBoar
            | Skill::PetCrocilisk
            | Skill::PetCarrionBird
            | Skill::PetCrab
            | Skill::PetGorilla
            | Skill::PetRaptor
            | Skill::PetTallstrider
            | Skill::PetScorpid
            | Skill::Arcane
            | Skill::PetTurtle
            | Skill::Assassination
            | Skill::Fury
            | Skill::Protection
            | Skill::Protection2
            | Skill::PetTalents
            | Skill::Demonology
            | Skill::Affliction
            | Skill::Enhancement
            | Skill::Restoration
            | Skill::ElementalCombat
            | Skill::Restoration2
            | Skill::Balance
            | Skill::Destruction
            | Skill::Holy2
            | Skill::Discipline
            | Skill::Lockpicking
            | Skill::PetBat
            | Skill::PetHyena
            | Skill::PetBirdOfPrey
            | Skill::PetWindSerpent
            | Skill::PetFelguard
            | Skill::PetDragonhawk
            | Skill::PetNetherRay
            | Skill::PetSporebat
            | Skill::PetWarpStalker
            | Skill::PetRavager
            | Skill::PetSerpent
            | Skill::Internal
            | Skill::DkBlood
            | Skill::DkFrost
            | Skill::DkUnholy
            | Skill::PetMoth
            | Skill::Runeforging
            | Skill::Mounts
            | Skill::Companions
            | Skill::PetExoticChimaera
            | Skill::PetExoticDevilsaur
            | Skill::PetGhoul
            | Skill::PetExoticSilithid
            | Skill::PetExoticWorm
            | Skill::PetWasp
            | Skill::PetExoticRhino
            | Skill::PetExoticCoreHound => SkillCategory::Class,

            Skill::PlateMail | Skill::Mail | Skill::Leather | Skill::Cloth | Skill::Shield => {
                SkillCategory::Armor
            }

            Skill::PetExoticSpiritBeast => SkillCategory::Attribute,

            Skill::Blacksmithing
            | Skill::Leatherworking
            | Skill::Alchemy
            | Skill::Herbalism
            | Skill::Mining
            | Skill::Tailoring
            | Skill::Engineering
            | Skill::Enchanting
            | Skill::Skinning
            | Skill::Jewelcrafting
            | Skill::Inscription => SkillCategory::PrimaryProfession,

            Skill::RacialDwarven
            | Skill::RacialTauren
            | Skill::OrcRacial
            | Skill::RacialNightElf
            | Skill::FirstAid
            | Skill::Survival2
            | Skill::RidingHorse
            | Skill::RidingWolf
            | Skill::RidingTiger
            | Skill::RidingRam
            | Skill::Swimming
            | Skill::Cooking
            | Skill::RacialUnded
            | Skill::Fishing
            | Skill::RidingRaptor
            | Skill::RidingMechanostrider
            | Skill::RidingUndeadHorse
            | Skill::RidingKodo
            | Skill::RacialTroll
            | Skill::RacialGnome
            | Skill::RacialHuman
            | Skill::RacialBloodelf
            | Skill::RacialDraenei
            | Skill::Riding => SkillCategory::SecondaryProfession,

            Skill::None | Skill::PetEventRc => return None,
        })
    }
}
