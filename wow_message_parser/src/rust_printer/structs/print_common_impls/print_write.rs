use crate::parser::types::array::{Array, ArrayType};
use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::if_statement::{Equation, IfStatement};
use crate::parser::types::objects::Objects;
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::ty::Type;
use crate::parser::types::{ContainerValue, IntegerType};
use crate::rust_printer::DefinerType;
use crate::rust_printer::Writer;
use crate::CONTAINER_SELF_SIZE_FIELD;

pub(crate) fn print_unencrypted_write_header(s: &mut Writer, e: &Container, postfix: &str) {
    match e.container_type() {
        ContainerType::Struct => {}
        ContainerType::SLogin(_) | ContainerType::CLogin(_) => {
            s.wln("// opcode: u8");
            s.wln(format!(
                "w.write_all(&Self::OPCODE.to_le_bytes()){postfix}?;",
                postfix = postfix
            ));
            s.newline();
        }
        _ => unreachable!("Non-login container found in login function"),
    }
}

pub(crate) fn print_write_field_array(
    s: &mut Writer,
    e: &Container,
    d: &StructMemberDefinition,
    variable_prefix: &str,
    array: &Array,
    postfix: &str,
) {
    if d.tags().is_compressed() {
        s.wln(
            "let mut encoder = flate2::write::ZlibEncoder::new(w, flate2::Compression::default());",
        );
    }

    s.open_curly(format!(
        "for i in {prefix}{name}.iter()",
        name = d.name(),
        prefix = variable_prefix
    ));

    let writer = if d.tags().is_compressed() {
        "encoder"
    } else {
        "w"
    };

    match array.ty() {
        ArrayType::Integer(integer_type) => s.wln(format!(
            "{writer}.write_all(&i.to_{endian}_bytes()){postfix}?;",
            writer = writer,
            endian = integer_type.rust_endian_str(),
            postfix = postfix,
        )),
        ArrayType::Struct(_) => {
            // Complex types use "write_into_vec", which means we can't write directly
            // into our ZLibEncoder. Instead, we write to an intermediary Vec first.
            if e.tags().compressed() || d.tags().is_compressed() {
                s.wln("let mut vec = Vec::new();");
                s.wln("i.write_into_vec(&mut vec)?;");
                s.wln(format!(
                    "{writer}.write_all(vec.as_slice());",
                    writer = writer
                ));
            } else {
                s.wln("i.write_into_vec(w)?;");
            }
        }
        ArrayType::CString => {
            s.wln(format!("w.write_all(i.as_bytes()){}?;", postfix));
            s.wln(format!("w.write_all(&[0]){}?;", postfix));
        }
        ArrayType::Guid => {
            s.wln(format!("w.write_all(&i.guid().to_le_bytes()){}?;", postfix));
        }
        ArrayType::PackedGuid => s.wln("i.write_packed_guid_into_vec(w);"),
    }

    s.closing_curly();
}

pub(crate) fn print_write_field_integer(
    s: &mut Writer,
    variable_name: &str,
    variable_prefix: &str,
    int_type: &IntegerType,
    used_as_size_in: &Option<String>,
    verified_value: &Option<ContainerValue>,
    size_of_fields_before_size: u64,
    postfix: &str,
) {
    if let Some(value) = verified_value {
        if value.original_string() == CONTAINER_SELF_SIZE_FIELD {
            s.wln(format!("w.write_all(&((self.size() - {minus_value}) as {basic_type}).to_{endian}_bytes()){postfix}?;",
                          minus_value = size_of_fields_before_size,
                          endian = int_type.rust_endian_str(),
                          basic_type = int_type.rust_str(),
                          postfix = postfix,
            ));
        } else {
            s.wln(format!(
                "w.write_all(&Self::{name}_VALUE.to_{endian}_bytes()){postfix}?;",
                name = variable_name.to_uppercase(),
                endian = int_type.rust_endian_str(),
                postfix = postfix,
            ));
        }
    } else if let Some(v) = used_as_size_in {
        s.wln(format!(
            "w.write_all(&({variable_prefix}{array}.len() as {basic_type}).to_{endian}_bytes()){postfix}?;",
            array = v,
            basic_type = int_type.rust_str(),
            endian = int_type.rust_endian_str(),
            variable_prefix = variable_prefix,
            postfix = postfix,
        ));
    } else {
        s.wln(format!(
            "w.write_all(&{prefix}{name}.to_{endian}_bytes()){postfix}?;",
            name = variable_name,
            endian = int_type.rust_endian_str(),
            prefix = variable_prefix,
            postfix = postfix,
        ));
    }
}

