use crate::parser::types::array::{Array, ArraySize, ArrayType};
use crate::parser::types::container::Container;
use crate::parser::types::if_statement::{Equation, IfStatement};
use crate::parser::types::objects::Objects;
use crate::parser::types::sizes::{GUID_SIZE, SPELL_SIZE};
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::ty::Type;
use crate::parser::types::IntegerType;
use crate::rust_printer::base_structs::base_struct_read_name;
use crate::rust_printer::get_optional_type_name;
use crate::rust_printer::rust_view::rust_definer::RustDefiner;
use crate::rust_printer::rust_view::rust_type::RustType;
use crate::rust_printer::structs::print_common_impls::print_size::{
    print_rust_members_sizes, print_size_of_ty_rust_view,
};
use crate::rust_printer::writer::Writer;
use crate::rust_printer::{get_new_flag_type_name, DefinerType};
use crate::{MAX_ALLOCATION_SIZE, MAX_ALLOCATION_SIZE_WRATH, UTILITY_PATH};

fn print_read_array(
    s: &mut Writer,
    array: &Array,
    e: &Container,
    d: &StructMemberDefinition,
    prefix: &str,
    postfix: &str,
) {
    let name = d.name();

    if array.is_constant_sized_u8_array() {
        s.body_no_indent_or_space_with_semicolon(|s| {
            s.wln(format!(
                "let mut {name} = [0_u8; {size}];",
                size = array.size().str()
            ));
            s.wln(format!("r.read_exact(&mut {name}){postfix}?;",));
            s.wln(name);
        });

        return;
    }

    // Already wrote assignment in outer function
    s.wln_no_indent("{");
    s.inc_indent();

    match array.size() {
        ArraySize::Fixed(size) => {
            if array.inner_type_is_copy() {
                s.wln(format!(
                    "let mut {name} = [{type_name}::default(); {size}];",
                    type_name = array.ty().rust_str(),
                ));
            } else {
                s.wln(format!(
                    "let mut {name} = [(); {size}].map(|_| {ty}::default());",
                    ty = array.ty().rust_str()
                ));
            }

            s.body(format!("for i in {name}.iter_mut()"), |s| {
                print_array_ty(s, array, d, prefix, postfix, true, None);
            });
        }
        ArraySize::Variable(m) => {
            let length = m.name();

            s.wln(format!(
                "let mut {name} = Vec::with_capacity({length} as usize);",
            ));
            let object_min_size = array.ty().sizes().minimum();

            let (max_size, max_alloc_size) = if e.tags().contains_wrath() {
                ("MAX_ALLOCATION_SIZE_WRATH", MAX_ALLOCATION_SIZE_WRATH)
            } else {
                ("MAX_ALLOCATION_SIZE", MAX_ALLOCATION_SIZE)
            };

            if e.tags().has_world_version()
                && m.manual_size_field_max_value() * object_min_size > max_alloc_size
            {
                s.newline();

                let length = if m.manual_size_field_integer_size() < 8 {
                    format!("u64::from({length})")
                } else {
                    length.to_string()
                };

                let multiply = if object_min_size == 1 {
                    "".to_string()
                } else {
                    format!(" * {object_min_size}")
                };

                s.wln(format!("let allocation_size = {length}{multiply};"));

                s.bodyn(format!("if allocation_size > crate::errors::{max_size}"), |s| {
                    s.wln("return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));");
                });
            }

            s.body(format!("for _ in 0..{length}", length = m.name()), |s| {
                print_array_ty(s, array, d, prefix, postfix, false, None);
            });
        }
        ArraySize::Endless => {
            if array.compressed() {
                s.wln(format!(
                    "let {name}_decompressed_size = crate::util::read_u32_le(&mut r)?;"
                ));
                s.newline();

                s.wln(format!(
                    "let mut buf = Vec::with_capacity({name}_decompressed_size as usize);"
                ));

                s.wln("let mut decoder = &mut flate2::read::ZlibDecoder::new(r);");
                // TODO: I want to know if this fails, should this be a separate error mode?
                s.wln("decoder.read_to_end(&mut buf).unwrap();");
                s.wln("let mut r = &buf[..];");
                s.newline();
            }

            print_size_before_variable(s, e, d.name());

            if array.compressed() {
                s.wln(format!(
                    "current_size += 4; // {name}_decompressed_size: u32"
                ));
            }

            let use_decoder = array.compressed() || e.tags().compressed() && array.is_endless();
            let loop_condition = if use_decoder {
                "while !r.is_empty()"
            } else {
                "while current_size < (body_size as usize)"
            };

            s.wln(format!(
                "let mut {name} = Vec::with_capacity(body_size as usize - current_size);",
            ));
            s.body(loop_condition, |s| {
                let array_prefix = "let a = ".to_string();
                let array_postfix = ";".to_string();

                let size = match array.ty() {
                    ArrayType::Integer(integer_type) => integer_type.size().to_string(),
                    ArrayType::Spell => SPELL_SIZE.to_string(),
                    ArrayType::CString => "a.len() + 1".to_string(),
                    ArrayType::PackedGuid => "crate::util::packed_guid_size(&a)".to_string(),
                    ArrayType::Guid => GUID_SIZE.to_string(),
                    ArrayType::Struct(e) => {
                        if e.is_constant_sized() {
                            e.sizes().maximum().to_string()
                        } else {
                            "a.size()".to_string()
                        }
                    }
                };

                let static_sized = array.ty().sizes().is_constant().is_some();
                let intermediate_variable = !static_sized && !array.compressed();

                let array_override = if intermediate_variable {
                    Some((array_prefix, array_postfix))
                } else {
                    None
                };

                print_array_ty(s, array, d, prefix, postfix, false, array_override);

                if !array.compressed() {
                    s.wln(format!("current_size += {size};"));
                }

                if intermediate_variable {
                    s.wln(format!("{name}.push(a);"));
                }
            });
        }
    }

    s.wln(d.name());
    s.closing_curly_with(";");
}

