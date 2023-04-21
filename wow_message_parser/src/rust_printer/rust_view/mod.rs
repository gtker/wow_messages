use crate::parser::types::array::ArrayType;
use crate::parser::types::definer::Definer;
use crate::parser::types::if_statement::{Equation, IfStatement};
use crate::parser::types::parsed::parsed_container::ParsedContainer;
use crate::parser::types::parsed::parsed_struct_member::ParsedStructMember;
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::tags::{MemberTags, ObjectTags};
use crate::parser::types::ty::Type;
use crate::rust_printer::{
    field_name_to_rust_name, get_new_flag_type_name, get_new_type_name, get_optional_type_name,
    DefinerType,
};
use rust_enumerator::RustEnumerator;
use rust_member::RustMember;
use rust_object::RustObject;
use rust_optional::RustOptional;
use rust_type::RustType;

pub(crate) mod rust_definer;
pub(crate) mod rust_enumerator;
pub(crate) mod rust_member;
pub(crate) mod rust_object;
pub(crate) mod rust_optional;
pub(crate) mod rust_type;

fn create_else_if_flag(
    statement: &IfStatement,
    struct_ty_name: &str,
    current_scope: &mut [RustMember],
) {
    assert_eq!(statement.conditional().equations().len(), 1);
    assert!(statement.else_members().is_empty());

    let enumerator = match &statement.conditional().equations()[0] {
        Equation::BitwiseAnd { value } => value.as_str(),
        _ => unreachable!(),
    };

    // Move enumerators into new RustMember
    let main_enum = find_subject(current_scope, statement).get_flag_enumerator(enumerator);
    find_subject(current_scope, statement).clear_flag_enumerator(enumerator);
    find_subject(current_scope, statement).set_is_elseif();

    // Push enumerators
    let mut enumerators = vec![main_enum];

    // Append elseifs
    for elseif in statement.else_ifs() {
        let name = match &elseif.conditional().equations()[0] {
            Equation::BitwiseAnd { value } => value.to_string(),
            _ => unreachable!(),
        };
        let enumerator = find_subject(current_scope, elseif).pop_flag_enumerator(&name);
        enumerators.push(enumerator);
    }

    let flag_int_ty = match find_subject(current_scope, statement).ty() {
        RustType::Flag { int_ty, .. } => *int_ty,
        _ => unreachable!(),
    };
    let flag_ty_name = &find_subject(current_scope, statement).original_ty;

    // Create new Enum RustMember
    let rm = RustMember::new(
        statement.name().to_string(),
        RustType::Enum {
            ty_name: get_new_flag_type_name(flag_ty_name, &field_name_to_rust_name(enumerator)),
            original_ty_name: flag_ty_name.clone(),
            enumerators,
            int_ty: flag_int_ty,
            is_simple: false,
            is_elseif: true,
            separate_if_statements: false,
        },
        struct_ty_name.to_string(),
        true,
        MemberTags::new(), // TODO Which tags should go in here?
    );

    // Move RustMember into
    find_subject(current_scope, statement).append_members_to_enumerator_equal_and_set_elseif(
        enumerator,
        &[rm],
        &[StructMember::IfStatement(statement.clone())],
    );
}

fn find_subject<'a>(
    current_scope: &'a mut [RustMember],
    statement: &IfStatement,
) -> &'a mut RustMember {
    let subject = current_scope
        .iter_mut()
        .find(|a| statement.name() == a.name())
        .unwrap();
    subject
}

