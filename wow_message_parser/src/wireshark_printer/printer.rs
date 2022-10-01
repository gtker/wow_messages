use crate::container::{Equation, IfStatement, StructMember, StructMemberDefinition};
use crate::parser::enumerator::Definer;
use crate::parser::types::ty::Type;
use crate::parser::types::{ArraySize, ArrayType, IntegerType};
use crate::rust_printer::Writer;
use crate::wireshark_printer::types::{WiresharkMember, WiresharkObject};
use crate::wireshark_printer::{
    clean_opcode_name, enum_name, enum_strings, enumerator_name, is_client_name, is_server_name,
    name_to_hf, pretty_name, server_to_client_name, ui_name,
};
use crate::{Container, Object, Objects, Tags};

pub fn print_parser(o: &Objects, w: &WiresharkObject) -> (Writer, Writer) {
    let mut s = Writer::new("");
    s.inc_indent();

    let mut variables = Vec::new();

    s.open_curly("switch (opcode)");
    for e in o.wireshark_messages() {
        if e.empty_body() || is_client_name(e.name()) {
            continue;
        }

        s.wln(format!("case {}:", clean_opcode_name(e.name())));
        s.inc_indent();
        if is_server_name(e.name()) {
            s.open_curly("if (WOWW_SERVER_TO_CLIENT)");

            print_message(&mut s, e, w, o, &mut variables);

            s.closing_curly(); // if (WOWW_SERVER_TO_CLIENT)

            s.open_curly("else");

            let v = o.wireshark_messages();
            let e = v
                .iter()
                .find(|a| a.name() == server_to_client_name(e.name()))
                .unwrap();
            print_message(&mut s, e, w, o, &mut variables);

            s.closing_curly(); // else
        } else {
            print_message(&mut s, e, w, o, &mut variables);
        }

        s.wln("break;");
        s.dec_indent();
    }

    s.newline();

    s.wln("default:");
    s.inc_indent();
    s.wln("break;");
    s.closing_curly();

    (s, print_variables(variables))
}

fn print_variables(mut v: Vec<String>) -> Writer {
    let mut s = Writer::new("");

    v.sort();
    v.dedup();

    for name in v {
        s.wln(format!("guint32 {} = 0;", name));
    }

    s
}

fn print_message(
    s: &mut Writer,
    e: &Container,
    w: &WiresharkObject,
    o: &Objects,
    variables: &mut Vec<String>,
) {
    for m in e.fields() {
        if !print_member(s, m, e, w, e.tags(), o, variables) {
            return;
        }
    }
}

fn print_member(
    s: &mut Writer,
    m: &StructMember,
    e: &Container,
    wo: &WiresharkObject,
    tags: &Tags,
    o: &Objects,
    variables: &mut Vec<String>,
) -> bool {
    match m {
        StructMember::Definition(d) => {
            let w = wo.get(&name_to_hf(d.name(), d.ty(), tags, o));
            if !print_definition(s, e.name(), d, w, o, tags, wo, variables) {
                return false;
            }
        }
        StructMember::IfStatement(statement) => {
            print_if_statement(s, e, wo, tags, o, statement, variables);
        }
        StructMember::OptionalStatement(_) => {}
    }

    true
}

fn print_if_statement(
    s: &mut Writer,
    e: &Container,
    wo: &WiresharkObject,
    tags: &Tags,
    o: &Objects,
    statement: &IfStatement,
    variables: &mut Vec<String>,
) -> bool {
    let name = statement.name();

    s.w("if (");

    for (i, eq) in statement.get_conditional().equations().iter().enumerate() {
        let (op, value) = match eq {
            Equation::Equals { value } => ("==", value),
            Equation::NotEquals { value } => ("!=", value),
            Equation::BitwiseAnd { value } => ("&", value),
        };
        let enumerator = enumerator_name(&statement.original_ty().rust_str(), value);
        if i != 0 {
            s.newline();
            s.w_no_indent(" || ");
        }

        s.w_no_indent(format!(
            "{name} {op} {enumerator}",
            name = name,
            op = op,
            enumerator = enumerator
        ));
    }

    s.wln_no_indent(") {");
    s.inc_indent();

    for m in statement.members() {
        print_member(s, m, e, wo, tags, o, variables);
    }

    s.closing_curly();

    for elseif in statement.else_ifs() {
        s.w("else if (");

        for (i, eq) in elseif.get_conditional().equations().iter().enumerate() {
            let (op, value) = match eq {
                Equation::Equals { value } => ("==", value),
                Equation::NotEquals { value } => ("!=", value),
                Equation::BitwiseAnd { value } => ("&", value),
            };

            let enumerator = enumerator_name(&elseif.original_ty().rust_str(), value);
            if i != 0 {
                s.newline();
                s.w(" || ");
            }

            s.w_no_indent(format!(
                "{name} {op} {enumerator}",
                name = name,
                op = op,
                enumerator = enumerator
            ));
        }

        s.wln_no_indent(") {");
        s.inc_indent();

        for m in elseif.members() {
            print_member(s, m, e, wo, tags, o, variables);
        }

        s.closing_curly();
    }

    if !statement.else_members().is_empty() {
        s.open_curly("else");

        for m in statement.else_members() {
            print_member(s, m, e, wo, tags, o, variables);
        }

        s.closing_curly(); // else
    }

    false
}

