use crate::file_utils::{get_base_shared_path, get_import_path, get_world_shared_path};
use crate::parser::types::array::{ArraySize, ArrayType};
use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::objects::Objects;
use crate::parser::types::tags::MemberTags;
use crate::parser::types::version::Version;
use crate::rust_printer::rust_view::{RustMember, RustType};
use crate::rust_printer::{
    print_docc_description_and_comment, print_member_docc_description_and_comment,
    print_serde_derive, Writer, CLIENT_MESSAGE_TRAIT_NAME, SERVER_MESSAGE_TRAIT_NAME,
};
use crate::wowm_printer::get_struct_wowm_definition;

mod print_common_impls;
mod print_new_types;
mod print_optional;
mod print_tests;

pub(crate) fn print_struct(e: &Container, o: &Objects, version: Version) -> Writer {
    let mut s = Writer::new(&get_import_path(version));

    print_includes(&mut s, e, version);

    print_declaration(&mut s, e, o);

    print_common_impls::print_common_impls(&mut s, e, o);

    print_new_types::print_new_types(&mut s, e);

    print_tests::print_tests(&mut s, e, o);

    if let Some(optional) = e.rust_object().optional() {
        print_optional::print_optional(&mut s, optional);
    }

    s
}

fn print_includes(s: &mut Writer, e: &Container, version: Version) {
    s.wln("use std::convert::{TryFrom, TryInto};");

    if e.contains_guid_or_packed_guid() {
        s.wln("use crate::Guid;");
    }

    if e.contains_datetime() {
        s.wln("use crate::DateTime;");
    }

    let import_path = get_import_path(version);

    if e.contains_aura_mask() {
        s.wln(format!("use {}::AuraMask;", import_path));
    }

    if e.contains_update_mask() {
        s.wln(format!("use {}::UpdateMask;", import_path));
    }

    for c in e.get_objects_needing_import() {
        let version = if !version.is_world() {
            // Login messages need to use the real object and not the reexports
            c.tags().import_version()
        } else {
            version
        };

        let module_name = if e.tags().has_world_version() && e.tags().shared() {
            if c.tags().is_in_base() {
                get_base_shared_path(c.name(), c.tags())
            } else {
                get_world_shared_path(c.name(), c.tags())
            }
        } else {
            get_import_path(version)
        };

        s.wln(format!(
            "use {module_name}::{name};",
            module_name = module_name,
            name = c.name(),
        ));
    }

    match e.container_type() {
        ContainerType::CLogin(_) => {
            s.wln(format!("use crate::{};", CLIENT_MESSAGE_TRAIT_NAME));
        }
        ContainerType::SLogin(_) => {
            s.wln(format!("use crate::{};", SERVER_MESSAGE_TRAIT_NAME));
        }
        _ => {}
    }

    s.wln("use std::io::{Write, Read};");

    s.newline();
}

fn print_declaration(s: &mut Writer, e: &Container, o: &Objects) {
    print_derives(
        s,
        &e.rust_object()
            .all_members()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>(),
        false,
    );
    print_serde_derive(s, e.tags().is_in_base());

    print_docc_description_and_comment(s, e.tags(), o, e.tags());

    print_struct_wowm_definition(s, e);

    s.new_struct(e.name(), |s| {
        for member in e.rust_object().members_in_struct() {
            print_member_docc_description_and_comment(s, member.tags(), o, e.tags());

            s.wln(format!(
                "pub {name}: {ty},",
                name = member.name(),
                ty = member.ty(),
            ));
        }

        if let Some(optional) = e.rust_object().optional() {
            print_member_docc_description_and_comment(s, &MemberTags::new(), o, e.tags());
            s.wln(format!(
                "pub {name}: Option<{ty}>,",
                name = optional.name(),
                ty = optional.ty()
            ));
        }
    });
}

fn print_struct_wowm_definition(s: &mut Writer, e: &Container) {
    s.docc_wowm(
        |s| {
            s.w(get_struct_wowm_definition(e, "/// "));
        },
        e.file_info(),
    );
}

pub(crate) fn print_derives(s: &mut Writer, members: &[RustMember], is_enum_type: bool) {
    s.w("#[derive(Debug, Clone");

    if can_derive_copy(members) {
        s.w_no_indent(", Copy");
    }

    s.w_no_indent(", PartialEq");
    if can_derive_eq(members) {
        s.w_no_indent(", Eq, Hash");
    }

    if !is_enum_type && can_derive_default(members) {
        s.w_no_indent(", Default");
    }

    s.wln_no_indent(")]");
}

fn can_derive_eq(members: &[RustMember]) -> bool {
    for m in members {
        if matches!(m.ty(), RustType::Floating(_)) {
            return false;
        }
    }

    true
}

fn can_derive_default(members: &[RustMember]) -> bool {
    for m in members {
        if let RustType::Array { array, .. } = m.ty() {
            if let (ArrayType::Integer(_), ArraySize::Fixed(size)) = (array.ty(), array.size()) {
                if size > 32 {
                    return false;
                }
            }
        }
    }

    true
}

fn can_derive_copy(members: &[RustMember]) -> bool {
    for m in members {
        match m.ty() {
            RustType::UpdateMask
            | RustType::AuraMask
            | RustType::PackedGuid
            | RustType::String
            | RustType::CString
            | RustType::SizedCString => return false,
            RustType::Array {
                array, inner_sizes, ..
            } => match array.size() {
                ArraySize::Variable(_) | ArraySize::Endless => {
                    return false;
                }
                ArraySize::Fixed(_) => {
                    if inner_sizes.is_constant().is_none() {
                        return false;
                    }
                }
            },
            RustType::Struct { sizes, .. } => {
                if sizes.is_constant().is_none() {
                    return false;
                }
            }
            _ => {}
        }
    }

    true
}
