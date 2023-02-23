use crate::parser::types::array::{ArraySize, ArrayType};
use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::objects::Objects;
use crate::parser::types::sizes::{DATETIME_SIZE, GOLD_SIZE, GUID_SIZE};
use crate::parser::types::ty::Type;
use crate::rust_printer::rust_view::{RustMember, RustObject, RustType};
use crate::rust_printer::{
    Writer, CLIENT_MESSAGE_TRAIT_NAME, PARSE_ERROR, SERVER_MESSAGE_TRAIT_NAME,
};
use crate::CONTAINER_SELF_SIZE_FIELD;

pub mod print_read;
pub mod print_write;

pub(crate) fn print_common_impls(s: &mut Writer, e: &Container, o: &Objects) {
    print_world_message_headers_and_constants(s, e);

    match e.container_type() {
        ContainerType::Struct => {
            let create_async_reads = e.tags().has_login_version();

            let visibility = if e.tags().is_in_base() {
                "pub"
            } else {
                "pub(crate)"
            };

            let error_name = if e.only_has_io_errors() {
                "std::io::Error"
            } else {
                PARSE_ERROR
            };

            s.impl_read_write_struct(
                e.name(),
                |s, it| {
                    print_read::print_read(s, e, o, it.prefix(), it.postfix());
                },
                |s, it| {
                    print_write::print_write(s, e, o, it.prefix(), it.postfix());
                },
                create_async_reads,
                error_name,
                visibility,
            );
        }
        ContainerType::CLogin(opcode) | ContainerType::SLogin(opcode) => {
            let mut sizes = e.sizes();
            let opcode_size = 1;
            sizes.inc_both(opcode_size);

            let trait_to_impl = match e.container_type() {
                ContainerType::CLogin(_) => CLIENT_MESSAGE_TRAIT_NAME,
                ContainerType::SLogin(_) => SERVER_MESSAGE_TRAIT_NAME,
                _ => unreachable!("Login branch has non login container"),
            };

            s.impl_read_and_writable_login(
                e.name(),
                opcode,
                trait_to_impl,
                |s, it| {
                    print_read::print_read(s, e, o, it.prefix(), it.postfix());
                },
                |s, it| {
                    print_write::print_unencrypted_write_header(s, e, it.postfix());
                    print_write::print_write(s, e, o, it.prefix(), it.postfix());
                },
                sizes,
            );
        }
        ContainerType::Msg(opcode) | ContainerType::CMsg(opcode) | ContainerType::SMsg(opcode) => {
            let bind = |s: &mut Writer, container_type, version| {
                s.impl_world_server_or_client_message(e.name(), container_type, version);
            };

            s.impl_world_message(
                e.name(),
                opcode,
                |s, it| {
                    print_write::print_write(s, e, o, it.prefix(), it.postfix());
                },
                |s, it| {
                    test_for_invalid_size(s, e);
                    print_read::print_read(s, e, o, it.prefix(), it.postfix());
                },
                Some(e.sizes()),
            );

            for version in e.tags().main_versions() {
                match e.container_type() {
                    ContainerType::CMsg(_) => bind(s, ContainerType::CMsg(0), version),
                    ContainerType::SMsg(_) => bind(s, ContainerType::SMsg(0), version),
                    ContainerType::Msg(_) => {
                        bind(s, ContainerType::CMsg(0), version);

                        bind(s, ContainerType::SMsg(0), version);
                    }
                    _ => unreachable!("non world container in world branch"),
                }
            }
        }
    }

    print_size_rust_view(s, e, "self.");

    // Compressed messages need an additional size implementation that measures the non-compressed size of the message.
    // This is used to calculate the decompressed_size field, a u32 written at the start of every compressed packet.
    if e.tags().compressed() {
        print_size_uncompressed_rust_view(s, e.rust_object(), "self.", "size_uncompressed");
    }
}

fn test_for_invalid_size(s: &mut Writer, e: &Container) {
    let header = match e.is_constant_sized() {
        true => format!("if body_size != {}", e.sizes().maximum()),
        false => {
            let min = e.sizes().minimum();
            let max = if e.sizes().maximum() >= u32::MAX as usize {
                (u32::MAX - 1) as usize // More realistic number here?
            } else {
                e.sizes().maximum()
            };
            if min == 0 {
                format!("if body_size > {max}",)
            } else {
                format!("if !({min}..={max}).contains(&body_size)",)
            }
        }
    };
    s.bodyn(header, |s| {
        s.wln(format!(
            "return Err({}::InvalidSize {{ opcode: {:#06X}, size: body_size as u32 }});",
            PARSE_ERROR,
            e.opcode(),
        ));
    });
}

