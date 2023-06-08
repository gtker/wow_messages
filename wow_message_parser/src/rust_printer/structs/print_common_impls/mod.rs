use crate::file_utils::get_import_path;
use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::objects::Objects;
use crate::parser::types::sizes::Sizes;
use crate::parser::types::ty::Type;
use crate::parser::types::version::{MajorWorldVersion, Version};
use crate::rust_printer::structs::print_common_impls::print_size::{
    print_size_rust_view, print_size_uncompressed_rust_view,
};
use crate::rust_printer::structs::test_case_string::print_to_testcase;
use crate::rust_printer::writer::Writer;
use crate::rust_printer::{
    ImplType, CFG_TESTCASE, CLIENT_MESSAGE_TRAIT_NAME, PARSE_ERROR, SERVER_MESSAGE_TRAIT_NAME,
};
use crate::CONTAINER_SELF_SIZE_FIELD;

pub mod print_read;
pub(crate) mod print_size;
pub mod print_write;

pub(crate) fn print_common_impls(s: &mut Writer, e: &Container, o: &Objects) {
    print_world_message_headers_and_constants(s, e);

    if !matches!(e.container_type(), ContainerType::Struct) {
        print_to_testcase(s, e, o);
    }

    match e.container_type() {
        ContainerType::Struct => {
            let create_async_reads = e.tags().has_login_version();

            let visibility = if e.tags().is_in_base() {
                "pub"
            } else {
                "pub(crate)"
            };

            let error_name = if e.only_has_io_errors() {
                "std::io::Error"
            } else {
                PARSE_ERROR
            };

            impl_read_write_non_trait(
                s,
                e.name(),
                error_name,
                |s, it| {
                    print_read::print_read(s, e, o, it.prefix(), it.postfix());
                },
                |s, it| {
                    print_write::print_write(s, e, o, it.prefix(), it.postfix());
                },
                visibility,
                visibility,
                create_async_reads,
            );
        }
        ContainerType::CLogin(opcode) | ContainerType::SLogin(opcode) => {
            let mut sizes = e.sizes();
            let opcode_size = 1;
            sizes.inc_both(opcode_size);

            let trait_to_impl = match e.container_type() {
                ContainerType::CLogin(_) => CLIENT_MESSAGE_TRAIT_NAME,
                ContainerType::SLogin(_) => SERVER_MESSAGE_TRAIT_NAME,
                _ => unreachable!("Login branch has non login container"),
            };

            impl_read_and_writable_login(
                s,
                e.name(),
                opcode,
                trait_to_impl,
                |s, it| {
                    print_read::print_read(s, e, o, it.prefix(), it.postfix());
                },
                |s, it| {
                    s.wln("// opcode: u8");
                    s.wln(format!(
                        "w.write_all(&Self::OPCODE.to_le_bytes()){postfix}?;",
                        postfix = it.postfix(),
                    ));
                    s.newline();

                    print_write::print_write(s, e, o, it.prefix(), it.postfix());
                },
                sizes,
                e.tests(o).is_empty(),
            );
        }
        ContainerType::Msg(opcode) | ContainerType::CMsg(opcode) | ContainerType::SMsg(opcode) => {
            let bind = |s: &mut Writer, container_type, version| {
                impl_world_server_or_client_message(
                    s,
                    e.name(),
                    container_type,
                    version,
                    e.tags().compressed() || e.contains_compressed_variable(),
                );
            };

            impl_world_message(
                s,
                e.name(),
                opcode,
                |s, it| {
                    print_write::print_write(s, e, o, it.prefix(), it.postfix());
                },
                |s, it| {
                    test_for_invalid_size(s, e);
                    print_read::print_read(s, e, o, it.prefix(), it.postfix());
                },
                Some(e.sizes()),
                e.tests(o).is_empty(),
            );

            for version in e.tags().main_versions() {
                match e.container_type() {
                    ContainerType::CMsg(_) => bind(s, ContainerType::CMsg(0), version),
                    ContainerType::SMsg(_) => bind(s, ContainerType::SMsg(0), version),
                    ContainerType::Msg(_) => {
                        bind(s, ContainerType::CMsg(0), version);

                        bind(s, ContainerType::SMsg(0), version);
                    }
                    _ => unreachable!("non world container in world branch"),
                }
            }
        }
    }

    print_size_rust_view(s, e, "self.");

    // Compressed messages need an additional size implementation that measures the non-compressed size of the message.
    // This is used to calculate the decompressed_size field, a u32 written at the start of every compressed packet.
    if e.tags().compressed() {
        print_size_uncompressed_rust_view(s, e.rust_object(), "self.", "size_uncompressed");
    }
}

