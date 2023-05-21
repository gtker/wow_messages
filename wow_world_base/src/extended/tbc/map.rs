use crate::tbc::Map;

impl Map {
    pub const fn instanced(&self) -> bool {
        match self {
            Map::AlliancePvpBarracks
            | Map::AzsharaCrater
            | Map::CashTest
            | Map::CollinsTest
            | Map::DeeprunTram
            | Map::DevelopmentLand
            | Map::EasternKingdoms
            | Map::EmeraldDream
            | Map::HordePvpBarracks
            | Map::Kalimdor
            | Map::MonasteryUnused
            | Map::OpeningOfTheDarkPortal
            | Map::Outland
            | Map::ScottTest
            | Map::StormwindPrison
            | Map::SunwellFixUnused
            | Map::Testing
            | Map::TransportBootyBayToRatchet
            | Map::TransportExodarToAuberdine
            | Map::TransportFeathermoonFerry
            | Map::TransportGromGolToUndercity
            | Map::TransportMenethilToAuberdine
            | Map::TransportMenethilToTheramore
            | Map::TransportOrgrimmarToGromGol
            | Map::TransportRutTheranToAuberdine
            | Map::TransportUndercityToOrgrimmar => false,

            Map::AhnQirajTemple
            | Map::AlteracValley
            | Map::ArathiBasin
            | Map::AuchindounAuchenaiCrypts
            | Map::AuchindounManaTombs
            | Map::AuchindounSethekkHalls
            | Map::AuchindounShadowLabyrinth
            | Map::BlackTemple
            | Map::BlackfathomDeeps
            | Map::BlackrockDepths
            | Map::BlackrockSpire
            | Map::BlackwingLair
            | Map::BladesEdgeArena
            | Map::CoilfangSerpentshrineCavern
            | Map::CoilfangTheSlavePens
            | Map::CoilfangTheSteamvault
            | Map::CoilfangTheUnderbog
            | Map::Deadmines
            | Map::DireMaul
            | Map::EyeOfTheStorm
            | Map::Gnomeregan
            | Map::GruulsLair
            | Map::HellfireCitadelRamparts
            | Map::HellfireCitadelTheBloodFurnace
            | Map::HellfireCitadelTheShatteredHalls
            | Map::Karazhan
            | Map::MagistersTerrace
            | Map::MagtheridonsLair
            | Map::Maraudon
            | Map::MoltenCore
            | Map::NagrandArena
            | Map::Naxxramas
            | Map::OnyxiasLair
            | Map::RagefireChasm
            | Map::RazorfenDowns
            | Map::RazorfenKraul
            | Map::RuinsOfAhnQiraj
            | Map::RuinsOfLordaeron
            | Map::ScarletMonastery
            | Map::Scholomance
            | Map::ShadowfangKeep
            | Map::StormwindStockade
            | Map::Stratholme
            | Map::SunkenTemple
            | Map::TempestKeep
            | Map::TempestKeepTheArcatraz
            | Map::TempestKeepTheBotanica
            | Map::TempestKeepTheMechanar
            | Map::TheBattleForMountHyjal
            | Map::TheEscapeFromDurnholde
            | Map::TheSunwell
            | Map::Uldaman
            | Map::WailingCaverns
            | Map::WarsongGulch
            | Map::ZulAman
            | Map::ZulFarrak
            | Map::ZulGurub => true,
        }
    }

    pub const fn battleground(&self) -> bool {
        matches!(
            self,
            Map::EyeOfTheStorm | Map::AlteracValley | Map::WarsongGulch | Map::ArathiBasin
        )
    }

    pub const fn arena(&self) -> bool {
        matches!(
            self,
            Map::NagrandArena | Map::BladesEdgeArena | Map::RuinsOfLordaeron
        )
    }

    pub const fn accessible_all(&self) -> bool {
        true
    }

