use crate::vanilla::position::{
    Position, DWARF_START_POSITION, GNOME_START_POSITION, HUMAN_START_POSITION,
    NIGHT_ELF_START_POSITION, ORC_START_POSITION, TAUREN_START_POSITION, TROLL_START_POSITION,
    UNDEAD_START_POSITION,
};
use std::fmt::Write;
use std::fs::read_to_string;
use wow_world_base::vanilla::Map;

#[test]
fn generate_positions() {
    let s = generate();

    const FILE: &str = "src/vanilla/position/positions.rs";

    let current = read_to_string(FILE).unwrap();

    if current != s.inner {
        std::fs::write(FILE, s.inner).unwrap();
        panic!("Generated positions do not match")
    }
}

fn generate() -> Writer {
    let mut s = Writer::new();
    s.wln("use crate::vanilla::position::Position;");
    s.wln("use wow_world_base::vanilla::Map;");
    s.newline();

    s.wln("pub fn get_position_from_str(name: &str) -> Option<Position> {");
    s.wln("    let i = match name {");

    for p in positions() {
        s.w("        ");
        for (i, name) in p.names.iter().enumerate() {
            if i != 0 {
                s.w("| ");
            }
            s.w(format!("\"{}\" ", get_string_name(name)))
        }

        s.wln(format!(
            "=> PositionIdentifier::{},",
            get_enum_name(p.names[0])
        ));
    }

    s.wln("        _ => return None,");

    s.wln("    };");
    s.newline();

    s.wln("    Some(get_position(i))");
    s.wln("}");
    s.newline();

    s.wln("pub const fn get_position(ident: PositionIdentifier) -> Position {");
    s.wln("    let i = match ident {");

    for (i, e) in positions().iter().enumerate() {
        s.wln(format!(
            "        PositionIdentifier::{} => {},",
            get_enum_name(e.names[0]),
            i
        ));
    }

    s.wln("    };");
    s.newline();
    s.wln("    POSITIONS[i]");
    s.wln("}");

    s.wln("pub enum PositionIdentifier {");

    for i in positions() {
        s.wln(format!("    {},", get_enum_name(i.names[0])));
    }

    s.wln("}");

    s.wln(format!(
        "const POSITIONS: [Position; {}] = [",
        positions().len()
    ));

    for i in positions() {
        s.wln(format!(
            "    Position::new(Map::{:?}, {}_f32, {}_f32, {}_f32, {}_f32),",
            i.position.map, i.position.x, i.position.y, i.position.z, i.position.orientation,
        ));
    }

    s.wln("];");
    s
}

fn get_enum_name(s: &str) -> String {
    s.replace('\'', "").replace(' ', "")
}

fn get_string_name(s: &str) -> String {
    s.replace('\'', "").replace(' ', "_").to_lowercase()
}

pub struct Writer {
    inner: String,
}

impl Writer {
    fn new() -> Self {
        Self {
            inner: String::with_capacity(4000),
        }
    }

    fn w(&mut self, s: impl AsRef<str>) {
        self.inner.write_str(s.as_ref()).unwrap();
    }

    fn newline(&mut self) {
        self.inner.write_char('\n').unwrap();
    }

    fn wln(&mut self, s: impl AsRef<str>) {
        self.w(s);
        self.newline();
    }
}

struct GenPos {
    position: Position,
    names: Vec<&'static str>,
}

impl GenPos {
    const fn new(position: Position, names: Vec<&'static str>) -> Self {
        Self { position, names }
    }
}

