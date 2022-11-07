mod data;
mod file_utils;
mod types;
mod writer;

use crate::data::{get_data_from_sqlite_file, Data};
use crate::file_utils::{
    overwrite_autogenerate_if_not_the_same, tbc_dir, vanilla_dir, workspace_directory, wrath_dir,
};
use crate::writer::Writer;
use std::path::Path;

fn main() {
    let sqlite_dir = {
        let mut p = workspace_directory();
        p.pop();
        p.join("wow_db_sqlite")
    };

    if !sqlite_dir.exists() {
        println!("Unable to find `wow_db_sqlite` directory next to the `wow_messages` directory.");
        println!("Exiting.");
        std::process::exit(1);
    }

    let vanilla_path = sqlite_dir.join("classic.sqlite");
    if !vanilla_path.exists() {
        println!("Unable to find `classic.sqlite` in `wow_db_sqlite` directory.");
        println!("Exiting.");
        std::process::exit(1);
    }

    let tbc_path = sqlite_dir.join("tbc.sqlite");
    if !vanilla_path.exists() {
        println!("Unable to find `tbc.sqlite` in `wow_db_sqlite` directory.");
        println!("Exiting.");
        std::process::exit(1);
    }

    let wrath_path = sqlite_dir.join("wotlk.sqlite");
    if !vanilla_path.exists() {
        println!("Unable to find `wotlk.sqlite` in `wow_db_sqlite` directory.");
        println!("Exiting.");
        std::process::exit(1);
    }

    let vanilla_data = get_data_from_sqlite_file(&vanilla_path);
    let tbc_data = get_data_from_sqlite_file(&tbc_path);
    let wrath_data = get_data_from_sqlite_file(&wrath_path);

    write_to_files(&vanilla_dir(), &vanilla_data);
    write_to_files(&tbc_dir(), &tbc_data);
    write_to_files(&wrath_dir(), &wrath_data);
}

fn write_to_files(directory: &Path, data: &Data) {
    write_exp(directory, data);
    write_stats(directory, data);
    write_skills(directory, data);
    write_spells(directory, data);
}

fn write_exp(directory: &Path, data: &Data) {
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

fn write_stats(directory: &Path, data: &Data) {
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

fn write_skills(directory: &Path, data: &Data) {
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

fn write_spells(directory: &Path, data: &Data) {
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
