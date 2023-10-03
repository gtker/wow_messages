use crate::file_utils::{get_import_path, major_version_to_string};
use crate::float_format;
use crate::parser::types::array::ArraySize;
use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::objects::Objects;
use crate::parser::types::test_case::{TestCase, TestCaseMember, TestValue};
use crate::parser::types::version::Version;
use crate::parser::utility::parse_value;
use crate::rust_printer::opcodes::get_enumerator_name;
use crate::rust_printer::rust_view::rust_enumerator::RustEnumerator;
use crate::rust_printer::rust_view::rust_member::RustMember;
use crate::rust_printer::rust_view::rust_type::RustType;
use crate::rust_printer::writer::Writer;
use crate::rust_printer::{
    get_new_flag_type_name, ByteInnerTy, ByteType, ImplType, UpdateMaskDataType,
    UpdateMaskObjectType, CLIENT_MESSAGE_TRAIT_NAME, LOGIN_CLIENT_MESSAGE_ENUM_NAME,
    LOGIN_SERVER_MESSAGE_ENUM_NAME, SERVER_MESSAGE_TRAIT_NAME, WORLD_CLIENT_MESSAGE_ENUM_NAME,
    WORLD_SERVER_MESSAGE_ENUM_NAME,
};

pub(super) fn print_tests(s: &mut Writer, e: &Container, o: &Objects) {
    if e.tests(o).is_empty() {
        return;
    }

    for version in e.tags().main_versions() {
        if version.is_world() && e.tags().shared() {
            let version = major_version_to_string(&version.as_major_world());
            s.wln(format!("#[cfg(all(feature = \"{version}\", test))]"));
        } else {
            s.wln("#[cfg(test)]");
        }
        if e.tags().shared() {
            s.open_curly(format!("mod test_{}", version.to_module_case()));
        } else {
            s.open_curly("mod test");
        }

        s.wln("#![allow(clippy::missing_const_for_fn)]");

        print_includes(e, version, s);

        match e.container_type() {
            ContainerType::CLogin(_) => {
                s.wln("const HEADER_SIZE: usize = 1;");
            }
            ContainerType::SLogin(_) => {
                s.wln("const HEADER_SIZE: usize = 1;");
            }
            ContainerType::SMsg(_) => {
                s.wln("const HEADER_SIZE: usize = 2 + 2;");
            }
            ContainerType::CMsg(_) => {
                s.wln("const HEADER_SIZE: usize = 2 + 4;");
            }
            _ => unimplemented!(),
        }

        if !e.empty_body() {
            s.bodyn(
                format!("fn assert(t: &{ty}, expected: &{ty})", ty = e.name()),
                |s| {
                    // Better error reporting when something is wrong.
                    for m in e.rust_object().members_in_struct() {
                        s.wln(format!(
                            "assert_eq!(t.{field}, expected.{field});",
                            field = m.name()
                        ));
                    }
                },
            );
        }

        for (i, t) in e.tests(o).iter().enumerate() {
            s.w(format!("const RAW{}: [u8; {}] = [", i, t.raw_bytes().len()));
            s.inc_indent();

            for i in t.raw_bytes() {
                s.w_break_at(format!(" {i:#04X},"));
            }

            s.dec_indent();
            s.wln_no_indent(" ];\n");

            s.funcn(format!("expected{i}()"), t.subject(), |s| {
                s.bodyn(t.subject(), |s| {
                    for m in e.rust_object().members_in_struct() {
                        print_value(s, m, t.members(), e, version);
                    }

                    if let Some(optional) = e.rust_object().optional() {
                        let name = e.name();
                        let optional_name = optional.name();

                        if !optional
                            .members_in_struct()
                            .iter()
                            .any(|a| TestCase::try_get_member(t.members(), a.name()).is_none())
                        {
                            s.body_closing_with(
                                format!("{optional_name}: Some({name}_{optional_name}"),
                                |s| {
                                    for m in optional.members_in_struct() {
                                        print_value(s, m, t.members(), e, version);
                                    }
                                },
                                ")",
                            );
                        } else {
                            s.wln(format!("{optional_name}: None,"))
                        }
                    }
                });
            });

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
                        "{func}fn {prefix}{subject}{i}()",
                        func = it.func(),
                        prefix = it.prefix(),
                        subject = t.subject().to_lowercase(),
                    ),
                    |s| {
                        print_test_case(s, t, e, it, i);
                    },
                );
            }
        }

        s.closing_curly_newline(); // mod test
    }
}