pub(crate) fn create_if_statement(
    statement: &IfStatement,
    struct_ty_name: &str,
    tags: &ObjectTags,
    containers: &[ParsedContainer],
    definers: &[Definer],
    e: &ParsedContainer,
    current_scope: &mut [RustMember],
) {
    let mut reversed = false;
    let mut main_enumerators = Vec::new();

    for i in statement.conditional().equations() {
        match i {
            Equation::BitwiseAnd { value } | Equation::Equals { value } => {
                main_enumerators.push(value)
            }
            Equation::NotEquals { value } => {
                main_enumerators.push(value);
                reversed = true;
            }
        }
    }

    let mut main_enumerator_members = Vec::new();
    let mut main_enumerator_originals = Vec::new();
    for m in statement.members() {
        create_struct_member(
            m,
            struct_ty_name,
            tags,
            e,
            containers,
            definers,
            &mut main_enumerator_members,
            &mut None,
        );

        main_enumerator_originals.push(m.clone());
    }

    let mut else_enumerator_members = Vec::new();
    let mut else_enumerator_originals = Vec::new();
    for m in statement.else_members() {
        create_struct_member(
            m,
            struct_ty_name,
            tags,
            e,
            containers,
            definers,
            &mut else_enumerator_members,
            &mut None,
        );

        else_enumerator_originals.push(m.clone());
    }

    find_subject(current_scope, statement).set_main_enumerators(&main_enumerators);
    if reversed {
        // Apply main to all except main_enumerators
        for i in &main_enumerators {
            find_subject(current_scope, statement).append_members_to_enumerator_not_equal(
                i,
                &main_enumerator_members,
                &main_enumerator_originals,
            );
        }

        // Apply other to main_enumerator
        for i in &main_enumerators {
            find_subject(current_scope, statement).append_members_to_enumerator_equal(
                i,
                &else_enumerator_members,
                &else_enumerator_originals,
            );
        }
    } else {
        // Apply main to main_enumerator
        for i in &main_enumerators {
            find_subject(current_scope, statement).append_members_to_enumerator_equal(
                i,
                &main_enumerator_members,
                &main_enumerator_originals,
            );
        }

        // Apply else_if to else_if, ..
        for else_if in statement.else_ifs() {
            let mut else_if_enumerators = Vec::new();
            for i in else_if.conditional().equations() {
                match i {
                    Equation::BitwiseAnd { value } | Equation::Equals { value } => {
                        main_enumerators.push(value);
                        else_if_enumerators.push(value);
                    }
                    Equation::NotEquals { .. } => unreachable!(),
                }
            }

            let mut else_if_enumerator_members = Vec::new();
            let mut else_if_originals = Vec::new();
            for m in else_if.members() {
                create_struct_member(
                    m,
                    struct_ty_name,
                    tags,
                    e,
                    containers,
                    definers,
                    &mut else_if_enumerator_members,
                    &mut None,
                );
                else_if_originals.push(m.clone());
            }

            for i in &else_if_enumerators {
                find_subject(current_scope, statement).append_members_to_enumerator_equal(
                    i,
                    &else_if_enumerator_members,
                    &else_if_originals,
                );
            }
        }

        // Apply other to other_enumerators
        find_subject(current_scope, statement).append_members_to_enumerator_not_equal_range(
            &main_enumerators,
            &else_enumerator_members,
            &else_enumerator_originals,
        );
    }

    if statement.is_elseif_flag() {
        create_else_if_flag(statement, struct_ty_name, current_scope);
    }
}

pub(crate) fn create_struct_member(
    m: &StructMember,
    struct_ty_name: &str,
    tags: &ObjectTags,
    e: &ParsedContainer,
    containers: &[ParsedContainer],
    definers: &[Definer],
    current_scope: &mut Vec<RustMember>,
    optional: &mut Option<RustOptional>,
) {
    match m {
        StructMember::Definition(d) => {
            create_struct_member_definition(e, containers, definers, current_scope, d);
        }
        StructMember::IfStatement(statement) => {
            create_if_statement(
                statement,
                struct_ty_name,
                tags,
                containers,
                definers,
                e,
                current_scope,
            );
        }
        StructMember::OptionalStatement(option) => {
            let mut members = Vec::new();

            for i in option.members() {
                create_struct_member(
                    i,
                    struct_ty_name,
                    tags,
                    e,
                    containers,
                    definers,
                    &mut members,
                    &mut None,
                );
            }

            *optional = Some(RustOptional::new(
                option.name().to_string(),
                get_optional_type_name(struct_ty_name, option.name()),
                members,
            ));
        }
    }
}

