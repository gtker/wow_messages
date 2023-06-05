use crate::base_printer::data::items::{Array, Field, FieldOptimization, Optimizations};
use crate::base_printer::{Expansion, ImportFrom};
use crate::rust_printer::Writer;
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

    for (field_index, e) in fields.iter().enumerate() {
        if matches!(
            import_location,
            ImportFrom::ItemsConstructors | ImportFrom::Items
        ) && optimizations.optimization(field_index).skip_field()
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
            ImportFrom::Items | ImportFrom::ItemPubUse | ImportFrom::ItemsConstructors => {
                set.insert(array.type_name);
            }
            ImportFrom::Definition => {
                if array.import_only {
                    set.insert(array.type_name);
                }
            }
        }

        for e in array.field_info().fields() {
            match import_location {
                ImportFrom::ItemPubUse | ImportFrom::Items | ImportFrom::Definition => {
                    if let Some(name) = e.value.import_name() {
                        set.insert(name);
                    }
                }
                ImportFrom::ItemsConstructors => {}
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
    let lower_ty = ty_name.to_lowercase();
    s.wln(format!(
        "/// Struct optimized for containing the original {lower_ty}s most efficiently."
    ));
    s.wln("///");
    s.wln(format!("/// This type is not supposed to be used by external users of the library for creating custom {lower_ty}s."));
    s.wln(format!(
        "/// It's only supposed to be used in conjunction with the `wow_{lower_ty}s` crate."
    ));
    s.wln("///");
    s.wln(format!("/// [`Hash`](core::hash::Hash), [`Ord`], and [`Eq`] all use only the {lower_ty} id without considering other fields."));

    s.wln("#[derive(Debug, Copy, Clone, Default)]");
    s.open_curly(format!("pub struct {ty_name}"));

    for (field_index, e) in fields.iter().enumerate() {
        if optimizations.optimization(field_index).skip_field() {
            continue;
        }

        s.wln(format!(
            "{}: {},",
            e.name,
            optimizations.type_name(e, field_index)
        ));
    }

    for array in arrays {
        s.wln(format!(
            "{}: &'static [{}],",
            array.variable_name, array.type_name,
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

        s.wln("#[derive(Debug, Copy, Clone, Default)]");
        s.wln("#[cfg_attr(feature = \"serde\", derive(serde::Deserialize, serde::Serialize))]");
        s.open_curly(format!("pub struct {}", array.type_name));

        for e in array.field_info().fields() {
            s.wln(format!("pub {}: {},", e.name, e.value.type_name()));
        }

        s.closing_curly();
        s.newline();

        s.open_curly(format!("impl {}", array.type_name));

        s.pub_const_fn_new(
            |s| {
                for e in array.field_info().fields() {
                    s.wln(format!("{}: {},", e.name, e.value.type_name()));
                }
            },
            |s| {
                for e in array.field_info().fields() {
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
    s.bodyn(format!("impl {ty_name}"), |s| {
        s.wln("#[doc(hidden)]");
        s.pub_const_fn_new(
            |s| {
                for (field_index, e) in fields.iter().enumerate() {
                    if optimizations.optimization(field_index).skip_field() {
                        continue;
                    }

                    s.wln(format!(
                        "{}: {},",
                        e.name,
                        optimizations.type_name(e, field_index)
                    ));
                }

                for array in arrays {
                    s.wln(format!(
                        "{}: &'static [{}],",
                        array.variable_name, array.type_name,
                    ));
                }
            },
            |s| {
                for (field_index, e) in fields.iter().enumerate() {
                    if optimizations.optimization(field_index).skip_field() {
                        continue;
                    }

                    s.wln(format!("{},", e.name));
                }

                for array in arrays {
                    s.wln(format!("{},", array.variable_name,));
                }
            },
        );

        getters_and_setters(s, fields, arrays, optimizations);
    });

    s.bodyn(format!("impl PartialOrd for {ty_name}"), |s| {
        s.body(
            "fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>",
            |s| s.wln("Some(self.cmp(other))"),
        );
    });

    s.bodyn(format!("impl Ord for {ty_name}"), |s| {
        s.body("fn cmp(&self, other: &Self) -> core::cmp::Ordering", |s| {
            s.wln("self.entry.cmp(&other.entry)")
        });
    });

    s.bodyn(format!("impl PartialEq<Self> for {ty_name}"), |s| {
        s.body("fn eq(&self, other: &Self) -> bool", |s| {
            s.wln("self.entry.eq(&other.entry)")
        });
    });

    s.wln(format!("impl Eq for {ty_name} {{}}"));

    s.bodyn(format!("impl core::hash::Hash for {ty_name}"), |s| {
        s.body("fn hash<H: std::hash::Hasher>(&self, state: &mut H)", |s| {
            s.wln("self.entry.hash(state)");
        });
    });
}

fn getters_and_setters(
    s: &mut Writer,
    fields: &[Field],
    arrays: &[Array],
    optimizations: &Optimizations,
) {
    for (field_index, field) in fields.iter().enumerate() {
        let opt = optimizations.optimization(field_index);

        match &opt {
            FieldOptimization::None => {}
            FieldOptimization::ConstantValue(v) => {
                s.wln(format!("/// Always returns `{}`.", v.to_string_value()));
            }
            FieldOptimization::Baseline(v, _) => {
                s.wln(format!(
                    "/// Returns `{}` except for specific item entries.",
                    v.to_string_value()
                ));
            }
        }

        s.pub_const_fn(field.name, field.value.type_name(), |s| match &opt {
            FieldOptimization::None => {
                s.wln(format!(
                    "self.{name}{extra}",
                    name = field.name,
                    extra = if let Some(v) = optimizations.native_integer_type_cast(field_index) {
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
                if let Some(extra) = mainline.definition_has_extra() {
                    s.wln(format!("{extra}(match self.entry {{"));
                    s.inc_indent();
                } else {
                    s.open_curly("match self.entry");
                }

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

                if mainline.definition_has_extra().is_some() {
                    s.wln("})");
                    s.dec_indent();
                } else {
                    s.closing_curly();
                }
            }
        });
        s.newline();
    }

    for array in arrays {
        s.pub_const_fn(
            format!("{}_array", array.variable_name),
            format!("[{}; {}]", array.type_name, array.array_length()),
            |s| {
                s.wln(format!(
                    "const D: {ty_name}={ty_name}{{",
                    ty_name = array.type_name
                ));
                s.inc_indent();

                for field in array.field_info().fields() {
                    s.wln(format!(
                        "{}:{},",
                        field.name,
                        field.value.default_value().to_string_value()
                    ));
                }

                s.dec_indent();
                s.wln("};");

                s.wln(format!("let l = self.{}.len();", array.variable_name));

                s.wln("[");
                s.inc_indent();

                for i in 0..array.array_length() {
                    s.wln(format!(
                        "if l >= {} {{ self.{}()[{i}] }} else {{ D }},",
                        i + 1,
                        array.variable_name
                    ));
                }

                s.dec_indent();
                s.wln("]");
            },
        );
        s.newline();

        s.pub_const_fn(
            array.variable_name,
            format!("&[{}]", array.type_name,),
            |s| {
                s.wln(format!("self.{}", array.variable_name));
            },
        );
        s.newline();
    }
}
