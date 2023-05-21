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

    /// Returns the name of the folder inside the MPQ that contains the map files.
    pub const fn directory_name(&self) -> &'static str {
        match self {
            Self::EasternKingdoms => "Azeroth",
            Self::Kalimdor => "Kalimdor",
            Self::Testing => "test",
            Self::ScottTest => "ScottTest",
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
            Self::Northrend => "Northrend",
            Self::RuinsOfLordaeron => "PVPLordaeron",
            Self::Exteriortest => "ExteriorTest",
            Self::UtgardeKeep => "Valgarde70",
            Self::UtgardePinnacle => "UtgardePinnacle",
            Self::TheNexus => "Nexus70",
            Self::TheOculus => "Nexus80",
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
            Self::TransportBoreanTundraTest => "Transport186238",
            Self::TransportBootyBayToRatchet => "Transport20808",
            Self::TransportHowlingFjordSisterMercyQuest => "Transport187038",
            Self::TheCullingOfStratholme => "StratholmeCOT",
            Self::TransportNaglfar => "Transport187263",
            Self::CraigTest => "CraigTest",
            Self::SunwellFixUnused => "Sunwell5Man",
            Self::HallsOfStone => "Ulduar70",
            Self::DrakTharonKeep => "DrakTheronKeep",
            Self::AzjolNerub => "Azjol_Uppercity",
            Self::HallsOfLightning => "Ulduar80",
            Self::Ulduar => "UlduarRaid",
            Self::Gundrak => "GunDrak",
            Self::DevelopmentLandNonWeightedTextures => "development_nonweighted",
            Self::QaAndDvd => "QA_DVD",
            Self::StrandOfTheAncients => "NorthrendBG",
            Self::VioletHold => "DalaranPrison",
            Self::EbonHold => "DeathKnightStart",
            Self::TransportTirisfalToVengeanceLanding => "Transport_Tirisfal _Vengeance_Landing",
            Self::TransportMenethilToValgarde => "Transport_Menethil_Valgarde",
            Self::TransportOrgrimmarToWarsongHold => "Transport_Orgrimmar_Warsong_Hold",
            Self::TransportStormwindToValianceKeep => "Transport_Stormwind_Valiance_Keep",
            Self::TheObsidianSanctum => "ChamberOfAspectsBlack",
            Self::TheEyeOfEternity => "NexusRaid",
            Self::DalaranSewers => "DalaranArena",
            Self::TheRingOfValor => "OrgrimmarArena",
            Self::AhnKahetTheOldKingdom => "Azjol_LowerCity",
            Self::TransportMoaKiToUnuPe => "Transport_Moa'ki_Unu'pe",
            Self::TransportMoaKiToKamagua => "Transport_Moa'ki_Kamagua",
            Self::TransportOrgrimsHammer => "Transport192241",
            Self::TransportTheSkybreaker => "Transport192242",
            Self::VaultOfArchavon => "WintergraspRaid",
            Self::IsleOfConquest => "IsleofConquest",
            Self::IcecrownCitadel => "IcecrownCitadel",
            Self::TheForgeOfSouls => "IcecrownCitadel5Man",
            Self::TransportAllianceAirshipBg => "Transport_AllianceAirshipBG",
            Self::TransportHordeairshipbg => "Transport_HordeAirshipBG",
            Self::TransportOrgrimmarToThunderBluff => "Transport_Orgrimmar_to_Thunderbluff",
            Self::TrialOfTheCrusader => "ArgentTournamentRaid",
            Self::TrialOfTheChampion => "ArgentTournamentDungeon",
            Self::PitOfSaron => "QuarryofTears",
            Self::HallsOfReflection => "HallsOfReflection",
            Self::TransportTheSkybreakerIcecrownCitadelRaid => "Transport197347",
            Self::TransportOrgrimsHammerIcecrownCitadelRaid => "Transport197348",
            Self::TransportTheSkybreakerIcDungeon => "Transport197349",
            Self::TransportOrgrimsHammerIcDungeon => "Transport197350",
            Self::TransportTheMightyWindIcecrownCitadelRaid => "Transport201834",
            Self::Stormwind => "Stormwind",
            Self::TheRubySanctum => "ChamberofAspectsRed",
        }
    }
}