fn print_definition(
    s: &mut Writer,
    container_name: &str,
    d: &StructMemberDefinition,
    w: Option<&WiresharkMember>,
    o: &Objects,
    tags: &Tags,
    wo: &WiresharkObject,
    variables: &mut Vec<String>,
) -> bool {
    match d.ty() {
        Type::Integer(i) => {
            let name = w.unwrap().name();
            let len = i.size();
            let enc = i.wireshark_endian_str();

            if d.used_as_size_in().is_some() {
                variables.push(d.name().to_string());
                s.wln(format!(
                    "ptvcursor_add_ret_uint(ptv, {name}, {len}, {enc}, &{var_name});",
                    name = name,
                    len = len,
                    enc = enc,
                    var_name = d.name(),
                ));
            } else {
                s.wln(format!(
                    "ptvcursor_add(ptv, {hf}, {len}, {enc});",
                    hf = name,
                    len = len,
                    enc = enc,
                ));
            }
            true
        }
        Type::Guid => {
            let name = w.unwrap().name();
            s.wln(format!(
                "ptvcursor_add(ptv, {hf}, 8, ENC_LITTLE_ENDIAN);",
                hf = name,
            ));
            true
        }
        Type::Bool => {
            let name = w.unwrap().name();
            s.wln(format!("ptvcursor_add(ptv, {hf}, 1, ENC_NA);", hf = name,));
            true
        }
        Type::DateTime => {
            let name = w.unwrap().name();
            s.wln(format!(
                "ptvcursor_add(ptv, {hf}, 4, ENC_LITTLE_ENDIAN);",
                hf = name,
            ));
            true
        }
        Type::FloatingPoint(i) => {
            let name = w.unwrap().name();
            let len = i.size();
            let enc = i.wireshark_endian_str();

            s.wln(format!(
                "ptvcursor_add(ptv, {hf}, {len}, {enc});",
                hf = name,
                len = len,
                enc = enc,
            ));
            true
        }
        Type::CString => {
            print_cstring(s, w);
            true
        }
        Type::SizedCString => {
            let name = w.unwrap().name();

            s.wln(format!("add_sized_cstring(ptv, &{hf});", hf = name));
            true
        }
        Type::Identifier {
            s: identifier,
            upcast,
        } => {
            if !print_identifier(
                s,
                identifier,
                w,
                upcast,
                o,
                wo,
                tags,
                container_name,
                d.name(),
                variables,
            ) {
                return false;
            }

            true
        }
        Type::Array(array) => {
            let len = match array.size() {
                ArraySize::Fixed(v) => v.to_string(),
                ArraySize::Variable(v) => v,
                ArraySize::Endless => {
                    s.wln("len = offset_packet_end - ptvcursor_current_offset(ptv);");
                    "len".to_string()
                }
            };

            if array.is_byte_array() {
                s.wln(format!(
                    "ptvcursor_add(ptv, {hf}, {len}, ENC_NA);",
                    hf = w.unwrap().name()
                ));

                return true;
            }

            s.open_curly(format!("for (i = 0; i < {len}; ++i)"));

            match array.ty() {
                ArrayType::Integer(i) => {
                    let name = w.unwrap().name();
                    let len = i.size();
                    let enc = i.wireshark_endian_str();

                    s.wln(format!(
                        "ptvcursor_add(ptv, {hf}, {len}, {enc});",
                        hf = name,
                        len = len,
                        enc = enc,
                    ));
                }
                ArrayType::Guid => {
                    let name = w.unwrap().name();
                    s.wln(format!(
                        "ptvcursor_add(ptv, {hf}, 8, ENC_LITTLE_ENDIAN);",
                        hf = name,
                    ));
                }
                ArrayType::Complex(identifier) => {
                    if !print_identifier(
                        s,
                        identifier,
                        w,
                        &None,
                        o,
                        wo,
                        tags,
                        container_name,
                        d.name(),
                        variables,
                    ) {
                        s.closing_curly();
                        return false;
                    }
                }
                ArrayType::CString => {
                    print_cstring(s, w);
                }
                ArrayType::PackedGuid => {
                    s.wln("add_packed_guid(ptv, pinfo, tree);");
                }
            }

            s.closing_curly();

            true
        }
        Type::PackedGuid => {
            s.wln("add_packed_guid(ptv, pinfo, tree);");
            true
        }
        Type::UpdateMask | Type::AuraMask => false,
        Type::String { .. } => unreachable!(),
    }
}

