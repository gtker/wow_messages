use crate::base_printer::data::items::Optimizations;
use crate::base_printer::write::items::definition::includes;
use crate::base_printer::write::items::GenericThing;
use crate::base_printer::writer::Writer;
use crate::base_printer::{Expansion, ImportFrom};
use std::collections::BTreeSet;

pub(crate) fn constructor(
    s: &mut Writer,
    items: &[GenericThing],
    expansion: Expansion,
    ty_name: &str,
    optimizations: &Optimizations,
) {
    includes(
        s,
        &items[0].fields,
        &items[0].arrays,
        expansion,
        ImportFrom::ItemsConstructors,
        ty_name,
        optimizations,
    );

    for ctor in get_constructors(items) {
        s.constructor(
            ctor.name(),
            ty_name,
            |s| {
                for e in &items[0].fields {
                    if optimizations.optimization(e.name).skip_field() {
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
                        s.wln(format!("{}_length: u8,", array.variable_name));

                        for instance in array.instances.instances() {
                            for field in instance.fields() {
                                s.wln(format!(
                                    "{}: {},",
                                    field.variable_name,
                                    field.value.type_name()
                                ));
                            }
                        }
                    }
                }
            },
            |s| {
                for e in &items[0].fields {
                    if optimizations.optimization(e.name).skip_field() {
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
                        s.wln("0,");
                    } else {
                        s.wln(format!("{}_length,", array.variable_name));
                    }

                    for instance in array.instances.instances() {
                        for field in instance.fields() {
                            if ctor.type_is_defaulted(array.type_name) {
                                s.wln(format!(
                                    "{},",
                                    field.value.default_value().to_string_value(),
                                ));
                            } else {
                                s.wln(format!("{},", field.variable_name,));
                            }
                        }
                    }
                }
            },
        );
    }
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
