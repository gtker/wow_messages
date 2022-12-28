mod data;
use crate::manual::vanilla::Item;
pub use data::*;

pub fn lookup_item(id: u32) -> Option<&'static Item> {
    ITEMS.iter().find(|a| a.entry as u32 == id)
}