fn print_cstring(s: &mut Writer, w: Option<&WiresharkMember>) {
    let name = w.unwrap().name();

    s.wln(format!("add_cstring(ptv, &{hf});", hf = name));
}

fn print_identifier(
    s: &mut Writer,
    identifier: &str,
    w: Option<&WiresharkMember>,
    upcast: &Option<IntegerType>,
    o: &Objects,
    wo: &WiresharkObject,
    tags: &Tags,
    container_name: &str,
    variable_name: &str,
    variables: &mut Vec<String>,
) -> bool {
    let e = o.get_object(identifier, tags);
    match e {
        Object::Container(e) => {
            s.wln(format!("ptvcursor_add_text_with_subtree(ptv, SUBTREE_UNDEFINED_LENGTH, ett_message, \"{}\");", e.name()));
            for m in e.fields() {
                if !print_member(s, m, &e, wo, e.tags(), o, variables) {
                    return false;
                }
            }
            s.wln("ptvcursor_pop_subtree(ptv);");

            true
        }
        Object::Enum(e) | Object::Flag(e) => {
            let (len, enc) = if let Some(upcast) = upcast {
                (upcast.size(), upcast.wireshark_endian_str())
            } else {
                (e.ty().size(), e.ty().wireshark_endian_str())
            };
            let name = w.unwrap().name();

            if e.used_in_if_in_object(container_name) {
                variables.push(variable_name.to_string());
                s.wln(format!(
                    "ptvcursor_add_ret_uint(ptv, {name}, {len}, {enc}, &{var_name});",
                    name = name,
                    len = len,
                    enc = enc,
                    var_name = variable_name,
                ));
            } else {
                s.wln(format!(
                    "ptvcursor_add(ptv, {hf}, {len}, {enc});",
                    hf = name,
                    len = len,
                    enc = enc,
                ));
            }

            true
        }
    }
}

pub fn print_register_info(w: &WiresharkObject) -> Writer {
    let mut s = Writer::new("");
    s.inc_indent();

    s.inc_indent();
    for m in w.members() {
        s.wln(format!("{{ &{},", m.name()));
        s.inc_indent();

        s.wln(format!(
            "{{ \"{pretty_name}\", \"woww.{ui_name}\",",
            pretty_name = pretty_name(m.name_without_hf_woww()),
            ui_name = ui_name(m.name_without_hf_woww()),
        ));
        s.inc_indent();

        let enum_strings = if let Some(e) = m.has_enum_strings() {
            format!("VALS({})", enum_strings(e.name()))
        } else {
            "NULL".to_string()
        };
        s.wln(format!(
            "{ty}, {base}, {enum_strings}, 0,",
            ty = m.ty().wireshark_str(),
            base = m.ty().wireshark_base(),
            enum_strings = enum_strings,
        ));

        s.wln("NULL, HFILL");

        s.closing_curly(); // { pretty_name, "ui_name"
        s.closing_curly_with(","); // { &hf_woww
    }
    s.dec_indent();

    s
}

pub fn print_int_declarations(w: &WiresharkObject) -> Writer {
    let mut s = Writer::new("");

    for m in w.members() {
        s.wln(format!("static int {} = -1;", m.name()));
    }

    s
}

pub fn print_enums(w: &WiresharkObject) -> Writer {
    let mut s = Writer::new("");

    for e in w.enums() {
        print_enum(&mut s, e);
    }
    s.newline();

    for e in w.flags() {
        print_flag(&mut s, e);
    }

    s
}

fn print_typedef(s: &mut Writer, e: &Definer) {
    let hex_width = e.hex_digit_width();

    s.body_closing_with(
        "typedef enum",
        |s| {
            for enumerator in e.fields() {
                s.wln(format!(
                    "{enumerator_name} = {value:#0hex_width$X},",
                    enumerator_name = enumerator_name(e.name(), enumerator.name()),
                    value = enumerator.value().int(),
                    hex_width = hex_width,
                ))
            }
        },
        format!(" {};", enum_name(e.name())),
    );
}

fn print_enum(s: &mut Writer, e: &Definer) {
    print_typedef(s, e);

    s.body_closing_with(
        format!("static const value_string {}[] = ", enum_strings(e.name())),
        |s| {
            for enumerator in e.fields() {
                s.wln(format!(
                    "{{ {enumerator_name}, \"{pretty_name}\" }},",
                    enumerator_name = enumerator_name(e.name(), enumerator.name()),
                    pretty_name = pretty_name(enumerator.name()),
                ));
            }

            s.wln("{ 0, NULL }");
        },
        ";",
    );
    s.newline();
}

fn print_flag(s: &mut Writer, e: &Definer) {
    print_typedef(s, e);
    s.newline();
}