fn create_struct_member_definition(
    e: &ParsedContainer,
    containers: &[ParsedContainer],
    definers: &[Definer],
    current_scope: &mut Vec<RustMember>,
    d: &StructMemberDefinition,
) {
    let mut in_rust_type = true;
    let ty = match d.ty() {
        Type::Integer(i) => {
            if d.used_as_size_in().is_some() || d.value().is_some() {
                in_rust_type = false;
            }
            RustType::Integer(*i)
        }
        Type::Bool(i) => RustType::Bool(*i),
        Type::DateTime => RustType::DateTime,
        Type::Guid => RustType::Guid,
        Type::NamedGuid => RustType::NamedGuid,
        Type::PackedGuid => RustType::PackedGuid,
        Type::FloatingPoint => RustType::Floating,
        Type::CString => RustType::CString,
        Type::String { .. } => RustType::String,
        Type::Array(array) => {
            let inner_object = match array.ty() {
                ArrayType::Struct(c) => Some(c.rust_object().clone()),
                ArrayType::Integer(_)
                | ArrayType::Guid
                | ArrayType::PackedGuid
                | ArrayType::CString => None,
            };

            RustType::Array {
                array: array.clone(),
                inner_sizes: array.ty().sizes(),
                inner_object,
            }
        }
        Type::Enum { e: definer, upcast } | Type::Flag { e: definer, upcast } => {
            let add_types = || -> Vec<RustEnumerator> {
                let mut enumerators = Vec::new();

                for field in definer.fields() {
                    enumerators.push(RustEnumerator::new(
                        field.name().to_string(),
                        field.rust_name().to_string(),
                        field.value().clone(),
                        vec![],
                        false,
                        vec![],
                        false,
                    ));
                }
                enumerators
            };
            let int_ty = if let Some(upcast) = upcast {
                *upcast
            } else {
                *definer.ty()
            };

            if definer.definer_ty() == DefinerType::Enum {
                let enumerators = add_types();

                RustType::Enum {
                    ty_name: definer.name().to_string(),
                    original_ty_name: definer.name().to_string(),
                    enumerators,
                    int_ty,
                    is_simple: true,
                    is_elseif: false,
                    separate_if_statements: e
                        .enum_type_used_in_separate_if_statements(definer.name()),
                }
            } else {
                let enumerators = add_types();

                RustType::Flag {
                    ty_name: definer.name().to_string(),
                    original_ty_name: definer.name().to_string(),
                    int_ty,
                    enumerators,
                    is_simple: true,
                    is_elseif: false,
                }
            }
        }
        Type::Struct { e } => {
            let object = e.rust_object().clone();

            RustType::Struct {
                ty_name: e.name().to_string(),
                sizes: e.sizes(),
                object,
            }
        }
        Type::UpdateMask { max_size } => RustType::UpdateMask {
            max_size: *max_size,
        },
        Type::AuraMask => RustType::AuraMask,
        Type::SizedCString => RustType::SizedCString,
        Type::AchievementDoneArray => RustType::AchievementDoneArray,
        Type::AchievementInProgressArray => RustType::AchievementInProgressArray,
        Type::MonsterMoveSplines => RustType::MonsterMoveSpline,
        Type::EnchantMask => RustType::EnchantMask,
        Type::InspectTalentGearMask => RustType::InspectTalentGearMask,
        Type::Gold => RustType::Gold,
        Type::Level => RustType::Level,
        Type::Level16 => RustType::Level16,
        Type::Level32 => RustType::Level32,
        Type::VariableItemRandomProperty => RustType::VariableItemRandomProperty,
        Type::AddonArray => RustType::AddonArray,
        Type::IpAddress => RustType::IpAddress,
        Type::Seconds => RustType::Seconds,
        Type::Milliseconds => RustType::Milliseconds,
    };

    let name = d.name().to_string();
    let mut sizes = d.ty().sizes();

    for m in e.fields() {
        match m {
            ParsedStructMember::Definition(_) => {}
            ParsedStructMember::IfStatement(statement) => {
                if statement.name() != name {
                    continue;
                }

                let complex_sizes =
                    ParsedContainer::get_complex_sizes(statement, e, containers, definers);
                sizes += complex_sizes;
            }
            ParsedStructMember::OptionalStatement(_) => {}
        }
    }

    current_scope.push(RustMember::new(
        name,
        ty,
        d.ty().str(),
        in_rust_type,
        d.tags().clone(),
    ));
}

pub(crate) fn create_rust_object(
    e: &ParsedContainer,
    members: &[StructMember],
    containers: &[ParsedContainer],
    definers: &[Definer],
) -> RustObject {
    let mut v = Vec::new();
    let mut optional = None;

    for m in members {
        create_struct_member(
            m,
            e.name(),
            e.tags(),
            e,
            containers,
            definers,
            &mut v,
            &mut optional,
        );
    }

    for m in &mut v {
        set_simple_objects_name(m, e.name());
    }

    RustObject::new(
        e.name().to_string(),
        v,
        optional,
        e.create_sizes(containers, definers),
    )
}

fn set_simple_objects_name(m: &mut RustMember, struct_ty_name: &str) {
    match &mut m.ty {
        RustType::Enum {
            ty_name,
            is_simple,
            enumerators,
            ..
        }
        | RustType::Flag {
            ty_name,
            is_simple,
            enumerators,
            ..
        } => {
            if !(*is_simple) {
                let definer_ty = ty_name.clone();
                *ty_name = get_new_type_name(struct_ty_name, &definer_ty);

                for e in enumerators {
                    for f in &mut e.members {
                        set_simple_objects_name(f, struct_ty_name);
                    }
                }
            }
        }
        _ => {}
    }
}
