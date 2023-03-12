use crate::parser::types::array::{ArraySize, ArrayType};
use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::objects::Objects;
use crate::parser::types::tags::MemberTags;
use crate::rust_printer::rust_view::{RustMember, RustType};
use crate::rust_printer::{
    print_docc_description_and_comment, print_member_docc_description_and_comment,
    print_serde_derive, Writer, CLIENT_MESSAGE_TRAIT_NAME, SERVER_MESSAGE_TRAIT_NAME,
};
use crate::wowm_printer::get_struct_wowm_definition;

pub(crate) mod print_common_impls;
mod print_new_types;
mod print_optional;
mod print_tests;

pub(crate) fn print_struct(e: &Container, o: &Objects) -> Writer {
    let mut s = Writer::new();

    print_includes(&mut s, e);

    print_declaration(&mut s, e, o);

    print_common_impls::print_common_impls(&mut s, e, o);

    print_new_types::print_new_types(&mut s, e);

    print_tests::print_tests(&mut s, e, o);

    if let Some(optional) = e.rust_object().optional() {
        print_optional::print_optional(&mut s, optional);
    }

    s
}

fn print_includes(s: &mut Writer, e: &Container) {
    match e.container_type() {
        ContainerType::CLogin(_) => {
            s.wln(format!("use crate::{CLIENT_MESSAGE_TRAIT_NAME};"));
        }
        ContainerType::SLogin(_) => {
            s.wln(format!("use crate::{SERVER_MESSAGE_TRAIT_NAME};"));
        }
        ContainerType::Msg(_) | ContainerType::CMsg(_) | ContainerType::SMsg(_) => {
            if e.tags().compressed() || e.contains_compressed_variable() {
                // We manually implement the trait to avoid double compression
                s.wln("use crate::Message;");
            }
        }
        ContainerType::Struct => {}
    }

    s.wln("use std::io::{Read, Write};");

    let (version, _) = e.tags().first_and_main_versions();
    for import in e.get_imports(version) {
        s.wln(format!("use {import};"));
    }

    s.newline();
}

fn print_declaration(s: &mut Writer, e: &Container, o: &Objects) {
    print_derives(s, &e.rust_object().all_members(), false);
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

pub(crate) fn print_derives(s: &mut Writer, members: &[&RustMember], is_enum_type: bool) {
    s.w("#[derive(Debug, Clone");

    if can_derive_copy(members) {
        s.w_no_indent(", Copy");
    }

    s.w_no_indent(", PartialEq");
    if can_derive_eq(members) {
        s.w_no_indent(", Eq, Hash");
    }

    s.w_no_indent(", PartialOrd");
    if can_derive_ord(members) {
        s.w_no_indent(", Ord");
    }

    if !is_enum_type && can_derive_default(members) {
        s.w_no_indent(", Default");
    }

    s.wln_no_indent(")]");
}

fn can_derive_ord(members: &[&RustMember]) -> bool {
    members.iter().all(|a| {
        !matches!(a.ty(), RustType::Floating(_) | RustType::MonsterMoveSpline)
            && can_derive_ord(&a.all_members_without_self())
    })
}

fn can_derive_eq(members: &[&RustMember]) -> bool {
    for m in members {
        if matches!(m.ty(), RustType::Floating(_) | RustType::MonsterMoveSpline) {
            return false;
        }
    }

    true
}

fn can_derive_default(members: &[&RustMember]) -> bool {
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

fn can_derive_copy(members: &[&RustMember]) -> bool {
    for m in members {
        match m.ty() {
            RustType::AchievementInProgressArray
            | RustType::AchievementDoneArray
            | RustType::MonsterMoveSpline
            | RustType::UpdateMask
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
