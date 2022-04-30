use crate::container::{Container, ContainerType, StructMember};
use crate::parser::types::objects::Objects;
use crate::parser::types::ty::Type;
use crate::parser::types::{ArraySize, ArrayType, IntegerType};
use crate::rust_printer::{
    Writer, LOGIN_CLIENT_MESSAGE_TRAIT_NAME, LOGIN_SERVER_MESSAGE_TRAIT_NAME,
    WORLD_CLIENT_HEADER_TRAIT_NAME, WORLD_SERVER_HEADER_TRAIT_NAME,
};
use crate::CSTRING_LARGEST_ALLOWED;

pub mod print_read;
pub mod print_write;

pub fn print_common_impls(s: &mut Writer, e: &Container, o: &Objects) {
    print_world_message_headers_and_constants(s, e);
    let error_ty = match e.only_has_io_errors() {
        true => "std::io::Error".to_string(),
        false => format!("{}Error", e.name()),
    };

    match e.container_type() {
        ContainerType::Struct | ContainerType::CLogin(_) | ContainerType::SLogin(_) => {
            s.impl_read_and_writable_with_error(
                e.name(),
                &error_ty,
                |s, it| {
                    print_read::print_read(s, e, o, it.prefix(), it.postfix());
                },
                |s, it| {
                    print_write::print_unencrypted_write_header(s, e, it.prefix(), it.postfix());
                    print_write::print_write(s, e, o, it.prefix(), it.postfix());
                },
            );
        }
        ContainerType::Msg(_) | ContainerType::CMsg(_) | ContainerType::SMsg(_) => {
            s.impl_world_read_and_writable_with_error(
                e.name(),
                error_ty,
                |s, it| {
                    print_read::print_read(s, e, o, it.prefix(), it.postfix());
                },
                |s, it| {
                    print_write::print_write(s, e, o, it.prefix(), it.postfix());
                },
            );
        }
    }

    print_size(s, e, o);
}