fn print_includes(e: &Container, version: Version, s: &mut Writer) {
    let import_path = get_import_path(version);

    s.wln(format!("use super::{};", e.name()));

    match version {
        Version::Login(_) => {
            s.wln("use crate::all::*;");
        }
        Version::World(_) => {}
    }

    s.wln("use super::*;");
    s.wln("use super::super::*;");
    s.wln(format!("use {};", e.get_opcode_import_path(version),));

    match e.container_type() {
        ContainerType::Msg(_) => {
            unimplemented!("test for MSG type")
        }
        ContainerType::CMsg(_) | ContainerType::SMsg(_) => {
            if e.contains_guid_or_packed_guid_transitively()
                || e.contains_update_mask_transitively()
            {
                s.wln("use crate::Guid;");
            }

            if e.contains_update_mask_transitively() {
                s.wln(format!("use {import_path}::{{UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer}};"));
            }

            if e.contains_aura_mask_transitively() {
                s.wln(format!("use {import_path}::{{AuraMask}};"));
            }

            s.wln(format!(
                "use {import_path}::{{{CLIENT_MESSAGE_TRAIT_NAME}, {SERVER_MESSAGE_TRAIT_NAME}}};",
            ));
        }
        _ => {}
    }
    s.newline();
}

fn print_test_case(s: &mut Writer, t: &TestCase, e: &Container, it: ImplType, i: usize) {
    s.wln(format!("let expected = expected{i}();"));
    let (opcode, read_text, write_text) = match e.container_type() {
        ContainerType::CLogin(_) => (
            LOGIN_CLIENT_MESSAGE_ENUM_NAME,
            format!("{}read", it.prefix()),
            format!("{}write", it.prefix()),
        ),
        ContainerType::SLogin(_) => (
            LOGIN_SERVER_MESSAGE_ENUM_NAME,
            format!("{}read", it.prefix()),
            format!("{}write", it.prefix()),
        ),
        ContainerType::SMsg(_) => (
            WORLD_SERVER_MESSAGE_ENUM_NAME,
            format!("{}read_unencrypted", it.prefix()),
            format!("{}write_unencrypted_server", it.prefix()),
        ),
        ContainerType::CMsg(_) => (
            WORLD_CLIENT_MESSAGE_ENUM_NAME,
            format!("{}read_unencrypted", it.prefix()),
            format!("{}write_unencrypted_client", it.prefix()),
        ),
        _ => unimplemented!(),
    };

    s.wln(format!(
        "let t = {opcode}::{read_text}(&mut {cursor}Cursor::new(&RAW{i})){postfix}.unwrap();",
        opcode = opcode,
        i = i,
        read_text = read_text,
        postfix = it.postfix(),
        cursor = match it {
            ImplType::Std | ImplType::Tokio => "std::io::",
            ImplType::AsyncStd => "async_std::io::",
        },
    ));

    let prefix = if e.empty_body() { "" } else { "let t = " };

    s.body_closing_with_semicolon(format!("{prefix}match t"), |s| {
        if e.empty_body() {
            s.wln(format!(
                "{opcode}::{subject} => {{}}",
                opcode = opcode,
                subject = get_enumerator_name(t.subject()),
            ));
        } else {
            s.wln(format!(
                "{opcode}::{subject}(t) => t,",
                opcode = opcode,
                subject = get_enumerator_name(t.subject()),
            ));
        }
        s.wln(format!(
            r#"opcode => panic!("incorrect opcode. Expected {}, got {{opcode:#?}}"),"#,
            get_enumerator_name(t.subject())
        ));
    });
    s.newline();

    if !e.empty_body() {
        s.wln("assert(&t, &expected);");
    }

    let compressed = e.tags().compressed() || e.contains_compressed_variable();

    if !compressed {
        // Size reports correct length
        match e.is_constant_sized() {
            false => {
                s.wln(format!("assert_eq!(t.size() + HEADER_SIZE, RAW{i}.len());"));
            }
            true => {
                let size = if e.sizes().maximum() == 0 {
                    "".to_string()
                } else {
                    format!("{} + ", e.sizes().maximum())
                };

                s.wln(format!("assert_eq!({size}HEADER_SIZE, RAW{i}.len());",));
            }
        }
        s.newline();
    }

    s.wln(format!("let mut dest = Vec::with_capacity(RAW{i}.len());"));
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

    if compressed {
        s.wln(format!(
            "let s = {opcode}::{read_text}(&mut {cursor}Cursor::new(&dest)){postfix}.unwrap();",
            opcode = opcode,
            read_text = read_text,
            postfix = it.postfix(),
            cursor = match it {
                ImplType::Std | ImplType::Tokio => "std::io::",
                ImplType::AsyncStd => "async_std::io::",
            },
        ));

        s.body_closing_with(
            "let s = match s",
            |s| {
                s.wln(format!(
                    "{opcode}::{subject}(s) => s,",
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

        s.wln("assert_eq!(t, s);");
    } else {
        s.wln(format!("assert_eq!(dest, RAW{i});"))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum EnumeratorType {
    Regular,
    Elseif {
        parent_name: String,
        parent_enum_name: String,
    },
}

pub(crate) fn get_enumerator<'a>(
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

fn print_value(
    s: &mut Writer,
    m: &RustMember,
    t: &[TestCaseMember],
    e: &Container,
    version: Version,
) {
    let member = TestCase::get_member(t, m.name());
    s.w(format!("{name}: ", name = m.name(),));

    match member.value() {
        TestValue::Number(i) => {
            s.wln_no_indent(format!("{:#X},", i.value()));
        }
        TestValue::Seconds(i) => {
            s.wln_no_indent(format!("Duration::from_secs({:#X}),", i.value()));
        }
        TestValue::Milliseconds(i) => {
            s.wln_no_indent(format!("Duration::from_millis({:#X}),", i.value()));
        }
        TestValue::IpAddress(i) => {
            s.wln_no_indent(format!("Ipv4Addr::from({:#X}_u32),", i.value()));
        }
        TestValue::DateTime(i) => {
            s.wln_no_indent(format!("DateTime::try_from({:#X}).unwrap(),", i.value()));
        }
        TestValue::Gold(i) => {
            s.wln_no_indent(format!("Gold::try_from({:#X}).unwrap(),", i.value()));
        }
        TestValue::Level(i) => {
            s.wln_no_indent(format!("Level::try_from({:#X}).unwrap(),", i.value()));
        }
        TestValue::Bool(b) => {
            s.wln_no_indent(format!("{b}, "));
        }
        TestValue::Guid(i) => {
            s.wln_no_indent(format!("Guid::new({:#X}),", i.value()));
        }
        TestValue::FloatingNumber { value, .. } => {
            s.wln_no_indent(format!("{value}_f32,"));
        }
        TestValue::Population { value } => {
            let value = *value;
            let value = if value == 200.0 {
                "Population::GreenRecommended".to_string()
            } else if value == 400.0 {
                "Population::RedFull".to_string()
            } else if value == 600.0 {
                "Population::BlueRecommended".to_string()
            } else {
                format!("Self::Other({value})")
            };

            s.wln_no_indent(format!("{value},"));
        }
        TestValue::IntegerArray { values, size } => {
            match size {
                ArraySize::Fixed(_) => s.w_no_indent("["),
                ArraySize::Variable(_) | ArraySize::Endless => s.w_no_indent("vec!["),
            }
            s.inc_indent();

            for value in values {
                s.w_break_at(format!(" {value:#04X},"));
            }

            s.dec_indent();
            s.wln_no_indent(" ],");
        }
        TestValue::StringArray { values, size } => {
            match size {
                ArraySize::Fixed(_) => s.w_no_indent("["),
                ArraySize::Variable(_) | ArraySize::Endless => s.w_no_indent("vec!["),
            }
            s.inc_indent();

            for value in values {
                s.w_break_at(format!(" \"{value}\".to_string(),"));
            }

            s.dec_indent();
            s.wln_no_indent(" ],");
        }
        TestValue::ArrayOfSubObject { c, members, size } => {
            let opening = match size {
                ArraySize::Fixed(_) => "[",
                ArraySize::Variable(_) | ArraySize::Endless => "vec![",
            };

            if members.is_empty() {
                s.wln_no_indent(format!("{opening}],"));
                return;
            }

            s.wln_no_indent(opening);
            s.inc_indent();

            for multiple in members {
                s.wln(format!("{} {{", c.name()));
                s.inc_indent();

                for m in c.rust_object().members_in_struct() {
                    print_value(s, m, multiple, c, version);
                }

                s.closing_curly_with(",");
            }

            s.dec_indent();
            s.wln("],");
        }
        TestValue::MonsterMoveSpline(values) => {
            s.wln_no_indent("vec![");
            s.inc_indent();

            for v in values {
                s.body_closing_with(
                    "Vector3d",
                    |s| {
                        s.wln(format!("x: {},", float_format(v.x)));
                        s.wln(format!("y: {},", float_format(v.y)));
                        s.wln(format!("z: {},", float_format(v.z)));
                    },
                    ",",
                );
            }

            s.dec_indent();
            s.wln("],");
        }
        TestValue::String(value) => {
            s.wln_no_indent(format!(r#"String::from("{value}"),"#,));
        }
        TestValue::Flag(flags) => {
            let rd = e.rust_object().get_rust_definer(&m.ty().str());
            s.wln_no_indent(format!("{ty}::empty()", ty = m.ty()));
            s.inc_indent();

            for flag in flags {
                if let Some((enumerator, elseif_ty)) =
                    get_enumerator(rd.enumerators(), flag.as_str())
                {
                    if enumerator.value().int() == 0 {
                        continue;
                    }

                    if !enumerator.has_members_in_struct() {
                        s.wln(format!(".set_{}()", enumerator.name().to_lowercase()));
                        continue;
                    }

                    match elseif_ty {
                        EnumeratorType::Regular => {
                            s.open_curly(format!(
                                ".set_{}({}",
                                enumerator.name().to_lowercase(),
                                get_new_flag_type_name(rd.ty_name(), enumerator.rust_name()),
                            ));

                            for m in enumerator.members_in_struct() {
                                print_value(s, m, t, e, version);
                            }

                            s.closing_curly_with(")");
                        }
                        EnumeratorType::Elseif {
                            parent_name: parent_ty_name,
                            parent_enum_name,
                        } => {
                            s.open_curly(format!(
                                ".set_{}({}::{}",
                                parent_enum_name.to_lowercase(),
                                parent_ty_name,
                                enumerator.rust_name()
                            ));

                            for m in enumerator.members_in_struct() {
                                print_value(s, m, t, e, version);
                            }

                            s.closing_curly_with(")");
                        }
                    }
                }
            }

            s.wln(",");
            s.dec_indent();
        }
        TestValue::SubObject { c, members } => {
            s.wln_no_indent(format!("{} {{", c.name()));
            s.inc_indent();

            let t = members.as_slice();
            for m in c.rust_object().members_in_struct() {
                print_value(s, m, t, c, version);
            }

            s.closing_curly_with(",");
        }
        TestValue::Enum(i) => {
            s.w_no_indent(format!(
                "{ty}::{enumerator}",
                ty = m.ty(),
                enumerator = i.rust_name(),
            ));

            let subvars = match m.ty() {
                RustType::Enum { enumerators, .. } => enumerators
                    .iter()
                    .find(|a| a.name() == i.original_string())
                    .unwrap()
                    .members_in_struct(),
                _ => unreachable!("{} is not an enum", m.ty()),
            };
            if subvars.is_empty() {
                s.wln_no_indent(",");
                return;
            }

            s.wln_no_indent(" {");
            s.inc_indent();

            for sf in subvars {
                print_value(s, sf, t, e, version);
            }

            s.closing_curly_with(",");
        }
        TestValue::UpdateMask(fields) => {
            let ty = fields
                .iter()
                .find(|a| a.ty() == UpdateMaskObjectType::Object && a.name() == "TYPE")
                .unwrap();
            let ty = parse_value(ty.value()).unwrap();

            const ITEM: i128 = 0x0002;
            const CONTAINER: i128 = 0x0004;
            const UNIT: i128 = 0x0008;
            const PLAYER: i128 = 0x0010;
            const GAMEOBJECT: i128 = 0x0020;
            const DYNAMICOBJECT: i128 = 0x0040;
            const CORPSE: i128 = 0x0080;
            if (ty & CONTAINER) != 0 {
                s.wln_no_indent("UpdateMask::Container(UpdateContainer::builder()");
            } else if (ty & ITEM) != 0 {
                s.wln_no_indent("UpdateMask::Item(UpdateItem::builder()");
            } else if (ty & PLAYER) != 0 {
                s.wln_no_indent("UpdateMask::Player(UpdatePlayer::builder()");
            } else if (ty & UNIT) != 0 {
                s.wln_no_indent("UpdateMask::Unit(UpdateUnit::builder()");
            } else if (ty & GAMEOBJECT) != 0 {
                s.wln_no_indent("UpdateMask::GameObject(UpdateGameObject::builder()");
            } else if (ty & DYNAMICOBJECT) != 0 {
                s.wln_no_indent("UpdateMask::DynamicObject(UpdateDynamicObject::builder()");
            } else if (ty & CORPSE) != 0 {
                s.wln_no_indent("UpdateMask::Corpse(UpdateCorpse::builder()");
            } else {
                unreachable!("Invalid type for UpdateMask")
            }
            s.inc_indent();

            for f in fields {
                if f.name() == "TYPE" {
                    // Automatically set through the struct
                    continue;
                }

                let fields = version.as_major_world().update_mask();

                let field = fields
                    .iter()
                    .find(|a| a.object_ty() == f.ty() && a.name() == f.name())
                    .unwrap()
                    .clone();

                match field.ty() {
                    UpdateMaskDataType::Guid => {
                        s.wln(format!(
                            ".set_{ty}_{field}(Guid::new({value}))",
                            value = f.value(),
                            ty = f.ty().to_string().to_lowercase(),
                            field = f.name().to_lowercase(),
                        ));
                    }
                    UpdateMaskDataType::Bytes(a_ty, b_ty, c_ty, d_ty) => {
                        let (a, b, c, d) = split_u32_str_into_u8s(f.value());

                        let get_try = |value: u8, byte_ty: ByteType| -> String {
                            match byte_ty.ty {
                                ByteInnerTy::Byte => value.to_string(),
                                ByteInnerTy::Definer(_) => format!("{value}.try_into()"),
                            }
                        };

                        let a = get_try(a, a_ty);
                        let b = get_try(b, b_ty);
                        let c = get_try(c, c_ty);
                        let d = get_try(d, d_ty);

                        s.wln(format!(
                            ".set_{ty}_{field}({a}.unwrap(), {b}.unwrap(), {c}.unwrap(), {d}.unwrap())",
                            ty = field.object_ty().to_string().to_lowercase(),
                            field = f.name().to_lowercase(),
                            a = a,
                            b = b,
                            c = c,
                            d = d
                        ))
                    }
                    _ => s.wln(format!(
                        ".set_{ty}_{field}({value})",
                        value = f.value(),
                        ty = f.ty().to_string().to_lowercase(),
                        field = f.name().to_lowercase(),
                    )),
                }
            }

            s.wln(".finalize()");

            s.dec_indent();
            s.wln("),")
        }
    }
}

fn split_u32_str_into_u8s(a: &str) -> (u8, u8, u8, u8) {
    let value = parse_value(a).unwrap() as u32;
    let value = value.to_le_bytes();

    (value[0], value[1], value[2], value[3])
}
