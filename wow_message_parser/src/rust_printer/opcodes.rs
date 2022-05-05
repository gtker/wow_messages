use crate::container::{Container, ContainerType};
use crate::file_utils::{get_import_path, get_login_logon_version_path, get_world_version_path};
use crate::parser::types::tags::{LoginVersion, WorldVersion};
use crate::rust_printer::{
    ImplType, Writer, ASYNC_STD_IMPORT, ASYNC_TRAIT_IMPORT, ASYNC_TRAIT_MACRO, CFG_ASYNC_ANY,
    CFG_ASYNC_ASYNC_STD, CFG_ASYNC_TOKIO, OPCODE_MESSAGE_TRAIT_NAME, TOKIO_IMPORT,
    WORLD_BODY_TRAIT_NAME, WORLD_CLIENT_HEADER_TRAIT_NAME, WORLD_SERVER_HEADER_TRAIT_NAME,
};

const CLOGIN_NAME: &str = "Client";
const SLOGIN_NAME: &str = "Server";
const CMSG_NAME: &str = "Client";
const SMSG_NAME: &str = "Server";

pub fn print_world_opcodes(
    v: &[&Container],
    version: &WorldVersion,
    container_type: ContainerType,
) -> Writer {
    let mut s = Writer::new(&get_world_version_path(version));
    let (ty, int_ty) = match container_type {
        ContainerType::SMsg(_) => (SMSG_NAME, "u16"),
        ContainerType::CMsg(_) => (CMSG_NAME, "u32"),
        _ => panic!("invalid type passed to opcode printer"),
    };

    includes(&mut s, v, container_type);

    opcode_enum_world(&mut s, v, ty, container_type);

    definition(&mut s, v, ty);

    common_impls_world(&mut s, v, ty, container_type);

    print_error(&mut s, v, ty, int_ty);

    s
}

pub fn print_login_opcodes(
    v: &[&Container],
    version: &LoginVersion,
    container_type: ContainerType,
) -> Writer {
    let mut s = Writer::new(&get_login_logon_version_path(version));
    let (ty, int_ty) = match container_type {
        ContainerType::CLogin(_) => (CLOGIN_NAME, "u8"),
        ContainerType::SLogin(_) => (SLOGIN_NAME, "u8"),
        _ => panic!("invalid type passed to opcode printer"),
    };

    includes(&mut s, v, container_type);

    definition(&mut s, v, ty);

    common_impls_login(&mut s, v, ty);

    print_error(&mut s, v, ty, int_ty);

    opcode_enum_login(&mut s, v, ty);

    s
}

pub fn includes(s: &mut Writer, v: &[&Container], container_type: ContainerType) {
    match container_type {
        ContainerType::SLogin(_) => {
            s.wln("use crate::ReadableAndWritable;");

            s.newline();

            s.wln(CFG_ASYNC_ANY);
            s.wln(ASYNC_TRAIT_IMPORT);
            s.wln(CFG_ASYNC_TOKIO);
            s.wln(TOKIO_IMPORT);
            s.wln(CFG_ASYNC_ASYNC_STD);
            s.wln(ASYNC_STD_IMPORT);
        }
        ContainerType::CMsg(_) => {
            s.wln(format!("use crate::{};", WORLD_BODY_TRAIT_NAME));
            s.wln(format!("use crate::{};", OPCODE_MESSAGE_TRAIT_NAME));
            s.wln(format!(
                "use crate::{{{}, {}}};",
                WORLD_SERVER_HEADER_TRAIT_NAME, WORLD_CLIENT_HEADER_TRAIT_NAME,
            ));
            s.wln("use wow_srp::header_crypto::{Decrypter, Encrypter};");

            s.newline();

            s.wln(CFG_ASYNC_ANY);
            s.wln(ASYNC_TRAIT_IMPORT);
            s.wln(CFG_ASYNC_TOKIO);
            s.wln(TOKIO_IMPORT);
            s.wln(CFG_ASYNC_ASYNC_STD);
            s.wln(ASYNC_STD_IMPORT);
        }
        _ => {}
    }

    for e in v {
        if let ContainerType::SMsg(_) = container_type {
            if let ContainerType::Msg(_) = e.container_type() {
                continue;
            }
        }

        let module_name = get_import_path(e.tags());
        if e.only_has_io_errors() {
            s.wln(format!(
                "use {module_name}::{name};",
                module_name = module_name,
                name = e.name(),
            ));
        } else {
            s.wln(format!(
                "use {module_name}::{{{name}, {name}Error}};",
                module_name = module_name,
                name = e.name(),
            ));
        }
    }

    s.newline();
}

