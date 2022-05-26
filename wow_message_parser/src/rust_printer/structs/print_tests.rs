use crate::container::{Container, ContainerType};
use crate::file_utils::get_import_path;
use crate::parser::types::objects::Objects;
use crate::parser::types::ArraySize;
use crate::rust_printer::opcodes::get_enumerator_name;
use crate::rust_printer::rust_view::{RustEnumerator, RustMember, RustType};
use crate::rust_printer::{
    ImplType, Writer, CLIENT_MESSAGE_TRAIT_NAME, LOGIN_CLIENT_MESSAGE_ENUM_NAME,
    LOGIN_SERVER_MESSAGE_ENUM_NAME, SERVER_MESSAGE_TRAIT_NAME, WORLD_BODY_TRAIT_NAME,
    WORLD_CLIENT_MESSAGE_ENUM_NAME, WORLD_SERVER_MESSAGE_ENUM_NAME,
};
use crate::test_case::{TestCase, TestCaseMember, TestValue};

pub(super) fn print_tests(s: &mut Writer, e: &Container, o: &Objects) {
    if e.tests().is_empty() {
        return;
    }

    s.wln("#[cfg(test)]");
    s.body("mod test", |s| {
        s.wln(format!("use super::{};", e.name()));

        for name in e.get_types_needing_import_recursively(o) {
            let tags = o.get_tags_of_object(name, e.tags());
            s.wln(format!(
                "use {import_path}::{ty};",
                import_path = get_import_path(tags),
                ty = name,
            ));
        }

        s.wln("use super::*;");
        s.wln("use super::super::*;");
        s.wln(format!("use {};", e.get_opcode_import_path(),));
        match e.container_type() {
            ContainerType::Msg(_) => panic!(),
            ContainerType::CMsg(_) | ContainerType::SMsg(_) => {
                s.wln(format!(
                    "use crate::{{{}, {}, {}}};",
                    WORLD_BODY_TRAIT_NAME, CLIENT_MESSAGE_TRAIT_NAME, SERVER_MESSAGE_TRAIT_NAME,
                ));
            }
            _ => {}
        }
        s.newline();

        for (i, t) in e.tests().iter().enumerate() {
            for it in ImplType::types() {
                s.metadata_comment(format!(
                    "Generated from `{filename}` line {line}.",
                    filename = t.file_info().name(),
                    line = t.file_info().start_line()
                ));
                s.wln(it.cfg());
                s.wln(it.test_macro());
                s.bodyn(
                    format!(
                        "{func}fn {prefix}{subject}{number}()",
                        func = it.func(),
                        prefix = it.prefix(),
                        subject = t.subject(),
                        number = i,
                    ),
                    |s| {
                        print_test_case(s, t, e, o, it);
                    },
                );
            }
        }
    });
}

