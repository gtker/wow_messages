use crate::parser::types::container::Container;
use crate::parser::types::struct_member::StructMember;
use crate::rust_printer::writer::Writer;

pub(crate) fn impl_update_mask_struct(s: &mut Writer, e: &Container) {
    s.body(format!("impl {}", e.name()), |s| {
        create_new(s, e);
    });
}

fn create_new(s: &mut Writer, e: &Container) {
    let mut name_and_args = String::new();

    for m in e.members() {
        match m {
            StructMember::Definition(d) => {
                if d.value().is_some() {
                    continue;
                }

                name_and_args += &format!("{}: {}, ", d.name(), d.ty().rust_str());
            }
            StructMember::IfStatement(_) | StructMember::OptionalStatement(_) => {
                panic!("used_in_update_mask structs can not have optionals or if statements")
            }
        }
    }

    s.funcn_pub_const(format!("new({name_and_args})"), "Self", |s| {
        s.body("Self", |s| {
            for m in e.members() {
                match m {
                    StructMember::Definition(d) => {
                        if d.value().is_some() {
                            continue;
                        }

                        s.wln(format!("{},", d.name()));
                    }
                    StructMember::IfStatement(_) | StructMember::OptionalStatement(_) => panic!(
                        "used_in_update_mask structs can not have optionals or if statements"
                    ),
                }
            }
        });
    });
}