pub(crate) fn impl_world_server_or_client_message(
    s: &mut Writer,
    type_name: impl AsRef<str>,
    container_type: ContainerType,
    version: Version,
    compressed: bool,
) {
    let (trait_to_impl, ty) = match container_type {
        ContainerType::CMsg(_) => (CLIENT_MESSAGE_TRAIT_NAME, "client"),
        ContainerType::SMsg(_) => (SERVER_MESSAGE_TRAIT_NAME, "server"),
        _ => unreachable!("login message in world server impl"),
    };

    s.wln(format!(
        "#[cfg(feature = \"{}\")]",
        version.as_major_world().feature_name()
    ));

    if compressed {
        s.open_curly(format!(
            "impl {}::{} for {}",
            get_import_path(version),
            trait_to_impl,
            type_name.as_ref(),
        ));

        let version = version.as_major_world();
        let feature_name = version.feature_name();

        let (size_cast, opcode_cast) = match container_type {
            ContainerType::CMsg(_) => match version {
                MajorWorldVersion::Vanilla | MajorWorldVersion::BurningCrusade => ("", ""),
                MajorWorldVersion::Wrath => ("", ""),
            },
            ContainerType::SMsg(_) => match version {
                MajorWorldVersion::Vanilla | MajorWorldVersion::BurningCrusade => ("", " as u16"),
                MajorWorldVersion::Wrath => (" as u32", " as u16"),
            },
            _ => unreachable!(),
        };

        let encrypter = match version {
            MajorWorldVersion::Vanilla => "wow_srp::vanilla_header::EncrypterHalf",
            MajorWorldVersion::BurningCrusade => "wow_srp::tbc_header::EncrypterHalf",
            MajorWorldVersion::Wrath => match container_type {
                ContainerType::CMsg(_) => "wow_srp::wrath_header::ClientEncrypterHalf",
                ContainerType::SMsg(_) => "wow_srp::wrath_header::ServerEncrypterHalf",
                _ => unreachable!(),
            },
        };

        s.wln("#[cfg(feature = \"sync\")]");
        s.open_curly(format!(
            "fn write_unencrypted_{ty}<W: Write>(&self, mut w: W) -> Result<(), std::io::Error>"
        ));

        print_unencrypted_body(s, "", feature_name, ty, version, &container_type);
        s.closing_curly_newline(); // fn write_unencrypted

        s.wln("#[cfg(all(feature = \"sync\", feature = \"encryption\"))]");
        s.wln(format!("fn write_encrypted_{ty}<W: Write>("));

        s.inc_indent();
        s.wln("&self,");
        s.wln("mut w: W,");
        s.wln(format!("e: &mut {encrypter},"));
        s.dec_indent();

        s.open_curly(") -> Result<(), std::io::Error>");
        print_encrypted_body(
            s,
            "",
            feature_name,
            ty,
            size_cast,
            opcode_cast,
            version,
            &container_type,
        );
        s.closing_curly_newline(); // ) -> Result

        for async_ty in ImplType::types() {
            if !async_ty.is_async() {
                continue;
            }

            let prefix = async_ty.prefix();
            s.wln(async_ty.cfg());
            s.wln(format!(
                "fn {prefix}write_unencrypted_{ty}<'s, 'async_trait, W>("
            ));
            s.inc_indent();
            s.wln("&'s self,");
            s.wln("mut w: W,");
            s.dec_indent();
            s.wln(") -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>");
            s.wln("where");

            s.inc_indent();
            let write = async_ty.write();
            s.wln(format!("W: 'async_trait + {write},"));
            s.wln("'s: 'async_trait,");
            s.wln("Self: Sync + 'async_trait,");
            s.dec_indent();

            s.open_curly(""); // fn body

            s.open_curly("Box::pin(async move");
            print_unencrypted_body(s, ".await", feature_name, ty, version, &container_type);
            s.closing_curly_with(")");

            s.closing_curly_newline(); // fn body

            s.wln(async_ty.cfg_and_encryption());
            s.wln(format!(
                "fn {prefix}write_encrypted_{ty}<'s, 'e, 'async_trait, W>("
            ));

            s.inc_indent();
            s.wln("&'s self,");
            s.wln("mut w: W,");
            s.wln(format!("e: &'e mut {encrypter},"));
            s.dec_indent();

            s.wln(") -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>");
            s.wln("where");

            s.inc_indent();
            s.wln(format!("W: 'async_trait + {write},"));
            s.wln("'s: 'async_trait,");
            s.wln("'e: 'async_trait,");
            s.wln("Self: Sync + 'async_trait,");
            s.dec_indent();

            s.open_curly(""); // fn body

            s.open_curly("Box::pin(async move");
            print_encrypted_body(
                s,
                ".await",
                feature_name,
                ty,
                size_cast,
                opcode_cast,
                version,
                &container_type,
            );
            s.closing_curly_with(")");

            s.closing_curly_newline(); // fn body
        }

        s.closing_curly(); // impl {} for {}
        s.newline();
    } else {
        s.wln(format!(
            "impl {}::{} for {} {{}}",
            get_import_path(version),
            trait_to_impl,
            type_name.as_ref(),
        ));
        s.newline();
    }
}

