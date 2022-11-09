mod data;
mod file_utils;
mod position;
mod types;
mod write;
mod writer;

use crate::data::{get_data_from_sqlite_file, Data};
use crate::file_utils::{tbc_dir, vanilla_dir, workspace_directory, wrath_dir};
use std::path::Path;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Expansion {
    Vanilla,
    BurningCrusade,
    WrathOfTheLichKing,
}

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

    write_to_files(&vanilla_dir(), &vanilla_data, Expansion::Vanilla);
    write_to_files(&tbc_dir(), &tbc_data, Expansion::BurningCrusade);
    write_to_files(&wrath_dir(), &wrath_data, Expansion::WrathOfTheLichKing);
}

fn write_to_files(directory: &Path, data: &Data, expansion: Expansion) {
    write::write_exp(directory, data);
    write::write_stats(directory, data);
    write::write_skills(directory, data);
    write::write_spells(directory, data);
    write::write_positions(directory, data, expansion);
    write::write_actions(directory, data);
}
