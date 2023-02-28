use crate::parser::types::array::{Array, ArraySize, ArrayType};
use crate::parser::types::container::Container;
use crate::parser::types::if_statement::{Equation, IfStatement};
use crate::parser::types::objects::Objects;
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::ty::Type;
use crate::rust_printer::rust_view::{RustDefiner, RustType};
use crate::rust_printer::structs::print_common_impls::{
    print_rust_members_sizes, print_size_of_ty_rust_view,
};
use crate::rust_printer::{get_new_flag_type_name, get_new_type_name, DefinerType};
use crate::rust_printer::{get_optional_type_name, Writer};
use crate::UTILITY_PATH;

fn print_read_array_fixed(
    s: &mut Writer,
    array: &Array,
    d: &StructMemberDefinition,
    prefix: &str,
    postfix: &str,
    size: i64,
) {
    s.open_curly(format!("let {name} =", name = d.name()));

    let inner_is_constant_sized = array.inner_type_is_constant_sized();
    if inner_is_constant_sized {
        s.wln(format!(
            "let mut {name} = [{type_name}::default(); {size}];",
            name = d.name(),
            type_name = array.ty().rust_str(),
            size = size
        ));
    } else {
        s.wln(format!(
            "let mut {name} = Vec::with_capacity({size});",
            name = d.name(),
            size = size
        ));
    }

    if inner_is_constant_sized {
        s.open_curly(format!("for i in {name}.iter_mut()", name = d.name()));
    } else {
        s.open_curly(format!("for i in 0..{size}"));
    }

    match array.ty() {
        ArrayType::Integer(integer) => {
            if inner_is_constant_sized {
                s.wln(format!(
                    "*i = {module}::{prefix}read_{int_type}_{endian}(r){postfix}?;",
                    module = UTILITY_PATH,
                    int_type = integer.rust_str(),
                    endian = integer.rust_endian_str(),
                    prefix = prefix,
                    postfix = postfix,
                ));
            } else {
                s.wln(format!(
                    "{name}.push({module}::{prefix}read_{int_type}_{endian}(r){postfix}?);",
                    name = d.name(),
                    module = UTILITY_PATH,
                    int_type = integer.rust_str(),
                    endian = integer.rust_endian_str(),
                    prefix = prefix,
                    postfix = postfix,
                ));
            }
        }
        ArrayType::Struct(_) => {
            if inner_is_constant_sized {
                s.wln(format!(
                    "*i = {type_name}::{prefix}read(r){postfix}?;",
                    type_name = array.ty().rust_str(),
                    prefix = prefix,
                    postfix = postfix,
                ));
            } else {
                s.wln(format!(
                    "{name}.push({type_name}::{prefix}read(r){postfix}?);",
                    name = d.name(),
                    type_name = array.ty().rust_str(),
                    prefix = prefix,
                    postfix = postfix,
                ));
            }
        }
        ArrayType::CString => {
            s.wln(format!(
                "let s = crate::util::{prefix}read_c_string_to_vec(r){postfix}?;",
            ));
            match array.size() {
                ArraySize::Fixed(_) => s.wln(format!(
                    "{name}.push(String::from_utf8(s)?);",
                    name = d.name()
                )),
                ArraySize::Variable(_) => unimplemented!(),
                ArraySize::Endless => unimplemented!(),
            }
        }
        ArrayType::Guid => {
            s.wln(format!("*i = Guid::{prefix}read(r){postfix}?;",));
        }
        ArrayType::PackedGuid => {
            s.wln(format!(
                "{name}.push(Guid::{prefix}read_packed(r){postfix}?);",
                name = d.name(),
                prefix = prefix,
                postfix = postfix,
            ));
        }
    }

    s.closing_curly();

    if !inner_is_constant_sized {
        s.wln(format!("{name}.try_into().unwrap()", name = d.name()));
    } else {
        s.wln(d.name());
    }

    s.closing_curly_with(";");
}

