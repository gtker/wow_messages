use crate::parser::types::array::{ArraySize, ArrayType};
use crate::parser::types::definer::Definer;
use crate::parser::types::if_statement::{Equation, IfStatement};
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::ty::Type;
use crate::parser::types::IntegerType;
use crate::rust_printer::Writer;
use crate::wireshark_printer::types::{WiresharkMember, WiresharkObject, WiresharkType};
use crate::wireshark_printer::{
    clean_opcode_name, enum_name, enum_strings, enumerator_name, is_client_name, is_server_name,
    name_to_hf, pretty_name, server_to_client_name, ui_name,
};
use crate::{Container, ObjectTags, Objects};
use std::fmt::UpperHex;

pub(crate) fn print_parser(o: &Objects, w: &WiresharkObject) -> (Writer, Writer) {
    let mut s = Writer::at_indentation(1);

    let mut variables = Vec::new();

    s.open_curly("switch (header_opcode)");
    for e in o.wireshark_messages() {
        if e.empty_body() || is_client_name(e.name()) {
            continue;
        }

        s.wln(format!("case {}:", clean_opcode_name(e.name())));
        s.inc_indent();

        let inside_compressed_message = e.tags().compressed();

        if inside_compressed_message {
            s.wln("ptvcursor_add(ptv, hf_woww_decompressed_size, 4, ENC_LITTLE_ENDIAN);");
            s.wln("compressed_tvb = tvb_uncompress(ptvcursor_tvbuff(ptv), ptvcursor_current_offset(ptv), offset_packet_end - ptvcursor_current_offset(ptv));");
            s.open_curly("if (compressed_tvb != NULL)");

            s.wln("ptvcursor_t* old_ptv = ptv;");
            s.wln("ptv = ptvcursor_new(wmem_packet_scope(), tree, compressed_tvb, 0);");
        }

        if is_server_name(e.name()) {
            s.open_curly("if (WOWW_SERVER_TO_CLIENT)");

            print_message(&mut s, e, w, o, &mut variables, inside_compressed_message);

            s.closing_curly(); // if (WOWW_SERVER_TO_CLIENT)

            let v = o.wireshark_messages();
            if let Some(e) = v
                .iter()
                .find(|a| a.name() == server_to_client_name(e.name()))
            {
                s.open_curly("else");

                print_message(&mut s, e, w, o, &mut variables, inside_compressed_message);

                s.closing_curly(); // else
            }
        } else {
            print_message(&mut s, e, w, o, &mut variables, inside_compressed_message);
        }

        if inside_compressed_message {
            s.wln("ptvcursor_free(ptv);");
            s.wln("ptv = old_ptv;");
            s.wln("compressed_tvb = NULL;");
            s.closing_curly(); // if (compressed_tvb != NULL)
        }

        s.wln("break;");
        s.dec_indent();
    }

    s.newline();

    s.wln("default:");
    s.inc_indent();
    s.wln("break;");
    s.dec_indent();
    s.closing_curly();

    (s, print_variables(variables))
}

fn print_variables(mut v: Vec<String>) -> Writer {
    let mut s = Writer::new();

    v.sort();
    v.dedup();

    for name in v {
        s.wln(format!("    guint32 {name} = 0;"));
    }

    s
}

fn print_message(
    s: &mut Writer,
    e: &Container,
    w: &WiresharkObject,
    o: &Objects,
    variables: &mut Vec<String>,
    inside_compressed_message: bool,
) {
    for m in e.members() {
        print_member(
            s,
            m,
            e,
            w,
            e.tags(),
            o,
            variables,
            inside_compressed_message,
            0,
        );
    }
}

fn print_member(
    s: &mut Writer,
    m: &StructMember,
    e: &Container,
    wo: &WiresharkObject,
    tags: &ObjectTags,
    o: &Objects,
    variables: &mut Vec<String>,
    inside_compressed_message: bool,
    depth: i32,
) {
    match m {
        StructMember::Definition(d) => {
            let w = wo.get(&name_to_hf(d.name(), d.ty()));
            print_definition(
                s,
                e.name(),
                d,
                w,
                o,
                wo,
                variables,
                inside_compressed_message,
                depth,
            );
        }
        StructMember::IfStatement(statement) => {
            print_if_statement(
                s,
                e,
                wo,
                tags,
                o,
                statement,
                variables,
                inside_compressed_message,
                depth,
            );
        }
        StructMember::OptionalStatement(optional) => {
            s.wln("len = offset_packet_end - ptvcursor_current_offset(ptv);");
            s.open_curly("if (len > 0)");

            for m in optional.members() {
                print_member(
                    s,
                    m,
                    e,
                    wo,
                    tags,
                    o,
                    variables,
                    inside_compressed_message,
                    depth,
                );
            }

            s.closing_curly(); // if (len > 0)
        }
    }
}