fn print_encrypted_body(
    s: &mut Writer,
    extra: &str,
    feature_name: &str,
    ty: &str,
    size_cast: &str,
    opcode_cast: &str,
    version: MajorWorldVersion,
    container_type: &ContainerType,
) {
    s.wln("let mut v = Vec::with_capacity(1024);");
    s.wln("let mut s = &mut v;");

    s.wln(format!(
        "crate::util::{feature_name}_get_unencrypted_{ty}(&mut s, Self::OPCODE as u16, 0)?;"
    ));

    s.wln("self.write_into_vec(&mut s)?;");

    print_header_size(s, version, container_type);

    s.wln("let size = v.len().saturating_sub(size_len) as u16;");

    s.wln(format!(
        "let header = e.encrypt_{ty}_header(size{size_cast}, Self::OPCODE{opcode_cast});"
    ));

    s.body("for (i, e) in header.iter().enumerate()", |s| {
        s.wln("v[i] = *e;");
    });
    s.wln(format!("w.write_all(&v){extra}"));
}

fn print_unencrypted_body(
    s: &mut Writer,
    extra: &str,
    feature_name: &str,
    ty: &str,
    version: MajorWorldVersion,
    container_type: &ContainerType,
) {
    s.wln("let mut v = Vec::with_capacity(1024);");
    s.wln("let mut s = &mut v;");

    s.wln(format!(
        "crate::util::{feature_name}_get_unencrypted_{ty}(&mut s, Self::OPCODE as u16, 0)?;"
    ));

    s.wln("self.write_into_vec(&mut s)?;");

    print_header_size(s, version, container_type);

    s.wln("let size = v.len().saturating_sub(size_len);");
    s.wln("let s = size.to_le_bytes();");

    s.wln("v[0] = s[1];");
    s.wln("v[1] = s[0];");

    match version {
        MajorWorldVersion::Vanilla | MajorWorldVersion::BurningCrusade => {}
        MajorWorldVersion::Wrath => {
            s.body("if size > 0x7FFF", |s| {
                s.wln("v[2] = s[2];");
            });
        }
    }

    s.wln(format!("w.write_all(&v){extra}"));
}

fn print_header_size(s: &mut Writer, version: MajorWorldVersion, container_type: &ContainerType) {
    match container_type {
        ContainerType::CMsg(_) => s.wln("let size_len = 2;"),
        ContainerType::SMsg(_) => match version {
            MajorWorldVersion::Vanilla | MajorWorldVersion::BurningCrusade => {
                s.wln("let size_len = 2;");
            }
            MajorWorldVersion::Wrath => {
                s.wln("let size_len = if v.len() > 0x7FFF { 3 } else { 2 };")
            }
        },
        _ => unreachable!(),
    }
}

fn test_for_invalid_size(s: &mut Writer, e: &Container) {
    let header = match e.is_constant_sized() {
        true => format!("if body_size != {}", e.sizes().maximum()),
        false => {
            let min = e.sizes().minimum();
            let max = match e.container_type() {
                ContainerType::CMsg(_) => {
                    // vmangos, mangos-tbc, trinitycore all use this max value
                    // mangos-tbc notes that 0x2800 is the largest buffer supported by the client
                    10240
                }
                ContainerType::Msg(_) | ContainerType::SMsg(_) => {
                    if e.tags().contains_wrath() {
                        0xFF_FF_FF // Wrath can have a variable length header size of 3 bytes
                    } else {
                        u16::MAX.into() // Non-wrath messages have a u16 size field
                    }
                }
                _ => unreachable!(),
            };

            let max = if e.sizes().maximum() >= u32::MAX as usize {
                max
            } else {
                e.sizes().maximum()
            };
            if min == 0 {
                format!("if body_size > {max}",)
            } else {
                format!("if !({min}..={max}).contains(&body_size)",)
            }
        }
    };
    s.bodyn(header, |s| {
        s.wln(format!(
            "return Err({}::InvalidSize {{ opcode: {:#06X}, size: body_size }});",
            PARSE_ERROR,
            e.opcode(),
        ));
    });
}

