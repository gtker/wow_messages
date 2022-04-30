use crate::container::Container;
use crate::parser::types::objects::Objects;
use crate::rust_printer::structs::print_common_impls::print_read::print_read;
use crate::rust_printer::structs::print_common_impls::print_write::print_write;
use crate::rust_printer::ImplType;
use crate::rust_printer::Writer;

pub fn print_async(s: &mut Writer, e: &Container, o: &Objects, error_ty: &str) {
    s.impl_async_read_and_writable_with_error(
        e.name(),
        error_ty,
        |s| {
            print_read(s, e, o, ImplType::Tokio.prefix(), ImplType::Tokio.postfix());
        },
        |s| {
            print_write(s, e, o, ImplType::Tokio.prefix(), ImplType::Tokio.postfix());
        },
    );
}
