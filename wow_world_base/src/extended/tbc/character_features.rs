use crate::shared::player_race_tbc_wrath::PlayerRace;
use crate::tbc::PlayerGender;

pub const fn character_features_are_valid(
    race: PlayerRace,
    gender: PlayerGender,
    skin: u8,
    facial_hair: u8,
    face: u8,
    hair_color: u8,
    hair_style: u8,
) -> bool {
    let max_skin_color = match (race, gender) {
        (PlayerRace::Human, PlayerGender::Male) => 9,
        (PlayerRace::Human, PlayerGender::Female) => 9,
        (PlayerRace::Orc, PlayerGender::Male) => 8,
        (PlayerRace::Orc, PlayerGender::Female) => 8,
        (PlayerRace::Dwarf, PlayerGender::Male) => 9,
        (PlayerRace::Dwarf, PlayerGender::Female) => 8,
        (PlayerRace::NightElf, PlayerGender::Male) => 8,
        (PlayerRace::NightElf, PlayerGender::Female) => 8,
        (PlayerRace::Undead, PlayerGender::Male) => 5,
        (PlayerRace::Undead, PlayerGender::Female) => 5,
        (PlayerRace::Tauren, PlayerGender::Male) => 18,
        (PlayerRace::Tauren, PlayerGender::Female) => 10,
        (PlayerRace::Gnome, PlayerGender::Male) => 4,
        (PlayerRace::Gnome, PlayerGender::Female) => 4,
        (PlayerRace::Troll, PlayerGender::Male) => 8,
        (PlayerRace::Troll, PlayerGender::Female) => 8,
        (PlayerRace::BloodElf, PlayerGender::Male) => 9,
        (PlayerRace::BloodElf, PlayerGender::Female) => 9,
        (PlayerRace::Draenei, PlayerGender::Male) => 13,
        (PlayerRace::Draenei, PlayerGender::Female) => 11,
    };

    let max_facial_hair = match (race, gender) {
        (PlayerRace::Human, PlayerGender::Male) => 8,
        (PlayerRace::Human, PlayerGender::Female) => 6,
        (PlayerRace::Orc, PlayerGender::Male) => 10,
        (PlayerRace::Orc, PlayerGender::Female) => 6,
        (PlayerRace::Dwarf, PlayerGender::Male) => 10,
        (PlayerRace::Dwarf, PlayerGender::Female) => 5,
        (PlayerRace::NightElf, PlayerGender::Male) => 5,
        (PlayerRace::NightElf, PlayerGender::Female) => 9,
        (PlayerRace::Undead, PlayerGender::Male) => 16,
        (PlayerRace::Undead, PlayerGender::Female) => 7,
        (PlayerRace::Tauren, PlayerGender::Male) => 6,
        (PlayerRace::Tauren, PlayerGender::Female) => 4,
        (PlayerRace::Gnome, PlayerGender::Male) => 7,
        (PlayerRace::Gnome, PlayerGender::Female) => 6,
        (PlayerRace::Troll, PlayerGender::Male) => 10,
        (PlayerRace::Troll, PlayerGender::Female) => 5,
        (PlayerRace::BloodElf, PlayerGender::Male) => 9,
        (PlayerRace::BloodElf, PlayerGender::Female) => 10,
        (PlayerRace::Draenei, PlayerGender::Male) => 7,
        (PlayerRace::Draenei, PlayerGender::Female) => 6,
    };

    let (max_face, max_face_color) = match (race, gender) {
        (PlayerRace::Human, PlayerGender::Male) => (11, 9),
        (PlayerRace::Human, PlayerGender::Female) => (14, 9),
        (PlayerRace::Orc, PlayerGender::Male) => (8, 8),
        (PlayerRace::Orc, PlayerGender::Female) => (8, 8),
        (PlayerRace::Dwarf, PlayerGender::Male) => (9, 8),
        (PlayerRace::Dwarf, PlayerGender::Female) => (9, 8),
        (PlayerRace::NightElf, PlayerGender::Male) => (8, 8),
        (PlayerRace::NightElf, PlayerGender::Female) => (8, 8),
        (PlayerRace::Undead, PlayerGender::Male) => (9, 5),
        (PlayerRace::Undead, PlayerGender::Female) => (9, 5),
        (PlayerRace::Tauren, PlayerGender::Male) => (4, 18),
        (PlayerRace::Tauren, PlayerGender::Female) => (3, 10),
        (PlayerRace::Gnome, PlayerGender::Male) => (6, 4),
        (PlayerRace::Gnome, PlayerGender::Female) => (6, 4),
        (PlayerRace::Troll, PlayerGender::Male) => (4, 5),
        (PlayerRace::Troll, PlayerGender::Female) => (5, 5),
        (PlayerRace::BloodElf, PlayerGender::Male) => (9, 9),
        (PlayerRace::BloodElf, PlayerGender::Female) => (9, 9),
        (PlayerRace::Draenei, PlayerGender::Male) => (9, 13),
        (PlayerRace::Draenei, PlayerGender::Female) => (9, 11),
    };

    let (max_hair, max_hair_color) = match (race, gender) {
        (PlayerRace::Human, PlayerGender::Male) => (11, 9),
        (PlayerRace::Human, PlayerGender::Female) => (18, 9),
        (PlayerRace::Orc, PlayerGender::Male) => (6, 7),
        (PlayerRace::Orc, PlayerGender::Female) => (7, 7),
        (PlayerRace::Dwarf, PlayerGender::Male) => (10, 9),
        (PlayerRace::Dwarf, PlayerGender::Female) => (13, 9),
        (PlayerRace::NightElf, PlayerGender::Male) => (6, 7),
        (PlayerRace::NightElf, PlayerGender::Female) => (6, 7),
        (PlayerRace::Undead, PlayerGender::Male) => (9, 9),
        (PlayerRace::Undead, PlayerGender::Female) => (9, 9),
        (PlayerRace::Tauren, PlayerGender::Male) => (7, 2),
        (PlayerRace::Tauren, PlayerGender::Female) => (6, 2),
        (PlayerRace::Gnome, PlayerGender::Male) => (6, 8),
        (PlayerRace::Gnome, PlayerGender::Female) => (6, 8),
        (PlayerRace::Troll, PlayerGender::Male) => (5, 9),
        (PlayerRace::Troll, PlayerGender::Female) => (4, 9),
        (PlayerRace::BloodElf, PlayerGender::Male) => (10, 9),
        (PlayerRace::BloodElf, PlayerGender::Female) => (13, 9),
        (PlayerRace::Draenei, PlayerGender::Male) => (8, 6),
        (PlayerRace::Draenei, PlayerGender::Female) => (10, 6),
    };

    skin <= max_skin_color
        && facial_hair <= max_facial_hair
        && hair_style <= max_hair
        && hair_color <= max_hair_color
        && face <= max_face
        && skin <= max_face_color
}