fn print_array_ty(
    s: &mut Writer,
    array: &Array,
    d: &StructMemberDefinition,
    prefix: &str,
    postfix: &str,
    array_is_fixed: bool,
    prefix_postfix_override: Option<(String, String)>,
) {
    let (array_prefix, array_postfix) = match (prefix_postfix_override, array_is_fixed) {
        (None, true) => ("*i = ".to_string(), ";".to_string()),
        (None, false) => (format!("{name}.push(", name = d.name()), ");".to_string()),
        (Some((prefix, postfix)), _) => (prefix, postfix),
    };

    match array.ty() {
        ArrayType::Integer(integer_type) => {
            s.wln(format!(
                "{array_prefix}{UTILITY_PATH}::{prefix}read_{int_type}_le(&mut r){postfix}?{array_postfix}",
                int_type = integer_type.rust_str(),
            ));
        }
        ArrayType::Spell => {
            s.wln(format!(
                "{array_prefix}{UTILITY_PATH}::{prefix}read_u32_le(&mut r){postfix}?{array_postfix}",
            ));
        }
        ArrayType::CString => {
            s.wln(format!(
                "let s = crate::util::{prefix}read_c_string_to_vec(&mut r){postfix}?;",
            ));
            s.wln(format!(
                "{array_prefix}String::from_utf8(s)?{array_postfix}",
            ));
        }
        ArrayType::PackedGuid => {
            s.wln(format!(
                "{array_prefix}crate::util::read_packed_guid(&mut r){postfix}?{array_postfix}",
            ));
        }
        ArrayType::Guid => {
            s.wln(format!(
                "{array_prefix}crate::util::read_guid(&mut r)?{array_postfix}",
            ));
        }
        ArrayType::Struct(e) => {
            if e.tags().is_in_base() {
                let f = base_struct_read_name(e);
                s.wln(format!(
                    "{array_prefix}crate::util::{f}(&mut r){postfix}?{array_postfix}",
                ));
            } else {
                s.wln(format!(
                    "{array_prefix}{ty}::{prefix}read(&mut r){postfix}?{array_postfix}",
                    ty = array.ty().rust_str(),
                ));
            }
        }
    }
}

