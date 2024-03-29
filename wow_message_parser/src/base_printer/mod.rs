mod data;
mod position;
mod types;
mod write;

use crate::base_printer::data::get_fields;
use crate::base_printer::write::items::{
    write_constructors, write_definition, write_pub_use, write_things,
};
use crate::path_utils::workspace_directory;
use data::{get_data_from_sqlite_file, Data};
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ImportFrom {
    ItemsConstructors,
    Items,
    Definition,
    ItemPubUse,
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
        workspace_directory()
            .join("wow_items")
            .join("src")
            .join(self.as_module_string())
            .join("data.rs")
    }

    pub fn csv_data_directory(&self) -> PathBuf {
        PathBuf::from(env!("WOWM_SQLITE_DB_PATH")).join(self.as_module_string())
    }

    pub fn base_extended_path(&self) -> PathBuf {
        workspace_directory()
            .join("wow_world_base")
            .join("src")
            .join("extended")
            .join(self.as_module_string())
    }

    pub fn item_definition_path(&self) -> PathBuf {
        workspace_directory()
            .join("wow_world_base")
            .join("src")
            .join("manual")
            .join(self.as_module_string())
            .join("item.rs")
    }

    pub fn item_constructor_path(&self) -> PathBuf {
        workspace_directory()
            .join("wow_items")
            .join("src")
            .join(self.as_module_string())
            .join("constructors.rs")
    }

    pub fn item_pub_use_path(&self) -> PathBuf {
        workspace_directory()
            .join("wow_items")
            .join("src")
            .join(self.as_module_string())
            .join("mod.rs")
    }

    pub fn spell_data_path(&self) -> PathBuf {
        workspace_directory()
            .join("wow_spells")
            .join("src")
            .join(self.as_module_string())
            .join("data.rs")
    }

    pub fn spell_definition_path(&self) -> PathBuf {
        workspace_directory()
            .join("wow_world_base")
            .join("src")
            .join("manual")
            .join(self.as_module_string())
            .join("spell.rs")
    }

    pub fn spell_constructor_path(&self) -> PathBuf {
        workspace_directory()
            .join("wow_spells")
            .join("src")
            .join(self.as_module_string())
            .join("constructors.rs")
    }

    pub fn spell_pub_use_path(&self) -> PathBuf {
        workspace_directory()
            .join("wow_spells")
            .join("src")
            .join(self.as_module_string())
            .join("mod.rs")
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

    fn run(path: &Path, expansion: Expansion) {
        let filename = match expansion {
            Expansion::Vanilla => "classic.sqlite",
            Expansion::BurningCrusade => "tbc.sqlite",
            Expansion::WrathOfTheLichKing => "wotlk.sqlite",
        };
        let path = path.join(filename);
        if !path.exists() {
            println!("Unable to find `{filename}` in `wow_db_sqlite` directory.");
            println!("Exiting.");
            std::process::exit(1);
        }

        let data = get_data_from_sqlite_file(expansion);
        std::thread::scope(|s| {
            s.spawn(|| write_to_files(&data, expansion));

            // Spells end up taking way longer than everything else so do it in parallel
            write_spells(&data, expansion);
        })
    }

    std::thread::scope(|s| {
        s.spawn(|| run(&sqlite_dir, Expansion::WrathOfTheLichKing));
        s.spawn(|| run(&sqlite_dir, Expansion::BurningCrusade));
        run(&sqlite_dir, Expansion::Vanilla);
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

    write_items(data, expansion);
}

fn write_items(data: &Data, expansion: Expansion) {
    const TY_NAME: &str = "Item";

    let items = &data.items.0;

    let optimizations = &data.items.1;

    write_things(
        &expansion.item_data_path(),
        items,
        expansion,
        TY_NAME,
        optimizations,
    );
    write_definition(
        &expansion.item_definition_path(),
        get_fields(items),
        &items[0].arrays,
        expansion,
        TY_NAME,
        optimizations,
    );
    write_constructors(
        &expansion.item_constructor_path(),
        items,
        expansion,
        TY_NAME,
        optimizations,
    );
    write_pub_use(
        &expansion.item_pub_use_path(),
        items,
        expansion,
        TY_NAME,
        optimizations,
    );
}

fn write_spells(data: &Data, expansion: Expansion) {
    const TY_NAME: &str = "Spell";

    let spells = &data.spells.0;

    let optimizations = &data.spells.1;

    write_things(
        &expansion.spell_data_path(),
        spells,
        expansion,
        TY_NAME,
        optimizations,
    );
    write_definition(
        &expansion.spell_definition_path(),
        &spells[0].fields,
        &spells[0].arrays,
        expansion,
        TY_NAME,
        optimizations,
    );
    write_constructors(
        &expansion.spell_constructor_path(),
        spells,
        expansion,
        TY_NAME,
        optimizations,
    );
    write_pub_use(
        &expansion.spell_pub_use_path(),
        spells,
        expansion,
        TY_NAME,
        optimizations,
    );
}

pub(crate) fn read_csv_file<T: DeserializeOwned>(dir: &Path, filename: &str) -> Vec<T> {
    let dir = dir.join(format!("{filename}.csv"));
    let mut r = csv::Reader::from_path(dir).expect(filename);

    r.deserialize().map(|a| a.expect(filename)).collect()
}