pub(crate) fn print_write_field_identifier(
    s: &mut Writer,
    variable_name: &str,
    variable_prefix: &str,
) {
    s.wln(format!(
        "{variable_prefix}{name}.write_into_vec(w)?;",
        name = variable_name,
        variable_prefix = variable_prefix,
    ));
}

pub(crate) fn print_write(s: &mut Writer, e: &Container, o: &Objects, prefix: &str, postfix: &str) {
    //Whether this write_into_vec method should do size checking. Size checks not supported for
    //messages that are compressed or contain any compression.
    let should_print_size_assert = e.container_type().is_top_level()
        && !e.is_constant_sized()
        && !e.tags().compressed()
        && !e
            .all_definitions()
            .iter()
            .any(|d| d.tags().compressed().is_some());

    //Cache the header size
    if should_print_size_assert {
        s.wln("let size_assert_header_size = w.len();");
    }

    // For fully compressed messages, replace the writer with a ZLibDecoder.
    if e.tags().compressed() {
        // Fully compressed messages include the decompressed size as a u32 at the start of the packet.
        s.wln("w.write_all(&(self.size_uncompressed() as u32).to_le_bytes())?;");
        s.newline();

        s.wln("let mut w = &mut flate2::write::ZlibEncoder::new(w, flate2::Compression::fast());");
        s.newline();
    }

    for field in e.members() {
        print_write_field(s, e, o, field, "self.", prefix, postfix);
    }

    //Print the actual size assert, using the cached header size to discard the header
    if should_print_size_assert {
        s.wln("assert_eq!(self.size() as usize + size_assert_header_size, w.len(), \"Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent\");");
    }
}

