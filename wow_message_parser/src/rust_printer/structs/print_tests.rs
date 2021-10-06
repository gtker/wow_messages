use crate::container::{Container, ContainerType};
use crate::file_utils::get_import_path;
use crate::parser::types::{ArraySize, Objects};
use crate::rust_printer::complex_print::{Declaration, DefinerType};
use crate::rust_printer::opcodes::get_enumerator_name;
use crate::rust_printer::{
    Writer, LOGIN_CLIENT_MESSAGE_ENUM_NAME, LOGIN_SERVER_MESSAGE_ENUM_NAME, WORLD_BODY_TRAIT_NAME,
    WORLD_CLIENT_HEADER_TRAIT_NAME, WORLD_CLIENT_MESSAGE_ENUM_NAME, WORLD_SERVER_HEADER_TRAIT_NAME,
    WORLD_SERVER_MESSAGE_ENUM_NAME,
};
use crate::test_case::{TestCase, TestCaseMember, TestValue};

pub(super) fn print_tests(s: &mut Writer, e: &Container, o: &Objects) {
    if e.tests().is_empty() {
        return;
    }

    s.wln("#[cfg(test)]");
    s.body("mod test", |s| {
        s.wln("use crate::ReadableAndWritable;");
        s.wln("use std::io::Cursor;");
        s.wln(format!("use super::{};", e.name()));

        match e.is_constant_sized() {
            false => s.wln("use crate::VariableSized;"),
            true => s.wln("use crate::ConstantSized;"),
        }
        for name in e.get_types_needing_import() {
            let tags = o.get_tags_of_object(name, e.tags());
            s.wln(format!(
                "use {import_path}::{ty};",
                import_path = get_import_path(tags),
                ty = name,
            ));
        }

        for ce in e.nested_types().new_enums() {
            s.wln(format!("use super::{};", ce.name()));
            match ce.definer_ty() {
                DefinerType::Enum => {}
                DefinerType::Flag => {
                    for f in ce.fields() {
                        if f.is_simple() {
                            continue;
                        }
                        s.wln(format!("use super::{}{};", ce.name(), f.name()));
                    }
                }
            }
        }
        s.wln(format!("use {};", e.get_opcode_import_path(),));
        match e.container_type() {
            ContainerType::Msg(_) => panic!(),
            ContainerType::CMsg(_) | ContainerType::SMsg(_) => {
                s.wln(format!(
                    "use crate::world::helper::{{{}, {}, {}, WorldMessage}};",
                    WORLD_BODY_TRAIT_NAME,
                    WORLD_CLIENT_HEADER_TRAIT_NAME,
                    WORLD_SERVER_HEADER_TRAIT_NAME,
                ));
            }
            _ => {}
        }
        s.newline();

        for (i, t) in e.tests().iter().enumerate() {
            s.metadata_comment(format!(
                "Generated from `{filename}` line {line}.",
                filename = t.file_info().name(),
                line = t.file_info().start_line()
            ));
            s.wln("#[test]");
            s.bodyn(
                format!("fn {subject}{number}()", subject = t.subject(), number = i,),
                |s| {
                    print_test_case(s, t, e, o);
                },
            );
        }
    });
}

fn print_test_case(s: &mut Writer, t: &TestCase, e: &Container, o: &Objects) {
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
            for m in e.nested_types().declarations() {
                if m.constant_value().is_some() || m.used_as_size_in().is_some() {
                    continue;
                }

                print_value(s, m, t.members(), e, o);
            }
        },
        ";\n",
    );

    let (opcode, read_text, write_text) = match e.container_type() {
        ContainerType::CLogin(_) => {
            s.wln("let header_size = 1;");
            (LOGIN_CLIENT_MESSAGE_ENUM_NAME, "read", "write")
        }
        ContainerType::SLogin(_) => {
            s.wln("let header_size = 1;");
            (LOGIN_SERVER_MESSAGE_ENUM_NAME, "read", "write")
        }
        ContainerType::SMsg(_) => {
            s.wln("let header_size = 2 + 2;");
            (
                WORLD_SERVER_MESSAGE_ENUM_NAME,
                "read_unencrypted",
                "write_unencrypted_server",
            )
        }
        ContainerType::CMsg(_) => {
            s.wln("let header_size = 2 + 4;");
            (
                WORLD_CLIENT_MESSAGE_ENUM_NAME,
                "read_unencrypted",
                "write_unencrypted_client",
            )
        }
        _ => unimplemented!(),
    };

    s.wln(format!(
        "let t = {opcode}::{read_text}(&mut Cursor::new(&raw)).unwrap();",
        opcode = opcode,
        read_text = read_text,
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
    for m in e.nested_types().declarations() {
        if m.constant_value().is_some() || m.used_as_size_in().is_some() {
            continue;
        }

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
                "assert_eq!({}::size() + header_size, raw.len());",
                e.name()
            ));
        }
    }
    s.newline();

    s.wln("let mut dest = Vec::with_capacity(raw.len());");
    s.wln(format!(
        "expected.{write_text}(&mut Cursor::new(&mut dest));",
        write_text = write_text
    ));
    s.newline();

    s.wln("assert_eq!(dest, raw);")
}

fn print_value(s: &mut Writer, m: &Declaration, t: &[TestCaseMember], e: &Container, o: &Objects) {
    let member = TestCase::get_member(t, m.name());
    s.w(format!(
        "{name}: {enum_prefix}",
        name = m.name(),
        enum_prefix = match m.does_not_have_subvariables() {
            true => "",
            false => e.name(),
        },
    ));

    match member.value() {
        TestValue::Number(i) => {
            s.wln_no_indent(format!("{:#X},", i.value()));
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

                for m in array_container.nested_types().declarations() {
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
            s.wln_no_indent(format!("{ty} {{", ty = m.ty().rust_str()));
            s.inc_indent();

            s.w("inner: 0");
            for f in flags {
                s.w_no_indent(format!(
                    " | {original_ty}::{value}",
                    original_ty = m.ty().rust_str(),
                    value = f,
                ))
            }
            s.wln_no_indent(",");

            for f in e
                .nested_types()
                .new_enums()
                .iter()
                .find(|a| a.variable_name() == m.name())
                .unwrap()
                .fields()
            {
                if f.is_simple() {
                    continue;
                }

                s.w(format!(
                    "{variable_name}: ",
                    variable_name = f.name().to_lowercase(),
                ));

                if let Some(flag) = flags.iter().find(|a| a.as_str() == f.name()) {
                    s.wln_no_indent(format!(
                        "Some({}{}{} {{",
                        e.name(),
                        m.ty().rust_str(),
                        f.name()
                    ));
                    s.inc_indent();
                    let subvars = m.get_subfields_for_enumerator(flag.as_str());
                    for sf in &subvars {
                        print_value(s, sf, t, e, o);
                    }
                    s.closing_curly_with("),");
                } else {
                    s.wln_no_indent("None,");
                }
            }

            s.closing_curly_with(",");
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
                ty = m.ty().rust_str(),
                enumerator = i.original_string(),
            ));

            let subvars = m.get_subfields_for_enumerator(member.value().original_string());
            if subvars.is_empty() {
                s.wln_no_indent(",");
                return;
            }

            s.wln_no_indent(" {");
            s.inc_indent();

            for sf in subvars {
                if sf.used_as_size_in().is_some() || sf.constant_value().is_some() {
                    continue;
                }
                print_value(s, sf, t, e, o);
            }

            s.closing_curly_with(",");
        }
    }
}
