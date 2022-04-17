use crate::container::{Container, OptionalStatement, StructMember};
use crate::parser::types::objects::Objects;
use crate::parser::types::ty::Type;
use crate::parser::types::ArrayType;
use crate::rust_printer::structs::print_common_impls::print_size_of_ty;
use crate::rust_printer::Writer;

pub fn print_optional(s: &mut Writer, e: &Container, optional: &OptionalStatement, o: &Objects) {
    print_declaration(s, e, optional);

    print_impls(s, e, optional, o);
}

fn print_declaration(s: &mut Writer, e: &Container, optional: &OptionalStatement) {
    s.wln("#[derive(Debug, Clone, PartialEq, Default)]");
    s.new_struct(
        format!(
            "{original_ty}_{name}",
            original_ty = e.name(),
            name = optional.name()
        ),
        |s| {
            for m in optional.members() {
                match m {
                    StructMember::Definition(d) => {
                        s.wln(format!(
                            "pub {name}: {ty},",
                            name = d.name(),
                            ty = d.ty().rust_str()
                        ));
                    }
                    StructMember::IfStatement(_) => panic!(),
                    StructMember::OptionalStatement(_) => panic!(),
                }
            }
        },
    );
}

fn print_impls(s: &mut Writer, e: &Container, optional: &OptionalStatement, o: &Objects) {
    s.open_curly(format!(
        "impl {original_ty}_{name}",
        original_ty = e.name(),
        name = optional.name()
    ));

    s.func_pub("size(&self)", "usize", |s| {
        for (i, m) in optional.members().iter().enumerate() {
            if i == 0 {
                s.w("");
            } else {
                s.w("+ ");
            }

            match m {
                StructMember::Definition(d) => {
                    let array_inner_constant = match d.ty() {
                        Type::Array(array) => match array.ty() {
                            ArrayType::Integer(_) => true,
                            ArrayType::Complex(ident) => {
                                o.type_has_constant_size(&Type::Identifier {
                                    s: ident.clone(),
                                    upcast: None,
                                })
                            }
                            ArrayType::CString => false,
                            ArrayType::Guid => true,
                            ArrayType::PackedGuid => false,
                        },
                        _ => false,
                    };

                    print_size_of_ty(
                        s,
                        d.ty(),
                        d.name(),
                        true,
                        o.type_has_constant_size(d.ty()),
                        array_inner_constant,
                        "self.",
                        &d.ty().str(),
                    );
                }
                StructMember::IfStatement(_) => panic!(),
                StructMember::OptionalStatement(_) => panic!(),
            }
        }
    });

    s.closing_curly();
    s.newline();
}
