mod data;
mod position;
mod types;
mod write;
mod writer;

use crate::base_printer::write::items::{
    unobtainable_item, write_constructors, write_definition, write_pub_use, write_things,
};
use crate::path_utils::workspace_directory;
use data::{get_data_from_sqlite_file, Data};
use std::path::PathBuf;

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
    write::write_initial_spells(&expansion.base_extended_path(), data);
    write::write_positions(&expansion.base_extended_path(), data, expansion);
    write::write_actions(&expansion.base_extended_path(), data);
    write::write_area_triggers(&expansion.base_extended_path(), data, expansion);
    write::write_pet_names(&expansion.base_extended_path(), data, expansion);

    write_items(data, expansion);
    write_spells(data, expansion);
}

fn write_items(data: &Data, expansion: Expansion) {
    const TY_NAME: &str = "Item";

    let items = &data.items.0;

    let optimizations = &data.items.1;

    write_things(
        &expansion.item_data_path(),
        &items,
        expansion,
        TY_NAME,
        |i| unobtainable_item(i.entry, i.extra_flags, &i.name),
        optimizations,
    );
    write_definition(
        &expansion.item_definition_path(),
        &items[0].fields,
        &items[0].arrays,
        expansion,
        TY_NAME,
        optimizations,
    );
    write_constructors(
        &expansion.item_constructor_path(),
        &items,
        expansion,
        TY_NAME,
        optimizations,
    );
    write_pub_use(
        &expansion.item_pub_use_path(),
        &items,
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
        &spells,
        expansion,
        TY_NAME,
        |_| false,
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
        &spells,
        expansion,
        TY_NAME,
        optimizations,
    );
    write_pub_use(
        &expansion.spell_pub_use_path(),
        &spells,
        expansion,
        TY_NAME,
        optimizations,
    );
}
