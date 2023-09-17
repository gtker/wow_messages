use crate::error_printer::{
    complex_not_found, enum_has_bitwise_and, flag_used_as_equals_or_not_equals,
    type_is_upcast_to_same, variable_in_if_not_found,
};
use crate::file_info::FileInfo;
use crate::parser::types::array::{Array, ArraySize, ArrayType};
use crate::parser::types::definer::Definer;
use crate::parser::types::if_statement::{Equation, IfStatement};
use crate::parser::types::objects::conversion;
use crate::parser::types::objects::conversion::{
    all_definitions, all_definitions_mut, get_container, get_definer, get_related,
    parsed_container_to_container,
};
use crate::parser::types::optional::OptionalStatement;
use crate::parser::types::parsed::parsed_array::{ParsedArray, ParsedArraySize, ParsedArrayType};
use crate::parser::types::parsed::parsed_container::ParsedContainer;
use crate::parser::types::parsed::parsed_if_statement::ParsedIfStatement;
use crate::parser::types::parsed::parsed_struct_member::{
    ParsedStructMember, ParsedStructMemberDefinition,
};
use crate::parser::types::parsed::parsed_test_case::{
    ParsedTestCase, ParsedTestCaseMember, ParsedTestValue,
};
use crate::parser::types::parsed::parsed_ty::ParsedType;
use crate::parser::types::sizes::update_mask_max;
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::test_case::{
    TestCase, TestCaseMember, TestUpdateMaskValue, TestValue, TestVector3d,
};
use crate::parser::types::ty::Type;
use crate::parser::types::ContainerValue;
use crate::parser::utility::parse_value;
use crate::rust_printer::UpdateMaskObjectType;
use crate::{DefinerType, ObjectTags, CONTAINER_SELF_SIZE_FIELD};

pub(crate) fn verify_and_set_members(
    members: &mut [ParsedStructMember],
    tags: &ObjectTags,
    containers: &[ParsedContainer],
    definers: &[Definer],
    fileinfo: &FileInfo,
) {
    set_used_as_size_in(members);

    set_verified_values(members, definers, tags);

    check_complex_types_exist(members, containers, definers, tags, fileinfo);
}

fn parsed_type_to_type(
    p: &ParsedContainer,
    containers: &[ParsedContainer],
    definers: &[Definer],
    t: ParsedType,
    tags: &ObjectTags,
) -> Type {
    match t {
        ParsedType::Integer(i) => Type::Integer(i),
        ParsedType::Bool(i) => Type::Bool(i),
        ParsedType::PackedGuid => Type::PackedGuid,
        ParsedType::Guid => Type::Guid,
        ParsedType::NamedGuid => Type::NamedGuid,
        ParsedType::DateTime => Type::DateTime,
        ParsedType::FloatingPoint => Type::FloatingPoint,
        ParsedType::CString => Type::CString,
        ParsedType::SizedCString => Type::SizedCString,
        ParsedType::String => Type::String,
        ParsedType::Array(a) => {
            Type::Array(parsed_array_to_array(p, a, containers, definers, tags))
        }
        ParsedType::Identifier { s, upcast } => {
            if let Some(e) = get_definer(definers, &s, tags) {
                if let Some(upcast) = &upcast {
                    if upcast == e.ty() {
                        type_is_upcast_to_same(&s, &p.file_info, *e.ty());
                    }
                }

                match e.definer_ty() {
                    DefinerType::Enum => Type::Enum {
                        e: e.clone(),
                        upcast,
                    },
                    DefinerType::Flag => Type::Flag {
                        e: e.clone(),
                        upcast,
                    },
                }
            } else if let Some(e) = get_container(containers, &s, tags) {
                Type::Struct {
                    e: parsed_container_to_container(e.clone(), containers, definers),
                }
            } else {
                let related = get_related(containers, definers, &s);
                complex_not_found(p.name(), p.tags(), &p.file_info, &s, &related);
            }
        }
        ParsedType::UpdateMask => Type::UpdateMask {
            max_size: update_mask_max(tags.main_versions().next().unwrap().as_major_world()),
        },
        ParsedType::AuraMask => Type::AuraMask,
        ParsedType::AchievementDoneArray => Type::AchievementDoneArray,
        ParsedType::AchievementInProgressArray => Type::AchievementInProgressArray,
        ParsedType::MonsterMoveSpline => Type::MonsterMoveSplines,
        ParsedType::EnchantMask => Type::EnchantMask,
        ParsedType::InspectTalentGearMask => Type::InspectTalentGearMask,
        ParsedType::Gold => Type::Gold,
        ParsedType::Level => Type::Level,
        ParsedType::Level16 => Type::Level16,
        ParsedType::Level32 => Type::Level32,
        ParsedType::VariableItemRandomProperty => Type::VariableItemRandomProperty,
        ParsedType::AddonArray => Type::AddonArray,
        ParsedType::IpAddress => Type::IpAddress,
        ParsedType::Seconds => Type::Seconds,
        ParsedType::Milliseconds => Type::Milliseconds,
        ParsedType::Population => Type::Population,
    }
}