pub fn definition(s: &mut Writer, v: &[&Container], ty: &str) {
    s.wln("#[derive(Debug)]");
    s.new_enum("pub", format!("{t}OpcodeMessage", t = ty), |s| {
        for &e in v {
            s.wln(format!(
                "{enum_name}({name}),",
                enum_name = get_enumerator_name(e.name()),
                name = e.name()
            ));
        }
    });
}

fn world_common_impls_read_write(
    s: &mut Writer,
    v: &[&Container],
    cd: &str,
    size: &str,
    opcode_size: i32,
    it: ImplType,
) {
    s.wln(format!("{}", it.cfg()));
    s.bodyn(format!("{func}fn {prefix}write_unencrypted<W: {write}>(&self, w: &mut W) -> std::result::Result<(), std::io::Error>", func = it.func(), prefix = it.prefix(), write = it.write() ), |s| {
        s.body("match self", |s| {
            for &e in v {
                s.wln(format!("Self::{name}(i) => i.{prefix}write_body(w){postfix}?,",
                    prefix = it.prefix(), postfix = it.postfix(),
                      name = get_enumerator_name(e.name())));
            }
        });
        s.wln("Ok(())");
    });

    s.wln(format!("{}", it.cfg()));
    s.bodyn(format!("{func}fn {prefix}read_unencrypted<R: {read}>(r: &mut R) -> std::result::Result<Self, Self::Error>", func = it.func(), prefix = it.prefix(), read = it.read()), |s| {
        s.wln(format!("let size = ({path}::{prefix}read_u16_be(r){postfix}? - {opcode_size}) as u32;", path = "crate::util", opcode_size = opcode_size, prefix = it.prefix(), postfix = it.postfix()));

        s.wln(format!("let opcode = {path}::{prefix}read_{size}_le(r){postfix}?;",path = "crate::util", size = size, prefix = it.prefix(), postfix = it.postfix()));

        s.body("match opcode", |s| {
            for &e in v {
                let opcode = match e.container_type() {
                    ContainerType::CMsg(i) => i,
                    ContainerType::SMsg(i) => i,
                    ContainerType::Msg(i) => i,
                    _ => panic!(),
                };
                s.wln(format!("{opcode:#06X} => Ok(Self::{enum_name}({name}::{prefix}read_body(r, size){postfix}?)),",
                              opcode = opcode,
                              name = e.name(),
                              prefix = it.prefix(),
                              postfix = it.postfix(),
                              enum_name = get_enumerator_name(e.name())));
            }
            s.wln("_ => Err(Self::Error::InvalidOpcode(opcode)),");
        });
    });

    s.wln(format!("{}", it.cfg()));
    s.bodyn(format!("{func}fn {prefix}read_encrypted<R: {read}, D: Decrypter{decrypter}>(r: &mut R, d: &mut D) -> std::result::Result<Self, Self::Error>", func = it.func(), prefix = it.prefix(), read = it.read(), decrypter = it.decrypter()), |s| {
        s.wln(format!("let mut header = [0u8; {header_size}];", header_size = opcode_size + 2));
        s.wln(format!("r.read_exact(&mut header){postfix}?;", postfix = it.postfix()));
        s.wln(format!("let header = d.decrypt_{cd}_header(header);", cd = cd));
        s.wln(format!("let header_size = (header.size - {opcode_size}) as u32;", opcode_size = opcode_size));
        s.body("match header.opcode", |s| {
            for &e in v {
                let opcode = match e.container_type() {
                    ContainerType::CMsg(i) => i,
                    ContainerType::SMsg(i) => i,
                    ContainerType::Msg(i) => i,
                    _ => panic!(),
                };

                s.wln(format!("{opcode:#06X} => Ok(Self::{enum_name}({name}::{prefix}read_body(r, header_size){postfix}?)),", prefix = it.prefix(), postfix = it.postfix(), opcode = opcode, name = e.name(), enum_name = get_enumerator_name(e.name())));
            }
            s.wln("_ => Err(Self::Error::InvalidOpcode(header.opcode)),");
        });
    });

    s.wln(format!("{}", it.cfg()));
    s.bodyn(format!("{func}fn {prefix}write_encrypted<W: {write}, E: Encrypter{decrypter}>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error>", decrypter = it.decrypter(), prefix = it.prefix(), func = it.func(), write = it.write()), |s| {
        s.body("match self", |s| {
            for &e in v {
                s.wln(format!("Self::{name}(i) => i.{prefix}write_encrypted_{cd}(w, e){postfix}?,", prefix = it.prefix(), postfix = it.postfix(), name = get_enumerator_name(e.name()), cd = cd));
            }
        });
        s.wln("Ok(())");
    });
}

