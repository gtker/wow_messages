use crate::container::Container;
use crate::parser::types::objects::Objects;
use crate::parser::types::ty::Type;
use crate::parser::types::ArrayType;
use crate::rust_printer::new_enums::{IfStatementType, NewEnumStructMember, NewIfStatement};
use crate::rust_printer::rust_view::RustDefiner;
use crate::rust_printer::structs::print_common_impls;
use crate::rust_printer::structs::print_common_impls::print_write::{
    print_enum_if_statement_new, print_flag_if_statement, print_write_definition, print_write_field,
};
use crate::rust_printer::structs::print_common_impls::{
    print_constant, print_size_of_ty_rust_view,
};
use crate::rust_printer::DefinerType;
use crate::rust_printer::{ImplType, Writer};

pub fn print_new_types(s: &mut Writer, e: &Container, o: &Objects) {
    for rd in e.rust_object().get_rust_definers() {
        match rd.definer_type() {
            DefinerType::Enum => {
                print_new_enum_declaration(s, &rd);

                print_from_new_enum_to_old(s, &rd);

                print_from_old_enum_to_new(s, &rd);

                print_default_for_new_enum(s, &rd);

                s.bodyn(format!("impl {name}", name = rd.ty_name()), |s| {
                    print_write_for_new_enum(s, &rd);
                });
                print_size_for_new_enum(s, &rd);
            }
            DefinerType::Flag => {
                print_new_flag_declaration(s, &rd);

                print_from_new_flag_to_old(s, &rd);

                s.body(format!("impl {name}", name = rd.ty_name()), |s| {
                    print_write_for_new_flag(s, &rd);
                    print_constructors_for_new_flag(s, &rd);
                });
                print_size_for_new_flag(s, &rd);

                print_types_for_new_flag(s, e, o, &rd);
            }
        }
    }
}

fn print_new_flag_declaration(s: &mut Writer, rd: &RustDefiner) {
    s.wln("#[derive(Default, Debug, PartialEq, Clone)]");
    s.new_flag(rd.ty_name(), rd.int_ty().rust_str(), |s| {
        for enumerator in rd.enumerators() {
            if !enumerator.should_not_be_in_flag_types() {
                s.wln(format!(
                    "{variable_name}: Option<{ce_name}{f_name}>,",
                    variable_name = enumerator.name().to_lowercase(),
                    ce_name = rd.ty_name(),
                    f_name = enumerator.name(),
                ));
            }
        }
    });
}

fn print_from_new_flag_to_old(s: &mut Writer, rd: &RustDefiner) {
    s.impl_from(format!("&{}", rd.ty_name()), rd.original_ty_name(), |s| {
        s.wln("Self::new(e.inner)");
    });
}

fn print_write_for_new_flag(s: &mut Writer, rd: &RustDefiner) {
    s.async_funcn_pub(
        "write",
        "<W: std::io::Write>(&self, w: &mut W)",
        "<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W)",
        "<W: WriteExt + Unpin + Send>(&self, w: &mut W)",
        "std::result::Result<(), std::io::Error>",
        |s, it| {
            s.wln(format!(
                "let a: {ty} = self.into();",
                ty = rd.original_ty_name(),
            ));
            s.wln(format!(
                "a.{prefix}write(w){postfix}?;",
                prefix = it.prefix(),
                postfix = it.postfix()
            ));
            s.wln("Ok(())");
        },
    );
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
            let new_ty = format!("{}{}", rd.ty_name(), enumerator.name());
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
                        s.wln(format!(
                            "inner: {parent}::{name},",
                            parent = rd.original_ty_name(),
                            name = enumerator.name()
                        ));

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
                    s.wln(format!(
                        "self.inner |= {ty}::{name};",
                        ty = rd.original_ty_name(),
                        name = enumerator.name()
                    ));

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
            format!("clear_{}(&mut self)", enumerator.name()),
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
                s.wln("// TODO: Cloning like this is not conductive to good performance but it is");
                s.wln("// temporarily necessary due to test syntax");
                s.wln("self.clone()");
            },
        );
    }
}

fn print_size_for_new_flag(s: &mut Writer, rd: &RustDefiner) {
    s.variable_size(
        rd.ty_name(),
        |s| {
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
        },
        |s| {
            s.wln(format!("{size} // inner", size = rd.int_ty().size(),));

            for enumerator in rd.enumerators() {
                if enumerator.should_not_be_in_flag_types() {
                    continue;
                }

                s.wln(format!(
                    "+ {ce}{f}::maximum_possible_size() // {f} enumerator",
                    ce = rd.ty_name(),
                    f = enumerator.name()
                ));
            }
        },
    );
}