fn print_world_message_headers_and_constants(s: &mut Writer, e: &Container) {
    if e.any_fields_have_constant_value() {
        s.bodyn(format!("impl {name}", name = e.name()), |s| {
            for d in e.all_definitions() {
                if let Some(v) = d.value() {
                    if v.original_string() == CONTAINER_SELF_SIZE_FIELD {
                        continue;
                    }
                    print_constant_member(s, d.name(), d.ty(), v.original_string(), v.value());
                }
            }
        });
    }
}

pub(crate) fn print_constant_member(
    s: &mut Writer,
    name: &str,
    ty: &Type,
    original_value: &str,
    value: u64,
) {
    s.docc(format!("The field `{name}` is constantly specified to be:"));
    s.docc_newline();
    s.docc("| Format | Value |");
    s.docc("| ------ | ----- |");
    s.docc(format!("| Decimal | `{value}` |"));
    s.docc(format!("| Hex | `{value:#04x}` |"));
    s.docc(format!("| Original | `{original_value}` |"));
    s.docc_newline();
    s.docc("**This field is not in the Rust struct, but is written as this constant value.**");

    s.wln(format!(
        "pub const {name}_VALUE: {ty} = {value:#04x};\n",
        name = name.to_uppercase(),
        ty = ty.rust_str(),
        value = value,
    ));
}

pub(crate) fn print_rust_members_sizes(
    s: &mut Writer,
    members: &[RustMember],
    is_elseif: Option<bool>,
    prefix: &str,
) {
    let mut i = 0;
    for m in members {
        if m.tags().skip_serialize() {
            continue;
        }

        let is_elseif = if let Some(b) = is_elseif { b } else { true };
        if i == 0 && is_elseif {
            s.w("");
        } else {
            s.w("+ ");
        }

        print_size_of_ty_rust_view(s, m, prefix);

        i += 1;
    }
}