fn print_size_before_variable(s: &mut Writer, e: &Container, variable_name: &str) {
    s.body_closing_with_semicolon("let mut current_size =", |s| {
        if e.rust_object().members().len() == 1 {
            s.wln("0");
        }

        for (i, m) in e.rust_object().members().iter().enumerate() {
            if m.name() == variable_name {
                // Fields after the endless array should not be counted here
                break;
            }

            if i != 0 {
                s.w("+ ");
            } else {
                s.w("");
            }

            // Complex enums are not fully formed yet so they do not have a .size()
            // method and they can't have one because enums can be upcast
            match m.ty() {
                RustType::Enum {
                    is_simple, int_ty, ..
                }
                | RustType::Flag {
                    is_simple, int_ty, ..
                } => {
                    if !is_simple {
                        s.w_no_indent(int_ty.size().to_string());
                        s.wln_no_indent(m.size_comment());
                        continue;
                    }
                }
                _ => {}
            }

            print_size_of_ty_rust_view(s, m, "");
        }
    });
}

fn print_read_definition(
    s: &mut Writer,
    e: &Container,
    d: &StructMemberDefinition,
    prefix: &str,
    postfix: &str,
    assignment_prefix: &str,
) {
    s.wln(format!("// {}: {}", d.name(), d.ty().str()));

    let name = d.name();
    let type_name = d.ty().rust_str();
    let value_set = if d.value().is_some() || d.is_manual_size_field() {
        "_"
    } else {
        ""
    };

    s.w(format!("{assignment_prefix}{value_set}{name} = "));

    match &d.ty() {
        Type::Bool(i) => {
            s.wln_no_indent(format!(
                "{UTILITY_PATH}::{prefix}read_bool_{ty}(&mut r){postfix}?;",
                ty = i.rust_str(),
            ));
        }
        Type::DateTime => {
            s.wln_no_indent(format!(
                "DateTime::try_from({UTILITY_PATH}::{prefix}read_u32_le(&mut r){postfix}?)?;",
            ));
        }
        Type::Integer(_) => {
            s.wln_no_indent(format!(
                "{UTILITY_PATH}::{prefix}read_{type_name}_le(&mut r){postfix}?;",
            ));
        }
        Type::IpAddress => {
            s.wln_no_indent(format!(
                "Ipv4Addr::from({UTILITY_PATH}::{prefix}read_u32_be(&mut r){postfix}?);",
            ));
        }
        Type::FloatingPoint => {
            s.wln_no_indent(format!(
                "{UTILITY_PATH}::{prefix}read_f32_le(&mut r){postfix}?;",
            ));
        }
        Type::SizedCString => {
            s.body_no_indent_or_space_with_semicolon(|s| {
                s.wln(format!(
                    "let {name} = crate::util::read_u32_le(&mut r)?;",
                    name = d.name()
                ));
                s.wln(format!(
                    "let {name} = crate::util::read_sized_c_string_to_vec(&mut r, {name})?;",
                    name = d.name()
                ));
                s.wln(format!("String::from_utf8({name})?"));
            });
        }
        Type::CString => {
            s.body_no_indent_or_space_with_semicolon(|s| {
                s.wln(format!(
                    "let {name} = {UTILITY_PATH}::{prefix}read_c_string_to_vec(&mut r){postfix}?;",
                ));
                s.wln(format!("String::from_utf8({name})?"));
            });
        }
        Type::String => {
            s.body_no_indent_or_space_with_semicolon(|s| {
                s.wln(format!(
                    "let {name} = crate::util::{prefix}read_u8_le(&mut r){postfix}?;",
                ));
                s.wln(format!(
                    "let {name} = {UTILITY_PATH}::{prefix}read_fixed_string_to_vec(&mut r, {name} as usize){postfix}?;",
                ));
                s.wln(format!(
                    "String::from_utf8({name})?",
                ));
            });
        }
        Type::Array(array) => {
            print_read_array(s, array, e, d, prefix, postfix);
        }
        Type::Enum { e, upcast } => {
            let (parens, integer, cast) = if let Some(integer) = upcast {
                ("(", integer, format!(" as {})", e.ty().rust_str()))
            } else {
                ("", e.ty(), "".to_string())
            };

            s.wln_no_indent(format!(
                "{parens}crate::util::{prefix}read_{ty}_le(&mut r){postfix}?{cast}.try_into()?;",
                ty = integer.rust_str(),
            ));
        }
        Type::Population => {
            s.wln_no_indent(format!(
                "crate::util::{prefix}read_f32_le(&mut r){postfix}?.into();"
            ));
        }
        Type::Flag { e, .. } => {
            if matches!(e.ty(), &IntegerType::U48) {
                s.body_no_indent_or_space_with_semicolon(|s| {
                    s.wln(format!(
                        "let a = crate::util::{prefix}read_u32_le(&mut r){postfix}?;"
                    ));
                    s.wln(format!(
                        "let b = crate::util::{prefix}read_u16_le(&mut r){postfix}?;"
                    ));
                    s.wln(format!("{type_name}::new((a as u64) | ((b as u64) << 32))",));
                });
            } else {
                s.wln_no_indent(format!(
                    "{type_name}::new(crate::util::{prefix}read_{ty}_le(&mut r){postfix}?);",
                    ty = e.ty().rust_str(),
                ));
            }
        }
        Type::PackedGuid => {
            s.wln_no_indent(format!("crate::util::read_packed_guid(&mut r){postfix}?;",));
        }

        Type::Level => {
            s.wln_no_indent(format!(
                "Level::new({UTILITY_PATH}::{prefix}read_u8_le(&mut r){postfix}?);",
            ));
        }
        Type::Level16 => {
            s.wln_no_indent(format!(
                "Level::new({UTILITY_PATH}::{prefix}read_u16_le(&mut r){postfix}? as u8);",
            ));
        }
        Type::Level32 => {
            s.wln_no_indent(format!(
                "Level::new({UTILITY_PATH}::{prefix}read_u32_le(&mut r){postfix}? as u8);",
            ));
        }
        Type::Spell16 => {
            s.wln_no_indent(format!(
                "{UTILITY_PATH}::{prefix}read_u16_le(&mut r){postfix}?;",
            ));
        }
        Type::Spell | Type::Item => {
            s.wln_no_indent(format!(
                "{UTILITY_PATH}::{prefix}read_u32_le(&mut r){postfix}?;",
            ));
        }

        Type::Seconds => {
            s.wln_no_indent(format!(
                "Duration::from_secs({UTILITY_PATH}::{prefix}read_u32_le(&mut r){postfix}?.into());",
            ));
        }
        Type::Milliseconds => {
            s.wln_no_indent(format!(
                "Duration::from_millis({UTILITY_PATH}::{prefix}read_u32_le(&mut r){postfix}?.into());",
            ));
        }

        Type::Gold => {
            s.wln_no_indent(format!(
                "Gold::new({UTILITY_PATH}::{prefix}read_u32_le(&mut r){postfix}?);",
            ));
        }

        Type::MonsterMoveSplines => {
            s.wln_no_indent(format!(
                "crate::util::read_monster_move_spline(&mut r){postfix}?;",
            ));
        }
        Type::AchievementDoneArray => {
            s.wln_no_indent(format!(
                "crate::util::read_achievement_done(&mut r){postfix}?;",
            ));
        }
        Type::AchievementInProgressArray => {
            s.wln_no_indent(format!(
                "crate::util::read_achievement_in_progress(&mut r){postfix}?;",
            ));
        }
        Type::AddonArray => {
            s.wln_no_indent(format!("crate::util::read_addon_array(&mut r){postfix}?;",));
        }

        Type::Guid => {
            s.wln_no_indent("crate::util::read_guid(&mut r)?;");
        }

        Type::CacheMask
        | Type::VariableItemRandomProperty
        | Type::NamedGuid
        | Type::UpdateMask { .. }
        | Type::AuraMask
        | Type::EnchantMask
        | Type::InspectTalentGearMask => {
            s.wln_no_indent(format!("{type_name}::{prefix}read(&mut r){postfix}?;",));
        }
        Type::Struct { e } => {
            if e.tags().is_in_base() {
                let f = base_struct_read_name(e);
                s.wln_no_indent(format!("crate::util::{f}(&mut r)?;",));
            } else {
                s.wln_no_indent(format!("{type_name}::{prefix}read(&mut r){postfix}?;",));
            }
        }
    }

    if d.is_manual_size_field() {
        s.wln(format!("// {name} is dynamic size of the object",));
    } else if let Some(value) = d.value() {
        s.wln(format!(
            "// {name} is expected to always be {constant_string} ({constant_value})",
            constant_string = value.original_string(),
            constant_value = value.value(),
        ));
    }

    s.newline();
}

