use crate::file_utils::get_import_path;
use crate::parser::enumerator::{Definer};
use crate::parser::types::{Endianness, IntegerType};
use crate::rust_printer::Writer;
use crate::{
    DISPLAY_STR, ENUM_SELF_VALUE_FIELD, LOGIN_MESSAGES_GITHUB_REPO, WORLD_MESSAGES_GITHUB_REPO,
};

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
            s.docc(format!(
                "{kind} {name} : {ty} {{",
                kind = kind,
                name = e.name(),
                ty = e.ty().str(),
            ));

            s.docc_inc();
            for field in e.fields() {
                s.docc(format!(
                    "{name} = {val};",
                    name = field.name(),
                    val = field.value().original()
                ));
            }

            if let Some(f) = e.self_value() {
                s.docc(format!(
                    "{name} = {self_value}",
                    name = f.name(),
                    self_value = ENUM_SELF_VALUE_FIELD
                ));
            }
            s.docc_dec();
            s.docc("}");
        },
        match e.tags().has_login_version() {
            true => LOGIN_MESSAGES_GITHUB_REPO,
            false => WORLD_MESSAGES_GITHUB_REPO,
        },
        e.file_info(),
    );
}

fn common_impls(s: &mut Writer, e: &Definer) {
    s.impl_for("ReadableAndWritable", e.name(), |s| {
        s.wln(format!(
            "type Error = {};\n",
            match e.self_value() {
                None => format!("{}Error", e.name()),
                Some(_) => "std::io::Error".to_string(),
            }
        ));
        print_read(s, e);

        print_write(s, e);
    });

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
        s.funcn_pub(
            format!(
                "read_{ty}_{endian}<R: std::io::Read>(r: &mut R)",
                ty = t.rust_str(),
                endian = t.rust_endian_str(),
            ),
            format!(
                "std::result::Result<Self, {name}>",
                name = match e.self_value() {
                    None => format!("{}Error", e.name()),
                    Some(_) => "std::io::Error".to_string(),
                },
            ),
            |s| {
                s.wln(format!(
                    "let a = {util_path}::read_{ty}_{endian}(r)?;",
                    util_path = "crate::util",
                    ty = t.rust_str(),
                    endian = t.rust_endian_str()
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

        s.funcn_pub(
            format!(
                "write_{ty}_{endian}<W: std::io::Write>(&self, w: &mut W)",
                ty = t.rust_str(),
                endian = t.rust_endian_str()
            ),
            "std::result::Result<(), std::io::Error>",
            |s| {
                s.wln(format!(
                    "{util_path}::write_{ty}_{endian}(w, self.as_{original_ty}() as {ty})?;",
                    util_path = "crate::util",
                    ty = t.rust_str(),
                    endian = t.rust_endian_str(),
                    original_ty = e.ty().rust_str(),
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

fn print_read(s: &mut Writer, e: &Definer) {
    s.bodyn(
        "fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error>",
        |s| {
            s.wln(format!(
                "let a = {util_path}::read_{ty}_{endian}(r)?;",
                util_path = "crate::util",
                ty = e.ty().rust_str(),
                endian = e.ty().rust_endian_str()
            ));
            s.newline();

            s.wln(format!(
                "Ok(a.{})",
                match e.self_value() {
                    None => "try_into()?",
                    Some(_) => "into()",
                }
            ));
        },
    );
}

fn print_write(s: &mut Writer, e: &Definer) {
    s.bodyn(
        "fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error>",
        |s| {
            s.wln(format!(
                "w.write_all(&self.as_{}().to_{}_bytes())?;",
                e.ty().rust_str(),
                e.ty().rust_endian_str(),
            ));

            s.wln("Ok(())");
        },
    );
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
