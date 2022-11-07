use crate::shared::race_scale;
use wow_world_base::vanilla::PlayerGender;
use wow_world_base::vanilla::PlayerRace;

race_scale!();

pub fn character_features_are_valid(
    race: PlayerRace,
    gender: PlayerGender,
    skin: u8,
    facial_hair: u8,
    face: u8,
    hair_color: u8,
    hair_style: u8,
) -> bool {
    // Valid skin colors are all between 0 and a race/gender specific number
    let max_skin_color = match race {
        PlayerRace::Human => 9,
        PlayerRace::Orc | PlayerRace::Dwarf | PlayerRace::NightElf => 8,
        PlayerRace::Tauren => match gender {
            PlayerGender::Male => 18,
            PlayerGender::Female => 10,
        },
        PlayerRace::Gnome => 4,
        PlayerRace::Undead | PlayerRace::Troll => 5,
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
    };

    let max_face = match (race, gender) {
        (PlayerRace::Dwarf, PlayerGender::Male | PlayerGender::Female) => 9,
        (PlayerRace::Gnome, PlayerGender::Male | PlayerGender::Female) => 6,
        (PlayerRace::Human, PlayerGender::Male) => 11,
        (PlayerRace::Human, PlayerGender::Female) => 14,
        (PlayerRace::NightElf, PlayerGender::Male | PlayerGender::Female) => 8,
        (PlayerRace::Orc, PlayerGender::Male | PlayerGender::Female) => 8,
        (PlayerRace::Tauren, PlayerGender::Male) => 4,
        (PlayerRace::Tauren, PlayerGender::Female) => 3,
        (PlayerRace::Troll, PlayerGender::Male) => 4,
        (PlayerRace::Troll, PlayerGender::Female) => 5,
        (PlayerRace::Undead, PlayerGender::Male | PlayerGender::Female) => 9,
    };

    let max_hairstyle = match (race, gender) {
        (PlayerRace::Dwarf, PlayerGender::Male) => 10,
        (PlayerRace::Dwarf, PlayerGender::Female) => 13,
        (PlayerRace::Gnome, PlayerGender::Male | PlayerGender::Female) => 6,
        (PlayerRace::Human, PlayerGender::Male) => 11,
        (PlayerRace::Human, PlayerGender::Female) => 18,
        (PlayerRace::NightElf, PlayerGender::Male | PlayerGender::Female) => 6,
        (PlayerRace::Orc, PlayerGender::Male) => 6,
        (PlayerRace::Orc, PlayerGender::Female) => 7,
        (PlayerRace::Undead, PlayerGender::Male | PlayerGender::Female) => 9,
        (PlayerRace::Tauren, PlayerGender::Male) => 7,
        (PlayerRace::Tauren, PlayerGender::Female) => 6,
        (PlayerRace::Troll, PlayerGender::Male) => 5,
        (PlayerRace::Troll, PlayerGender::Female) => 4,
    };

    let max_haircolor = match race {
        PlayerRace::Dwarf => 9,
        PlayerRace::Gnome => 8,
        PlayerRace::Human => 9,
        PlayerRace::NightElf => 7,
        PlayerRace::Orc => 7,
        PlayerRace::Undead => 9,
        PlayerRace::Tauren => 2,
        PlayerRace::Troll => 9,
    };

    skin <= max_skin_color
        && facial_hair <= max_facial_hair
        && face <= max_face
        && hair_color <= max_haircolor
        && hair_style <= max_hairstyle
}
