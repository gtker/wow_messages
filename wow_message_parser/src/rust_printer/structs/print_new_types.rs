use crate::container::Container;
use crate::rust_printer::rust_view::RustDefiner;
use crate::rust_printer::structs::print_common_impls::print_size_of_ty_rust_view;
use crate::rust_printer::Writer;
use crate::rust_printer::{get_new_flag_type_name, DefinerType};

pub fn print_new_types(s: &mut Writer, e: &Container) {
    for rd in e.rust_object().get_rust_definers() {
        match rd.definer_type() {
            DefinerType::Enum => {
                print_new_enum_declaration(s, &rd);

                if !rd.is_elseif() {
                    print_default_for_new_enum(s, &rd);
                }

                s.bodyn(format!("impl {name}", name = rd.ty_name()), |s| {
                    print_enum_as_int(s, &rd);
                });
                print_size_for_new_enum(s, &rd);
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
    s.wln("#[derive(Default, Debug, PartialEq, Clone)]");
    s.new_flag(rd.ty_name(), rd.int_ty().rust_str(), |s| {
        for enumerator in rd.enumerators() {
            if !enumerator.should_not_be_in_flag_types() {
                s.wln(format!(
                    "{variable_name}: Option<{ty_name}>,",
                    variable_name = enumerator.name().to_lowercase(),
                    ty_name = get_new_flag_type_name(rd.ty_name(), enumerator.name()),
                ));
            }
        }
    });
}

fn print_constructors_for_new_flag(s: &mut Writer, rd: &RustDefiner) {
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

    for enumerator in rd.enumerators() {
        if !enumerator.has_members_in_struct() {
            s.funcn_pub_const(format!("new_{}()", enumerator.name()), "Self", |s| {
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
            });

            s.funcn_pub(
                format!("set_{}(&mut self)", enumerator.name()),
                "Self",
                |s| {
                    s.wln(format!(
                        "self.inner |= {ty}::{name};",
                        ty = rd.original_ty_name(),
                        name = enumerator.name()
                    ));

                    s.wln("self.clone()");
                },
            );

            s.funcn_pub_const(format!("get_{}(&self)", enumerator.name()), "bool", |s| {
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
            });
        } else {
            let new_ty = get_new_flag_type_name(rd.ty_name(), enumerator.name());
            s.funcn_pub_const(
                format!(
                    "new_{upper_name}({lower_name}: {new_ty})",
                    upper_name = enumerator.name(),
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

            s.funcn_pub(
                format!(
                    "set_{upper_name}(&mut self, {lower_name}: {new_ty})",
                    upper_name = enumerator.name(),
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

                    s.wln("self.clone()");
                },
            );

            s.funcn_pub_const(
                format!("get_{}(&self)", enumerator.name()),
                format!("Option<&{new_ty}>", new_ty = new_ty),
                |s| {
                    s.wln(format!(
                        "self.{}.as_ref()",
                        enumerator.name().to_lowercase()
                    ));
                },
            );
        }
        s.funcn_pub(
            format!("clear_{}(mut self)", enumerator.name()),
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
    s.variable_size(rd.ty_name(), |s| {
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
                        s.wln("s.size()");
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

        let new_type_name = get_new_flag_type_name(rd.ty_name(), enumerator.name());
        s.wln("#[derive(Debug, PartialEq, Clone)]");
        s.new_struct(&new_type_name, |s| {
            for m in enumerator.members_in_struct() {
                s.wln(format!(
                    "pub {name}: {ty},",
                    name = m.name(),
                    ty = m.ty().rust_str(),
                ));
            }
        });

        s.variable_size(&new_type_name, |s| {
            for (i, m) in enumerator.members().iter().enumerate() {
                if i != 0 {
                    s.w("+ ");
                } else {
                    s.w("");
                }

                print_size_of_ty_rust_view(s, m, "self.");
            }
        });
    }
}

fn print_new_enum_declaration(s: &mut Writer, rd: &RustDefiner) {
    s.wln("#[derive(Debug, PartialEq, Clone)]");
    s.new_enum("pub", rd.ty_name(), |s| {
        for enumerator in rd.enumerators() {
            s.w(enumerator.name());

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
    s.bodyn(
        format!("impl Default for {name}", name = rd.ty_name()),
        |s| {
            s.body("fn default() -> Self", |s| {
                s.wln("// First enumerator without any fields");
                let enumerator = rd.enumerators().first().unwrap();
                if enumerator.has_members_in_struct() {
                    s.open_curly(format!("Self::{}", enumerator.name()));

                    for m in enumerator.members_in_struct() {
                        s.wln(format!("{name}: Default::default(),", name = m.name()));
                    }

                    s.closing_curly();
                } else {
                    s.wln(format!("Self::{}", enumerator.name()));
                }
            });
        },
    );
}

fn print_size_for_new_enum(s: &mut Writer, re: &RustDefiner) {
    s.variable_size(re.ty_name(), |s| {
        s.body("match self", |s| {
            for enumerator in re.enumerators() {
                if enumerator.has_members_in_struct() {
                    s.open_curly(format!("Self::{name}", name = enumerator.name()));
                    for m in enumerator.members_in_struct() {
                        s.wln(format!("{},", m.name()));
                    }
                    s.closing_curly_with(" => {");
                    s.inc_indent();
                } else {
                    s.open_curly(format!("Self::{name} =>", name = enumerator.name()));
                }

                if re.is_elseif() {
                    s.wln("// Not an actual enum sent over the wire");
                } else {
                    s.wln(format!("{}", re.int_ty().size()));
                }

                for (i, m) in enumerator.members().iter().enumerate() {
                    if i == 0 && re.is_elseif() {
                        s.w("");
                    } else {
                        s.w("+ ");
                    }

                    print_size_of_ty_rust_view(s, m, "");
                }
                s.closing_curly();
            }
        });
    });
}

fn print_enum_as_int(s: &mut Writer, rd: &RustDefiner) {
    s.funcn_const("as_int(&self)", rd.int_ty().rust_str(), |s| {
        s.body("match self", |s| {
            for enumerator in rd.enumerators() {
                s.wln(format!(
                    "Self::{enumerator}{extras} => {value},",
                    enumerator = enumerator.name(),
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
