use crate::container::{Container, ContainerType, Equation, StructMember};
use crate::file_utils::get_import_path;
use crate::parser::types::{ArraySize, ArrayType, ObjectType, Objects, Type};
use crate::rust_printer::{
    Writer, LOGIN_CLIENT_MESSAGE_TRAIT_NAME, LOGIN_SERVER_MESSAGE_TRAIT_NAME,
    WORLD_BODY_TRAIT_NAME, WORLD_CLIENT_HEADER_TRAIT_NAME, WORLD_SERVER_HEADER_TRAIT_NAME,
};
use crate::{LOGIN_MESSAGES_GITHUB_REPO, WORLD_MESSAGES_GITHUB_REPO};

mod print_common_impls;
mod print_new_types;
mod print_optional;
mod print_tests;

pub fn print_struct(e: &Container, o: &Objects) -> Writer {
    let mut s = Writer::new(&get_import_path(e.tags()));

    print_includes(&mut s, e, o);

    print_declaration(&mut s, e);

    print_common_impls::print_common_impls(&mut s, e, o);

    print_errors(&mut s, e, o);

    print_new_types::print_new_types(&mut s, e, o);

    print_tests::print_tests(&mut s, e, o);

    for field in e.fields() {
        match field {
            StructMember::Definition(_) => {}
            StructMember::IfStatement(_) => {}
            StructMember::OptionalStatement(optional) => {
                print_optional::print_optional(&mut s, e, optional, o);
            }
        }
    }

    s
}

fn print_includes(s: &mut Writer, e: &Container, o: &Objects) {
    s.wln("use std::convert::{TryFrom, TryInto};");

    if e.contains_guid_or_packed_guid() {
        s.wln("use crate::Guid;");
    }

    if e.contains_aura_mask() {
        s.wln("use crate::AuraMask;");
    }

    if e.contains_update_mask() {
        s.wln("use crate::UpdateMask;");
    }

    for name in e.get_types_needing_import() {
        let module_name = get_import_path(o.get_tags_of_object(name, e.tags()));

        match o.get_object_type_of(name, e.tags()) {
            ObjectType::Flag => {
                s.wln(format!(
                    "use {module_name}::{{{name}}};",
                    module_name = module_name,
                    name = name,
                ));
            }
            _ => {
                if o.object_has_only_io_errors(name, e.tags()) {
                    s.wln(format!(
                        "use {module_name}::{name};",
                        module_name = module_name,
                        name = name,
                    ));
                } else {
                    s.wln(format!(
                        "use {module_name}::{{{name}, {name}Error}};",
                        module_name = module_name,
                        name = name,
                    ));
                }
            }
        }
    }

    match e.container_type() {
        ContainerType::CLogin(_) => {
            s.wln(format!("use crate::{};", LOGIN_CLIENT_MESSAGE_TRAIT_NAME));
        }
        ContainerType::SLogin(_) => {
            s.wln(format!("use crate::{};", LOGIN_SERVER_MESSAGE_TRAIT_NAME));
        }
        ContainerType::SMsg(_) => {
            s.wln(format!(
                "use crate::{{{}, {}}};",
                WORLD_SERVER_HEADER_TRAIT_NAME, WORLD_BODY_TRAIT_NAME,
            ));
            s.wln("use wow_srp::header_crypto::Encrypter;");
        }
        ContainerType::CMsg(_) => {
            s.wln(format!(
                "use crate::{{{}, {}}};",
                WORLD_CLIENT_HEADER_TRAIT_NAME, WORLD_BODY_TRAIT_NAME,
            ));
            s.wln("use wow_srp::header_crypto::Encrypter;");
        }
        ContainerType::Msg(_) => {
            s.wln(format!(
                "use crate::{{{}, {}, {}}};",
                WORLD_CLIENT_HEADER_TRAIT_NAME,
                WORLD_SERVER_HEADER_TRAIT_NAME,
                WORLD_BODY_TRAIT_NAME,
            ));
            s.wln("use wow_srp::header_crypto::Encrypter;");
        }
        _ => {}
    }
    s.wln("use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};");

    s.newline();
}