fn print_if_statement(
    s: &mut Writer,
    e: &Container,
    wo: &WiresharkObject,
    tags: &ObjectTags,
    o: &Objects,
    statement: &IfStatement,
    variables: &mut Vec<String>,
    inside_compressed_message: bool,
    depth: i32,
) {
    let name = statement.name();

    s.w("if (");

    match statement.conditional().equation() {
        Equation::Equals { values: value } => {
            for (i, v) in value.iter().enumerate() {
                let enumerator = enumerator_name(&statement.original_ty().rust_str(), v);
                if i != 0 {
                    s.newline();
                    s.w(" || ");
                }

                s.w_no_indent(format!("{name} == {enumerator}"));
            }
        }
        Equation::BitwiseAnd { values: value } => {
            for (i, v) in value.iter().enumerate() {
                let enumerator = enumerator_name(&statement.original_ty().rust_str(), v);
                if i != 0 {
                    s.newline();
                    s.w(" || ");
                }

                s.w_no_indent(format!("{name} & {enumerator}"));
            }
        }
        Equation::NotEquals { value } => {
            let enumerator = enumerator_name(&statement.original_ty().rust_str(), value);
            s.w_no_indent(format!("{name} != {enumerator}"));
        }
    }

    s.wln_no_indent(") {");
    s.inc_indent();

    for m in statement.members() {
        print_member(
            s,
            m,
            e,
            wo,
            tags,
            o,
            variables,
            inside_compressed_message,
            depth,
        );
    }

    s.closing_curly();

    for elseif in statement.else_ifs() {
        s.w("else if (");

        match elseif.conditional().equation() {
            Equation::Equals { values: value } => {
                for (i, v) in value.iter().enumerate() {
                    let enumerator = enumerator_name(&statement.original_ty().rust_str(), v);
                    if i != 0 {
                        s.newline();
                        s.w(" || ");
                    }

                    s.w_no_indent(format!("{name} == {enumerator}"));
                }
            }
            Equation::BitwiseAnd { values: value } => {
                for (i, v) in value.iter().enumerate() {
                    let enumerator = enumerator_name(&statement.original_ty().rust_str(), v);
                    if i != 0 {
                        s.newline();
                        s.w(" || ");
                    }

                    s.w_no_indent(format!("{name} & {enumerator}"));
                }
            }
            Equation::NotEquals { value } => {
                s.w_no_indent(format!("{name} != {value}"));
            }
        }

        s.wln_no_indent(") {");
        s.inc_indent();

        for m in elseif.members() {
            print_member(
                s,
                m,
                e,
                wo,
                tags,
                o,
                variables,
                inside_compressed_message,
                depth,
            );
        }

        s.closing_curly();
    }

    if !statement.else_members().is_empty() {
        s.open_curly("else");

        for m in statement.else_members() {
            print_member(
                s,
                m,
                e,
                wo,
                tags,
                o,
                variables,
                inside_compressed_message,
                depth,
            );
        }

        s.closing_curly(); // else
    }
}

