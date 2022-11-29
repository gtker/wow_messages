use crate::wrath::Map;

impl Map {
    pub const fn instanced(&self) -> bool {
        match self {
            Map::AlliancePvpBarracks
            | Map::AzsharaCrater
            | Map::CollinsTest
            | Map::CraigTest
            | Map::DeeprunTram
            | Map::DevelopmentLand
            | Map::DevelopmentLandNonWeightedTextures
            | Map::EasternKingdoms
            | Map::EmeraldDream
            | Map::Exteriortest
            | Map::HordePvpBarracks
            | Map::Kalimdor
            | Map::MonasteryUnused
            | Map::Northrend
            | Map::OpeningOfTheDarkPortal
            | Map::Outland
            | Map::QaAndDvd
            | Map::ScottTest
            | Map::Stormwind
            | Map::StormwindPrison
            | Map::SunwellFixUnused
            | Map::Testing
            | Map::TransportAllianceAirshipBg
            | Map::TransportBootyBayToRatchet
            | Map::TransportBoreanTundraTest
            | Map::TransportExodarToAuberdine
            | Map::TransportFeathermoonFerry
            | Map::TransportGromGolToUndercity
            | Map::TransportHordeairshipbg
            | Map::TransportHowlingFjordSisterMercyQuest
            | Map::TransportMenethilToAuberdine
            | Map::TransportMenethilToTheramore
            | Map::TransportMenethilToValgarde
            | Map::TransportMoaKiToKamagua
            | Map::TransportMoaKiToUnuPe
            | Map::TransportNaglfar
            | Map::TransportOrgrimmarToGromGol
            | Map::TransportOrgrimmarToThunderBluff
            | Map::TransportOrgrimmarToWarsongHold
            | Map::TransportOrgrimsHammer
            | Map::TransportOrgrimsHammerIcDungeon
            | Map::TransportOrgrimsHammerIcecrownCitadelRaid
            | Map::TransportRutTheranToAuberdine
            | Map::TransportStormwindToValianceKeep
            | Map::TransportTheSkybreaker
            | Map::TransportTheSkybreakerIcDungeon
            | Map::TransportTheSkybreakerIcecrownCitadelRaid
            | Map::TransportTirisfalToVengeanceLanding
            | Map::TransportUndercityToOrgrimmar
            | Map::TransportTheMightyWindIcecrownCitadelRaid => false,

            Map::AhnKahetTheOldKingdom
            | Map::AhnQirajTemple
            | Map::AlteracValley
            | Map::ArathiBasin
            | Map::AuchindounAuchenaiCrypts
            | Map::AuchindounManaTombs
            | Map::AuchindounSethekkHalls
            | Map::AuchindounShadowLabyrinth
            | Map::AzjolNerub
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
            | Map::DalaranSewers
            | Map::Deadmines
            | Map::DireMaul
            | Map::DrakTharonKeep
            | Map::EbonHold
            | Map::EyeOfTheStorm
            | Map::Gnomeregan
            | Map::GruulsLair
            | Map::Gundrak
            | Map::HallsOfLightning
            | Map::HallsOfReflection
            | Map::HallsOfStone
            | Map::HellfireCitadelRamparts
            | Map::HellfireCitadelTheBloodFurnace
            | Map::HellfireCitadelTheShatteredHalls
            | Map::IcecrownCitadel
            | Map::IsleOfConquest
            | Map::Karazhan
            | Map::MagistersTerrace
            | Map::MagtheridonsLair
            | Map::Maraudon
            | Map::MoltenCore
            | Map::NagrandArena
            | Map::Naxxramas
            | Map::OnyxiasLair
            | Map::PitOfSaron
            | Map::RagefireChasm
            | Map::RazorfenDowns
            | Map::RazorfenKraul
            | Map::RuinsOfAhnQiraj
            | Map::RuinsOfLordaeron
            | Map::ScarletMonastery
            | Map::Scholomance
            | Map::ShadowfangKeep
            | Map::StormwindStockade
            | Map::StrandOfTheAncients
            | Map::Stratholme
            | Map::SunkenTemple
            | Map::TempestKeep
            | Map::TempestKeepTheArcatraz
            | Map::TempestKeepTheBotanica
            | Map::TempestKeepTheMechanar
            | Map::TheBattleForMountHyjal
            | Map::TheCullingOfStratholme
            | Map::TheEscapeFromDurnholde
            | Map::TheEyeOfEternity
            | Map::TheForgeOfSouls
            | Map::TheNexus
            | Map::TheObsidianSanctum
            | Map::TheOculus
            | Map::TheRingOfValor
            | Map::TheRubySanctum
            | Map::TheSunwell
            | Map::TrialOfTheChampion
            | Map::TrialOfTheCrusader
            | Map::Uldaman
            | Map::Ulduar
            | Map::UtgardeKeep
            | Map::UtgardePinnacle
            | Map::VaultOfArchavon
            | Map::VioletHold
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
            Map::IsleOfConquest
                | Map::StrandOfTheAncients
                | Map::EyeOfTheStorm
                | Map::AlteracValley
                | Map::WarsongGulch
                | Map::ArathiBasin
        )
    }