pub fn common_impls_world(
    s: &mut Writer,
    v: &[&Container],
    ty: &str,
    container_type: ContainerType,
) {
    let (cd, size, opcode_size) = match container_type {
        ContainerType::CMsg(_) => ("client", "u32", 4),
        ContainerType::SMsg(_) => ("server", "u16", 2),
        _ => panic!(),
    };
    s.wln(ASYNC_TRAIT_MACRO);
    s.impl_for(
        OPCODE_MESSAGE_TRAIT_NAME,
        format!("{t}OpcodeMessage", t = ty),
        |s| {
            s.wln(format!("type Error = {t}OpcodeMessageError;", t = ty));

            for it in ImplType::types() {
                world_common_impls_read_write(s, v, cd, size, opcode_size, it);
            }
        },
    );
}

pub fn common_impls_login(s: &mut Writer, v: &[&Container], ty: &str) {
    s.impl_readable_and_writable(
        format!("{t}OpcodeMessage", t = ty),
        |s, it| {
            s.wln(format!("let opcode = {t}Opcode::{prefix}read(r){postfix}?;", t = ty, prefix = it.prefix(), postfix = it.postfix()));

            s.body("match opcode", |s| {
                for e in v {
                    s.wln(format!(
                        "{t}Opcode::{enum_name} => Ok(Self::{enum_name}({name}::{prefix}read(r){postfix}?)),",
                        name = e.name(),
                        enum_name = get_enumerator_name(e.name()),
                        t = ty,
                        prefix = it.prefix(),
                        postfix = it.postfix(),
                    ));
                }
            });
        },
        |s, it| {
            s.wln(format!("{t}Opcode::from(self).{prefix}write(w){postfix}?;\n", t = ty, prefix = it.prefix(), postfix = it.postfix()));
            s.bodyn("match self", |s| {
                for e in v {
                    s.wln(format!(
                        "Self::{enum_name}(e) => e.{prefix}write(w){postfix}?,",
                        enum_name = get_enumerator_name(e.name()),
                        prefix = it.prefix(), postfix = it.postfix(),
                    ));
                }
            });

            s.wln("Ok(())");
        },
    );
}

