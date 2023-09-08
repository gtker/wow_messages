use crate::file_utils::overwrite_autogenerate_if_not_same_contents;
use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::objects::{Object, Objects};
use crate::path_utils::utils_shared_file;
use crate::rust_printer::structs::print_common_impls::{print_read, print_write};
use crate::rust_printer::writer::Writer;
use crate::rust_printer::{ImplType, PARSE_ERROR_KIND};

pub(crate) fn print_read_write_base_structs(o: &Objects) {
    let mut s = Writer::new();

    for e in o.all_objects() {
        let e = match e {
            Object::Container(e) => e,
            Object::Enum(_) | Object::Flag(_) => continue,
        };

        if !(matches!(e.container_type(), ContainerType::Struct) && e.tags().is_in_base()) {
            continue;
        }

        let ty = e.base_read_write_full_import_path();
        let error_name = if e.only_has_io_errors() {
            "std::io::Error"
        } else {
            PARSE_ERROR_KIND
        };

        let prefix = ImplType::Std.prefix();
        let postfix = ImplType::Std.postfix();
        let read_name = base_struct_read_name(e);
        let write_name = base_struct_write_name(e);

        s.bodyn(
            format!(
                "pub(crate) fn {read_name}<R: std::io::Read>(mut r: R) -> Result<{ty}, {error_name}>"
            ),
            |s| {
                print_read::print_read(s, e, o, prefix, postfix, Some(&ty));
            },
        );

        s.bodyn(
            format!(
                "pub(crate) fn {write_name}(s: &{ty}, mut w: impl std::io::Write) -> Result<(), std::io::Error>"
            ),
            |s| {
                print_write::print_write(s, e, o, prefix, postfix, "s.");

                s.wln("Ok(())");
            },
        );
    }

    overwrite_autogenerate_if_not_same_contents(s.inner(), &utils_shared_file());
}

pub(crate) fn base_struct_read_name(e: &Container) -> String {
    let versions = e
        .tags()
        .versions()
        .map(|a| a.try_as_major_world().unwrap().module_name())
        .collect::<Vec<_>>()
        .join("_");
    let name = e.name().to_lowercase();

    format!("{versions}_{name}_read")
}

pub(crate) fn base_struct_write_name(e: &Container) -> String {
    let versions = e
        .tags()
        .versions()
        .map(|a| a.try_as_major_world().unwrap().module_name())
        .collect::<Vec<_>>()
        .join("_");
    let name = e.name().to_lowercase();

    format!("{versions}_{name}_write_into_vec")
}