fn parsed_array_to_array(
    p: &ParsedContainer,
    array: ParsedArray,
    containers: &[ParsedContainer],
    definers: &[Definer],
    tags: &ObjectTags,
) -> Array {
    let size = match array.size() {
        ParsedArraySize::Fixed(v) => ArraySize::Fixed(v),
        ParsedArraySize::Variable(v) => {
            let m = p.get_field(&v);
            let m = parsed_struct_member_definition_to_struct_member(
                p, containers, definers, tags, m, None,
            );
            ArraySize::Variable(Box::new(m))
        }
        ParsedArraySize::Endless => ArraySize::Endless,
    };

    let inner = match array.ty() {
        ParsedArrayType::Integer(i) => ArrayType::Integer(*i),
        ParsedArrayType::Complex(c) => {
            let c = parsed_container_to_container(
                get_container(containers, c, tags).unwrap().clone(),
                containers,
                definers,
            );
            ArrayType::Struct(Box::new(c))
        }
        ParsedArrayType::CString => ArrayType::CString,
        ParsedArrayType::Guid => ArrayType::Guid,
        ParsedArrayType::PackedGuid => ArrayType::PackedGuid,
    };

    Array::new(inner, size, array.compressed)
}

fn parsed_struct_member_definition_to_struct_member(
    c: &ParsedContainer,
    containers: &[ParsedContainer],
    definers: &[Definer],
    tags: &ObjectTags,
    d: ParsedStructMemberDefinition,
    size_of_fields_before_size: Option<i128>,
) -> StructMemberDefinition {
    let (is_manual_size_field, value) = if let Some(v) = d.verified_value {
        if v.original_string() == CONTAINER_SELF_SIZE_FIELD {
            (true, None)
        } else {
            (false, Some(v))
        }
    } else {
        (false, None)
    };

    StructMemberDefinition::new(
        d.name,
        parsed_type_to_type(c, containers, definers, d.struct_type, tags),
        value,
        d.used_as_size_in,
        d.used_in_if.unwrap(),
        if is_manual_size_field {
            size_of_fields_before_size
        } else {
            None
        },
        d.tags,
    )
}