    pub const fn accessible_user(&self) -> bool {
        match self {
            Map::AhnQirajTemple
            | Map::AlliancePvpBarracks
            | Map::AlteracValley
            | Map::ArathiBasin
            | Map::AuchindounAuchenaiCrypts
            | Map::AuchindounManaTombs
            | Map::AuchindounSethekkHalls
            | Map::AuchindounShadowLabyrinth
            | Map::BlackTemple
            | Map::BlackfathomDeeps
            | Map::BlackrockDepths
            | Map::BlackrockSpire
            | Map::BlackwingLair
            | Map::BladesEdgeArena
            | Map::CoilfangSerpentshrineCavern
            | Map::CoilfangTheSlavePens
            | Map::CoilfangTheSteamvault
            | Map::CoilfangTheUnderbog
            | Map::Deadmines
            | Map::DeeprunTram
            | Map::DireMaul
            | Map::EasternKingdoms
            | Map::EyeOfTheStorm
            | Map::Gnomeregan
            | Map::GruulsLair
            | Map::HellfireCitadelRamparts
            | Map::HellfireCitadelTheBloodFurnace
            | Map::HellfireCitadelTheShatteredHalls
            | Map::HordePvpBarracks
            | Map::Kalimdor
            | Map::Karazhan
            | Map::MagistersTerrace
            | Map::MagtheridonsLair
            | Map::Maraudon
            | Map::MoltenCore
            | Map::NagrandArena
            | Map::Naxxramas
            | Map::OnyxiasLair
            | Map::Outland
            | Map::RagefireChasm
            | Map::RazorfenDowns
            | Map::RazorfenKraul
            | Map::RuinsOfAhnQiraj
            | Map::RuinsOfLordaeron
            | Map::ScarletMonastery
            | Map::Scholomance
            | Map::ShadowfangKeep
            | Map::StormwindStockade
            | Map::Stratholme
            | Map::SunkenTemple
            | Map::TempestKeep
            | Map::TempestKeepTheArcatraz
            | Map::TempestKeepTheBotanica
            | Map::TempestKeepTheMechanar
            | Map::TheBattleForMountHyjal
            | Map::TheEscapeFromDurnholde
            | Map::TheSunwell
            | Map::Uldaman
            | Map::WailingCaverns
            | Map::WarsongGulch
            | Map::ZulAman
            | Map::ZulFarrak
            | Map::ZulGurub => true,

            Map::AzsharaCrater
            | Map::CashTest
            | Map::CollinsTest
            | Map::DevelopmentLand
            | Map::EmeraldDream
            | Map::MonasteryUnused
            | Map::OpeningOfTheDarkPortal
            | Map::ScottTest
            | Map::StormwindPrison
            | Map::SunwellFixUnused
            | Map::Testing
            | Map::TransportBootyBayToRatchet
            | Map::TransportExodarToAuberdine
            | Map::TransportFeathermoonFerry
            | Map::TransportGromGolToUndercity
            | Map::TransportMenethilToAuberdine
            | Map::TransportMenethilToTheramore
            | Map::TransportOrgrimmarToGromGol
            | Map::TransportRutTheranToAuberdine
            | Map::TransportUndercityToOrgrimmar => false,
        }
    }

