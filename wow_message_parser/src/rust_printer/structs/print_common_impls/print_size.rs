use crate::parser::types::array::{ArraySize, ArrayType};
use crate::parser::types::container::Container;
use crate::rust_printer::rust_view::rust_member::RustMember;
use crate::rust_printer::rust_view::rust_object::RustObject;
use crate::rust_printer::rust_view::rust_type::RustType;
use crate::rust_printer::writer::Writer;

pub(crate) fn print_rust_members_sizes(
    s: &mut Writer,
    members: &[RustMember],
    is_elseif: Option<bool>,
    prefix: &str,
) {
    for (i, m) in members.iter().enumerate() {
        let is_elseif = if let Some(b) = is_elseif { b } else { true };
        if i == 0 && is_elseif {
            s.w("");
        } else {
            s.w("+ ");
        }

        print_size_of_ty_rust_view(s, m, prefix);
    }
}

pub(crate) fn print_size_of_ty_rust_view(s: &mut Writer, m: &RustMember, prefix: &str) {
    let name = m.name();

    let str = match m.ty() {
        RustType::String => format!("{prefix}{name}.len() + 1"),
        RustType::CString => format!("{prefix}{name}.len() + 1"),
        RustType::SizedCString => {
            format!("{prefix}{name}.len() + 5")
        }
        RustType::Struct { sizes, .. } => {
            if let Some(size) = sizes.is_constant() {
                size.to_string()
            } else {
                format!("{prefix}{name}.size()")
            }
        }

        RustType::MonsterMoveSpline => {
            format!("crate::util::monster_move_spline_size({prefix}{name}.as_slice())",)
        }
        RustType::PackedGuid => {
            format!("crate::util::packed_guid_size(&{prefix}{name})",)
        }
        RustType::AchievementDoneArray => {
            format!("{prefix}{name}.len() * 4")
        }
        RustType::AddonArray => {
            format!("{prefix}{name}.len() * 8")
        }
        RustType::AchievementInProgressArray => {
            format!("{prefix}{name}.iter().fold(0, |acc, x| acc + x.size())",)
        }

        RustType::VariableItemRandomProperty
        | RustType::NamedGuid
        | RustType::EnchantMask
        | RustType::InspectTalentGearMask
        | RustType::UpdateMask { .. }
        | RustType::AuraMask => {
            format!("{prefix}{name}.size()")
        }
        RustType::Enum {
            is_simple, int_ty, ..
        }
        | RustType::Flag {
            is_simple, int_ty, ..
        } => {
            if !is_simple {
                format!("{prefix}{name}.size()")
            } else {
                int_ty.size().to_string()
            }
        }
        RustType::Array {
            array, inner_sizes, ..
        } => {
            let inner_is_constant = inner_sizes.is_constant().is_some();
            match array.ty() {
                ArrayType::Integer(integer_type) => match array.size() {
                    ArraySize::Fixed(fixed_value) => {
                        (integer_type.size() as i128 * fixed_value).to_string()
                    }
                    ArraySize::Variable(_) | ArraySize::Endless => {
                        // ZLib compression is not predictable, so we compress the data and count the bytes.
                        if array.compressed() {
                            format!(
                                "crate::util::zlib_compressed_size({ref}{prefix}{name}) + 4",
                                ref = if prefix.is_empty() { "" } else { "&" },
                            )
                        } else {
                            format!(
                                "{prefix}{name}.len() * core::mem::size_of::<{ty}>()",
                                ty = integer_type.rust_str(),
                            )
                        }
                    }
                },
                ArrayType::Spell => {
                    format!("{prefix}{name}.len() * 4",)
                }
                ArrayType::Struct(e) => match array.size() {
                    ArraySize::Fixed(fixed_value) => match e.sizes().is_constant() {
                        Some(size) => {
                            format!("{fixed_value} * {size}")
                        }
                        None => format!("{prefix}{name}.iter().fold(0, |acc, x| acc + x.size())",),
                    },
                    ArraySize::Variable(_) | ArraySize::Endless => {
                        // ZLib compression is not predictable, so we need to serialise each array member into a Vec
                        // and compress the entire buffer to calculate the true size of the packet.
                        if array.compressed() {
                            format!(
                                "crate::util::zlib_compressed_size({prefix}{name}.iter().fold(Vec::new(), |mut acc, x| {{ x.write_into_vec(&mut acc).unwrap(); acc }} ).as_slice()) + 4",
                            )
                        } else {
                            match inner_is_constant {
                                true => {
                                    format!(
                                        "{prefix}{name}.len() * {size}",
                                        size = inner_sizes.maximum(),
                                    )
                                }
                                false => {
                                    format!(
                                        "{prefix}{name}.iter().fold(0, |acc, x| acc + x.size())",
                                    )
                                }
                            }
                        }
                    }
                },
                ArrayType::CString => {
                    format!("{prefix}{name}.iter().fold(0, |acc, x| acc + x.len() + 1)",)
                }
                ArrayType::Guid => {
                    format!("{prefix}{name}.len() *  8",)
                }
                ArrayType::PackedGuid => {
                    format!("{prefix}{name}.iter().fold(0, |acc, x| acc + crate::util::packed_guid_size(x))",)
                }
            }
        }

        _ => m.ty().to_type().sizes().is_constant().unwrap().to_string(),
    };
    s.w_no_indent(str);
    s.wln_no_indent(m.size_comment());
}

pub(crate) fn print_size_rust_view(s: &mut Writer, c: &Container, prefix: &str) {
    let r = c.rust_object();

    if !r.constant_sized() {
        if c.tags().compressed() {
            variable_size(s, r.name(), "size", false, |s| {
                s.wln("use crate::traits::Message;");
                s.newline();

                s.wln("let mut v = Vec::new();");
                s.wln("self.write_into_vec(&mut v);");
                s.wln("v.len()");
            });
        } else {
            print_size_uncompressed_rust_view(s, r, prefix, "size");
        }
    }
}

pub(crate) fn print_size_uncompressed_rust_view(
    s: &mut Writer,
    r: &RustObject,
    prefix: &str,
    function_name: &str,
) {
    if !r.constant_sized() {
        let const_fn = r.members_in_struct().all(|a| a.ty().size_is_const_fn())
            && if let Some(optional) = r.optional() {
                optional
                    .members_in_struct()
                    .iter()
                    .all(|a| a.ty().size_is_const_fn())
            } else {
                true
            };

        variable_size(s, r.name(), function_name, const_fn, |s| {
            print_rust_members_sizes(s, r.members(), None, prefix);

            if let Some(optional) = r.optional() {
                s.body_else(
                    format!(
                        "{plus}if let Some({name}) = &{prefix}{name}",
                        name = optional.name(),
                        prefix = prefix,
                        plus = match r.members().is_empty() {
                            true => "",
                            false => "+ ",
                        }
                    ),
                    |s| {
                        let prefix = format!("{}.", optional.name());
                        print_rust_members_sizes(s, optional.members(), None, &prefix);
                    },
                    |s| {
                        s.wln("0");
                    },
                );
            }
        });
    }
}

pub(crate) fn variable_size(
    s: &mut Writer,
    name: impl AsRef<str>,
    function_name: &str,
    const_fn: bool,
    variable_sized: impl Fn(&mut Writer),
) {
    s.open_curly(format!("impl {}", name.as_ref()));
    let const_fn = match const_fn {
        true => " const",
        false => "",
    };
    s.open_curly(format!(
        "pub(crate){const_fn} fn {function_name}(&self) -> usize"
    ));

    variable_sized(s);

    s.closing_curly();
    s.closing_curly_newline();
}