fn print_test_case(s: &mut Writer, t: &TestCase, e: &Container, o: &Objects, it: ImplType) {
    s.w("let raw: Vec<u8> = vec![");
    s.inc_indent();
    for i in t.raw_bytes() {
        s.w_break_at(format!(" {:#04X},", i), 80);
    }
    s.dec_indent();
    s.wln_no_indent(" ];\n");

    s.body_closing_with(
        format!("let expected = {}", t.subject()),
        |s| {
            for m in e.rust_object().members_in_struct() {
                print_value(s, m, t.members(), e, o);
            }
        },
        ";\n",
    );

    let (opcode, read_text, write_text) = match e.container_type() {
        ContainerType::CLogin(_) => {
            s.wln("let header_size = 1;");
            (
                LOGIN_CLIENT_MESSAGE_ENUM_NAME,
                format!("{}read", it.prefix()),
                format!("{}write", it.prefix()),
            )
        }
        ContainerType::SLogin(_) => {
            s.wln("let header_size = 1;");
            (
                LOGIN_SERVER_MESSAGE_ENUM_NAME,
                format!("{}read", it.prefix()),
                format!("{}write", it.prefix()),
            )
        }
        ContainerType::SMsg(_) => {
            s.wln("let header_size = 2 + 2;");
            (
                WORLD_SERVER_MESSAGE_ENUM_NAME,
                format!("{}read_unencrypted", it.prefix()),
                format!("{}write_unencrypted_server", it.prefix()),
            )
        }
        ContainerType::CMsg(_) => {
            s.wln("let header_size = 2 + 4;");
            (
                WORLD_CLIENT_MESSAGE_ENUM_NAME,
                format!("{}read_unencrypted", it.prefix()),
                format!("{}write_unencrypted_client", it.prefix()),
            )
        }
        _ => unimplemented!(),
    };

    s.wln(format!(
        "let t = {opcode}::{read_text}(&mut {cursor}Cursor::new(&raw)){postfix}.unwrap();",
        opcode = opcode,
        read_text = read_text,
        postfix = it.postfix(),
        cursor = match it {
            ImplType::Std | ImplType::Tokio => "std::io::",
            ImplType::AsyncStd => "async_std::io::",
        },
    ));

    s.body_closing_with(
        "let t = match t",
        |s| {
            s.wln(format!(
                "{opcode}::{subject}(t) => t,",
                opcode = opcode,
                subject = get_enumerator_name(t.subject()),
            ));
            s.wln(format!(
                r#"opcode => panic!("incorrect opcode. Expected {}, got {{opcode:#?}}", opcode = opcode),"#,
                get_enumerator_name(t.subject())
            ));
        },
        ";\n",
    );

    // Better error reporting when something is wrong.
    for m in e.rust_object().members_in_struct() {
        s.wln(format!(
            "assert_eq!(t.{field}, expected.{field});",
            field = m.name()
        ));
    }
    s.newline();

    // Size reports correct length
    match e.is_constant_sized() {
        false => {
            s.wln("assert_eq!(t.size() + header_size, raw.len());");
        }
        true => {
            s.wln(format!(
                "assert_eq!({} + header_size, raw.len());",
                e.sizes(o).maximum(),
            ));
        }
    }
    s.newline();

    s.wln("let mut dest = Vec::with_capacity(raw.len());");
    s.wln(format!(
        "expected.{write_text}(&mut {cursor}Cursor::new(&mut dest)){postfix}.unwrap();",
        write_text = write_text,
        postfix = it.postfix(),
        cursor = match it {
            ImplType::Std | ImplType::Tokio => "std::io::",
            ImplType::AsyncStd => "async_std::io::",
        },
    ));
    s.newline();

    s.wln("assert_eq!(dest, raw);")
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EnumeratorType {
    Regular,
    Elseif {
        parent_name: String,
        parent_enum_name: String,
    },
}

pub fn get_enumerator<'a>(
    enumerators: &'a [RustEnumerator],
    enumerator_name: &str,
) -> Option<(&'a RustEnumerator, EnumeratorType)> {
    for enumerator in enumerators {
        if enumerator.contains_elseif() {
            let member = &enumerator.members()[0];
            let member_name = member.ty().rust_str();
            match member.ty() {
                RustType::Enum {
                    enumerators,
                    is_elseif,
                    ..
                } => {
                    assert!(is_elseif);

                    for inner in enumerators {
                        if inner.name() == enumerator_name {
                            return Some((
                                inner,
                                EnumeratorType::Elseif {
                                    parent_name: member_name,
                                    parent_enum_name: enumerator.name().to_string(),
                                },
                            ));
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        if enumerator.name() == enumerator_name {
            return Some((enumerator, EnumeratorType::Regular));
        }
    }

    None
}

fn print_value(s: &mut Writer, m: &RustMember, t: &[TestCaseMember], e: &Container, o: &Objects) {
    let member = TestCase::get_member(t, m.name());
    s.w(format!("{name}: ", name = m.name(),));

    match member.value() {
        TestValue::Number(i) => {
            s.wln_no_indent(format!("{:#X},", i.value()));
        }
        TestValue::Guid(i) => {
            s.wln_no_indent(format!("Guid::new({:#X}),", i.value()));
        }
        TestValue::FloatingNumber { value, .. } => {
            s.wln_no_indent(format!("{:1.1},", value));
        }
        TestValue::Array { values, size } => {
            match size {
                ArraySize::Fixed(_) => s.w_no_indent("["),
                ArraySize::Variable(_) => s.w_no_indent("vec!["),
                ArraySize::Endless => panic!(),
            }
            s.inc_indent();

            for value in values {
                s.w_break_at(format!(" {:#04X},", value), 80);
            }

            s.dec_indent();
            s.wln_no_indent(" ],");
        }
        TestValue::ArrayOfSubObject(ty_name, multiples) => {
            if multiples.is_empty() {
                s.wln_no_indent("vec![],");
                return;
            }

            s.wln_no_indent("vec![");
            s.inc_indent();

            let array_container = o.get_container(ty_name, e.tags());

            for multiple in multiples {
                s.wln(format!("{} {{", ty_name));
                s.inc_indent();

                for m in array_container.rust_object().members_in_struct() {
                    print_value(s, m, multiple, array_container, o);
                }

                s.closing_curly_with(",");
            }

            s.dec_indent();
            s.wln("],");
        }
        TestValue::String(value) => {
            s.wln_no_indent(format!(r#"String::from("{}"),"#, value,));
        }
        TestValue::Flag(flags) => {
            let rd = e.rust_object().get_rust_definer(&m.ty().str());
            s.wln_no_indent(format!("{ty}::empty()", ty = m.ty()));
            s.inc_indent();

            for flag in flags {
                if let Some((enumerator, elseif_ty)) =
                    get_enumerator(rd.enumerators(), flag.as_str())
                {
                    if !enumerator.has_members_in_struct() {
                        s.wln(format!(".set_{}()", enumerator.name()));
                        continue;
                    }

                    match elseif_ty {
                        EnumeratorType::Regular => {
                            s.open_curly(format!(
                                ".set_{}({}{}",
                                enumerator.name(),
                                rd.ty_name(),
                                enumerator.name()
                            ));

                            for m in enumerator.members_in_struct() {
                                print_value(s, m, t, e, o);
                            }

                            s.closing_curly_with(")");
                        }
                        EnumeratorType::Elseif {
                            parent_name: parent_ty_name,
                            parent_enum_name,
                        } => {
                            s.open_curly(format!(
                                ".set_{}({}::{}",
                                parent_enum_name,
                                parent_ty_name,
                                enumerator.name()
                            ));

                            for m in enumerator.members_in_struct() {
                                print_value(s, m, t, e, o);
                            }

                            s.closing_curly_with(")");
                        }
                    }
                }
            }

            s.wln(",");
            s.dec_indent();
        }
        TestValue::SubObject { ty_name, members } => {
            s.wln_no_indent(format!("{} {{", ty_name));
            s.inc_indent();

            for m in members {
                s.wln(format!("{}: {},", m.name(), m.value().original_string()));
            }

            s.closing_curly_with(",");
        }
        TestValue::Enum(i) => {
            s.w_no_indent(format!(
                "{ty}::{enumerator}",
                ty = m.ty(),
                enumerator = i.original_string(),
            ));

            let subvars = match m.ty() {
                RustType::Enum { enumerators, .. } => enumerators
                    .iter()
                    .find(|a| a.name() == i.original_string())
                    .unwrap()
                    .members_in_struct(),
                _ => panic!("{} is not an enum", m.ty()),
            };
            if subvars.is_empty() {
                s.wln_no_indent(",");
                return;
            }

            s.wln_no_indent(" {");
            s.inc_indent();

            for sf in subvars {
                print_value(s, sf, t, e, o);
            }

            s.closing_curly_with(",");
        }
    }
}
