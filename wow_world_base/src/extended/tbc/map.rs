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
}
