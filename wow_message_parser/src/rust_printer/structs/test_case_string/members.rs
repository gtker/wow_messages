use crate::parser::types::array::ArrayType;
use crate::parser::types::container::Container;
use crate::parser::types::if_statement::{Equation, IfStatement};
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::ty::Type;
use crate::rust_printer::structs::test_case_string;
use crate::rust_printer::writer::Writer;
use crate::rust_printer::DefinerType;

pub(crate) fn print_members(s: &mut Writer, e: &Container, variable_prefix: &str, prefix: &str) {
    s.wln("// Members");

    for m in e.members() {
        print_struct_member(s, e, m, variable_prefix, prefix);
    }

    s.newline();
}

fn print_struct_member(
    s: &mut Writer,
    e: &Container,
    m: &StructMember,
    variable_prefix: &str,
    prefix: &str,
) {
    match m {
        StructMember::Definition(d) => {
            print_member_definition(s, d, variable_prefix, prefix);
        }
        StructMember::IfStatement(statement) => match statement.definer_type() {
            DefinerType::Enum => {
                print_if_statement_enum(
                    s,
                    e,
                    statement,
                    variable_prefix,
                    prefix,
                    print_struct_member,
                );
            }
            DefinerType::Flag => {
                print_if_statement_flag(
                    s,
                    e,
                    statement,
                    variable_prefix,
                    prefix,
                    print_struct_member,
                );
            }
        },
        StructMember::OptionalStatement(optional) => {
            let name = optional.name();
            s.body(format!("if let Some({name}) = &self.{name}"), |s| {
                for m in optional.members() {
                    let variable_prefix = format!("{name}.");
                    print_struct_member(s, e, m, &variable_prefix, prefix);
                }
            });
        }
    }
}

pub(crate) fn print_if_statement_flag(
    s: &mut Writer,
    e: &Container,
    statement: &IfStatement,
    variable_prefix: &str,
    prefix: &str,
    print_function: impl Fn(&mut Writer, &Container, &StructMember, &str, &str),
) {
    s.open_curly(format!(
        "if let Some(if_statement) = &{variable_prefix}{variable}.get_{variant}()",
        variable = statement.name(),
        variant = &statement.flag_get_enumerator().to_lowercase(),
    ));
    if statement.else_ifs().is_empty() {
        let variable_prefix = "if_statement.";
        for m in statement.members() {
            print_function(s, e, m, variable_prefix, prefix);
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
            let import_path = e.get_import_path_from_world();

            s.open_curly(format!(
                "{import_path}::{ty}::{enumerator}",
                ty = rd.ty_name(),
                enumerator = enumerator.rust_name(),
            ));

            for m in enumerator.members_in_struct() {
                s.wln(format!("{name},", name = m.name()));
            }
            s.closing_curly_with(" => {"); // Self::enumerator
            s.inc_indent();

            let variable_prefix = "";
            for m in enumerator.original_fields() {
                print_function(s, e, m, variable_prefix, prefix);
            }

            s.closing_curly(); // Enumerator body
        }

        s.closing_curly(); // match self
    }

    s.closing_curly_newline(); // if let Some(s)
}

pub(crate) fn print_if_statement_enum(
    s: &mut Writer,
    e: &Container,
    statement: &IfStatement,
    variable_prefix: &str,
    prefix: &str,
    print_function: impl Fn(&mut Writer, &Container, &StructMember, &str, &str),
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

        let import_path = e.get_import_path_from_world();

        s.open_curly(format!(
            "{import_path}::{new_enum}::{variant}",
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
                print_function(s, e, m, "", prefix);
            }
        }

        s.closing_curly(); // enum::enumerator
    }

    if unused_enumerators {
        s.wln("_ => {}");
    }

    s.closing_curly_newline(); // match
}

