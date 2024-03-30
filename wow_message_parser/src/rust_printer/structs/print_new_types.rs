use crate::parser::types::array::ArraySize;
use crate::parser::types::container::Container;
use crate::rust_printer::rust_view::rust_definer::RustDefiner;
use crate::rust_printer::rust_view::rust_type::RustType;
use crate::rust_printer::structs::print_common_impls::print_size::{
    print_rust_members_sizes, variable_size,
};
use crate::rust_printer::structs::print_derives;
use crate::rust_printer::writer::Writer;
use crate::rust_printer::{get_new_flag_type_name, DefinerType};

pub(crate) fn print_new_types(s: &mut Writer, e: &Container) {
    for rd in e.rust_object().get_rust_definers() {
        match rd.definer_type() {
            DefinerType::Enum => {
                if !rd.is_single_rust_definer() {
                    print_new_enum_declaration(s, &rd, rd.ty_name());
                }

                if !rd.is_elseif() {
                    print_default_for_new_enum(s, &rd);
                }

                let ty_name = rd.ty_name();
                s.bodyn(format!("impl {ty_name}"), |s| {
                    print_enum_as_int(s, &rd);
                });
                print_enum_display(s, &rd);

                if !rd.is_single_rust_definer() {
                    print_size_for_new_enum(s, &rd);
                }
            }
            DefinerType::Flag => {
                print_new_flag_declaration(s, &rd);

                s.body(format!("impl {name}", name = rd.ty_name()), |s| {
                    print_constructors_for_new_flag(s, &rd);
                    print_flag_as_int(s, &rd);
                });
                print_size_for_new_flag(s, &rd);

                print_types_for_new_flag(s, &rd);
            }
        }
    }
}

fn print_flag_as_int(s: &mut Writer, rd: &RustDefiner) {
    s.funcn_const("as_int(&self)", rd.int_ty().rust_str(), |s| {
        s.wln("self.inner");
    });
}

fn print_new_flag_declaration(s: &mut Writer, rd: &RustDefiner) {
    print_derives(s, &rd.all_members(), false);
    s.new_flag(rd.ty_name(), rd.int_ty().rust_str(), |s| {
        for enumerator in rd.enumerators() {
            if !enumerator.should_not_be_in_flag_types() {
                s.wln(format!(
                    "{variable_name}: Option<{ty_name}>,",
                    variable_name = enumerator.name().to_lowercase(),
                    ty_name = get_new_flag_type_name(rd.ty_name(), enumerator.rust_name()),
                ));
            }
        }
    });
}

