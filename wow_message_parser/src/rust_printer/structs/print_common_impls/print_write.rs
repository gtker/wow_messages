use crate::parser::types::array::{Array, ArraySize, ArrayType};
use crate::parser::types::container::Container;
use crate::parser::types::if_statement::{Equation, IfStatement};
use crate::parser::types::objects::Objects;
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::ty::Type;
use crate::parser::types::{ContainerValue, IntegerType};
use crate::rust_printer::base_structs::base_struct_write_name;
use crate::rust_printer::writer::Writer;
use crate::rust_printer::DefinerType;

pub(crate) fn print_write_field_array(
    s: &mut Writer,
    d: &StructMemberDefinition,
    variable_prefix: &str,
    array: &Array,
    postfix: &str,
) {
    let name = d.name();

    let writer = if array.compressed() {
        let size = match array.size() {
            ArraySize::Fixed(e) => e.to_string(),
            ArraySize::Endless | ArraySize::Variable(_) => {
                format!("{variable_prefix}{}.len()", d.name())
            }
        };
        let calc = match array.ty() {
            ArrayType::Integer(i) => {
                let i = i.size();
                format!("{i} * {size}")
            }
            ArrayType::Spell => {
                format!("4 * {size}")
            }
            ArrayType::Guid => {
                format!("8 * {size}")
            }
            ArrayType::Struct(e) => {
                if let Some(constant) = e.sizes().is_constant() {
                    format!("{constant} * {size}")
                } else {
                    format!("{variable_prefix}{name}.iter().fold(0, |acc, x| acc + x.size())")
                }
            }
            ArrayType::PackedGuid | ArrayType::CString => {
                format!("{variable_prefix}{name}.iter().fold(0, |acc, x| acc + x.size())")
            }
        };
        s.wln(format!("let decompressed_size: u32 = {calc} as u32;"));

        s.wln(format!(
            "w.write_all(&decompressed_size.to_le_bytes()){postfix}?;"
        ));

        s.wln(
            "let mut encoder = flate2::write::ZlibEncoder::new(w, flate2::Compression::default());",
        );

        "encoder"
    } else {
        "w"
    };

    s.body(
        format!("for i in {variable_prefix}{name}.iter()"),
        |s| match array.ty() {
            ArrayType::Spell | ArrayType::Integer(_) => {
                s.wln(format!("{writer}.write_all(&i.to_le_bytes()){postfix}?;",))
            }
            ArrayType::Struct(e) => {
                if e.tags().is_in_base() {
                    let f = base_struct_write_name(e);
                    s.wln(format!("crate::util::{f}(i, &mut {writer})?;"));
                } else {
                    s.wln(format!("i.write_into_vec(&mut {writer})?;"));
                }
            }
            ArrayType::CString => {
                s.wln(format!("w.write_all(i.as_bytes()){postfix}?;"));
                s.wln(format!("w.write_all(&[0]){postfix}?;"));
            }
            ArrayType::Guid => {
                s.wln(format!("w.write_all(&i.guid().to_le_bytes()){postfix}?;"));
            }
            ArrayType::PackedGuid => s.wln("crate::util::write_packed_guid(i, &mut w)?;"),
        },
    );
}

pub(crate) fn print_write_field_integer(
    s: &mut Writer,
    variable_name: &str,
    variable_prefix: &str,
    int_type: &IntegerType,
    used_as_size_in: &Option<String>,
    verified_value: &Option<ContainerValue>,
    size_of_fields_before_size: Option<i128>,
    is_manual_size_field: bool,
    postfix: &str,
) {
    let basic_type = int_type.rust_str();

    if is_manual_size_field {
        let size_of_fields_before_size = size_of_fields_before_size.unwrap();
        s.wln(format!("w.write_all(&((self.size() - {size_of_fields_before_size}) as {basic_type}).to_le_bytes()){postfix}?;"));
    } else if verified_value.is_some() {
        s.wln(format!(
            "w.write_all(&Self::{name}_VALUE.to_le_bytes()){postfix}?;",
            name = variable_name.to_uppercase(),
        ));
    } else if let Some(array) = used_as_size_in {
        s.wln(format!(
            "w.write_all(&({variable_prefix}{array}.len() as {basic_type}).to_le_bytes()){postfix}?;",
        ));
    } else {
        s.wln(format!(
            "w.write_all(&{variable_prefix}{variable_name}.to_le_bytes()){postfix}?;",
        ));
    }
}