fn print_member_definition(
    s: &mut Writer,
    d: &StructMemberDefinition,
    variable_prefix: &str,
    prefix: &str,
) {
    if d.value().is_some() || d.is_manual_size_field() {
        return;
    }

    let name = d.name();
    let var_name = if let Some(size) = d.used_as_size_in() {
        format!("{variable_prefix}{size}.len()")
    } else {
        format!("{variable_prefix}{name}")
    };

    match d.ty() {
        Type::Integer(_) => {
            test_case_string::wlna(s, format!("{prefix}{name} = {{}};"), var_name);
        }
        Type::Bool(_) => {
            let extra = if variable_prefix.is_empty() { "*" } else { "" };

            test_case_string::wlna(
                s,
                format!("{prefix}{name} = {{}};"),
                format!("if {extra}{var_name} {{ \"TRUE\" }} else {{ \"FALSE\" }}"),
            );
        }
        Type::Guid | Type::PackedGuid => {
            test_case_string::wlna(
                s,
                format!("{prefix}{name} = {{}};"),
                format!("{var_name}.guid()"),
            );
        }
        Type::Seconds => {
            test_case_string::wlna(
                s,
                format!("{prefix}{name} = {{}};"),
                format!("{var_name}.as_secs()"),
            );
        }
        Type::Milliseconds => {
            test_case_string::wlna(
                s,
                format!("{prefix}{name} = {{}};"),
                format!("{var_name}.as_millis()"),
            );
        }
        Type::Gold | Type::Level | Type::Level16 | Type::Level32 | Type::DateTime => {
            test_case_string::wlna(
                s,
                format!("{prefix}{name} = {{}};"),
                format!("{var_name}.as_int()"),
            );
        }
        Type::Population => {
            test_case_string::wlna(s, format!("{prefix}{name} = {{}};"), format!("if {var_name}.as_int().to_string().contains(\'.\') {{ {var_name}.as_int().to_string() }} else {{ format!(\"{{}}.0\", {var_name}.as_int()) }}"));
        }
        Type::String | Type::CString | Type::SizedCString => {
            test_case_string::wlna(s, format!("{prefix}{name} = \\\"{{}}\\\";"), var_name);
        }
        Type::FloatingPoint => {
            test_case_string::wlna(s, format!("{prefix}{name} = {{}};"), format!("if {var_name}.to_string().contains(\'.\') {{ {var_name}.to_string() }} else {{ format!(\"{{}}.0\", {var_name}) }}"));
        }
        Type::IpAddress => {
            test_case_string::wlna(
                s,
                format!("{prefix}{name} = {{:#08X}};"),
                format!("u32::from_be_bytes({var_name}.octets())"),
            );
        }
        Type::Enum { e, upcast } => {
            let ty = e.name();
            let extra = if !d.used_in_if() {
                format!("{var_name}.as_test_case_value()")
            } else {
                let extra = if upcast.is_some() {
                    format!("as {}", e.ty().rust_str())
                } else {
                    "".to_string()
                };
                format!("{ty}::try_from({var_name}.as_int(){extra}).unwrap().as_test_case_value()")
            };

            test_case_string::wlna(s, format!("{prefix}{name} = {{}};"), extra);
        }
        Type::Flag { e, upcast } => {
            let ty = e.name();
            let extra = if !d.used_in_if() {
                format!("{var_name}.as_test_case_value()")
            } else {
                let extra = if upcast.is_some() {
                    format!("as {}", e.ty().rust_str())
                } else {
                    "".to_string()
                };
                format!("{ty}::new({var_name}.as_int(){extra}).as_test_case_value()")
            };

            test_case_string::wlna(s, format!("{prefix}{name} = {{}};"), extra);
        }
        Type::Struct { e } => {
            let ty_name = e.name();
            s.wln(format!("// {name}: {ty_name}"));

            test_case_string::wln(s, format!("{prefix}{name} = {{{{"));

            let variable_prefix = format!("{var_name}.");

            let prefix = format!("{prefix}    ");
            print_members(s, e, &variable_prefix, &prefix);

            test_case_string::wln(s, "    }};");
        }

        Type::Array(array) => {
            s.wln(format!("writeln!(s, \"{prefix}{name} = [\").unwrap();"));

            s.body(format!("for v in {var_name}.as_slice()"), |s| {
                match array.ty() {
                    ArrayType::Integer(_) => {
                        s.wln("write!(s, \"{v:#04X}, \").unwrap();");
                    }
                    ArrayType::Guid | ArrayType::PackedGuid => {
                        s.wln("write!(s, \"{v:#08X}, \").unwrap();");
                    }
                    ArrayType::CString => {
                        s.wln("write!(s, \"\\\"{v}\\\", \").unwrap();");
                    }
                    ArrayType::Struct(e) => {
                        let prefix = format!("{prefix}    ");
                        test_case_string::wln(s, format!("{prefix}{{{{"));

                        let variable_prefix = "v.";

                        let new_prefix = format!("{prefix}    ");
                        print_members(s, e, variable_prefix, &new_prefix);

                        test_case_string::wln(s, format!("{prefix}}}}},"));
                    }
                }
            });

            test_case_string::wln(s, format!("{prefix}];"));
        }

        Type::UpdateMask { .. }
        | Type::MonsterMoveSplines
        | Type::AuraMask
        | Type::AchievementDoneArray
        | Type::AchievementInProgressArray
        | Type::EnchantMask
        | Type::InspectTalentGearMask
        | Type::VariableItemRandomProperty
        | Type::AddonArray
        | Type::NamedGuid => {
            s.wln(format!(
                "panic!(\"unsupported type for test case printing: '{}' for variable '{name}'\");",
                d.ty().str()
            ));
        }
    }
}