fn positions() -> Vec<GenPos> {
    vec![
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -8828.82,
                625.60803,
                93.944046,
                0.73748857,
            ),
            vec!["Stormwind", "SW"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 1492.9845, -4413.507, 23.130007, 0.19397898),
            vec!["Orgrimmar", "Org"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -4926.8213,
                -948.0789,
                501.5725,
                5.4404144,
            ),
            vec!["Ironforge", "IF"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 9951.911, 2280.2957, 1341.3949, 1.6373771),
            vec!["Darnassus", "Dar"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                1559.0254,
                225.00748,
                -43.102295,
                0.4707756,
            ),
            vec!["Undercity", "UC"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -9489.702,
                68.08857,
                56.033043,
                6.0672016,
            ),
            vec!["Goldshire"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -7192.246, -3804.6765, 9.804289, 0.0039429665),
            vec!["Gadgetzan"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -8201.845, -4622.949, 8.87625, 4.959796),
            vec!["Caverns Of Time"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -7087.052, -1275.4875, -182.79518, 3.4989605),
            vec!["Fire Plume Ridge"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -6189.1343, -1107.5111, -217.65813, 0.6126077),
            vec!["Marshals Refuge"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -6817.7075, 815.28516, 50.372143, 4.751644),
            vec!["Cenarion Hold"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -8183.4214, 1533.6359, 4.195668, 3.1140885),
            vec!["Ahn'Qiraj", "AQ"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 16226.2, 16257.0, 13.2022, 1.65007),
            vec!["GM Island"],
        ),
        GenPos::new(
            Position::new(
                Map::DevelopmentLand,
                16215.59,
                16112.857,
                69.44344,
                2.3562057,
            ),
            vec!["Programmer Isle"],
        ),
        GenPos::new(
            Position::new(
                Map::DevelopmentLand,
                16246.754,
                -16214.334,
                66.50971,
                0.9228294,
            ),
            vec!["Designer Island"],
        ),
        GenPos::new(
            Position::new(Map::EmeraldDream, -366.091, 3097.86, 92.317, 0.0487625),
            vec!["Emerald Dream"],
        ),
        GenPos::new(
            Position::new(Map::Testing, 0.0, 0.0, 0.0, 0.0),
            vec!["Testing"],
        ),
        GenPos::new(
            Position::new(Map::ScottTest, 0.0, 0.0, 10.0, 0.0),
            vec!["Scott Test"],
        ), // TODO: Scott Test Location
        GenPos::new(
            Position::new(Map::CollinsTest, 0.0, 0.0, 0.0, 0.0),
            vec!["Collins Test"],
        ),
        GenPos::new(
            Position::new(Map::Deadmines, -1689.895, 800.9268, 10.418647, 4.5317407),
            vec!["Deadmines Outland"],
        ),
        GenPos::new(
            Position::new(Map::CashTest, 2.0, 0.0, -144.70836, 0.0),
            vec!["Cash Test"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -10063.524, -5667.059, -503.43323, 3.0709772),
            vec!["Tanaris Underwater Goblin Buildings South"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -9643.565, -5616.614, -503.43362, 5.623549),
            vec!["Tanaris Underwater Goblin Buildings North"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                4264.9478,
                -2782.7117,
                6.037976,
                0.49006122,
            ),
            vec!["Quel'Thalas"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -4844.1763,
                -1066.8918,
                501.0099,
                4.5756893,
            ),
            vec!["Old Ironforge Entrance"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -14407.421,
                420.5257,
                22.361256,
                1.1756706,
            ),
            vec!["Booty Bay", "BB"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -13228.442,
                228.70625,
                32.968674,
                1.0681336,
            ),
            vec!["Gurubashi Arena", "Arena"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -12409.323,
                208.20271,
                31.609468,
                6.2635527,
            ),
            vec!["Grom'gol Base Camp"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -11916.2295,
                -1206.456,
                92.288,
                4.7202272,
            ),
            vec!["Zul Gurub", "ZG"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -11813.183,
                1238.1315,
                1.0812873,
                2.768512,
            ),
            vec!["Yojamba Isle"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -11328.041,
                -203.44608,
                75.36599,
                0.29839292,
            ),
            vec!["Rebel Camp"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -10556.513,
                -1174.8112,
                27.791433,
                1.7042117,
            ),
            vec!["Darkshire"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -10761.903,
                310.5415,
                37.33325,
                0.2080247,
            ),
            vec!["Raven Hill"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -10594.387,
                -435.3123,
                84.21301,
                6.216311,
            ),
            vec!["Twilight Grove"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -10516.406,
                1066.4332,
                55.06653,
                5.167803,
            ),
            vec!["Sentinell Hill"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -11207.697,
                1672.9551,
                24.685108,
                1.5549753,
            ),
            vec!["Deadmines", "DM", "VC"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -9472.111,
                -1297.8641,
                41.34214,
                2.0027518,
            ),
            vec!["Eastvale Logging Camp"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -11123.135,
                -2015.4913,
                47.092323,
                0.8128557,
            ),
            vec!["Karazhan", "KZ"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -10437.816,
                -3261.3662,
                20.178669,
                3.3339722,
            ),
            vec!["Stonard"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -10183.236,
                -3993.1228,
                -109.1943,
                6.031753,
            ),
            vec!["Sunken Temple", "ST"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -11087.073,
                -3478.6855,
                67.074974,
                0.4554863,
            ),
            vec!["Nethergarde Keep"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -11865.143,
                -3203.4097,
                -22.789457,
                3.2632866,
            ),
            vec!["The Dark Portal"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -12078.074,
                -2385.6267,
                63.266243,
                3.8798268,
            ),
            vec!["The Tainted Scar", "Kazzak"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -9270.091,
                -2188.616,
                64.08998,
                4.653347,
            ),
            vec!["Lakeshire"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -8346.261,
                -2745.6055,
                182.78365,
                3.5027378,
            ),
            vec!["Morgans Vigil"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -7499.5664,
                -1068.3705,
                264.53174,
                3.5577283,
            ),
            vec!["Blackrock Mountain", "BRM"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -7528.446,
                -1222.4823,
                285.73254,
                5.27382,
            ),
            vec!["Blackrock Spire", "BRS", "LBRS", "UBRS"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -7660.1064,
                -1220.2639,
                287.7881,
                2.6820083,
            ),
            vec!["Blackwing Lair", "BWL"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -7515.337,
                -1045.4636,
                182.30055,
                0.72636116,
            ),
            vec!["Molten Core", "MC"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -7181.1353,
                -917.22266,
                165.49086,
                5.0264363,
            ),
            vec!["Blackrock Depths", "BRD"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -6538.502,
                -1167.587,
                309.25507,
                3.7579732,
            ),
            vec!["Thorium Point"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -6693.3545,
                -2187.6528,
                247.0427,
                0.10584588,
            ),
            vec!["Kargath"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -6082.3706,
                -2956.2278,
                204.88348,
                0.06256648,
            ),
            vec!["Uldaman", "Ulda"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -5410.2583,
                -2919.538,
                344.12415,
                5.86672,
            ),
            vec!["Thelsamar"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -5631.007,
                -483.99567,
                396.9806,
                0.05651281,
            ),
            vec!["Kharanos"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -3777.9895,
                -788.3876,
                8.389523,
                2.1142633,
            ),
            vec!["Menethil Harbor"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -1261.2355,
                -2506.4968,
                20.966883,
                5.2715607,
            ),
            vec!["Refuge Point"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -1197.9244,
                -2532.7283,
                22.359148,
                0.017230026,
            ),
            vec!["Arathi Basin", "AB"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -920.31854,
                -3498.4885,
                70.450485,
                3.7242925,
            ),
            vec!["Hammerfall"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -31.854528,
                -924.46967,
                54.500553,
                4.4036865,
            ),
            vec!["Tarren Mill"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -842.6798,
                -535.1748,
                12.251574,
                2.7700403,
            ),
            vec!["Southshore"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                508.7585,
                1620.8267,
                125.41955,
                1.5762274,
            ),
            vec!["The Sepulcher"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -233.84456,
                1564.7162,
                76.89218,
                1.2620664,
            ),
            vec!["Shadowfang Keep", "SFK"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                103.14152,
                -187.1672,
                127.14359,
                1.9452847,
            ),
            vec!["Alterac Valley Alliance"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                534.28546,
                -1087.8927,
                106.073326,
                0.6611368,
            ),
            vec!["Alterac Valley Horde"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                288.13354,
                -2006.357,
                194.12657,
                4.0147424,
            ),
            vec!["Aerie Peak"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -559.7038,
                -4583.314,
                9.782037,
                3.790904,
            ),
            vec!["Revantusk Village"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                958.4925,
                -1436.145,
                63.178745,
                1.5369555,
            ),
            vec!["Chillwind Camp"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                1264.3054,
                -2558.559,
                94.12662,
                0.45311096,
            ),
            vec!["Scholomance", "Scholo"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                2282.6506,
                -5317.4546,
                88.576965,
                5.310723,
            ),
            vec!["Lights Hope Chapel"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                3390.9521,
                -3413.627,
                142.25818,
                1.513327,
            ),
            vec!["Stratholme Front", "Strat", "Strat Living"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                3230.8235,
                -4038.9807,
                108.42333,
                5.126111,
            ),
            vec!["Stratholme Back", "Strat Undead", "Strat UD"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                2902.7686,
                -1461.5986,
                147.03375,
                1.0577556,
            ),
            vec!["Hearthglenn"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                2892.1628,
                -810.6476,
                160.3318,
                0.37051573,
            ),
            vec!["Scarlet Monastary", "SM"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                2237.692,
                289.37708,
                35.18794,
                6.099991,
            ),
            vec!["Brill"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 9861.5205, 980.3109, 1307.9097, 3.9386046),
            vec!["Dolanaar"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 7964.5425, -2492.6584, 487.7729, 3.212889),
            vec!["Nighthaven"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 6415.2974, 485.25916, 8.199045, 6.177766),
            vec!["Auberdine"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 3989.673, -1293.3595, 251.34242, 6.1031523),
            vec!["Emerald Sanctuary"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 5097.3076, -331.16077, 363.33374, 6.189533),
            vec!["Bloodvenom Post"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 6149.3193, -1916.8485, 574.3046, 5.3923407),
            vec!["Talonbranch Glade"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 6744.6084, -4659.263, 725.5811, 3.1421802),
            vec!["Everlook"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 4946.141, -4542.272, 854.81915, 3.240363),
            vec!["Darkwhisper Gorge"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 2756.5515, -365.28177, 108.84804, 3.7705035),
            vec!["Astranaar"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 2312.1682, -2531.9675, 101.15512, 0.067322105),
            vec!["Splintertree Post"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 3361.5115, 1000.95807, 4.4325194, 2.502061),
            vec!["Zoram'gar Outpost"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 2730.967, -3872.025, 99.333305, 3.7076955),
            vec!["Talrendis Point"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 2312.8145, -6142.701, 151.15845, 0.7702508),
            vec!["Azuregos"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 2863.0378, -7292.9844, 16.63317, 3.2560158),
            vec!["Bay Of Storms"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 1812.6093, -4411.6543, -18.37125, 5.1930285),
            vec!["Ragefire Chasm", "RFC"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 2672.9688, 1466.9081, 231.6673, 0.34710538),
            vec!["Stonetalon Peak"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 932.5083, 902.80615, 104.371704, 2.581552),
            vec!["Sun Rock Retreat"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -473.69476, -2615.616, 105.73716, 5.892001),
            vec!["The Crossroads", "Xroads"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -730.1619, -2217.6897, 17.013126, 2.8250074),
            vec!["Wailing Caverns", "WC"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 325.06967, -4713.446, 13.531451, 4.4075813),
            vec!["Razor Hill"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -1265.7731, 116.62132, 130.5818, 5.204764),
            vec!["Thunder Bluff", "TB"],
        ),
        GenPos::new(
            Position::new(
                Map::Kalimdor,
                -2360.4736,
                -402.02548,
                -7.1913066,
                0.22925918,
            ),
            vec!["Bloodhoof Village"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 197.13333, 1283.307, 194.79616, 2.0395973),
            vec!["Nijels Point"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -1658.6687, 3096.4565, 30.497904, 1.8236027),
            vec!["Shadowprey Village"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -1459.9744, 2615.1187, 76.60868, 3.1705635),
            vec!["Maraudon Orange", "Mara"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -1185.922, 2880.7166, 85.89174, 5.0084076),
            vec!["Maraudon Purple"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -4417.863, 3345.6887, 27.765293, 5.907698),
            vec!["Feathermoon Stronghold"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -3520.455, 1085.5947, 161.04933, 4.733527),
            vec!["Dire Maul North", "DMN", "Dire Maul"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -3753.7266, 934.3449, 161.02307, 0.013270611),
            vec!["Dire Maul East", "DME"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -3823.5608, 1249.8356, 160.27025, 3.1195211),
            vec!["Dire Maul West", "DMW"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -4404.082, 211.47047, 25.776245, 1.057855),
            vec!["Camp Mojache"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -5442.1694, -2443.8982, 89.95757, 2.7700207),
            vec!["Freewind Post"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -4466.59, -1668.9615, 81.88813, 0.9204006),
            vec!["Razorfen Kraul", "RFK"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -4656.743, -2516.7097, 80.934616, 4.541121),
            vec!["Razorfen Downs", "RFD"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -2369.415, -1935.8875, 96.26798, 1.8864615),
            vec!["Camp Taurajo"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -884.9719, -3763.1104, 14.541181, 2.81715),
            vec!["Ratchet"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -3641.397, -4432.1587, 14.489666, 5.7074075),
            vec!["Theramore Isle"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -4732.9785, -3743.646, 55.336838, 3.6889355),
            vec!["Onyxias Lair", "Onyxia", "Ony"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -6183.548, -3953.7861, -58.749676, 3.1730258),
            vec!["Mirage Raceway"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -6808.5454, -2890.8203, 8.886945, 6.275331),
            vec!["Zul'Farrak", "ZF"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -9623.2, -2784.508, 7.838657, 3.2044282),
            vec!["Uldum"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -8502.676, -4512.1855, -211.89098, 1.4804432),
            vec!["Inside Caverns Of Time"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -6927.2515, -4799.978, 10.295544, 5.442787),
            vec!["Steamwheedle Port"],
        ),
        GenPos::new(
            Position::new(Map::DeeprunTram, 70.22373, 10.290868, -4.2973423, 3.0433981),
            vec!["Deeprun Tram Ironforge"],
        ),
        GenPos::new(
            Position::new(Map::DeeprunTram, 71.059746, 2491.1597, -4.296402, 3.129787),
            vec!["Deeprun Tram Stormwind"],
        ),
        GenPos::new(
            Position::new(Map::EasternKingdoms, -129.094, 835.621, 63.598, 0.0),
            vec!["Ambermill"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -5163.2563,
                927.2142,
                257.18063,
                1.4137135,
            ),
            vec!["Gnomeregan", "Gnomer"],
        ),
        GenPos::new(
            Position::new(Map::EasternKingdoms, -7501.51, -2183.08, 165.926, 0.0),
            vec!["Flame Crest"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -8771.881,
                840.1909,
                90.941216,
                0.6872213,
            ),
            vec!["The Stockades", "Stock", "Stocks"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                24.125174,
                -1579.8298,
                192.65678,
                4.543523,
            ),
            vec!["Ravenholdt Manor"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                3117.0757,
                -3723.7307,
                136.43544,
                5.8237267,
            ),
            vec!["Naxxramas", "Naxx"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                344.40958,
                204.78403,
                42.877777,
                1.8928034,
            ),
            vec!["Dalaran"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 4249.99, 740.102, -25.671, 1.3469582),
            vec!["Blackfathom Deeps", "BFD"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -11826.205, -4757.019, 6.5862513, 0.30235526),
            vec!["South Seas Islands"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 8755.133, 954.65765, 26.561329, 3.326159),
            vec!["Darnassus Below"],
        ),
        GenPos::new(
            HUMAN_START_POSITION,
            vec!["Human Start Zone", "Human Start"],
        ),
        GenPos::new(
            DWARF_START_POSITION,
            vec!["Dwarf Start Zone", "Dwarf Start"],
        ),
        GenPos::new(
            GNOME_START_POSITION,
            vec!["Gnome Start Zone", "Gnome Start"],
        ),
        GenPos::new(
            NIGHT_ELF_START_POSITION,
            vec!["Night Elf Start Zone", "Night Elf Start"],
        ),
        GenPos::new(ORC_START_POSITION, vec!["Orc Start Zone", "Orc Start"]),
        GenPos::new(
            TROLL_START_POSITION,
            vec!["Troll Start Zone", "Troll Start"],
        ),
        GenPos::new(
            TAUREN_START_POSITION,
            vec!["Tauren Start Zone", "Tauren Start"],
        ),
        GenPos::new(
            UNDEAD_START_POSITION,
            vec!["Undead Start Zone", "Undead Start"],
        ),
        GenPos::new(
            Position::new(
                Map::AzsharaCrater,
                125.96644,
                125.43644,
                232.53398,
                4.7084036,
            ),
            vec!["Azshara Crater"],
        ),
        GenPos::new(
            Position::new(Map::Monastery, 45.10906, -0.5430225, 15.210853, 0.031333923),
            vec!["Old Scarlet Monastery"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 7434.314, -1559.79, 187.37662, 2.737099),
            vec!["Dancing Troll Village"],
        ),
        GenPos::new(
            Position::new(Map::CavernsOfTime, 2394.23, 1195.9457, 68.229645, 4.0949802),
            vec!["Old Hillsbrad"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 4617.013, -3853.4287, 944.0778, 1.1349027),
            vec!["Hyjal Start"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 4617.013, -3853.4287, 944.0778, 1.1349027),
            vec!["Hyjal"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 5380.2334, -3369.9954, 1655.6182, 4.9715676),
            vec!["Hyjal End"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 5475.9175, -3728.8867, 1593.4441, 5.957213),
            vec!["Hyjal Construction"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 4817.0, -1742.0, 1158.2507, 5.5252714),
            vec!["Hyjal Portal Outside"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 4858.0, -1840.0, 1143.6442, 4.9244504),
            vec!["Hyjal Portal Inside"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -10736.898, 2472.5898, 6.3625216, 4.700596),
            vec!["Silithus Farm"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, -399.51453, -2184.7974, 157.68736, 0.28666997),
            vec!["Shrine Of The Fallen"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 1951.5121, 1530.4756, 247.28815, 0.62046385),
            vec!["Stonetalon Logging Camp"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 9725.27, -21.43, 18.797596, 0.0),
            vec!["Teldrassil Furbolg Camp"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 520.3782, -237.52525, 582.3156, 5.324994),
            vec!["Stonetalon Help Mountain"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -3853.3806,
                -3463.5928,
                582.74634,
                4.7555747,
            ),
            vec!["Wetlands Help Mountain"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -4796.1597,
                -1665.3699,
                504.1609,
                6.130035,
            ),
            vec!["Ironforge Airport"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -3981.335,
                -1279.616,
                158.16386,
                3.593201,
            ),
            vec!["Wetlands Farm"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -5308.1855,
                -2510.7708,
                485.1017,
                3.5892704,
            ),
            vec!["Ortell's Hideout"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -12878.046,
                -1411.4612,
                120.08431,
                0.5340776,
            ),
            vec!["Stranglethorn Secret Cave"],
        ),
        GenPos::new(
            Position::new(Map::EasternKingdoms, -11073.0, -1956.0, 38.753963, 0.0),
            vec!["Karazhan Smiley"],
        ),
        GenPos::new(
            Position::new(Map::EasternKingdoms, -8324.0, -339.0, 145.63689, 0.0),
            vec!["Stormwind Falls"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -11084.0,
                -1801.0,
                52.741642,
                1.8456863,
            ),
            vec!["Well Of The Forgotten"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -1832.7662,
                -4163.301,
                9.870084,
                5.815872,
            ),
            vec!["Arathi Farm"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -6376.278,
                1259.4205,
                7.187727,
                1.2330625,
            ),
            vec!["Newmann's Landing"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                1698.6284,
                239.52962,
                -40.68876,
                0.015705844,
            ),
            vec!["Undercity Top Tier"],
        ),
        GenPos::new(
            Position::new(Map::EasternKingdoms, -8521.3, 599.5, 101.39934, 0.0),
            vec!["Cut Throat Alley"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -6174.1846,
                -772.7827,
                420.7437,
                6.181049,
            ),
            vec!["Dun Morogh Plane Camp"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -4601.6494,
                -817.7569,
                639.3065,
                3.8523853,
            ),
            vec!["Dun Morogh Troll Cave"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -5938.429,
                451.62952,
                508.29916,
                1.8967295,
            ),
            vec!["Forgotten Gnome Camp"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -822.21686,
                1554.9584,
                22.552053,
                6.0436397,
            ),
            vec!["The Greymane Wall"],
        ),
        GenPos::new(
            Position::new(
                Map::EasternKingdoms,
                -5605.3657,
                731.79034,
                486.01495,
                1.4851742,
            ),
            vec!["Isolated Dun Morogh"],
        ),
        GenPos::new(
            Position::new(Map::EasternKingdoms, -7347.0, -642.0, 294.58627, 5.9454575),
            vec!["Blackchar Cave"],
        ),
        GenPos::new(
            Position::new(Map::Kalimdor, 16227.0, 16403.0, -64.379105, 0.0),
            vec!["GM Island Room"],
        ),
        GenPos::new(
            Position::new(Map::EmeraldDream, -2128.12, -1005.89, 132.213, 0.0),
            vec!["The Verdant Fields"],
        ),
        GenPos::new(
            Position::new(
                Map::EmeraldDream,
                2738.6653,
                -3272.0889,
                101.14731,
                1.1898761,
            ),
            vec!["Emerald Forest"],
        ),
        GenPos::new(
            Position::new(
                Map::EmeraldDream,
                2838.2842,
                2995.9792,
                27.672451,
                1.7396579,
            ),
            vec!["Emerald Statue"],
        ),
        GenPos::new(
            Position::new(
                Map::CavernsOfTime,
                -1997.7993,
                6572.519,
                -154.93341,
                5.9925404,
            ),
            vec!["The Black Morass Start"],
        ),
        GenPos::new(
            Position::new(
                Map::CavernsOfTime,
                -1646.0367,
                7117.7476,
                25.474968,
                2.9413476,
            ),
            vec!["The Black Morass Swamp"],
        ),
        GenPos::new(
            Position::new(
                Map::CavernsOfTime,
                -2063.8213,
                7122.72,
                30.269022,
                3.0513024,
            ),
            vec!["The Black Morass Dark Portal"],
        ),
        GenPos::new(
            Position::new(
                Map::StormwindPrison,
                -1.8775121,
                52.88342,
                -27.550669,
                1.5236611,
            ),
            vec!["Stormwind Vault"],
        ),
        GenPos::new(
            Position::new(Map::DeeprunTram, 3.297, 1382.282, -100.303, 6.275331),
            vec!["Deeprun Tram Aquarium"],
        ),
    ]
}