fn print_constructors_for_new_flag(s: &mut Writer, rd: &RustDefiner) {
    use std::fmt::Write;

    let mut function_name = format!("new(inner: {ty}, ", ty = rd.int_ty().rust_str());

    for enumerator in rd.enumerators() {
        if !enumerator.should_not_be_in_flag_types() {
            write!(
                function_name,
                "{variable_name}: Option<{ty_name}>,",
                variable_name = enumerator.name().to_lowercase(),
                ty_name = get_new_flag_type_name(rd.ty_name(), enumerator.rust_name())
            )
            .unwrap();
        }
    }

    write!(function_name, ")").unwrap();

    s.funcn_pub_const(function_name, "Self", |s| {
        s.body("Self", |s| {
            s.wln("inner,");

            for enumerator in rd.enumerators() {
                if !enumerator.should_not_be_in_flag_types() {
                    s.wln(format!(
                        "{variable_name}, ",
                        variable_name = enumerator.name().to_lowercase(),
                    ));
                }
            }
        });
    });

    s.funcn_pub_const("empty()", "Self", |s| {
        s.body("Self", |s| {
            s.wln("inner: 0,");
            for enumerator in rd.complex_flag_enumerators() {
                s.wln(format!(
                    "{name}: None,",
                    name = enumerator.name().to_lowercase()
                ))
            }
        });
    });

    s.funcn_pub_const("is_empty(&self)", "bool", |s| {
        s.wln("self.inner == 0");
        for enumerator in rd.complex_flag_enumerators() {
            s.wln(format!(
                "&& self.{name}.is_none()",
                name = enumerator.name().to_lowercase()
            ))
        }
    });

    const CLIPPY_MISSING_FN: &str = "#[allow(clippy::missing_const_for_fn)] // false positive";

    for enumerator in rd.enumerators() {
        if enumerator.value().int() == 0 {
            continue;
        }

        if !enumerator.has_members_in_struct() {
            s.funcn_pub_const(
                format!("new_{}()", enumerator.name().to_lowercase()),
                "Self",
                |s| {
                    s.body("Self", |s| {
                        s.wln(format!(
                            "inner: {parent}::{name},",
                            parent = rd.original_ty_name(),
                            name = enumerator.name()
                        ));

                        for inner_enumerator in rd.complex_flag_enumerators() {
                            s.wln(format!(
                                "{name}: None,",
                                name = inner_enumerator.name().to_lowercase()
                            ));
                        }
                    });
                },
            );

            s.wln(CLIPPY_MISSING_FN);
            s.funcn_pub(
                format!("set_{}(mut self)", enumerator.name().to_lowercase()),
                "Self",
                |s| {
                    s.wln(format!(
                        "self.inner |= {ty}::{name};",
                        ty = rd.original_ty_name(),
                        name = enumerator.name()
                    ));

                    s.wln("self");
                },
            );

            s.funcn_pub_const(
                format!("get_{}(&self)", enumerator.name().to_lowercase()),
                "bool",
                |s| {
                    if enumerator.value().int() == 0 {
                        s.wln("// Underlying value is 0");
                        s.wln(format!(
                            "self.inner == {ty}::{name}",
                            ty = rd.original_ty_name(),
                            name = enumerator.name()
                        ));
                    } else {
                        s.wln(format!(
                            "(self.inner & {ty}::{name}) != 0",
                            ty = rd.original_ty_name(),
                            name = enumerator.name()
                        ));
                    }
                },
            );
        } else {
            let new_ty = get_new_flag_type_name(rd.ty_name(), enumerator.rust_name());
            s.funcn_pub_const(
                format!(
                    "new_{lower_name}({lower_name}: {new_ty})",
                    lower_name = enumerator.name().to_lowercase(),
                    new_ty = new_ty,
                ),
                "Self",
                |s| {
                    s.body("Self", |s| {
                        if enumerator.contains_elseif() {
                            s.wln(format!(
                                "inner: {lower_name}.as_int(),",
                                lower_name = enumerator.name().to_lowercase(),
                            ));
                        } else {
                            s.wln(format!(
                                "inner: {parent}::{name},",
                                parent = rd.original_ty_name(),
                                name = enumerator.name()
                            ));
                        }

                        for inner_enumerator in rd.complex_flag_enumerators() {
                            if inner_enumerator.name() == enumerator.name() {
                                s.wln(format!(
                                    "{name}: Some({name}),",
                                    name = inner_enumerator.name().to_lowercase()
                                ));
                            } else {
                                s.wln(format!(
                                    "{name}: None,",
                                    name = inner_enumerator.name().to_lowercase()
                                ));
                            }
                        }
                    });
                },
            );

            s.wln(CLIPPY_MISSING_FN);
            s.funcn_pub(
                format!(
                    "set_{lower_name}(mut self, {lower_name}: {new_ty})",
                    lower_name = enumerator.name().to_lowercase(),
                    new_ty = new_ty,
                ),
                "Self",
                |s| {
                    if enumerator.contains_elseif() {
                        s.wln(format!(
                            "self.inner |= {lower_name}.as_int();",
                            lower_name = enumerator.name().to_lowercase(),
                        ));
                    } else {
                        s.wln(format!(
                            "self.inner |= {ty}::{name};",
                            ty = rd.original_ty_name(),
                            name = enumerator.name()
                        ));
                    }

                    s.wln(format!(
                        "self.{name_lower} = Some({name_lower});",
                        name_lower = enumerator.name().to_lowercase()
                    ));

                    s.wln("self");
                },
            );

            s.funcn_pub_const(
                format!("get_{}(&self)", enumerator.name().to_lowercase()),
                format!("Option<&{new_ty}>"),
                |s| {
                    s.wln(format!(
                        "self.{}.as_ref()",
                        enumerator.name().to_lowercase()
                    ));
                },
            );
        }

        s.wln(CLIPPY_MISSING_FN);
        s.funcn_pub(
            format!("clear_{}(mut self)", enumerator.name().to_lowercase()),
            "Self",
            |s| {
                s.wln(format!(
                    "self.inner &= {ty}::{name}.reverse_bits();",
                    ty = rd.original_ty_name(),
                    name = enumerator.name()
                ));
                if enumerator.has_members_in_struct() {
                    s.wln(format!("self.{} = None;", enumerator.name().to_lowercase()));
                }
                s.wln("self");
            },
        );
    }
}

fn print_size_for_new_flag(s: &mut Writer, rd: &RustDefiner) {
    variable_size(s, rd.ty_name(), "size", rd.size_is_const_fn(), |s| {
        s.wln(format!("{size} // inner", size = rd.int_ty().size(),));

        for enumerator in rd.enumerators() {
            if enumerator.should_not_be_in_flag_types() {
                continue;
            }

            s.body("+", |s| {
                s.body_else(
                    format!(
                        "if let Some(s) = &self.{name}",
                        name = enumerator.name().to_lowercase()
                    ),
                    |s| {
                        if let Some(size) = enumerator.is_constant() {
                            s.wln(size.to_string());
                        } else {
                            s.wln("s.size()");
                        }
                    },
                    |s| {
                        s.wln("0");
                    },
                );
            });
        }
    });
}

