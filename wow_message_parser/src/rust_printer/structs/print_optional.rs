use crate::rust_printer::rust_view::RustOptional;
use crate::rust_printer::structs::print_common_impls::print_size_of_ty_rust_view;
use crate::rust_printer::Writer;

pub fn print_optional(s: &mut Writer, optional: &RustOptional) {
    print_declaration(s, optional);

    print_impls(s, optional);
}

fn print_declaration(s: &mut Writer, optional: &RustOptional) {
    s.wln("#[derive(Debug, Clone, PartialEq, Default)]");
    s.new_struct(optional.ty(), |s| {
        for m in optional.members() {
            s.wln(format!("pub {name}: {ty},", name = m.name(), ty = m.ty(),));
        }
    });
}

fn print_impls(s: &mut Writer, optional: &RustOptional) {
    s.open_curly(format!("impl {ty}", ty = optional.ty(),));

    s.func_pub("size(&self)", "usize", |s| {
        for (i, m) in optional.members().iter().enumerate() {
            // Optional can never be empty so we don't need a 0 for empty cases
            if i == 0 {
                s.w("");
            } else {
                s.w("+ ");
            }

            print_size_of_ty_rust_view(s, m, "self.");
        }
    });

    s.closing_curly();
    s.newline();
}