pub(crate) fn print_write(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    prefix: &str,
    postfix: &str,
    variable_prefix: &str,
) {
    // For fully compressed messages, replace the writer with a ZLibDecoder.
    if e.tags().compressed() {
        // Fully compressed messages include the decompressed size as a u32 at the start of the packet.
        s.wln("w.write_all(&(self.size_uncompressed() as u32).to_le_bytes())?;");
        s.newline();

        s.wln("let mut w = &mut flate2::write::ZlibEncoder::new(w, flate2::Compression::fast());");
        s.newline();
    }

    for field in e.members() {
        print_write_field(s, e, o, field, variable_prefix, prefix, postfix);
    }
}

pub(crate) fn print_write_definition(
    s: &mut Writer,
    variable_prefix: &str,
    d: &StructMemberDefinition,
    postfix: &str,
) {
    let name = d.name();

    s.wln(format!("// {name}: {type_name}", type_name = d.ty().str()));

    match d.ty() {
        Type::Integer(int_type) => {
            print_write_field_integer(
                s,
                d.name(),
                variable_prefix,
                int_type,
                d.used_as_size_in(),
                d.value(),
                d.size_of_fields_before_size(),
                d.is_manual_size_field(),
                postfix,
            );
        }
        Type::Bool(i) => {
            s.wln(format!(
                "w.write_all({ty}::from({reference}{variable_prefix}{name}).to_le_bytes().as_slice()){postfix}?;",
                reference = if variable_prefix.is_empty() { "*" } else { "" }, // non-self variables are &bool
                ty = i.rust_str(),
            ));
        }
        Type::Level16 => {
            s.wln(format!(
                "w.write_all(&u16::from({variable_prefix}{name}.as_int()).to_le_bytes()){postfix}?;",
            ));
        }
        Type::Level32 => {
            s.wln(format!(
                "w.write_all(&u32::from({variable_prefix}{name}.as_int()).to_le_bytes()){postfix}?;",
            ));
        }
        Type::Level => {
            s.wln(format!(
                "w.write_all(&{variable_prefix}{name}.as_int().to_le_bytes()){postfix}?;",
            ));
        }
        Type::Gold => {
            s.wln(format!(
                "w.write_all(({variable_prefix}{name}.as_int()).to_le_bytes().as_slice()){postfix}?;",
            ));
        }
        Type::Spell16 | Type::Item | Type::Spell => {
            s.wln(format!(
                "w.write_all(&{variable_prefix}{name}.to_le_bytes()){postfix}?;",
            ));
        }
        Type::Seconds => {
            s.wln(format!(
                "w.write_all(({variable_prefix}{name}.as_secs() as u32).to_le_bytes().as_slice()){postfix}?;",
            ));
        }
        Type::Milliseconds => {
            s.wln(format!(
                "w.write_all(({variable_prefix}{name}.as_millis() as u32).to_le_bytes().as_slice()){postfix}?;",
            ));
        }
        Type::IpAddress => {
            s.wln(format!(
                "w.write_all(&{variable_prefix}{name}.octets()){postfix}?;",
            ));
        }
        Type::FloatingPoint => {
            s.wln(format!(
                "w.write_all(&{variable_prefix}{name}.to_le_bytes()){postfix}?;",
            ));
        }
        Type::Population => {
            s.wln(format!(
                "w.write_all(&{variable_prefix}{name}.as_int().to_le_bytes()){postfix}?;",
            ));
        }
        Type::SizedCString => {
            s.wln(format!(
                "w.write_all(&(({variable_prefix}{name}.len() + 1) as u32).to_le_bytes()){postfix}?;",
            ));
            s.wln(format!(
                "w.write_all({variable_prefix}{name}.as_bytes()){postfix}?;",
            ));
            s.wln("// Null terminator");
            s.wln(format!("w.write_all(&[0]){postfix}?;",));
        }
        //TODO: types that prevent null bytes in strings
        Type::CString => {
            s.wln("// TODO: Guard against strings that are already null-terminated");
            s.wln(format!(
                "assert_ne!({variable_prefix}{name}.as_bytes().iter().next_back(), Some(&0_u8), \"String `{name}` must not be null-terminated.\");",
            ));
            s.wln(format!(
                "w.write_all({variable_prefix}{name}.as_bytes()){postfix}?;",
            ));
            s.wln("// Null terminator");
            s.wln(format!("w.write_all(&[0]){postfix}?;"));
        }
        Type::String { .. } => {
            s.wln(format!(
                "w.write_all(&({variable_prefix}{name}.len() as u8).to_le_bytes()){postfix}?;",
            ));
            s.wln(format!(
                "w.write_all({variable_prefix}{name}.as_bytes()){postfix}?;",
            ));
        }
        Type::Array(array) => {
            print_write_field_array(s, d, variable_prefix, array, postfix);
        }
        Type::Enum { e, upcast } | Type::Flag { e, upcast } => {
            let integer = match upcast {
                None => e.ty(),
                Some(integer) => integer,
            };

            if matches!(integer, IntegerType::U48) {
                s.wln(format!(
                    "w.write_all(&({variable_prefix}{name}.as_int() as u32).to_le_bytes()){postfix}?;",
                ));
                s.wln(format!(
                    "w.write_all(&(({variable_prefix}{name}.as_int() >> 32) as u16).to_le_bytes()){postfix}?;",
                ));
            } else if upcast.is_none() || d.used_in_if() {
                s.wln(format!(
                    "w.write_all(&({variable_prefix}{name}.as_int().to_le_bytes())){postfix}?;",
                ));
            } else {
                s.wln(format!(
                        "w.write_all(&{ty}::from({variable_prefix}{name}.as_int()).to_le_bytes()){postfix}?;",
                        ty = integer.rust_str(),
                    ));
            }
        }
        Type::PackedGuid => {
            s.wln(format!(
                "crate::util::write_packed_guid(&{variable_prefix}{name}, &mut w)?;",
            ));
        }
        Type::DateTime => {
            s.wln(format!(
                "w.write_all(&{variable_prefix}{name}.as_int().to_le_bytes()){postfix}?;",
            ));
        }
        Type::Guid => {
            s.wln(format!(
                "w.write_all(&{variable_prefix}{name}.guid().to_le_bytes()){postfix}?;",
            ));
        }

        Type::MonsterMoveSplines => {
            s.wln(format!(
                "crate::util::write_monster_move_spline({variable_prefix}{name}.as_slice(), &mut w){postfix}?;",
            ));
        }
        Type::AchievementDoneArray => {
            s.wln(format!(
                 "crate::util::write_achievement_done({variable_prefix}{name}.as_slice(), &mut w){postfix}?;",
             ));
        }
        Type::AchievementInProgressArray => {
            s.wln(format!(
                 "crate::util::write_achievement_in_progress({variable_prefix}{name}.as_slice(), &mut w){postfix}?;",
             ));
        }
        Type::AddonArray => {
            s.wln(format!(
                "crate::util::write_addon_array({variable_prefix}{name}.as_slice(), &mut w){postfix}?;",
            ));
        }

        Type::VariableItemRandomProperty
        | Type::NamedGuid
        | Type::EnchantMask
        | Type::InspectTalentGearMask
        | Type::UpdateMask { .. }
        | Type::AuraMask => {
            s.wln(format!(
                "{variable_prefix}{name}.write_into_vec(&mut w){postfix}?;",
            ));
        }

        Type::Struct { e } => {
            if e.tags().is_in_base() {
                let f = base_struct_write_name(e);
                s.wln_no_indent(format!(
                    "crate::util::{f}(&{variable_prefix}{name}, &mut w)?;",
                ));
            } else {
                s.wln(format!(
                    "{variable_prefix}{name}.write_into_vec(&mut w){postfix}?;",
                ));
            }
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
            print_write_definition(s, variable_prefix, d, postfix);
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

    let enumerator_name = match statement.conditional().equation() {
        Equation::Equals { values: value } => &value[0],
        Equation::NotEquals { value } => value,
        Equation::BitwiseAnd { .. } => {
            unreachable!("enum printer is bitwise and")
        }
    };

    let rd = e
        .rust_object()
        .rust_definer_with_variable_name_and_enumerator(statement.name(), enumerator_name);

    let mut unused_enumerators = false;

    for enumerator in rd.enumerators() {
        if !enumerator.has_members() {
            unused_enumerators = true;

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
            if statement.contains(m) {
                print_write_field(s, e, o, m, "", prefix, postfix);
            }
        }

        s.closing_curly(); // enum::enumerator
    }

    if unused_enumerators {
        s.wln("_ => {}");
    }

    s.closing_curly_newline(); // match
}
