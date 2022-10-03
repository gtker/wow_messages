use crate::parser::types::definer::Definer;
use crate::parser::types::if_statement::{Equation, IfStatement};
use crate::parser::types::objects::conversion;
use crate::parser::types::objects::conversion::{
    all_definitions, all_definitions_mut, get_definer,
};
use crate::parser::types::optional::OptionalStatement;
use crate::parser::types::parsed::parsed_container::ParsedContainer;
use crate::parser::types::parsed::parsed_if_statement::ParsedIfStatement;
use crate::parser::types::parsed::parsed_struct_member::ParsedStructMember;
use crate::parser::types::parsed::parsed_test_case::{
    ParsedTestCase, ParsedTestCaseMember, ParsedTestValue,
};
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::test_case::{TestCase, TestCaseMember, TestUpdateMaskValue, TestValue};
use crate::parser::types::ty::Type;
use crate::parser::types::{ArraySize, ArrayType, ContainerValue};
use crate::parser::utility::parse_value;
use crate::rust_printer::UpdateMaskType;
use crate::{DefinerType, Tags};

pub(crate) fn get_tests_for_object(
    tests: &mut Vec<TestCase>,
    name: &str,
    tags: &Tags,
) -> Vec<TestCase> {
    let mut v = Vec::new();
    let mut indices = Vec::new();

    for (i, t) in tests.iter().enumerate() {
        if t.subject() == name && t.tags().has_version_intersections(tags) {
            indices.push(i);
            v.push(t.clone());
        }
    }
    indices.reverse();

    for i in indices {
        tests.remove(i);
    }

    v
}

pub(crate) fn verify_and_set_members(
    members: &mut [ParsedStructMember],
    tags: &Tags,
    containers: &[ParsedContainer],
    definers: &[Definer],
) {
    set_used_as_size_in(members);

    set_verified_values(members, definers);

    check_complex_types_exist(members, containers, definers, tags);
}

pub(crate) fn parsed_members_to_members(members: Vec<ParsedStructMember>) -> Vec<StructMember> {
    let mut v = Vec::with_capacity(members.len());

    for m in members {
        v.push(match m {
            ParsedStructMember::Definition(d) => {
                StructMember::Definition(StructMemberDefinition::new(
                    d.name,
                    d.struct_type,
                    d.verified_value,
                    d.used_as_size_in,
                    d.used_in_if.unwrap(),
                    d.tags,
                ))
            }
            ParsedStructMember::IfStatement(s) => StructMember::IfStatement(IfStatement::new(
                s.conditional,
                parsed_members_to_members(s.members),
                parsed_if_statement_to_if_statement(s.else_ifs),
                parsed_members_to_members(s.else_statement_members),
                s.original_ty.unwrap(),
            )),
            ParsedStructMember::OptionalStatement(o) => StructMember::OptionalStatement(
                OptionalStatement::new(o.name, parsed_members_to_members(o.members), o.tags),
            ),
        });
    }

    v
}

fn parsed_if_statement_to_if_statement(parsed: Vec<ParsedIfStatement>) -> Vec<IfStatement> {
    let mut v = Vec::with_capacity(parsed.len());

    for p in parsed {
        v.push(IfStatement::new(
            p.conditional,
            parsed_members_to_members(p.members),
            parsed_if_statement_to_if_statement(p.else_ifs),
            parsed_members_to_members(p.else_statement_members),
            p.original_ty.unwrap(),
        ))
    }

    v
}

