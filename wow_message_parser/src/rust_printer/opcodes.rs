use crate::file_utils::{get_import_path, major_version_to_string};
use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::version::{LoginVersion, MajorWorldVersion, Version};
use crate::rust_printer::structs::print_common_impls::impl_read_write_non_trait;
use crate::rust_printer::{
    ImplType, Writer, ASYNC_STD_IMPORT, CFG_ASYNC_ASYNC_STD, CFG_ASYNC_TOKIO, CFG_TESTCASE,
    CLIENT_MESSAGE_TRAIT_NAME, EXPECTED_OPCODE_ERROR, PARSE_ERROR, SERVER_MESSAGE_TRAIT_NAME,
    SYNC_IMPORT, TOKIO_IMPORT,
};

const CLOGIN_NAME: &str = "Client";
const SLOGIN_NAME: &str = "Server";
const CMSG_NAME: &str = "Client";
const SMSG_NAME: &str = "Server";

pub(crate) fn print_world_opcodes(
    v: &[&Container],
    version: &MajorWorldVersion,
    container_type: ContainerType,
) -> Writer {
    let mut s = Writer::new();
    let ty = match container_type {
        ContainerType::SMsg(_) => SMSG_NAME,
        ContainerType::CMsg(_) => CMSG_NAME,
        _ => unreachable!("invalid type passed to opcode printer"),
    };

    includes(&mut s, v, container_type, Version::World(*version));

    definition(&mut s, v, ty, (*version).into());

    common_impls_world(&mut s, v, ty, container_type, *version);

    s
}

pub(crate) fn print_login_opcodes(
    v: &[&Container],
    version: &LoginVersion,
    container_type: ContainerType,
) -> Writer {
    let mut s = Writer::new();
    let ty = match container_type {
        ContainerType::CLogin(_) => CLOGIN_NAME,
        ContainerType::SLogin(_) => SLOGIN_NAME,
        _ => unreachable!("invalid type passed to opcode printer"),
    };

    includes(&mut s, v, container_type, Version::Login(*version));

    definition(&mut s, v, ty, (*version).into());

    common_impls_login(&mut s, v, ty);

    s
}

fn any_container_is_pure_movement_info(v: &[&Container]) -> bool {
    v.iter().any(|a| a.is_pure_movement_info())
}

pub(crate) fn includes(
    s: &mut Writer,
    v: &[&Container],
    container_type: ContainerType,
    version: Version,
) {
    match container_type {
        ContainerType::SLogin(_) => {
            s.wln(format!(
                "use crate::{{{SERVER_MESSAGE_TRAIT_NAME}, {CLIENT_MESSAGE_TRAIT_NAME}}};"
            ));

            s.wln(SYNC_IMPORT);

            s.wln(CFG_ASYNC_TOKIO);
            s.wln(TOKIO_IMPORT);
            s.wln(CFG_ASYNC_ASYNC_STD);
            s.wln(ASYNC_STD_IMPORT);

            s.wln("use super::*;");
            s.wln("use crate::all::*;");
        }
        ContainerType::CMsg(_) => {
            s.wln(format!(
                "use crate::{}::{{{}, {}}};",
                major_version_to_string(&version.as_major_world()),
                SERVER_MESSAGE_TRAIT_NAME,
                CLIENT_MESSAGE_TRAIT_NAME,
            ));

            s.wln("#[cfg(feature = \"encryption\")]");
            let import_path = version.as_major_world().encryption_path();
            match version.as_major_world() {
                MajorWorldVersion::Vanilla | MajorWorldVersion::BurningCrusade => {
                    s.wln(format!(
                        "use {import_path}::{{DecrypterHalf, EncrypterHalf}};"
                    ));
                }
                MajorWorldVersion::Wrath => {
                    s.wln(format!("use {import_path}::{{ClientEncrypterHalf, ClientDecrypterHalf, ServerEncrypterHalf, ServerDecrypterHalf}};"));
                }
            }

            s.newline();

            s.wln(SYNC_IMPORT);

            s.wln(CFG_ASYNC_TOKIO);
            s.wln(TOKIO_IMPORT);
            s.wln(CFG_ASYNC_ASYNC_STD);
            s.wln(ASYNC_STD_IMPORT);

            if any_container_is_pure_movement_info(v) {
                s.wln(format!(
                    "use {module_name}::MovementInfo;",
                    module_name = get_import_path(version)
                ));
            }

            s.wln(format!("use {PARSE_ERROR};"));

            s.wln(format!(
                "use crate::{}::opcode_to_name;",
                major_version_to_string(&version.as_major_world()),
            ));

            s.wln("use super::*;");
        }
        _ => {}
    }

    s.newline();
}

