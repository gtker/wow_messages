use crate::parser::types::container::{Container, UpdateMaskMember};
use crate::parser::types::ty::Type;
use crate::rust_printer::writer::Writer;

pub(crate) fn create_mask_values(s: &mut Writer, e: &Container, words: &[Vec<UpdateMaskMember>]) {
    let name = e.name();

    let version = e.tags().only_main_world_version().module_name();
    let size = words.iter().fold(0, |sum, a| {
        if !a.is_empty() {
            let size = match a[0].member.ty() {
                Type::Guid => 2,
                Type::Array(array) => {
                    let size = array.fixed_size() * array.ty().sizes().maximum();
                    size / 4
                }
                _ => 1,
            };

            sum + size
        } else {
            sum
        }
    });

    s.open_curly(format!("pub(crate) const fn mask_values(&self, index: crate::{version}::{name}Index) -> [(u16, u32); {size}]"));
    s.wln("let offset = index.offset();");

    for word in words {
        for d in word {
            let name = d.member.name();

            if d.member.ty() == &Type::Guid {
                s.wln(format!(
                    "let ({name}_lower, {name}_upper) = self.{name}.to_u32s();"
                ));
            }
        }
    }

    s.wln("[");
    s.inc_indent();

    for (i, word) in words.iter().enumerate() {
        let offset = if i == 0 {
            "offset".to_string()
        } else {
            format!("offset + {i}")
        };

        match word.as_slice() {
            [member] => {
                let name = member.member.name();

                match member.member.ty() {
                    Type::Guid => {
                        s.wln(format!("({offset}, {name}_lower),"));

                        let second_offset = if i == 0 {
                            "offset + 1".to_string()
                        } else {
                            let i = i + 1;
                            format!("offset + {i}")
                        };
                        s.wln(format!("({second_offset}, {name}_upper),"));
                    }
                    Type::Enum { .. } => {
                        s.wln(format!("({offset}, self.{name}.as_int()),"));
                    }
                    Type::Array(array) => match array.ty().sizes().maximum() {
                        4 => {
                            for array_i in 0..array.fixed_size() {
                                let offset = if i == 0 && array_i == 0 {
                                    "offset".to_string()
                                } else {
                                    format!("offset + {}", i as i128 + array_i)
                                };

                                s.wln(format!("({offset}, self.{name}[{array_i}]),"));
                            }
                        }
                        2 => {
                            for array_i in (0..array.fixed_size()).step_by(2) {
                                let second_index = array_i + 1;

                                s.wln(format!("({offset}, crate::util::u16s_to_u32(self.{name}[{array_i}], self.{name}[{second_index}])),"));
                            }
                        }
                        _ => unimplemented!(),
                    },
                    _ => {
                        s.wln(format!("({offset}, self.{name}),"));
                    }
                }
            }

            [first, second] => {
                fn value(a: &UpdateMaskMember) -> String {
                    let name = a.member.name();

                    match a.member.ty() {
                        Type::Flag { .. } | Type::Enum { .. } => format!("self.{name}.as_int()"),
                        _ => format!("self.{name}"),
                    }
                }

                let first_value = value(first);
                let second_value = value(second);

                s.wln(format!(
                    "({offset}, crate::util::u16s_to_u32({second_value}, {first_value})),"
                ));
            }

            [] => continue,

            _ => unimplemented!(),
        }

        s.newline();
    }

    s.dec_indent();
    s.wln("]");

    s.closing_curly(); // fn mask_values
}
