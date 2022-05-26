use crate::container::{Container, ContainerType};
use crate::file_utils::{get_import_path, get_login_logon_version_path, get_world_version_path};
use crate::parser::types::tags::{LoginVersion, WorldVersion};
use crate::rust_printer::{
    ImplType, Writer, ASYNC_STD_IMPORT, CFG_ASYNC_ASYNC_STD, CFG_ASYNC_TOKIO,
    CLIENT_MESSAGE_TRAIT_NAME, SERVER_MESSAGE_TRAIT_NAME, TOKIO_IMPORT,
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

    definition(&mut s, v, ty);

    common_impls_world(&mut s, v, ty, container_type);

    print_error(&mut s, ty, int_ty);

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

    print_error(&mut s, ty, int_ty);

    s
}

pub fn includes(s: &mut Writer, v: &[&Container], container_type: ContainerType) {
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

fn world_common_impls_read_opcodes(
    s: &mut Writer,
    v: &[&Container],
    size: &str,
    error_ty: &str,
    ty: &str,
) {
    s.bodyn(format!("fn read_opcodes(opcode: {size}, body_size: u32, mut r: &[u8]) -> std::result::Result<Self, {error_ty}>"), |s| {
        s.open_curly("match opcode");

        for &e in v {
            let opcode = match e.container_type() {
                ContainerType::CMsg(i) => i,
                ContainerType::SMsg(i) => i,
                ContainerType::Msg(i) => i,
                _ => panic!(),
            };
            s.wln(format!("{opcode:#06X} => Ok(Self::{enum_name}(<{name} as {impl_trait}Message>::read_body(&mut r, body_size)?)),",
                          opcode = opcode,
                          name = e.name(),
                          impl_trait = ty,
                          enum_name = get_enumerator_name(e.name())));
        }
        s.wln(format!("_ => Err({error_ty}::InvalidOpcode(opcode)),", error_ty = error_ty));

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
        "let size = ({path}::{prefix}read_u16_be(r){postfix}? - {opcode_size}) as u32;",
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
        "let mut header = [0u8; {header_size}];",
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
        "let body_size = (header.size - {opcode_size}) as u32;",
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
        world_common_impls_read_opcodes(s, v, size, &format!("{t}OpcodeMessageError", t = ty), ty);

        for it in ImplType::types() {
            world_common_impls_read_write(
                s,
                cd,
                size,
                opcode_size,
                &format!("{t}OpcodeMessageError", t = ty),
                it,
            );
        }
    });
}

pub fn common_impls_login(s: &mut Writer, v: &[&Container], ty: &str) {
    s.impl_read_write_non_trait_pub(
        format!("{t}OpcodeMessage", t = ty),
        format!("{t}OpcodeMessageError", t = ty),
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

                s.wln(format!("opcode => Err({t}OpcodeMessageError::InvalidOpcode(opcode)),", t = ty));
            });
        },
        |s, _it| {
            s.bodyn("match self", |s| {
                for e in v {
                    s.wln(format!(
                        "Self::{enum_name}(e) => std::io::Write::write_all(&mut w, &e.as_bytes()?)?,",
                        enum_name = get_enumerator_name(e.name()),
                    ));
                }
            });
        },
        None
    );
}

pub fn print_error(s: &mut Writer, ty: &str, int_ty: &str) {
    s.wln("#[derive(Debug)]");
    s.new_enum("pub", format!("{t}OpcodeMessageError", t = ty), |s| {
        s.wln("Io(std::io::Error),");
        s.wln(format!("InvalidOpcode({int_ty}),", int_ty = int_ty));
        s.wln("String(std::string::FromUtf8Error),");
        s.wln("Enum(crate::errors::EnumError),");
    });

    s.wln(format!(
        "impl std::error::Error for {t}OpcodeMessageError {{}}",
        t = ty
    ));

    s.impl_for("std::fmt::Display", format!("{t}OpcodeMessageError", t = ty), |s| {
        s.body("fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result", |s| {
            s.body("match self", |s| {
                s.wln("Self::Io(i) => i.fmt(f),");
                s.wln("Self::String(i) => i.fmt(f),");
                s.wln("Self::Enum(i) => i.fmt(f),");
                s.wln(format!(r#"Self::InvalidOpcode(i) => f.write_fmt(format_args!("invalid opcode received for {ty}Message: '{{}}'", i)),"#, ty = ty));
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
        "crate::errors::ParseError",
        format!("{t}OpcodeMessageError", t = ty),
        |s| {
            s.body("match e", |s| {
                s.wln("crate::errors::ParseError::Io(i) => Self::Io(i),");
                s.wln("crate::errors::ParseError::Enum(i) => Self::Enum(i),");
                s.wln("crate::errors::ParseError::String(i) => Self::String(i),");
            });
        },
    );
}

pub fn get_enumerator_name(name: &str) -> String {
    name.replace("_Client", "").replace("_Server", "")
}
