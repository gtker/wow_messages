use crate::file_utils::overwrite_if_not_same_contents;
use crate::parser::types::version::MajorWorldVersion;
use crate::path_utils::expected_file;
use crate::rust_printer::writer::Writer;
use crate::rust_printer::{ImplType, MessageSide};

pub(crate) fn print_expected() {
    for version in MajorWorldVersion::versions() {
        print_expected_world_version(*version);
    }
}

fn print_expect_message(
    s: &mut Writer,
    message_side: MessageSide,
    it: ImplType,
    encrypted: bool,
    version: MajorWorldVersion,
) {
    let read = it.read();
    let prefix = it.prefix();
    let postfix = it.postfix();
    let func = it.func();
    let title_side = message_side.title_world_str();
    let side = message_side.world_str();
    let module = version.module_name();

    let encryption = if encrypted {
        s.wln(it.cfg_and_encryption());
        "_encryption"
    } else {
        s.wln(it.cfg());
        ""
    };
    s.wln(format!(
        "pub {func}fn {prefix}expect_{side}_message{encryption}<M: crate::{module}::{title_side}Message, R: {read}>("
    ));
    s.inc_indent();
    s.wln("r: &mut R,");
    if encrypted {
        let name = match version {
            MajorWorldVersion::Vanilla | MajorWorldVersion::BurningCrusade => {
                "DecrypterHalf".to_string()
            }
            MajorWorldVersion::Wrath => {
                let other_title_side = message_side.opposite_title_world_str();
                format!("{other_title_side}DecrypterHalf")
            }
        };

        s.wln(format!("d: &mut wow_srp::{module}_header::{name},"));
    }

    s.dec_indent();
    s.open_curly(") -> Result<M, crate::errors::ExpectedOpcodeError>");

    if !(version.wrath_or_greater() && message_side.is_server()) {
        let header_size = message_side.non_wrath_world_header_size();
        s.wln(format!("let mut header = [0_u8; {header_size}];"));
        s.wln(format!("r.read_exact(&mut header){postfix}?;"));

        if encrypted {
            s.wln(format!("let d = d.decrypt_{side}_header(header);"));
        } else {
            s.wln(format!(
                "let d = crate::util::{title_side}Header::from_array(header);"
            ));
        };
    } else if encrypted {
        s.wln("let mut buf = [0_u8; 4];");
        s.wln(format!("r.read_exact(&mut buf){postfix}?;"));

        s.body_closing_with(
            "let d = match d.attempt_decrypt_server_header(buf)",
            |s| {
                s.wln("wow_srp::wrath_header::WrathServerAttempt::Header(h) => h,");
                s.body(
                    "wow_srp::wrath_header::WrathServerAttempt::AdditionalByteRequired =>",
                    |s| {
                        s.wln("let mut lsb = [0_u8; 1];");
                        s.wln(format!("r.read_exact(&mut lsb){postfix}?;"));
                        s.wln("d.decrypt_large_server_header(lsb[0])");
                    },
                );
            },
            ";",
        );
    } else {
        s.block(format!(
            "\
let mut buf = [0_u8; 4];
r.read_exact(&mut buf){postfix}?;
let d = if buf[0] & 0x80 != 0 {{
    let mut lsb = [0_u8; 1];
    r.read_exact(&mut lsb){postfix}?;
    let buf = [buf[0], buf[1], buf[2], buf[3], lsb[0]];
    crate::util::ServerHeader::from_large_array(buf)
}} else {{
    crate::util::ServerHeader::from_array(buf)
}};
",
        ));
    }
    s.newline();

    let opcode_size = message_side.world_opcode_size();
    s.wln(format!(
        "let mut buf = vec![0_u8; d.size.saturating_sub({opcode_size}) as usize];"
    ));
    s.wln(format!("r.read_exact(&mut buf){postfix}?;"));
    s.newline();

    let size_into = match message_side {
        MessageSide::Server => {
            if version.wrath_or_greater() {
                ""
            } else if encrypted {
                ".into()"
            } else {
                ""
            }
        }
        MessageSide::Client => "",
    };
    let opcode_into = match message_side {
        MessageSide::Server => ".into()",
        MessageSide::Client => "",
    };

    s.wln(format!(
        "read_{side}_body(&mut buf.as_slice(), d.size{size_into}, d.opcode{opcode_into})"
    ));

    s.closing_curly_newline(); // ) -> Result<M
}

fn print_expected_world_version(version: MajorWorldVersion) {
    let module = version.module_name();
    let mut s = Writer::new();

    for it in ImplType::types() {
        for encrypted in [false, true] {
            for side in MessageSide::values() {
                print_expect_message(&mut s, *side, it, encrypted, version);
            }
        }
    }

    print_async_cfg(&mut s, &ImplType::types());
    s.block(format!(
        "\
fn read_server_body<M: crate::{module}::ServerMessage>(
    buf: &mut &[u8],
    size: u32,
    opcode: u32,
) -> Result<M, crate::errors::ExpectedOpcodeError> {{
    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {{
        let m = M::read_body::<crate::traits::private::Internal>(
            buf,
            size.saturating_sub(2),
        );
        match m {{
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }}
    }} else {{
        Err(crate::errors::ExpectedOpcodeError::Opcode {{
            opcode,
            name: crate::{module}::opcode_to_name(opcode),
            size,
        }})
    }}
}}
",
    ));
    s.newline();

    print_async_cfg(&mut s, &ImplType::types());
    s.block(format!(
        "\
fn read_client_body<M: crate::{module}::ClientMessage>(
    buf: &mut &[u8],
    size: u16,
    opcode: u32,
) -> Result<M, crate::errors::ExpectedOpcodeError> {{
    // Unable to match on associated const M::OPCODE, so we do if
    if opcode == M::OPCODE {{
        let m = M::read_body::<crate::traits::private::Internal>(
            buf,
            size.saturating_sub(4) as u32,
        );
        match m {{
            Ok(m) => Ok(m),
            Err(e) => Err(e.into()),
        }}
    }} else {{
        Err(crate::errors::ExpectedOpcodeError::Opcode {{
            opcode,
            name: crate::{module}::opcode_to_name(opcode),
            size: size.into(),
        }})
    }}
}}
"
    ));

    overwrite_if_not_same_contents(s.inner(), &expected_file(version));
}

fn print_async_cfg(s: &mut Writer, it: &[ImplType]) {
    s.w("#[cfg(any(");
    for (i, feature) in it.iter().enumerate() {
        if i != 0 {
            s.w(", ");
        }

        s.w(format!("feature = \"{}\"", feature.cfg_name()));
    }
    s.wln("))]");
}
