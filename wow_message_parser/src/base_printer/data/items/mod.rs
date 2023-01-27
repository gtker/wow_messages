use crate::base_printer::Expansion;
use rusqlite::Connection;
use tbc::TbcItem;
use vanilla::VanillaItem;
use wrath::WrathItem;

pub mod tbc;
pub mod vanilla;
pub mod wrath;

pub enum Items {
    Vanilla(Vec<VanillaItem>),
    BurningCrusade(Vec<TbcItem>),
    Wrath(Vec<WrathItem>),
}

fn i32_to_u32(v: i32) -> u32 {
    u32::from_le_bytes(v.to_le_bytes())
}

pub(crate) fn get_items(conn: &Connection, expansion: Expansion) -> Items {
    match expansion {
        Expansion::Vanilla => Items::Vanilla(vanilla::vanilla(conn)),
        Expansion::BurningCrusade => Items::BurningCrusade(tbc::tbc(conn)),
        Expansion::WrathOfTheLichKing => Items::Wrath(wrath::wrath(conn)),
    }
}
