use std::collections::HashMap;

use crate::container::{Container, StructMember, StructMemberDefinition};
use crate::parser::types::objects::Objects;
use crate::parser::types::ty::Type;
use crate::parser::types::{Array, ArraySize, ArrayType, ObjectType};
use crate::rust_printer::complex_print::{ComplexEnum, DefinerType};
use crate::rust_printer::new_enums::{
    IfStatementType, NewEnumStructMember, NewEnumerator, NewIfStatement,
};
use crate::rust_printer::rust_view::RustType;
use crate::rust_printer::structs::print_common_impls::print_size_of_ty;
use crate::rust_printer::{ImplType, Writer};
use crate::UTILITY_PATH;

fn print_read_array(
    s: &mut Writer,
    array: &Array,
    e: &Container,
    d: &StructMemberDefinition,
    o: &Objects,
    prefix: &str,
    postfix: &str,
) {
    if array.is_constant_sized_u8_array() {
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
        s.newline();
        return;
    }

    match array.size() {
        ArraySize::Fixed(size) => {
            if e.is_constant_sized() {
                s.wln(format!(
                    "let mut {name} = [{type_name}::default(); {size}];",
                    name = d.name(),
                    type_name = match array.ty() {
                        ArrayType::Integer(i) => {
                            i.rust_str()
                        }
                        ArrayType::Complex(v) => {
                            v
                        }
                        ArrayType::CString => "String",
                        ArrayType::Guid | ArrayType::PackedGuid => "Guid",
                    },
                    size = size
                ));
            } else {
                s.wln(format!(
                    "let mut {name} = Vec::with_capacity({size} as usize);",
                    name = d.name(),
                    size = size
                ));
            }
            s.open_curly(format!("for i in 0..{size}", size = size));

            match array.ty() {
                ArrayType::Integer(integer) => {
                    if e.is_constant_sized() {
                        s.wln(format!(
                            "{name}[i] = {module}::{prefix}read_{int_type}_{endian}(r){postfix}?;",
                            name = d.name(),
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
                ArrayType::Complex(_) => {
                    if e.is_constant_sized() {
                        s.wln(format!(
                            "{name}[i] = {type_name}::{prefix}read(r){postfix}?;",
                            name = d.name(),
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
                        prefix = prefix,
                        postfix = postfix,
                    ));
                    match array.size() {
                        ArraySize::Fixed(_) => s.wln(format!(
                            "{name}[i] = String::from_utf8(s)?;",
                            name = d.name()
                        )),
                        ArraySize::Variable(_) => unreachable!(),
                        ArraySize::Endless => panic!(),
                    }
                }
                ArrayType::Guid => {
                    s.wln(format!(
                        "{name}[i] = Guid::{prefix}read(r){postfix}?;",
                        name = d.name(),
                        prefix = prefix,
                        postfix = postfix,
                    ));
                }
                ArrayType::PackedGuid => {
                    s.wln(format!(
                        "{name}[i] = Guid::{prefix}read_packed(r){postfix}?;",
                        name = d.name(),
                        prefix = prefix,
                        postfix = postfix,
                    ));
                }
            }

            s.closing_curly();

            if !e.is_constant_sized() {
                s.wln(format!(
                    "let {name} = {name}.try_into().unwrap();",
                    name = d.name()
                ));
            }

            s.newline();
        }
        ArraySize::Variable(length) => {
            s.wln(format!(
                "let mut {name} = Vec::with_capacity({length} as usize);",
                name = d.name(),
                length = length
            ));
            s.open_curly(format!("for i in 0..{length}", length = length));

            match array.ty() {
                ArrayType::Integer(integer_type) => {
                    s.wln(format!(
                        "{name}.push({module}::{prefix}read_{int_type}_{endian}(r){postfix}?);",
                        name = d.name(),
                        module = UTILITY_PATH,
                        int_type = integer_type.rust_str(),
                        endian = integer_type.rust_endian_str(),
                        prefix = prefix,
                        postfix = postfix,
                    ));
                }
                ArrayType::Complex(_) => {
                    s.wln(format!(
                        "{name}.push({type_name}::{prefix}read(r){postfix}?);",
                        name = d.name(),
                        type_name = array.ty().rust_str(),
                        prefix = prefix,
                        postfix = postfix,
                    ));
                }
                ArrayType::CString => {
                    s.wln(format!(
                        "let s = crate::util::{prefix}read_c_string_to_vec(r){postfix}?;",
                        prefix = prefix,
                        postfix = postfix,
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
                        "{name}.push(Guid::{prefix}read_packed(r){postfix}?;",
                        name = d.name(),
                        prefix = prefix,
                        postfix = postfix,
                    ));
                }
            }

            s.closing_curly_newline()
        }
        ArraySize::Endless => {
            print_current_size(s, e, d, o);

            s.wln(format!(
                "let mut {name} = Vec::with_capacity(body_size as usize - current_size);",
                name = d.name()
            ));
            s.body("while current_size < (body_size as usize)", |s| {
                match array.ty() {
                    ArrayType::Integer(integer_type) => {
                        s.wln(format!(
                            "{name}.push({module}::{prefix}read_{int_type}_{endian}(r){postfix}?);",
                            name = d.name(),
                            module = UTILITY_PATH,
                            int_type = integer_type.rust_str(),
                            endian = integer_type.rust_endian_str(),
                            prefix = prefix,
                            postfix = postfix,
                        ));
                    }
                    ArrayType::Complex(_) => {
                        s.wln(format!(
                            "{name}.push({type_name}::{prefix}read(r){postfix}?);",
                            name = d.name(),
                            type_name = array.ty().rust_str(),
                            prefix = prefix,
                            postfix = postfix,
                        ));
                    }
                    ArrayType::CString => {
                        s.wln(format!(
                            "let s = crate::util::{prefix}read_c_string_to_vec(r){postfix}?;",
                            prefix = prefix,
                            postfix = postfix,
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
                            "{name}.push(Guid::{prefix}read_packed(r){postfix}?;",
                            name = d.name(),
                            prefix = prefix,
                            postfix = postfix,
                        ));
                    }
                }
                s.wln("current_size += 1;");
            });
            s.newline();
        }
    }
}

fn print_current_size(s: &mut Writer, e: &Container, d: &StructMemberDefinition, o: &Objects) {
    s.body_closing_with(
        "let mut current_size =",
        |s| {
            for (i, size) in e.nested_types().declarations().iter().enumerate() {
                if size.name() == d.name() {
                    break;
                }
                if i != 0 {
                    s.w("+ ");
                } else {
                    s.w("");
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
                    size.ty(),
                    size.name(),
                    true,
                    true,
                    array_inner_constant,
                    "",
                    &size.ty().str(),
                );
            }
        },
        ";",
    );
}

fn print_read_definition(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    d: &StructMemberDefinition,
    prefix: &str,
    postfix: &str,
) {
    s.wln(format!("// {}: {}", d.name(), d.ty().str()));

    match &d.ty() {
        Type::Integer(integer) => {
            let value_set = if d.verified_value().is_some() {
                "_"
            } else {
                ""
            };
            s.wln(format!(
                "let {value_set}{name} = {module_name}::{prefix}read_{ty}_{endian}(r){postfix}?;",
                value_set = value_set,
                name = d.name(),
                module_name = UTILITY_PATH,
                ty = integer.rust_str(),
                endian = integer.rust_endian_str(),
                prefix = prefix,
                postfix = postfix,
            ));
            if d.verified_value().is_some() {
                s.wln(format!(
                    "// {name} is expected to always be {value}",
                    name = d.name(),
                    value = d.verified_value().as_ref().unwrap(),
                ))
            }

            s.newline();
        }
        Type::FloatingPoint(floating) => {
            s.wln(format!(
                "let {name} = {module_name}::{prefix}read_{ty}_{endian}(r){postfix}?;",
                name = d.name(),
                module_name = UTILITY_PATH,
                ty = floating.rust_str(),
                endian = floating.rust_endian_str(),
                prefix = prefix,
                postfix = postfix,
            ));
        }
        Type::CString => {
            s.wln(format!(
                "let {name} = {module}::{prefix}read_c_string_to_vec(r){postfix}?;",
                name = d.name(),
                module = UTILITY_PATH,
                prefix = prefix,
                postfix = postfix,
            ));
            s.wln(format!(
                "let {name} = String::from_utf8({name})?;",
                name = d.name()
            ));

            s.newline();
        }
        Type::String { length } => {
            s.wln(format!(
                "let {name} = {module}::{prefix}read_fixed_string_to_vec(r, {length} as usize){postfix}?;",
                name = d.name(),
                module = UTILITY_PATH,
                length = length,
                prefix = prefix, postfix = postfix,
            ));
            s.wln(format!(
                "let {name} = String::from_utf8({name})?;",
                name = d.name()
            ));

            s.newline();
        }
        Type::Array(array) => {
            print_read_array(s, array, e, d, o, prefix, postfix);
        }
        Type::Identifier { s: ty, upcast } => {
            if o.get_object_type_of(ty, e.tags()) == ObjectType::Enum {
                if let Some(integer) = upcast {
                    if let Some(value) = d.verified_value() {
                        s.wln(format!(
                            "let _{name} = {ty_name}::{prefix}read_{ty}_{endian}(r){postfix}?;",
                            name = d.name(),
                            ty_name = ty,
                            ty = integer.rust_str(),
                            endian = integer.rust_endian_str(),
                            prefix = prefix,
                            postfix = postfix,
                        ));
                        s.wln(format!(
                            "// {name} is expected to always be {constant_string} ({constant_value})",
                            name = d.name(),
                            constant_string = value.original_string(),
                            constant_value = value.value(),
                        ));
                        s.newline();
                    } else {
                        s.wln(format!(
                            "let {name} = {ty_name}::{prefix}read_{ty}_{endian}(r){postfix}?;",
                            name = d.name(),
                            ty_name = ty,
                            ty = integer.rust_str(),
                            endian = integer.rust_endian_str(),
                            prefix = prefix,
                            postfix = postfix,
                        ));
                        s.newline();
                    }
                    return;
                }
            }
            s.wln(format!(
                "let {value_set}{name} = {type_name}::{prefix}read(r){postfix}?;",
                name = d.name(),
                type_name = d.ty().rust_str(),
                value_set = if d.value().is_some() { "_" } else { "" },
                prefix = prefix,
                postfix = postfix,
            ));
            if d.verified_value().is_some() {
                s.wln(format!(
                    "// {name} is expected to always be {constant_string} ({constant_value})",
                    name = d.name(),
                    constant_string = d.verified_value().as_ref().unwrap().original_string(),
                    constant_value = d.verified_value().as_ref().unwrap().value(),
                ));
            }

            s.newline();
        }
        Type::PackedGuid => {
            s.wln(format!(
                "let {name} = Guid::{prefix}read_packed(r){postfix}?;",
                name = d.name(),
                prefix = prefix,
                postfix = postfix,
            ));
            s.newline();
        }
        Type::Guid => {
            s.wln(format!(
                "let {name} = Guid::{prefix}read(r){postfix}?;",
                name = d.name(),
                prefix = prefix,
                postfix = postfix,
            ));
            s.newline();
        }
        Type::UpdateMask => {
            s.wln(format!(
                "let {name} = UpdateMask::{prefix}read(r){postfix}?;",
                name = d.name(),
                prefix = prefix,
                postfix = postfix,
            ));
            s.newline();
        }
        Type::AuraMask => {
            s.wln(format!(
                "let {name} = AuraMask::{prefix}read(r){postfix}?;",
                name = d.name(),
                prefix = prefix,
                postfix = postfix,
            ));
            s.newline();
        }
    }
}

fn print_read_field(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    field: &StructMember,
    prefix: &str,
    postfix: &str,
) {
    match field {
        StructMember::Definition(d) => {
            print_read_definition(s, e, o, d, prefix, postfix);
        }
        StructMember::IfStatement(statement) => match statement.definer_type() {
            DefinerType::Enum => {
                print_read_if_statement_enum_new(s, e, o, statement.new_enum(), prefix, postfix);
            }
            DefinerType::Flag => {
                print_read_if_statement_flag_new(s, e, o, statement.new_enum(), prefix, postfix);
            }
        },
        StructMember::OptionalStatement(optional) => {
            // TODO OPTIONAL
            s.wln(format!("// optional {}", optional.name()));
            s.body_closing_with(
                "let current_size =",
                |s| {
                    // TODO: Make this only show up when no fields are present
                    s.wln("0 // If no fields are present, TODO remove when not needed");

                    for size in e.fields() {
                        let size = match size {
                            StructMember::Definition(d) => d,
                            StructMember::IfStatement(_) => panic!(),
                            StructMember::OptionalStatement(opt) => {
                                if opt.name() == optional.name() {
                                    break;
                                }
                                panic!()
                            }
                        };
                        if size.name() == optional.name() {
                            break;
                        }
                        s.w("+ ");

                        let array_inner_constant = match size.ty() {
                            Type::Array(array) => match array.ty() {
                                ArrayType::Integer(_) => true,
                                ArrayType::Complex(ident) => {
                                    o.type_has_constant_size(&Type::Identifier {
                                        s: ident.clone(),
                                        upcast: None,
                                    })
                                }
                                ArrayType::CString => false,
                                ArrayType::Guid => true,
                                ArrayType::PackedGuid => false,
                            },
                            _ => false,
                        };

                        print_size_of_ty(
                            s,
                            size.ty(),
                            size.name(),
                            true,
                            true,
                            array_inner_constant,
                            "",
                            &size.ty().str(),
                        );
                    }
                },
                ";",
            );

            s.body_else_with_closing(
                format!(
                    "let {name} = if current_size < body_size as usize",
                    name = optional.name()
                ),
                ";",
                |s| {
                    for field in optional.members() {
                        print_read_field(s, e, o, field, prefix, postfix);
                    }

                    s.body_closing_with(
                        format!(
                            "Some({original_name}_{name}",
                            original_name = e.name(),
                            name = optional.name()
                        ),
                        |s| {
                            for field in optional.members() {
                                match field {
                                    StructMember::Definition(d) => {
                                        s.wln(format!("{name},", name = d.name()));
                                    }
                                    StructMember::IfStatement(_) => {}
                                    StructMember::OptionalStatement(_) => {}
                                }
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

fn print_read_if_statement_flag_multiple(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    ne: &NewIfStatement,
    prefix: &str,
    postfix: &str,
) {
    let enumerators = ne
        .enumerators()
        .iter()
        .filter(|a| !a.fields().is_empty())
        .collect::<Vec<&NewEnumerator>>();

    let original_enumerator = enumerators[0].name();
    for (i, enumerator) in enumerators.iter().enumerate() {
        if enumerator.fields().is_empty() {
            continue;
        }

        if i == 0 {
            s.w(format!(
                "let {name}_{enumerator} = ",
                name = ne.variable_name(),
                enumerator = enumerator.name()
            ));
        } else {
            s.w("else ");
        }

        s.wln_no_indent(format!(
            "if {name}.is_{enumerator}() {{",
            name = ne.variable_name(),
            enumerator = enumerator.name()
        ));
        s.inc_indent();

        for f in enumerator.fields() {
            match f {
                NewEnumStructMember::Definition(d) => {
                    print_read_definition(s, e, o, d, prefix, postfix);
                }
                NewEnumStructMember::IfStatement(statement) => match statement.enum_or_flag() {
                    IfStatementType::Enum => {
                        print_read_if_statement_enum_new(s, e, o, statement, prefix, postfix);
                    }
                    IfStatementType::Flag => {
                        print_read_if_statement_flag_new(s, e, o, statement, prefix, postfix);
                    }
                },
            }
        }

        s.body_closing_with(
            format!(
                "Some({new_ty_name}_{original_enumerator}::{enumerator}",
                new_ty_name = ne.new_ty_name(),
                original_enumerator = original_enumerator,
                enumerator = enumerator.name(),
            ),
            |s| {
                for d in enumerator.get_variable_names_for_members() {
                    if d.used_as_size_in().is_some() {
                        continue;
                    }
                    if d.verified_value().is_some() {
                        s.wln(format!("{name}: _{name},", name = d.name()));
                    } else {
                        s.wln(format!("{name},", name = d.name()));
                    };
                }
            },
            ")",
        );

        s.closing_curly();
    }

    s.body_closing_with(
        "else",
        |s| {
            s.wln("None");
        },
        ";",
    );
    s.newline();
}

fn print_read_if_statement_flag_new(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    ne: &NewIfStatement,
    prefix: &str,
    postfix: &str,
) {
    let size = ne
        .enumerators()
        .iter()
        .filter(|a| !a.fields().is_empty())
        .count();
    if size != 1 {
        print_read_if_statement_flag_multiple(s, e, o, ne, prefix, postfix);
        return;
    }

    assert!(ne.enum_or_flag() == IfStatementType::Flag);
    let enumerator = ne.single_enumerator_with_fields();

    s.body_else_with_closing(
        format!(
            "let {var_name}_{field_name} = if {var_name}.is_{field_name}()",
            var_name = ne.variable_name(),
            field_name = enumerator.name(),
        ),
        ";",
        |s| {
            for f in enumerator.fields() {
                match f {
                    NewEnumStructMember::Definition(d) => {
                        print_read_definition(s, e, o, d, prefix, postfix);
                    }
                    NewEnumStructMember::IfStatement(statement) => match statement.enum_or_flag() {
                        IfStatementType::Enum => {
                            print_read_if_statement_enum_new(s, e, o, statement, prefix, postfix);
                        }
                        IfStatementType::Flag => {
                            print_read_if_statement_flag_new(s, e, o, statement, prefix, postfix);
                        }
                    },
                }
            }

            s.open_curly(format!(
                "Some({}{}{}",
                e.name(),
                ne.original_ty_name(),
                enumerator.name(),
            ));
            for d in enumerator.get_variable_names_for_members() {
                if d.used_as_size_in().is_some() {
                    continue;
                }
                if d.verified_value().is_some() {
                    s.wln(format!("{name}: _{name},", name = d.name()));
                } else {
                    s.wln(format!("{name},", name = d.name()));
                };
            }

            s.closing_curly_with(")");
        },
        |s| {
            s.wln("None");
        },
    );
    s.newline();
}

fn print_read_if_statement_enum_new(
    s: &mut Writer,
    e: &Container,
    o: &Objects,
    ne: &NewIfStatement,
    prefix: &str,
    postfix: &str,
) {
    assert!(ne.enum_or_flag() == IfStatementType::Enum);

    s.open_curly(format!(
        "let {name}_if = match {name}",
        name = ne.variable_name()
    ));

    for en in ne.enumerators() {
        if en.fields().is_empty() {
            s.wln(format!(
                "{old_enum}::{enumerator} => {new_enum}::{enumerator},",
                old_enum = ne.original_ty_name(),
                new_enum = ne.new_ty_name(),
                enumerator = en.name()
            ));
            continue;
        }

        s.open_curly(format!(
            "{old_enum}::{enumerator} =>",
            old_enum = ne.original_ty_name(),
            enumerator = en.name()
        ));

        for f in en.fields() {
            match f {
                NewEnumStructMember::Definition(d) => {
                    print_read_definition(s, e, o, d, prefix, postfix);
                }
                NewEnumStructMember::IfStatement(statement) => match statement.enum_or_flag() {
                    IfStatementType::Enum => {
                        print_read_if_statement_enum_new(s, e, o, statement, prefix, postfix);
                    }
                    IfStatementType::Flag => {
                        print_read_if_statement_flag_new(s, e, o, statement, prefix, postfix);
                    }
                },
            }
        }

        let mut nested_enums = Vec::new();
        for n in e.nested_types().new_enums() {
            for f in en.fields() {
                if n.variable_name()
                    == match f {
                        NewEnumStructMember::Definition(d) => d.name(),
                        NewEnumStructMember::IfStatement(_) => continue,
                    }
                {
                    nested_enums.push(n.clone());
                }
            }
        }
        print_read_final_flag(s, nested_enums.as_slice());

        s.open_curly(format!(
            "{new_enum}::{enumerator}",
            new_enum = ne.new_ty_name(),
            enumerator = en.name()
        ));
        let mut flags_written = HashMap::new();
        for f in en.fields() {
            s.wln(format!(
                "{},",
                match f {
                    NewEnumStructMember::Definition(d) => {
                        if d.verified_value().is_some()
                            || d.used_as_size_in().is_some()
                            || d.used_in_if()
                        {
                            continue;
                        }
                        d.name().to_string()
                    }
                    NewEnumStructMember::IfStatement(statement) => {
                        match statement.enum_or_flag() {
                            IfStatementType::Enum => {
                                format!("{name}: {name}_if", name = statement.variable_name())
                            }
                            IfStatementType::Flag => {
                                if flags_written.contains_key(statement.variable_name()) {
                                    continue;
                                }
                                flags_written.insert(statement.variable_name(), ());
                                statement.variable_name().to_string()
                            }
                        }
                    }
                }
            ));
        }
        s.closing_curly();
        s.closing_curly();
    }

    s.closing_curly_with(";");
    s.newline();
}

fn print_read_final_flag(s: &mut Writer, nested_types: &[ComplexEnum]) {
    for c in nested_types {
        match c.definer_ty() {
            DefinerType::Enum => {}
            DefinerType::Flag => {
                s.open_curly(format!(
                    "let {var_name} = {struct_name}{ty_name}",
                    var_name = c.variable_name(),
                    struct_name = c.original_struct_name(),
                    ty_name = c.original_ty_name(),
                ));
                s.wln(format!(
                    "inner: {var_name}.as_{ty}(),",
                    var_name = c.variable_name(),
                    ty = c.ty().rust_str()
                ));

                for f in c.fields() {
                    if f.should_not_be_in_type() {
                        continue;
                    }

                    s.wln(format!(
                        "{field_name}: {c_var_name}_{f_name},",
                        field_name = f.name().to_lowercase(),
                        c_var_name = c.variable_name(),
                        f_name = f.name(),
                    ));
                }

                s.closing_curly_with(";\n");
            }
        }
    }
}

pub fn print_read(s: &mut Writer, e: &Container, o: &Objects, prefix: &str, postfix: &str) {
    for field in e.fields() {
        print_read_field(s, e, o, field, prefix, postfix);
    }

    let mut nested_types = Vec::new();
    for n in e.nested_types().new_enums() {
        for d in e.nested_types().declarations() {
            if n.variable_name() == d.name() {
                nested_types.push(n.clone());
            }
        }
    }

    print_read_final_flag(s, &nested_types);

    s.open_curly("Ok(Self");

    for f in e.rust_object().members() {
        match f.ty() {
            RustType::Enum { is_simple, .. } => {
                if !is_simple {
                    s.wln(format!("{name}: {name}_if,", name = f.name()));
                    continue;
                }
            }
            _ => {}
        }
        s.wln(format!("{name},", name = f.name()));
    }

    if let Some(optional) = e.rust_object().optional() {
        s.wln(format!("{name},", name = optional.name()));
    }

    s.dec_indent();
    s.wln("})");
}