pub(crate) fn print_write_definition(
    s: &mut Writer,
    e: &Container,
    variable_prefix: &str,
    d: &StructMemberDefinition,
    postfix: &str,
) {
    if !(d.used_as_size_in().is_some() && d.tags().skip_serialize()) {
        s.wln(format!(
            "// {name}: {type_name}",
            name = d.name(),
            type_name = d.ty().str()
        ));
    } else {
        s.wln(format!(
            "// {name} is included in the struct but explicitly excluded due to its `skip_serialize` tag.",
            name = d.name(),
        ));
    }

    match d.ty() {
        Type::Integer(int_type) => {
            let size = if let Some(v) = d.value() {
                if v.original_string() == CONTAINER_SELF_SIZE_FIELD {
                    e.size_of_fields_before_size()
                } else {
                    0
                }
            } else {
                0
            };

            if !(d.used_as_size_in().is_some() && d.tags().skip_serialize()) {
                print_write_field_integer(
                    s,
                    d.name(),
                    variable_prefix,
                    int_type,
                    d.used_as_size_in(),
                    d.value(),
                    size,
                    postfix,
                );
            }
        }
        Type::Bool(i) => {
            s.wln(format!(
                "w.write_all({ty}::from({reference}{variable_prefix}{variable_name}).to_le_bytes().as_slice()){postfix}?;",
                reference = if variable_prefix.is_empty() { "*" } else { "" }, // non-self variables are &bool
                variable_prefix = variable_prefix,
                variable_name = d.name(),
                postfix = postfix,
                ty = i.rust_str(),
            ));
        }
        Type::FloatingPoint(floating) => {
            s.wln(format!(
                "w.write_all(&{variable_prefix}{variable_name}.to_{endian}_bytes()){postfix}?;",
                variable_prefix = variable_prefix,
                variable_name = d.name(),
                endian = floating.rust_endian_str(),
                postfix = postfix,
            ));
        }
        Type::SizedCString => {
            s.wln(format!(
                "w.write_all(&(({prefix}{name}.len() + 1) as u32).to_le_bytes()){postfix}?;",
                prefix = variable_prefix,
                name = d.name(),
                postfix = postfix,
            ));
            s.wln(format!(
                "w.write_all({prefix}{name}.as_bytes()){postfix}?;",
                prefix = variable_prefix,
                name = d.name(),
                postfix = postfix,
            ));
            s.wln("// Null terminator");
            s.wln(format!("w.write_all(&[0]){postfix}?;", postfix = postfix,));
        }
        //TODO: types that prevent null bytes in strings
        Type::CString => {
            s.wln("// TODO: Guard against strings that are already null-terminated");
            s.wln(format!(
                "assert_ne!({prefix}{name}.as_bytes().iter().rev().next(), Some(&0_u8), \"String `{name}` must not be null-terminated.\");",
                name = d.name(),
                prefix = variable_prefix,
            ));
            s.wln(format!(
                "w.write_all({prefix}{name}.as_bytes()){postfix}?;",
                name = d.name(),
                prefix = variable_prefix,
                postfix = postfix,
            ));
            s.wln("// Null terminator");
            s.wln(format!("w.write_all(&[0]){postfix}?;", postfix = postfix));
        }
        Type::String { .. } => {
            s.wln(format!(
                "w.write_all(&({variable_prefix}{name}.len() as u8).to_le_bytes()){postfix}?;",
                name = d.name(),
            ));
            s.wln(format!(
                "w.write_all({prefix}{name}.as_bytes()){postfix}?;",
                name = d.name(),
                prefix = variable_prefix,
                postfix = postfix,
            ));
        }
        Type::Array(array) => {
            print_write_field_array(s, e, d, variable_prefix, array, postfix);
        }
        Type::Enum { e, upcast } | Type::Flag { e, upcast } => {
            let integer = match upcast {
                None => e.ty(),
                Some(integer) => integer,
            };

            s.wln(format!(
                "w.write_all(&({variable_prefix}{name}.as_int() as {ty}).to_{endian}_bytes()){postfix}?;",
                variable_prefix = variable_prefix,
                postfix = postfix,
                name = d.name(),
                ty = integer.rust_str(),
                endian = integer.rust_endian_str()
            ));

            s.newline();
            return;
        }
        Type::Struct { .. } => {
            print_write_field_identifier(s, d.name(), variable_prefix);
        }
        Type::PackedGuid => {
            s.wln(format!(
                "{variable_prefix}{name}.write_packed_guid_into_vec(w);",
                variable_prefix = variable_prefix,
                name = d.name(),
            ));
        }
        Type::DateTime => {
            s.wln(format!(
                "w.write_all(&{variable_prefix}{name}.as_int().to_le_bytes()){postfix}?;",
                variable_prefix = variable_prefix,
                postfix = postfix,
                name = d.name()
            ));
        }
        Type::Guid => {
            s.wln(format!(
                "w.write_all(&{variable_prefix}{name}.guid().to_le_bytes()){postfix}?;",
                variable_prefix = variable_prefix,
                postfix = postfix,
                name = d.name()
            ));
        }
        Type::AchievementDoneArray
        | Type::AchievementInProgressArray
        | Type::UpdateMask
        | Type::AuraMask => {
            s.wln(format!(
                "{variable_prefix}{name}.write_into_vec(w){postfix}?;",
                variable_prefix = variable_prefix,
                postfix = postfix,
                name = d.name()
            ));
        }
    }

    s.newline();
}