fn print_types_for_new_flag_flag_elseif(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    ne: &NewIfStatement,
    enumerator_name: &str,
) {
    let new_ty_name = format!("{}{}", ne.new_ty_name(), enumerator_name);

    s.wln("#[derive(Debug, PartialEq, Clone)]");
    s.new_enum(
        "pub",
        format!(
            "{new_ty_name}{enumerator}",
            new_ty_name = ne.new_ty_name(),
            enumerator = enumerator_name
        ),
        |s| {
            for enumerator in ne.enumerators() {
                if enumerator.fields().is_empty() {
                    continue;
                }

                s.open_curly(enumerator.name());
                for d in enumerator.get_variable_names_for_members() {
                    if d.used_as_size_in().is_some() {
                        continue;
                    }
                    if d.verified_value().is_some() {
                        continue;
                    } else {
                        s.wln(format!(
                            "{name}: {ty},",
                            name = d.name(),
                            ty = d.ty().rust_str()
                        ));
                    };
                }
                s.closing_curly_with(",");
            }
        },
    );

    s.variable_size(
        &new_ty_name,
        |s| {
            s.open_curly("match self");

            for enumerator in ne.enumerators() {
                if enumerator.fields().is_empty() {
                    continue;
                }

                s.open_curly(format!(
                    "Self::{enumerator} =>",
                    enumerator = enumerator.name()
                ));

                let mut printed_statements = Vec::new();
                for (i, sf) in enumerator.fields().iter().enumerate() {
                    let sf = match sf {
                        NewEnumStructMember::Definition(d) => d,
                        NewEnumStructMember::IfStatement(statement) => {
                            if printed_statements.contains(&statement.variable_name()) {
                                continue;
                            }
                            printed_statements.push(statement.variable_name());

                            match i != 0 {
                                true => s.w("+ "),
                                false => s.w(""),
                            }

                            s.wln_no_indent(format!(
                                "{name}.size() // {name}",
                                name = statement.variable_name()
                            ));
                            continue;
                        }
                    };

                    match i != 0 {
                        true => s.w("+ "),
                        false => s.w(""),
                    }

                    let array_inner_constant = match sf.ty() {
                        Type::Array(array) => match array.ty() {
                            ArrayType::Integer(_) => true,
                            ArrayType::Complex(ident) => {
                                o.type_has_constant_size(&Type::Identifier {
                                    s: ident.clone(),
                                    upcast: None,
                                })
                            }
                            ArrayType::CString => false,
                            ArrayType::Guid => true,
                            ArrayType::PackedGuid => false,
                        },
                        _ => false,
                    };

                    print_common_impls::print_size_of_ty(
                        s,
                        sf.ty(),
                        sf.name(),
                        true,
                        o.type_has_constant_size(sf.ty()),
                        array_inner_constant,
                        "self.",
                        &sf.ty().str(),
                    );
                }

                s.closing_curly();
            }

            s.closing_curly();
        },
        |s| {
            s.wln("65536 // TODO Flag elseif");
        },
    );

    s.open_curly(format!("impl {new_ty_name}", new_ty_name = &new_ty_name));
    s.async_funcn_pub(
        "write",
        "<W: std::io::Write>(&self, w: &mut W)",
        "<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W)",
        "<W: WriteExt + Unpin + Send>(&self, w: &mut W)",
        "std::result::Result<(), std::io::Error>",
        |s, it| {
            s.open_curly("match &self");
            for enumerator in ne.enumerators() {
                if enumerator.fields().is_empty() {
                    continue;
                }

                s.open_curly(format!(
                    "Self::{enumerator}",
                    enumerator = enumerator.name()
                ));
                for f in enumerator.fields() {
                    match f {
                        NewEnumStructMember::Definition(d) => {
                            s.wln(format!("{name},", name = d.name()));
                        }
                        NewEnumStructMember::IfStatement(statement) => {
                            s.wln(format!("{name},", name = statement.variable_name()));
                        }
                    }
                }
                s.closing_curly_with(" => {");
                s.inc_indent();

                for f in enumerator.fields() {
                    match f {
                        NewEnumStructMember::Definition(d) => {
                            print_write_definition(s, e, o, "", d, it.prefix(), it.postfix());
                        }
                        NewEnumStructMember::IfStatement(statement) => {
                            match statement.enum_or_flag() {
                                IfStatementType::Enum => {
                                    print_enum_if_statement_new(
                                        s,
                                        e,
                                        o,
                                        "",
                                        statement,
                                        it.prefix(),
                                        it.postfix(),
                                    );
                                }
                                IfStatementType::Flag => print_flag_if_statement(
                                    s,
                                    "",
                                    statement,
                                    it.prefix(),
                                    it.postfix(),
                                ),
                            }
                        }
                    }
                }

                s.closing_curly();
            }

            s.closing_curly();

            s.newline();
            s.wln("Ok(())");
        },
    );

    s.closing_curly();
    s.newline();
}

