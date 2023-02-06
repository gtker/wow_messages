use crate::base_printer::data::items::{Array, Field, FieldOptimization, Optimizations};
use crate::base_printer::writer::Writer;
use crate::base_printer::{Expansion, ImportFrom};
use std::collections::BTreeSet;

pub(crate) fn definition(
    s: &mut Writer,
    fields: &[Field],
    arrays: &[Array],
    expansion: Expansion,
    ty_name: &str,
    optimizations: &Optimizations,
) {
    s.wln("#![allow(clippy::too_many_arguments)]");
    includes(
        s,
        fields,
        arrays,
        expansion,
        ImportFrom::Definition,
        ty_name,
        optimizations,
    );

    struct_definition(s, fields, arrays, ty_name, optimizations);
    impl_block(s, fields, arrays, ty_name, optimizations);

    array_definitions(s, arrays);
}

pub(crate) fn includes(
    s: &mut Writer,
    fields: &[Field],
    arrays: &[Array],
    expansion: Expansion,
    import_location: ImportFrom,
    ty_name: &str,
    optimizations: &Optimizations,
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
        if matches!(
            import_location,
            ImportFrom::ItemsConstructors | ImportFrom::Items
        ) && optimizations.optimization(e.name).skip_field()
        {
            continue;
        }

        if import_location == ImportFrom::Items && e.value.definition_has_extra().is_some() {
            continue;
        }

        if let Some(name) = e.value.import_name() {
            set.insert(name);
        }
    }

    for array in arrays {
        match import_location {
            ImportFrom::ItemPubUse => {
                set.insert(array.type_name);
            }
            ImportFrom::Items | ImportFrom::ItemsConstructors => {}
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
        s.wln(format!("{e},"));
    }

    s.dec_indent();
    s.wln("};");
    s.newline();
}

fn struct_definition(
    s: &mut Writer,
    fields: &[Field],
    arrays: &[Array],
    ty_name: &str,
    optimizations: &Optimizations,
) {
    s.wln("#[derive(Debug, Copy, Clone)]");
    s.open_curly(format!("pub struct {ty_name}"));

    for e in fields {
        if optimizations.optimization(e.name).skip_field() {
            continue;
        }

        s.wln(format!("{}: {},", e.name, optimizations.type_name(e)));
    }

    for array in arrays {
        s.wln(format!(
            "{}: [{}; {}],",
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

fn impl_block(
    s: &mut Writer,
    fields: &[Field],
    arrays: &[Array],
    ty_name: &str,
    optimizations: &Optimizations,
) {
    s.open_curly(format!("impl {ty_name}"));

    s.pub_const_fn_new(
        |s| {
            for e in fields {
                if optimizations.optimization(e.name).skip_field() {
                    continue;
                }

                s.wln(format!("{}: {},", e.name, optimizations.type_name(e)));
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
                if optimizations.optimization(e.name).skip_field() {
                    continue;
                }

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

    getters_and_setters(s, fields, arrays, optimizations);

    s.closing_curly();
}

fn getters_and_setters(
    s: &mut Writer,
    fields: &[Field],
    arrays: &[Array],
    optimizations: &Optimizations,
) {
    for field in fields {
        s.pub_const_fn(field.name, field.value.type_name(), |s| match optimizations
            .optimization(field.name)
        {
            FieldOptimization::None => {
                s.wln(format!(
                    "self.{name}{extra}",
                    name = field.name,
                    extra = if let Some(v) = optimizations.native_integer_type_cast(field) {
                        format!(" as {v}")
                    } else {
                        "".to_string()
                    }
                ));
            }
            FieldOptimization::ConstantValue(v) => {
                s.wln(v.to_string_value());
            }
            FieldOptimization::Baseline(mainline, outliers) => {
                s.open_curly(format!("match self.entry"));

                for outlier in outliers {
                    if outlier.0.len() == 1 {
                        s.wln(format!(
                            "{} => {},",
                            outlier.0[0],
                            outlier.1.to_string_value()
                        ));
                    } else {
                        for (i, entry) in outlier.0.iter().enumerate() {
                            if i == 0 {
                                s.w(format!("{entry}"))
                            } else {
                                s.w_no_indent(format!(" | {entry}"))
                            }
                        }
                        s.wln_no_indent(format!(" => {},", outlier.1.to_string_value()))
                    }
                }
                s.wln(format!("_ => {},", mainline.to_string_value()));

                s.closing_curly();
            }
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