fn can_derive_default(e: &Container) -> bool {
    fn inner(m: &StructMember) -> bool {
        if let StructMember::Definition(d) = m {
            if let Type::Array(array) = d.ty() {
                if let ArrayType::Integer(_) = array.ty() {
                    if let ArraySize::Fixed(size) = array.size() {
                        if size > 32 {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    for m in e.fields() {
        match inner(m) {
            true => {}
            false => return false,
        }
    }

    true
}

fn print_declaration(s: &mut Writer, e: &Container) {
    s.w("#[derive(Debug, PartialEq, Clone");
    if can_derive_default(e) {
        s.w_no_indent(", Default");
    }
    s.wln_no_indent(")]");

    if e.is_constant_sized() {
        s.wln("#[derive(Copy)]");
    }

    print_struct_wowm_definition(s, e);

    s.new_struct(e.name(), |s| {
        let types = e.nested_types();

        for field in types.declarations() {
            if field.constant_value().is_some() || field.used_as_size_in().is_some() {
                continue;
            }
            s.wln(format!(
                "pub {name}: {enum_prefix}{type_name},",
                name = field.name(),
                type_name = field.ty().rust_str(),
                enum_prefix = match field.does_not_have_subvariables() {
                    true => "",
                    false => e.name(),
                },
            ));
        }

        for m in e.fields() {
            match m {
                StructMember::Definition(_) => {}
                StructMember::IfStatement(_) => {}
                StructMember::OptionalStatement(optional) => {
                    s.wln(format!(
                        "pub {name}: Option<{original_ty}_{name}>,",
                        name = optional.name(),
                        original_ty = e.name()
                    ));
                }
            }
        }
    });
}

fn print_docc_members(s: &mut Writer, e: &Container, field: &StructMember) {
    match field {
        StructMember::Definition(d) => {
            s.docc(format!(
                "{ty} {name}{constant};",
                ty = d.ty().str(),
                name = d.name(),
                constant = match d.verified_value() {
                    None => "".to_string(),
                    Some(c) => format!(" = {val}", val = c.original_string()),
                }
            ));
        }
        StructMember::IfStatement(statement) => {
            let equations = statement.conditional.equations();
            for (i, eq) in equations.iter().enumerate() {
                let name = statement.conditional.variable_name();
                let (op, cond) = match eq {
                    Equation::Equals { value } => ("==", value),
                    Equation::NotEquals { value } => ("!=", value),
                    Equation::BitwiseAnd { value } => ("&", value),
                };

                if i == 0 {
                    s.docc_w(format!(
                        "if ({name} {op} {cond}",
                        name = name,
                        op = op,
                        cond = cond
                    ));
                    s.docc_inc();
                } else {
                    s.docc_w(format!(
                        "|| {name} {op} {cond}",
                        name = name,
                        op = op,
                        cond = cond
                    ));
                }

                if i == equations.len() - 1 {
                    s.wln_no_indent(") {");
                } else {
                    s.wln_no_indent("");
                }
            }

            for f in statement.members() {
                print_docc_members(s, e, f);
            }

            if !statement.else_ifs().is_empty() {
                s.docc("ELSE-IF-STATEMENT-DOCC: unimplemented");
            }

            s.docc_dec();
            if statement.else_statement_members.is_empty() {
                s.docc("}");
            } else {
                s.docc("} else {");
                s.docc_inc();

                for f in &statement.else_statement_members {
                    print_docc_members(s, e, f);
                }

                s.docc_dec();
                s.docc("}");
            }
        }
        StructMember::OptionalStatement(_) => s.docc("OPTIONAL-STATEMENT-DOCC: unimplemented"),
    }
}

fn print_struct_wowm_definition(s: &mut Writer, e: &Container) {
    s.docc_wowm(
        |s| {
            s.docc(format!(
                "{kind} {name}{opcode} {{",
                kind = e.container_type().str(),
                name = e.name(),
                opcode = match e.container_type() {
                    ContainerType::Struct => "".to_string(),
                    ContainerType::CLogin(o)
                    | ContainerType::SLogin(o)
                    | ContainerType::Msg(o)
                    | ContainerType::CMsg(o)
                    | ContainerType::SMsg(o) => format!(" = 0x{opcode:X}", opcode = o),
                }
            ));

            s.docc_inc();

            for field in e.fields() {
                print_docc_members(s, e, field);
            }

            s.docc_dec();
            s.docc("}")
        },
        match e.tags().has_login_version() {
            true => LOGIN_MESSAGES_GITHUB_REPO,
            false => WORLD_MESSAGES_GITHUB_REPO,
        },
        e.file_info(),
    );
}

fn print_errors(s: &mut Writer, e: &Container, o: &Objects) {
    if e.only_has_io_errors() {
        return;
    }

    print_general_error(s, e, o);
    print_from_general_error(s, e, o);
}

fn print_general_error(s: &mut Writer, e: &Container, o: &Objects) {
    s.wln("#[derive(Debug)]");
    s.new_enum(format!("{name}Error", name = e.name()), |s| {
        s.wln("Io(std::io::Error),");
        if e.contains_string_or_cstring() {
            s.wln("String(std::string::FromUtf8Error),");
        }

        for t in e.get_types_needing_errors() {
            if o.object_has_only_io_errors(t, e.tags()) {
                continue;
            }

            match o.get_object_type_of(t, e.tags()) {
                ObjectType::Flag => {}
                _ => {
                    s.wln(format!("{name}({name}Error),", name = t));
                }
            }
        }
    });

    print_display_for_general_error(s, e, o);
}

fn print_display_for_general_error(s: &mut Writer, e: &Container, o: &Objects) {
    s.wln(format!(
        "impl std::error::Error for {name}Error {{}}",
        name = e.name()
    ));

    s.bodyn(
        format!("impl std::fmt::Display for {name}Error", name = e.name()),
        |s| {
            s.body(
                "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result",
                |s| {
                    s.body("match self", |s| {
                        s.wln("Self::Io(i) => i.fmt(f),");

                        if e.contains_string_or_cstring() {
                            s.wln("Self::String(i) => i.fmt(f),");
                        }

                        for t in e.get_types_needing_errors() {
                            if o.object_has_only_io_errors(t, e.tags()) {
                                continue;
                            }

                            match o.get_object_type_of(t, e.tags()) {
                                ObjectType::Flag => {}
                                _ => {
                                    s.wln(format!("Self::{name}(i) => i.fmt(f),", name = t));
                                }
                            }
                        }
                    });
                },
            );
        },
    );
}

fn print_from_general_error(s: &mut Writer, e: &Container, o: &Objects) {
    s.bodyn(
        format!("impl From<std::io::Error> for {name}Error", name = e.name()),
        |s| {
            s.body("fn from(e : std::io::Error) -> Self", |s| {
                s.wln("Self::Io(e)");
            });
        },
    );

    if e.contains_string_or_cstring() {
        s.bodyn(
            format!(
                "impl From<std::string::FromUtf8Error> for {name}Error",
                name = e.name()
            ),
            |s| {
                s.body("fn from(e: std::string::FromUtf8Error) -> Self", |s| {
                    s.wln("Self::String(e)");
                });
            },
        );
    }

    for t in e.get_types_needing_errors() {
        if o.object_has_only_io_errors(t, e.tags()) {
            continue;
        }

        if o.get_object_type_of(t, e.tags()) == ObjectType::Flag {
            continue;
        }

        s.bodyn(
            format!(
                "impl From<{from_name}Error> for {name}Error",
                name = e.name(),
                from_name = t
            ),
            |s| {
                s.body(
                    format!("fn from(e: {from_name}Error) -> Self", from_name = t),
                    |s| {
                        s.wln(format!("Self::{from_name}(e)", from_name = t));
                    },
                );
            },
        );
    }
}