pub(crate) fn definition(s: &mut Writer, v: &[&Container], ty: &str, version: Version) {
    // Login does not have any floats guaranteed while World does.
    // We could also dynamically check, but then adding a message with floats
    // would be a breaking change.
    match version {
        Version::Login(_) => {
            s.wln("#[derive(Debug, Clone, PartialEq, Eq)]");
        }
        Version::World(_) => {
            s.wln("#[derive(Debug, Clone, PartialEq)]");
        }
    }

    s.new_enum("pub", format!("{ty}OpcodeMessage"), |s| {
        for &e in v {
            let ty = if e.empty_body() {
                "".to_string()
            } else {
                format!("({})", e.name())
            };
            s.wln(format!(
                "{enum_name}{ty},",
                enum_name = get_enumerator_name(e.name()),
            ));
        }
    });
}

fn world_common_impls_read_opcodes(s: &mut Writer, v: &[&Container], size: &str, error_ty: &str) {
    s.bodyn(format!("fn read_opcodes(opcode: {size}, body_size: u32, mut r: &[u8]) -> Result<Self, {error_ty}>"), |s| {
        s.open_curly("match opcode");

        for &e in v {
            let opcode = match e.container_type() {
                ContainerType::CMsg(i) => i,
                ContainerType::SMsg(i) => i,
                ContainerType::Msg(i) => i,
                _ => unreachable!()
            };
            if e.empty_body() {
                s.wln(
                    format!(
                        "{opcode:#06X} => crate::util::assert_empty(body_size, opcode).map(|_| Self::{enum_name}),",
                              enum_name = get_enumerator_name(e.name())
                ));
            } else {
                s.wln(format!("{opcode:#06X} => Ok(Self::{enum_name}(<{name} as crate::Message>::read_body::<crate::traits::private::Internal>(&mut r, body_size).map_err(|a| {{ if let ParseError::Io(io) = a {{ ParseError::BufferSizeTooSmall {{ opcode: {opcode:#06X}, size: body_size, io, }} }} else {{ a }} }})?)),",
                              opcode = opcode,
                              name = e.name(),
                              enum_name = get_enumerator_name(e.name())
                ));
            }
        }

        let (opcode_text, opcode_to_name_text) = if size == "u32" {
            ("opcode", "opcode")
        } else {
            ("opcode: opcode.into()", "opcode.into()")
        };
        s.wln(format!("_ => Err({error_ty}::Opcode{{ {opcode_text}, name: opcode_to_name({opcode_to_name_text}), size: body_size }}),"));

        s.closing_curly(); // match opcode
    });
}

