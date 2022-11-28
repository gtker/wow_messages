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
            | Map::Monastery
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
        match self {
            Map::AlteracValley | Map::WarsongGulch | Map::ArathiBasin => true,
            _ => false,
        }
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
            | Map::Monastery
            | Map::OpeningOfTheDarkPortal
            | Map::ScottTest
            | Map::StormwindPrison
            | Map::Testing => false,
        }
    }
}