fn print_types_for_new_flag(s: &mut Writer, e: &Container, o: &Objects, rd: &RustDefiner) {
    for enumerator in rd.complex_flag_enumerators() {
        if let Some(ne) = e.complex_enum_enumerator_has_else_if(enumerator.name()) {
            print_types_for_new_flag_flag_elseif(s, e, o, ne, enumerator.name());
            continue;
        }

        let new_type_name = format!("{}{}", rd.ty_name(), enumerator.name());
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

        s.variable_size(
            &new_type_name,
            |s| {
                for (i, m) in enumerator.members().iter().enumerate() {
                    if i != 0 {
                        s.w("+ ");
                    } else {
                        s.w("");
                    }

                    print_size_of_ty_rust_view(s, m, "self.");
                }
            },
            |s| {
                for (i, m) in enumerator.members().iter().enumerate() {
                    if i != 0 {
                        s.w("+ ");
                    } else {
                        s.w("");
                    }

                    s.w_no_indent(format!("{}", m.sizes().maximum()));

                    s.wln_no_indent(format!(
                        " // {name}: {ty}",
                        name = m.name(),
                        ty = m.ty().str()
                    ));
                }
            },
        );

        s.bodyn(
            format!("impl {}", new_type_name),
            |s| {

                for m in enumerator.original_fields() {
                    print_constant(s, m);
                }

                for it in ImplType::types() {

                    s.wln(it.cfg());
                    let header = format!("pub {func}fn {prefix}write<W: {write}>(&self, w: &mut W) -> std::result::Result<(), std::io::Error>", func = it.func(), prefix = it.prefix(), write = it.write());
                    s.bodyn(header, |s| {
                        for m in enumerator.original_fields() {
                            print_write_field(s, e, o, m, "self.", it.prefix(), it.postfix());
                        }

                        s.wln("Ok(())");
                    });
                }
            },
        );
    }
}

fn print_new_enum_declaration(s: &mut Writer, rd: &RustDefiner) {
    s.wln("#[derive(Debug, PartialEq, Clone)]");
    s.new_enum("pub", rd.ty_name(), |s| {
        for enumerator in rd.enumerators() {
            s.w(format!("{}", enumerator.name()));

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

fn print_from_new_enum_to_old(s: &mut Writer, rd: &RustDefiner) {
    s.impl_from(format!("&{}", rd.original_ty_name()), rd.ty_name(), |s| {
        s.body("match &e", |s| {
            for enumerator in rd.enumerators() {
                if !enumerator.has_members_in_struct() {
                    s.wln(format!(
                        "{original}::{field} => Self::{field},",
                        original = rd.original_ty_name(),
                        field = enumerator.name(),
                    ));
                    continue;
                }

                s.body_closing_with(
                    format!(
                        "{original}::{field} => Self::{field}",
                        original = rd.original_ty_name(),
                        field = enumerator.name(),
                    ),
                    |s| {
                        for m in enumerator.members_in_struct() {
                            s.wln(format!("{name}: Default::default(),", name = m.name()))
                        }
                    },
                    ",",
                );
            }
        });
    });
}

fn print_from_old_enum_to_new(s: &mut Writer, rd: &RustDefiner) {
    s.bodyn(
        format!(
            "impl From<&{new}> for {original}",
            new = rd.ty_name(),
            original = rd.original_ty_name(),
        ),
        |s| {
            s.body(
                format!("fn from(v: &{new}) -> Self", new = rd.ty_name()),
                |s| {
                    s.body("match &v", |s| {
                        for enumerator in rd.enumerators() {
                            if enumerator.has_members_in_struct() {
                                s.wln(format!(
                                    "{new}::{field} {{ .. }} => Self::{field},",
                                    new = rd.ty_name(),
                                    field = enumerator.name(),
                                ));
                            } else {
                                s.wln(format!(
                                    "{new}::{field} => Self::{field},",
                                    new = rd.ty_name(),
                                    field = enumerator.name(),
                                ));
                            }
                        }
                    });
                },
            );
        },
    );
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

fn print_write_for_new_enum(s: &mut Writer, rd: &RustDefiner) {
    s.async_funcn_pub(
        "write",
        "<W: std::io::Write>(&self, w: &mut W)",
        "<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W)",
        "<W: WriteExt + Unpin + Send>(&self, w: &mut W)",
        "std::result::Result<(), std::io::Error>",
        |s, it| {
            s.wln(format!(
                "let a: {ty} = self.into();",
                ty = rd.original_ty_name(),
            ));
            s.wln(format!(
                "a.{prefix}write(w){postfix}?;",
                prefix = it.prefix(),
                postfix = it.postfix()
            ));
            s.wln("Ok(())");
        },
    );

    s.funcn("as_int(&self)", rd.int_ty().rust_str(), |s| {
        s.wln(format!(
            "let a: {ty} = self.into();",
            ty = rd.original_ty_name(),
        ));
        s.wln(format!("a.as_int() as {ty}", ty = rd.int_ty().rust_str()));
    });
}

fn print_size_for_new_enum(s: &mut Writer, re: &RustDefiner) {
    s.variable_size(
        re.ty_name(),
        |s| {
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
                        s.wln("0 // Not an actual enum sent over the wire");
                    } else {
                        s.wln(format!("{}", re.int_ty().size()));
                    }

                    for m in enumerator.members() {
                        s.w("+ ");

                        print_size_of_ty_rust_view(s, m, "");
                    }
                    s.closing_curly();
                }
            });
        },
        |s| {
            let sizes = re.inner().sizes();
            if sizes.maximum() > u16::MAX.into() {
                s.wln(format!(
                    "{} // Capped at u16::MAX due to size field.",
                    u16::MAX
                ));
            } else {
                s.wln(format!("{}", sizes.maximum()))
            }
        },
    );
}
