mod data;
mod position;
mod types;
mod write;
mod writer;

use crate::base_printer::write::items::{write_definition, write_items};
use crate::path_utils::{
    tbc_base_extended_dir, tbc_item_data_path, tbc_item_definition_path, vanilla_base_extended_dir,
    vanilla_item_data_path, vanilla_item_definition_path, wrath_base_extended_dir,
    wrath_item_data_path, wrath_item_definition_path,
};
use data::{get_data_from_sqlite_file, Data};
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ImportFrom {
    Items,
    Base,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Expansion {
    Vanilla,
    BurningCrusade,
    WrathOfTheLichKing,
}

impl Expansion {
    pub(crate) fn as_map_string(&self, map: u32) -> Option<String> {
        Some(match self {
            Expansion::Vanilla => format!(
                "Map::{:?}",
                wow_world_base::vanilla::Map::try_from(map).ok()?
            ),
            Expansion::BurningCrusade => {
                format!("Map::{:?}", wow_world_base::tbc::Map::try_from(map).ok()?)
            }
            Expansion::WrathOfTheLichKing => {
                format!("Map::{:?}", wow_world_base::wrath::Map::try_from(map).ok()?)
            }
        })
    }

    pub fn as_module_string(&self) -> &'static str {
        match self {
            Expansion::Vanilla => "vanilla",
            Expansion::BurningCrusade => "tbc",
            Expansion::WrathOfTheLichKing => "wrath",
        }
    }

    pub fn item_data_path(&self) -> PathBuf {
        match self {
            Expansion::Vanilla => vanilla_item_data_path(),
            Expansion::BurningCrusade => tbc_item_data_path(),
            Expansion::WrathOfTheLichKing => wrath_item_data_path(),
        }
    }

    pub fn base_extended_path(&self) -> PathBuf {
        match self {
            Expansion::Vanilla => vanilla_base_extended_dir(),
            Expansion::BurningCrusade => tbc_base_extended_dir(),
            Expansion::WrathOfTheLichKing => wrath_base_extended_dir(),
        }
    }

    pub fn item_definition_path(&self) -> PathBuf {
        match self {
            Expansion::Vanilla => vanilla_item_definition_path(),
            Expansion::BurningCrusade => tbc_item_definition_path(),
            Expansion::WrathOfTheLichKing => wrath_item_definition_path(),
        }
    }
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

    let vanilla_data = get_data_from_sqlite_file(&vanilla_path, Expansion::Vanilla);
    let tbc_data = get_data_from_sqlite_file(&tbc_path, Expansion::BurningCrusade);
    let wrath_data = get_data_from_sqlite_file(&wrath_path, Expansion::WrathOfTheLichKing);

    write_to_files(&vanilla_data, Expansion::Vanilla);
    write_to_files(&tbc_data, Expansion::BurningCrusade);
    write_to_files(&wrath_data, Expansion::WrathOfTheLichKing);
}

fn write_to_files(data: &Data, expansion: Expansion) {
    write::write_exp(&expansion.base_extended_path(), data);
    write::write_stats(&expansion.base_extended_path(), data);
    write::write_skills(&expansion.base_extended_path(), data, expansion);
    write::write_spells(&expansion.base_extended_path(), data);
    write::write_positions(&expansion.base_extended_path(), data, expansion);
    write::write_actions(&expansion.base_extended_path(), data);
    write::write_area_triggers(&expansion.base_extended_path(), data, expansion);
    write::write_pet_names(&expansion.base_extended_path(), data, expansion);
    write_items(&expansion.item_data_path(), data);
    write_definition(&expansion.item_definition_path(), data);
}
