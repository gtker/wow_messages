use crate::file_utils::get_import_path;
use crate::parser::types::array::{ArraySize, ArrayType};
use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::objects::Objects;
use crate::parser::types::sizes::{Sizes, DATETIME_SIZE, GOLD_SIZE, GUID_SIZE};
use crate::parser::types::ty::Type;
use crate::parser::types::version::{MajorWorldVersion, Version};
use crate::rust_printer::rust_view::{RustMember, RustObject, RustType};
use crate::rust_printer::{
    ImplType, Writer, CLIENT_MESSAGE_TRAIT_NAME, PARSE_ERROR, SERVER_MESSAGE_TRAIT_NAME,
};
use crate::CONTAINER_SELF_SIZE_FIELD;

pub mod print_read;
pub mod print_write;

pub(crate) fn print_common_impls(s: &mut Writer, e: &Container, o: &Objects) {
    print_world_message_headers_and_constants(s, e);

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

            impl_read_write_struct(
                s,
                e.name(),
                |s, it| {
                    print_read::print_read(s, e, o, it.prefix(), it.postfix());
                },
                |s, it| {
                    print_write::print_write(s, e, o, it.prefix(), it.postfix());
                },
                create_async_reads,
                error_name,
                visibility,
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
            "fn write_unencrypted_{ty}<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error>"
        ));

        print_unencrypted_body(s, "", feature_name, ty, version);
        s.closing_curly_newline(); // fn write_unencrypted

        s.wln("#[cfg(all(feature = \"sync\", feature = \"encryption\"))]");
        s.wln(format!("fn write_encrypted_{ty}<W: std::io::Write>("));

        s.inc_indent();
        s.wln("&self,");
        s.wln("mut w: W,");
        s.wln(format!("e: &mut {encrypter},"));
        s.dec_indent();

        s.open_curly(") -> Result<(), std::io::Error>");
        print_encrypted_body(s, "", feature_name, ty, size_cast, opcode_cast);
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
            print_unencrypted_body(s, ".await", feature_name, ty, version);
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
            print_encrypted_body(s, ".await", feature_name, ty, size_cast, opcode_cast);
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
) {
    s.wln("let mut v = Vec::with_capacity(1024);");
    s.wln("let mut s = &mut v;");
    s.wln(format!(
        "crate::util::{feature_name}_get_unencrypted_{ty}(&mut s, Self::OPCODE as u16, 0)?;"
    ));
    s.wln("self.write_into_vec(&mut s)?;");
    s.wln("let size = v.len().saturating_sub(2) as u16;");
    s.wln(format!(
        "let header = e.encrypt_{ty}_header(size{size_cast}, Self::OPCODE{opcode_cast});"
    ));
    s.open_curly("for (i, e) in header.iter().enumerate()");
    s.wln("v[i] = *e;");
    s.closing_curly();
    s.wln(format!("w.write_all(&v){extra}"));
}

fn print_unencrypted_body(
    s: &mut Writer,
    extra: &str,
    feature_name: &str,
    ty: &str,
    version: MajorWorldVersion,
) {
    s.wln("let mut v = Vec::with_capacity(1024);");
    s.wln("let mut s = &mut v;");
    s.wln(format!(
        "crate::util::{feature_name}_get_unencrypted_{ty}(&mut s, Self::OPCODE as u16, 0)?;"
    ));
    s.wln("self.write_into_vec(&mut s)?;");

    s.wln("let size = v.len().saturating_sub(2);");
    s.wln("let s = size.to_le_bytes();");

    s.wln("v[0] = s[1];");
    s.wln("v[1] = s[0];");

    match version {
        MajorWorldVersion::Vanilla | MajorWorldVersion::BurningCrusade => {}
        MajorWorldVersion::Wrath => {
            s.open_curly("if size > 0x7FFF");
            s.wln("v[2] = s[2];");
            s.closing_curly();
        }
    }

    s.wln(format!("w.write_all(&v){extra}"));
}

