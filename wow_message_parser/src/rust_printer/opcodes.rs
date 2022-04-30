use crate::container::{Container, ContainerType};
use crate::file_utils::{get_import_path, get_login_logon_version_path, get_world_version_path};
use crate::parser::types::tags::{LoginVersion, WorldVersion};
use crate::rust_printer::{
    Writer, ASYNC_TRAIT, ASYNC_TRAIT_IMPORT, CFG_ASYNC_ANY, CFG_ASYNC_TOKIO, TOKIO_IMPORT,
    WORLD_BODY_TRAIT_NAME, WORLD_CLIENT_HEADER_TRAIT_NAME, WORLD_SERVER_HEADER_TRAIT_NAME,
};

const CLOGIN_NAME: &str = "Client";
const SLOGIN_NAME: &str = "Server";
const CMSG_NAME: &str = "WorldClient";
const SMSG_NAME: &str = "WorldServer";

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
            s.wln(format!("use crate::{};", ASYNC_TRAIT));
            s.wln(CFG_ASYNC_ANY);
            s.wln(ASYNC_TRAIT_IMPORT);
            s.wln(CFG_ASYNC_TOKIO);
            s.wln(TOKIO_IMPORT);
        }
        ContainerType::CMsg(_) => {
            s.wln(format!("use crate::{};", WORLD_BODY_TRAIT_NAME));
            s.wln("use crate::WorldMessage;");
            s.wln(format!(
                "use crate::{{{}, {}}};",
                WORLD_SERVER_HEADER_TRAIT_NAME, WORLD_CLIENT_HEADER_TRAIT_NAME,
            ));
            s.wln("use wow_srp::header_crypto::{Decrypter, Encrypter};");
            s.newline();
            s.wln(CFG_ASYNC_ANY);
            s.wln(format!("use crate::{};", ASYNC_TRAIT));
            s.wln(CFG_ASYNC_ANY);
            s.wln(ASYNC_TRAIT_IMPORT);
            s.wln(CFG_ASYNC_TOKIO);
            s.wln(TOKIO_IMPORT);
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
    s.new_enum(format!("{t}OpcodeMessage", t = ty), |s| {
        for &e in v {
            s.wln(format!(
                "{enum_name}({name}),",
                enum_name = get_enumerator_name(e.name()),
                name = e.name()
            ));
        }
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
    s.impl_for("WorldMessage", format!("{t}OpcodeMessage", t = ty), |s| {
        s.wln(format!("type Error = {t}OpcodeMessageError;", t = ty));

        s.bodyn("fn write_unencrypted<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error>", |s| {
            s.body("match self", |s| {
                for &e in v {
                    s.wln(format!("Self::{name}(i) => i.write_body(w)?,",
                                  name = get_enumerator_name(e.name())));
                }
            });
            s.wln("Ok(())");
        });

        s.bodyn("fn read_unencrypted<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error>", |s| {
            s.wln(format!("let size = {path}::read_u16_be(r)?;",path = "crate::util"));
            s.wln(format!("let opcode = {path}::read_{size}_le(r)?;",path = "crate::util", size = size));
            s.body("match opcode", |s| {
                for &e in v {
                    let opcode = match e.container_type() {
                        ContainerType::CMsg(i) => i,
                        ContainerType::SMsg(i) => i,
                        ContainerType::Msg(i) => i,
                        _ => panic!(),
                    };
                    s.wln(format!("{opcode:#04x} => Ok(Self::{enum_name}({name}::read_body(r, (size - {opcode_size}) as u32)?)),",
                                opcode = opcode,
                                opcode_size = opcode_size,
                                name = e.name(),
                                enum_name = get_enumerator_name(e.name())));
                }
                s.wln("_ => Err(Self::Error::InvalidOpcode(opcode)),");
            });
        });

        s.bodyn("fn read_encrypted<R: std::io::Read, D: Decrypter>(r: &mut R, d: &mut D) -> std::result::Result<Self, Self::Error>", |s| {
            s.wln(format!("let header = d.read_and_decrypt_{cd}_header(r)?;", cd = cd));
            s.body("match header.opcode", |s| {
                for &e in v {
                    let opcode = match e.container_type() {
                        ContainerType::CMsg(i) => i,
                        ContainerType::SMsg(i) => i,
                        ContainerType::Msg(i) => i,
                        _ => panic!(),
                    };
                    s.wln(format!("{opcode:#04x} => Ok(Self::{enum_name}({name}::read_body(r, (header.size - {opcode_size}) as u32)?)),", opcode = opcode, opcode_size = opcode_size, name = e.name(), enum_name = get_enumerator_name(e.name())));
                }
                s.wln("_ => Err(Self::Error::InvalidOpcode(header.opcode)),");
            });
        });

        s.bodyn("fn write_encrypted<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error>", |s| {
            s.body("match self", |s| {
                for &e in v {
                    s.wln(format!("Self::{name}(i) => i.write_encrypted_{cd}(w, e)?,", name = get_enumerator_name(e.name()), cd = cd));
                }
            });
            s.wln("Ok(())");
        });

    });
}

pub fn common_impls_login(s: &mut Writer, v: &[&Container], ty: &str) {
    s.impl_readable_and_writable(
        format!("{t}OpcodeMessage", t = ty),
        |s| {
            s.wln(format!("let opcode = {t}Opcode::read(r)?;", t = ty));

            s.body("match opcode", |s| {
                for e in v {
                    s.wln(format!(
                        "{t}Opcode::{enum_name} => Ok(Self::{enum_name}({name}::read(r)?)),",
                        name = e.name(),
                        enum_name = get_enumerator_name(e.name()),
                        t = ty,
                    ));
                }
            });
        },
        |s| {
            s.wln(format!("{t}Opcode::from(self).write(w)?;\n", t = ty));
            s.bodyn("match self", |s| {
                for e in v {
                    s.wln(format!(
                        "Self::{enum_name}(e) => e.write(w)?,",
                        enum_name = get_enumerator_name(e.name()),
                    ));
                }
            });

            s.wln("Ok(())");
        },
    );

    s.impl_async_readable_and_writable(
        format!("{t}OpcodeMessage", t = ty),
        |s| {
            s.wln(format!("let opcode = {t}Opcode::tokio_read(r).await?;", t = ty));

            s.body("match opcode", |s| {
                for e in v {
                    s.wln(format!(
                        "{t}Opcode::{enum_name} => Ok(Self::{enum_name}({name}::tokio_read(r).await?)),",
                        name = e.name(),
                        enum_name = get_enumerator_name(e.name()),
                        t = ty,
                    ));
                }
            });
        },
        |s| {
            s.wln(format!("{t}Opcode::from(self).tokio_write(w).await?;\n", t = ty));
            s.bodyn("match self", |s| {
                for e in v {
                    s.wln(format!(
                        "Self::{enum_name}(e) => e.tokio_write(w).await?,",
                        enum_name = get_enumerator_name(e.name()),
                    ));
                }
            });

            s.wln("Ok(())");
        },
    );
}

pub fn print_error(s: &mut Writer, v: &[&Container], ty: &str, int_ty: &str) {
    s.wln("#[derive(Debug)]");
    s.new_enum(format!("{t}OpcodeMessageError", t = ty), |s| {
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
    s.new_enum(format!("{t}Opcode", t = ty), |s| {
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
    s.new_enum(format!("{t}OpcodeError", t = ty), |s| {
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
    s.new_enum(format!("{t}Opcode", t = ty), |s| {
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
        |s| {
            s.wln("let opcode = crate::util::read_u8_le(r)?;\n");

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
        |s| {
            s.wln("crate::util::write_u8_le(w, self.as_u8())?;");
            s.wln("Ok(())");
        },
    );

    s.impl_async_readable_and_writable(
        &format!("{t}Opcode", t = ty),
        |s| {
            s.wln("let opcode = crate::util::tokio_read_u8_le(r).await?;\n");

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
        |s| {
            s.wln("crate::util::tokio_write_u8_le(w, self.as_u8()).await?;");
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
    s.new_enum(format!("{t}OpcodeError", t = ty), |s| {
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
