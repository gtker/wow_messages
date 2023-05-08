use crate::shared::player_race_tbc_wrath::PlayerRace;
use crate::wrath::PlayerGender;

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
        (PlayerRace::Human, _) => 14,
        (PlayerRace::Orc, PlayerGender::Male) => 17,
        (PlayerRace::Orc, PlayerGender::Female) => 13,
        (PlayerRace::Dwarf, PlayerGender::Male) => 21,
        (PlayerRace::Dwarf, PlayerGender::Female) => 13,
        (PlayerRace::NightElf, _) => 11,
        (PlayerRace::Undead, _) => 5,
        (PlayerRace::Tauren, PlayerGender::Male) => 24,
        (PlayerRace::Tauren, PlayerGender::Female) => 16,
        (PlayerRace::Gnome, _) => 9,
        (PlayerRace::Troll, _) => 17,
        (PlayerRace::BloodElf, _) => 18,
        (PlayerRace::Draenei, PlayerGender::Male) => 16,
        (PlayerRace::Draenei, PlayerGender::Female) => 14,
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

    let face_valid = match (race, gender) {
        (PlayerRace::Human, PlayerGender::Male) => match face {
            0 | 2 | 11 => less_and_between(skin, 9, 12, 14),
            1..=23 => skin <= 9,
            _ => false,
        },
        (PlayerRace::Human, PlayerGender::Female) => match face {
            0 | 1 | 10 => less_and_between(skin, 9, 12, 14),
            2..=29 => skin <= 9,
            _ => false,
        },
        (PlayerRace::Orc, PlayerGender::Male) => match face {
            0 | 1 | 5 => less_and_between(skin, 8, 15, 17),
            2..=17 => skin <= 8,
            _ => false,
        },
        (PlayerRace::Orc, PlayerGender::Female) => match face {
            0 | 7 | 8 => less_and_between(skin, 8, 11, 13),
            1..=17 => skin <= 8,
            _ => false,
        },
        (PlayerRace::Dwarf, PlayerGender::Male) => match face {
            0 | 1 | 9 => less_and_between(skin, 8, 19, 21),
            2..=19 => skin <= 8,
            _ => false,
        },
        (PlayerRace::Dwarf, PlayerGender::Female) => match face {
            0 | 5 | 6 => less_and_between(skin, 8, 11, 13),
            1..=19 => skin <= 8,
            _ => false,
        },
        (PlayerRace::NightElf, PlayerGender::Male) => match face {
            0 | 1 | 4 => skin <= 11,
            2..=17 => skin <= 8,
            _ => false,
        },
        (PlayerRace::NightElf, PlayerGender::Female) => match face {
            0 | 6 | 7 => skin <= 11,
            1..=17 => skin <= 8,
            _ => false,
        },
        (PlayerRace::Undead, _) => match face {
            0..=19 => skin <= 5,
            _ => false,
        },
        (PlayerRace::Tauren, PlayerGender::Male) => match face {
            0 | 2 | 4 => skin <= 21,
            1..=9 => skin <= 18,
            _ => false,
        },
        (PlayerRace::Tauren, PlayerGender::Female) => match face {
            0 | 2 | 3 => skin <= 13,
            1..=7 => skin <= 10,
            _ => false,
        },
        (PlayerRace::Gnome, PlayerGender::Male) => match face {
            0 | 2 | 3 => less_and_between(skin, 4, 7, 9),
            1..=13 => skin <= 4,
            _ => false,
        },
        (PlayerRace::Gnome, PlayerGender::Female) => match face {
            0 | 3 | 5 => less_and_between(skin, 4, 7, 9),
            1..=13 => skin <= 4,
            _ => false,
        },
        (PlayerRace::Troll, PlayerGender::Male) => match face {
            0 | 1 | 3 => less_and_between(skin, 5, 15, 17),
            2..=9 => skin <= 5,
            _ => false,
        },
        (PlayerRace::Troll, PlayerGender::Female) => match face {
            0 | 3 | 5 => less_and_between(skin, 5, 15, 17),
            1..=11 => skin <= 5,
            _ => false,
        },
        (PlayerRace::BloodElf, PlayerGender::Male) => match face {
            2 | 4 | 7 => less_and_between(skin, 9, 16, 18),
            0..=19 => skin <= 9,
            _ => false,
        },
        (PlayerRace::BloodElf, PlayerGender::Female) => match face {
            0 | 3 | 8 => less_and_between(skin, 9, 16, 18),
            1..=19 => skin <= 9,
            _ => false,
        },
        (PlayerRace::Draenei, PlayerGender::Male) => match face {
            1 | 4 | 9 => skin <= 16,
            0..=19 => skin <= 13,
            _ => false,
        },
        (PlayerRace::Draenei, PlayerGender::Female) => match face {
            0 | 4 | 8 => skin <= 14,
            1..=19 => skin <= 11,
            _ => false,
        },
    };

    let hair_style_valid = match (race, gender) {
        (PlayerRace::Human, PlayerGender::Male) => match hair_style {
            0..=16 => hair_color <= 12,
            _ => false,
        },
        (PlayerRace::Human, PlayerGender::Female) => match hair_style {
            0..=23 => hair_color <= 12,
            _ => false,
        },
        (PlayerRace::Orc, PlayerGender::Male) => match hair_style {
            0..=11 => hair_color <= 10,
            _ => false,
        },
        (PlayerRace::Orc, PlayerGender::Female) => match hair_style {
            0..=12 => hair_color <= 10,
            _ => false,
        },
        (PlayerRace::Dwarf, PlayerGender::Male) => match hair_style {
            0..=15 => hair_color <= 12,
            _ => false,
        },
        (PlayerRace::Dwarf, PlayerGender::Female) => match hair_style {
            0..=18 => hair_color <= 12,
            _ => false,
        },
        (PlayerRace::NightElf, _) => match hair_style {
            0..=11 => less_and_between(hair_color, 7, 10, 12),
            _ => false,
        },
        (PlayerRace::Undead, _) => match hair_style {
            0..=14 => hair_color <= 9,
            _ => false,
        },
        (PlayerRace::Tauren, PlayerGender::Male) => match hair_style {
            0..=12 => hair_color <= 3,
            _ => false,
        },
        (PlayerRace::Tauren, PlayerGender::Female) => match hair_style {
            0..=11 => hair_color <= 3,
            _ => false,
        },
        (PlayerRace::Gnome, _) => match hair_style {
            0..=11 => hair_color <= 11,
            _ => false,
        },
        (PlayerRace::Troll, _) => match hair_style {
            0..=9 => hair_color <= 12,
            _ => false,
        },
        (PlayerRace::BloodElf, PlayerGender::Male) => match hair_style {
            0..=15 => hair_color <= 12,
            _ => false,
        },
        (PlayerRace::BloodElf, PlayerGender::Female) => match hair_style {
            0..=18 => hair_color <= 12,
            _ => false,
        },
        (PlayerRace::Draenei, PlayerGender::Male) => match hair_style {
            0..=13 => hair_color <= 9,
            _ => false,
        },
        (PlayerRace::Draenei, PlayerGender::Female) => match hair_style {
            0..=15 => hair_color <= 9,
            _ => false,
        },
    };

    skin <= max_skin_color && facial_hair <= max_facial_hair && face_valid && hair_style_valid
}

const fn between(v: u8, lower: u8, upper: u8) -> bool {
    v >= lower && v <= upper
}

const fn less_and_between(v: u8, less_than: u8, lower: u8, upper: u8) -> bool {
    v <= less_than || between(v, lower, upper)
}
