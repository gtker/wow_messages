use crate::container::{
    Container, ContainerType, Equation, IfStatement, StructMember, StructMemberDefinition,
};
use crate::parser::types::objects::Objects;
use crate::parser::types::ty::Type;
use crate::parser::types::{
    Array, ArrayType, FloatingPointType, IntegerType, ObjectType, VerifiedContainerValue,
};
use crate::rust_printer::DefinerType;
use crate::rust_printer::Writer;
use crate::CONTAINER_SELF_SIZE_FIELD;

pub fn print_unencrypted_write_header(s: &mut Writer, e: &Container, postfix: &str) {
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
        _ => unreachable!(),
    }
}

pub fn print_write_field_cstring(
    s: &mut Writer,
    variable_name: &str,
    variable_prefix: &str,
    postfix: &str,
) {
    s.wln(format!(
        "w.write_all({prefix}{name}.as_bytes()){postfix}?;",
        name = variable_name,
        prefix = variable_prefix,
        postfix = postfix,
    ));
    s.wln("// Null terminator");
    s.wln(format!("w.write_all(&[0]){postfix}?;", postfix = postfix));
    s.newline();
}

pub fn print_write_field_string(
    s: &mut Writer,
    variable_name: &str,
    variable_prefix: &str,
    postfix: &str,
) {
    s.wln(format!(
        "w.write_all({prefix}{name}.as_bytes()){postfix}?;",
        name = variable_name,
        prefix = variable_prefix,
        postfix = postfix,
    ));

    s.newline();
}

pub fn print_write_field_array(
    s: &mut Writer,
    variable_name: &str,
    variable_prefix: &str,
    array: &Array,
    prefix: &str,
    postfix: &str,
) {
    s.open_curly(format!(
        "for i in {prefix}{name}.iter()",
        name = variable_name,
        prefix = variable_prefix
    ));

    match array.ty() {
        ArrayType::Integer(integer_type) => s.wln(format!(
            "w.write_all(&i.to_{endian}_bytes()){postfix}?;",
            endian = integer_type.rust_endian_str(),
            postfix = postfix,
        )),
        ArrayType::Complex(_) => s.wln(format!("i.{}write(w){}?;", prefix, postfix)),
        ArrayType::CString => {
            s.wln(format!("w.write_all(&i.as_bytes()){}?;", postfix));
            s.wln(format!("w.write_all(&[0]){}?;", postfix));
        }
        ArrayType::Guid => {
            s.wln(format!("i.{}write(w){}?;", prefix, postfix));
        }
        ArrayType::PackedGuid => s.wln(format!("w.write_all(&i.packed_guid()){}?;", postfix)),
    }

    s.closing_curly_newline();
}

pub fn print_write_field_floating(
    s: &mut Writer,
    variable_name: &str,
    variable_prefix: &str,
    floating: &FloatingPointType,
    postfix: &str,
) {
    s.wln(format!(
        "w.write_all(&{variable_prefix}{variable_name}.to_{endian}_bytes()){postfix}?;",
        variable_prefix = variable_prefix,
        variable_name = variable_name,
        endian = floating.rust_endian_str(),
        postfix = postfix,
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

    s.newline();
}

pub fn print_write_field_identifier(
    s: &mut Writer,
    variable_name: &str,
    variable_prefix: &str,
    verified_value: &Option<VerifiedContainerValue>,
    identifier: &str,
    prefix: &str,
    postfix: &str,
) {
    if verified_value.is_some() {
        s.wln(format!(
            "{type_name}::{variant}.{prefix}write(w){postfix}?;",
            type_name = identifier,
            prefix = prefix,
            postfix = postfix,
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
            "{variable_prefix}{name}.{prefix}write(w){postfix}?;",
            name = variable_name,
            variable_prefix = variable_prefix,
            prefix = prefix,
            postfix = postfix,
        ));
    }

    s.newline();
}

pub fn print_write(s: &mut Writer, e: &Container, o: &Objects, prefix: &str, postfix: &str) {
    for field in e.fields() {
        print_write_field(s, e, o, field, "self.", prefix, postfix);
    }

    s.wln("Ok(())");
}

pub fn print_write_definition(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    variable_prefix: &str,
    d: &StructMemberDefinition,
    prefix: &str,
    postfix: &str,
) {
    s.wln(format!(
        "// {name}: {type_name}",
        name = d.name(),
        type_name = d.ty().str()
    ));
    match d.ty() {
        Type::Integer(int_type) => {
            let size = if let Some(v) = d.verified_value() {
                if v.original_string() == CONTAINER_SELF_SIZE_FIELD {
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
                postfix,
            );
        }
        Type::FloatingPoint(floating) => {
            print_write_field_floating(s, d.name(), variable_prefix, floating, postfix);
        }
        Type::CString => {
            print_write_field_cstring(s, d.name(), variable_prefix, postfix);
        }
        Type::String { .. } => {
            print_write_field_string(s, d.name(), variable_prefix, postfix);
        }
        Type::Array(array) => {
            print_write_field_array(s, d.name(), variable_prefix, array, prefix, postfix);
        }
        Type::Identifier {
            s: identifier,
            upcast,
        } => {
            match o.get_object_type_of(identifier, e.tags()) {
                ObjectType::Enum | ObjectType::Flag => {
                    let integer = match upcast {
                        None => {
                            let definer = o.get_definer(identifier, e.tags());
                            definer.ty()
                        }
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
                _ => {}
            }

            print_write_field_identifier(
                s,
                d.name(),
                variable_prefix,
                d.verified_value(),
                identifier,
                prefix,
                postfix,
            );
        }
        Type::PackedGuid => {
            s.wln(format!(
                "w.write_all(&{variable_prefix}{name}.packed_guid()){postfix}?;",
                variable_prefix = variable_prefix,
                postfix = postfix,
                name = d.name(),
            ));
            s.newline();
        }
        Type::Guid | Type::UpdateMask | Type::AuraMask => {
            s.wln(format!(
                "{variable_prefix}{name}.{prefix}write(w){postfix}?;",
                variable_prefix = variable_prefix,
                prefix = prefix,
                postfix = postfix,
                name = d.name()
            ));
            s.newline();
        }
    }
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
                enumerator = enumerator.name(),
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

pub fn print_write_field(
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
            print_write_definition(s, e, o, variable_prefix, d, prefix, postfix);
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

    let enumerator_name = match &statement.get_conditional().equations()[0] {
        Equation::Equals { value, .. } | Equation::NotEquals { value, .. } => value,
        _ => unreachable!(),
    };

    let rd = e
        .rust_object()
        .rust_definer_with_variable_name_and_enumerator(statement.name(), enumerator_name);

    for enumerator in rd.enumerators() {
        if !enumerator.has_members() {
            s.wln(format!(
                "{new_enum}::{variant} => {{}}",
                new_enum = rd.ty_name(),
                variant = enumerator.name()
            ));

            continue;
        }

        s.open_curly(format!(
            "{new_enum}::{variant}",
            new_enum = rd.ty_name(),
            variant = enumerator.name(),
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