fn parsed_member_to_member(
    c: &ParsedContainer,
    m: ParsedStructMember,
    containers: &[ParsedContainer],
    definers: &[Definer],
    tags: &ObjectTags,
    size_of_fields_before_size: Option<i128>,
) -> StructMember {
    match m {
        ParsedStructMember::Definition(d) => {
            StructMember::Definition(parsed_struct_member_definition_to_struct_member(
                c,
                containers,
                definers,
                tags,
                *d,
                size_of_fields_before_size,
            ))
        }
        ParsedStructMember::IfStatement(s) => {
            let member = c.get_field_ty(s.conditional.variable_name()).str();
            let definer = get_definer(definers, &member, c.tags()).unwrap();

            match s.get_conditional().equation() {
                Equation::Equals { values: value } | Equation::BitwiseAnd { values: value } => {
                    for v in value {
                        if definer.get_field_with_name(v).is_none() {
                            variable_in_if_not_found(
                                s.conditional.variable_name(),
                                v,
                                &c.file_info,
                                definer.name(),
                            );
                        }
                    }
                }
                Equation::NotEquals { value } => {
                    if definer.get_field_with_name(value).is_none() {
                        variable_in_if_not_found(
                            s.conditional.variable_name(),
                            value,
                            &c.file_info,
                            definer.name(),
                        );
                    }
                }
            }

            let separate_if_statement = c.enum_variable_used_in_separate_if_statements(s.name());

            StructMember::IfStatement(IfStatement::new(
                s.conditional,
                parsed_members_to_members(c, s.members, containers, definers, tags, None),
                parsed_if_statement_to_if_statement(c, s.else_ifs, containers, definers, tags),
                parsed_members_to_members(
                    c,
                    s.else_statement_members,
                    containers,
                    definers,
                    tags,
                    None,
                ),
                parsed_type_to_type(c, containers, definers, s.original_ty.unwrap(), tags),
                separate_if_statement,
            ))
        }
        ParsedStructMember::OptionalStatement(o) => {
            StructMember::OptionalStatement(OptionalStatement::new(
                o.name,
                parsed_members_to_members(c, o.members, containers, definers, tags, None),
            ))
        }
    }
}

pub(crate) fn parsed_members_to_members(
    c: &ParsedContainer,
    members: Vec<ParsedStructMember>,
    containers: &[ParsedContainer],
    definers: &[Definer],
    tags: &ObjectTags,
    size_of_fields_before_size: Option<i128>,
) -> Vec<StructMember> {
    let mut v = Vec::with_capacity(members.len());

    for m in members {
        v.push(parsed_member_to_member(
            c,
            m,
            containers,
            definers,
            tags,
            size_of_fields_before_size,
        ));
    }

    v
}

fn parsed_if_statement_to_if_statement(
    c: &ParsedContainer,
    parsed: Vec<ParsedIfStatement>,
    containers: &[ParsedContainer],
    definers: &[Definer],
    tags: &ObjectTags,
) -> Vec<IfStatement> {
    let mut v = Vec::with_capacity(parsed.len());

    for p in parsed {
        let separate_if_statement = c.enum_variable_used_in_separate_if_statements(p.name());

        v.push(IfStatement::new(
            p.conditional,
            parsed_members_to_members(c, p.members, containers, definers, tags, None),
            parsed_if_statement_to_if_statement(c, p.else_ifs, containers, definers, tags),
            parsed_members_to_members(
                c,
                p.else_statement_members,
                containers,
                definers,
                tags,
                None,
            ),
            parsed_type_to_type(c, containers, definers, p.original_ty.unwrap(), tags),
            separate_if_statement,
        ))
    }

    v
}

fn set_used_as_size_in(members: &mut [ParsedStructMember]) {
    let mut variables_used_as_size_in = Vec::new();

    for d in all_definitions(members) {
        if let ParsedType::Array(array) = d.ty() {
            if let ParsedArraySize::Variable(length) = array.size() {
                if length.parse::<u8>().is_err() {
                    variables_used_as_size_in.push((d.name().to_string(), length.to_string()));
                }
            }
        }
    }

    fn contains<'a>(v: &'a [(String, String)], name: &str) -> Option<&'a (String, String)> {
        v.iter().find(|a| a.1 == name)
    }

    for d in all_definitions_mut(members) {
        if let Some((var, _)) = contains(&variables_used_as_size_in, d.name()) {
            d.set_used_as_size_in(var.clone());
        }
    }
}

fn set_verified_values(
    members: &mut [ParsedStructMember],
    definers: &[Definer],
    tags: &ObjectTags,
) {
    for d in all_definitions_mut(members) {
        d.set_verified_value(definers, tags);
    }
}

fn contains_complex_type(
    containers: &[ParsedContainer],
    definers: &[Definer],
    ty_name: &str,
    tags: &ObjectTags,
    struct_name: &str,
    struct_fileinfo: &FileInfo,
) {
    for e in definers {
        if e.name() == ty_name && e.tags().fulfills_all(tags) {
            return;
        }
    }

    for e in containers {
        if e.name() == ty_name && e.tags().fulfills_all(tags) {
            return;
        }
    }

    let related = get_related(containers, definers, ty_name);
    complex_not_found(struct_name, tags, struct_fileinfo, ty_name, &related);
}

