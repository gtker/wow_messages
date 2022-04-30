use crate::container::Container;
use crate::parser::types::objects::Objects;
use crate::rust_printer::structs::print_common_impls::print_read::print_read;
use crate::rust_printer::{ImplType, ASYNC_TRAIT_MACRO, CFG_ASYNC_ANY, CFG_ASYNC_TOKIO};
use crate::rust_printer::{Writer, ASYNC_TRAIT};

pub fn print_async(s: &mut Writer, e: &Container, o: &Objects, error_ty: &str) {
    s.wln(CFG_ASYNC_ANY);
    s.wln(ASYNC_TRAIT_MACRO);
    s.body(format!("impl {async_trait} for {name}", async_trait = ASYNC_TRAIT, name = e.name()), |s| {
        s.wln(format!("type Error = {};", error_ty));

        s.wln(CFG_ASYNC_TOKIO);
        s.body("async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> Result<Self, Self::Error>", |s| {
            print_read(s, e, o, ImplType::Tokio);
        });
    });
}

pub fn print_async_read(s: &mut Writer, e: &Container, o: &Objects) {}
