use crate::parser::types::array::{ArraySize, ArrayType};
use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::ty::Type;
use crate::rust_printer::structs::test_case_string;
use crate::rust_printer::structs::test_case_string::members::{
    print_if_statement_enum, print_if_statement_flag,
};
use crate::rust_printer::structs::test_case_string::{wln, wlna};
use crate::rust_printer::writer::Writer;
use crate::rust_printer::DefinerType;

pub(crate) fn print_bytes(s: &mut Writer, e: &Container) {
    s.newline();

    if matches!(
        e.container_type(),
        ContainerType::SMsg(_) | ContainerType::CMsg(_)
    ) {
        let additional_size = match e.container_type() {
            ContainerType::CMsg(_) => 4,
            ContainerType::SMsg(_) => 2,
            _ => 0,
        };

        if let Some(size) = e.sizes().is_constant() {
            let size = size + additional_size;
            s.wln(format!("let [a, b] = {size}_u16.to_be_bytes();"));
        } else {
            s.wln(format!(
                "let [a, b] = (u16::try_from(self.size() + {additional_size}).unwrap()).to_be_bytes();"
            ));
        }

        test_case_string::wln(s, "    {a:#04X}, {b:#04X}, /* size */");
        let opcode = e.opcode();
        match e.container_type() {
            ContainerType::SMsg(_) => {
                s.wln(format!("let [a, b] = {opcode}_u16.to_le_bytes();"));
                test_case_string::wln(s, "    {a:#04X}, {b:#04X}, /* opcode */");
            }
            ContainerType::CMsg(_) => {
                s.wln(format!("let [a, b, c, d] = {opcode}_u32.to_le_bytes();"));
                test_case_string::wln(
                    s,
                    "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */",
                );
            }
            _ => {}
        }
    }

    print_bytes_members(s, e);

    s.newline();
}

fn print_bytes_members(s: &mut Writer, e: &Container) {
    s.wln("let mut bytes: Vec<u8> = Vec::new();");
    s.wln("self.write_into_vec(&mut bytes).unwrap();");
    s.wln("let mut bytes = bytes.into_iter();");

    s.newline();

    if matches!(
        e.container_type(),
        ContainerType::CLogin(_) | ContainerType::SLogin(_)
    ) {
        wlna(s, "    {:#04X}, /* opcode */ ", "bytes.next().unwrap()");
    }

    for m in e.members() {
        print_bytes_struct_member(s, e, m, "self.", "    ");
    }

    s.newline();
}

fn print_bytes_struct_member(
    s: &mut Writer,
    e: &Container,
    m: &StructMember,
    variable_prefix: &str,
    prefix: &str,
) {
    match m {
        StructMember::Definition(d) => {
            print_bytes_definition(s, d, variable_prefix, prefix);
        }
        StructMember::IfStatement(statement) => match statement.definer_type() {
            DefinerType::Enum => {
                print_if_statement_enum(
                    s,
                    e,
                    statement,
                    variable_prefix,
                    prefix,
                    print_bytes_struct_member,
                );
            }
            DefinerType::Flag => {
                print_if_statement_flag(
                    s,
                    e,
                    statement,
                    variable_prefix,
                    prefix,
                    print_bytes_struct_member,
                );
            }
        },
        StructMember::OptionalStatement(optional) => {
            let name = optional.name();
            s.body(format!("if let Some({name}) = &self.{name}"), |s| {
                for m in optional.members() {
                    let variable_prefix = format!("{name}.");
                    print_bytes_struct_member(s, e, m, &variable_prefix, prefix);
                }
            });
        }
    }
}