fn print_read_if_statement_flag(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    statement: &IfStatement,
    prefix: &str,
    postfix: &str,
) {
    s.open_curly(format!(
        "let {var_name}_{enumerator_name} = if {var_name}.is_{enumerator_name}()",
        var_name = statement.variable_name(),
        enumerator_name = statement.flag_get_enumerator().to_lowercase(),
    ));

    for m in statement.members() {
        print_read_field(s, e, o, m, prefix, postfix, "let ");
    }

    print_read_final_flag(
        s,
        &e.rust_object()
            .rust_definers_in_enumerator(&statement.flag_get_enumerator()),
    );

    let rd = e
        .rust_object()
        .rust_definer_with_variable_name_and_enumerator(
            statement.variable_name(),
            &statement.flag_get_enumerator(),
        );
    let new_ty_name = if statement.is_elseif_flag() {
        rd.ty_name().to_string()
    } else {
        get_new_flag_type_name(rd.ty_name(), &statement.flag_get_enumerator_rust_name())
    };

    if statement.else_ifs().is_empty() {
        s.open_curly(format!("Some({new_ty_name}",));
    } else {
        s.open_curly(format!(
            "Some({new_ty_name}::{enumerator}",
            enumerator = statement.flag_get_enumerator_rust_name(),
        ));
    }

    let enumerator = rd.get_enumerator(&statement.flag_get_enumerator());
    for m in enumerator.members_in_struct() {
        s.wln(m.struct_initialization_string());
    }

    s.closing_curly_with(")"); // Some(
    s.closing_curly(); // if

    for elseif in statement.else_ifs() {
        s.open_curly(format!(
            "else if {var_name}.is_{enumerator_name}()",
            var_name = elseif.variable_name(),
            enumerator_name = elseif.flag_get_enumerator().to_lowercase(),
        ));

        for m in elseif.members() {
            print_read_field(s, e, o, m, prefix, postfix, "let ");
        }

        print_read_final_flag(
            s,
            &e.rust_object()
                .rust_definers_in_enumerator(&elseif.flag_get_enumerator()),
        );

        s.open_curly(format!(
            "Some({new_ty_name}::{enumerator}",
            enumerator = elseif.flag_get_enumerator_rust_name(),
        ));

        let rd = e
            .rust_object()
            .rust_definer_with_variable_name_and_enumerator(
                elseif.variable_name(),
                &elseif.flag_get_enumerator(),
            );
        let enumerator = rd.get_enumerator(&elseif.flag_get_enumerator());

        for m in enumerator.members_in_struct() {
            s.wln(m.struct_initialization_string());
        }

        s.closing_curly_with(")"); // Some(

        s.closing_curly(); // elseif
    }

    s.open_curly("else");
    s.wln("None");
    s.closing_curly_with(";"); // else
    s.newline();
}

