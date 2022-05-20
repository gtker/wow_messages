use crate::container::{Container, ContainerType, StructMember};
use crate::file_utils::get_import_path;
use crate::parser::types::objects::Objects;
use crate::parser::types::ty::Type;
use crate::parser::types::{ArraySize, ArrayType, ObjectType};
use crate::rust_printer::{
    Writer, LOGIN_CLIENT_MESSAGE_TRAIT_NAME, LOGIN_SERVER_MESSAGE_TRAIT_NAME,
    WORLD_BODY_TRAIT_NAME, WORLD_CLIENT_HEADER_TRAIT_NAME, WORLD_SERVER_HEADER_TRAIT_NAME,
};
use crate::wowm_printer::get_struct_wowm_definition;

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

    print_new_types::print_new_types(&mut s, e);

    print_tests::print_tests(&mut s, e, o);

    if let Some(optional) = e.rust_object().optional() {
        print_optional::print_optional(&mut s, optional);
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
            s.wln("use crate::ReadableAndWritable;");
        }
        ContainerType::SLogin(_) => {
            s.wln(format!("use crate::{};", LOGIN_SERVER_MESSAGE_TRAIT_NAME));
            s.wln("use crate::ReadableAndWritable;");
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

    match e.container_type() {
        ContainerType::Struct => {
            s.write_async_read_includes();
            s.wln("use std::io::Write;");
        }
        _ => {
            s.write_async_includes();
        }
    }

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
        for member in e.rust_object().members_in_struct() {
            s.wln(format!(
                "pub {name}: {ty},",
                name = member.name(),
                ty = member.ty(),
            ));
        }

        if let Some(optional) = e.rust_object().optional() {
            s.wln(format!(
                "pub {name}: Option<{ty}>,",
                name = optional.name(),
                ty = optional.ty()
            ));
        }
    });
}

fn print_struct_wowm_definition(s: &mut Writer, e: &Container) {
    s.docc_wowm(
        |s| {
            s.w(get_struct_wowm_definition(e, "/// "));
        },
        e.file_info(),
    );
}

fn print_errors(s: &mut Writer, e: &Container, o: &Objects) {
    if e.only_has_io_errors() {
        return;
    }

    print_general_error(s, e, o);
}

fn print_general_error(s: &mut Writer, e: &Container, o: &Objects) {
    s.wln("#[derive(Debug)]");
    s.new_enum("pub", format!("{name}Error", name = e.name()), |s| {
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
    print_from_general_error(s, e, o);
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
