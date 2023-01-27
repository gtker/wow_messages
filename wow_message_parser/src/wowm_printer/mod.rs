use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::definer::Definer;
use crate::parser::types::if_statement::{Equation, IfStatement};
use crate::parser::types::struct_member::StructMember;
use crate::parser::types::ty::Type;
use crate::rust_printer::Writer;
use crate::ENUM_SELF_VALUE_FIELD;
use std::fmt::Write;

struct WowmWriter {
    inner: String,
    prefix: String,
    indentation: u8,
}

impl WowmWriter {
    pub(crate) fn new(prefix: &str) -> Self {
        Self {
            inner: String::with_capacity(4000),
            prefix: prefix.to_string(),
            indentation: 0,
        }
    }

    pub(crate) fn w(&mut self, s: impl AsRef<str>) {
        self.inner.write_str(self.prefix.as_str()).unwrap();

        for _ in 0..self.indentation {
            self.inner.write_str(Writer::INDENTATION).unwrap();
        }

        self.inner.write_str(s.as_ref()).unwrap();
    }

    pub(crate) fn newline(&mut self) {
        self.inner.write_str("\n").unwrap();
    }

    pub(crate) fn wln(&mut self, s: impl AsRef<str>) {
        self.w(s);
        self.newline();
    }

    pub(crate) fn inc(&mut self) {
        assert_ne!(self.indentation, 0xFF);

        self.indentation += 1;
    }

    pub(crate) fn dec(&mut self) {
        assert_ne!(self.indentation, 0);

        self.indentation -= 1;
    }

    pub(crate) fn wln_no_indent(&mut self, s: impl AsRef<str>) {
        self.inner.write_str(s.as_ref()).unwrap();
        self.newline();
    }
}

pub(crate) fn get_definer_wowm_definition(kind: &str, e: &Definer, prefix: &str) -> String {
    let mut s = WowmWriter::new(prefix);
    s.wln(&format!(
        "{kind} {name} : {ty} {{",
        kind = kind,
        name = e.name(),
        ty = e.ty().str(),
    ));

    s.inc();
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
    s.dec();
    s.wln("}");

    s.inner
}

pub(crate) fn get_struct_wowm_definition(e: &Container, prefix: &str) -> String {
    let mut s = WowmWriter::new(prefix);

    s.wln(format!(
        "{kind} {name}{opcode} {{",
        kind = e.container_type().str(),
        name = e.name(),
        opcode = match e.container_type() {
            ContainerType::Struct => "".to_string(),
            ContainerType::CLogin(o) | ContainerType::SLogin(o) =>
                format!(" = 0x{o:0>2X}"),
            ContainerType::Msg(o) | ContainerType::CMsg(o) | ContainerType::SMsg(o) =>
                format!(" = 0x{o:0>4X}"),
        }
    ));

    s.inc();

    if e.tags().unimplemented() {
        s.wln("unimplemented");
    } else {
        for field in e.members() {
            print_members(&mut s, field);
        }
    }

    s.dec();
    s.wln("}");

    s.inner
}

fn print_members(s: &mut WowmWriter, field: &StructMember) {
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

            s.dec();
            s.wln("}");

            if !statement.else_ifs().is_empty() {
                for else_if in statement.else_ifs() {
                    print_wowm_if_statement(s, else_if, "else if");

                    for m in else_if.members() {
                        print_members(s, m);
                    }

                    s.dec();
                    s.wln("}");
                }
            }

            if !statement.else_members().is_empty() {
                s.wln("else {");
                s.inc();

                for f in statement.else_members() {
                    print_members(s, f);
                }

                s.dec();
                s.wln("}");
            }
        }
        StructMember::OptionalStatement(optional) => {
            s.wln(format!("optional {name} {{", name = optional.name()));
            s.inc();

            for m in optional.members() {
                print_members(s, m);
            }

            s.dec();
            s.wln("}");
        }
    }
}

fn print_wowm_if_statement(s: &mut WowmWriter, statement: &IfStatement, condition: &str) {
    let equations = statement.conditional().equations();
    for (i, eq) in equations.iter().enumerate() {
        let name = statement.conditional().variable_name();
        let (op, cond) = match eq {
            Equation::Equals { value } => ("==", value),
            Equation::NotEquals { value } => ("!=", value),
            Equation::BitwiseAnd { value } => ("&", value),
        };

        if i == 0 {
            s.w(format!(
                "{condition} ({name} {op} {cond}"
            ));
            s.inc();
        } else {
            s.w(format!(
                "|| {name} {op} {cond}"
            ));
        }

        if i == equations.len() - 1 {
            s.wln_no_indent(") {");
        } else {
            s.wln_no_indent("");
        }
    }
}