fn print_definition(
    s: &mut Writer,
    container_name: &str,
    d: &StructMemberDefinition,
    w: Option<&WiresharkMember>,
    o: &Objects,
    wo: &WiresharkObject,
    variables: &mut Vec<String>,
    inside_compressed_message: bool,
    depth: i32,
) {
    if d.tags().compressed().is_some() {
        s.wln("compressed_tvb = tvb_uncompress(ptvcursor_tvbuff(ptv), ptvcursor_current_offset(ptv), offset_packet_end - ptvcursor_current_offset(ptv));");
        s.open_curly("if (compressed_tvb != NULL)");

        s.wln("ptvcursor_t* old_ptv = ptv;");
        s.wln("ptv = ptvcursor_new(wmem_packet_scope(), tree, compressed_tvb, 0);");
    }

    match d.ty() {
        Type::Integer(i) => {
            let name = w.unwrap().name();
            let len = i.size();

            if d.used_as_size_in().is_some() {
                variables.push(d.name().to_string());
                s.wln(format!(
                    "ptvcursor_add_ret_uint(ptv, {name}, {len}, ENC_LITTLE_ENDIAN, &{var_name});",
                    name = name,
                    len = len,
                    var_name = d.name(),
                ));
            } else {
                s.wln(format!(
                    "ptvcursor_add(ptv, {name}, {len}, ENC_LITTLE_ENDIAN);",
                ));
            }
        }
        Type::Level16 => {
            let name = w.unwrap().name();
            s.wln(format!("ptvcursor_add(ptv, {name}, 2, ENC_LITTLE_ENDIAN);",));
        }
        Type::Seconds | Type::Milliseconds | Type::Gold | Type::Level32 => {
            let name = w.unwrap().name();
            s.wln(format!("ptvcursor_add(ptv, {name}, 4, ENC_LITTLE_ENDIAN);",));
        }
        Type::Level => {
            let name = w.unwrap().name();
            s.wln(format!("ptvcursor_add(ptv, {name}, 1, ENC_LITTLE_ENDIAN);",));
        }
        Type::Guid => {
            let name = w.unwrap().name();
            s.wln(format!("ptvcursor_add(ptv, {name}, 8, ENC_LITTLE_ENDIAN);",));
        }
        Type::Bool(i) => {
            let name = w.unwrap().name();

            let enc = if *i == IntegerType::U8 {
                "ENC_NA"
            } else {
                "ENC_LITTLE_ENDIAN"
            };

            s.wln(format!(
                "ptvcursor_add(ptv, {hf}, {len}, {enc});",
                hf = name,
                len = i.size()
            ));
        }
        Type::DateTime => {
            let name = w.unwrap().name();
            s.wln(format!("ptvcursor_add(ptv, {name}, 4, ENC_LITTLE_ENDIAN);",));
        }
        Type::FloatingPoint => {
            let name = w.unwrap().name();

            s.wln(format!("ptvcursor_add(ptv, {name}, 4, ENC_LITTLE_ENDIAN);",));
        }
        Type::CString => {
            print_cstring(s, w);
        }
        Type::SizedCString => {
            let name = w.unwrap().name();

            s.wln(format!("add_sized_cstring(ptv, &{name});"));
        }
        Type::Enum { e, upcast } | Type::Flag { e, upcast } => {
            print_definer(s, w, upcast, container_name, d.name(), variables, e);
        }
        Type::Struct { e } => {
            print_container(
                s,
                o,
                wo,
                variables,
                e,
                inside_compressed_message,
                None,
                depth,
            );
        }
        Type::Array(array) => {
            let len = match array.size() {
                ArraySize::Fixed(v) => v.to_string(),
                ArraySize::Variable(v) => v.name().to_string(),
                ArraySize::Endless => "len".to_string(),
            };

            if array.is_byte_array() {
                if matches!(array.size(), ArraySize::Endless) {
                    s.wln("len = offset_packet_end - ptvcursor_current_offset(ptv);");
                }

                s.wln(format!(
                    "ptvcursor_add(ptv, {hf}, {len}, ENC_NA);",
                    hf = w.unwrap().name()
                ));
            } else {
                let depth = depth + 1;

                let iteration_variable = match array.size() {
                    ArraySize::Fixed(_) | ArraySize::Variable(_) => {
                        s.open_curly(format!(
                            "for (guint32 i{depth} = 0; i{depth} < {len}; ++i{depth})"
                        ));

                        Some(format!("i{depth}").to_string())
                    }
                    ArraySize::Endless => {
                        let packet_end = if d.tags().compressed().is_some()
                            || inside_compressed_message
                        {
                            s.wln("gint compression_end = tvb_reported_length(compressed_tvb);");
                            "compression_end"
                        } else {
                            "offset_packet_end"
                        };
                        s.open_curly(format!(
                            "while (ptvcursor_current_offset(ptv) < {packet_end})"
                        ));

                        None
                    }
                };

                match array.ty() {
                    ArrayType::Integer(i) => {
                        let name = w.unwrap().name();
                        let len = i.size();

                        s.wln(format!(
                            "ptvcursor_add(ptv, {name}, {len}, ENC_LITTLE_ENDIAN);",
                        ));
                    }
                    ArrayType::Guid => {
                        let name = w.unwrap().name();
                        s.wln(format!("ptvcursor_add(ptv, {name}, 8, ENC_LITTLE_ENDIAN);",));
                    }
                    ArrayType::Struct(c) => {
                        print_container(
                            s,
                            o,
                            wo,
                            variables,
                            c,
                            inside_compressed_message,
                            iteration_variable,
                            depth,
                        );
                    }
                    ArrayType::CString => {
                        print_cstring(s, w);
                    }
                    ArrayType::PackedGuid => {
                        s.wln("add_packed_guid(ptv, pinfo);");
                    }
                }

                s.closing_curly();
            }
        }
        Type::PackedGuid => {
            s.wln("add_packed_guid(ptv, pinfo);");
        }
        Type::AuraMask => {
            s.wln("add_aura_mask(ptv);");
        }
        Type::UpdateMask { .. } => {
            s.wln("add_update_mask(ptv);");
        }
        Type::MonsterMoveSplines => {
            s.wln("add_monster_move_spline(ptv);");
        }
        Type::String { .. } => unreachable!("Strings are only in login messages"),
        Type::AchievementInProgressArray | Type::AchievementDoneArray => {
            unreachable!("achievement arrays are only in 3.3.5")
        }
        Type::EnchantMask => {
            unreachable!("enchant masks are only in 3.3.5")
        }
        Type::InspectTalentGearMask => {
            unreachable!("inspect talent gear masks are only in 3.3.5")
        }
        Type::NamedGuid => {
            unreachable!("named guid only in 2.4.3 and 3.3.5")
        }
        Type::VariableItemRandomProperty => {
            unreachable!("variable item random property id only in 2.4.3 and 3.3.5")
        }
        Type::AddonArray => {
            unreachable!("addon array only in 2.4.3/3.3.5")
        }
        Type::IpAddress => {
            unreachable!("ip addresses are only in login")
        }
    }

    if d.tags().compressed().is_some() {
        s.wln("ptvcursor_free(ptv);");
        s.wln("ptv = old_ptv;");
        s.wln("compressed_tvb = NULL;");
        s.closing_curly(); // if (compressed_tvb != NULL)
    }
}

