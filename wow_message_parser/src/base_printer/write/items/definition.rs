use crate::base_printer::data::items::{Array, Field};
use crate::base_printer::writer::Writer;
use crate::base_printer::{Expansion, ImportFrom};
use std::collections::BTreeSet;

pub(crate) fn definition(s: &mut Writer, fields: &[Field], arrays: &[Array], expansion: Expansion) {
    includes(s, &fields, &arrays, expansion, ImportFrom::Definition);

    struct_definition(s, &fields, &arrays);
    impl_block(s, &fields, &arrays);

    array_definitions(s, &arrays);
}

pub(crate) fn includes(
    s: &mut Writer,
    fields: &[Field],
    arrays: &[Array],
    expansion: Expansion,
    import_location: ImportFrom,
) {
    let mut set = BTreeSet::new();

    match import_location {
        ImportFrom::ItemsConstructors | ImportFrom::Items => {
            set.insert("Item");
        }
        ImportFrom::Definition => {}
    }

    if import_location == ImportFrom::Items {
        s.wln("use super::constructors::*;");
    }

    let location = match import_location {
        ImportFrom::ItemsConstructors | ImportFrom::Items => "wow_world_base",
        ImportFrom::Definition => "crate",
    };
    s.wln(format!(
        "use {location}::{}::{{",
        expansion.as_module_string()
    ));
    s.inc_indent();

    for e in fields {
        if let Some(name) = e.value.import_name() {
            set.insert(name);
        }
    }

    for array in arrays {
        match import_location {
            ImportFrom::ItemsConstructors => {
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

fn struct_definition(s: &mut Writer, fields: &[Field], arrays: &[Array]) {
    s.wln("#[derive(Debug, Copy, Clone)]");
    s.open_curly("pub struct Item");

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

fn impl_block(s: &mut Writer, fields: &[Field], arrays: &[Array]) {
    s.open_curly("impl Item");
    s.wln("#[allow(clippy::complexity)]");

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
                        s.open_curly(format!("{}", array.type_name));

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

    s.closing_curly();
}
