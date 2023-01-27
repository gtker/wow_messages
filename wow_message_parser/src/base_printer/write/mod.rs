pub mod items;

use super::data::Data;
use super::writer::Writer;
use super::Expansion;
use crate::base_printer::data::area_triggers::{
    AreaTrigger, TBC_AREA_TRIGGERS, VANILLA_AREA_TRIGGERS, WRATH_AREA_TRIGGERS,
};
use crate::base_printer::data::pet_names::Pet;
use crate::base_printer::data::Trigger;
use crate::file_utils::overwrite_autogenerate_if_not_the_same;
use std::path::Path;

pub(crate) fn write_exp(directory: &Path, data: &Data) {
    let mut s = Writer::new();

    s.wln("const EXP_REQUIRED_FOR_LEVEL: &[i32] = &[");
    s.inc_indent();
    for x in &data.exp_per_level {
        s.wln(format!(
            "{}, // Required to ding from level {}",
            x.exp, x.level
        ));
    }
    s.dec_indent();
    s.wln("];");

    s.newline();

    s.wln("const EXPLORATION_EXP_PER_LEVEL: &[i32] = &[");
    s.inc_indent();

    for x in &data.exploration_exp_per_level {
        s.wln(format!(
            "{}, // Exploration exp for level {}",
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
        let levels = data.base_stats.get(combination).unwrap();

        s.wln(format!(
            "const {race}_{class}: &[BaseStats] = &[",
            race = combination.race.const_name(),
            class = combination.class.const_name(),
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

pub(crate) fn write_skills(directory: &Path, data: &Data, expansion: Expansion) {
    let mut s = Writer::new();

    for combination in &data.combinations {
        let skills = data.skills.get(combination).unwrap();

        s.wln(format!(
            "const {race}_{class}: &[Skill] = &[",
            race = combination.race.const_name(),
            class = combination.class.const_name(),
        ));
        s.inc_indent();

        for skill in skills {
            let comment = skill.1.clone();
            let skill = match expansion {
                Expansion::Vanilla => format!(
                    "{:?}",
                    wow_world_base::vanilla::Skill::try_from(skill.0 as u16).unwrap()
                ),
                Expansion::BurningCrusade => format!(
                    "{:?}",
                    wow_world_base::wrath::Skill::try_from(skill.0 as u16).unwrap()
                ),
                Expansion::WrathOfTheLichKing => format!(
                    "{:?}",
                    wow_world_base::wrath::Skill::try_from(skill.0 as u16).unwrap()
                ),
            };

            s.wln(format!("Skill::{skill}, // {comment}"));
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
        let spells = data.spells.get(combination).unwrap();

        s.wln(format!(
            "const {race}_{class}: &[u32] = &[",
            race = combination.race.const_name(),
            class = combination.class.const_name(),
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
    s.replace(['\'', ':'], "").replace(' ', "_").to_lowercase()
}

fn get_enum_name(s: &str) -> String {
    s.replace(['\'', ' ', ':', '-', '.'], "")
}

pub(crate) fn write_positions(directory: &Path, data: &Data, expansion: Expansion) {
    let mut s = Writer::new();

    s.wln("pub fn position_from_str(name: &str) -> Option<Position> {");
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

    s.wln("    Some(position(i))");
    s.wln("}");
    s.newline();

    s.wln("pub const fn position(ident: PositionIdentifier) -> Position {");
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
        let map = expansion.as_map_string(i.map).unwrap();

        s.wln(format!(
            "    Position::new({}, {:.1}, {:.1}, {:.1}, {:.1}),",
            map, i.x, i.y, i.z, i.orientation,
        ));
    }

    s.wln("];");

    let path = directory.join("position").join("positions.rs");
    overwrite_autogenerate_if_not_the_same(&path, s.inner());
}

pub(crate) fn write_actions(directory: &Path, data: &Data) {
    let mut s = Writer::new();

    for combination in &data.combinations {
        let actions = data.actions.get(combination).unwrap();

        s.wln(format!(
            "const {race}_{class}: &[Action] = &[",
            race = combination.race.const_name(),
            class = combination.class.const_name(),
        ));
        s.inc_indent();

        for action in actions {
            s.wln(format!(
                "Action::new({}, {}, {}),",
                action.button, action.action, action.ty
            ));
        }

        s.dec_indent();
        s.wln("];");
    }

    let path = directory.join("actions.rs");
    overwrite_autogenerate_if_not_the_same(&path, s.inner());
}

pub(crate) fn write_pet_names(directory: &Path, data: &Data, expansion: Expansion) {
    let mut s = Writer::new();

    for pet in &data.pet_names {
        let name = match pet.0 {
            Pet::Imp => "IMP",
            Pet::Felhunter => "FELHUNTER",
            Pet::Voidwalker => "VOIDWALKER",
            Pet::Succubus => "SUCCUBUS",
            Pet::Felguard => {
                if expansion == Expansion::Vanilla {
                    continue;
                }
                "FELGUARD"
            }
            Pet::RisenGhoul => "GHOUL",
        };
        s.wln(format!("const {name}_NAMES: &[&str] = &["));
        s.inc_indent();

        let names = pet.1;

        for first_name in &names.first_names {
            for last_name in &names.last_names {
                s.wln(format!("\"{first_name}{last_name}\","));
            }
        }

        s.dec_indent();
        s.wln("];");
        s.newline();
    }

    let path = directory.join("creature_family.rs");
    overwrite_autogenerate_if_not_the_same(&path, s.inner());
}

pub(crate) fn write_area_triggers(directory: &Path, data: &Data, expansion: Expansion) {
    let mut s = Writer::new();

    let triggers = match expansion {
        Expansion::Vanilla => VANILLA_AREA_TRIGGERS,
        Expansion::BurningCrusade => TBC_AREA_TRIGGERS,
        Expansion::WrathOfTheLichKing => WRATH_AREA_TRIGGERS,
    };

    for area_trigger in triggers {
        let position = area_trigger.1.position();
        let map = match expansion.as_map_string(position.map) {
            None => continue,
            Some(e) => e,
        };
        let x = position.x;
        let y = position.y;
        let z = position.z;

        let triggers: Vec<_> = data
            .triggers
            .iter()
            .filter(|a| a.id() == area_trigger.0)
            .collect();
        if triggers.is_empty() {
            continue;
        }

        s.wln(format!("({}, (", area_trigger.0));
        s.inc_indent();
        match &area_trigger.1 {
            AreaTrigger::Circle { radius, .. } => {
                s.wln(format!("AreaTrigger::Circle {{ position: Position::new({map}, {x:.1}, {y:.1}, {z:.1}, 0.0), radius: {radius:.1} }}, "));
            }
            AreaTrigger::Square {
                length,
                width,
                height,
                yaw,
                ..
            } => {
                s.wln(format!("AreaTrigger::Square {{ position: Position::new({map}, {x:.1}, {y:.1}, {z:.1}, 0.0), length: {length:.1}, width: {width:.1}, height: {height:.1}, yaw: {yaw:.1} }},"));
            }
        }

        s.wln("&[");
        s.inc_indent();

        for trigger in triggers {
            match trigger.clone() {
                Trigger::Inn { .. } => {
                    s.wln("Trigger::Inn,");
                }
                Trigger::Quest { quest, .. } => {
                    s.wln(format!("Trigger::Quest {{ quest_id: {quest} }},"))
                }
                Trigger::Teleport {
                    map,
                    x,
                    y,
                    z,
                    orientation,
                    required_level,
                    required_item,
                    required_quest,
                    failed_text,
                    heroic_keys,
                    heroic_required_quest,
                    ..
                } => {
                    let map = expansion.as_map_string(map).unwrap();

                    let failed_text = if let Some(t) = failed_text {
                        format!("Some(\"{}\")", t.replace('\"', "\\\""))
                    } else {
                        "None".to_string()
                    };

                    let heroic_keys = if !heroic_keys.is_empty() {
                        use std::fmt::Write;
                        let mut s = "Some(&[".to_string();
                        for key in heroic_keys {
                            write!(s, "{key}, ").unwrap();
                        }
                        write!(s, "])").unwrap();
                        s
                    } else {
                        "None".to_string()
                    };

                    s.wln(format!("Trigger::Teleport {{ location: Position::new({map}, {x:.1}, {y:.1}, {z:.1}, {orientation:.1}),\
required_level: {required_level},\
required_item: {required_item},\
required_quest: {required_quest},\
failed_text: {failed_text},"));
                    match expansion {
                        Expansion::Vanilla => {
                            s.wln_no_indent("},");
                        }
                        Expansion::BurningCrusade | Expansion::WrathOfTheLichKing => {
                            s.wln_no_indent(format!(
                                "heroic_keys: {heroic_keys},\
heroic_required_quest: {heroic_required_quest} }},"
                            ));
                        }
                    }
                }
            }
        }

        s.dec_indent();
        s.wln("]");

        s.dec_indent();

        s.wln(")),");
    }

    let path = directory.join("trigger").join("triggers.rs");
    overwrite_autogenerate_if_not_the_same(&path, s.inner());
}
