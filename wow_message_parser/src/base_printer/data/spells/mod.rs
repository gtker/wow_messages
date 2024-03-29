pub(crate) mod tbc;
pub(crate) mod vanilla;
pub(crate) mod wrath;

use crate::base_printer::data::items::Optimizations;
use crate::base_printer::data::spells::tbc::tbc;
use crate::base_printer::data::spells::vanilla::vanilla;
use crate::base_printer::data::spells::wrath::wrath;
use crate::base_printer::write::items::GenericThing;
use crate::base_printer::Expansion;
use std::path::Path;

pub(crate) fn get_spells(expansion: Expansion, dir: &Path) -> (Vec<GenericThing>, Optimizations) {
    match expansion {
        Expansion::Vanilla => vanilla(dir),
        Expansion::BurningCrusade => tbc(dir),
        Expansion::WrathOfTheLichKing => wrath(dir),
    }
}
