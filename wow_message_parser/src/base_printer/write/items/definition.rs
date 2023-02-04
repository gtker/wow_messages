use crate::base_printer::data::items::{Array, Field};
use crate::base_printer::writer::Writer;
use crate::base_printer::{Expansion, ImportFrom};
use std::collections::BTreeSet;

pub(crate) fn definition(
    s: &mut Writer,
    fields: &[Field],
    arrays: &[Array],
    expansion: Expansion,
    ty_name: &str,
) {
    s.wln("#![allow(clippy::too_many_arguments)]");
    includes(
        s,
        fields,
        arrays,
        expansion,
        ImportFrom::Definition,
        ty_name,
    );

    struct_definition(s, fields, arrays, ty_name);
    impl_block(s, fields, arrays, ty_name);

    array_definitions(s, arrays);
}

pub(crate) fn includes(
    s: &mut Writer,
    fields: &[Field],
    arrays: &[Array],
    expansion: Expansion,
    import_location: ImportFrom,
    ty_name: &str,
) {
    let mut set = BTreeSet::new();

    match import_location {
        ImportFrom::ItemPubUse | ImportFrom::ItemsConstructors | ImportFrom::Items => {
            set.insert(ty_name);
        }
        ImportFrom::Definition => {}
    }

    if import_location == ImportFrom::Items {
        s.wln("use super::constructors::*;");
    }

    let location = match import_location {
        ImportFrom::ItemPubUse | ImportFrom::ItemsConstructors | ImportFrom::Items => {
            "wow_world_base"
        }
        ImportFrom::Definition => "crate",
    };
    s.wln(format!(
        "use {location}::{}::{{",
        expansion.as_module_string()
    ));
    s.inc_indent();

    for e in fields {
        if import_location == ImportFrom::Items && e.value.definition_has_extra().is_some() {
            continue;
        }

        if let Some(name) = e.value.import_name() {
            set.insert(name);
        }
    }

    for array in arrays {
        match import_location {
            ImportFrom::ItemPubUse | ImportFrom::ItemsConstructors => {
                set.insert(array.type_name);
            }
            ImportFrom::Items => {}
            ImportFrom::Definition => {
                if array.import_only {
                    set.insert(array.type_name);
                }
            }
        }

        for e in &array.instances[0] {
            if let Some(name) = e.value.import_name() {
                set.insert(name);
            }
        }
    }

    for e in set {
        s.wln(format!("{},", e));
    }

    s.dec_indent();
    s.wln("};");
    s.newline();
}

fn struct_definition(s: &mut Writer, fields: &[Field], arrays: &[Array], ty_name: &str) {
    s.wln("#[derive(Debug, Copy, Clone)]");
    s.open_curly(format!("pub struct {ty_name}"));

    for e in fields {
        s.wln(format!("pub {}: {},", e.name, e.value.type_name()));
    }

    for array in arrays {
        s.wln(format!(
            "pub {}: [{}; {}],",
            array.variable_name,
            array.type_name,
            array.instances.len()
        ));
    }

    s.closing_curly();
    s.newline();
}

fn array_definitions(s: &mut Writer, arrays: &[Array]) {
    for array in arrays {
        if array.import_only {
            continue;
        }

        s.wln("#[derive(Debug, Copy, Clone)]");
        s.open_curly(format!("pub struct {}", array.type_name));

        for e in &array.instances[0] {
            s.wln(format!("pub {}: {},", e.name, e.value.type_name()));
        }

        s.closing_curly();
        s.newline();

        s.open_curly(format!("impl {}", array.type_name));

        s.pub_const_fn_new(
            |s| {
                for e in &array.instances[0] {
                    s.wln(format!("{}: {},", e.name, e.value.type_name()));
                }
            },
            |s| {
                for e in &array.instances[0] {
                    s.wln(format!("{},", e.name));
                }
            },
        );

        s.closing_curly(); // impl block
    }
}

fn impl_block(s: &mut Writer, fields: &[Field], arrays: &[Array], ty_name: &str) {
    s.open_curly(format!("impl {ty_name}"));

    s.pub_const_fn_new(
        |s| {
            for e in fields {
                s.wln(format!("{}: {},", e.name, e.value.type_name()));
            }

            for array in arrays {
                for instance in &array.instances {
                    for field in instance {
                        s.wln(format!(
                            "{}: {},",
                            field.variable_name,
                            field.value.type_name()
                        ));
                    }
                }
            }
        },
        |s| {
            for e in fields {
                s.wln(format!("{},", e.name));
            }

            for array in arrays {
                s.wln(format!("{}: [", array.variable_name));

                for instance in &array.instances {
                    if array.import_only {
                        s.open_curly(array.type_name);

                        for field in instance {
                            s.wln(format!("{}: {},", field.name, field.variable_name,));
                        }

                        s.dec_indent();
                        s.wln("},");
                    } else {
                        s.wln(format!("{}::new(", array.type_name));
                        for field in instance {
                            s.wln(format!("{},", field.variable_name,));
                        }
                        s.wln("),");
                    }
                }

                s.wln("],");
            }
        },
    );

    getters_and_setters(s, fields, arrays);

    s.closing_curly();
}

fn getters_and_setters(s: &mut Writer, fields: &[Field], arrays: &[Array]) {
    for field in fields {
        s.pub_const_fn(field.name, field.value.type_name(), |s| {
            s.wln(format!("self.{}", field.name));
        });
        s.newline();
    }

    for array in arrays {
        s.pub_const_fn(
            array.variable_name,
            format!("&[{}; {}]", array.type_name, array.instances.len()),
            |s| {
                s.wln(format!("&self.{}", array.variable_name));
            },
        );
        s.newline();
    }
}
