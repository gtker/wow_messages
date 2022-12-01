use crate::manual::vanilla::player_race::PlayerRace;
use crate::tbc::Class;
use crate::vanilla::Race;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

/// Enum containing only the allowed race/class combinations.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum RaceClass {
    DwarfHunter,
    DwarfPaladin,
    DwarfPriest,
    DwarfRogue,
    DwarfWarrior,
    GnomeMage,
    GnomeRogue,
    GnomeWarlock,
    GnomeWarrior,
    HumanMage,
    HumanPaladin,
    HumanPriest,
    HumanRogue,
    HumanWarlock,
    HumanWarrior,
    NightElfDruid,
    NightElfHunter,
    NightElfPriest,
    NightElfRogue,
    NightElfWarrior,
    OrcHunter,
    OrcRogue,
    OrcShaman,
    OrcWarlock,
    OrcWarrior,
    TaurenDruid,
    TaurenHunter,
    TaurenShaman,
    TaurenWarrior,
    TrollHunter,
    TrollMage,
    TrollPriest,
    TrollRogue,
    TrollShaman,
    TrollWarrior,
    UndeadMage,
    UndeadPriest,
    UndeadRogue,
    UndeadWarlock,
    UndeadWarrior,
}

impl RaceClass {
    pub const fn race(&self) -> PlayerRace {
        self.to_race_class().0
    }

    pub const fn class(&self) -> Class {
        self.to_race_class().1
    }

    pub const fn to_race_class(&self) -> (PlayerRace, Class) {
        match self {
            RaceClass::DwarfHunter => (PlayerRace::Dwarf, Class::Hunter),
            RaceClass::DwarfPaladin => (PlayerRace::Dwarf, Class::Paladin),
            RaceClass::DwarfPriest => (PlayerRace::Dwarf, Class::Priest),
            RaceClass::DwarfRogue => (PlayerRace::Dwarf, Class::Rogue),
            RaceClass::DwarfWarrior => (PlayerRace::Dwarf, Class::Warrior),
            RaceClass::GnomeMage => (PlayerRace::Gnome, Class::Mage),
            RaceClass::GnomeRogue => (PlayerRace::Gnome, Class::Rogue),
            RaceClass::GnomeWarlock => (PlayerRace::Gnome, Class::Warlock),
            RaceClass::GnomeWarrior => (PlayerRace::Gnome, Class::Warrior),
            RaceClass::HumanMage => (PlayerRace::Human, Class::Mage),
            RaceClass::HumanPaladin => (PlayerRace::Human, Class::Paladin),
            RaceClass::HumanPriest => (PlayerRace::Human, Class::Priest),
            RaceClass::HumanRogue => (PlayerRace::Human, Class::Rogue),
            RaceClass::HumanWarlock => (PlayerRace::Human, Class::Warlock),
            RaceClass::HumanWarrior => (PlayerRace::Human, Class::Warrior),
            RaceClass::NightElfDruid => (PlayerRace::NightElf, Class::Druid),
            RaceClass::NightElfHunter => (PlayerRace::NightElf, Class::Hunter),
            RaceClass::NightElfPriest => (PlayerRace::NightElf, Class::Priest),
            RaceClass::NightElfRogue => (PlayerRace::NightElf, Class::Rogue),
            RaceClass::NightElfWarrior => (PlayerRace::NightElf, Class::Warrior),
            RaceClass::OrcHunter => (PlayerRace::Orc, Class::Hunter),
            RaceClass::OrcRogue => (PlayerRace::Orc, Class::Rogue),
            RaceClass::OrcShaman => (PlayerRace::Orc, Class::Shaman),
            RaceClass::OrcWarlock => (PlayerRace::Orc, Class::Warlock),
            RaceClass::OrcWarrior => (PlayerRace::Orc, Class::Warrior),
            RaceClass::TaurenDruid => (PlayerRace::Tauren, Class::Druid),
            RaceClass::TaurenHunter => (PlayerRace::Tauren, Class::Hunter),
            RaceClass::TaurenShaman => (PlayerRace::Tauren, Class::Shaman),
            RaceClass::TaurenWarrior => (PlayerRace::Tauren, Class::Warrior),
            RaceClass::TrollHunter => (PlayerRace::Troll, Class::Hunter),
            RaceClass::TrollMage => (PlayerRace::Troll, Class::Mage),
            RaceClass::TrollPriest => (PlayerRace::Troll, Class::Priest),
            RaceClass::TrollRogue => (PlayerRace::Troll, Class::Rogue),
            RaceClass::TrollShaman => (PlayerRace::Troll, Class::Shaman),
            RaceClass::TrollWarrior => (PlayerRace::Troll, Class::Warrior),
            RaceClass::UndeadMage => (PlayerRace::Undead, Class::Mage),
            RaceClass::UndeadPriest => (PlayerRace::Undead, Class::Priest),
            RaceClass::UndeadRogue => (PlayerRace::Undead, Class::Rogue),
            RaceClass::UndeadWarlock => (PlayerRace::Undead, Class::Warlock),
            RaceClass::UndeadWarrior => (PlayerRace::Undead, Class::Warrior),
        }
    }
}

impl TryFrom<(Race, Class)> for RaceClass {
    type Error = (Race, Class);

