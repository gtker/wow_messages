pub(crate) mod tbc;
pub(crate) mod vanilla;
pub(crate) mod wrath;

use crate::base_printer::data::items::Field;
use crate::base_printer::data::spells::tbc::tbc;
use crate::base_printer::data::spells::vanilla::vanilla;
use crate::base_printer::data::spells::wrath::wrath;
use crate::base_printer::Expansion;
use rusqlite::Connection;

pub(crate) struct GenericSpell {
    pub entry: i32,
    pub fields: Vec<Field>,
}

pub(crate) fn get_spells(conn: &Connection, expansion: Expansion) -> Vec<GenericSpell> {
    match expansion {
        Expansion::Vanilla => vanilla(conn),
        Expansion::BurningCrusade => tbc(conn),
        Expansion::WrathOfTheLichKing => wrath(conn),
    }
}
