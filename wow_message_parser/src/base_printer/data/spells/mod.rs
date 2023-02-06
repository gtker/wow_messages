pub(crate) mod tbc;
pub(crate) mod vanilla;
pub(crate) mod wrath;

use crate::base_printer::data::items::Optimizations;
use crate::base_printer::data::spells::tbc::tbc;
use crate::base_printer::data::spells::vanilla::vanilla;
use crate::base_printer::data::spells::wrath::wrath;
use crate::base_printer::write::items::GenericThing;
use crate::base_printer::Expansion;
use rusqlite::Connection;

pub(crate) fn get_spells(
    conn: &Connection,
    expansion: Expansion,
) -> (Vec<GenericThing>, Optimizations) {
    match expansion {
        Expansion::Vanilla => vanilla(conn),
        Expansion::BurningCrusade => tbc(conn),
        Expansion::WrathOfTheLichKing => wrath(conn),
    }
}
