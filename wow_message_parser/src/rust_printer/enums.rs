use crate::file_utils::get_import_path;
use crate::parser::enumerator::Definer;
use crate::parser::types::{Endianness, IntegerType};
use crate::rust_printer::{ImplType, Writer};
use crate::wowm_printer::get_definer_wowm_definition;
use crate::{DISPLAY_STR, LOGIN_MESSAGES_GITHUB_REPO};

pub fn print_enum(e: &Definer) -> Writer {
    let mut s = Writer::new(&get_import_path(e.tags()));

    includes(&mut s);

    declaration(&mut s, e);

    common_impls(&mut s, e);

    print_default(&mut s, e);

    print_display(&mut s, e);

    print_from_or_try_from(&mut s, e);

    print_errors(&mut s, e);

    s
}

fn includes(s: &mut Writer) {
    s.wln("use std::convert::{TryFrom, TryInto};");
    s.wln("use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};");

    s.write_async_includes();

    s.newline();
}

fn declaration(s: &mut Writer, e: &Definer) {
    print_wowm_definition("enum", s, e);

    s.wln("#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]");
    s.new_enum(e.name(), |s| {
        for field in e.fields() {
            s.wln(format!("{},", field.name()));
        }

        match e.self_value() {
            None => {}
            Some(self_value) => {
                s.wln(format!("{}({}),", self_value.name(), e.ty().rust_str()));
            }
        }
    });
}

pub fn print_wowm_definition(kind: &str, s: &mut Writer, e: &Definer) {
    s.docc_wowm(
        |s| {
            s.wln(get_definer_wowm_definition(kind, e, "/// "));
        },
        LOGIN_MESSAGES_GITHUB_REPO,
        e.file_info(),
    );
}

fn common_impls(s: &mut Writer, e: &Definer) {
    let error_string = match e.self_value() {
        None => format!("{}Error", e.name()),
        Some(_) => "std::io::Error".to_string(),
    };
    s.impl_read_and_writable_with_error(
        e.name(),
        error_string,
        |s, it| {
            print_read(s, e, it);
        },
        |s, it| {
            print_write(s, e, it);
        },
    );

    s.bodyn(format!("impl {}", e.name()), |s| {
        read_write_as(s, e);

        as_type(s, e);

        print_new(s, e);
    });

    s.constant_sized(e.name(), |a| {
        a.wln(format!("{}", e.ty().size()));
    });
}

pub fn get_upcast_types(ty: &IntegerType) -> Vec<IntegerType> {
    match ty {
        IntegerType::U8 => vec![
            IntegerType::U16(Endianness::Little),
            IntegerType::U16(Endianness::Big),
            IntegerType::U32(Endianness::Little),
            IntegerType::U32(Endianness::Big),
            IntegerType::U64(Endianness::Little),
            IntegerType::U64(Endianness::Big),
        ],
        IntegerType::U16(e) => {
            vec![
                match e {
                    Endianness::Little => IntegerType::U16(Endianness::Big),
                    Endianness::Big => IntegerType::U16(Endianness::Little),
                },
                IntegerType::U32(Endianness::Little),
                IntegerType::U32(Endianness::Big),
                IntegerType::U64(Endianness::Little),
                IntegerType::U64(Endianness::Big),
            ]
        }
        IntegerType::U32(e) => {
            vec![
                match e {
                    Endianness::Little => IntegerType::U32(Endianness::Big),
                    Endianness::Big => IntegerType::U32(Endianness::Little),
                },
                IntegerType::U64(Endianness::Little),
                IntegerType::U64(Endianness::Big),
            ]
        }
        IntegerType::U64(e) => {
            vec![match e {
                Endianness::Little => IntegerType::U64(Endianness::Big),
                Endianness::Big => IntegerType::U64(Endianness::Little),
            }]
        }
    }
}