fn world_common_impls_read_write(
    s: &mut Writer,
    cd: &str,
    dec_prefix: &str,
    size: &str,
    opcode_size: i32,
    error_ty: &str,
    it: ImplType,
    version: MajorWorldVersion,
    ty: &str,
) {
    s.wln(it.cfg());
    s.open_curly(format!(
        "pub {func}fn {prefix}read_unencrypted<R: {read}>(mut r: R) -> Result<Self, {error_ty}>",
        prefix = it.prefix(),
        read = it.read(),
        func = it.func(),
        error_ty = error_ty,
    ));

    if version.wrath_or_greater() && ty == "Server" {
        s.wln("let mut header = [0_u8; 4];");
        s.wln(format!(
            "r.read_exact(&mut header){postfix}?;",
            postfix = it.postfix()
        ));
        s.newline();

        s.body("let (size, opcode) = if header[0] & 0x80 != 0", |s| {
            s.wln("let size = u32::from_be_bytes([0x00, header[0] & 0x7F, header[1], header[2]]).saturating_sub(3);");
            s.newline();

            s.wln("let mut last_byte = [0_u8; 1];");
            s.wln(format!(
                "r.read_exact(&mut last_byte){postfix}?;",
                postfix = it.postfix()
            ));
            s.wln("let opcode = u16::from_le_bytes([header[3], last_byte[0]]);");
            s.wln("(size, opcode)");
        });
        s.body_closing_with(
            "else",
            |s| {
                s.wln("let size = u16::from_be_bytes([header[0], header[1]]).saturating_sub(2).into();");
                s.wln("let opcode = u16::from_le_bytes([header[2], header[3]]);");
                s.wln("(size, opcode)");
            },
            ";",
        );
    } else {
        s.wln(format!(
            "let size = ({path}::{prefix}read_u16_be(&mut r){postfix}?.saturating_sub({opcode_size})) as u32;",
            path = "crate::util",
            opcode_size = opcode_size,
            prefix = it.prefix(),
            postfix = it.postfix()
        ));

        s.wln(format!(
            "let opcode = {path}::{prefix}read_{size}_le(&mut r){postfix}?;",
            path = "crate::util",
            size = size,
            prefix = it.prefix(),
            postfix = it.postfix()
        ));
    }
    s.newline();

    s.wln("let mut buf = vec![0; size as usize];");
    s.wln(format!(
        "r.read_exact(&mut buf){postfix}?;",
        postfix = it.postfix()
    ));

    s.wln("Self::read_opcodes(opcode, size, &buf)");

    s.closing_curly(); // read_unencrypted

    s.wln(it.cfg_and_encryption());
    s.open_curly(
        format!("pub {func}fn {prefix}read_encrypted<R: {read}>(mut r: R, d: &mut {dec_prefix}DecrypterHalf) -> Result<Self, {error_ty}>",
                func = it.func(),
                prefix = it.prefix(),
                read = it.read(),
                error_ty = error_ty,
                dec_prefix = dec_prefix,
        )
    );

    if version.wrath_or_greater() && ty == "Server" {
        s.wln("let mut header = [0_u8; 4];");
        s.wln(format!(
            "r.read_exact(&mut header){postfix}?;",
            postfix = it.postfix()
        ));
        s.wln("d.decrypt(&mut header);");
        s.newline();

        s.body("let (body_size, opcode) = if header[0] & 0x80 != 0", |s| {
            s.wln("let size = u32::from_be_bytes([0x00, header[0] & 0x7F, header[1], header[2]]).saturating_sub(3);");
            s.newline();

            s.wln("let mut last_byte = [0_u8; 1];");
            s.wln(format!(
                "r.read_exact(&mut last_byte){postfix}?;",
                postfix = it.postfix()
            ));
            s.wln("d.decrypt(&mut last_byte);");
            s.wln("let opcode = u16::from_le_bytes([header[3], last_byte[0]]);");
            s.wln("(size, opcode)");
        });
        s.body_closing_with(
            "else",
            |s| {
                s.wln("let size = u16::from_be_bytes([header[0], header[1]]).saturating_sub(2).into();");
                s.wln("let opcode = u16::from_le_bytes([header[2], header[3]]);");
                s.wln("(size, opcode)");
            },
            ";",
        );
    } else {
        s.wln(format!(
            "let mut header = [0_u8; {header_size}];",
            header_size = opcode_size + 2
        ));
        s.wln(format!(
            "r.read_exact(&mut header){postfix}?;",
            postfix = it.postfix()
        ));
        s.wln(format!("let header = d.decrypt_{cd}_header(header);"));
        s.wln("let opcode = header.opcode;");
        s.wln(format!(
            "let body_size = (header.size.saturating_sub({opcode_size})) as u32;"
        ));
    }
    s.newline();

    s.wln("let mut buf = vec![0; body_size as usize];");
    s.wln(format!(
        "r.read_exact(&mut buf){postfix}?;",
        postfix = it.postfix()
    ));

    s.wln("Self::read_opcodes(opcode, body_size, &buf)");

    s.closing_curly_newline();
}

