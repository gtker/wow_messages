use crate::container::Container;
use crate::parser::types::objects::Objects;
use crate::parser::types::ty::Type;
use crate::parser::types::{ArrayType, VerifiedContainerValue};
use crate::rust_printer::complex_print::{ComplexEnum, DefinerType, Enumerator};
use crate::rust_printer::enums::get_upcast_types;
use crate::rust_printer::new_enums::{IfStatementType, NewEnumStructMember, NewIfStatement};
use crate::rust_printer::structs::print_common_impls;
use crate::rust_printer::structs::print_common_impls::print_write;
use crate::rust_printer::structs::print_common_impls::print_write::{
    print_enum_if_statement_new, print_flag_if_statement, print_write_definition,
};
use crate::rust_printer::Writer;

pub fn print_new_types(s: &mut Writer, e: &Container, o: &Objects) {
    for ce in e.nested_types().new_enums() {
        match ce.definer_ty() {
            DefinerType::Enum => {
                print_new_enum_declaration(s, ce);

                print_from_new_enum_to_old(s, ce);

                print_from_old_enum_to_new(s, ce);

                print_default_for_new_enum(s, ce);

                s.bodyn(format!("impl {name}", name = ce.name()), |s| {
                    print_write_for_new_enum(s, ce);
                });

                print_size_for_new_enum(s, ce, e, o);
            }
            DefinerType::Flag => {
                print_new_flag_declaration(s, ce);

                print_from_new_flag_to_old(s, ce);

                s.body(format!("impl {name}", name = ce.name()), |s| {
                    print_write_for_new_flag(s, ce);
                    print_constructors_for_new_flag(s, ce);
                });

                print_size_for_new_flag(s, ce);

                print_types_for_new_flag(s, ce, e, o);
            }
        }
    }
}

fn print_new_flag_declaration(s: &mut Writer, ce: &ComplexEnum) {
    s.wln("#[derive(Default, Debug, PartialEq, Clone)]");
    s.new_flag(ce.name(), ce.ty().rust_str(), |s| {
        for f in ce.fields() {
            if !f.should_not_be_in_type() {
                s.wln(format!(
                    "{variable_name}: Option<{ce_name}{f_name}>,",
                    variable_name = f.name().to_lowercase(),
                    ce_name = ce.name(),
                    f_name = f.name()
                ));
            }
        }
    });
}

fn print_from_new_flag_to_old(s: &mut Writer, ce: &ComplexEnum) {
    s.impl_from(format!("&{}", ce.name()), ce.original_ty_name(), |s| {
        s.wln("Self::new(e.inner)");
    });
}

fn print_write_for_new_flag(s: &mut Writer, ce: &ComplexEnum) {
    s.funcn_pub(
        "write<W: std::io::Write>(&self, w: &mut W)",
        "std::result::Result<(), std::io::Error>",
        |s| {
            s.wln(format!(
                "let a: {ty} = self.into();",
                ty = ce.original_ty_name(),
            ));
            s.wln("a.write(w)?;");
            s.wln("Ok(())");
        },
    );
}