pub(crate) fn print_size_of_ty_rust_view(s: &mut Writer, m: &RustMember, prefix: &str) {
    let str = match m.ty() {
        RustType::Bool(i) => format!("{}", i.size()),
        RustType::Integer(i) => i.size().to_string(),
        RustType::Floating(f) => f.size().to_string(),
        RustType::Guid => GUID_SIZE.to_string(),
        RustType::Gold => GOLD_SIZE.to_string(),
        RustType::DateTime => DATETIME_SIZE.to_string(),
        RustType::String => format!("{prefix}{name}.len() + 1", name = m.name(), prefix = prefix),
        RustType::CString => format!("{prefix}{name}.len() + 1", name = m.name(), prefix = prefix),
        RustType::SizedCString => {
            format!("{prefix}{name}.len() + 5", name = m.name(), prefix = prefix)
        }
        RustType::Struct { sizes, .. } => {
            if let Some(size) = sizes.is_constant() {
                format!("{size}")
            } else {
                format!("{prefix}{name}.size()", prefix = prefix, name = m.name())
            }
        }
        RustType::EnchantMask
        | RustType::InspectTalentGearMask
        | RustType::MonsterMoveSpline
        | RustType::AchievementDoneArray
        | RustType::AchievementInProgressArray
        | RustType::PackedGuid
        | RustType::UpdateMask
        | RustType::AuraMask => {
            format!("{prefix}{name}.size()", prefix = prefix, name = m.name())
        }
        RustType::Enum {
            is_simple, int_ty, ..
        }
        | RustType::Flag {
            is_simple, int_ty, ..
        } => {
            if !is_simple {
                format!("{prefix}{name}.size()", name = m.name(), prefix = prefix)
            } else {
                int_ty.size().to_string()
            }
        }
        RustType::Array {
            array, inner_sizes, ..
        } => {
            let inner_is_constant = inner_sizes.is_constant().is_some();
            match array.ty() {
                ArrayType::Integer(integer_type) => match array.size() {
                    ArraySize::Fixed(fixed_value) => format!(
                        "{array_size} * core::mem::size_of::<{ty}>()",
                        array_size = fixed_value,
                        ty = integer_type.rust_str(),
                    ),
                    ArraySize::Variable(_) | ArraySize::Endless => {
                        // ZLib compression is not predictable, so we compress the data and count the bytes.
                        if m.tags().is_compressed() {
                            format!(
                                "crate::util::zlib_compressed_size({ref}{prefix}{name})",
                                prefix = prefix,
                                ref = if prefix.is_empty() { "" } else { "&" },
                                name = m.name()
                            )
                        } else {
                            format!(
                                "{prefix}{name}.len() * core::mem::size_of::<{ty}>()",
                                name = m.name(),
                                prefix = prefix,
                                ty = integer_type.rust_str(),
                            )
                        }
                    }
                },
                ArrayType::Struct(_) => match array.size() {
                    ArraySize::Fixed(fixed_value) => match m.constant_sized() {
                        true => format!(
                            "{fixed_value} * {inner_type_size}",
                            inner_type_size = inner_sizes.maximum(),
                            fixed_value = fixed_value,
                        ),
                        false => format!(
                            "{prefix}{name}.iter().fold(0, |acc, x| acc + x.size())",
                            name = m.name(),
                            prefix = prefix,
                        ),
                    },
                    ArraySize::Variable(_) | ArraySize::Endless => {
                        // ZLib compression is not predictable, so we need to serialise each array member into a Vec
                        // and compress the entire buffer to calculate the true size of the packet.
                        if m.tags().is_compressed() {
                            format!(
                                "crate::util::zlib_compressed_size({prefix}{name}.iter().fold(Vec::new(), |mut acc, x| {{ x.write_into_vec(&mut acc).unwrap(); acc }} ).as_slice())",
                                prefix = prefix,
                                name = m.name()
                            )
                        } else {
                            match inner_is_constant {
                                true => {
                                    format!(
                                        "{prefix}{name}.len() * {size}",
                                        name = m.name(),
                                        prefix = prefix,
                                        size = inner_sizes.maximum(),
                                    )
                                }
                                false => {
                                    format!(
                                        "{prefix}{name}.iter().fold(0, |acc, x| acc + x.size())",
                                        name = m.name(),
                                        prefix = prefix,
                                    )
                                }
                            }
                        }
                    }
                },
                ArrayType::CString => {
                    format!(
                        "{prefix}{name}.iter().fold(0, |acc, x| acc + x.len() + 1)",
                        name = m.name(),
                        prefix = prefix,
                    )
                }
                ArrayType::Guid => {
                    format!(
                        "{prefix}{name}.iter().fold(0, |acc, _| acc + 8)",
                        name = m.name(),
                        prefix = prefix,
                    )
                }
                ArrayType::PackedGuid => {
                    format!(
                        "{prefix}{name}.iter().fold(0, |acc, x| acc + x.size())",
                        name = m.name(),
                        prefix = prefix,
                    )
                }
            }
        }
    };
    s.w_no_indent(str);
    s.wln_no_indent(m.size_comment());
}

pub(crate) fn print_size_rust_view(s: &mut Writer, c: &Container, prefix: &str) {
    let r = c.rust_object();

    if !r.constant_sized() {
        if c.tags().compressed() {
            s.variable_size(r.name(), "size", |s| {
                s.wln("use crate::traits::Message;");
                s.newline();

                s.wln("let mut v = Vec::new();");
                s.wln("self.write_into_vec(&mut v);");
                s.wln("v.len()");
            });
        } else {
            print_size_uncompressed_rust_view(s, r, prefix, "size");
        }
    }
}

pub(crate) fn print_size_uncompressed_rust_view(
    s: &mut Writer,
    r: &RustObject,
    prefix: &str,
    function_name: &str,
) {
    if !r.constant_sized() {
        s.variable_size(r.name(), function_name, |s| {
            print_rust_members_sizes(s, r.members(), None, prefix);

            if let Some(optional) = r.optional() {
                s.body_else(
                    format!(
                        "{plus}if let Some({name}) = &{prefix}{name}",
                        name = optional.name(),
                        prefix = prefix,
                        plus = match r.members().is_empty() {
                            true => "",
                            false => "+ ",
                        }
                    ),
                    |s| {
                        let prefix = format!("{}.", optional.name());
                        print_rust_members_sizes(s, optional.members(), None, &prefix);
                    },
                    |s| {
                        s.wln("0");
                    },
                );
            }
        });
    }
}