fn print_cstring(s: &mut Writer, w: Option<&WiresharkMember>) {
    let name = w.unwrap().name();

    s.wln(format!("add_cstring(ptv, &{name});"));
}

fn print_container(
    s: &mut Writer,
    o: &Objects,
    wo: &WiresharkObject,
    variables: &mut Vec<String>,
    e: &Container,
    inside_compressed_message: bool,
    iteration_variable: Option<String>,
    depth: i32,
) {
    let name = e.name();
    let text = if let Some(variable) = iteration_variable {
        format!("\"{name} %i\", {variable}")
    } else {
        format!("\"{name}\"")
    };

    s.wln(format!(
        "ptvcursor_add_text_with_subtree(ptv, SUBTREE_UNDEFINED_LENGTH, ett_message, {text});",
    ));
    for m in e.members() {
        print_member(
            s,
            m,
            e,
            wo,
            e.tags(),
            o,
            variables,
            inside_compressed_message,
            depth,
        );
    }
    s.wln("ptvcursor_pop_subtree(ptv);");
}

fn print_definer(
    s: &mut Writer,
    w: Option<&WiresharkMember>,
    upcast: &Option<IntegerType>,
    container_name: &str,
    variable_name: &str,
    variables: &mut Vec<String>,
    e: &Definer,
) {
    let len = if let Some(upcast) = upcast {
        upcast.size()
    } else {
        e.ty().size()
    };
    let name = w.unwrap().name();

    if e.used_in_if_in_object(container_name) {
        variables.push(variable_name.to_string());
        s.wln(format!(
            "ptvcursor_add_ret_uint(ptv, {name}, {len}, ENC_LITTLE_ENDIAN, &{variable_name});",
        ));
    } else {
        s.wln(format!(
            "ptvcursor_add(ptv, {name}, {len}, ENC_LITTLE_ENDIAN);",
        ));
    }
}

pub(crate) fn print_register_info(w: &WiresharkObject) -> Writer {
    let mut s = Writer::at_indentation(2);

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
            let vals = if matches!(
                m.ty(),
                WiresharkType::Enum(_, IntegerType::U64) | WiresharkType::Enum(_, IntegerType::I64)
            ) {
                "VALS64"
            } else {
                "VALS"
            };
            format!("{vals}({})", enum_strings(e.name()))
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

    s
}

pub(crate) fn print_int_declarations(w: &WiresharkObject) -> Writer {
    let mut s = Writer::new();

    for m in w.members() {
        s.wln(format!("static int {} = -1;", m.name()));
    }

    s
}

pub(crate) fn print_enums(w: &WiresharkObject) -> Writer {
    let mut s = Writer::new();

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

    struct ReallySigned(i32);

    impl UpperHex for ReallySigned {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let prefix = if f.alternate() { "0x" } else { "" };

            let bare_hex = {
                let value = if self.0 == i32::MIN {
                    self.0 + 1
                } else {
                    self.0
                };
                format!("{:X}", value.abs())
            };
            f.pad_integral(self.0 >= 0, prefix, &bare_hex)
        }
    }

    s.body_closing_with(
        "typedef enum",
        |s| {
            for enumerator in e.fields() {
                // C can not have different enum types other than 'int', so we'll need to
                // make the integers signed. Rust does not format hex numbers sign aware, so
                // we'll need `ReallySigned` to wrap.
                let value = ReallySigned(enumerator.value().int() as i32);
                let value = format!("{value:#0hex_width$X}");

                s.wln(format!(
                    "{enumerator_name} = {value},",
                    enumerator_name = enumerator_name(e.name(), enumerator.name()),
                ))
            }
        },
        format!(" {};", enum_name(e.name())),
    );
}

fn print_enum(s: &mut Writer, e: &Definer) {
    print_typedef(s, e);

    let value_string = if matches!(e.ty(), &IntegerType::U64 | &IntegerType::I64) {
        "val64_string"
    } else {
        "value_string"
    };

    s.body_closing_with(
        format!(
            "static const {value_string} {}[] = ",
            enum_strings(e.name())
        ),
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