fn print_world_message_headers_and_constants(s: &mut Writer, e: &Container) {
    if e.any_fields_have_constant_value() {
        s.bodyn(format!("impl {name}", name = e.name()), |s| {
            for d in e.all_definitions() {
                if let Some(v) = d.value() {
                    if v.original_string() == CONTAINER_SELF_SIZE_FIELD {
                        continue;
                    }
                    print_constant_member(s, d.name(), d.ty(), v.original_string(), v.value());
                }
            }
        });
    }
}

pub(crate) fn print_constant_member(
    s: &mut Writer,
    name: &str,
    ty: &Type,
    original_value: &str,
    value: u64,
) {
    s.docc(format!("The field `{name}` is constantly specified to be:"));
    s.docc_newline();
    s.docc("| Format | Value |");
    s.docc("| ------ | ----- |");
    s.docc(format!("| Decimal | `{value}` |"));
    s.docc(format!("| Hex | `{value:#04x}` |"));
    s.docc(format!("| Original | `{original_value}` |"));
    s.docc_newline();
    s.docc("**This field is not in the Rust struct, but is written as this constant value.**");

    s.wln(format!(
        "pub const {name}_VALUE: {ty} = {value:#04x};\n",
        name = name.to_uppercase(),
        ty = ty.rust_str(),
        value = value,
    ));
}

pub(crate) fn impl_world_message(
    s: &mut Writer,
    type_name: impl AsRef<str>,
    opcode: u16,
    write_function: impl Fn(&mut Writer, ImplType),
    read_function: impl Fn(&mut Writer, ImplType),
    sizes: Option<Sizes>,
    should_write_test_case_string: bool,
) {
    let type_name = type_name.as_ref();

    s.wln(format!("impl crate::private::Sealed for {type_name} {{}}",));

    s.impl_for("crate::Message", type_name, |s| {
        s.wln(format!("const OPCODE: u32 = {opcode:#06x};"));
        s.newline();

        if should_write_test_case_string {
            s.wln(CFG_TESTCASE);
            s.bodyn("fn to_test_case_string(&self) -> Option<String>", |s| {
                s.wln(format!("{type_name}::to_test_case_string(self)"))
            });
        }

        s.bodyn("fn size_without_header(&self) -> u32", |s| {
            if sizes.is_some() && sizes.unwrap().is_constant().is_some() {
                s.wln(format!("{}", sizes.unwrap().maximum()));
            } else {
                s.wln("self.size() as u32");
            }
        });

        s.bodyn(
            "fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error>",
            |s| {
                write_function(s, ImplType::Std);

                s.wln("Ok(())");
            },
        );

        s.bodyn(format!(
            "fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, {PARSE_ERROR}>",
        ), |s| {
            read_function(s, ImplType::Std);
        });
    });
}

pub(crate) fn impl_read_and_writable_login(
    s: &mut Writer,
    type_name: impl AsRef<str>,
    opcode: u16,
    trait_to_impl: impl AsRef<str>,
    read_function: impl Fn(&mut Writer, ImplType),
    write_function: impl Fn(&mut Writer, ImplType),
    sizes: Sizes,
    should_write_to_test_case_string: bool,
) {
    let type_name = type_name.as_ref();
    let trait_to_impl = trait_to_impl.as_ref();

    write_into_vec(s, type_name, write_function, "pub(crate)");

    s.wln(format!("impl crate::private::Sealed for {type_name} {{}}",));
    s.newline();

    s.open_curly(format!("impl {trait_to_impl} for {type_name}",));
    s.wln(format!("const OPCODE: u8 = {opcode:#04x};"));
    s.newline();

    if should_write_to_test_case_string {
        s.wln(CFG_TESTCASE);
        s.bodyn("fn to_test_case_string(&self) -> Option<String>", |s| {
            s.wln(format!("{type_name}::to_test_case_string(self)"))
        });
    }

    for it in ImplType::types() {
        print_read_decl(s, it);

        read_function(s, it);
        if it.is_async() {
            s.closing_curly_with(")"); // Box::pin
        }
        s.closing_curly_newline();

        print_write_decl(s, it);

        call_as_bytes(s, it, sizes);

        if it.is_async() {
            s.closing_curly_with(")"); // Box::pin
        }

        s.closing_curly_newline(); // Write Function
    }

    s.closing_curly_newline(); // impl
}