fn check_complex_types_exist(
    members: &[ParsedStructMember],
    containers: &[ParsedContainer],
    definers: &[Definer],
    tags: &ObjectTags,
    fileinfo: &FileInfo,
) {
    for d in all_definitions(members) {
        match &d.ty() {
            ParsedType::Array(a) => {
                if let ParsedArrayType::Complex(c) = &a.ty() {
                    contains_complex_type(containers, definers, c, tags, d.name(), fileinfo)
                }
            }
            ParsedType::Identifier { s: i, .. } => {
                contains_complex_type(containers, definers, i, tags, d.name(), fileinfo);

                match d.value() {
                    None => {}
                    Some(v) => match v.identifier().parse::<usize>() {
                        Ok(_) => {}
                        Err(_) => {
                            let e = get_definer(definers, i, tags).unwrap();
                            e.get_field_with_name(v.identifier()).unwrap();
                        }
                    },
                }
            }
            _ => {}
        }
    }
}

pub(crate) fn check_if_statement_operators(e: &ParsedContainer, definers: &[Definer]) {
    fn inner(m: &ParsedStructMember, e: &ParsedContainer, definers: &[Definer]) {
        match m {
            ParsedStructMember::IfStatement(statement) => {
                let ty = match e.get_field_ty(statement.name()) {
                    ParsedType::Identifier { s, .. } => s,
                    _ => unreachable!(),
                };

                let definer = get_definer(definers, ty, e.tags()).unwrap();
                match definer.definer_ty() {
                    DefinerType::Enum => match statement.get_conditional().equation() {
                        Equation::Equals { .. } | Equation::NotEquals { .. } => {}
                        Equation::BitwiseAnd { .. } => {
                            enum_has_bitwise_and(
                                e.name(),
                                statement.name(),
                                definer.name(),
                                &e.file_info,
                            );
                        }
                    },
                    DefinerType::Flag => match statement.get_conditional().equation() {
                        Equation::Equals { .. } | Equation::NotEquals { .. } => {
                            flag_used_as_equals_or_not_equals(
                                e.name(),
                                statement.name(),
                                definer.name(),
                                &e.file_info,
                            );
                        }
                        Equation::BitwiseAnd { .. } => {}
                    },
                }

                for m in statement.all_members() {
                    inner(m, e, definers);
                }
            }
            ParsedStructMember::OptionalStatement(optional) => {
                for m in optional.members() {
                    inner(m, e, definers);
                }
            }
            ParsedStructMember::Definition(_) => {}
        }
    }

    for m in e.fields() {
        inner(m, e, definers);
    }
}

