use crate::data::Data;
use crate::file_utils::overwrite_autogenerate_if_not_the_same;
use crate::writer::Writer;
use crate::Expansion;
use std::path::Path;

pub(crate) fn write_exp(directory: &Path, data: &Data) {
    let mut s = Writer::new();

    s.wln(format!(
        "const EXP_REQUIRED_FOR_LEVEL: [i32; {}] = [",
        data.exp_per_level.len()
    ));
    s.inc_indent();
    for x in &data.exp_per_level {
        s.wln(format!(
            "{}, // Required to ding from level {}",
            x.exp, x.level
        ));
    }
    s.dec_indent();
    s.wln("];");

    let path = directory.join("exp.rs");
    overwrite_autogenerate_if_not_the_same(&path, s.inner());
}

pub(crate) fn write_stats(directory: &Path, data: &Data) {
    let mut s = Writer::new();

    for combination in &data.combinations {
        let race = combination.0;
        let class = combination.1;
        let levels = data.base_stats.get(combination).unwrap();

        s.wln(format!(
            "const {race}_{class}: &[BaseStats] = &[",
            race = race.const_name(),
            class = class.const_name(),
        ));
        s.inc_indent();

        let mut counter = 1;
        for level in levels {
            assert_eq!(*level.0, counter);
            counter += 1;

            let stats = level.1;

            s.wln(format!("BaseStats::new({strength}, {agility}, {stamina}, {intellect}, {spirit}, {health}, {mana}),", strength = stats.strength, agility = stats.agility, stamina = stats.stamina, intellect = stats.intellect, spirit = stats.spirit, health = stats.health, mana = stats.mana));
        }

        s.dec_indent();
        s.wln("];");
    }

    let path = directory.join("base_stats.rs");
    overwrite_autogenerate_if_not_the_same(&path, s.inner());
}

pub(crate) fn write_skills(directory: &Path, data: &Data) {
    let mut s = Writer::new();

    for combination in &data.combinations {
        let race = combination.0;
        let class = combination.1;
        let skills = data.skills.get(combination).unwrap();

        s.wln(format!(
            "const {race}_{class}: &[u32] = &[",
            race = race.const_name(),
            class = class.const_name(),
        ));
        s.inc_indent();

        for skill in skills {
            s.wln(format!("{}, // {}", skill.0, skill.1));
        }

        s.dec_indent();
        s.wln("];");
    }

    let path = directory.join("skills.rs");
    overwrite_autogenerate_if_not_the_same(&path, s.inner());
}

pub(crate) fn write_spells(directory: &Path, data: &Data) {
    let mut s = Writer::new();

    for combination in &data.combinations {
        let race = combination.0;
        let class = combination.1;
        let spells = data.spells.get(combination).unwrap();

        s.wln(format!(
            "const {race}_{class}: &[u32] = &[",
            race = race.const_name(),
            class = class.const_name(),
        ));
        s.inc_indent();

        for spell in spells {
            s.wln(format!("{}, // {}", spell.0, spell.1));
        }

        s.dec_indent();
        s.wln("];");
    }

    let path = directory.join("spells.rs");
    overwrite_autogenerate_if_not_the_same(&path, s.inner());
}

fn get_string_name(s: &str) -> String {
    s.replace('\'', "").replace(' ', "_").to_lowercase()
}

fn get_enum_name(s: &str) -> String {
    s.replace('\'', "").replace(' ', "")
}

pub(crate) fn write_positions(directory: &Path, data: &Data, expansion: Expansion) {
    let mut s = Writer::new();

    s.wln("pub fn get_position_from_str(name: &str) -> Option<Position> {");
    s.wln("    let i = match name {");

    for p in data.positions(expansion) {
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

    for (i, e) in data.positions(expansion).enumerate() {
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

    for i in data.positions(expansion) {
        s.wln(format!("    {},", get_enum_name(i.names[0])));
    }

    s.wln("}");

    s.wln("const POSITIONS: &[Position] = &[");

    for i in data.positions(expansion) {
        let map = match expansion {
            Expansion::Vanilla => format!(
                "{:?}",
                wow_world_base::vanilla::Map::try_from(i.map).unwrap()
            ),
            Expansion::BurningCrusade => {
                format!("{:?}", wow_world_base::tbc::Map::try_from(i.map).unwrap())
            }
            Expansion::WrathOfTheLichKing => {
                format!("{:?}", wow_world_base::wrath::Map::try_from(i.map).unwrap())
            }
        };

        s.wln(format!(
            "    Position::new(Map::{}, {:.1}, {:.1}, {:.1}, {:.1}),",
            map, i.x, i.y, i.z, i.orientation,
        ));
    }

    s.wln("];");

    let path = directory.join("position").join("positions.rs");
    overwrite_autogenerate_if_not_the_same(&path, s.inner());
}