fn print_read_if_statement_enum(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    statement: &IfStatement,
    prefix: &str,
    postfix: &str,
) {
    let if_statement_variable_name = format!("{}_if", statement.variable_name());
    let match_prefix = if !statement.part_of_separate_if_statement() {
        format!("let {if_statement_variable_name} = ")
    } else {
        "".to_string()
    };
    s.open_curly(format!(
        "{match_prefix}match {name}",
        name = statement.variable_name()
    ));

    let enumerator_name = match statement.equation() {
        Equation::Equals { values: value } => &value[0],
        Equation::NotEquals { value } => value,
        Equation::BitwiseAnd { .. } => {
            unreachable!("enum printer is bitwise and")
        }
    };

    let rd = e
        .rust_object()
        .rust_definer_with_variable_name_and_enumerator(statement.variable_name(), enumerator_name);

    for enumerator in rd.enumerators() {
        let new_enum = if e.single_rust_definer().is_none() {
            rd.ty_name()
        } else {
            e.name()
        };

        if !enumerator.has_members_in_struct() {
            let result = if !statement.part_of_separate_if_statement() {
                format!(
                    "{new_enum}::{enumerator},",
                    enumerator = enumerator.rust_name()
                )
            } else {
                "{}".to_string()
            };
            s.wln(format!(
                "{old_enum}::{enumerator} => {result}",
                old_enum = rd.original_ty_name(),
                enumerator = enumerator.rust_name()
            ));
            continue;
        }

        s.open_curly(format!(
            "{old_enum}::{enumerator} =>",
            old_enum = rd.original_ty_name(),
            enumerator = enumerator.rust_name()
        ));

        let assignment_prefix = if !statement.part_of_separate_if_statement() {
            "let ".to_string()
        } else {
            format!("{if_statement_variable_name}_")
        };
        for m in enumerator.original_fields() {
            if statement.contains(m) {
                print_read_field(s, e, o, m, prefix, postfix, &assignment_prefix);
            }
        }

        print_read_final_flag(s, &e.rust_object().rust_definers_in_namespace(rd.ty_name()));

        if !statement.part_of_separate_if_statement() {
            s.open_curly(format!(
                "{new_enum}::{enumerator}",
                enumerator = enumerator.rust_name()
            ));

            for m in enumerator.members_in_struct() {
                s.wln(m.struct_initialization_string());
            }
            s.closing_curly(); // enum
        }

        s.closing_curly(); // =>
    }

    s.closing_curly_with(";"); // match
    s.newline();
}

