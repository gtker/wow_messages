use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::definer::Definer;
use crate::parser::types::if_statement::{Equation, IfStatement};
use crate::parser::types::struct_member::StructMember;
use crate::parser::types::ty::Type;
use crate::rust_printer::Writer;
use crate::ENUM_SELF_VALUE_FIELD;

pub(crate) fn get_definer_wowm_definition(kind: &str, e: &Definer, prefix: &str) -> String {
    let mut s = Writer::with_prefix(prefix);
    s.wln(&format!(
        "{kind} {name} : {ty} {{",
        kind = kind,
        name = e.name(),
        ty = e.ty().str(),
    ));

    s.inc_indent();
    for field in e.fields() {
        s.wln(format!(
            "{name} = {val};",
            name = field.name(),
            val = field.value().original()
        ));
    }

    if let Some(f) = e.self_value() {
        s.wln(format!(
            "{name} = {self_value}",
            name = f.name(),
            self_value = ENUM_SELF_VALUE_FIELD
        ));
    }
    s.dec_indent();
    s.wln("}");

    s.into_inner()
}

pub(crate) fn get_struct_wowm_definition(e: &Container, prefix: &str) -> String {
    let mut s = Writer::with_prefix(prefix);

    s.wln(format!(
        "{kind} {name}{opcode} {{",
        kind = e.container_type().str(),
        name = e.name(),
        opcode = match e.container_type() {
            ContainerType::Struct => "".to_string(),
            ContainerType::CLogin(o) | ContainerType::SLogin(o) => format!(" = 0x{o:0>2X}"),
            ContainerType::Msg(o) | ContainerType::CMsg(o) | ContainerType::SMsg(o) =>
                format!(" = 0x{o:0>4X}"),
        }
    ));

    s.inc_indent();

    if e.tags().unimplemented() {
        s.wln("unimplemented");
    } else {
        for field in e.members() {
            print_members(&mut s, field);
        }
    }

    s.dec_indent();
    s.wln("}");

    s.into_inner()
}

fn print_members(s: &mut Writer, field: &StructMember) {
    match field {
        StructMember::Definition(d) => {
            let upcast = match d.ty() {
                Type::Enum { upcast, .. } | Type::Flag { upcast, .. } => {
                    if let Some(upcast) = upcast {
                        format!("({})", upcast.rust_str())
                    } else {
                        "".to_string()
                    }
                }
                _ => "".to_string(),
            };

            s.wln(format!(
                "{upcast}{ty} {name}{constant};",
                ty = d.ty().str(),
                name = d.name(),
                constant = match d.value() {
                    None => "".to_string(),
                    Some(c) => format!(" = {val}", val = c.original_string()),
                }
            ));
        }
        StructMember::IfStatement(statement) => {
            print_wowm_if_statement(s, statement, "if");

            for f in statement.members() {
                print_members(s, f);
            }

            s.dec_indent();
            s.wln("}");

            if !statement.else_ifs().is_empty() {
                for else_if in statement.else_ifs() {
                    print_wowm_if_statement(s, else_if, "else if");

                    for m in else_if.members() {
                        print_members(s, m);
                    }

                    s.dec_indent();
                    s.wln("}");
                }
            }

            if !statement.else_members().is_empty() {
                s.wln("else {");
                s.inc_indent();

                for f in statement.else_members() {
                    print_members(s, f);
                }

                s.dec_indent();
                s.wln("}");
            }
        }
        StructMember::OptionalStatement(optional) => {
            s.wln(format!("optional {name} {{", name = optional.name()));
            s.inc_indent();

            for m in optional.members() {
                print_members(s, m);
            }

            s.dec_indent();
            s.wln("}");
        }
    }
}

fn print_wowm_if_statement(s: &mut Writer, statement: &IfStatement, condition: &str) {
    let name = statement.name();
    match statement.conditional().equation() {
        Equation::Equals { values: value } => {
            for (i, v) in value.iter().enumerate() {
                if i == 0 {
                    s.w(format!("{condition} ({name} == {v}"));
                    s.inc_indent();
                } else {
                    s.w(format!("|| {name} == {v}"));
                }

                if i == value.len() - 1 {
                    s.wln_no_indent(") {");
                } else {
                    s.wln_no_indent("");
                }
            }
        }
        Equation::BitwiseAnd { values: value } => {
            for (i, v) in value.iter().enumerate() {
                if i == 0 {
                    s.w(format!("{condition} ({name} & {v}"));
                    s.inc_indent();
                } else {
                    s.w(format!("|| {name} & {v}"));
                }

                if i == value.len() - 1 {
                    s.wln_no_indent(") {");
                } else {
                    s.wln_no_indent("");
                }
            }
        }
        Equation::NotEquals { value } => {
            s.wln(format!("{condition} ({name} != {value}) {{"));
            s.inc_indent();
        }
    }
}