fn print_bytes_definition(
    s: &mut Writer,
    d: &StructMemberDefinition,
    variable_prefix: &str,
    prefix: &str,
) {
    let name = d.name();
    let var_name = format!("{variable_prefix}{name}");
    let ty_name = d.ty().str();

    let prefix_text = format!("\"{prefix}\"");

    match d.ty() {
        Type::Guid
        | Type::Bool(_)
        | Type::Integer(_)
        | Type::Level
        | Type::Level16
        | Type::IpAddress
        | Type::FloatingPoint
        | Type::DateTime
        | Type::Seconds
        | Type::Milliseconds
        | Type::Level32
        | Type::Gold => {
            let size = d.ty().sizes().is_constant().unwrap();
            s.wln(format!(
                "crate::util::write_bytes(&mut s, &mut bytes, {size}, \"{name}\", {prefix_text});"
            ));
        }

        Type::Population => {
            s.wln(format!(
                "crate::util::write_bytes(&mut s, &mut bytes, 4, \"{name}\", {prefix_text});"
            ));
        }

        Type::Enum { e, upcast } | Type::Flag { e, upcast } => {
            let size = if let Some(upcast) = upcast {
                upcast.size()
            } else {
                e.ty().size()
            };

            s.wln(format!(
                "crate::util::write_bytes(&mut s, &mut bytes, {size}, \"{name}\", {prefix_text});"
            ));
        }

        Type::PackedGuid => {
            s.wln(format!(
                "crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&{var_name}), \"{name}\", {prefix_text});"
            ));
        }

        Type::String | Type::CString => {
            s.wln(format!(
                "crate::util::write_bytes(&mut s, &mut bytes, {var_name}.len() + 1, \"{name}\", {prefix_text});"
            ));
        }

        Type::SizedCString => {
            s.wln(format!(
                "crate::util::write_bytes(&mut s, &mut bytes, {var_name}.len() + 5, \"{name}\", {prefix_text});"
            ));
        }

        Type::Struct { e } => {
            wln(s, format!("    /* {name}: {ty_name} start */"));
            let variable_prefix = format!("{var_name}.");
            let prefix = format!("{prefix}    ");

            for m in e.members() {
                print_bytes_struct_member(s, e, m, &variable_prefix, &prefix);
            }
            wln(s, format!("    /* {name}: {ty_name} end */"));
        }

        Type::Array(array) => {
            if array.is_byte_array() {
                s.wln(format!(
                    "crate::util::write_bytes(&mut s, &mut bytes, {var_name}.len(), \"{name}\", {prefix_text});"
                ));
            } else {
                let can_be_empty = { !matches!(array.size(), ArraySize::Fixed(_)) };
                if can_be_empty {
                    s.open_curly(format!("if !{var_name}.is_empty()"));
                }

                wln(s, format!("    /* {name}: {ty_name} start */"));

                let name_text = format!("&format!(\"{name} {{i}}\")");

                s.body(format!("for (i, v) in {var_name}.iter().enumerate()"), |s| {
                    match array.ty() {
                        ArrayType::Integer(i) => {
                            let size = i.size();

                            s.wln(format!(
                                "crate::util::write_bytes(&mut s, &mut bytes, {size}, {name_text}, {prefix_text});"
                            ));
                        }
                        ArrayType::Guid => {
                            s.wln(format!(
                                "crate::util::write_bytes(&mut s, &mut bytes, 8, {name_text}, {prefix_text});"
                            ));
                        }
                        ArrayType::PackedGuid => {
                            s.wln(format!(
                                "crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(v), {name_text}, {prefix_text});"
                            ));
                        }
                        ArrayType::CString => {
                            s.wln(format!(
                                "crate::util::write_bytes(&mut s, &mut bytes, v.len() + 1, {name_text}, {prefix_text});"
                            ));
                        }
                        ArrayType::Struct(e) => {
                            wln(s, format!("    /* {name}: {ty_name} {{i}} start */"));

                            let variable_prefix = "v.";

                            let prefix = format!("{prefix}    ");
                            for m in e.members() {
                                print_bytes_struct_member(s, e, m, variable_prefix, &prefix);
                            }

                            wln(s, format!("    /* {name}: {ty_name} {{i}} end */"));
                        }
                    }

                });

                wln(s, format!("    /* {name}: {ty_name} end */"));

                if can_be_empty {
                    s.closing_curly(); // if !is_empty
                }
            }
        }

        Type::MonsterMoveSplines => {
            s.wln(format!("crate::util::write_bytes(&mut s, &mut bytes, 4, \"{name}_length\", {prefix_text});"));
            s.open_curly(format!("if !{var_name}.is_empty()"));
            wln(s, format!("    /* {name}: {ty_name} start */"));

            s.wln(format!("let mut v = {var_name}.iter();"));
            s.wln("let _ = v.next().unwrap();");
            s.wln(format!(
                "crate::util::write_bytes(&mut s, &mut bytes, 4, \"{name}_x\", {prefix_text});"
            ));
            s.wln(format!(
                "crate::util::write_bytes(&mut s, &mut bytes, 4, \"{name}_y\", {prefix_text});"
            ));
            s.wln(format!(
                "crate::util::write_bytes(&mut s, &mut bytes, 4, \"{name}_z\", {prefix_text});"
            ));

            s.bodyn(format!("for v in v"), |s| {
                s.wln(format!("crate::util::write_bytes(&mut s, &mut bytes, 4, \"{name}_packed\", {prefix_text});"));
            });

            wln(s, format!("    /* {name}: {ty_name} end */"));
            s.closing_curly(); // if !is empty
        }

        Type::UpdateMask { .. }
        | Type::AuraMask
        | Type::AchievementDoneArray
        | Type::AchievementInProgressArray
        | Type::EnchantMask
        | Type::InspectTalentGearMask
        | Type::VariableItemRandomProperty
        | Type::AddonArray
        | Type::NamedGuid => {
            s.wln(format!(
                "panic!(\"unsupported type {} for variable '{name}'\");",
                d.ty().rust_str()
            ));
        }
    }
}