pub(crate) fn impl_read_write_non_trait(
    s: &mut Writer,
    type_name: impl AsRef<str>,
    error_name: impl AsRef<str>,
    read_function: impl Fn(&mut Writer, ImplType),
    write_function: impl Fn(&mut Writer, ImplType),
    read_visibility: impl AsRef<str>,
    write_visibility: impl AsRef<str>,
    create_async_reads: bool,
) {
    write_into_vec(s, &type_name, write_function, read_visibility.as_ref());

    s.open_curly(format!("impl {}", type_name.as_ref()));

    for it in ImplType::types() {
        if it.is_async() {
            if !create_async_reads {
                continue;
            }

            s.wln(it.cfg());
        }
        s.open_curly(format!(
            "{visibility} {func}fn {prefix}read<R: {read}>(mut r: R) -> Result<Self, {error}>",
            prefix = it.prefix(),
            read = it.read(),
            error = error_name.as_ref(),
            func = it.func(),
            visibility = write_visibility.as_ref(),
        ));
        read_function(s, it);
        s.closing_curly_newline();
    }

    s.closing_curly_newline(); // impl
}

fn print_read_decl(s: &mut Writer, it: ImplType) {
    if !it.is_async() {
        s.open_curly(format!(
            "fn {prefix}read<R: {read}, I: crate::private::Sealed>(mut r: R) -> Result<Self, {error}>",
            prefix = it.prefix(),
            read = it.read(),
            error = PARSE_ERROR,
        ));

        return;
    }

    s.wln(it.cfg());
    s.wln(format!(
        "fn {}read<'async_trait, R, I: crate::private::Sealed>(",
        it.prefix()
    ));

    s.inc_indent();
    s.wln("mut r: R,");
    s.dec_indent();

    s.wln(") -> core::pin::Pin<Box<");

    s.inc_indent();
    s.wln(format!(
        "dyn core::future::Future<Output = Result<Self, {PARSE_ERROR}>>",
    ));
    s.inc_indent();

    s.wln("+ Send + 'async_trait,");
    s.dec_indent();
    s.dec_indent();

    s.wln(">> where");

    s.inc_indent();
    s.wln(format!("R: 'async_trait + {},", it.read()));
    s.wln("Self: 'async_trait,");
    s.dec_indent();

    s.open_curly("");
    s.open_curly("Box::pin(async move");
}

fn print_write_decl(s: &mut Writer, it: ImplType) {
    s.wln(it.cfg());

    if !it.is_async() {
        s.open_curly(format!(
            "fn {prefix}write<W: {write}>(&self, mut w: W) -> Result<(), std::io::Error>",
            prefix = it.prefix(),
            write = it.write(),
        ));

        return;
    }

    s.wln(format!(
        "fn {prefix}write<'life0, 'async_trait, W>(",
        prefix = it.prefix(),
    ));
    s.inc_indent();

    s.wln("&'life0 self,");
    s.wln("mut w: W,");
    s.dec_indent();

    s.wln(") -> core::pin::Pin<Box<");
    s.inc_indent();

    s.wln("dyn core::future::Future<Output = Result<(), std::io::Error>>");
    s.inc_indent();

    s.wln("+ Send + 'async_trait");
    s.dec_indent();
    s.dec_indent();

    s.wln(">> where");

    s.inc_indent();
    s.wln(format!("W: 'async_trait + {},", it.write()));
    s.wln("'life0: 'async_trait,");
    s.wln("Self: 'async_trait,");
    s.dec_indent();

    s.open_curly("");
    s.open_curly("Box::pin(async move");
}

pub(crate) fn write_into_vec(
    s: &mut Writer,
    type_name: impl AsRef<str>,
    write_function: impl Fn(&mut Writer, ImplType),
    visibility: &str,
) {
    s.open_curly(format!("impl {}", type_name.as_ref()));

    s.open_curly(format!(
        "{visibility} fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error>",
    ));

    write_function(s, ImplType::Std);

    s.wln("Ok(())");

    s.closing_curly();
    s.closing_curly_newline();
}

fn call_as_bytes(s: &mut Writer, it: ImplType, sizes: Sizes) {
    let size = if let Some(size) = sizes.is_constant() {
        size.to_string()
    } else {
        "self.size() + 1".to_string()
    };

    s.wln(format!("let mut v = Vec::with_capacity({size});"));
    s.wln("self.write_into_vec(&mut v)?;");
    s.wln(format!("w.write_all(&v){postfix}", postfix = it.postfix()));
}