fn print_read_array(
    s: &mut Writer,
    array: &Array,
    e: &Container,
    d: &StructMemberDefinition,
    prefix: &str,
    postfix: &str,
) {
    if array.is_constant_sized_u8_array() {
        s.body_closing_with_semicolon(format!("let {name} =", name = d.name()), |s| {
            s.wln(format!(
                "let mut {name} = [0_u8; {size}];",
                name = d.name(),
                size = array.size().str()
            ));
            s.wln(format!(
                "r.read_exact(&mut {name}){postfix}?;",
                name = d.name(),
                postfix = postfix
            ));
            s.wln(d.name());
        });

        return;
    }

    match array.size() {
        ArraySize::Fixed(size) => {
            print_read_array_fixed(s, array, d, prefix, postfix, size);
            return;
        }
        ArraySize::Variable(m) => {
            s.open_curly(format!("let {name} =", name = d.name()));
            s.wln(format!(
                "let mut {name} = Vec::with_capacity({length} as usize);",
                name = d.name(),
                length = m.name()
            ));

            s.body(format!("for i in 0..{length}", length = m.name()), |s| {
                print_array_ty(s, array, d, prefix, "r", postfix);
            });
        }
        ArraySize::Endless => {
            s.open_curly(format!("let {name} =", name = d.name()));

            if d.tags().is_compressed() {
                s.wln("let mut decoder = &mut flate2::read::ZlibDecoder::new(r);");
                s.newline();
            }

            print_size_before_variable(s, e, d.name());

            let reader = if d.tags().is_compressed() {
                "decoder"
            } else {
                "r"
            };
            let loop_condition = if let Some(decompressed_size_field) = d.tags().compressed() {
                format!("while decoder.total_out() < ({decompressed_size_field} as u64)")
            } else {
                "while current_size < (body_size as usize)".to_string()
            };

            s.wln(format!(
                "let mut {name} = Vec::with_capacity(body_size as usize - current_size);",
                name = d.name()
            ));
            s.body(loop_condition.as_str(), |s| {
                print_array_ty(s, array, d, prefix, reader, postfix);
                s.wln("current_size += 1;")
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
    reader: &str,
    postfix: &str,
) {
    match array.ty() {
        ArrayType::Integer(integer_type) => {
            s.wln(format!(
                "{name}.push({module}::{prefix}read_{int_type}_{endian}({reader}){postfix}?);",
                name = d.name(),
                module = UTILITY_PATH,
                int_type = integer_type.rust_str(),
                endian = integer_type.rust_endian_str(),
                prefix = prefix,
                reader = reader,
                postfix = postfix,
            ));
        }
        ArrayType::Struct(c) => {
            s.wln(format!(
                "{name}.push({type_name}::{prefix}read({reader}){postfix}?);",
                name = d.name(),
                type_name = c.name(),
                prefix = prefix,
                reader = reader,
                postfix = postfix,
            ));
        }
        ArrayType::CString => {
            s.wln(format!(
                "let s = crate::util::{prefix}read_c_string_to_vec(r){postfix}?;",
            ));
            s.wln(format!(
                "{name}.push(String::from_utf8(s)?);",
                name = d.name()
            ));
        }
        ArrayType::Guid => {
            s.wln(format!(
                "{name}.push(Guid::{prefix}read(r){postfix}?);",
                name = d.name(),
                prefix = prefix,
                postfix = postfix,
            ));
        }
        ArrayType::PackedGuid => {
            s.wln(format!(
                "{name}.push(Guid::{prefix}read_packed(r){postfix}?);",
                name = d.name(),
                prefix = prefix,
                postfix = postfix,
            ));
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

    match &d.ty() {
        Type::Bool(i) => {
            s.wln(format!(
                "{assignment_prefix}{name} = {module_name}::{prefix}read_{ty}_le(r){postfix}? != 0;",
                name = d.name(),
                module_name = UTILITY_PATH,
                prefix = prefix,
                postfix = postfix,
                ty = i.rust_str(),
            ));
        }
        Type::DateTime => {
            s.wln(format!(
                "{assignment_prefix}{name}: DateTime = {module_name}::{prefix}read_u32_le(r){postfix}?.try_into()?;",
                name = d.name(),
                module_name = UTILITY_PATH,
                prefix = prefix,
                postfix = postfix,
            ));
        }
        Type::Integer(integer) => {
            let value_set = if d.value().is_some() { "_" } else { "" };
            s.wln(format!(
                "{assignment_prefix}{value_set}{name} = {module_name}::{prefix}read_{ty}_{endian}(r){postfix}?;",
                value_set = value_set,
                name = d.name(),
                module_name = UTILITY_PATH,
                ty = integer.rust_str(),
                endian = integer.rust_endian_str(),
                prefix = prefix,
                postfix = postfix,
            ));
        }
        Type::Gold => {
            s.wln(format!(
                "{assignment_prefix}{name} = Gold::new({module_name}::{prefix}read_u32_le(r){postfix}?);",
                name = d.name(),
                module_name = UTILITY_PATH,
                prefix = prefix,
                postfix = postfix,
            ));
        }
        Type::FloatingPoint(floating) => {
            s.wln(format!(
                "{assignment_prefix}{name} = {module_name}::{prefix}read_{ty}_{endian}(r){postfix}?;",
                name = d.name(),
                module_name = UTILITY_PATH,
                ty = floating.rust_str(),
                endian = floating.rust_endian_str(),
                prefix = prefix,
                postfix = postfix,
            ));
        }
        Type::SizedCString => {
            s.body_closing_with_semicolon(
                format!("{assignment_prefix}{name} =", name = d.name()),
                |s| {
                    s.wln(format!(
                        "let {name} = crate::util::read_u32_le(r)?;",
                        name = d.name()
                    ));
                    s.wln(format!(
                        "let {name} = crate::util::read_sized_c_string_to_vec(r, {name})?;",
                        name = d.name()
                    ));
                    s.wln(format!("String::from_utf8({name})?", name = d.name()));
                },
            );
        }
        Type::CString => {
            s.body_closing_with_semicolon(
                format!("{assignment_prefix}{name} =", name = d.name()),
                |s| {
                    s.wln(format!(
                        "let {name} = {module}::{prefix}read_c_string_to_vec(r){postfix}?;",
                        name = d.name(),
                        module = UTILITY_PATH,
                        prefix = prefix,
                        postfix = postfix,
                    ));
                    s.wln(format!("String::from_utf8({name})?", name = d.name()));
                },
            );
        }
        Type::String => {
            s.body_closing_with_semicolon(format!("{assignment_prefix}{name} =", name = d.name()), |s| {
                s.wln(format!(
                    "let {name} = crate::util::{prefix}read_u8_le(r){postfix}?;",
                    name = d.name()
                ));
                s.wln(format!(
                    "let {name} = {module}::{prefix}read_fixed_string_to_vec(r, {name} as usize){postfix}?;",
                    name = d.name(),
                    module = UTILITY_PATH,
                    prefix = prefix, postfix = postfix,
                ));
                s.wln(format!(
                    "String::from_utf8({name})?",
                    name = d.name()
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

            s.wln(format!(
                "{assignment_prefix}{value_set}{name}: {type_name} = {parens}crate::util::{prefix}read_{ty}_{endian}(r){postfix}?{cast}.{into};",
                name = d.name(),
                type_name = d.ty().rust_str(),
                value_set = if d.value().is_some() { "_" } else { "" },
                endian = integer.rust_endian_str(),
                ty = integer.rust_str(),
                into = match e.self_value().is_some() {
                    true => "into()",
                    false => "try_into()?",
                },
                prefix = prefix,
                postfix = postfix,
            ));
        }
        Type::Flag { e, .. } => {
            s.wln(format!(
                "{assignment_prefix}{value_set}{name} = {type_name}::new(crate::util::{prefix}read_{ty}_{endian}(r){postfix}?);",
                name = d.name(),
                type_name = d.ty().rust_str(),
                value_set = if d.value().is_some() { "_" } else { "" },
                endian = e.ty().rust_endian_str(),
                ty = e.ty().rust_str(),
                prefix = prefix,
                postfix = postfix,
            ));
        }
        Type::PackedGuid => {
            s.wln(format!(
                "{assignment_prefix}{name} = Guid::{prefix}read_packed(r){postfix}?;",
                name = d.name(),
                prefix = prefix,
                postfix = postfix,
            ));
        }

        Type::Struct { .. }
        | Type::Guid
        | Type::UpdateMask
        | Type::AuraMask
        | Type::AchievementDoneArray
        | Type::AchievementInProgressArray
        | Type::MonsterMoveSpline
        | Type::EnchantMask
        | Type::InspectTalentGearMask => {
            s.wln(format!(
                "{assignment_prefix}{name} = {ty}::{prefix}read(r){postfix}?;",
                name = d.name(),
                ty = d.ty().rust_str(),
                prefix = prefix,
                postfix = postfix,
            ));
        }
    }

    if let Some(value) = d.value() {
        s.wln(format!(
            "// {name} is expected to always be {constant_string} ({constant_value})",
            name = d.name(),
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
        var_name = statement.name(),
        enumerator_name = statement.flag_get_enumerator(),
    ));

    for m in statement.members() {
        print_read_field(s, e, o, m, prefix, postfix, "let ");
    }

    print_read_final_flag(
        s,
        &e.rust_object()
            .rust_definers_in_enumerator(&statement.flag_get_enumerator()),
    );

    let new_ty_name = get_new_flag_type_name(
        &get_new_type_name(e.name(), &statement.original_ty().rust_str()),
        &statement.flag_get_enumerator_rust_name(),
    );

    if statement.else_ifs().is_empty() {
        s.open_curly(format!("Some({new_ty_name}",));
    } else {
        s.open_curly(format!(
            "Some({new_ty_name}::{enumerator}",
            enumerator = statement.flag_get_enumerator_rust_name(),
        ));
    }

    let rd = e
        .rust_object()
        .rust_definer_with_variable_name_and_enumerator(
            statement.name(),
            &statement.flag_get_enumerator(),
        );
    let enumerator = rd.get_enumerator(&statement.flag_get_enumerator());
    for m in enumerator.members_in_struct() {
        s.wln(m.struct_initialization_string());
    }

    s.closing_curly_with(")"); // Some(
    s.closing_curly(); // if

    for elseif in statement.else_ifs() {
        s.open_curly(format!(
            "else if {var_name}.is_{enumerator_name}()",
            var_name = elseif.name(),
            enumerator_name = elseif.flag_get_enumerator(),
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
                elseif.name(),
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
    let if_statement_variable_name = format!("{}_if", statement.name());
    let match_prefix = if !statement.part_of_separate_if_statement() {
        format!("let {} = ", if_statement_variable_name)
    } else {
        "".to_string()
    };
    s.open_curly(format!(
        "{match_prefix}match {name}",
        name = statement.name()
    ));

    let enumerator_name = match &statement.conditional().equations()[0] {
        Equation::Equals { value, .. } | Equation::NotEquals { value, .. } => value,
        _ => unreachable!("enum printer is bitwise and"),
    };

    let rd = e
        .rust_object()
        .rust_definer_with_variable_name_and_enumerator(statement.name(), enumerator_name);

    for enumerator in rd.enumerators() {
        if !enumerator.has_members_in_struct() {
            let result = if !statement.part_of_separate_if_statement() {
                format!(
                    "{new_enum}::{enumerator},",
                    new_enum = rd.ty_name(),
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
            format!("{}_", if_statement_variable_name)
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
                new_enum = rd.ty_name(),
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

        match rd.definer_type() {
            DefinerType::Enum => {}
            DefinerType::Flag => {
                s.open_curly(format!(
                    "let {var_name} = {ty_name}",
                    var_name = rd.variable_name(),
                    ty_name = rd.ty_name(),
                ));
                s.wln(format!(
                    "inner: {var_name}.as_int(),",
                    var_name = rd.variable_name()
                ));

                for enumerator in rd.complex_flag_enumerators() {
                    s.wln(format!(
                        "{field_name}: {c_var_name}_{f_name},",
                        field_name = enumerator.name().to_lowercase(),
                        c_var_name = rd.variable_name(),
                        f_name = enumerator.name(),
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

pub(crate) fn print_read(s: &mut Writer, e: &Container, o: &Objects, prefix: &str, postfix: &str) {
    if e.all_definitions()
        .iter()
        .any(|a| a.tags().skip_serialize())
    {
        s.wln("panic!(\"SKIP_SERIALIZE_READ_PANIC This message has a `skip_serialize` tag which makes it impossible to generate a correct read implementation for it.\")");
        return;
    }

    for variable in e.enum_separate_if_statement_variables() {
        s.wln(format!("let mut {} = Default::default();", variable));
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
        s.newline();
    }

    for field in e.members() {
        print_read_field(s, e, o, field, prefix, postfix, "let ");
    }

    let rust_definers = e.rust_object().rust_definers_in_global_scope();
    print_read_final_flag(s, &rust_definers);
    print_read_final_enums(s, &rust_definers);

    s.open_curly("Ok(Self");

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
