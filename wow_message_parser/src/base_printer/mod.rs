mod data;
mod position;
mod types;
mod write;
mod writer;

use crate::path_utils::{
    tbc_base_extended_dir, vanilla_base_extended_dir, wrath_base_extended_dir,
};
use data::{get_data_from_sqlite_file, Data};
use std::path::{Path, PathBuf};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Expansion {
    Vanilla,
    BurningCrusade,
    WrathOfTheLichKing,
}

pub(crate) fn print_base() {
    let sqlite_dir = if let Ok(p) = std::env::var("WOWM_SQLITE_DB_PATH") {
        PathBuf::from(p)
    } else {
        return;
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

    write_to_files(
        &vanilla_base_extended_dir(),
        &vanilla_data,
        Expansion::Vanilla,
    );
    write_to_files(
        &tbc_base_extended_dir(),
        &tbc_data,
        Expansion::BurningCrusade,
    );
    write_to_files(
        &wrath_base_extended_dir(),
        &wrath_data,
        Expansion::WrathOfTheLichKing,
    );
}

fn write_to_files(directory: &Path, data: &Data, expansion: Expansion) {
    write::write_exp(directory, data);
    write::write_stats(directory, data);
    write::write_skills(directory, data, expansion);
    write::write_spells(directory, data);
    write::write_positions(directory, data, expansion);
    write::write_actions(directory, data);
}
