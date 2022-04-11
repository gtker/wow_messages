use crate::container::{Container, ContainerType, StructMember, StructMemberDefinition};
use crate::parser::types::objects::Objects;
use crate::parser::types::ty::Type;
use crate::parser::types::{
    Array, ArrayType, FloatingPointType, IntegerType, VerifiedContainerValue,
};
use crate::rust_printer::complex_print::DefinerType;
use crate::rust_printer::new_enums::{
    IfStatementType, NewEnumStructMember, NewEnumerator, NewIfStatement,
};
use crate::rust_printer::Writer;

pub fn print_unencrypted_write_header(s: &mut Writer, e: &Container) {
    match e.container_type() {
        ContainerType::Struct => {}
        ContainerType::SLogin(_) | ContainerType::CLogin(_) => {
            s.wln("// opcode: u8");
            s.wln("w.write_all(&Self::OPCODE.to_le_bytes())?;");
            s.newline();
        }
        ContainerType::Msg(_) => panic!("msg opcode not handled"),
        ContainerType::CMsg(_) => {
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
                "{import_path}::write_u32_le(w, Self::OPCODE)?;",
                import_path = "crate::util",
            ));
            s.newline();
        }
        ContainerType::SMsg(_) => {
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
                "{import_path}::write_u16_le(w, Self::OPCODE)?;",
                import_path = "crate::util",
            ));
            s.newline();
        }
    }
}

pub fn print_write_field_cstring(s: &mut Writer, variable_name: &str, variable_prefix: &str) {
    s.wln(format!(
        "w.write_all({prefix}{name}.as_bytes())?;",
        name = variable_name,
        prefix = variable_prefix,
    ));
    s.wln("// Null terminator");
    s.wln("w.write_all(&[0])?;");
    s.newline();
}

pub fn print_write_field_string(s: &mut Writer, variable_name: &str, variable_prefix: &str) {
    s.wln(format!(
        "w.write_all({prefix}{name}.as_bytes())?;",
        name = variable_name,
        prefix = variable_prefix,
    ));

    s.newline();
}

pub fn print_write_field_array(
    s: &mut Writer,
    variable_name: &str,
    variable_prefix: &str,
    array: &Array,
) {
    s.open_curly(format!(
        "for i in {prefix}{name}.iter()",
        name = variable_name,
        prefix = variable_prefix
    ));

    match array.ty() {
        ArrayType::Integer(integer_type) => s.wln(format!(
            "w.write_all(&i.to_{endian}_bytes())?;",
            endian = integer_type.rust_endian_str()
        )),
        ArrayType::Complex(_) => s.wln("i.write(w)?;"),
        ArrayType::CString => {
            s.wln("w.write_all(&i.as_bytes())?;");
            s.wln("w.write_all(&[0])?;")
        }
        ArrayType::Guid => {
            s.wln("i.write(w)?;");
        }
        ArrayType::PackedGuid => s.wln("i.write_packed(w)?;"),
    }

    s.closing_curly_newline();
}

pub fn print_write_field_floating(
    s: &mut Writer,
    variable_name: &str,
    variable_prefix: &str,
    floating: &FloatingPointType,
) {
    s.wln(format!(
        "w.write_all(&{variable_prefix}{variable_name}.to_{endian}_bytes())?;",
        variable_prefix = variable_prefix,
        variable_name = variable_name,
        endian = floating.rust_endian_str()
    ));

    s.newline();
}

pub fn print_write_field_integer(
    s: &mut Writer,
    variable_name: &str,
    variable_prefix: &str,
    int_type: &IntegerType,
    used_as_size_in: &Option<String>,
    verified_value: &Option<VerifiedContainerValue>,
    size_of_fields_before_size: u64,
) {
    if let Some(value) = verified_value {
        if value.original_string() == "self.size" {
            s.wln(format!("w.write_all(&((self.size() - {minus_value}) as {basic_type}).to_{endian}_bytes())?;",
                          minus_value = size_of_fields_before_size,
                          endian = int_type.rust_endian_str(),
                          basic_type = int_type.rust_str(),
            ));
        } else {
            s.wln(format!(
                "w.write_all(&Self::{name}_VALUE.to_{endian}_bytes())?;",
                name = variable_name.to_uppercase(),
                endian = int_type.rust_endian_str(),
            ));
        }
    } else if let Some(v) = used_as_size_in {
        s.wln(format!(
            "w.write_all(&({variable_prefix}{array}.len() as {basic_type}).to_{endian}_bytes())?;",
            array = v,
            basic_type = int_type.rust_str(),
            endian = int_type.rust_endian_str(),
            variable_prefix = variable_prefix
        ));
    } else {
        s.wln(format!(
            "w.write_all(&{prefix}{name}.to_{endian}_bytes())?;",
            name = variable_name,
            endian = int_type.rust_endian_str(),
            prefix = variable_prefix,
        ));
    }

    s.newline();
}

pub fn print_write_field_identifier(
    s: &mut Writer,
    variable_name: &str,
    variable_prefix: &str,
    verified_value: &Option<VerifiedContainerValue>,
    identifier: &str,
) {
    if verified_value.is_some() {
        s.wln(format!(
            "{type_name}::{variant}.write(w)?;",
            type_name = identifier,
            variant = verified_value.as_ref().unwrap().original_string()
        ));
        s.wln(format!(
            "// {name} is set to always be {variant} ({value})",
            name = variable_name,
            variant = verified_value.as_ref().unwrap().original_string(),
            value = verified_value.as_ref().unwrap().value()
        ));
    } else {
        s.wln(format!(
            "{prefix}{name}.write(w)?;",
            name = variable_name,
            prefix = variable_prefix
        ));
    }

    s.newline();
}

