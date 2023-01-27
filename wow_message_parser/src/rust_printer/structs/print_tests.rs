use crate::file_utils::{get_import_path, major_version_to_string};
use crate::parser::types::array::ArraySize;
use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::objects::Objects;
use crate::parser::types::test_case::{TestCase, TestCaseMember, TestValue};
use crate::parser::types::version::{MajorWorldVersion, Version};
use crate::parser::utility::parse_value;
use crate::rust_printer::opcodes::get_enumerator_name;
use crate::rust_printer::rust_view::{RustEnumerator, RustMember, RustType};
use crate::rust_printer::update_mask::vanilla_fields::FIELDS;
use crate::rust_printer::{
    get_new_flag_type_name, ByteInnerTy, ByteType, ImplType, UfType, UpdateMaskType, Writer,
    CLIENT_MESSAGE_TRAIT_NAME, LOGIN_CLIENT_MESSAGE_ENUM_NAME, LOGIN_SERVER_MESSAGE_ENUM_NAME,
    SERVER_MESSAGE_TRAIT_NAME, WORLD_CLIENT_MESSAGE_ENUM_NAME, WORLD_SERVER_MESSAGE_ENUM_NAME,
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

        print_includes(e, version, s);

        for (i, t) in e.tests(o).iter().enumerate() {
            s.w(format!("const RAW{}: [u8; {}] = [", i, t.raw_bytes().len()));
            s.inc_indent();

            for i in t.raw_bytes() {
                s.w_break_at(format!(" {i:#04X},"), 80);
            }

            s.dec_indent();
            s.wln_no_indent(" ];\n");

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
                        print_test_case(s, t, e, it, i, version);
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

fn print_test_case(
    s: &mut Writer,
    t: &TestCase,
    e: &Container,
    it: ImplType,
    i: usize,
    version: Version,
) {
    s.body_closing_with(
        format!("let expected = {}", t.subject()),
        |s| {
            for m in e.rust_object().members_in_struct() {
                print_value(s, m, t.members(), e, version);
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
            s.wln(format!(
                "assert_eq!(t.size() + header_size, RAW{i}.len());"
            ));
        }
        true => {
            let size = if e.sizes().maximum() == 0 {
                "".to_string()
            } else {
                format!("{} + ", e.sizes().maximum())
            };

            s.wln(format!(
                "assert_eq!({size}header_size, RAW{i}.len());",
            ));
        }
    }
    s.newline();

    s.wln(format!(
        "let mut dest = Vec::with_capacity(RAW{i}.len());"
    ));
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

    s.wln(format!("assert_eq!(dest, RAW{i});"))
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
        TestValue::DateTime(i) => {
            s.wln_no_indent(format!("DateTime::try_from({:#X}).unwrap(),", i.value()));
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
        TestValue::Array { values, size } => {
            match size {
                ArraySize::Fixed(_) => s.w_no_indent("["),
                ArraySize::Variable(_) | ArraySize::Endless => s.w_no_indent("vec!["),
            }
            s.inc_indent();

            for value in values {
                s.w_break_at(format!(" {value:#04X},"), 80);
            }

            s.dec_indent();
            s.wln_no_indent(" ],");
        }
        TestValue::ArrayOfSubObject(array_container, multiples) => {
            if multiples.is_empty() {
                s.wln_no_indent("vec![],");
                return;
            }

            s.wln_no_indent("vec![");
            s.inc_indent();

            for multiple in multiples {
                s.wln(format!("{} {{", array_container.name()));
                s.inc_indent();

                for m in array_container.rust_object().members_in_struct() {
                    print_value(s, m, multiple, array_container, version);
                }

                s.closing_curly_with(",");
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
                        s.wln(format!(".set_{}()", enumerator.name()));
                        continue;
                    }

                    match elseif_ty {
                        EnumeratorType::Regular => {
                            s.open_curly(format!(
                                ".set_{}({}",
                                enumerator.name(),
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
                                parent_enum_name,
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
                .find(|a| a.ty() == UpdateMaskType::Object && a.name() == "TYPE")
                .unwrap();
            let ty = parse_value(ty.value()).unwrap();

            const ITEM: u64 = 0x0002;
            const CONTAINER: u64 = 0x0004;
            const UNIT: u64 = 0x0008;
            const PLAYER: u64 = 0x0010;
            const GAMEOBJECT: u64 = 0x0020;
            const DYNAMICOBJECT: u64 = 0x0040;
            const CORPSE: u64 = 0x0080;
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

                let fields = match version.as_major_world() {
                    MajorWorldVersion::Vanilla => FIELDS.as_slice(),
                    MajorWorldVersion::BurningCrusade => unimplemented!(),
                    MajorWorldVersion::Wrath => unimplemented!(),
                };

                let field = fields
                    .iter()
                    .find(|a| a.object_ty() == f.ty() && a.name() == f.name())
                    .unwrap()
                    .clone();

                match field.ty() {
                    UfType::Guid => {
                        s.wln(format!(
                            ".set_{ty}_{field}(Guid::new({value}))",
                            value = f.value(),
                            ty = f.ty(),
                            field = f.name()
                        ));
                    }
                    UfType::Bytes => {
                        let (a, b, c, d) = split_u32_str_into_u8s(f.value());

                        s.wln(format!(
                            ".set_{ty}_{field}({a}, {b}, {c}, {d})",
                            ty = field.object_ty(),
                            field = f.name(),
                            a = a,
                            b = b,
                            c = c,
                            d = d
                        ))
                    }
                    UfType::BytesWith(a_ty, b_ty, c_ty, d_ty) => {
                        let (a, b, c, d) = split_u32_str_into_u8s(f.value());

                        let get_try = |value: u8, byte_ty: ByteType| -> String {
                            match byte_ty.ty {
                                ByteInnerTy::Byte => value.to_string(),
                                ByteInnerTy::Ty(_) => format!("{value}.try_into()"),
                            }
                        };

                        let a = get_try(a, a_ty);
                        let b = get_try(b, b_ty);
                        let c = get_try(c, c_ty);
                        let d = get_try(d, d_ty);

                        s.wln(format!(
                            ".set_{ty}_{field}({a}.unwrap(), {b}.unwrap(), {c}.unwrap(), {d}.unwrap())",
                            ty = field.object_ty(),
                            field = f.name(),
                            a = a,
                            b = b,
                            c = c,
                            d = d
                        ))
                    }
                    _ => s.wln(format!(
                        ".set_{ty}_{field}({value})",
                        value = f.value(),
                        ty = f.ty(),
                        field = f.name()
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
