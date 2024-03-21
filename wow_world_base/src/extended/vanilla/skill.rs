use crate::vanilla::{Skill, SkillCategory};

impl Skill {
    pub const fn category(&self) -> Option<SkillCategory> {
        Some(match self {
            Skill::PlateMail | Skill::Mail | Skill::Leather | Skill::Cloth | Skill::Shield => {
                SkillCategory::Armor
            }

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
            | Skill::Riding => SkillCategory::SecondaryProfession,

            Skill::Frost
            | Skill::Fire
            | Skill::Arms
            | Skill::Combat
            | Skill::Subtlety
            | Skill::Poisons
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
            | Skill::BeastTraining
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
            | Skill::PetOwl
            | Skill::PetWindSerpent => SkillCategory::Class,

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

            Skill::Blacksmithing
            | Skill::Leatherworking
            | Skill::Alchemy
            | Skill::Herbalism
            | Skill::Mining
            | Skill::Tailoring
            | Skill::Engineering
            | Skill::Enchanting
            | Skill::Skinning => SkillCategory::PrimaryProfession,

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
            | Skill::LanguageGutterspeak => SkillCategory::Language,

            Skill::GenericDnd => SkillCategory::Generic,

            Skill::None | Skill::PetEventRc => return None,
        })
    }
}
