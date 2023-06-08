use crate::rust_printer::rust_view::rust_optional::RustOptional;
use crate::rust_printer::structs::print_derives;
use crate::rust_printer::writer::Writer;

pub(crate) fn print_optional(s: &mut Writer, optional: &RustOptional) {
    print_declaration(s, optional);
}

fn print_declaration(s: &mut Writer, optional: &RustOptional) {
    print_derives(s, &optional.all_members(), false);
    s.new_struct(optional.ty(), |s| {
        for m in optional.members_in_struct() {
            s.wln(format!("pub {name}: {ty},", name = m.name(), ty = m.ty(),));
        }
    });
}