fn read_write_as(s: &mut Writer, e: &Definer) {
    // All types with equal size or greater
    let types = get_upcast_types(e.ty());

    for t in types {
        let return_type = format!(
            "std::result::Result<Self, {name}>",
            name = match e.self_value() {
                None => format!("{}Error", e.name()),
                Some(_) => "std::io::Error".to_string(),
            },
        );

        s.async_funcn_pub(
            format!(
                "read_{ty}_{endian}<R: std::io::Read>(r: &mut R)",
                ty = t.rust_str(),
                endian = t.rust_endian_str(),
            ),
            format!(
                "tokio_read_{ty}_{endian}<R: AsyncReadExt + Unpin + Send>(r: &mut R)",
                ty = t.rust_str(),
                endian = t.rust_endian_str()
            ),
            format!(
                "astd_read_{ty}_{endian}<R: ReadExt + Unpin + Send>(r: &mut R)",
                ty = t.rust_str(),
                endian = t.rust_endian_str()
            ),
            &return_type,
            |s, it| {
                s.wln(format!(
                    "let a = {util_path}::{prefix}read_{ty}_{endian}(r){postfix}?;",
                    util_path = "crate::util",
                    ty = t.rust_str(),
                    endian = t.rust_endian_str(),
                    prefix = it.prefix(),
                    postfix = it.postfix(),
                ));
                s.wln(format!(
                    "Ok((a as {original_ty}).{from})",
                    original_ty = e.ty().rust_str(),
                    from = match e.self_value() {
                        None => "try_into()?",
                        Some(_) => "into()",
                    }
                ));
            },
        );

        let return_type = "std::result::Result<(), std::io::Error>";
        s.async_funcn_pub(
            format!(
                "write_{ty}_{endian}<W: std::io::Write>(&self, w: &mut W)",
                ty = t.rust_str(),
                endian = t.rust_endian_str()
            ),
            format!(
                "tokio_write_{ty}_{endian}<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W)",
                ty = t.rust_str(),
                endian = t.rust_endian_str()
            ),
            format!(
                "astd_write_{ty}_{endian}<W: WriteExt + Unpin + Send>(&self, w: &mut W)",
                ty = t.rust_str(),
                endian = t.rust_endian_str()
            ),
            &return_type,
            |s, it| {
                s.wln(format!(
                    "crate::util::{prefix}write_{ty}_{endian}(w, self.as_{original_ty}() as {ty}){postfix}?;",
                    ty = t.rust_str(),
                    endian = t.rust_endian_str(),
                    original_ty = e.ty().rust_str(),
                    prefix = it.prefix(),
                    postfix = it.postfix(),
                ));
                s.wln("Ok(())");
            },
        );
    }
}

fn as_type(s: &mut Writer, e: &Definer) {
    s.funcn_pub_const(
        format!("as_{type_name}(&self)", type_name = e.ty().rust_str()).as_str(),
        e.ty().rust_str(),
        |s| {
            s.body("match self", |s| {
                for field in e.fields() {
                    s.wln(format!(
                        "Self::{} => 0x{:x},",
                        field.name(),
                        field.value().int()
                    ));
                }
                match e.self_value() {
                    None => {}
                    Some(self_value) => {
                        s.wln(format!("Self::{}(v) => *v,", self_value.name()));
                    }
                }
            });
        },
    );
}

fn print_new(s: &mut Writer, e: &Definer) {
    s.const_fn("new()", "Self", |a| {
        a.wln(format!("Self::{}", e.fields()[0].name()));
    });
}

fn print_read(s: &mut Writer, e: &Definer, it: ImplType) {
    s.wln(format!(
        "let a = {util_path}::{prefix}read_{ty}_{endian}(r){postfix}?;",
        util_path = "crate::util",
        ty = e.ty().rust_str(),
        endian = e.ty().rust_endian_str(),
        prefix = it.prefix(),
        postfix = it.postfix(),
    ));
    s.newline();

    s.wln(format!(
        "Ok(a.{})",
        match e.self_value() {
            None => "try_into()?",
            Some(_) => "into()",
        }
    ));
}

fn print_write(s: &mut Writer, e: &Definer, it: ImplType) {
    s.wln(format!(
        "w.write_all(&self.as_{}().to_{}_bytes()){}?;",
        e.ty().rust_str(),
        e.ty().rust_endian_str(),
        it.postfix(),
    ));

    s.wln("Ok(())");
}

fn print_default(s: &mut Writer, e: &Definer) {
    s.impl_for("Default", e.name(), |s| {
        s.body("fn default() -> Self", |s| {
            s.wln(format!("Self::{}", e.fields()[0].name()));
        });
    });
}

