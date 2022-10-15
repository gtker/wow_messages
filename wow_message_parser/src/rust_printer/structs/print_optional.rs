use crate::rust_printer::rust_view::RustOptional;
use crate::rust_printer::structs::print_common_impls::print_rust_members_sizes;
use crate::rust_printer::structs::print_derives;
use crate::rust_printer::Writer;

pub(crate) fn print_optional(s: &mut Writer, optional: &RustOptional) {
    print_declaration(s, optional);

    print_impls(s, optional);
}

fn print_declaration(s: &mut Writer, optional: &RustOptional) {
    print_derives(
        s,
        &optional
            .all_members()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>(),
        false,
    );
    s.new_struct(optional.ty(), |s| {
        for m in optional.members() {
            s.wln(format!("pub {name}: {ty},", name = m.name(), ty = m.ty(),));
        }
    });
}

fn print_impls(s: &mut Writer, optional: &RustOptional) {
    s.open_curly(format!("impl {ty}", ty = optional.ty(),));

    s.funcn("size(&self)", "usize", |s| {
        print_rust_members_sizes(s, optional.members(), None, "self.");
    });

    s.closing_curly();
    s.newline();
}
