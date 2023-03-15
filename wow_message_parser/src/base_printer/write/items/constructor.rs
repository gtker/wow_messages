use crate::base_printer::data::get_fields;
use crate::base_printer::data::items::{Array, Optimizations};
use crate::base_printer::write::items::definition::includes;
use crate::base_printer::write::items::GenericThing;
use crate::base_printer::{Expansion, ImportFrom};
use crate::rust_printer::Writer;
use std::collections::BTreeSet;

pub(crate) fn constructor(
    s: &mut Writer,
    items: &[GenericThing],
    expansion: Expansion,
    ty_name: &str,
    optimizations: &Optimizations,
) {
    let fields = get_fields(items);
    includes(
        s,
        fields,
        &items[0].arrays,
        expansion,
        ImportFrom::ItemsConstructors,
        ty_name,
        optimizations,
    );

    print_empty_arrays(s, &items[0].arrays);

    for ctor in get_constructors(items) {
        s.constructor(
            ctor.name(),
            ty_name,
            |s| {
                for (field_index, e) in fields.iter().enumerate() {
                    if optimizations.optimization(field_index).skip_field() {
                        continue;
                    }

                    let ty_name = if optimizations.is_non_native_type(e) {
                        optimizations.type_name(e)
                    } else {
                        e.value.constructor_type_name()
                    };

                    s.wln(format!("{}: {ty_name},", e.name));
                }

                for array in &items[0].arrays {
                    if !ctor.type_is_defaulted(array.type_name) {
                        s.wln(format!(
                            "{}: &'static [{}],",
                            array.variable_name, array.type_name
                        ));
                    }
                }
            },
            |s| {
                for (field_index, e) in fields.iter().enumerate() {
                    if optimizations.optimization(field_index).skip_field() {
                        continue;
                    }

                    if let Some(prefix) = e.value.definition_has_extra() {
                        s.wln(format!("{prefix}({}),", e.name));
                    } else {
                        s.wln(format!("{},", e.name));
                    }
                }

                for array in &items[0].arrays {
                    if ctor.type_is_defaulted(array.type_name) {
                        s.wln(format!("{},", empty_array_name(array.type_name)));
                    } else {
                        s.wln(format!("{},", array.variable_name));
                    }
                }
            },
        );
    }
}

fn print_empty_arrays(s: &mut Writer, arrays: &[Array]) {
    for array in arrays {
        let ty_name = array.type_name;
        s.wln(format!(
            "const {}:&[{ty_name}]=&[];",
            empty_array_name(array.type_name)
        ));
    }
}

fn empty_array_name(s: &str) -> String {
    format!("EMPTY_{}", s.to_uppercase())
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Constructor {
    name: String,
    types_that_are_defaulted: BTreeSet<&'static str>,
}

impl Constructor {
    pub(crate) fn type_is_defaulted(&self, name: &str) -> bool {
        self.types_that_are_defaulted.get(name).is_some()
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

fn get_constructors(items: &[GenericThing]) -> BTreeSet<Constructor> {
    let mut v = BTreeSet::new();

    for item in items {
        v.insert(Constructor {
            name: item.constructor_name(),
            types_that_are_defaulted: item.types_that_are_defaulted(),
        });
    }

    v
}
