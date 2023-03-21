use crate::base_printer::data::get_fields;
use crate::base_printer::data::items::{ArrayInstances, IntegerSize, Optimizations, Value};
use crate::base_printer::write::items::definition::includes;
use crate::base_printer::write::items::GenericThing;
use crate::base_printer::{Expansion, ImportFrom};
use crate::rust_printer::Writer;
use hashbrown::HashMap;

pub fn unobtainable_item(entry: u32, extra_flags: i32, name: &str) -> bool {
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
    s.wln("#[cfg(feature = \"unobtainable\")]");
}

pub(crate) fn all_items(
    s: &mut Writer,
    items: &[GenericThing],
    expansion: Expansion,
    unobtainable: impl Fn(&GenericThing) -> bool,
    ty_name: &str,
    default_values: &HashMap<(Value, Option<IntegerSize>), String>,
    arrays: &HashMap<(&ArrayInstances, &'static str), String>,
    optimizations: &Optimizations,
) {
    includes(
        s,
        get_fields(items),
        &items[0].arrays,
        expansion,
        ImportFrom::Items,
        ty_name,
        optimizations,
    );

    s.wln(format!("pub const Z________DATA: &[{ty_name}] = &["));

    for item in items {
        if unobtainable(item) {
            print_unobtainable_cfg(s);
        }
        s.w(format!("{}(", item.constructor_name()));

        for (field_index, value) in item.fields.iter().enumerate() {
            if optimizations.optimization(field_index).skip_field() {
                continue;
            }

            if let Some(const_name) = default_values.get(&(
                value.value.const_value(),
                optimizations.integer_size(field_index),
            )) {
                s.w_no_indent(format!("{const_name},"));
            } else {
                s.w_no_indent(format!("{},", value.value.to_string_value()));
            }
        }

        for array in &item.arrays {
            if array.is_default() {
                continue;
            }

            s.w_no_indent(format!(
                "{},",
                arrays.get(&(&array.instances, array.type_name)).unwrap()
            ));
        }

        s.wln_no_indent("),");
    }

    s.wln("];");
}
