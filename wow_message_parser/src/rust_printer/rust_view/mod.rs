use conditional_union::ConditionalUnion;
use rust_enumerator::RustEnumerator;
use rust_member::RustMember;
use rust_object::RustObject;
use rust_optional::RustOptional;
use rust_type::RustType;

use crate::parser::types::array::{ArraySize, ArrayType};
use crate::parser::types::definer::Definer;
use crate::parser::types::if_statement::{Equation, IfStatement};
use crate::parser::types::parsed::parsed_container::ParsedContainer;
use crate::parser::types::parsed::parsed_struct_member::ParsedStructMember;
use crate::parser::types::sizes::Sizes;
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::tags::{MemberTags, ObjectTags};
use crate::parser::types::ty::Type;
use crate::rust_printer::{
    field_name_to_rust_name, get_new_flag_type_name, get_new_type_name, get_optional_type_name,
    DefinerType,
};

pub(crate) mod conditional_union;
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
    match statement.equation() {
        Equation::NotEquals { .. } => {}
        Equation::BitwiseAnd { values: value } | Equation::Equals { values: value } => {
            assert_eq!(value.len(), 1)
        }
    }
    assert!(statement.else_members().is_empty());

    let enumerator = match statement.equation() {
        Equation::BitwiseAnd { values: value } => &value[0],
        Equation::Equals { .. } | Equation::NotEquals { .. } => unreachable!(),
    };

    // Move enumerators into new RustMember
    let subject = find_subject(current_scope, statement);
    let main_enum = match &mut subject.ty {
        RustType::Flag {
            is_elseif,
            enumerators,
            ..
        } => {
            let main_enum = enumerators
                .iter()
                .find(|a| a.name() == enumerator)
                .unwrap()
                .clone();

            enumerators
                .iter_mut()
                .find(|a| a.name() == enumerator)
                .unwrap()
                .members
                .clear();
            *is_elseif = true;

            main_enum
        }
        _ => unreachable!(),
    };

    // Push enumerators
    let mut enumerators = vec![main_enum];

    // Append elseifs
    for elseif in statement.else_ifs() {
        let name = match elseif.equation() {
            Equation::BitwiseAnd { values: value } => &value[0],
            Equation::Equals { .. } | Equation::NotEquals { .. } => unreachable!(),
        };
        let enumerator = subject.pop_flag_enumerator(name);
        enumerators.push(enumerator);
    }

    let flag_int_ty = match subject.ty() {
        RustType::Flag { int_ty, .. } => *int_ty,
        _ => unreachable!(),
    };
    let flag_ty_name = &subject.original_ty;

    // Create new Enum RustMember
    let rm = RustMember::new(
        statement.variable_name().to_string(),
        RustType::Enum {
            ty_name: get_new_flag_type_name(flag_ty_name, &field_name_to_rust_name(enumerator)),
            original_ty_name: flag_ty_name.clone(),
            enumerators,
            int_ty: flag_int_ty,
            is_simple: false,
            is_elseif: true,
            separate_if_statements: false,
            is_single_rust_definer: false,
        },
        struct_ty_name.to_string(),
        true,
        MemberTags::new(), // TODO Which tags should go in here?
    );

    // Move RustMember into
    subject.append_members_to_enumerator_equal_and_set_elseif(
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
        .find(|a| statement.variable_name() == a.name())
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

    match statement.equation() {
        Equation::Equals { values: value } | Equation::BitwiseAnd { values: value } => {
            for v in value {
                main_enumerators.push(v);
            }
        }
        Equation::NotEquals { value } => {
            main_enumerators.push(value);
            reversed = true;
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

    let subject = find_subject(current_scope, statement);
    subject.set_main_enumerators(&main_enumerators);
    if reversed {
        // Apply main to all except main_enumerators
        for i in &main_enumerators {
            subject.append_members_to_enumerator_not_equal(
                i,
                &main_enumerator_members,
                &main_enumerator_originals,
            );
        }

        // Apply other to main_enumerator
        for i in &main_enumerators {
            subject.append_members_to_enumerator_equal(
                i,
                &else_enumerator_members,
                &else_enumerator_originals,
            );
        }
    } else {
        // Apply main to main_enumerator
        for i in &main_enumerators {
            subject.append_members_to_enumerator_equal(
                i,
                &main_enumerator_members,
                &main_enumerator_originals,
            );
        }

        // Apply else_if to else_if, ..
        for else_if in statement.else_ifs() {
            let mut else_if_enumerators = Vec::new();
            match else_if.equation() {
                Equation::Equals { values: value } | Equation::BitwiseAnd { values: value } => {
                    for v in value {
                        main_enumerators.push(v);
                        else_if_enumerators.push(v);
                    }
                }
                Equation::NotEquals { .. } => unreachable!(),
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
                subject.append_members_to_enumerator_equal(
                    i,
                    &else_if_enumerator_members,
                    &else_if_originals,
                );
            }
        }

        // Apply other to other_enumerators
        subject.append_members_to_enumerator_not_equal_range(
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
            current_scope.push(create_struct_member_definition(e, containers, definers, d));
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
    d: &StructMemberDefinition,
) -> RustMember {
    let mut in_rust_type = true;
    let ty = match d.ty() {
        Type::Integer(i) => {
            if d.is_manual_size_field() || d.used_as_size_in().is_some() || d.value().is_some() {
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
        Type::String => RustType::String,
        Type::Array(array) => RustType::Array {
            array: array.clone(),
            inner_sizes: array.ty().sizes(),
        },
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
                    is_single_rust_definer: false,
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
        Type::Population => RustType::Population,
        Type::Spell => RustType::Spell,
        Type::Spell16 => RustType::Spell16,
        Type::Item => RustType::Item,
        Type::CacheMask => RustType::CacheMask,
    };

    let name = d.name().to_string();
    let mut sizes = d.ty().sizes();

    for m in e.fields() {
        match m {
            ParsedStructMember::Definition(_) => {}
            ParsedStructMember::IfStatement(statement) => {
                if statement.variable_name() != name {
                    continue;
                }

                let complex_sizes =
                    ParsedContainer::get_complex_sizes(statement, e, containers, definers);
                sizes += complex_sizes;
            }
            ParsedStructMember::OptionalStatement(_) => {}
        }
    }

    RustMember::new(name, ty, d.ty().str(), in_rust_type, d.tags().clone())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MovementSplineBranchKind {
    Linear,
    Full,
}

struct ConditionalUnionCandidate<'a> {
    statement_index: usize,
    statement: &'a IfStatement,
    selector_type_name: String,
    condition_flags: Vec<String>,
    full_when_condition: bool,
}

fn movement_spline_branch_kind(members: &[StructMember]) -> Option<MovementSplineBranchKind> {
    let definitions: Vec<_> = members
        .iter()
        .map(|member| match member {
            StructMember::Definition(definition) => Some(definition),
            StructMember::IfStatement(_) | StructMember::OptionalStatement(_) => None,
        })
        .collect::<Option<_>>()?;

    if definitions.len() == 1 && matches!(definitions[0].ty(), Type::MonsterMoveSplines) {
        return Some(MovementSplineBranchKind::Linear);
    }

    let arrays: Vec<_> = definitions
        .iter()
        .filter_map(|definition| match definition.ty() {
            Type::Array(array)
                if matches!(array.size(), ArraySize::Variable(_))
                    && matches!(array.ty(), ArrayType::Struct(_)) =>
            {
                Some(*definition)
            }
            _ => None,
        })
        .collect();

    let [array_definition] = arrays.as_slice() else {
        return None;
    };
    let Type::Array(array) = array_definition.ty() else {
        return None;
    };
    let ArraySize::Variable(_) = array.size() else {
        return None;
    };

    if definitions.iter().all(|definition| {
        std::ptr::eq(*definition, *array_definition)
            || definition.used_as_size_in().as_deref() == Some(array_definition.name())
    }) {
        Some(MovementSplineBranchKind::Full)
    } else {
        None
    }
}

fn conditional_union_candidate<'a>(
    members: &'a [StructMember],
) -> Option<ConditionalUnionCandidate<'a>> {
    let (statement_index, statement) = match members.last()? {
        StructMember::IfStatement(statement) => (members.len() - 1, statement),
        StructMember::Definition(_) | StructMember::OptionalStatement(_) => return None,
    };

    if statement.definer_type() != DefinerType::Flag
        || statement.else_ifs().len() != 0
        || !matches!(statement.equation(), Equation::BitwiseAnd { values } if !values.is_empty())
    {
        return None;
    }

    let full_when_condition = match (
        movement_spline_branch_kind(statement.members())?,
        movement_spline_branch_kind(statement.else_members())?,
    ) {
        (MovementSplineBranchKind::Full, MovementSplineBranchKind::Linear) => true,
        (MovementSplineBranchKind::Linear, MovementSplineBranchKind::Full) => false,
        _ => return None,
    };

    let Equation::BitwiseAnd { values } = statement.equation() else {
        return None;
    };

    let selector_type_name = members[..statement_index]
        .iter()
        .find_map(|member| match member {
            StructMember::Definition(definition)
                if definition.name() == statement.variable_name() =>
            {
                match definition.ty() {
                    Type::Flag { e, .. } => Some(e.name().to_string()),
                    _ => None,
                }
            }
            StructMember::Definition(_)
            | StructMember::IfStatement(_)
            | StructMember::OptionalStatement(_) => None,
        })?;

    Some(ConditionalUnionCandidate {
        statement_index,
        statement,
        selector_type_name,
        condition_flags: values.clone(),
        full_when_condition,
    })
}

fn create_union_record_object(
    record_name: String,
    owner_name: &str,
    members: &[StructMember],
    e: &ParsedContainer,
    containers: &[ParsedContainer],
    definers: &[Definer],
) -> RustObject {
    let mut rust_members = Vec::new();
    let mut optional = None;

    for member in members {
        create_struct_member(
            member,
            owner_name,
            e.tags(),
            e,
            containers,
            definers,
            &mut rust_members,
            &mut optional,
        );
    }

    for member in &mut rust_members {
        set_simple_objects_name(member, owner_name);
    }

    RustObject::new(
        record_name,
        rust_members,
        optional,
        Sizes::exact(0, usize::MAX as i128),
        None,
    )
}

fn create_conditional_union(
    candidate: &ConditionalUnionCandidate<'_>,
    members: &[StructMember],
    e: &ParsedContainer,
    containers: &[ParsedContainer],
    definers: &[Definer],
) -> ConditionalUnion {
    let common_members = members[..candidate.statement_index].to_vec();
    let linear_members = if movement_spline_branch_kind(candidate.statement.members())
        == Some(MovementSplineBranchKind::Linear)
    {
        candidate.statement.members().to_vec()
    } else {
        candidate.statement.else_members().to_vec()
    };
    let full_members = if candidate.full_when_condition {
        candidate.statement.members().to_vec()
    } else {
        candidate.statement.else_members().to_vec()
    };

    let base_name = e
        .name()
        .strip_suffix("Variant")
        .filter(|name| !name.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| format!("{}Data", e.name()));
    let full_record_name = format!("Full{base_name}");
    let linear_record_name = base_name;

    let linear_object = create_union_record_object(
        linear_record_name.clone(),
        e.name(),
        &[common_members.clone(), linear_members.clone()].concat(),
        e,
        containers,
        definers,
    );
    let full_object = create_union_record_object(
        full_record_name.clone(),
        e.name(),
        &[common_members.clone(), full_members.clone()].concat(),
        e,
        containers,
        definers,
    );

    ConditionalUnion::new(
        candidate.statement.variable_name().to_string(),
        candidate.selector_type_name.clone(),
        candidate.condition_flags.clone(),
        candidate.full_when_condition,
        common_members,
        linear_members,
        full_members,
        linear_record_name,
        linear_object,
        full_record_name,
        full_object,
    )
}

pub(crate) fn create_rust_object(
    e: &ParsedContainer,
    members: &[StructMember],
    containers: &[ParsedContainer],
    definers: &[Definer],
) -> RustObject {
    let candidate = conditional_union_candidate(members);
    let mut v = Vec::new();
    let mut optional = None;

    for (index, member) in members.iter().enumerate() {
        if candidate
            .as_ref()
            .is_some_and(|candidate| candidate.statement_index == index)
        {
            continue;
        }

        create_struct_member(
            member,
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

    let conditional_union = candidate
        .as_ref()
        .map(|candidate| create_conditional_union(candidate, members, e, containers, definers));

    let mut r = RustObject::new(
        e.name().to_string(),
        v,
        optional,
        e.create_sizes(containers, definers),
        conditional_union,
    );

    if r.single_rust_definer().is_none() {
        return r;
    }

    for m in r.members_mut() {
        match &mut m.ty {
            RustType::Enum {
                is_single_rust_definer,
                ty_name,
                ..
            } => {
                *ty_name = e.name().to_string();
                *is_single_rust_definer = true;
            }
            _ => {}
        }
    }

    r
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
