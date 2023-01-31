pub(crate) mod conversions;
pub(crate) mod definition;

use crate::base_printer::data::items::{Field, Items};

pub(crate) struct Stats {
    pub strength: i32,
    pub agility: i32,
    pub stamina: i32,
    pub intellect: i32,
    pub spirit: i32,
    pub health: i32,
    pub mana: i32,
}

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

struct GenericItem {
    entry: u32,
    extra_flags: i32,
    name: String,
    fields: Vec<Field>,
}

pub(crate) fn write_items(path: &Path, data: &Data) {
    let mut s = Writer::new();

    let (items, expansion): (Vec<GenericItem>, Expansion) = match &data.items {
        Items::Vanilla(v) => (
            v.iter()
                .map(|a| GenericItem {
                    entry: a.entry,
                    extra_flags: a.extra_flags,
                    name: a.name.clone(),
                    fields: a.values(),
                })
                .collect(),
            Expansion::Vanilla,
        ),
        Items::BurningCrusade(v) => (
            v.iter()
                .map(|a| GenericItem {
                    entry: a.entry,
                    extra_flags: a.extra_flags,
                    name: a.name.clone(),
                    fields: a.values(),
                })
                .collect(),
            Expansion::BurningCrusade,
        ),
        Items::Wrath(v) => (
            v.iter()
                .map(|a| GenericItem {
                    entry: a.entry,
                    extra_flags: a.extra_flags,
                    name: a.name.clone(),
                    fields: a.values(),
                })
                .collect(),
            Expansion::WrathOfTheLichKing,
        ),
    };

    all_items(&mut s, &items, expansion);

    overwrite_autogenerate_if_not_the_same(&path, s.inner());
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

fn all_items(s: &mut Writer, items: &[GenericItem], expansion: Expansion) {
    includes(s, &items[0].fields, expansion, ImportFrom::Items);

    s.wln("pub const ITEMS: &[Item] = &[");
    s.inc_indent();

    for item in items {
        if unobtainable(item.entry, item.extra_flags, &item.name) {
            print_unobtainable_cfg(s);
        }
        s.w("Item::new(");

        for value in &item.fields {
            s.w_no_indent(format!("{},", value.value.to_string()));
        }

        s.wln_no_indent("),");
    }

    s.dec_indent();
    s.wln("];");
}
