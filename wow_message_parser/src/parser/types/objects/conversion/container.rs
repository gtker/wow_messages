use crate::parser::types::definer::Definer;
use crate::parser::types::objects::conversion;
use crate::parser::types::parsed::parsed_container::ParsedContainer;
use crate::parser::types::parsed::parsed_test_case::{
    ParsedTestCase, ParsedTestCaseMember, TestCaseValueInitial,
};
use crate::parser::types::struct_member::StructMember;
use crate::parser::types::test_case::{TestCase, TestCaseMember, TestUpdateMaskValue, TestValue};
use crate::parser::types::ty::Type;
use crate::parser::types::{ArraySize, ArrayType, VerifiedContainerValue};
use crate::parser::utility::parse_value;
use crate::rust_printer::UpdateMaskType;

pub fn parsed_members_to_members(
    mut members: Vec<StructMember>,
    definers: &[Definer],
) -> Vec<StructMember> {
    set_used_as_size_in(&mut members);

    set_verified_values(&mut members, definers);

    members
}

fn set_used_as_size_in(members: &mut Vec<StructMember>) {
    fn used_as_size(m: &StructMember, variables_used_as_size_in: &mut Vec<(String, String)>) {
        match m {
            StructMember::Definition(d) => match d.ty() {
                Type::String { length } => {
                    if length.parse::<u8>().is_err() {
                        variables_used_as_size_in.push((d.name().to_string(), length.to_string()));
                    }
                }
                Type::Array(array) => match array.size() {
                    ArraySize::Variable(length) => {
                        if length.parse::<u8>().is_err() {
                            variables_used_as_size_in
                                .push((d.name().to_string(), length.to_string()));
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            StructMember::IfStatement(statement) => {
                for m in statement.all_members() {
                    used_as_size(m, variables_used_as_size_in);
                }
            }
            StructMember::OptionalStatement(optional) => {
                for m in optional.members() {
                    used_as_size(m, variables_used_as_size_in);
                }
            }
        }
    }

    let mut variables_used_as_size_in = Vec::new();

    for m in members.iter_mut() {
        used_as_size(m, &mut variables_used_as_size_in);
    }

    fn set_used_as_size(m: &mut StructMember, variables_used_as_size_in: &[(String, String)]) {
        fn contains<'a>(v: &'a [(String, String)], name: &str) -> Option<&'a (String, String)> {
            v.iter().find(|a| a.1 == name)
        }

        match m {
            StructMember::Definition(d) => {
                if let Some((var, _)) = contains(variables_used_as_size_in, d.name()) {
                    d.set_used_as_size_in(var.clone());
                }
            }
            StructMember::IfStatement(statement) => {
                for m in statement.all_members_mut() {
                    set_used_as_size(m, variables_used_as_size_in);
                }
            }
            StructMember::OptionalStatement(optional) => {
                for m in optional.members_mut() {
                    set_used_as_size(m, variables_used_as_size_in);
                }
            }
        }
    }

    for m in members {
        set_used_as_size(m, &variables_used_as_size_in);
    }
}

fn set_verified_values(members: &mut Vec<StructMember>, definers: &[Definer]) {
    fn set_verified_values(m: &mut StructMember, definers: &[Definer]) {
        match m {
            StructMember::Definition(d) => d.set_verified_value(definers),
            StructMember::IfStatement(statement) => {
                for m in statement.all_members_mut() {
                    set_verified_values(m, definers);
                }
            }
            StructMember::OptionalStatement(optional) => {
                for m in optional.members_mut() {
                    set_verified_values(m, definers);
                }
            }
        }
    }

    for m in members {
        set_verified_values(m, definers);
    }
}

fn convert_parsed_test_case_value_to_test_case_value(
    variable_name: &str,
    test: TestCaseValueInitial,
    c: &ParsedContainer,
    containers: &[ParsedContainer],
    enums: &[Definer],
    flags: &[Definer],
) -> TestValue {
    let ty = c.get_field_ty(variable_name);

    let value = match test {
        TestCaseValueInitial::Single(s) => s.clone(),
        TestCaseValueInitial::Multiple(mut multiple) => {
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
                        TestCaseValueInitial::Single(v) => v.clone(),
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
        TestCaseValueInitial::ArrayOfMultiple(array) => {
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
        Type::DateTime => TestValue::DateTime(VerifiedContainerValue::new(
            parse_value(&value).unwrap(),
            value.clone(),
        )),
        Type::Integer(_) => TestValue::Number(VerifiedContainerValue::new(
            parse_value(&value).unwrap(),
            value.clone(),
        )),
        Type::Guid | Type::PackedGuid => TestValue::Guid(VerifiedContainerValue::new(
            parse_value(&value).unwrap(),
            value.clone(),
        )),
        Type::Identifier { .. } => {
            if let Some(_) = conversion::get_definer(flags, ty.rust_str().as_str(), c.tags()) {
                let mut v = Vec::new();
                for flag in value.split('|') {
                    v.push(flag.trim().to_owned());
                }
                TestValue::Flag(v)
            } else if let Some(e) = conversion::get_definer(enums, ty.rust_str().as_str(), c.tags())
            {
                let v = e.get_field_with_name(&value).unwrap().value().int();
                TestValue::Enum(VerifiedContainerValue::new(v, value))
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

pub fn parsed_test_case_to_test_case(
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
