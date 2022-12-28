mod data;

pub use data::*;

use crate::tbc::Item;

pub fn lookup_item(id: u32) -> Option<&'static Item> {
    ITEMS.iter().find(|a| a.entry as u32 == id)
}
