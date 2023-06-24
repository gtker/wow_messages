use crate::base_printer::data::get_fields;
use crate::base_printer::data::items::{ArrayInstances, IntegerSize, Optimizations, Value};
use crate::base_printer::write::items::definition::includes;
use crate::base_printer::write::items::GenericThing;
use crate::base_printer::{Expansion, ImportFrom};
use crate::rust_printer::writer::Writer;
use hashbrown::HashMap;

pub(crate) fn all_items(
    s: &mut Writer,
    items: &[GenericThing],
    expansion: Expansion,
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

    print_data(s, items, default_values, arrays, optimizations, ty_name);

    print_lookup(s, items);
}

fn print_data(
    s: &mut Writer,
    items: &[GenericThing],
    default_values: &HashMap<(Value, Option<IntegerSize>), String>,
    arrays: &HashMap<(&ArrayInstances, &str), String>,
    optimizations: &Optimizations,
    ty_name: &str,
) {
    s.wln(format!("pub const Z________DATA: &[{ty_name}] = &["));

    for item in items {
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
    s.newline();
}

fn print_lookup(s: &mut Writer, items: &[GenericThing]) {
    let max_index = items.len();
    let ty = lookup_type(max_index);

    s.wln(format!("pub const Z________LOOKUP: &[{ty}] = &["));

    let min = items.iter().next().unwrap().entry;
    let max = items.iter().last().unwrap().entry;

    let mut offset = 0;
    for i in min..=max {
        if let Some((pos, new_offset)) = find_item_position(items, i, offset) {
            offset = new_offset;
            s.wln(format!("{pos},"));
        } else {
            s.wln(format!("{ty}::MAX,"));
        }
    }

    s.wln("];");
}

fn find_item_position(items: &[GenericThing], entry: u32, offset: usize) -> Option<(usize, usize)> {
    for (i, item) in items[offset..].iter().enumerate() {
        if item.entry == entry {
            return Some((offset + i, offset + i));
        } else if item.entry > entry {
            return None;
        }
    }

    None
}

pub(crate) fn lookup_type(max_index: usize) -> &'static str {
    if max_index > u16::MAX as usize {
        "usize"
    } else {
        "u16"
    }
}
