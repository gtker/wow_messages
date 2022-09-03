use crate::container::{Container, ContainerType};
use crate::file_utils::{get_import_path, get_login_logon_version_path, get_world_version_path};
use crate::parser::types::tags::{LoginVersion, WorldVersion};
use crate::rust_printer::{
    ImplType, Version, Writer, ASYNC_STD_IMPORT, CFG_ASYNC_ASYNC_STD, CFG_ASYNC_TOKIO,
    CLIENT_MESSAGE_TRAIT_NAME, EXPECTED_OPCODE_ERROR, SERVER_MESSAGE_TRAIT_NAME, TOKIO_IMPORT,
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
    let ty = match container_type {
        ContainerType::SMsg(_) => SMSG_NAME,
        ContainerType::CMsg(_) => CMSG_NAME,
        _ => panic!("invalid type passed to opcode printer"),
    };

    includes(&mut s, v, container_type, Version::World(*version));

    definition(&mut s, v, ty);

    common_impls_world(&mut s, v, ty, container_type);

    s
}

pub fn print_login_opcodes(
    v: &[&Container],
    version: &LoginVersion,
    container_type: ContainerType,
) -> Writer {
    let mut s = Writer::new(&get_login_logon_version_path(version));
    let ty = match container_type {
        ContainerType::CLogin(_) => CLOGIN_NAME,
        ContainerType::SLogin(_) => SLOGIN_NAME,
        _ => panic!("invalid type passed to opcode printer"),
    };

    includes(&mut s, v, container_type, Version::Login(*version));

    definition(&mut s, v, ty);

    common_impls_login(&mut s, v, ty);

    s
}

fn any_container_is_pure_movement_info(v: &[&Container]) -> bool {
    v.iter().any(|a| a.is_pure_movement_info())
}