fn convert_parsed_test_case_value_to_test_case_value(
    variable_name: &str,
    test: ParsedTestValue,
    c: &ParsedContainer,
    containers: &[ParsedContainer],
    enums: &[Definer],
    flags: &[Definer],
    definers: &[Definer],
) -> TestValue {
    let ty = c.get_field_ty(variable_name);

    let value = match test {
        ParsedTestValue::Single(s) => s,
        ParsedTestValue::Multiple(mut multiple) => {
            if ty == &ParsedType::UpdateMask {
                let mut v = Vec::new();
                for m_inner in multiple.iter_mut() {
                    let (ty, name) = &m_inner.variable_name.split_once('_').unwrap();
                    let ty = match *ty {
                        "OBJECT" => UpdateMaskObjectType::Object,
                        "UNIT" => UpdateMaskObjectType::Unit,
                        "ITEM" => UpdateMaskObjectType::Item,
                        "PLAYER" => UpdateMaskObjectType::Player,
                        "CONTAINER" => UpdateMaskObjectType::Container,
                        "GAMEOBJECT" => UpdateMaskObjectType::GameObject,
                        "DYNAMICOBJECT" => UpdateMaskObjectType::DynamicObject,
                        "CORPSE" => UpdateMaskObjectType::Corpse,
                        _ => panic!("invalid update mask type: '{ty}'"),
                    };

                    let value = match &m_inner.value {
                        ParsedTestValue::Single(v) => v.clone(),
                        _ => unreachable!(),
                    };

                    v.push(TestUpdateMaskValue::new(ty, name.to_string(), value))
                }

                return TestValue::UpdateMask(v);
            }

            let mut members = Vec::with_capacity(multiple.len());
            let inner_c = get_container(containers, ty.str().as_str(), c.tags()).unwrap();
            for m_inner in multiple {
                members.push(convert_test_case_member_to_test_case(
                    m_inner, inner_c, containers, enums, flags, definers,
                ));
            }

            let c = parsed_container_to_container(inner_c.clone(), containers, definers);

            return TestValue::SubObject { c, members };
        }
        ParsedTestValue::ArrayOfMultiple(array) => {
            let mut v = Vec::new();

            let ty_name = match ty {
                ParsedType::Array(array) => match array.ty() {
                    ParsedArrayType::Integer(_) => unimplemented!(),
                    ParsedArrayType::Complex(c) => c.as_str(),
                    ParsedArrayType::CString => unimplemented!(),
                    ParsedArrayType::Guid => "Guid",
                    ParsedArrayType::PackedGuid => "Guid",
                },
                ParsedType::MonsterMoveSpline => {
                    let mut v = Vec::new();

                    for multiple in array {
                        let f = |x: &ParsedTestCaseMember| match &x.value {
                            ParsedTestValue::Single(v) => v.parse::<f32>().unwrap(),
                            ParsedTestValue::Multiple(_) | ParsedTestValue::ArrayOfMultiple(_) => {
                                unreachable!()
                            }
                        };

                        let x = multiple.iter().find(|a| a.variable_name == "x").unwrap();
                        let x = f(x);

                        let y = multiple.iter().find(|a| a.variable_name == "y").unwrap();
                        let y = f(y);

                        let z = multiple.iter().find(|a| a.variable_name == "z").unwrap();
                        let z = f(z);

                        v.push(TestVector3d { x, y, z })
                    }

                    return TestValue::MonsterMoveSpline(v);
                }
                _ => unimplemented!(),
            };

            for multiple in array {
                let mut members = Vec::new();
                let inner_c = conversion::get_container(containers, ty_name, c.tags()).unwrap();

                for m_inner in multiple {
                    members.push(convert_test_case_member_to_test_case(
                        m_inner, inner_c, containers, enums, flags, definers,
                    ));
                }

                v.push(members);
            }

            let array = match ty {
                ParsedType::Array(array) => array,
                _ => unreachable!(),
            };
            let size =
                parsed_array_to_array(c, array.clone(), containers, definers, c.tags()).size();

            let c = if let Some(c) = get_container(containers, ty_name, c.tags()) {
                parsed_container_to_container(c.clone(), containers, definers)
            } else {
                let related = get_related(containers, definers, ty_name);
                complex_not_found(c.name(), c.tags(), &c.file_info, ty_name, &related);
            };

            return TestValue::ArrayOfSubObject {
                c,
                members: v,
                size,
            };
        }
    };

    let tv = match ty {
        ParsedType::SizedCString | ParsedType::CString | ParsedType::String { .. } => {
            TestValue::String(value.replace('\"', ""))
        }
        ParsedType::Bool(_) => TestValue::Bool(if value == "TRUE" {
            true
        } else if value == "FALSE" {
            false
        } else {
            panic!("incorrect boolean value: '{value}'")
        }),
        ParsedType::Array(array) => {
            assert!(value.contains('['));
            assert!(value.contains(']'));
            let val = &value.replace(['[', ']'], "");
            let mut v = Vec::new();
            for value in val.split(',') {
                let value = value.trim();
                if value.is_empty() {
                    continue;
                }

                v.push(parse_value(value).unwrap() as usize);
            }
            let size =
                parsed_array_to_array(c, array.clone(), containers, definers, c.tags()).size();
            TestValue::Array { values: v, size }
        }
        ParsedType::Population => TestValue::Population {
            value: value.parse().unwrap(),
        },
        ParsedType::FloatingPoint => TestValue::FloatingNumber {
            value: value.parse().unwrap(),
            original_string: value.clone(),
        },
        ParsedType::DateTime => TestValue::DateTime(ContainerValue::new(
            parse_value(&value).unwrap(),
            value.clone(),
        )),
        ParsedType::Seconds => TestValue::Seconds(ContainerValue::new(
            parse_value(&value).unwrap(),
            value.clone(),
        )),
        ParsedType::Milliseconds => TestValue::Milliseconds(ContainerValue::new(
            parse_value(&value).unwrap(),
            value.clone(),
        )),
        ParsedType::Level16 | ParsedType::Level32 | ParsedType::Level => TestValue::Level(
            ContainerValue::new(parse_value(&value).unwrap(), value.clone()),
        ),
        ParsedType::Gold => TestValue::Gold(ContainerValue::new(
            parse_value(&value).unwrap(),
            value.clone(),
        )),
        ParsedType::Integer(_) => TestValue::Number(ContainerValue::new(
            parse_value(&value).unwrap(),
            value.clone(),
        )),
        ParsedType::Guid | ParsedType::PackedGuid => TestValue::Guid(ContainerValue::new(
            parse_value(&value).unwrap(),
            value.clone(),
        )),
        ParsedType::IpAddress => TestValue::IpAddress(ContainerValue::new(
            parse_value(&value).unwrap(),
            value.clone(),
        )),
        ParsedType::Identifier { s, .. } => {
            if get_definer(flags, s, c.tags()).is_some() {
                let mut v = Vec::new();
                for flag in value.split('|') {
                    v.push(flag.trim().to_owned());
                }
                TestValue::Flag(v)
            } else if let Some(e) = get_definer(enums, s, c.tags()) {
                let v = e.get_field_with_name(&value).unwrap().value().int();
                TestValue::Enum(ContainerValue::new(v, value))
            } else {
                let related = get_related(containers, definers, s);
                complex_not_found(c.name(), c.tags(), &c.file_info, s, &related);
            }
        }
        ParsedType::AddonArray
        | ParsedType::VariableItemRandomProperty
        | ParsedType::NamedGuid
        | ParsedType::EnchantMask
        | ParsedType::InspectTalentGearMask
        | ParsedType::MonsterMoveSpline
        | ParsedType::AchievementDoneArray
        | ParsedType::AchievementInProgressArray
        | ParsedType::UpdateMask
        | ParsedType::AuraMask => {
            unimplemented!()
        }
    };

    tv
}