fn print_constructors_for_new_flag(s: &mut Writer, ce: &ComplexEnum) {
    s.funcn_pub_const("empty()", "Self", |s| {
        s.body("Self", |s| {
            s.wln("inner: 0,");
            for f in ce.fields() {
                if !f.should_not_be_in_type() {
                    s.wln(format!("{name}: None,", name = f.name().to_lowercase()))
                }
            }
        });
    });

    for f in ce.fields() {
        if f.is_simple() {
            s.funcn_pub_const(format!("new_{}()", f.name()), "Self", |s| {
                s.body("Self", |s| {
                    s.wln(format!(
                        "inner: {parent}::{name},",
                        parent = ce.original_ty_name(),
                        name = f.name()
                    ));

                    for inner_f in ce.fields() {
                        if !inner_f.should_not_be_in_type() {
                            s.wln(format!(
                                "{name}: None,",
                                name = inner_f.name().to_lowercase()
                            ))
                        }
                    }
                });
            });

            s.funcn_pub(format!("set_{}(&mut self)", f.name()), "Self", |s| {
                s.wln(format!(
                    "self.inner |= {ty}::{name};",
                    ty = ce.original_ty_name(),
                    name = f.name()
                ));

                s.wln("self.clone()");
            });

            s.funcn_pub_const(format!("get_{}(&self)", f.name()), "bool", |s| {
                if f.value().int() == 0 {
                    s.wln("// Underlying value is 0");
                    s.wln(format!(
                        "self.inner == {ty}::{name}",
                        ty = ce.original_ty_name(),
                        name = f.name()
                    ));
                } else {
                    s.wln(format!(
                        "(self.inner & {ty}::{name}) != 0",
                        ty = ce.original_ty_name(),
                        name = f.name()
                    ));
                }
            });
        } else {
            let new_ty = format!(
                "{}{}{}",
                ce.original_struct_name(),
                ce.original_ty_name(),
                f.name()
            );
            s.funcn_pub_const(
                format!(
                    "new_{upper_name}({lower_name}: {new_ty})",
                    upper_name = f.name(),
                    lower_name = f.name().to_lowercase(),
                    new_ty = new_ty,
                ),
                "Self",
                |s| {
                    s.body("Self", |s| {
                        s.wln(format!(
                            "inner: {parent}::{name},",
                            parent = ce.original_ty_name(),
                            name = f.name()
                        ));

                        for inner_f in ce.fields() {
                            if !inner_f.should_not_be_in_type() {
                                if inner_f.name() == f.name() {
                                    s.wln(format!(
                                        "{name}: Some({name}),",
                                        name = inner_f.name().to_lowercase()
                                    ))
                                } else {
                                    s.wln(format!(
                                        "{name}: None,",
                                        name = inner_f.name().to_lowercase()
                                    ))
                                }
                            }
                        }
                    });
                },
            );

            s.funcn_pub(
                format!(
                    "set_{upper_name}(&mut self, {lower_name}: {new_ty})",
                    upper_name = f.name(),
                    lower_name = f.name().to_lowercase(),
                    new_ty = new_ty,
                ),
                "Self",
                |s| {
                    s.wln(format!(
                        "self.inner |= {ty}::{name};",
                        ty = ce.original_ty_name(),
                        name = f.name()
                    ));

                    s.wln(format!(
                        "self.{name_lower} = Some({name_lower});",
                        name_lower = f.name().to_lowercase()
                    ));

                    s.wln("self.clone()");
                },
            );

            s.funcn_pub_const(
                format!("get_{}(&self)", f.name()),
                format!("Option<&{new_ty}>", new_ty = new_ty),
                |s| {
                    s.wln(format!("self.{}.as_ref()", f.name().to_lowercase()));
                },
            );
        }

        s.funcn_pub(format!("clear_{}(&mut self)", f.name()), "Self", |s| {
            s.wln(format!(
                "self.inner &= {ty}::{name}.reverse_bits();",
                ty = ce.original_ty_name(),
                name = f.name()
            ));
            if !f.is_simple() {
                s.wln(format!("self.{} = None;", f.name().to_lowercase()));
            }
            s.wln("// TODO: Cloning like this is not conductive to good performance but it is");
            s.wln("// temporarily necessary due to test syntax");
            s.wln("self.clone()");
        });
    }
}