pub fn includes(s: &mut Writer, v: &[&Container], container_type: ContainerType, version: Version) {
    match container_type {
        ContainerType::SLogin(_) => {
            s.wln(format!(
                "use crate::{{{}, {}}};",
                SERVER_MESSAGE_TRAIT_NAME, CLIENT_MESSAGE_TRAIT_NAME
            ));

            s.wln(CFG_ASYNC_TOKIO);
            s.wln(TOKIO_IMPORT);
            s.wln(CFG_ASYNC_ASYNC_STD);
            s.wln(ASYNC_STD_IMPORT);
        }
        ContainerType::CMsg(_) => {
            s.wln(format!(
                "use crate::{{{}, {}}};",
                SERVER_MESSAGE_TRAIT_NAME, CLIENT_MESSAGE_TRAIT_NAME,
            ));
            s.wln("use wow_srp::header_crypto::{Decrypter, Encrypter};");

            s.newline();

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
        }
        _ => {}
    }

    for e in v {
        if let ContainerType::SMsg(_) = container_type {
            if let ContainerType::Msg(_) = e.container_type() {
                continue;
            }
        }

        let import_version = match e.tags().import_version() {
            Version::Login(l) => l.into(),
            // TODO: World does not deduplicate same types
            Version::World(_) => version,
        };
        let module_name = get_import_path(import_version);
        s.wln(format!(
            "use {module_name}::{name};",
            module_name = module_name,
            name = e.name(),
        ));
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

fn world_common_impls_read_opcodes(s: &mut Writer, v: &[&Container], size: &str, error_ty: &str) {
    s.bodyn(format!("fn read_opcodes(opcode: {size}, body_size: u32, mut r: &[u8]) -> std::result::Result<Self, {error_ty}>"), |s| {
        s.open_curly("match opcode");

        for &e in v {
            let opcode = match e.container_type() {
                ContainerType::CMsg(i) => i,
                ContainerType::SMsg(i) => i,
                ContainerType::Msg(i) => i,
                _ => panic!(),
            };
            s.wln(format!("{opcode:#06X} => Ok(Self::{enum_name}(<{name} as crate::Message>::read_body(&mut r, body_size)?)),",
                          opcode = opcode,
                          name = e.name(),
                          enum_name = get_enumerator_name(e.name())));
        }

        let opcode_text = if size == "u32" {
            "opcode"
        } else {
            "opcode: opcode.into()"
        };
        s.wln(format!("_ => Err({error_ty}::Opcode{{ {opcode_text}, size: body_size }}),", error_ty = error_ty, opcode_text = opcode_text));

        s.closing_curly(); // match opcode
    });
}

fn world_common_impls_read_write(
    s: &mut Writer,
    cd: &str,
    size: &str,
    opcode_size: i32,
    error_ty: &str,
    it: ImplType,
) {
    s.wln(it.cfg());
    s.open_curly(format!(
        "pub {func}fn {prefix}read_unencrypted<R: {read}>(r: &mut R) -> std::result::Result<Self, {error_ty}>",
        prefix = it.prefix(),
        read = it.read(),
        func = it.func(),
        error_ty = error_ty,
    ));
    s.wln(format!(
        "let size = ({path}::{prefix}read_u16_be(r){postfix}?.saturating_sub({opcode_size})) as u32;",
        path = "crate::util",
        opcode_size = opcode_size,
        prefix = it.prefix(),
        postfix = it.postfix()
    ));

    s.wln(format!(
        "let opcode = {path}::{prefix}read_{size}_le(r){postfix}?;",
        path = "crate::util",
        size = size,
        prefix = it.prefix(),
        postfix = it.postfix()
    ));
    s.newline();

    s.wln("let mut buf = vec![0; size as usize];");
    s.wln(format!(
        "r.read_exact(&mut buf){postfix}?;",
        postfix = it.postfix()
    ));

    s.wln("Self::read_opcodes(opcode, size, &buf)");

    s.closing_curly();

    s.wln(it.cfg());
    s.open_curly(
        format!("pub {func}fn {prefix}read_encrypted<R: {read}, D: Decrypter{decrypter}>(r: &mut R, d: &mut D) -> std::result::Result<Self, {error_ty}>",
                func = it.func(),
                prefix = it.prefix(),
                read = it.read(),
                decrypter = it.decrypter(),
                error_ty = error_ty,
        )
    );

    s.wln(format!(
        "let mut header = [0_u8; {header_size}];",
        header_size = opcode_size + 2
    ));
    s.wln(format!(
        "r.read_exact(&mut header){postfix}?;",
        postfix = it.postfix()
    ));
    s.wln(format!(
        "let header = d.decrypt_{cd}_header(header);",
        cd = cd
    ));
    s.wln(format!(
        "let body_size = (header.size.saturating_sub({opcode_size})) as u32;",
        opcode_size = opcode_size
    ));
    s.newline();

    s.wln("let mut buf = vec![0; body_size as usize];");
    s.wln(format!(
        "r.read_exact(&mut buf){postfix}?;",
        postfix = it.postfix()
    ));

    s.wln("Self::read_opcodes(header.opcode, body_size, &buf)");

    s.closing_curly_newline();
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
    s.bodyn(format!("impl {t}OpcodeMessage", t = ty), |s| {
        world_common_impls_read_opcodes(s, v, size, EXPECTED_OPCODE_ERROR);

        for it in ImplType::types() {
            world_common_impls_read_write(s, cd, size, opcode_size, EXPECTED_OPCODE_ERROR, it);
        }

        for it in ImplType::types() {
            world_inner(s, v, cd, it);
        }

        if any_container_is_pure_movement_info(v) {
            world_movement_info(s, v);
        }
    });

    for &e in v {
        s.impl_for(
            format!("From<{}>", e.name()),
            format!("{t}OpcodeMessage", t = ty),
            |s| {
                s.body(format!("fn from(c: {}) -> Self", e.name()), |s| {
                    s.wln(format!("Self::{}(c)", get_enumerator_name(e.name())));
                });
            },
        );
    }
}

fn world_inner(s: &mut Writer, v: &[&Container], cd: &str, it: ImplType) {
    s.wln(it.cfg());
    s.bodyn(
        format!(
            "pub {func}fn {prefix}write_encrypted_{cd}<W: {write}, E: wow_srp::header_crypto::Encrypter{decrypter}>(&self, w: &mut W, e: &mut E) -> Result<(), std::io::Error>",
            cd = cd,
            func = it.func(),
            write = it.write(),
            prefix = it.prefix(),
            decrypter = it.decrypter(),
        ), |s| {
            s.body("match self", |s| {
                for container in v {
                    s.wln(format!("Self::{en}(c) => c.{prefix}write_encrypted_{cd}(w, e){postfix},",
                                  en = get_enumerator_name(container.name()),
                                  prefix = it.prefix(),
                                  postfix = it.postfix(),
                    ));
                }
            });
        });

    s.wln(it.cfg());
    s.bodyn(
        format!(
            "pub {func}fn {prefix}write_unencrypted_{cd}<W: {write}>(&self, w: &mut W) -> Result<(), std::io::Error>",
            cd = cd,
            func = it.func(),
            write = it.write(),
            prefix = it.prefix(),
        ), |s| {
            s.body("match self", |s| {
                for container in v {
                    s.wln(format!("Self::{en}(c) => c.{prefix}write_unencrypted_{cd}(w){postfix},",
                                  en = get_enumerator_name(container.name()),
                                  prefix = it.prefix(),
                                  postfix = it.postfix())
                    );
                }
            });
        },
    );
}

fn world_movement_info(s: &mut Writer, v: &[&Container]) {
    s.funcn_pub("movement_info(&self)", "Option<&MovementInfo>", |s| {
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

pub fn common_impls_login(s: &mut Writer, v: &[&Container], ty: &str) {
    s.impl_read_write_opcode(
        format!("{t}OpcodeMessage", t = ty),
        EXPECTED_OPCODE_ERROR,
        |s, it| {
            s.wln(format!("let opcode = crate::util::{prefix}read_u8_le(r){postfix}?;", prefix = it.prefix(), postfix = it.postfix()));

            s.body("match opcode", |s| {
                for e in v {
                    let opcode = match e.container_type() {
                        ContainerType::CLogin(opcode) |
                        ContainerType::SLogin(opcode) => opcode,
                        _ => unreachable!()
                    };

                    s.wln(format!(
                        "{opcode:#04X} => Ok(Self::{enum_name}({name}::{prefix}read(r){postfix}?)),",
                        name = e.name(),
                        enum_name = get_enumerator_name(e.name()),
                        opcode = opcode,
                        prefix = it.prefix(),
                        postfix = it.postfix(),
                    ));
                }

                s.wln(format!("opcode => Err({}::Opcode(opcode as u32)),", EXPECTED_OPCODE_ERROR));
            });
        },
        |s, _it| {
            s.bodyn("match self", |s| {
                for e in v {
                    s.wln(format!(
                        "Self::{enum_name}(e) => e.write_into_vec(w)?,",
                        enum_name = get_enumerator_name(e.name()),
                    ));
                }
            });
        },
        true,
    );

    for &e in v {
        s.impl_for(
            format!("From<{}>", e.name()),
            format!("{t}OpcodeMessage", t = ty),
            |s| {
                s.body(format!("fn from(c: {}) -> Self", e.name()), |s| {
                    s.wln(format!("Self::{}(c)", get_enumerator_name(e.name())));
                });
            },
        );
    }
}

pub fn get_enumerator_name(name: &str) -> String {
    name.replace("_Client", "").replace("_Server", "")
}