fn convert_test_case_member_to_test_case(
    member: ParsedTestCaseMember,
    c: &ParsedContainer,
    containers: &[ParsedContainer],
    enums: &[Definer],
    flags: &[Definer],
    definers: &[Definer],
) -> TestCaseMember {
    let value = convert_parsed_test_case_value_to_test_case_value(
        &member.variable_name,
        member.value,
        c,
        containers,
        enums,
        flags,
        definers,
    );
    TestCaseMember::new(member.variable_name, value, member.tags)
}

fn convert_parsed_test_case_to_test_case(
    test: ParsedTestCase,
    c: &ParsedContainer,
    containers: &[ParsedContainer],
    enums: &[Definer],
    flags: &[Definer],
    definers: &[Definer],
) -> TestCase {
    let mut value = Vec::with_capacity(test.members.len());

    for m in test.members {
        value.push(convert_test_case_member_to_test_case(
            m, c, containers, enums, flags, definers,
        ));
    }

    TestCase::new(
        test.subject,
        value,
        test.raw_bytes,
        test.tags,
        test.file_info,
    )
}

pub(crate) fn parsed_test_case_to_test_case(
    parsed: Vec<ParsedTestCase>,
    containers: &[ParsedContainer],
    enums: &[Definer],
    flags: &[Definer],
) -> Vec<TestCase> {
    let mut v = Vec::with_capacity(parsed.len());

    let definers = [enums, flags].concat();

    for p in parsed {
        let c = if let Some(a) = conversion::get_container(containers, p.subject(), p.tags()) {
            a
        } else {
            panic!("Unable to find test subject '{}' for test.", p.subject());
        };

        v.push(convert_parsed_test_case_to_test_case(
            p, c, containers, enums, flags, &definers,
        ));
    }

    v
}