    /// Returns the name of the folder inside the MPQ that contains the map files.
    pub const fn directory_name(&self) -> &'static str {
        match self {
            Self::EasternKingdoms => "Azeroth",
            Self::Kalimdor => "Kalimdor",
            Self::Testing => "test",
            Self::ScottTest => "ScottTest",
            Self::CashTest => "Test",
            Self::AlteracValley => "PVPZone01",
            Self::ShadowfangKeep => "Shadowfang",
            Self::StormwindStockade => "StormwindJail",
            Self::StormwindPrison => "StormwindPrison",
            Self::Deadmines => "DeadminesInstance",
            Self::AzsharaCrater => "PVPZone02",
            Self::CollinsTest => "Collin",
            Self::WailingCaverns => "WailingCaverns",
            Self::MonasteryUnused => "Monastery",
            Self::RazorfenKraul => "RazorfenKraulInstance",
            Self::BlackfathomDeeps => "Blackfathom",
            Self::Uldaman => "Uldaman",
            Self::Gnomeregan => "GnomeragonInstance",
            Self::SunkenTemple => "SunkenTemple",
            Self::RazorfenDowns => "RazorfenDowns",
            Self::EmeraldDream => "EmeraldDream",
            Self::ScarletMonastery => "MonasteryInstances",
            Self::ZulFarrak => "TanarisInstance",
            Self::BlackrockSpire => "BlackRockSpire",
            Self::BlackrockDepths => "BlackrockDepths",
            Self::OnyxiasLair => "OnyxiaLairInstance",
            Self::OpeningOfTheDarkPortal => "CavernsOfTime",
            Self::Scholomance => "SchoolofNecromancy",
            Self::ZulGurub => "Zul'gurub",
            Self::Stratholme => "Stratholme",
            Self::Maraudon => "Mauradon",
            Self::DeeprunTram => "DeeprunTram",
            Self::RagefireChasm => "OrgrimmarInstance",
            Self::MoltenCore => "MoltenCore",
            Self::DireMaul => "DireMaul",
            Self::AlliancePvpBarracks => "AlliancePVPBarracks",
            Self::HordePvpBarracks => "HordePVPBarracks",
            Self::DevelopmentLand => "development",
            Self::BlackwingLair => "BlackwingLair",
            Self::WarsongGulch => "PVPZone03",
            Self::RuinsOfAhnQiraj => "AhnQiraj",
            Self::ArathiBasin => "PVPZone04",
            Self::Outland => "Expansion01",
            Self::AhnQirajTemple => "AhnQirajTemple",
            Self::Karazhan => "Karazahn",
            Self::Naxxramas => "Stratholme Raid",
            Self::TheBattleForMountHyjal => "HyjalPast",
            Self::HellfireCitadelTheShatteredHalls => "HellfireMilitary",
            Self::HellfireCitadelTheBloodFurnace => "HellfireDemon",
            Self::HellfireCitadelRamparts => "HellfireRampart",
            Self::MagtheridonsLair => "HellfireRaid",
            Self::CoilfangTheSteamvault => "CoilfangPumping",
            Self::CoilfangTheUnderbog => "CoilfangMarsh",
            Self::CoilfangTheSlavePens => "CoilfangDraenei",
            Self::CoilfangSerpentshrineCavern => "CoilfangRaid",
            Self::TempestKeep => "TempestKeepRaid",
            Self::TempestKeepTheArcatraz => "TempestKeepArcane",
            Self::TempestKeepTheBotanica => "TempestKeepAtrium",
            Self::TempestKeepTheMechanar => "TempestKeepFactory",
            Self::AuchindounShadowLabyrinth => "AuchindounShadow",
            Self::AuchindounSethekkHalls => "AuchindounDemon",
            Self::AuchindounManaTombs => "AuchindounEthereal",
            Self::AuchindounAuchenaiCrypts => "AuchindounDraenei",
            Self::NagrandArena => "PVPZone05",
            Self::TheEscapeFromDurnholde => "HillsbradPast",
            Self::BladesEdgeArena => "bladesedgearena",
            Self::BlackTemple => "BlackTemple",
            Self::GruulsLair => "GruulsLair",
            Self::EyeOfTheStorm => "NetherstormBG",
            Self::ZulAman => "ZulAman",
            Self::RuinsOfLordaeron => "PVPLordaeron",
            Self::TheSunwell => "SunwellPlateau",
            Self::TransportRutTheranToAuberdine => "Transport176244",
            Self::TransportMenethilToTheramore => "Transport176231",
            Self::MagistersTerrace => "Sunwell5ManFix",
            Self::TransportExodarToAuberdine => "Transport181645",
            Self::TransportFeathermoonFerry => "Transport177233",
            Self::TransportMenethilToAuberdine => "Transport176310",
            Self::TransportOrgrimmarToGromGol => "Transport175080",
            Self::TransportGromGolToUndercity => "Transport176495",
            Self::TransportUndercityToOrgrimmar => "Transport164871",
            Self::TransportBootyBayToRatchet => "Transport20808",
            Self::SunwellFixUnused => "Sunwell5Man",
        }
    }
}