    fn try_from(value: (Race, Class)) -> Result<Self, Self::Error> {
        Ok(match value {
            (Race::Dwarf, Class::Hunter) => Self::DwarfHunter,
            (Race::Dwarf, Class::Paladin) => Self::DwarfPaladin,
            (Race::Dwarf, Class::Priest) => Self::DwarfPriest,
            (Race::Dwarf, Class::Rogue) => Self::DwarfRogue,
            (Race::Dwarf, Class::Warrior) => Self::DwarfWarrior,
            (Race::Gnome, Class::Mage) => Self::GnomeMage,
            (Race::Gnome, Class::Rogue) => Self::GnomeRogue,
            (Race::Gnome, Class::Warlock) => Self::GnomeWarlock,
            (Race::Gnome, Class::Warrior) => Self::GnomeWarrior,
            (Race::Human, Class::Mage) => Self::HumanMage,
            (Race::Human, Class::Paladin) => Self::HumanPaladin,
            (Race::Human, Class::Priest) => Self::HumanPriest,
            (Race::Human, Class::Rogue) => Self::HumanRogue,
            (Race::Human, Class::Warlock) => Self::HumanWarlock,
            (Race::Human, Class::Warrior) => Self::HumanWarrior,
            (Race::NightElf, Class::Druid) => Self::NightElfDruid,
            (Race::NightElf, Class::Hunter) => Self::NightElfHunter,
            (Race::NightElf, Class::Priest) => Self::NightElfPriest,
            (Race::NightElf, Class::Rogue) => Self::NightElfRogue,
            (Race::NightElf, Class::Warrior) => Self::NightElfWarrior,
            (Race::Orc, Class::Hunter) => Self::OrcHunter,
            (Race::Orc, Class::Rogue) => Self::OrcRogue,
            (Race::Orc, Class::Shaman) => Self::OrcShaman,
            (Race::Orc, Class::Warlock) => Self::OrcWarlock,
            (Race::Orc, Class::Warrior) => Self::OrcWarrior,
            (Race::Tauren, Class::Druid) => Self::TaurenDruid,
            (Race::Tauren, Class::Hunter) => Self::TaurenHunter,
            (Race::Tauren, Class::Shaman) => Self::TaurenShaman,
            (Race::Tauren, Class::Warrior) => Self::TaurenWarrior,
            (Race::Troll, Class::Hunter) => Self::TrollHunter,
            (Race::Troll, Class::Mage) => Self::TrollMage,
            (Race::Troll, Class::Priest) => Self::TrollPriest,
            (Race::Troll, Class::Rogue) => Self::TrollRogue,
            (Race::Troll, Class::Shaman) => Self::TrollShaman,
            (Race::Troll, Class::Warrior) => Self::TrollWarrior,
            (Race::Undead, Class::Mage) => Self::UndeadMage,
            (Race::Undead, Class::Priest) => Self::UndeadPriest,
            (Race::Undead, Class::Rogue) => Self::UndeadRogue,
            (Race::Undead, Class::Warlock) => Self::UndeadWarlock,
            (Race::Undead, Class::Warrior) => Self::UndeadWarrior,
            v => return Err(v),
        })
    }
}

impl TryFrom<(PlayerRace, Class)> for RaceClass {
    type Error = (PlayerRace, Class);

    fn try_from(value: (PlayerRace, Class)) -> Result<Self, Self::Error> {
        let race: Race = value.0.into();
        let class = value.1;
        match RaceClass::try_from((race, class)) {
            Ok(e) => Ok(e),
            Err(_) => Err(value),
        }
    }
}

impl Default for RaceClass {
    fn default() -> Self {
        Self::HumanWarrior
    }
}

impl Display for RaceClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            RaceClass::DwarfHunter => "Dwarf Hunter",
            RaceClass::DwarfPaladin => "Dwarf Paladin",
            RaceClass::DwarfPriest => "Dwarf Priest",
            RaceClass::DwarfRogue => "Dwarf Rogue",
            RaceClass::DwarfWarrior => "Dwarf Warrior",
            RaceClass::GnomeMage => "Gnome Mage",
            RaceClass::GnomeRogue => "Gnome Rogue",
            RaceClass::GnomeWarlock => "Gnome Warlock",
            RaceClass::GnomeWarrior => "Gnome Warrior",
            RaceClass::HumanMage => "Human Mage",
            RaceClass::HumanPaladin => "Human Paladin",
            RaceClass::HumanPriest => "Human Priest",
            RaceClass::HumanRogue => "Human Rogue",
            RaceClass::HumanWarlock => "Human Warlock",
            RaceClass::HumanWarrior => "Human Warrior",
            RaceClass::NightElfDruid => "Night Elf Druid",
            RaceClass::NightElfHunter => "Night Elf Hunter",
            RaceClass::NightElfPriest => "Night Elf Priest",
            RaceClass::NightElfRogue => "Night Elf Rogue",
            RaceClass::NightElfWarrior => "Night Elf Warrior",
            RaceClass::OrcHunter => "Orc Hunter",
            RaceClass::OrcRogue => "Orc Rogue",
            RaceClass::OrcShaman => "Orc Shaman",
            RaceClass::OrcWarlock => "Orc Warlock",
            RaceClass::OrcWarrior => "Orc Warrior",
            RaceClass::TaurenDruid => "Tauren Druid",
            RaceClass::TaurenHunter => "Tauren Hunter",
            RaceClass::TaurenShaman => "Tauren Shaman",
            RaceClass::TaurenWarrior => "Tauren Warrior",
            RaceClass::TrollHunter => "Troll Hunter",
            RaceClass::TrollMage => "Troll Mage",
            RaceClass::TrollPriest => "Troll Priest",
            RaceClass::TrollRogue => "Troll Rogue",
            RaceClass::TrollShaman => "Troll Shaman",
            RaceClass::TrollWarrior => "Troll Warrior",
            RaceClass::UndeadMage => "Undead Mage",
            RaceClass::UndeadPriest => "Undead Priest",
            RaceClass::UndeadRogue => "Undead Rogue",
            RaceClass::UndeadWarlock => "Undead Warlock",
            RaceClass::UndeadWarrior => "Undead Warrior",
        })
    }
}
