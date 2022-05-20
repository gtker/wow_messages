use crate::container::{Container, ContainerType, StructMember, GUID_SIZE};
use crate::parser::types::objects::Objects;
use crate::parser::types::ty::Type;
use crate::parser::types::{ArraySize, ArrayType};
use crate::rust_printer::rust_view::{RustMember, RustObject, RustType};
use crate::rust_printer::{
    Writer, LOGIN_CLIENT_MESSAGE_TRAIT_NAME, LOGIN_SERVER_MESSAGE_TRAIT_NAME,
    WORLD_CLIENT_HEADER_TRAIT_NAME, WORLD_SERVER_HEADER_TRAIT_NAME,
};
use crate::CONTAINER_SELF_SIZE_FIELD;

pub mod print_read;
pub mod print_write;

pub fn print_common_impls(s: &mut Writer, e: &Container, o: &Objects) {
    print_world_message_headers_and_constants(s, e);
    let error_ty = match e.only_has_io_errors() {
        true => "std::io::Error".to_string(),
        false => format!("{}Error", e.name()),
    };

    match e.container_type() {
        ContainerType::Struct => {
            s.impl_read_write_non_trait(
                e.name(),
                error_ty,
                |s, it| {
                    print_read::print_read(s, e, o, it.prefix(), it.postfix());
                },
                |s, it| {
                    print_write::print_write_no_ok(s, e, o, it.prefix(), it.postfix());
                },
            );
        }
        ContainerType::CLogin(_) | ContainerType::SLogin(_) => {
            s.impl_read_and_writable_with_error(
                e.name(),
                &error_ty,
                |s, it| {
                    print_read::print_read(s, e, o, it.prefix(), it.postfix());
                },
                |s, it| {
                    print_write::print_unencrypted_write_header(s, e, it.postfix());
                    print_write::print_write_no_ok(s, e, o, it.prefix(), it.postfix());
                },
            );
        }
        ContainerType::Msg(opcode) | ContainerType::CMsg(opcode) | ContainerType::SMsg(opcode) => {
            s.impl_world_read_and_writable_with_error(
                e.name(),
                error_ty,
                opcode,
                e.rust_object().constant_sized(),
                |s, it| {
                    print_read::print_read(s, e, o, it.prefix(), it.postfix());
                },
                |s, it| {
                    print_write::print_write_no_ok(s, e, o, it.prefix(), it.postfix());
                },
            );
        }
    }

    //print_size(s, e, o);
    print_size_rust_view(s, e.rust_object(), "self.");
}

pub fn print_constant(s: &mut Writer, m: &StructMember) {
    match m {
        StructMember::Definition(d) => {
            if let Some(v) = d.verified_value() {
                if v.original_string() == CONTAINER_SELF_SIZE_FIELD {
                    return;
                }
                print_constant_member(s, d.name(), d.ty(), v.original_string(), v.value());
            }
        }
        StructMember::IfStatement(statement) => {
            for member in statement.all_members() {
                print_constant(s, member);
            }
        }
        StructMember::OptionalStatement(optional) => {
            for member in optional.members() {
                print_constant(s, member);
            }
        }
    }
}

fn print_world_message_headers_and_constants(s: &mut Writer, e: &Container) {
    let empty_impl = |s: &mut Writer, t| {
        s.wln(format!("impl {} for {} {{}}", t, e.name()));
        s.newline();
    };

    match e.container_type() {
        ContainerType::Struct => {}
        ContainerType::CMsg(_) => {
            empty_impl(s, WORLD_CLIENT_HEADER_TRAIT_NAME);
        }
        ContainerType::SMsg(_) => {
            empty_impl(s, WORLD_SERVER_HEADER_TRAIT_NAME);
        }
        ContainerType::Msg(_) => {
            empty_impl(s, WORLD_CLIENT_HEADER_TRAIT_NAME);
            empty_impl(s, WORLD_SERVER_HEADER_TRAIT_NAME);
        }
        ContainerType::CLogin(v) => {
            s.body(
                format!("impl {} for {}", LOGIN_CLIENT_MESSAGE_TRAIT_NAME, e.name()),
                |s| {
                    s.wln(format!("const OPCODE: u8 = {:#04x};", v));
                },
            );
        }
        ContainerType::SLogin(v) => {
            s.body(
                format!("impl {} for {}", LOGIN_SERVER_MESSAGE_TRAIT_NAME, e.name()),
                |s| {
                    s.wln(format!("const OPCODE: u8 = {:#04x};", v));
                },
            );
        }
    }

    if e.any_fields_have_constant_value() {
        s.bodyn(format!("impl {name}", name = e.name()), |s| {
            for m in e.fields() {
                print_constant(s, m);
            }
        });
    }
}