fn print_types_for_new_flag(s: &mut Writer, rd: &RustDefiner) {
    for enumerator in rd.complex_flag_enumerators() {
        if enumerator.contains_elseif() {
            continue;
        }

        let new_type_name = get_new_flag_type_name(rd.ty_name(), enumerator.rust_name());
        print_derives(s, &enumerator.all_members(), false);
        s.new_struct(&new_type_name, |s| {
            for m in enumerator.members_in_struct() {
                s.wln(format!(
                    "pub {name}: {ty},",
                    name = m.name(),
                    ty = m.ty().rust_str(),
                ));
            }
        });

        if enumerator.is_constant().is_none() {
            let const_fn = enumerator
                .members_in_struct()
                .iter()
                .all(|a| a.ty().size_is_const_fn());
            variable_size(s, &new_type_name, "size", const_fn, |s| {
                print_rust_members_sizes(s, enumerator.members(), None, "self.");
            });
        }
    }
}

pub(crate) fn print_new_enum_declaration(s: &mut Writer, rd: &RustDefiner, ty_name: &str) {
    print_derives(s, &rd.all_members(), true);
    s.new_enum("pub", ty_name, |s| {
        for enumerator in rd.enumerators() {
            s.w(enumerator.rust_name());

            if !enumerator.has_members_in_struct() {
                s.wln_no_indent(",");
                continue;
            }

            s.wln_no_indent(" {");
            s.inc_indent();

            for m in enumerator.members_in_struct() {
                s.wln(format!("{name}: {ty},", name = m.name(), ty = m.ty()));
            }
            s.closing_curly_with(",")
        }
    });
}

fn print_default_for_new_enum(s: &mut Writer, rd: &RustDefiner) {
    let ty_name = rd.ty_name();
    s.bodyn(format!("impl Default for {ty_name}"), |s| {
        s.body("fn default() -> Self", |s| {
            s.wln("// First enumerator without any fields");
            let enumerator = if let Some(enumerator) =
                rd.enumerators().iter().find(|a| !a.has_members_in_struct())
            {
                enumerator
            } else {
                rd.enumerators().first().unwrap()
            };

            if enumerator.has_members_in_struct() {
                s.open_curly(format!("Self::{}", enumerator.rust_name()));

                for m in enumerator.members_in_struct() {
                    match m.ty() {
                        RustType::Array { array, .. } => match array.size() {
                            ArraySize::Fixed(v) => {
                                s.wln(format!(
                                    "{name}: [Default::default(); {size}],",
                                    name = m.name(),
                                    size = v
                                ));
                            }
                            _ => s.wln(format!("{name}: Default::default(),", name = m.name())),
                        },
                        _ => s.wln(format!("{name}: Default::default(),", name = m.name())),
                    }
                }

                s.closing_curly();
            } else {
                s.wln(format!("Self::{}", enumerator.rust_name()));
            }
        });
    });
}

pub(crate) fn print_size_for_new_enum_inner(s: &mut Writer, re: &RustDefiner) {
    s.body("match self", |s| {
        for enumerator in re.enumerators() {
            if !enumerator.has_members() {
                continue;
            }

            let name = enumerator.rust_name();
            if enumerator.has_members_in_struct() {
                s.open_curly(format!("Self::{name}"));

                for m in enumerator.members_in_struct() {
                    if m.ty().size_requires_variable() {
                        s.wln(format!("{},", m.name()));
                    }
                }

                if enumerator
                    .members_in_struct()
                    .iter()
                    .any(|a| !a.ty().size_requires_variable())
                {
                    s.wln("..");
                }

                s.closing_curly_with(" => {");
                s.inc_indent();
            } else {
                s.open_curly(format!("Self::{name} =>"));
            }

            if re.is_elseif() {
                s.wln("// Not an actual enum sent over the wire");
            } else {
                s.wln(format!("{}", re.int_ty().size()));
            }

            print_rust_members_sizes(s, enumerator.members(), Some(re.is_elseif()), "");
            s.closing_curly();
        }

        if re.enumerators().iter().any(|a| !a.has_members()) {
            s.wln(format!("_ => {},", re.int_ty().size()));
        }
    });
}

fn print_size_for_new_enum(s: &mut Writer, re: &RustDefiner) {
    variable_size(s, re.ty_name(), "size", re.size_is_const_fn(), |s| {
        print_size_for_new_enum_inner(s, re)
    });
}

fn print_enum_as_int(s: &mut Writer, rd: &RustDefiner) {
    s.funcn_const("as_int(&self)", rd.int_ty().rust_str(), |s| {
        s.body("match self", |s| {
            for enumerator in rd.enumerators() {
                s.wln(format!(
                    "Self::{enumerator}{extras} => {value},",
                    enumerator = enumerator.rust_name(),
                    value = enumerator.value().int(),
                    extras = match enumerator.has_members_in_struct() {
                        true => " { .. }",
                        false => "",
                    },
                ));
            }
        });
    });
}

fn print_enum_display(s: &mut Writer, rd: &RustDefiner) {
    s.impl_for("std::fmt::Display", rd.ty_name(), |s| {
        s.body(
            "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result",
            |s| {
                s.body("match self", |s| {
                    for field in rd.enumerators() {
                        let display = field.rust_name();
                        let extra = if field.has_members_in_struct() {
                            "{ .. }"
                        } else {
                            ""
                        };

                        s.wln(format!(
                            r#"Self::{name}{extra} => f.write_str("{display}"),"#,
                            name = field.rust_name(),
                            display = display,
                        ));
                    }
                });
            },
        );
    });
}