pub fn print_error(s: &mut Writer, v: &[&Container], ty: &str, int_ty: &str) {
    s.wln("#[derive(Debug)]");
    s.new_enum("pub", format!("{t}OpcodeMessageError", t = ty), |s| {
        s.wln("Io(std::io::Error),");
        s.wln(format!("InvalidOpcode({int_ty}),", int_ty = int_ty));
        for e in v {
            if e.only_has_io_errors() {
                continue;
            }

            s.wln(format!(
                "{enum_name}({name}Error),",
                enum_name = get_enumerator_name(e.name()),
                name = e.name()
            ));
        }
    });

    s.wln(format!(
        "impl std::error::Error for {t}OpcodeMessageError {{}}",
        t = ty
    ));

    s.impl_for("std::fmt::Display", format!("{t}OpcodeMessageError", t = ty), |s| {
        s.body("fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result", |s| {
            s.body("match self", |s| {
                s.wln("Self::Io(i) => i.fmt(f),");
                s.wln(format!(r#"Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode received for {ty}Message: '{{}}'", i)),"#, ty = ty));
                for e in v {
                    if e.only_has_io_errors() {
                        continue;
                    }

                    s.wln(format!("Self::{enum_name}(i) => i.fmt(f),", enum_name = get_enumerator_name(e.name())));
                }
            });

        });
    });

    s.impl_from(
        "std::io::Error",
        format!("{t}OpcodeMessageError", t = ty),
        |s| {
            s.wln("Self::Io(e)");
        },
    );

    s.impl_from(
        format!("{t}OpcodeError", t = ty),
        format!("{t}OpcodeMessageError", t = ty),
        |s| {
            s.body("match e", |s| {
                s.wln(format!("{t}OpcodeError::Io(i) => Self::Io(i),", t = ty));
                s.wln(format!(
                    "{t}OpcodeError::InvalidOpcode(i) => Self::InvalidOpcode(i),",
                    t = ty
                ));
            });
        },
    );

    for e in v {
        if e.only_has_io_errors() {
            continue;
        }

        s.impl_from(
            format!("{}Error", e.name()),
            format!("{t}OpcodeMessageError", t = ty),
            |s| {
                s.body("match e", |s| {
                    s.wln(format!(
                        "{name}Error::Io(i) => Self::Io(i),",
                        name = e.name()
                    ));
                    s.wln(format!(
                        "_ => Self::{enum_name}(e),",
                        enum_name = get_enumerator_name(e.name())
                    ));
                });
            },
        );
    }
}

pub fn opcode_enum_world(
    s: &mut Writer,
    v: &[&Container],
    ty: &str,
    container_type: ContainerType,
) {
    s.wln("#[derive(Debug)]");
    s.new_enum("pub", format!("{t}Opcode", t = ty), |s| {
        for e in v {
            s.wln(format!(
                "{enum_name},",
                enum_name = get_enumerator_name(e.name())
            ));
        }
    });
    let int_ty = match container_type {
        ContainerType::CMsg(_) => "u32",
        ContainerType::SMsg(_) => "u16",
        _ => panic!(),
    };

    s.bodyn(format!("impl {t}Opcode", t = ty), |s| {
        s.funcn_pub_const(
            format!("as_{int_ty}(&self)", int_ty = int_ty).as_str(),
            int_ty,
            |s| {
                s.body("match self", |s| {
                    for e in v {
                        s.wln(format!(
                            "Self::{enum_name} => {value:#04x},",
                            enum_name = get_enumerator_name(e.name()),
                            value = match e.container_type() {
                                ContainerType::SMsg(i)
                                | ContainerType::CMsg(i)
                                | ContainerType::Msg(i) => i,
                                _ => panic!("invalid type for opcode enum"),
                            }
                        ))
                    }
                });
            },
        );
    });

    s.bodyn(&format!("impl {t}Opcode", t = ty), |s| {
        s.bodyn(
            format!(
                "pub fn new(opcode: {int_ty}) -> std::result::Result<Self, {t}OpcodeError>",
                int_ty = int_ty,
                t = ty
            ),
            |s| {
                s.body("match opcode", |s| {
                    for e in v {
                        s.wln(format!(
                            "{value:#04x} => Ok(Self::{enum_name}),",
                            enum_name = get_enumerator_name(e.name()),
                            value = match e.container_type() {
                                ContainerType::SMsg(i)
                                | ContainerType::CMsg(i)
                                | ContainerType::Msg(i) => i,
                                _ => panic!("invalid type for opcode enum"),
                            }
                        ));
                    }

                    s.wln(format!(
                        "opcode => Err({t}OpcodeError::InvalidOpcode(opcode)),",
                        t = ty
                    ));
                });
            },
        );
    });

    s.impl_from(
        format!("&{t}OpcodeMessage", t = ty),
        format!("{t}Opcode", t = ty),
        |s| {
            s.body("match *e", |s| {
                for e in v {
                    s.wln(format!(
                        "{t}OpcodeMessage::{enum_name}(_) => Self::{enum_name},",
                        t = ty,
                        enum_name = get_enumerator_name(e.name()),
                    ));
                }
            });
        },
    );

    s.wln("#[derive(Debug)]");
    s.new_enum("pub", format!("{t}OpcodeError", t = ty), |s| {
        s.wln("Io(std::io::Error),");
        s.wln(format!("InvalidOpcode({int_ty}),", int_ty = int_ty));
    });

    s.impl_for(
        "std::error::Error",
        format!("{t}OpcodeError", t = ty),
        |_| {},
    );

    s.impl_for(
        "std::fmt::Display",
        format!("{t}OpcodeError", t = ty),
        |s| {
            s.body(
                "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result",
                |s| {
                    s.body("match self", |s| {
                        s.wln("Self::Io(i) => i.fmt(f),");
                        s.wln(format!(r#"Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode for {t}: '{{}}'", i)),"#, t = ty));
                    });
                },
            );
        },
    );

    s.impl_from("std::io::Error", format!("{t}OpcodeError", t = ty), |s| {
        s.wln("Self::Io(e)")
    });
}

pub fn opcode_enum_login(s: &mut Writer, v: &[&Container], ty: &str) {
    s.wln("#[derive(Debug)]");
    s.new_enum("pub", format!("{t}Opcode", t = ty), |s| {
        for e in v {
            s.wln(format!(
                "{enum_name},",
                enum_name = get_enumerator_name(e.name())
            ));
        }
    });

    s.bodyn(format!("impl {t}Opcode", t = ty), |s| {
        s.funcn_pub_const("as_u8(&self)", "u8", |s| {
            s.body("match self", |s| {
                for e in v {
                    s.wln(format!(
                        "Self::{enum_name} => {value:#04x},",
                        enum_name = get_enumerator_name(e.name()),
                        value = match e.container_type() {
                            ContainerType::CLogin(i) | ContainerType::SLogin(i) => i,
                            ContainerType::SMsg(i) | ContainerType::CMsg(i) => i,
                            _ => panic!("invalid type for opcode enum"),
                        }
                    ))
                }
            });
        });
    });

    s.impl_readable_and_writable(
        &format!("{t}Opcode", t = ty),
        |s, it| {
            s.wln(format!(
                "let opcode = crate::util::{prefix}read_u8_le(r){postfix}?;\n",
                prefix = it.prefix(),
                postfix = it.postfix()
            ));

            s.body("match opcode", |s| {
                for e in v {
                    s.wln(format!(
                        "{value:#04x} => Ok(Self::{enum_name}),",
                        enum_name = get_enumerator_name(e.name()),
                        value = match e.container_type() {
                            ContainerType::CLogin(i) | ContainerType::SLogin(i) => i,
                            ContainerType::SMsg(i) | ContainerType::CMsg(i) => i,
                            _ => panic!("invalid type for opcode enum"),
                        }
                    ));
                }

                s.wln(format!(
                    "opcode => Err({t}OpcodeError::InvalidOpcode(opcode)),",
                    t = ty
                ));
            });
        },
        |s, it| {
            s.wln(format!(
                "crate::util::{prefix}write_u8_le(w, self.as_u8()){postfix}?;",
                prefix = it.prefix(),
                postfix = it.postfix()
            ));
            s.wln("Ok(())");
        },
    );

    s.impl_from(
        format!("&{t}OpcodeMessage", t = ty),
        format!("{t}Opcode", t = ty),
        |s| {
            s.body("match *e", |s| {
                for e in v {
                    s.wln(format!(
                        "{t}OpcodeMessage::{enum_name}(_) => Self::{enum_name},",
                        t = ty,
                        enum_name = get_enumerator_name(e.name()),
                    ));
                }
            });
        },
    );

    s.wln("#[derive(Debug)]");
    s.new_enum("pub", format!("{t}OpcodeError", t = ty), |s| {
        s.wln("Io(std::io::Error),");
        s.wln("InvalidOpcode(u8),");
    });

    s.impl_for(
        "std::fmt::Display",
        format!("{t}OpcodeError", t = ty),
        |s| {
            s.body(
                "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result",
                |s| {
                    s.body("match self", |s| {
                        s.wln("Self::Io(i) => i.fmt(f),");
                        s.wln(format!(r#"Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode for {t}: '{{}}'", i)),"#, t = ty));
                    });
                },
            );
        },
    );

    s.impl_for(
        "std::error::Error",
        format!("{t}OpcodeError", t = ty),
        |_| {},
    );

    s.impl_from("std::io::Error", format!("{t}OpcodeError", t = ty), |s| {
        s.wln("Self::Io(e)")
    });
}

pub fn get_enumerator_name(name: &str) -> String {
    name.replace("_Client", "").replace("_Server", "")
}