fn test_for_invalid_size(s: &mut Writer, e: &Container) {
    let header = match e.is_constant_sized() {
        true => format!("if body_size != {}", e.sizes().maximum()),
        false => {
            let min = e.sizes().minimum();
            let max = if e.sizes().maximum() >= u32::MAX as usize {
                (u32::MAX - 1) as usize // More realistic number here?
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
            "return Err({}::InvalidSize {{ opcode: {:#06X}, size: body_size as u32 }});",
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

pub(crate) fn print_rust_members_sizes(
    s: &mut Writer,
    members: &[RustMember],
    is_elseif: Option<bool>,
    prefix: &str,
) {
    let mut i = 0;
    for m in members {
        if m.tags().skip_serialize() {
            continue;
        }

        let is_elseif = if let Some(b) = is_elseif { b } else { true };
        if i == 0 && is_elseif {
            s.w("");
        } else {
            s.w("+ ");
        }

        print_size_of_ty_rust_view(s, m, prefix);

        i += 1;
    }
}

pub(crate) fn print_size_of_ty_rust_view(s: &mut Writer, m: &RustMember, prefix: &str) {
    let str = match m.ty() {
        RustType::Bool(i) => format!("{}", i.size()),
        RustType::Integer(i) => i.size().to_string(),
        RustType::Floating(f) => f.size().to_string(),
        RustType::Guid => GUID_SIZE.to_string(),
        RustType::Gold => GOLD_SIZE.to_string(),
        RustType::DateTime => DATETIME_SIZE.to_string(),
        RustType::String => format!("{prefix}{name}.len() + 1", name = m.name(), prefix = prefix),
        RustType::CString => format!("{prefix}{name}.len() + 1", name = m.name(), prefix = prefix),
        RustType::SizedCString => {
            format!("{prefix}{name}.len() + 5", name = m.name(), prefix = prefix)
        }
        RustType::Struct { sizes, .. } => {
            if let Some(size) = sizes.is_constant() {
                format!("{size}")
            } else {
                format!("{prefix}{name}.size()", prefix = prefix, name = m.name())
            }
        }
        RustType::EnchantMask
        | RustType::InspectTalentGearMask
        | RustType::MonsterMoveSpline
        | RustType::AchievementDoneArray
        | RustType::AchievementInProgressArray
        | RustType::PackedGuid
        | RustType::UpdateMask
        | RustType::AuraMask => {
            format!("{prefix}{name}.size()", prefix = prefix, name = m.name())
        }
        RustType::Enum {
            is_simple, int_ty, ..
        }
        | RustType::Flag {
            is_simple, int_ty, ..
        } => {
            if !is_simple {
                format!("{prefix}{name}.size()", name = m.name(), prefix = prefix)
            } else {
                int_ty.size().to_string()
            }
        }
        RustType::Array {
            array, inner_sizes, ..
        } => {
            let inner_is_constant = inner_sizes.is_constant().is_some();
            match array.ty() {
                ArrayType::Integer(integer_type) => match array.size() {
                    ArraySize::Fixed(fixed_value) => format!(
                        "{array_size} * core::mem::size_of::<{ty}>()",
                        array_size = fixed_value,
                        ty = integer_type.rust_str(),
                    ),
                    ArraySize::Variable(_) | ArraySize::Endless => {
                        // ZLib compression is not predictable, so we compress the data and count the bytes.
                        if m.tags().is_compressed() {
                            format!(
                                "crate::util::zlib_compressed_size({ref}{prefix}{name})",
                                prefix = prefix,
                                ref = if prefix.is_empty() { "" } else { "&" },
                                name = m.name()
                            )
                        } else {
                            format!(
                                "{prefix}{name}.len() * core::mem::size_of::<{ty}>()",
                                name = m.name(),
                                prefix = prefix,
                                ty = integer_type.rust_str(),
                            )
                        }
                    }
                },
                ArrayType::Struct(_) => match array.size() {
                    ArraySize::Fixed(fixed_value) => match m.constant_sized() {
                        true => format!(
                            "{fixed_value} * {inner_type_size}",
                            inner_type_size = inner_sizes.maximum(),
                            fixed_value = fixed_value,
                        ),
                        false => format!(
                            "{prefix}{name}.iter().fold(0, |acc, x| acc + x.size())",
                            name = m.name(),
                            prefix = prefix,
                        ),
                    },
                    ArraySize::Variable(_) | ArraySize::Endless => {
                        // ZLib compression is not predictable, so we need to serialise each array member into a Vec
                        // and compress the entire buffer to calculate the true size of the packet.
                        if m.tags().is_compressed() {
                            format!(
                                "crate::util::zlib_compressed_size({prefix}{name}.iter().fold(Vec::new(), |mut acc, x| {{ x.write_into_vec(&mut acc).unwrap(); acc }} ).as_slice())",
                                prefix = prefix,
                                name = m.name()
                            )
                        } else {
                            match inner_is_constant {
                                true => {
                                    format!(
                                        "{prefix}{name}.len() * {size}",
                                        name = m.name(),
                                        prefix = prefix,
                                        size = inner_sizes.maximum(),
                                    )
                                }
                                false => {
                                    format!(
                                        "{prefix}{name}.iter().fold(0, |acc, x| acc + x.size())",
                                        name = m.name(),
                                        prefix = prefix,
                                    )
                                }
                            }
                        }
                    }
                },
                ArrayType::CString => {
                    format!(
                        "{prefix}{name}.iter().fold(0, |acc, x| acc + x.len() + 1)",
                        name = m.name(),
                        prefix = prefix,
                    )
                }
                ArrayType::Guid => {
                    format!(
                        "{prefix}{name}.iter().fold(0, |acc, _| acc + 8)",
                        name = m.name(),
                        prefix = prefix,
                    )
                }
                ArrayType::PackedGuid => {
                    format!(
                        "{prefix}{name}.iter().fold(0, |acc, x| acc + x.size())",
                        name = m.name(),
                        prefix = prefix,
                    )
                }
            }
        }
    };
    s.w_no_indent(str);
    s.wln_no_indent(m.size_comment());
}

pub(crate) fn print_size_rust_view(s: &mut Writer, c: &Container, prefix: &str) {
    let r = c.rust_object();

    if !r.constant_sized() {
        if c.tags().compressed() {
            s.variable_size(r.name(), "size", |s| {
                s.wln("use crate::traits::Message;");
                s.newline();

                s.wln("let mut v = Vec::new();");
                s.wln("self.write_into_vec(&mut v);");
                s.wln("v.len()");
            });
        } else {
            print_size_uncompressed_rust_view(s, r, prefix, "size");
        }
    }
}

pub(crate) fn print_size_uncompressed_rust_view(
    s: &mut Writer,
    r: &RustObject,
    prefix: &str,
    function_name: &str,
) {
    if !r.constant_sized() {
        s.variable_size(r.name(), function_name, |s| {
            print_rust_members_sizes(s, r.members(), None, prefix);

            if let Some(optional) = r.optional() {
                s.body_else(
                    format!(
                        "{plus}if let Some({name}) = &{prefix}{name}",
                        name = optional.name(),
                        prefix = prefix,
                        plus = match r.members().is_empty() {
                            true => "",
                            false => "+ ",
                        }
                    ),
                    |s| {
                        let prefix = format!("{}.", optional.name());
                        print_rust_members_sizes(s, optional.members(), None, &prefix);
                    },
                    |s| {
                        s.wln("0");
                    },
                );
            }
        });
    }
}

pub(crate) fn impl_world_message(
    s: &mut Writer,
    type_name: impl AsRef<str>,
    opcode: u16,
    write_function: impl Fn(&mut Writer, ImplType),
    read_function: impl Fn(&mut Writer, ImplType),
    sizes: Option<Sizes>,
) {
    s.open_curly(format!("impl crate::Message for {}", type_name.as_ref()));
    s.wln(format!("const OPCODE: u32 = {opcode:#06x};"));
    s.newline();

    s.open_curly("fn size_without_header(&self) -> u32");
    if sizes.is_some() && sizes.unwrap().is_constant().is_some() {
        s.wln(format!("{}", sizes.unwrap().maximum()));
    } else {
        s.wln("self.size() as u32");
    }
    s.closing_curly(); // size_without_header
    s.newline();

    s.write_into_vec_trait(&write_function);

    s.open_curly(format!(
        "fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, {PARSE_ERROR}>",
    ));

    read_function(s, ImplType::Std);

    s.closing_curly_newline(); // read_body

    s.closing_curly(); // impl Message
}

pub(crate) fn impl_read_and_writable_login(
    s: &mut Writer,
    type_name: impl AsRef<str>,
    opcode: u16,
    trait_to_impl: impl AsRef<str>,
    read_function: impl Fn(&mut Writer, ImplType),
    write_function: impl Fn(&mut Writer, ImplType),
    sizes: Sizes,
) {
    s.write_into_vec(&type_name, write_function, "pub(crate)");

    s.open_curly(format!(
        "impl {} for {}",
        trait_to_impl.as_ref(),
        type_name.as_ref(),
    ));
    s.wln(format!("const OPCODE: u8 = {opcode:#04x};"));
    s.newline();

    for it in ImplType::types() {
        s.print_read_decl(it);

        read_function(s, it);
        if it.is_async() {
            s.closing_curly_with(")"); // Box::pin
        }
        s.closing_curly_newline();

        s.print_write_decl(it);

        s.call_as_bytes(it, sizes);

        if it.is_async() {
            s.closing_curly_with(")"); // Box::pin
        }

        s.closing_curly_newline(); // Write Function
    }

    s.closing_curly_newline(); // impl
}

pub(crate) fn impl_read_write_opcode(
    s: &mut Writer,
    type_name: impl AsRef<str>,
    error_name: impl AsRef<str>,
    read_function: impl Fn(&mut Writer, ImplType),
    write_function: impl Fn(&mut Writer, ImplType),
    create_async_reads: bool,
) {
    impl_read_write_non_trait(
        s,
        type_name,
        error_name,
        read_function,
        write_function,
        "pub(crate)",
        "pub",
        create_async_reads,
    )
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
    s.write_into_vec(&type_name, write_function, read_visibility.as_ref());

    s.open_curly(format!("impl {}", type_name.as_ref()));

    for it in ImplType::types() {
        if it.is_async() {
            if !create_async_reads {
                continue;
            }

            s.wln(it.cfg());
        }
        s.open_curly(format!(
            "{visibility} {func}fn {prefix}read<R: {read}>(mut r: R) -> std::result::Result<Self, {error}>",
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

pub(crate) fn impl_read_write_struct(
    s: &mut Writer,
    type_name: impl AsRef<str>,
    read_function: impl Fn(&mut Writer, ImplType),
    write_function: impl Fn(&mut Writer, ImplType),
    create_async_reads: bool,
    error_name: &str,
    visibility: &str,
) {
    impl_read_write_non_trait(
        s,
        type_name,
        error_name,
        read_function,
        write_function,
        visibility,
        visibility,
        create_async_reads,
    )
}
