use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::definer::Definer;
use crate::parser::types::if_statement::{Equation, IfStatement};
use crate::parser::types::struct_member::StructMember;
use crate::parser::types::ty::Type;
use crate::rust_printer::writer::Writer;
use crate::CONTAINER_SELF_SIZE_FIELD;

pub(crate) fn get_definer_wowm_definition(kind: &str, e: &Definer, prefix: &str) -> String {
    let mut s = Writer::with_prefix(prefix);
    s.body(
        &format!(
            "{kind} {name} : {ty}",
            kind = kind,
            name = e.name(),
            ty = e.ty().str(),
        ),
        |s| {
            for field in e.fields() {
                s.wln(format!(
                    "{name} = {val};",
                    name = field.name(),
                    val = field.value().original()
                ));
            }
        },
    );

    s.into_inner()
}

pub(crate) fn get_struct_wowm_definition(e: &Container, prefix: &str) -> String {
    let mut s = Writer::with_prefix(prefix);

    s.body(
        format!(
            "{kind} {name}{opcode}",
            kind = e.container_type().str(),
            name = e.name(),
            opcode = match e.container_type() {
                ContainerType::Struct => "".to_string(),
                ContainerType::CLogin(o) | ContainerType::SLogin(o) => format!(" = 0x{o:0>2X}"),
                ContainerType::Msg(o) | ContainerType::CMsg(o) | ContainerType::SMsg(o) =>
                    format!(" = 0x{o:0>4X}"),
            }
        ),
        |s| {
            if e.tags().unimplemented() {
                s.wln("unimplemented");
            } else {
                for field in e.members() {
                    print_members(s, field);
                }
            }
        },
    );

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
                "{upcast}{ty} {name}{constant}{size_field};",
                ty = d.ty().str(),
                name = d.name(),
                constant = match d.value() {
                    None => "".to_string(),
                    Some(c) => format!(" = {val}", val = c.original_string()),
                },
                size_field = if d.is_manual_size_field() {
                    format!(" = {CONTAINER_SELF_SIZE_FIELD}")
                } else {
                    String::new()
                }
            ));
        }
        StructMember::IfStatement(statement) => {
            print_wowm_if_statement(s, statement, "if", |s| {
                for f in statement.members() {
                    print_members(s, f);
                }
            });

            if !statement.else_ifs().is_empty() {
                for else_if in statement.else_ifs() {
                    print_wowm_if_statement(s, else_if, "else if", |s| {
                        for m in else_if.members() {
                            print_members(s, m);
                        }
                    });
                }
            }

            if !statement.else_members().is_empty() {
                s.body("else", |s| {
                    for f in statement.else_members() {
                        print_members(s, f);
                    }
                });
            }
        }
        StructMember::OptionalStatement(optional) => {
            s.body(format!("optional {name}", name = optional.name()), |s| {
                for m in optional.members() {
                    print_members(s, m);
                }
            });
        }
    }
}

fn print_wowm_if_statement(
    s: &mut Writer,
    statement: &IfStatement,
    condition: &str,
    f: impl Fn(&mut Writer),
) {
    let name = statement.variable_name();
    match statement.equation() {
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

    f(s);

    s.dec_indent();
    s.wln("}");
}
