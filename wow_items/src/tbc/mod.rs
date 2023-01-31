pub(crate) mod constructors;
mod data;

use crate::find_and_data;
pub use wow_world_base::tbc::Item;

find_and_data!(data::ITEMS);