fn print_read_field(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    field: &StructMember,
    prefix: &str,
    postfix: &str,
    assignment_prefix: &str,
) {
    match field {
        StructMember::Definition(d) => {
            print_read_definition(s, e, d, prefix, postfix, assignment_prefix);
        }
        StructMember::IfStatement(statement) => match statement.definer_type() {
            DefinerType::Enum => {
                print_read_if_statement_enum(s, e, o, statement, prefix, postfix);
            }
            DefinerType::Flag => {
                print_read_if_statement_flag(s, e, o, statement, prefix, postfix);
            }
        },
        StructMember::OptionalStatement(optional) => {
            s.wln(format!("// optional {}", optional.name()));
            s.body_closing_with_semicolon("let current_size =", |s| {
                if e.rust_object().members().is_empty() {
                    s.wln("0");
                }

                print_rust_members_sizes(s, e.rust_object().members(), None, "");
            });

            s.body_else_with_closing(
                format!(
                    "let {name} = if current_size < body_size as usize",
                    name = optional.name()
                ),
                ";",
                |s| {
                    for field in optional.members() {
                        print_read_field(s, e, o, field, prefix, postfix, assignment_prefix);
                    }

                    s.body_closing_with(
                        format!(
                            "Some({optional_ty_name}",
                            optional_ty_name = get_optional_type_name(e.name(), optional.name()),
                        ),
                        |s| {
                            let rust_optional = e.rust_object().optional().unwrap();
                            for field in rust_optional.members_in_struct() {
                                s.wln(format!("{name},", name = field.name()));
                            }
                        },
                        ")",
                    );
                },
                |s| {
                    s.wln("None");
                },
            );

            s.newline();
        }
    }
}

fn print_read_final_flag(s: &mut Writer, rds: &[RustDefiner]) {
    for rd in rds {
        if rd.is_simple() {
            continue;
        }

        let var_name = rd.variable_name();

        match rd.definer_type() {
            DefinerType::Enum => {}
            DefinerType::Flag => {
                s.open_curly(format!(
                    "let {var_name} = {ty_name}",
                    ty_name = rd.ty_name(),
                ));
                s.wln(format!("inner: {var_name}.as_int(),",));

                for enumerator in rd.complex_flag_enumerators() {
                    s.wln(format!(
                        "{field_name}: {var_name}_{f_name},",
                        field_name = enumerator.name().to_lowercase(),
                        f_name = enumerator.name().to_lowercase(),
                    ));
                }

                s.closing_curly_with(";\n");
            }
        }
    }
}