fn print_world_message_headers_and_constants(s: &mut Writer, e: &Container) {
    match e.container_type() {
        ContainerType::Struct => {}
        ContainerType::CMsg(v) => {
            print_client_message_header(s, e, v);
        }
        ContainerType::SMsg(v) => {
            print_server_message_header(s, e, v);
        }
        ContainerType::Msg(v) => {
            print_client_message_header(s, e, v);
            print_server_message_header(s, e, v);
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

    fn print_constant(s: &mut Writer, m: &StructMember) {
        match m {
            StructMember::Definition(d) => {
                if let Some(v) = d.verified_value() {
                    if v.original_string() == "self.size" {
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

    if e.any_fields_have_constant_value() {
        s.bodyn(format!("impl {name}", name = e.name()), |s| {
            for m in e.fields() {
                print_constant(s, m);
            }
        });
    }
}

fn print_client_message_header(s: &mut Writer, e: &Container, v: u16) {
    s.bodyn(
        format!("impl {} for {}", WORLD_CLIENT_HEADER_TRAIT_NAME, e.name()),
        |s| {
            s.wln(format!("const OPCODE: u32 = {:#04x};", v));
            s.newline();

            s.bodyn("fn size_without_size_field(&self) -> u16", |s| {
                s.wln(format!(
                    "{}",
                    match e.is_constant_sized() {
                        true => "Self::size() as u16",
                        false => "self.size() as u16",
                    }
                ))
            });
        },
    );
}

fn print_server_message_header(s: &mut Writer, e: &Container, v: u16) {
    s.bodyn(
        format!("impl {} for {}", WORLD_SERVER_HEADER_TRAIT_NAME, e.name()),
        |s| {
            s.wln(format!("const OPCODE: u16 = {:#04x};", v));
            s.newline();
            s.bodyn("fn size_without_size_field(&self) -> u16", |s| {
                s.wln(format!(
                    "{}",
                    match e.is_constant_sized() {
                        true => "Self::size() as u16",
                        false => "self.size() as u16",
                    }
                ))
            });
        },
    );
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

pub fn print_size_of_ty(
    s: &mut Writer,
    ty: &Type,
    variable_name: &str,
    d_does_not_have_subvariables: bool,
    o_type_has_constant_size: bool,
    array_inner_has_constant: bool,
    prefix: &str,
    original_ty_name: &str,
) {
    match ty {
        Type::Integer(integer_type) => {
            s.w_no_indent(format!("{size}", size = integer_type.size(),))
        }
        Type::FloatingPoint(floating) => s.w_no_indent(format!("{size}", size = floating.size(),)),
        Type::CString => s.w_no_indent(format!(
            "{prefix}{name}.len() + 1",
            prefix = prefix,
            name = variable_name
        )),
        Type::String { .. } => s.w_no_indent(format!(
            "{prefix}{name}.len()",
            name = variable_name,
            prefix = prefix,
        )),
        Type::Array(array) => match array.ty() {
            ArrayType::Integer(integer_type) => match array.size() {
                ArraySize::Fixed(fixed_value) => s.w_no_indent(format!(
                    "{array_size} * core::mem::size_of::<{ty}>()",
                    array_size = fixed_value,
                    ty = integer_type.rust_str(),
                )),
                ArraySize::Variable(_) | ArraySize::Endless => {
                    s.w_no_indent(format!(
                        "{prefix}{name}.len() * core::mem::size_of::<{ty}>()",
                        name = variable_name,
                        prefix = prefix,
                        ty = integer_type.rust_str(),
                    ));
                }
            },
            ArrayType::Complex(complex_type) => match array.size() {
                ArraySize::Fixed(fixed_value) => match o_type_has_constant_size {
                    true => s.w_no_indent(format!(
                        "{fixed_value} * {inner_type_name}::size()",
                        inner_type_name = complex_type,
                        fixed_value = fixed_value,
                    )),
                    false => s.w_no_indent(format!(
                        "{prefix}{name}.iter().fold(0, |acc, x| acc + x.size())",
                        name = variable_name,
                        prefix = prefix,
                    )),
                },
                ArraySize::Variable(_) | ArraySize::Endless => {
                    s.w_no_indent(format!(
                        "{prefix}{name}.iter().fold(0, |acc, x| acc + {size_function})",
                        name = variable_name,
                        prefix = prefix,
                        size_function = match array_inner_has_constant {
                            true => format!("{type_name}::size()", type_name = complex_type),
                            false => "x.size()".to_string(),
                        },
                    ));
                }
            },
            ArrayType::CString => {
                s.w_no_indent(format!(
                    "{prefix}{name}.iter().fold(0, |acc, x| acc + x.len() + 1)",
                    name = variable_name,
                    prefix = prefix,
                ));
            }
            ArrayType::Guid => {
                s.w_no_indent(format!(
                    "{prefix}{name}.iter().fold(0, |acc, _| acc + 8)",
                    name = variable_name,
                    prefix = prefix,
                ));
            }
            ArrayType::PackedGuid => {
                s.w_no_indent(format!(
                    "{prefix}{name}.iter().fold(0, |acc, x| acc + x.size())",
                    name = variable_name,
                    prefix = prefix,
                ));
            }
        },
        Type::Identifier {
            s: identifier,
            upcast,
        } => {
            if let Some(upcast) = upcast {
                s.w_no_indent(format!("{size}", size = upcast.size(),));
            } else if d_does_not_have_subvariables {
                if o_type_has_constant_size {
                    s.w_no_indent(format!("{type_name}::size()", type_name = identifier,));
                } else {
                    s.w_no_indent(format!(
                        "{prefix}{name}.size()",
                        name = variable_name,
                        prefix = prefix,
                    ));
                }
            } else {
                s.w_no_indent(format!(
                    "{prefix}{name}.size()",
                    name = variable_name,
                    prefix = prefix,
                ));
            }
        }
        Type::Guid => {
            s.w_no_indent("8");
        }
        Type::UpdateMask | Type::AuraMask | Type::PackedGuid => {
            s.w_no_indent(format!(
                "{prefix}{name}.size()",
                name = variable_name,
                prefix = prefix,
            ));
        }
    }

    print_comment_for_size_of_ty(
        s,
        ty,
        variable_name,
        d_does_not_have_subvariables,
        original_ty_name,
    );
}

fn print_comment_for_size_of_ty(
    s: &mut Writer,
    ty: &Type,
    variable_name: &str,
    d_does_not_have_subvariables: bool,
    original_ty_name: &str,
) {
    s.w_no_indent(format!(
        " // {name}: {type_name}",
        name = variable_name,
        type_name = original_ty_name,
    ));

    match ty {
        Type::CString => {
            s.w_no_indent(" and Null Terminator");
        }
        Type::Identifier { upcast, .. } => {
            if let Some(upcast) = upcast {
                s.w_no_indent(format!(" upcasted to {ty}", ty = upcast.rust_str()));
            } else if !d_does_not_have_subvariables {
                s.w_no_indent(" and subfields");
            }
        }
        _ => {}
    }

    s.wln_no_indent("");
}

fn print_size(s: &mut Writer, e: &Container, o: &Objects) {
    let size_func = |s: &mut Writer| {
        if e.fields().is_empty() {
            s.wln("0");
            return;
        }
        let types = e.nested_types().declarations();
        for (i, d) in types.iter().enumerate() {
            match i != 0 {
                true => s.w("+ "),
                false => s.w(""),
            }

            let array_inner_constant = match d.ty() {
                Type::Array(array) => match array.ty() {
                    ArrayType::Integer(_) => true,
                    ArrayType::Complex(ident) => o.type_has_constant_size(&Type::Identifier {
                        s: ident.clone(),
                        upcast: None,
                    }),
                    ArrayType::CString => false,
                    ArrayType::Guid => true,
                    ArrayType::PackedGuid => false,
                },
                _ => false,
            };

            print_size_of_ty(
                s,
                d.ty(),
                d.name(),
                d.does_not_have_subvariables(),
                o.type_has_constant_size(d.ty()),
                array_inner_constant,
                "self.",
                &d.ty().str(),
            );
        }

        if let Some(optional) = e.rust_object().optional() {
            if e.nested_types().declarations().is_empty() {
                s.w("");
            } else {
                s.w("+ ");
            }

            s.wln_no_indent("{");
            s.inc_indent();

            s.body_else(
                format!("if let Some(v) = &self.{name}", name = optional.name()),
                |s| {
                    s.wln("v.size()");
                },
                |s| {
                    s.wln("0");
                },
            );
            s.closing_curly_with(format!(" // optional {name}", name = optional.name()));
        }
    };

    match o.type_has_constant_size(&Type::Identifier {
        s: e.name().to_owned(),
        upcast: None,
    }) {
        true => s.constant_sized(e.name(), size_func),
        false => {
            s.variable_size(e.name(), size_func, |s| {
                for (i, d) in e.nested_types().declarations().iter().enumerate() {
                    match i != 0 {
                        true => s.w("+ "),
                        false => s.w(""),
                    }
                    let array_type_largest_possible_size =
                        get_array_type_largest_possible_value(d.ty(), e);
                    print_maximum_size_of_type(
                        s,
                        d.ty(),
                        d.name(),
                        array_type_largest_possible_size,
                    );
                }

                for m in e.fields() {
                    match m {
                        StructMember::Definition(_) => {}
                        StructMember::IfStatement(_) => {}
                        StructMember::OptionalStatement(optional) => {
                            if e.nested_types().declarations().is_empty() {
                                s.w("");
                            } else {
                                s.w("+ ");
                            }

                            s.wln_no_indent(format!(
                                "65536 // optional {name}",
                                name = optional.name()
                            ));
                        }
                    }
                }
            });
        }
    }
}

pub fn get_array_type_largest_possible_value(ty: &Type, e: &Container) -> usize {
    match ty {
        Type::String { length } => {
            if let Ok(v) = length.parse::<usize>() {
                v
            } else {
                match e.get_type_of_variable(length) {
                    Type::Integer(i) => match i {
                        IntegerType::U8 => u8::MAX as usize,
                        IntegerType::U16(_) => u16::MAX as usize,
                        IntegerType::U32(_) => u32::MAX as usize,
                        IntegerType::U64(_) => u64::MAX as usize,
                    },
                    _ => panic!("non int used as size for string"),
                }
            }
        }
        Type::Array(array) => match array.size() {
            ArraySize::Fixed(_) => 0,
            ArraySize::Variable(v) => match e.get_type_of_variable(&v) {
                Type::Integer(i) => match i {
                    IntegerType::U8 => u8::MAX as usize,
                    IntegerType::U16(_) => u16::MAX as usize,
                    IntegerType::U32(_) => u32::MAX as usize,
                    IntegerType::U64(_) => u64::MAX as usize,
                },
                _ => panic!("non int used as size for array"),
            },
            ArraySize::Endless => 0,
        },
        _ => 0,
    }
}

pub fn print_maximum_size_of_type(
    s: &mut Writer,
    ty: &Type,
    d_name: &str,
    array_type_largest_possible_value: usize,
) {
    let size = match ty {
        Type::Integer(integer) => integer.size().to_string(),
        Type::FloatingPoint(floating) => floating.size().to_string(),
        Type::CString => CSTRING_LARGEST_ALLOWED.to_string(),
        Type::String { .. } => array_type_largest_possible_value.to_string(),
        Type::Array(array) => match array.size() {
            ArraySize::Fixed(v) => match array.ty() {
                ArrayType::Integer(i) => {
                    format!(
                        "{size} * core::mem::size_of::<{ty}>()",
                        size = v,
                        ty = i.rust_str()
                    )
                }
                ArrayType::Complex(i) => {
                    format!("{size} * {ty}::maximum_possible_size()", size = v, ty = i)
                }
                ArrayType::CString => {
                    format!("{size} * {ty}", size = v, ty = CSTRING_LARGEST_ALLOWED)
                }
                ArrayType::Guid => {
                    format!("{size} * 8", size = v)
                }
                ArrayType::PackedGuid => {
                    format!("{size} * 9", size = v)
                }
            },
            ArraySize::Variable(_) => match array.ty() {
                ArrayType::Integer(i) => {
                    format!(
                        "{size} * core::mem::size_of::<{ty}>()",
                        size = array_type_largest_possible_value,
                        ty = i.rust_str(),
                    )
                }
                ArrayType::Complex(i) => {
                    format!(
                        "{size} * {ty}::maximum_possible_size()",
                        size = array_type_largest_possible_value,
                        ty = i
                    )
                }
                ArrayType::CString => {
                    format!(
                        "{size} * {ty}",
                        size = array_type_largest_possible_value,
                        ty = CSTRING_LARGEST_ALLOWED,
                    )
                }
                ArrayType::Guid => {
                    format!("{size} * 8", size = array_type_largest_possible_value,)
                }
                ArrayType::PackedGuid => {
                    format!("{size} * 9", size = array_type_largest_possible_value,)
                }
            },
            ArraySize::Endless => {
                // TODO: Not incorrect
                65536.to_string()
            }
        },
        Type::Identifier { .. } => {
            format!("{ident}::maximum_possible_size()", ident = ty.str())
        }
        Type::PackedGuid => 9.to_string(),
        Type::Guid => 8.to_string(),
        Type::UpdateMask => {
            // TODO: Not inCorrect?
            65536.to_string()
        }
        Type::AuraMask => {
            // TODO: Not inCorrect?
            65536.to_string()
        }
    };

    s.wln_no_indent(format!(
        "{size} // {name}: {ty}",
        size = size,
        name = d_name,
        ty = ty.str()
    ));
}
