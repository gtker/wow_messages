pub(crate) mod conversions;
pub(crate) mod definition;

use crate::base_printer::data::items::Items;

pub(crate) struct Stats {
    pub strength: i32,
    pub agility: i32,
    pub stamina: i32,
    pub intellect: i32,
    pub spirit: i32,
    pub health: i32,
    pub mana: i32,
}

use crate::base_printer::data::items::tbc::TbcItem;
use crate::base_printer::data::items::vanilla::VanillaItem;
use crate::base_printer::data::items::wrath::WrathItem;
use crate::base_printer::data::Data;
use crate::base_printer::write::items::definition::{definition, includes};
use crate::base_printer::writer::Writer;
use crate::base_printer::{Expansion, ImportFrom};
use crate::file_utils::overwrite_autogenerate_if_not_the_same;
use std::path::Path;

pub(crate) fn write_definition(path: &Path, data: &Data) {
    let mut s = Writer::new();

    match &data.items {
        Items::Vanilla(v) => definition(&mut s, &v[0].values(), Expansion::Vanilla),
        Items::BurningCrusade(v) => definition(&mut s, &v[0].values(), Expansion::BurningCrusade),
        Items::Wrath(v) => definition(&mut s, &v[0].values(), Expansion::WrathOfTheLichKing),
    }

    overwrite_autogenerate_if_not_the_same(&path, s.inner());
}

pub(crate) fn write_items(path: &Path, data: &Data) {
    let mut s = Writer::new();

    match &data.items {
        Items::Vanilla(v) => vanilla(&mut s, v),
        Items::BurningCrusade(v) => tbc(&mut s, v),
        Items::Wrath(v) => wrath(&mut s, v),
    }

    overwrite_autogenerate_if_not_the_same(&path, s.inner());
}

fn vanilla_unobtainable(item: &VanillaItem) -> bool {
    unobtainable(item.entry, item.extra_flags, &item.name)
}

fn tbc_unobtainable(item: &TbcItem) -> bool {
    unobtainable(item.entry, item.extra_flags, &item.name)
}

fn wrath_unobtainable(item: &WrathItem) -> bool {
    unobtainable(item.entry, item.extra_flags, &item.name)
}

fn unobtainable(entry: u32, extra_flags: i32, name: &str) -> bool {
    const UNOBTAINABLE_FLAG: i32 = 0x04;
    let unobtainable_flag_is_set = extra_flags & UNOBTAINABLE_FLAG != 0;

    let name_ends_with_deprecated = name.ends_with("DEPRECATED") || name.ends_with("DEP");
    let name_ends_with_test = name.ends_with(" Test") || name.ends_with("(Test)");

    let name_starts_with_old = name.starts_with("OLD") || name.starts_with("(OLD)");
    let name_starts_with_monster = name.starts_with("Monster - ");
    let name_starts_with_test = name.starts_with("TEST ");
    let name_starts_with_deprecated = name.starts_with("Deprecated");

    let name_contains_ph = name.contains("[PH]");

    let martin_thunder_or_martin_fury = entry == 17 || entry == 192;

    let glaive_of_the_defender = entry == 23051;

    let warglaives_of_azzinoth = entry == 18582 || entry == 18583 || entry == 18584;

    unobtainable_flag_is_set
        || name_ends_with_deprecated
        || name_starts_with_old
        || name_ends_with_test
        || name_starts_with_monster
        || name_starts_with_test
        || name_starts_with_deprecated
        || name_contains_ph
        || martin_thunder_or_martin_fury
        || glaive_of_the_defender
        || warglaives_of_azzinoth
}

fn print_unobtainable_cfg(s: &mut Writer) {
    s.wln("#[cfg(feature = \"unobtainable-items\")]");
}

fn vanilla(s: &mut Writer, items: &[VanillaItem]) {
    includes(s, &items[0].values(), Expansion::Vanilla, ImportFrom::Items);

    s.wln("pub const ITEMS: &[Item] = &[");
    s.inc_indent();

    for item in items {
        if vanilla_unobtainable(item) {
            print_unobtainable_cfg(s);
        }
        s.w("Item::new(");

        for value in item.values() {
            s.w_no_indent(format!("{},", value.value.to_string()));
        }

        s.wln_no_indent("),");
    }

    s.dec_indent();
    s.wln("];");
}

fn tbc(s: &mut Writer, items: &[TbcItem]) {
    includes(
        s,
        &items[0].values(),
        Expansion::BurningCrusade,
        ImportFrom::Items,
    );

    s.wln("pub const ITEMS: &[Item] = &[");
    s.inc_indent();

    for item in items {
        if tbc_unobtainable(item) {
            print_unobtainable_cfg(s);
        }
        s.w("Item::new(");

        for value in item.values() {
            s.w_no_indent(format!("{},", value.value.to_string()));
        }

        s.wln_no_indent("),");
    }

    s.dec_indent();
    s.wln("];");
}

fn wrath(s: &mut Writer, items: &[WrathItem]) {
    includes(
        s,
        &items[0].values(),
        Expansion::WrathOfTheLichKing,
        ImportFrom::Items,
    );

    s.wln("pub const ITEMS: &[Item] = &[");
    s.inc_indent();

    for item in items {
        if wrath_unobtainable(item) {
            print_unobtainable_cfg(s);
        }
        s.w("Item::new(");

        for value in item.values() {
            s.w_no_indent(format!("{},", value.value.to_string()));
        }

        s.wln_no_indent("),");
    }

    s.dec_indent();
    s.wln("];");
}