fn print_read_final_enums(s: &mut Writer, rds: &[RustDefiner]) {
    for rd in rds {
        if !rd.has_separate_if_statements() {
            continue;
        }

        match rd.definer_type() {
            DefinerType::Flag => unreachable!(),
            DefinerType::Enum => {
                let variable_name = rd.variable_name();
                let ty_name = rd.ty_name();
                s.open_curly(format!("let {variable_name}_if = match {variable_name}"));

                for enumerator in rd.enumerators() {
                    let enumerator_name = enumerator.rust_name();
                    let simple_ty_name = rd.original_ty_name();
                    s.body(format!("{simple_ty_name}::{enumerator_name} =>"), |s| {
                        s.open_curly(format!("{ty_name}::{enumerator_name}"));

                        for member in enumerator.members_in_struct() {
                            let member_name = member.name();
                            s.wln(format!("{member_name}: {variable_name}_if_{member_name},"));
                        }

                        s.closing_curly(); // NewEnumName::EnumeratorName
                    });
                }

                s.closing_curly_with(";"); // let _if = match _
            }
        }

        s.newline();
    }
}

pub(crate) fn print_read(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    prefix: &str,
    postfix: &str,
    object_create_overwrite: Option<&str>,
) {
    if e.all_definitions()
        .iter()
        .any(|a| matches!(a.ty(), Type::AddonArray))
    {
        s.wln("panic!(\"SKIP_SERIALIZE_READ_PANIC This message has an `AddonArray` tag which makes it impossible to generate a correct read implementation for it.\")");
        return;
    }

    for variable in e.enum_separate_if_statement_variables() {
        s.wln(format!("let mut {variable} = Default::default();"));
    }

    if !e.enum_separate_if_statement_variables().is_empty() {
        s.newline();
    }

    // For fully compressed messages, replace the reader with a ZLibDecoder.
    if e.tags().compressed() {
        // Fully compressed messages always start with a u32 containing the decompressed size.
        // We don't care about that, so we just ignore it.
        s.wln("let decompressed_size = crate::util::read_u32_le(r)?;;");
        s.wln("let decompressed_buffer = vec![0; decompressed_size as usize];");
        s.wln("let mut r = &mut flate2::read::ZlibDecoder::new_with_buf(r, decompressed_buffer);");

        s.wln("let mut buf = Vec::with_capacity(decompressed_size as usize);");
        // TODO: I want to know if this fails, should this be a separate error mode?
        s.wln("r.read_to_end(&mut buf).unwrap();");
        s.wln("let mut r = &buf[..];");
        s.newline();
    }

    for field in e.members() {
        print_read_field(s, e, o, field, prefix, postfix, "let ");
    }

    let rust_definers = e.rust_object().rust_definers_in_global_scope();
    print_read_final_flag(s, &rust_definers);
    print_read_final_enums(s, &rust_definers);

    if let Some(rd) = e.single_rust_definer() {
        s.wln(format!("Ok({}_if)", rd.variable_name()));
    } else {
        if let Some(object_create_overwrite) = object_create_overwrite {
            s.open_curly(format!("Ok({object_create_overwrite}"));
        } else {
            s.open_curly("Ok(Self");
        }

        for f in e.rust_object().members_in_struct() {
            if let RustType::Enum { is_simple, .. } = f.ty() {
                if !is_simple {
                    s.wln(format!("{name}: {name}_if,", name = f.name()));
                    continue;
                }
            }
            s.wln(format!("{name},", name = f.name()));
        }

        if let Some(optional) = e.rust_object().optional() {
            s.wln(format!("{name},", name = optional.name()));
        }

        s.dec_indent();
        s.wln("})");
    }
}
