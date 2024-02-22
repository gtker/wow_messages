use crate::parser::types::definer::Definer;
use crate::parser::types::IntegerType;
use crate::rust_printer::writer::Writer;
use crate::rust_printer::{
    print_docc_description_and_comment, print_member_docc_description_and_comment,
    print_serde_derive, CFG_TESTCASE,
};
use crate::wowm_printer::get_definer_wowm_definition;
use crate::Objects;

pub(crate) fn print_enum_for_base(e: &Definer, o: &Objects) -> Writer {
    print_enum_inner(e, o, true)
}

pub(crate) fn print_enum(e: &Definer, o: &Objects) -> Writer {
    print_enum_inner(e, o, false)
}

fn print_enum_inner(e: &Definer, o: &Objects, visibility_override: bool) -> Writer {
    let mut s = Writer::new();

    declaration(&mut s, e, o, visibility_override);

    common_impls(&mut s, e, visibility_override);

    testcase_string(&mut s, e);

    s.wln(format!("const NAME: &str = \"{}\";", e.name()));
    s.newline();

    print_default(&mut s, e);

    print_display(&mut s, e);

    print_try_from(&mut s, e);

    s
}

fn declaration(s: &mut Writer, e: &Definer, o: &Objects, common_visibility_override: bool) {
    print_docc_description_and_comment(s, e.tags(), o, e.tags());
    print_wowm_definition("enum", s, e);

    s.wln("#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]");
    print_serde_derive(s, e.tags().is_in_base(), false);
    let visibility = match e.only_used_in_if() && !common_visibility_override && !e.tags().shared()
    {
        true => "pub(crate)",
        false => "pub",
    };
    s.new_enum(visibility, e.name(), |s| {
        for field in e.fields() {
            print_member_docc_description_and_comment(s, field.tags(), o, e.tags());

            s.wln(format!("{},", field.rust_name()));
        }
    });
}

pub(crate) fn print_wowm_definition(kind: &str, s: &mut Writer, e: &Definer) {
    s.docc_wowm(
        |s| {
            s.w(get_definer_wowm_definition(kind, e, "/// "));
        },
        e.file_info(),
    );
}

fn common_impls(s: &mut Writer, e: &Definer, visibility_override: bool) {
    s.bodyn(format!("impl {}", e.name()), |s| {
        as_type(s, e, visibility_override);
        iterators(s, e);
        from_type(s, e);
    });
}

fn iterators(s: &mut Writer, e: &Definer) {
    let size = e.fields().len();
    s.funcn_pub_const("variants()", format!("[Self; {size}]"), |s| {
        s.wln("[");
        s.inc_indent();

        for field in e.fields() {
            s.wln(format!("Self::{},", field.rust_name()));
        }

        s.dec_indent();
        s.wln("]");
    });
}

fn from_type(s: &mut Writer, e: &Definer) {
    let ty_name = e.ty().rust_str();
    s.body(
        format!(
            "pub const fn from_int(value: {ty_name}) -> Result<Self, crate::errors::EnumError>",
        ),
        |s| {
            s.body("match value", |s| {
                for field in e.fields() {
                    let value = field.value().int();
                    let field_name = field.rust_name();

                    s.wln(format!("{value} => Ok(Self::{field_name}),",));
                }

                s.wln("v => Err(crate::errors::EnumError::new(NAME, v as i128),)");
            });
        },
    );
}

fn as_type(s: &mut Writer, e: &Definer, visibility_override: bool) {
    let f = if visibility_override {
        Writer::funcn_pub_const
    } else {
        Writer::funcn_const
    };

    f(s, "as_int(&self)", e.ty().rust_str(), |s: &mut Writer| {
        s.body("match self", |s| {
            for field in e.fields() {
                let name = field.rust_name();

                let value = field.value().int();
                let value = if value >= 0 {
                    format!("{value:#x}")
                } else {
                    format!("{value}")
                };
                s.wln(format!("Self::{name} => {value},",));
            }
        });
    });
}

fn testcase_string(s: &mut Writer, e: &Definer) {
    let name = e.name();
    s.wln(CFG_TESTCASE);
    s.bodyn(format!("impl {name}"), |s| {
        s.funcn_pub_const("as_test_case_value(&self)", "&'static str", |s| {
            s.body("match self", |s| {
                for field in e.fields() {
                    let rust = field.rust_name();
                    let field = field.name();

                    s.wln(format!("Self::{rust} => \"{field}\","));
                }
            });
        });
    });
}

fn print_default(s: &mut Writer, e: &Definer) {
    s.impl_for("Default", e.name(), |s| {
        s.body("fn default() -> Self", |s| {
            s.wln(format!("Self::{}", e.fields()[0].rust_name()));
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
                        let display = match field.tags().display() {
                            None => field.rust_name(),
                            Some(v) => v,
                        };
                        s.wln(format!(
                            r#"Self::{name} => f.write_str("{display}"),"#,
                            name = field.rust_name(),
                            display = display,
                        ));
                    }
                });
            },
        );
    });
}

fn print_try_from(s: &mut Writer, e: &Definer) {
    let ty_name = e.ty().rust_str();
    s.impl_for(format!("TryFrom<{ty_name}>"), e.name(), |s| {
        s.wln("type Error = crate::errors::EnumError;");

        s.body(
            format!("fn try_from(value: {ty_name}) -> Result<Self, Self::Error>",),
            |s| {
                s.wln("Self::from_int(value)");
            },
        );
    });

    for (from_size, from_signed, from_ty) in IntegerType::try_from_types() {
        let from_ty = *from_ty;
        if from_ty == ty_name || (e.tags().has_login_version() && from_ty == "i64") {
            continue;
        }

        s.impl_for(format!("TryFrom<{from_ty}>"), e.name(), |s| {
            s.wln("type Error = crate::errors::EnumError;");

            s.body(
                format!("fn try_from(value: {from_ty}) -> Result<Self, Self::Error>"),
                |s| {
                    let conversion_is_infallible =
                        e.ty()
                            .conversion_is_infallible(*from_size, *from_signed, from_ty);

                    if conversion_is_infallible {
                        if *from_signed != e.ty().is_signed() {
                            let converted_ty = if from_ty.contains('i') {
                                from_ty.replace('i', "u")
                            } else {
                                from_ty.replace('u', "i")
                            };

                            s.wln(format!(
                                "let v = {converted_ty}::from_le_bytes(value.to_le_bytes());"
                            ));

                            let into = if *from_size == e.ty().size() as i128 {
                                ""
                            } else {
                                ".into()"
                            };

                            s.wln(format!("Self::from_int(v{into})"));
                        } else {
                            s.wln("Self::from_int(value.into())");
                        }
                    } else {
                        s.wln(format!("TryInto::<{ty_name}>::try_into(value)"));
                        s.inc_indent();

                        let convert = if from_ty == "usize" {
                            " as i128"
                        } else {
                            ".into()"
                        };

                        s.wln(format!(
                            ".map_err(|_| crate::errors::EnumError::new(NAME, value{convert}))?"
                        ));
                        s.wln(".try_into()");
                        s.dec_indent();
                    }
                },
            );
        });
    }
}