pub fn print_constant_member(
    s: &mut Writer,
    name: &str,
    ty: &Type,
    original_value: &str,
    value: u64,
) {
    s.docc(format!(
        "The field `{name}` is constantly specified to be:",
        name = name
    ));
    s.docc("");
    s.docc("| Format | Value |");
    s.docc("| ------ | ----- |");
    s.docc(format!("| Decimal | `{value}` |", value = value));
    s.docc(format!("| Hex | `{value:#04x}` |", value = value));
    s.docc(format!("| Original | `{value}` |", value = original_value));
    s.docc("");
    s.docc("**This field is not in the struct, but is written as this constant value.**");

    s.wln(format!(
        "pub const {name}_VALUE: {ty} = {value:#04x};\n",
        name = name.to_uppercase(),
        ty = ty.rust_str(),
        value = value,
    ));
}

pub fn print_size_of_ty_rust_view(s: &mut Writer, m: &RustMember, prefix: &str) {
    let str = match m.ty() {
        RustType::Integer(i) => i.size().to_string(),
        RustType::Floating(f) => f.size().to_string(),
        RustType::Guid => GUID_SIZE.to_string(),
        RustType::String => format!("{prefix}{name}.len()", name = m.name(), prefix = prefix),
        RustType::CString => format!("{prefix}{name}.len() + 1", name = m.name(), prefix = prefix),
        RustType::Struct(_) | RustType::PackedGuid | RustType::UpdateMask | RustType::AuraMask => {
            let prefixes = if m.constant_sized() {
                format!("{ty_name}::", ty_name = m.ty())
            } else {
                format!("{prefix}{name}.", name = m.name(), prefix = prefix,)
            };
            format!("{prefixes}size()", prefixes = prefixes,)
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
            array,
            inner_is_constant,
        } => match array.ty() {
            ArrayType::Integer(integer_type) => match array.size() {
                ArraySize::Fixed(fixed_value) => format!(
                    "{array_size} * core::mem::size_of::<{ty}>()",
                    array_size = fixed_value,
                    ty = integer_type.rust_str(),
                ),
                ArraySize::Variable(_) | ArraySize::Endless => {
                    format!(
                        "{prefix}{name}.len() * core::mem::size_of::<{ty}>()",
                        name = m.name(),
                        prefix = prefix,
                        ty = integer_type.rust_str(),
                    )
                }
            },
            ArrayType::Complex(complex_type) => match array.size() {
                ArraySize::Fixed(fixed_value) => match m.constant_sized() {
                    true => format!(
                        "{fixed_value} * {inner_type_name}::size()",
                        inner_type_name = complex_type,
                        fixed_value = fixed_value,
                    ),
                    false => format!(
                        "{prefix}{name}.iter().fold(0, |acc, x| acc + x.size())",
                        name = m.name(),
                        prefix = prefix,
                    ),
                },
                ArraySize::Variable(_) | ArraySize::Endless => {
                    format!(
                        "{prefix}{name}.iter().fold(0, |acc, x| acc + {size_function})",
                        name = m.name(),
                        prefix = prefix,
                        size_function = match inner_is_constant {
                            true => format!("{type_name}::size()", type_name = complex_type),
                            false => "x.size()".to_string(),
                        },
                    )
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
        },
    };
    s.w_no_indent(str);
    s.wln_no_indent(m.size_comment());
}

fn print_maximum_possible_size(s: &mut Writer, r: &RustObject) {
    if r.sizes().maximum() > u16::MAX.into() {
        s.wln(format!(
            "{} // Capped at u16::MAX due to size field.",
            u16::MAX
        ));
    } else {
        s.wln("0");

        for m in r.members() {
            s.w("+ ");

            s.w_no_indent(format!("{}", m.sizes().maximum()));

            s.wln_no_indent(format!(
                " // {name}: {ty}",
                name = m.name(),
                ty = m.ty().str()
            ));
        }
    }
}

pub fn print_size_rust_view(s: &mut Writer, r: &RustObject, prefix: &str) {
    if r.constant_sized() {
        s.constant_sized(r.name(), |s| {
            print_maximum_possible_size(s, r);
        });
    } else {
        s.variable_size(r.name(), |s| {
            s.wln("0");

            for m in r.members() {
                s.w("+ ");

                print_size_of_ty_rust_view(s, m, prefix);
            }

            if let Some(optional) = r.optional() {
                s.body_else(
                    format!(
                        "+ if let Some({name}) = &{prefix}{name}",
                        name = optional.name(),
                        prefix = prefix
                    ),
                    |s| {
                        s.wln("0");

                        let prefix = format!("{}.", optional.name());
                        for m in optional.members_in_struct() {
                            s.w("+ ");

                            print_size_of_ty_rust_view(s, m, &prefix);
                        }
                    },
                    |s| {
                        s.wln("0");
                    },
                );
            }
        });
    }
}
