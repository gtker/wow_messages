pub(crate) mod items;
pub(crate) mod spells;

use super::{Expansion};
use crate::base_printer::data::items::{get_items, Field, Optimizations};
use crate::base_printer::data::spells::get_spells;
use crate::base_printer::write::GenericThing;

pub(crate) struct Data {
    pub items: (Vec<GenericThing>, Optimizations),
    pub spells: (Vec<GenericThing>, Optimizations),
}

pub(crate) fn get_fields(things: &[GenericThing]) -> &[Field] {
    &things[0].fields
}

pub(crate) fn get_data_from_csv_files(expansion: Expansion) -> Data {
    let spell_thread =
        std::thread::spawn(move || get_spells(expansion, &expansion.csv_data_directory()));
    let csv_directory = expansion.csv_data_directory();

    let items = get_items(&csv_directory, expansion);

    let spells = spell_thread.join().unwrap();

    Data {
        items,
        spells,
    }
}