    pub const fn arena(&self) -> bool {
        matches!(
            self,
            Map::DalaranSewers
                | Map::TheRingOfValor
                | Map::NagrandArena
                | Map::BladesEdgeArena
                | Map::RuinsOfLordaeron
        )
    }

    pub const fn accessible_all(&self) -> bool {
        !matches!(self, Map::DevelopmentLand)
    }

    pub const fn accessible_user(&self) -> bool {
        match self {
            Map::AhnKahetTheOldKingdom
            | Map::AhnQirajTemple
            | Map::AlliancePvpBarracks
            | Map::AlteracValley
            | Map::ArathiBasin
            | Map::AuchindounAuchenaiCrypts
            | Map::AuchindounManaTombs
            | Map::AuchindounSethekkHalls
            | Map::AuchindounShadowLabyrinth
            | Map::AzjolNerub
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
            | Map::DalaranSewers
            | Map::Deadmines
            | Map::DeeprunTram
            | Map::DireMaul
            | Map::DrakTharonKeep
            | Map::EasternKingdoms
            | Map::EbonHold
            | Map::EyeOfTheStorm
            | Map::Gnomeregan
            | Map::GruulsLair
            | Map::Gundrak
            | Map::HallsOfLightning
            | Map::HallsOfReflection
            | Map::HallsOfStone
            | Map::HellfireCitadelRamparts
            | Map::HellfireCitadelTheBloodFurnace
            | Map::HellfireCitadelTheShatteredHalls
            | Map::HordePvpBarracks
            | Map::IcecrownCitadel
            | Map::IsleOfConquest
            | Map::Kalimdor
            | Map::Karazhan
            | Map::MagistersTerrace
            | Map::MagtheridonsLair
            | Map::Maraudon
            | Map::MoltenCore
            | Map::NagrandArena
            | Map::Naxxramas
            | Map::Northrend
            | Map::OnyxiasLair
            | Map::Outland
            | Map::PitOfSaron
            | Map::RagefireChasm
            | Map::RazorfenDowns
            | Map::RazorfenKraul
            | Map::RuinsOfAhnQiraj
            | Map::RuinsOfLordaeron
            | Map::ScarletMonastery
            | Map::Scholomance
            | Map::ShadowfangKeep
            | Map::Stormwind
            | Map::StormwindStockade
            | Map::StrandOfTheAncients
            | Map::Stratholme
            | Map::SunkenTemple
            | Map::TempestKeep
            | Map::TempestKeepTheArcatraz
            | Map::TempestKeepTheBotanica
            | Map::TempestKeepTheMechanar
            | Map::TheBattleForMountHyjal
            | Map::TheCullingOfStratholme
            | Map::TheEscapeFromDurnholde
            | Map::TheEyeOfEternity
            | Map::TheForgeOfSouls
            | Map::TheNexus
            | Map::TheObsidianSanctum
            | Map::TheOculus
            | Map::TheRingOfValor
            | Map::TheRubySanctum
            | Map::TheSunwell
            | Map::TrialOfTheChampion
            | Map::TrialOfTheCrusader
            | Map::Uldaman
            | Map::Ulduar
            | Map::UtgardeKeep
            | Map::UtgardePinnacle
            | Map::VaultOfArchavon
            | Map::VioletHold
            | Map::WailingCaverns
            | Map::WarsongGulch
            | Map::ZulAman
            | Map::ZulFarrak
            | Map::ZulGurub => true,

            Map::AzsharaCrater
            | Map::CollinsTest
            | Map::CraigTest
            | Map::DevelopmentLand
            | Map::DevelopmentLandNonWeightedTextures
            | Map::EmeraldDream
            | Map::Exteriortest
            | Map::MonasteryUnused
            | Map::OpeningOfTheDarkPortal
            | Map::QaAndDvd
            | Map::ScottTest
            | Map::StormwindPrison
            | Map::SunwellFixUnused
            | Map::Testing
            | Map::TransportAllianceAirshipBg
            | Map::TransportBootyBayToRatchet
            | Map::TransportBoreanTundraTest
            | Map::TransportExodarToAuberdine
            | Map::TransportFeathermoonFerry
            | Map::TransportGromGolToUndercity
            | Map::TransportHordeairshipbg
            | Map::TransportHowlingFjordSisterMercyQuest
            | Map::TransportMenethilToAuberdine
            | Map::TransportMenethilToTheramore
            | Map::TransportMenethilToValgarde
            | Map::TransportMoaKiToKamagua
            | Map::TransportMoaKiToUnuPe
            | Map::TransportNaglfar
            | Map::TransportOrgrimmarToGromGol
            | Map::TransportOrgrimmarToThunderBluff
            | Map::TransportOrgrimmarToWarsongHold
            | Map::TransportOrgrimsHammer
            | Map::TransportOrgrimsHammerIcDungeon
            | Map::TransportOrgrimsHammerIcecrownCitadelRaid
            | Map::TransportRutTheranToAuberdine
            | Map::TransportStormwindToValianceKeep
            | Map::TransportTheSkybreaker
            | Map::TransportTheSkybreakerIcDungeon
            | Map::TransportTheSkybreakerIcecrownCitadelRaid
            | Map::TransportTirisfalToVengeanceLanding
            | Map::TransportUndercityToOrgrimmar
            | Map::TransportTheMightyWindIcecrownCitadelRaid => false,
        }
    }
}
