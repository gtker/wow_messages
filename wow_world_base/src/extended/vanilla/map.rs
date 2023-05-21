use crate::vanilla::Map;

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
            | Map::ScottTest
            | Map::StormwindPrison
            | Map::Testing => false,

            Map::AhnQirajTemple
            | Map::AlteracValley
            | Map::ArathiBasin
            | Map::BlackfathomDeeps
            | Map::BlackrockDepths
            | Map::BlackrockSpire
            | Map::BlackwingLair
            | Map::Deadmines
            | Map::DireMaul
            | Map::Gnomeregan
            | Map::Maraudon
            | Map::MoltenCore
            | Map::Naxxramas
            | Map::OnyxiasLair
            | Map::RagefireChasm
            | Map::RazorfenDowns
            | Map::RazorfenKraul
            | Map::RuinsOfAhnQiraj
            | Map::ScarletMonastery
            | Map::Scholomance
            | Map::ShadowfangKeep
            | Map::StormwindStockade
            | Map::Stratholme
            | Map::SunkenTemple
            | Map::Uldaman
            | Map::WailingCaverns
            | Map::WarsongGulch
            | Map::ZulFarrak
            | Map::ZulGurub => true,
        }
    }

    pub const fn battleground(&self) -> bool {
        matches!(
            self,
            Map::AlteracValley | Map::WarsongGulch | Map::ArathiBasin
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
            | Map::BlackfathomDeeps
            | Map::BlackrockDepths
            | Map::BlackrockSpire
            | Map::BlackwingLair
            | Map::Deadmines
            | Map::DeeprunTram
            | Map::DireMaul
            | Map::EasternKingdoms
            | Map::Gnomeregan
            | Map::HordePvpBarracks
            | Map::Kalimdor
            | Map::Maraudon
            | Map::MoltenCore
            | Map::Naxxramas
            | Map::OnyxiasLair
            | Map::RagefireChasm
            | Map::RazorfenDowns
            | Map::RazorfenKraul
            | Map::RuinsOfAhnQiraj
            | Map::ScarletMonastery
            | Map::Scholomance
            | Map::ShadowfangKeep
            | Map::StormwindStockade
            | Map::Stratholme
            | Map::SunkenTemple
            | Map::Uldaman
            | Map::WailingCaverns
            | Map::WarsongGulch
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
            | Map::Testing => false,
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
            Self::AhnQirajTemple => "AhnQirajTemple",
            Self::Naxxramas => "Stratholme Raid",
        }
    }
}