pub(crate) fn common_impls_world(
    s: &mut Writer,
    v: &[&Container],
    ty: &str,
    container_type: ContainerType,
    version: MajorWorldVersion,
) {
    let ((enc_prefix, dec_prefix), cd, size, opcode_size) = match container_type {
        ContainerType::CMsg(_) => (
            if version.wrath_or_greater() {
                ("Client", "Server")
            } else {
                ("", "")
            },
            "client",
            "u32",
            4,
        ),
        ContainerType::SMsg(_) => (
            if version.wrath_or_greater() {
                ("Server", "Client")
            } else {
                ("", "")
            },
            "server",
            "u16",
            2,
        ),
        _ => unreachable!("not a world type: '{:#?}'", container_type),
    };
    s.bodyn(format!("impl {ty}OpcodeMessage"), |s| {
        world_common_impls_read_opcodes(s, v, size, EXPECTED_OPCODE_ERROR);

        for it in ImplType::types() {
            world_common_impls_read_write(
                s,
                cd,
                dec_prefix,
                size,
                opcode_size,
                EXPECTED_OPCODE_ERROR,
                it,
                version,
                ty,
            );
        }

        for it in ImplType::types() {
            world_inner(s, v, cd, it, enc_prefix);
        }

        if any_container_is_pure_movement_info(v) {
            world_movement_info(s, v);
        }

        s.wln(CFG_TESTCASE);
        s.funcn_pub("to_test_case_string(&self)", "Option<String>", |s| {
            s.body("match self", |s| {
                for container in v {
                    let en = get_enumerator_name(container.name());
                    let name = container.name();

                    if container.empty_body() {
                        s.wln(format!("Self::{en} => {name}{{}}.to_test_case_string(),",));
                    } else {
                        s.wln(format!("Self::{en}(c) => c.to_test_case_string(),",));
                    }
                }
            });
        });

        s.wln(CFG_TESTCASE);
        s.funcn_pub("message_name(&self)", "&'static str", |s| {
            s.body("match self", |s| {
                for container in v {
                    let en = get_enumerator_name(container.name());
                    let name = container.name();

                    if container.empty_body() {
                        s.wln(format!("Self::{en} => \"{name}\",",));
                    } else {
                        s.wln(format!("Self::{en}(c) => \"{name}\",",));
                    }
                }
            });
        });
    });

    impl_display(s, v, ty);

    print_froms(s, v, ty);
}

fn impl_display(s: &mut Writer, v: &[&Container], ty: &str) {
    s.bodyn(
        format!("impl std::fmt::Display for {ty}OpcodeMessage"),
        |s| {
            s.body(
                "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result",
                |s| {
                    s.body_closing_with(
                        "f.write_str(match self",
                        |s| {
                            for e in v {
                                if e.empty_body() {
                                    s.wln(format!(
                                        "{ty}OpcodeMessage::{enumerator} => \"{message}\",",
                                        ty = ty,
                                        enumerator = get_enumerator_name(e.name()),
                                        message = e.name(),
                                    ));
                                } else {
                                    s.wln(format!(
                                        "{ty}OpcodeMessage::{enumerator}(_) => \"{message}\",",
                                        ty = ty,
                                        enumerator = get_enumerator_name(e.name()),
                                        message = e.name(),
                                    ));
                                }
                            }
                        },
                        ")",
                    );
                },
            )
        },
    );
}

fn world_inner(s: &mut Writer, v: &[&Container], cd: &str, it: ImplType, enc_prefix: &str) {
    s.wln(it.cfg_and_encryption());
    s.bodyn(
        format!(
            "pub {func}fn {prefix}write_encrypted_{cd}<W: {write}>(&self, mut w: W, e: &mut {enc_prefix}EncrypterHalf) -> Result<(), std::io::Error>",
            cd = cd,
            enc_prefix = enc_prefix,
            func = it.func(),
            write = it.write(),
            prefix = it.prefix(),
        ), |s| {
            s.body("match self", |s| {
                for container in v {
                    if container.empty_body() {
                        s.wln(format!("Self::{en} => {name}{{}}.{prefix}write_encrypted_{cd}(w, e){postfix},",
                                      en = get_enumerator_name(container.name()),
                                      name = container.name(),
                                      prefix = it.prefix(),
                                      postfix = it.postfix(),
                        ));
                    } else {
                        s.wln(format!("Self::{en}(c) => c.{prefix}write_encrypted_{cd}(w, e){postfix},",
                                      en = get_enumerator_name(container.name()),
                                      prefix = it.prefix(),
                                      postfix = it.postfix(),
                        ));
                    }
                }
            });
        });

    s.wln(it.cfg());
    s.bodyn(
        format!(
            "pub {func}fn {prefix}write_unencrypted_{cd}<W: {write}>(&self, mut w: W) -> Result<(), std::io::Error>",
            cd = cd,
            func = it.func(),
            write = it.write(),
            prefix = it.prefix(),
        ), |s| {
            s.body("match self", |s| {
                for container in v {
                    if container.empty_body() {
                        s.wln(format!("Self::{en} => {name}{{}}.{prefix}write_unencrypted_{cd}(w){postfix},",
                                      en = get_enumerator_name(container.name()),
                                      name = container.name(),
                                      prefix = it.prefix(),
                                      postfix = it.postfix()
                        ));
                    } else {
                        s.wln(format!("Self::{en}(c) => c.{prefix}write_unencrypted_{cd}(w){postfix},",
                                      en = get_enumerator_name(container.name()),
                                      prefix = it.prefix(),
                                      postfix = it.postfix()
                        ));
                    }
                }
            });
        },
    );
}