fn print_size_for_new_flag(s: &mut Writer, ce: &ComplexEnum) {
    s.variable_size(
        ce.name(),
        |s| {
            s.wln(format!(
                "{size} // inner: {ty_name} ({ty})",
                size = ce.ty().size(),
                ty_name = ce.original_ty_name(),
                ty = ce.ty().str()
            ));
            for f in ce.fields() {
                if f.should_not_be_in_type() {
                    continue;
                }

                s.body("+", |s| {
                    s.body_else(
                        format!(
                            "if let Some(s) = &self.{name}",
                            name = f.name().to_lowercase()
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
            s.wln(format!(
                "{size} // inner: {ty_name} ({ty})",
                size = ce.ty().size(),
                ty_name = ce.original_ty_name(),
                ty = ce.ty().str()
            ));
            for f in ce.fields() {
                if f.should_not_be_in_type() {
                    continue;
                }

                s.wln(format!(
                    "+ {ce}{f}::maximum_possible_size() // {f} enumerator",
                    ce = ce.name(),
                    f = f.name()
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
    f: &Enumerator,
) {
    let new_ty_name = format!("{}{}", ne.new_ty_name(), f.name());
    let prefix = "";
    let postfix = "";

    s.wln("#[derive(Debug, PartialEq, Clone)]");
    s.new_enum(
        format!(
            "{new_ty_name}{enumerator}",
            new_ty_name = ne.new_ty_name(),
            enumerator = f.name()
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

                            s.wln(format!(
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
    s.func_pub(
        "write<W: std::io::Write>(&self, w: &mut W)",
        "std::result::Result<(), std::io::Error>",
        |s| {
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
                            s.wln(format!("{name}", name = statement.variable_name()));
                        }
                    }
                }
                s.closing_curly_with(" => {");
                s.inc_indent();

                for f in enumerator.fields() {
                    match f {
                        NewEnumStructMember::Definition(d) => {
                            print_write_definition(s, e, o, "", d, prefix, postfix);
                        }
                        NewEnumStructMember::IfStatement(statement) => {
                            match statement.enum_or_flag() {
                                IfStatementType::Enum => {
                                    print_enum_if_statement_new(
                                        s, e, o, "", statement, prefix, postfix,
                                    );
                                }
                                IfStatementType::Flag => {
                                    print_flag_if_statement(s, "", statement, prefix, postfix)
                                }
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

fn print_types_for_new_flag(s: &mut Writer, ce: &ComplexEnum, e: &Container, o: &Objects) {
    let prefix = "";
    let postfix = "";

    for f in ce.fields() {
        if f.should_not_be_in_type() {
            continue;
        }

        if let Some(ne) = e.complex_enum_enumerator_has_else_if(f.name()) {
            print_types_for_new_flag_flag_elseif(s, e, o, ne, f);
            continue;
        }

        let new_type_name = format!("{}{}", ce.name(), f.name());
        s.wln("#[derive(Debug, PartialEq, Clone)]");
        s.new_struct(&new_type_name, |s| {
            for sf in f.subfields() {
                s.wln(format!(
                    "pub {name}: {ty},",
                    name = sf.name(),
                    ty = sf.ty().rust_str()
                ));
            }
        });

        s.variable_size(
            &new_type_name,
            |s| {
                for (i, sf) in f.subfields().iter().enumerate() {
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
            },
            |s| {
                for (i, sf) in f.subfields().iter().enumerate() {
                    match i != 0 {
                        true => s.w("+ "),
                        false => s.w(""),
                    }
                    let array_type_largest_possible_value =
                        print_common_impls::get_array_type_largest_possible_value(sf.ty(), e);

                    print_common_impls::print_maximum_size_of_type(
                        s,
                        sf.ty(),
                        sf.name(),
                        array_type_largest_possible_value,
                    );
                }
            },
        );

        s.bodyn(
            format!(
                "impl {c_name}{f_name}",
                c_name = ce.name(),
                f_name = f.name()
            ),
            |s| {
                for sf in f.subfields() {
                    if let Some(v) = sf.constant_value() {
                        print_common_impls::print_constant_member(
                            s,
                            sf.name(),
                            sf.ty(),
                            v.as_str(),
                            v.parse().unwrap(),
                        );
                    }
                }

                s.func_pub(
                    "write<W: std::io::Write>(&self, w: &mut W)",
                    "std::result::Result<(), std::io::Error>",
                    |s| {
                        for sf in f.subfields() {
                            match sf.ty() {
                                Type::Integer(int_type) => {
                                    let verified = match sf.constant_value() {
                                        None => None,
                                        Some(v) => {
                                            let parsed_value =
                                                crate::parser::utility::parse_value(v);
                                            if let Ok(v) = parsed_value {
                                                Some(VerifiedContainerValue::new(
                                                    v,
                                                    sf.constant_value().as_ref().unwrap().clone(),
                                                ))
                                            } else {
                                                None
                                            }
                                        }
                                    };
                                    print_write::print_write_field_integer(
                                        s,
                                        sf.name(),
                                        "self.",
                                        int_type,
                                        sf.used_as_size_in(),
                                        &verified,
                                        0,
                                        postfix,
                                    );
                                }
                                Type::FloatingPoint(floating) => {
                                    print_write::print_write_field_floating(
                                        s,
                                        sf.name(),
                                        "self.",
                                        floating,
                                        postfix,
                                    );
                                }
                                Type::CString => {
                                    print_write::print_write_field_cstring(
                                        s,
                                        sf.name(),
                                        "self.",
                                        postfix,
                                    );
                                }
                                Type::String { .. } => {
                                    print_write::print_write_field_string(
                                        s,
                                        sf.name(),
                                        "self.",
                                        postfix,
                                    );
                                }
                                Type::Array(array) => {
                                    print_write::print_write_field_array(
                                        s,
                                        sf.name(),
                                        "self.",
                                        array,
                                        postfix,
                                    );
                                }
                                Type::Identifier { s: identifier, .. } => {
                                    print_write::print_write_field_identifier(
                                        s,
                                        sf.name(),
                                        "self.",
                                        &None,
                                        identifier,
                                        prefix,
                                        postfix,
                                    )
                                }
                                Type::PackedGuid => {
                                    s.wln(format!(
                                        "{variable_prefix}{name}.write_packed(w)?;",
                                        variable_prefix = "self.",
                                        name = sf.name()
                                    ));
                                    s.newline();
                                }
                                Type::Guid | Type::UpdateMask | Type::AuraMask => {
                                    s.wln(format!(
                                        "{variable_prefix}{name}.write(w)?;",
                                        variable_prefix = "self.",
                                        name = sf.name()
                                    ));
                                    s.newline();
                                }
                            }
                        }
                        s.wln("Ok(())");
                    },
                );
            },
        );
    }
}

fn print_new_enum_declaration(s: &mut Writer, ce: &ComplexEnum) {
    s.wln("#[derive(Debug, PartialEq, Clone)]");
    s.new_enum(ce.name(), |s| {
        for f in ce.fields() {
            s.wln(format!(
                "{field_name}{extra}",
                field_name = f.name(),
                extra = match f.is_simple_or_subfields_const() {
                    true => ",",
                    false => " {",
                }
            ));

            if f.is_simple_or_subfields_const() {
                continue;
            } else {
                s.inc_indent();
                for sf in f.subfields() {
                    if sf.used_as_size_in().is_some() {
                        continue;
                    }
                    s.wln(format!(
                        "{name}: {type_name},",
                        name = sf.name(),
                        type_name = sf.ty().rust_str(),
                    ));
                }
                s.closing_curly_with(",");
            }
        }
    });
}

fn print_from_new_enum_to_old(s: &mut Writer, ce: &ComplexEnum) {
    s.impl_from(format!("&{}", ce.original_ty_name()), ce.name(), |s| {
        s.body("match &e", |s| {
            for f in ce.fields() {
                if f.is_simple_or_subfields_const() {
                    s.wln(format!(
                        "{original}::{field} => Self::{field},",
                        original = ce.original_ty_name(),
                        field = f.name(),
                    ));
                } else {
                    s.body_closing_with(
                        format!(
                            "{original}::{field} => Self::{field}",
                            original = ce.original_ty_name(),
                            field = f.name(),
                        ),
                        |s| {
                            for sf in f.subfields() {
                                if sf.used_as_size_in().is_some() || sf.constant_value().is_some() {
                                    continue;
                                }
                                s.wln(format!("{name}: Default::default(),", name = sf.name()))
                            }
                        },
                        ",",
                    );
                }
            }
        });
    });
}

fn print_from_old_enum_to_new(s: &mut Writer, ce: &ComplexEnum) {
    s.bodyn(
        format!(
            "impl From<&{new}> for {original}",
            new = ce.name(),
            original = ce.original_ty_name(),
        ),
        |s| {
            s.body(
                format!("fn from(v: &{new}) -> Self", new = ce.name()),
                |s| {
                    s.body("match &v", |s| {
                        for f in ce.fields() {
                            if f.is_simple_or_subfields_const() {
                                s.wln(format!(
                                    "{new}::{field} => Self::{field},",
                                    new = ce.name(),
                                    field = f.name(),
                                ));
                            } else {
                                s.wln(format!(
                                    "{new}::{field} {{ .. }} => Self::{field},",
                                    new = ce.name(),
                                    field = f.name(),
                                ));
                            }
                        }
                    });
                },
            );
        },
    );
}

fn print_default_for_new_enum(s: &mut Writer, ce: &ComplexEnum) {
    s.bodyn(format!("impl Default for {name}", name = ce.name()), |s| {
        s.body("fn default() -> Self", |s| {
            s.wln("// First enumerator without any fields");
            let f = ce.fields().first().unwrap();
            if f.is_simple_or_subfields_const() {
                s.wln(format!("Self::{variant}", variant = f.name()));
            } else {
                s.open_curly(format!("Self::{variant}", variant = f.name()));

                for sf in f.subfields() {
                    if sf.used_as_size_in().is_some() || sf.constant_value().is_some() {
                        continue;
                    }
                    s.wln(format!("{name}: Default::default(),", name = sf.name()));
                }

                s.closing_curly();
            }
        });
    });
}

fn print_write_for_new_enum(s: &mut Writer, ce: &ComplexEnum) {
    s.funcn_pub(
        "write<W: std::io::Write>(&self, w: &mut W)",
        "std::result::Result<(), std::io::Error>",
        |s| {
            s.wln(format!(
                "let a: {ty} = self.into();",
                ty = ce.original_ty_name(),
            ));
            s.wln("a.write(w)?;");
            s.wln("Ok(())");
        },
    );

    let types = get_upcast_types(ce.ty());

    for t in types {
        s.funcn_pub(
            format!(
                "write_{ty}_{endian}<W: std::io::Write>(&self, w: &mut W)",
                ty = t.rust_str(),
                endian = t.rust_endian_str()
            ),
            "std::result::Result<(), std::io::Error>",
            |s| {
                s.wln(format!(
                    "let a: {ty} = self.into();",
                    ty = ce.original_ty_name(),
                ));
                s.wln(format!(
                    "a.write_{ty}_{endian}(w)",
                    ty = t.rust_str(),
                    endian = t.rust_endian_str()
                ));
            },
        );
    }
}

fn print_size_for_new_enum(s: &mut Writer, ce: &ComplexEnum, e: &Container, o: &Objects) {
    s.variable_size(
        ce.name(),
        |s| {
            s.body("match self", |s| {
                for field in ce.fields() {
                    if !field.is_simple_or_subfields_const() {
                        s.open_curly(format!("Self::{name} ", name = field.name()));
                        for sf in field.subfields() {
                            if sf.used_as_size_in().is_some() {
                                continue;
                            }
                            s.wln(format!("{name},", name = sf.name()));
                        }
                        s.closing_curly_with(" => {");
                        s.inc_indent();
                    } else {
                        s.open_curly(format!("Self::{name} => ", name = field.name()));
                    }

                    s.wln(format!("{self_size}", self_size = ce.ty().size()));

                    for sf in field.subfields() {
                        let array_inner_is_constant = match sf.ty() {
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

                        let does_not_have_subvariables = match sf.ty() {
                            Type::Identifier {
                                s: identifier,
                                upcast: None,
                            } => !e
                                .nested_types()
                                .new_enums()
                                .iter()
                                .any(|a| a.name() == identifier),
                            _ => false,
                        };

                        s.w("+ ");

                        print_common_impls::print_size_of_ty(
                            s,
                            sf.ty(),
                            sf.name(),
                            does_not_have_subvariables,
                            o.type_has_constant_size(sf.ty()),
                            array_inner_is_constant,
                            "",
                            &sf.ty().str(),
                        );
                    }

                    s.closing_curly();
                }
            });
        },
        |s| s.wln("65536 // maximum possible u16 size. TODO value."),
    );
}