fn print_display(s: &mut Writer, e: &Definer) {
    s.impl_for("std::fmt::Display", e.name(), |s| {
        s.body(
            "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result",
            |s| {
                s.body("match self", |s| {
                    for field in e.fields() {
                        let display = match field.tags().get_ref(DISPLAY_STR) {
                            None => field.name(),
                            Some(v) => v,
                        };
                        s.wln(format!(
                            r#"Self::{name} => f.write_str("{display}"),"#,
                            name = field.name(),
                            display = display,
                        ));
                    }

                    match e.self_value() {
                        None => {}
                        Some(self_value) => {
                            s.wln(format!(
                                r#"Self::{name}(v) => f.write_fmt(format_args!("{name}({{}})", v)),"#,
                                name = self_value.name(),
                            ));
                        }
                    }
                });
            },
        );
    });
}

fn print_from_or_try_from(s: &mut Writer, e: &Definer) {
    if e.self_value().is_none() {
        print_try_from(s, e);
    } else {
        print_from(s, e);
    }
}

fn print_try_from(s: &mut Writer, e: &Definer) {
    s.impl_for(format!("TryFrom<{}>", e.ty().rust_str()), e.name(), |s| {
        s.wln(format!("type Error = TryFrom{}Error;", e.name()));

        s.body(
            format!(
                "fn try_from(value: {}) -> std::result::Result<Self, Self::Error>",
                e.ty().rust_str()
            ),
            |s| {
                s.body("match value", |s| {
                    for field in e.fields() {
                        s.wln(format!(
                            "{} => Ok(Self::{}),",
                            field.value().int(),
                            field.name()
                        ));
                    }

                    s.wln(format!("_ => Err(TryFrom{}Error::new(value))", e.name()));
                });
            },
        );
    });
}

fn print_from(s: &mut Writer, e: &Definer) {
    s.impl_for(format!("From<{}>", e.ty().rust_str()), e.name(), |s| {
        s.body(
            format!("fn from(value: {}) -> Self", e.ty().rust_str()),
            |s| {
                s.body("match value", |s| {
                    for field in e.fields() {
                        s.wln(format!(
                            "{} => Self::{},",
                            field.value().int(),
                            field.name()
                        ));
                    }

                    s.wln(format!(
                        "v => Self::{}(v)",
                        e.self_value().as_ref().unwrap().name()
                    ));
                });
            },
        );
    });
}

fn print_errors(s: &mut Writer, e: &Definer) {
    if e.self_value().is_some() {
        return;
    }

    s.wln("#[derive(Debug)]");
    s.new_struct(format!("TryFrom{}Error", e.name()), |s| {
        s.wln(format!("value: {},", e.ty().rust_str()));
    });

    s.bodyn(format!("impl TryFrom{}Error", e.name()), |s| {
        s.body(
            format!("pub const fn new(value: {}) -> Self", e.ty().rust_str()),
            |s| {
                s.wln("Self { value }");
            },
        );
    });

    s.wln("#[derive(Debug)]");
    s.new_enum(format!("{}Error", e.name()), |s| {
        s.wln("Read(std::io::Error),");
        s.wln(format!("TryFrom(TryFrom{}Error),", e.name()));
    });

    s.wln(format!("impl std::error::Error for {}Error {{}}", e.name()));

    s.impl_for(
        "std::fmt::Display",format!("TryFrom{}Error", e.name()),
        |s| {
            s.body(
                "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result",
                |s| {
                    s.wln(format!(
                        r#"f.write_fmt(format_args!("invalid value for enum '{}': '{{}}'", self.value))"#,
                        e.name()
                    ));
                },
            );
        },
    );

    s.impl_for("std::fmt::Display", format!("{}Error", e.name()), |s| {
        s.body(
            "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result",
            |s| {
                s.body("match self", |s| {
                    s.wln("Self::Read(e) => e.fmt(f),");
                    s.wln("Self::TryFrom(e) => e.fmt(f),");
                });
            },
        );
    });

    s.impl_for("From<std::io::Error>", format!("{}Error", e.name()), |s| {
        s.body("fn from(value: std::io::Error) -> Self", |s| {
            s.wln("Self::Read(value)");
        });
    });

    s.impl_for(
        format!("From<TryFrom{}Error>", e.name()),
        format!("{}Error", e.name(),),
        |s| {
            s.body(
                format!("fn from(value: TryFrom{}Error) -> Self", e.name()),
                |s| {
                    s.wln("Self::TryFrom(value)");
                },
            );
        },
    );
}