fn set_used_as_size_in(members: &mut [ParsedStructMember]) {
    let mut variables_used_as_size_in = Vec::new();

    for d in all_definitions(members) {
        match d.ty() {
            Type::String { length } => {
                if length.parse::<u8>().is_err() {
                    variables_used_as_size_in.push((d.name().to_string(), length.to_string()));
                }
            }
            Type::Array(array) => {
                if let ArraySize::Variable(length) = array.size() {
                    if length.parse::<u8>().is_err() {
                        variables_used_as_size_in.push((d.name().to_string(), length.to_string()));
                    }
                }
            }
            _ => {}
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

fn set_verified_values(members: &mut [ParsedStructMember], definers: &[Definer]) {
    for d in all_definitions_mut(members) {
        d.set_verified_value(definers);
    }
}

fn contains_complex_type(
    containers: &[ParsedContainer],
    definers: &[Definer],
    ty_name: &str,
    tags: &Tags,
    struct_name: &str,
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

    panic!(
        "Complex type not found: '{}' for object: '{}' for versions logon: '{:?}', versions: '{:?}'",
        ty_name,
        struct_name,
        tags.logon_versions(),
        tags.versions()
    );
}

fn check_complex_types_exist(
    members: &[ParsedStructMember],
    containers: &[ParsedContainer],
    definers: &[Definer],
    tags: &Tags,
) {
    for d in all_definitions(members) {
        match &d.ty() {
            Type::Array(a) => {
                if let ArrayType::Complex(c) = &a.ty() {
                    contains_complex_type(containers, definers, c, tags, d.name())
                }
            }
            Type::Identifier { s: i, .. } => {
                contains_complex_type(containers, definers, i, tags, d.name());

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
                    Type::Identifier { s, .. } => s,
                    _ => unreachable!(),
                };

                let definer = get_definer(definers, ty, e.tags()).unwrap();
                match definer.definer_ty() {
                    DefinerType::Enum => {
                        for c in statement.get_conditional().equations() {
                            match c {
                                Equation::Equals { .. } | Equation::NotEquals { .. } => {}
                                Equation::BitwiseAnd { .. } => {
                                    panic!("enum has bitwise and")
                                }
                            }
                        }
                    }
                    DefinerType::Flag => {
                        for c in statement.get_conditional().equations() {
                            match c {
                                Equation::Equals { .. } | Equation::NotEquals { .. } => {
                                    panic!("flag has equals or not equals")
                                }
                                Equation::BitwiseAnd { .. } => {}
                            }
                        }
                    }
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
) -> TestValue {
    let ty = c.get_field_ty(variable_name);

    let value = match test {
        ParsedTestValue::Single(s) => s,
        ParsedTestValue::Multiple(mut multiple) => {
            if ty == &Type::UpdateMask {
                let mut v = Vec::new();
                for m_inner in multiple.iter_mut() {
                    let (ty, name) = &m_inner.variable_name.split_once('_').unwrap();
                    let ty = match *ty {
                        "OBJECT" => UpdateMaskType::Object,
                        "UNIT" => UpdateMaskType::Unit,
                        "ITEM" => UpdateMaskType::Item,
                        "PLAYER" => UpdateMaskType::Player,
                        "CONTAINER" => UpdateMaskType::Container,
                        "GAMEOBJECT" => UpdateMaskType::GameObject,
                        "DYNAMICOBJECT" => UpdateMaskType::DynamicObject,
                        "CORPSE" => UpdateMaskType::Corpse,
                        _ => panic!("invalid update mask type: '{}'", ty),
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
            let inner_c =
                conversion::get_container(containers, ty.rust_str().as_str(), c.tags()).unwrap();
            for m_inner in multiple {
                members.push(convert_test_case_member_to_test_case(
                    m_inner, inner_c, containers, enums, flags,
                ));
            }

            return TestValue::SubObject {
                ty_name: ty.rust_str(),
                members,
            };
        }
        ParsedTestValue::ArrayOfMultiple(array) => {
            let mut v = Vec::new();

            let ty_name = match ty {
                Type::Array(array) => match array.ty() {
                    ArrayType::Integer(_) => panic!(),
                    ArrayType::Complex(c) => c.as_str(),
                    ArrayType::CString => unimplemented!(),
                    ArrayType::Guid => "Guid",
                    ArrayType::PackedGuid => "Guid",
                },
                _ => panic!(),
            };

            for multiple in array {
                let mut members = Vec::new();
                let inner_c = conversion::get_container(containers, ty_name, c.tags()).unwrap();

                for m_inner in multiple {
                    members.push(convert_test_case_member_to_test_case(
                        m_inner, inner_c, containers, enums, flags,
                    ));
                }

                v.push(members);
            }

            return TestValue::ArrayOfSubObject(ty_name.to_string(), v);
        }
    };

    let tv = match ty {
        Type::SizedCString | Type::CString | Type::String { .. } => {
            TestValue::String(value.replace('\"', ""))
        }
        Type::Bool => TestValue::Bool(if value == "TRUE" {
            true
        } else if value == "FALSE" {
            false
        } else {
            panic!("incorrect boolean value: '{}'", value)
        }),
        Type::Array(array) => {
            assert!(value.contains('['));
            assert!(value.contains(']'));
            let val = &value.replace('[', "").replace(']', "");
            let mut v = Vec::new();
            for value in val.split(',') {
                let value = value.trim();
                if value.is_empty() {
                    continue;
                }

                v.push(parse_value(value).unwrap() as usize);
            }
            TestValue::Array {
                values: v,
                size: array.size(),
            }
        }
        Type::FloatingPoint(_) => TestValue::FloatingNumber {
            value: value.parse().unwrap(),
            original_string: value.clone(),
        },
        Type::DateTime => TestValue::DateTime(ContainerValue::new(
            parse_value(&value).unwrap(),
            value.clone(),
        )),
        Type::Integer(_) => TestValue::Number(ContainerValue::new(
            parse_value(&value).unwrap(),
            value.clone(),
        )),
        Type::Guid | Type::PackedGuid => TestValue::Guid(ContainerValue::new(
            parse_value(&value).unwrap(),
            value.clone(),
        )),
        Type::Identifier { .. } => {
            if conversion::get_definer(flags, ty.rust_str().as_str(), c.tags()).is_some() {
                let mut v = Vec::new();
                for flag in value.split('|') {
                    v.push(flag.trim().to_owned());
                }
                TestValue::Flag(v)
            } else if let Some(e) = conversion::get_definer(enums, ty.rust_str().as_str(), c.tags())
            {
                let v = e.get_field_with_name(&value).unwrap().value().int();
                TestValue::Enum(ContainerValue::new(v, value))
            } else {
                unreachable!()
            }
        }
        Type::UpdateMask | Type::AuraMask => {
            panic!("unimplemented")
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
) -> TestCaseMember {
    let value = convert_parsed_test_case_value_to_test_case_value(
        &member.variable_name,
        member.value,
        c,
        containers,
        enums,
        flags,
    );
    TestCaseMember::new(member.variable_name, value, member.tags)
}

fn convert_parsed_test_case_to_test_case(
    test: ParsedTestCase,
    c: &ParsedContainer,
    containers: &[ParsedContainer],
    enums: &[Definer],
    flags: &[Definer],
) -> TestCase {
    let mut value = Vec::with_capacity(test.members.len());

    for m in test.members {
        value.push(convert_test_case_member_to_test_case(
            m, c, containers, enums, flags,
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

    for p in parsed {
        let c = conversion::get_container(containers, p.subject(), p.tags()).unwrap();

        v.push(convert_parsed_test_case_to_test_case(
            p, c, containers, enums, flags,
        ));
    }

    v
}
