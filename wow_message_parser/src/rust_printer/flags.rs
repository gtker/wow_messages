use crate::file_utils::get_import_path;
use crate::parser::enumerator::Definer;
use crate::rust_printer::enums::print_wowm_definition;
use crate::rust_printer::{
    Writer, ASYNC_TRAIT, ASYNC_TRAIT_IMPORT, ASYNC_TRAIT_MACRO, CFG_ASYNC_ANY, CFG_ASYNC_TOKIO,
    TOKIO_IMPORT,
};
use crate::UTILITY_PATH;

pub fn print_flag(e: &Definer) -> Writer {
    let mut s = Writer::new(&get_import_path(e.tags()));

    includes(&mut s);

    declaration(&mut s, e);

    common_impls(&mut s, e);

    s
}

fn includes(s: &mut Writer) {
    s.wln("use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable};\n");

    s.wln(CFG_ASYNC_ANY);
    s.wln(ASYNC_TRAIT_IMPORT);
    s.wln(CFG_ASYNC_TOKIO);
    s.wln(TOKIO_IMPORT);

    s.wln(CFG_ASYNC_ANY);
    s.wln(format!("use crate::{};", ASYNC_TRAIT))
}

fn declaration(s: &mut Writer, e: &Definer) {
    print_wowm_definition("flag", s, e);

    s.wln("#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]");
    s.new_flag(e.name(), e.ty().rust_str(), |_| {});

    s.bodyn(format!("impl {name}", name = e.name()), |s| {
        s.funcn_pub_const(
            format!("new(inner: {ty})", ty = e.ty().rust_str()),
            "Self",
            |s| {
                s.wln("Self { inner }");
            },
        );
    });
}

fn common_impls(s: &mut Writer, e: &Definer) {
    s.impl_read_and_writable_with_error(
        e.name(),
        "std::io::Error",
        |s| {
            s.wln(format!(
                "let inner = {module}::read_{ty}_{endian}(r)?;",
                module = UTILITY_PATH,
                ty = e.ty().rust_str(),
                endian = e.ty().rust_endian_str()
            ));
            s.wln("Ok(Self { inner })");
        },
        |s| {
            s.wln(format!(
                "w.write_all(&self.inner.to_{endian}_bytes())?;",
                endian = e.ty().rust_endian_str()
            ));
            s.wln("Ok(())");
        },
    );

    s.wln(CFG_ASYNC_ANY);
    s.wln(ASYNC_TRAIT_MACRO);
    s.body(format!("impl {} for {}", ASYNC_TRAIT, e.name()), |s| {
        s.wln("type Error = std::io::Error;");

        s.wln(CFG_ASYNC_TOKIO);
        s.body("async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> Result<Self, Self::Error>", |s| {
            s.wln(format!(
                "let inner = {module}::tokio_read_{ty}_{endian}(r).await?;",
                module = UTILITY_PATH,
                ty = e.ty().rust_str(),
                endian = e.ty().rust_endian_str()
            ));
            s.wln("Ok(Self { inner })");
        });

        s.wln(CFG_ASYNC_TOKIO);
        s.body("async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error>", |s| {
            s.wln(format!(
                "w.write_all(&self.inner.to_{endian}_bytes()).await?;",
                endian = e.ty().rust_endian_str(),
            ));
            s.wln("Ok(())");
        });
    });

    s.bodyn(format!("impl {name}", name = e.name()), |s| {
        print_fields(s, e);

        s.funcn_pub_const(
            format!("as_{ty}(&self)", ty = e.ty().rust_str()),
            e.ty().rust_str(),
            |s| {
                s.wln("self.inner");
            },
        );
    });

    s.constant_sized(e.name(), |s| {
        s.wln(format!("{size}", size = e.ty().size()));
    });
}

fn print_fields(s: &mut Writer, e: &Definer) {
    for f in e.fields() {
        s.wln(format!(
            "pub const {name}: {ty} = {value:#04x};",
            name = f.name(),
            ty = e.ty().rust_str(),
            value = f.value().int(),
        ));
    }
    s.newline();

    s.funcn_pub_const("empty()", "Self", |s| {
        s.wln("Self { inner: 0 }");
    });

    s.funcn_pub_const("all()", "Self", |s| {
        s.body("Self", |s| {
            s.wln(format!("inner: Self::{name}", name = e.fields()[0].name()));
            s.inc_indent();
            for (i, f) in e.fields().iter().enumerate() {
                if i == 0 {
                    continue;
                }
                s.wln(format!("| Self::{name}", name = f.name()));
            }
            s.dec_indent();
        });
    });

    for f in e.fields() {
        s.funcn_pub_const(
            format!("is_{name}(&self)", name = f.name()),
            "bool",
            |s| match f.value().int() {
                0 => {
                    s.wln("// Underlying value is 0");
                    s.wln(format!("self.inner == Self::{name}", name = f.name()));
                }
                _ => {
                    s.wln(format!("(self.inner & Self::{name}) != 0", name = f.name()));
                }
            },
        );

        s.funcn_pub_const(format!("new_{name}()", name = f.name()), "Self", |s| {
            s.wln(format!("Self {{ inner: Self::{name} }}", name = f.name()));
        });

        s.funcn_pub(
            format!("set_{name}(&mut self)", name = f.name()),
            "Self",
            |s| {
                s.wln(format!("self.inner |= Self::{name};", name = f.name()));
                s.wln("*self");
            },
        );

        s.funcn_pub(
            format!("clear_{name}(&mut self)", name = f.name()),
            "Self",
            |s| {
                s.wln(format!(
                    "self.inner &= Self::{name}.reverse_bits();",
                    name = f.name()
                ));
                s.wln("*self");
            },
        );
    }
}
