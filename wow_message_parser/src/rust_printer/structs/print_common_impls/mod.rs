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
                error_ty,
                |s| {
                    print_read::print_read(s, e, o);
                },
                |s| {
                    print_write::print_unencrypted_write_header(s, e);
                    print_write::print_write(s, e, o);
                },
            );
        }
        ContainerType::Msg(_) | ContainerType::CMsg(_) | ContainerType::SMsg(_) => {
            s.impl_world_read_and_writable_with_error(
                e.name(),
                error_ty,
                |s| {
                    print_read::print_read(s, e, o);
                },
                |s| {
                    print_write::print_write(s, e, o);
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
    s.body(
        format!("impl {} for {}", WORLD_CLIENT_HEADER_TRAIT_NAME, e.name()),
        |s| {
            s.wln(format!("const OPCODE: u32 = {:#04x};", v));
            s.newline();

            s.bodyn("fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error>", |s| {
                s.wln("// size: u16_be, and opcode: u32");
                s.wln(format!(
                    "{import_path}::write_u16_be(w, ({size}size() + 4) as u16)?;",
                    import_path = "crate::util",
                    size = match e.is_constant_sized() {
                        true => "Self::",
                        false => "self.",
                    }
                ));
                s.wln(format!(
                    "{import_path}::write_u32_le(w, <Self as {header}>::OPCODE)?;",
                    import_path = "crate::util",
                    header = WORLD_CLIENT_HEADER_TRAIT_NAME,
                ));
                s.newline();

                s.wln("self.write_body(w)?;");
                s.wln("Ok(())");
            });

            s.body("fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error>", |s| {
                s.wln("// size: u16_be, and opcode: u32");
                s.wln(format!(
                    "e.write_encrypted_client_header(w, ({size}size() + 4) as u16, <Self as {header}>::OPCODE)?;",
                    size = match e.is_constant_sized() {
                        true => "Self::",
                        false => "self.",
                    },
                    header = WORLD_CLIENT_HEADER_TRAIT_NAME,
                ));
                s.newline();

                s.wln("self.write_body(w)?;");
                s.wln("Ok(())");
            });
        },
    );
}

fn print_server_message_header(s: &mut Writer, e: &Container, v: u16) {
    s.body(
        format!("impl {} for {}", WORLD_SERVER_HEADER_TRAIT_NAME, e.name()),
        |s| {
            s.wln(format!("const OPCODE: u16 = {:#04x};", v));
            s.newline();

            s.bodyn("fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error>", |s| {
                s.wln("// size: u16_be, and opcode: u16");
                s.wln(format!(
                    "{import_path}::write_u16_be(w, ({size}size() + 2) as u16)?;",
                    import_path = "crate::util",
                    size = match e.is_constant_sized() {
                        true => "Self::",
                        false => "self.",
                    }
                ));
                s.wln(format!(
                    "{import_path}::write_u16_le(w, <Self as {header}>::OPCODE)?;",
                    import_path = "crate::util",
                    header = WORLD_SERVER_HEADER_TRAIT_NAME,
                ));
                s.newline();

                s.wln("self.write_body(w)?;");
                s.wln("Ok(())");
            });

            s.body("fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error>", |s| {
                s.wln("// size: u16_be, and opcode: u16");
                s.wln(format!(
                    "e.write_encrypted_server_header(w, ({size}size() + 2) as u16, <Self as {header}>::OPCODE)?;",
                    size = match e.is_constant_sized() {
                        true => "Self::",
                        false => "self.",
                    },
                    header = WORLD_SERVER_HEADER_TRAIT_NAME,
                ));
                s.newline();

                s.wln("self.write_body(w)?;");
                s.wln("Ok(())");
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
    d_name: &str,
    d_does_not_have_subvariables: bool,
    o_type_has_constant_size: bool,
    array_inner_has_constant: bool,
    prefix: &str,
) {
    match ty {
        Type::Integer(integer_type) => s.wln_no_indent(format!(
            "{size} // {name}: {type_name}",
            size = integer_type.size(),
            name = d_name,
            type_name = ty.str()
        )),
        Type::FloatingPoint(floating) => {
            s.wln_no_indent(format!("{size} // {name}: {type_name}", size = floating.size(),
            name = d_name, type_name = ty.str()))
        }
        Type::CString => s.wln_no_indent(format!(
            "{prefix}{name}.len() + 1 // {name}: CString and Null Terminator",
            prefix = prefix,
            name = d_name
        )),
        Type::String { .. } => s.wln_no_indent(format!(
            "{prefix}{name}.len() // {name}: {type_name}",
            name = d_name,
            prefix = prefix,
            type_name = ty.str()
        )),
        Type::Array(array) => {
            match array.ty() {
                ArrayType::Integer(integer_type) => match array.size() {
                    ArraySize::Fixed(fixed_value) => s.wln_no_indent(format!(
                        "{array_size} * core::mem::size_of::<{ty}>() // {name}: {type_name}",
                        array_size = fixed_value,
                        ty = integer_type.rust_str(),
                        name = d_name,
                        type_name = ty.str(),
                    )),
                    ArraySize::Variable(_) | ArraySize::Endless=> {
                        s.wln_no_indent(format!(
                            "{prefix}{name}.len() * core::mem::size_of::<{ty}>() // {name}: {type_name}",
                            name = d_name,
                            prefix = prefix,
                            ty = integer_type.rust_str(),
                            type_name = ty.str(),
                        ));
                    }
                },
                ArrayType::Complex(complex_type) => {
                    match array.size() {
                        ArraySize::Fixed(fixed_value) => {
                            match o_type_has_constant_size {
                            true => s.wln_no_indent(format!(
                                "{fixed_value} * {inner_type_name}::size() // {name}: {type_name}",
                                inner_type_name = complex_type,
                                name = d_name,
                                type_name = array.str(),
                                fixed_value = fixed_value,
                            )),
                            false => {
                                s.wln_no_indent(
                                    format!("{prefix}{name}.iter().fold(0, |acc, x| acc + x.size()) // {name}: {type_name}",
                                        name = d_name,
                                        prefix = prefix,
                                        type_name = ty.str()))
                            }
                        }
                        }
                        ArraySize::Variable(_) | ArraySize::Endless => {
                            s.wln_no_indent(
                            format!("{prefix}{name}.iter().fold(0, |acc, x| acc + {size_function}) // {name}: {type_name}",
                                name = d_name,
                                prefix = prefix,
                                type_name = ty.str(),
                                size_function = match array_inner_has_constant {
                                    true => format!("{type_name}::size()", type_name = complex_type),
                                    false => "x.size()".to_string(),
                                },
                            ));
                        }
                    }
                }
                ArrayType::CString => {
                    s.wln_no_indent(
                        format!("{prefix}{name}.iter().fold(0, |acc, x| acc + x.len() + 1) // {name}: {type_name}",
                                name = d_name,
                                prefix = prefix,
                                type_name = ty.str()));
                }
                ArrayType::Guid => {
                    s.wln_no_indent(format!(
                        "8 // {name}: {type_name}",
                        name = d_name,
                        type_name = ty.str()
                    ))
                }
                ArrayType::PackedGuid => {
                    s.wln_no_indent(
                        format!("{prefix}{name}.iter().fold(0, |acc, x| acc + x.size()) // {name}: {type_name}",
                                name = d_name,
                                prefix = prefix,
                                type_name = ty.str()));
                }
            }
        }
        Type::Identifier{s: identifier, upcast} => {
            match upcast {
                None => {}
                Some(integer) => {
                    s.wln_no_indent(format!("{size} // {name}: {type_name} upcasted to {ty}",
                                            size = integer.size(),
                                            name = d_name,
                                            type_name = identifier,
                                            ty = integer.rust_str()));
                    return;
                }
            }
            if d_does_not_have_subvariables {
                if o_type_has_constant_size {
                    s.wln_no_indent(format!(
                        "{type_name}::size() // {name}: {type_name}",
                        name = d_name,
                        type_name = identifier,
                    ));
                } else {
                    s.wln_no_indent(format!(
                        "{prefix}{name}.size() // {name}: {type_name}",
                        name = d_name,
                        prefix = prefix,
                        type_name = identifier,
                    ));
                }
            } else {
                s.wln_no_indent(format!(
                    "{prefix}{name}.size() // {name}: {type_name} and subfields",
                    name = d_name,
                    prefix = prefix,
                    type_name = identifier,
                ));
            }
        }
        Type::PackedGuid => {
            s.wln_no_indent(format!("{prefix}{name}.size() // {name}: {type_name}",
                                    name = d_name,
                                    prefix = prefix,
                                    type_name = ty.str()));
        }
        Type::Guid => {
            s.wln_no_indent(format!("8 // {name}: {type_name}",
                                    name = d_name,
                                    type_name = ty.str()));
        }
        Type::UpdateMask | Type::AuraMask => {

            s.wln_no_indent(format!("{prefix}{name}.size() // {name}: {type_name}",
                                    name = d_name,
                                    prefix = prefix,
                                    type_name = ty.str()));
        }
    }
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
            }
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