pub fn print_write(s: &mut Writer, e: &Container, o: &Objects) {
    for field in e.fields() {
        print_write_field(s, e, o, field, "self.");
    }

    s.wln("Ok(())");
}

pub fn print_write_definition(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    variable_prefix: &str,
    d: &StructMemberDefinition,
) {
    s.wln(format!(
        "// {name}: {type_name}",
        name = d.name(),
        type_name = d.ty().str()
    ));
    match d.ty() {
        Type::Integer(int_type) => {
            let size = if let Some(v) = d.verified_value() {
                if v.original_string() == "self.size" {
                    e.size_of_fields_before_size(o)
                } else {
                    0
                }
            } else {
                0
            };
            print_write_field_integer(
                s,
                d.name(),
                variable_prefix,
                int_type,
                d.used_as_size_in(),
                d.verified_value(),
                size,
            );
        }
        Type::FloatingPoint(floating) => {
            print_write_field_floating(s, d.name(), variable_prefix, floating);
        }
        Type::CString => {
            print_write_field_cstring(s, d.name(), variable_prefix);
        }
        Type::String { .. } => {
            print_write_field_string(s, d.name(), variable_prefix);
        }
        Type::Array(array) => {
            print_write_field_array(s, d.name(), variable_prefix, array);
        }
        Type::Identifier {
            s: identifier,
            upcast,
        } => {
            match upcast {
                None => {}
                Some(integer) => {
                    s.wln(format!(
                        "{variable_prefix}{name}.write_{ty}_{endian}(w)?;",
                        variable_prefix = variable_prefix,
                        name = d.name(),
                        ty = integer.rust_str(),
                        endian = integer.rust_endian_str()
                    ));
                    s.newline();
                    return;
                }
            }
            print_write_field_identifier(
                s,
                d.name(),
                variable_prefix,
                d.verified_value(),
                identifier,
            );
        }
        Type::PackedGuid => {
            s.wln(format!(
                "{variable_prefix}{name}.write_packed(w)?;",
                variable_prefix = variable_prefix,
                name = d.name()
            ));
            s.newline();
        }
        Type::Guid | Type::UpdateMask | Type::AuraMask => {
            s.wln(format!(
                "{variable_prefix}{name}.write(w)?;",
                variable_prefix = variable_prefix,
                name = d.name()
            ));
            s.newline();
        }
    }
}

fn print_write_field(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    field: &StructMember,
    variable_prefix: &str,
) {
    match field {
        StructMember::Definition(d) => {
            print_write_definition(s, e, o, variable_prefix, d);
        }
        StructMember::IfStatement(statement) => match statement.definer_type() {
            DefinerType::Enum => {
                print_enum_if_statement_new(s, e, o, variable_prefix, statement.new_enum());
            }
            DefinerType::Flag => print_flag_if_statement(s, variable_prefix, statement.new_enum()),
        },
        StructMember::OptionalStatement(optional) => {
            s.wln(format!("// optional {name}", name = optional.name()));

            s.body(
                format!("if let Some(v) = &self.{name}", name = optional.name()),
                |s| {
                    for m in optional.members() {
                        print_write_field(s, e, o, m, "v.");
                    }
                },
            );

            s.newline();
        }
    }
}

pub fn print_flag_if_statement(s: &mut Writer, variable_prefix: &str, statement: &NewIfStatement) {
    assert!(statement.enum_or_flag() == IfStatementType::Flag);

    let enumerator = statement
        .enumerators()
        .iter()
        .filter(|a| !a.fields().is_empty())
        .collect::<Vec<&NewEnumerator>>()[0];

    s.bodyn(
        format!(
            "if let Some(s) = &{variable_prefix}{variable}.{variant}",
            variable_prefix = variable_prefix,
            variable = statement.variable_name(),
            variant = &enumerator.name().to_lowercase(),
        ),
        |s| {
            s.wln("s.write(w)?;");
        },
    );
}

pub fn print_enum_if_statement_new(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    variable_prefix: &str,
    ne: &NewIfStatement,
) {
    s.open_curly(format!(
        "match &{prefix}{name}",
        name = ne.variable_name(),
        prefix = match e.type_definition_in_same_scope(ne.variable_name()) {
            false => "self.",
            true => variable_prefix,
        },
    ));

    for en in ne.enumerators() {
        if en.fields().is_empty() {
            s.wln(format!(
                "{new_enum}::{variant} => {{}}",
                new_enum = ne.new_ty_name(),
                variant = en.name()
            ));

            continue;
        }

        s.open_curly(format!(
            "{new_enum}::{variant}",
            new_enum = ne.new_ty_name(),
            variant = en.name()
        ));

        for m in en.fields() {
            match m {
                NewEnumStructMember::Definition(d) => {
                    if d.used_as_size_in().is_some() || d.verified_value().is_some() {
                        continue;
                    }
                    s.wln(format!("{name},", name = d.name()));
                }
                NewEnumStructMember::IfStatement(_) => {}
            }
        }
        s.closing_curly_with(" => {");
        s.inc_indent();

        for m in en.fields() {
            match m {
                NewEnumStructMember::Definition(d) => {
                    print_write_definition(s, e, o, "", d);
                }
                NewEnumStructMember::IfStatement(statement) => match statement.enum_or_flag() {
                    IfStatementType::Enum => {
                        print_enum_if_statement_new(s, e, o, "", statement);
                    }
                    IfStatementType::Flag => {
                        print_flag_if_statement(s, "", statement);
                    }
                },
            }
        }

        s.closing_curly();
    }

    s.closing_curly_newline();
}