fn world_movement_info(s: &mut Writer, v: &[&Container]) {
    s.funcn_pub_const("movement_info(&self)", "Option<&MovementInfo>", |s| {
        s.body("match self", |s| {
            for container in v {
                if container.is_pure_movement_info() {
                    s.wln(format!(
                        "Self::{}(c) => Some(&c.info),",
                        get_enumerator_name(container.name())
                    ));
                }
            }

            s.wln("_ => None,");
        });
    });
}

pub(crate) fn common_impls_login(s: &mut Writer, v: &[&Container], ty: &str) {
    impl_read_write_non_trait(
        s,
        format!("{ty}OpcodeMessage"),
        EXPECTED_OPCODE_ERROR,
        |s, it| {
            s.wln(format!(
                "let opcode = crate::util::{prefix}read_u8_le(&mut r){postfix}?;",
                prefix = it.prefix(),
                postfix = it.postfix()
            ));

            s.body("match opcode", |s| {
                for e in v {
                    let opcode = match e.container_type() {
                        ContainerType::CLogin(opcode) |
                        ContainerType::SLogin(opcode) => opcode,
                        _ => unreachable!()
                    };

                    if e.empty_body() {
                        s.wln(format!(
                            "{opcode:#04X} => Ok(Self::{enum_name}),",
                            enum_name = get_enumerator_name(e.name()),
                            opcode = opcode,
                        ));
                    } else {
                        s.wln(format!(
                            "{opcode:#04X} => Ok(Self::{enum_name}({name}::{prefix}read::<R, crate::private::Internal>(r){postfix}?)),",
                            name = e.name(),
                            enum_name = get_enumerator_name(e.name()),
                            opcode = opcode,
                            prefix = it.prefix(),
                            postfix = it.postfix(),
                        ));
                    }

                }

                s.wln(format!("opcode => Err({EXPECTED_OPCODE_ERROR}::Opcode(opcode as u32)),"));
            });
        },
        |s, _it| {
            s.bodyn("match self", |s| {
                for e in v {
                    if e.empty_body() {
                        s.wln(format!(
                            "Self::{enum_name} => {{}}",
                            enum_name = get_enumerator_name(e.name()),
                        ));
                    } else {
                        s.wln(format!(
                            "Self::{enum_name}(e) => e.write_into_vec(w)?,",
                            enum_name = get_enumerator_name(e.name()),
                        ));
                    }
                }
            });
        },
        "pub(crate)",
        "pub",
        true,
    );

    impl_display(s, v, ty);

    print_froms(s, v, ty);
}

fn print_froms(s: &mut Writer, v: &[&Container], ty: &str) {
    for &e in v {
        s.impl_for(
            format!("From<{}>", e.name()),
            format!("{ty}OpcodeMessage"),
            |s| {
                let (variable, extra) = if e.empty_body() {
                    ("_", "")
                } else {
                    ("c", "(c)")
                };

                s.body(format!("fn from({variable}: {}) -> Self", e.name()), |s| {
                    s.wln(format!("Self::{}{extra}", get_enumerator_name(e.name())));
                });
            },
        );
    }
}

pub(crate) fn get_enumerator_name(name: &str) -> String {
    name.replace("_Client", "").replace("_Server", "")
}
