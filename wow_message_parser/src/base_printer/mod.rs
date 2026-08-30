mod data;
mod position;
mod types;
mod write;

use crate::path_utils::workspace_directory;
use data::{get_data_from_csv_files, Data};
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};

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

    pub fn csv_data_directory(&self) -> PathBuf {
        if let Some(s) = option_env!("WOWM_SQLITE_DB_PATH") {
            PathBuf::from(s)
        } else {
            workspace_directory().join("../wow_db_sqlite")
        }
        .join(self.as_module_string())
    }

    pub fn base_extended_path(&self) -> PathBuf {
        workspace_directory()
            .join("wow_world_base")
            .join("src")
            .join("extended")
            .join(self.as_module_string())
    }

    pub fn values() -> [Self; 3] {
        [
            Self::Vanilla,
            Self::BurningCrusade,
            Self::WrathOfTheLichKing,
        ]
    }
}

pub(crate) fn print_base() {
    let sqlite_dir = if let Ok(p) = std::env::var("WOWM_SQLITE_DB_PATH") {
        PathBuf::from(p)
    } else {
        PathBuf::from("../wow_db_sqlite")
    };

    if !sqlite_dir.exists() {
        println!("Unable to find `wow_db_sqlite` directory next to the `wow_messages` directory.");
        println!("Exiting.");
        std::process::exit(1);
    }

    for expansion in Expansion::values() {
        let path = expansion.csv_data_directory();
        if !path.exists() {
            println!(
                "Unable to find `{}` in `wow_db_sqlite` directory.",
                path.display()
            );
            println!("Exiting.");
            std::process::exit(1);
        }
    }

    fn run(expansion: Expansion) {
        let data = get_data_from_csv_files(expansion);
        write_to_files(&data, expansion);
    }

    std::thread::scope(|s| {
        s.spawn(|| run(Expansion::WrathOfTheLichKing));
        s.spawn(|| run(Expansion::BurningCrusade));
        run(Expansion::Vanilla);
    });
}

fn write_to_files(data: &Data, expansion: Expansion) {
    write::write_exp(&expansion.base_extended_path(), data);
    write::write_stats(&expansion.base_extended_path(), data);
    write::write_skills(&expansion.base_extended_path(), data, expansion);
    write::write_initial_spells(&expansion.base_extended_path(), data);
    write::write_positions(&expansion.base_extended_path(), data, expansion);
    write::write_actions(&expansion.base_extended_path(), data);
    write::write_area_triggers(&expansion.base_extended_path(), data, expansion);
    write::write_pet_names(&expansion.base_extended_path(), data, expansion);
}

pub(crate) fn read_csv_file<T: DeserializeOwned>(dir: &Path, filename: &str) -> Vec<T> {
    let dir = dir.join(format!("{filename}.csv"));
    let dir_display = dir.display();

    let mut r = match csv::Reader::from_path(&dir) {
        Ok(e) => e,
        Err(e) => {
            panic!("unable to read {dir_display}: {e}");
        }
    };

    r.deserialize()
        .map(|a| match a {
            Ok(a) => a,
            Err(e) => panic!("unable to unwrap {dir_display}: {e}"),
        })
        .collect()
}