fn print_write_flag_if_statement(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    variable_prefix: &str,
    statement: &IfStatement,
    prefix: &str,
    postfix: &str,
) {
    s.open_curly(format!(
        "if let Some(if_statement) = &{variable_prefix}{variable}.{variant}",
        variable_prefix = variable_prefix,
        variable = statement.name(),
        variant = &statement.flag_get_enumerator().to_lowercase(),
    ));
    if statement.else_ifs().is_empty() {
        for m in statement.members() {
            print_write_field(s, e, o, m, "if_statement.", prefix, postfix);
        }
    } else {
        s.open_curly("match if_statement");
        let rd = e
            .rust_object()
            .rust_definer_with_variable_name_and_enumerator(
                statement.name(),
                &statement.flag_get_enumerator(),
            );

        for enumerator in rd.enumerators() {
            s.open_curly(format!(
                "{ty}::{enumerator}",
                ty = rd.ty_name(),
                enumerator = enumerator.rust_name(),
            ));

            for m in enumerator.members_in_struct() {
                s.wln(format!("{name},", name = m.name()));
            }
            s.closing_curly_with(" => {"); // Self::enumerator
            s.inc_indent();

            for m in enumerator.original_fields() {
                print_write_field(s, e, o, m, "", prefix, postfix);
            }

            s.closing_curly(); // Enumerator body
        }

        s.closing_curly(); // match self
    }

    s.closing_curly_newline(); // if let Some(s)
}

pub(crate) fn print_write_field(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    field: &StructMember,
    variable_prefix: &str,
    prefix: &str,
    postfix: &str,
) {
    match field {
        StructMember::Definition(d) => {
            print_write_definition(s, e, variable_prefix, d, postfix);
        }
        StructMember::IfStatement(statement) => match statement.definer_type() {
            DefinerType::Enum => {
                print_write_if_enum_statement(s, e, o, variable_prefix, statement, prefix, postfix);
            }
            DefinerType::Flag => {
                print_write_flag_if_statement(s, e, o, variable_prefix, statement, prefix, postfix);
            }
        },
        StructMember::OptionalStatement(optional) => {
            s.wln(format!("// optional {name}", name = optional.name()));

            s.body(
                format!("if let Some(v) = &self.{name}", name = optional.name()),
                |s| {
                    for m in optional.members() {
                        print_write_field(s, e, o, m, "v.", prefix, postfix);
                    }
                },
            );

            s.newline();
        }
    }
}

fn print_write_if_enum_statement(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    variable_prefix: &str,
    statement: &IfStatement,
    prefix: &str,
    postfix: &str,
) {
    s.open_curly(format!(
        "match &{prefix}{name}",
        name = statement.name(),
        prefix = match e.type_definition_in_same_scope(statement.name()) {
            false => "self.",
            true => variable_prefix,
        },
    ));

    let enumerator_name = match &statement.conditional().equations()[0] {
        Equation::Equals { value, .. } | Equation::NotEquals { value, .. } => value,
        _ => unreachable!("enum branch has bitwise and"),
    };

    let rd = e
        .rust_object()
        .rust_definer_with_variable_name_and_enumerator(statement.name(), enumerator_name);

    for enumerator in rd.enumerators() {
        if !enumerator.has_members() {
            s.wln(format!(
                "{new_enum}::{variant} => {{}}",
                new_enum = rd.ty_name(),
                variant = enumerator.rust_name()
            ));

            continue;
        }

        s.open_curly(format!(
            "{new_enum}::{variant}",
            new_enum = rd.ty_name(),
            variant = enumerator.rust_name(),
        ));
        for m in enumerator.members_in_struct() {
            s.wln(format!("{},", m.name()));
        }
        s.closing_curly_with(" => {");
        s.inc_indent();

        for m in enumerator.original_fields() {
            print_write_field(s, e, o, m, "", prefix, postfix);
        }

        s.closing_curly(); // enum::enumerator
    }

    s.closing_curly_newline(); // match
}
