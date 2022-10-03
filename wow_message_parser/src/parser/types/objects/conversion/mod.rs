use crate::parser::types::definer::Definer;
use crate::parser::types::objects::conversion::container::check_if_statement_operators;
use crate::parser::types::parsed::parsed_container::ParsedContainer;
use crate::parser::types::parsed::parsed_definer::ParsedDefiner;
use crate::parser::types::parsed::parsed_object::get_definer_objects_used_in;
use crate::parser::types::parsed::parsed_test_case::ParsedTestCase;
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::test_case::TestCase;
use crate::{Container, Objects, Tags};

mod container;

pub(crate) fn object_new(
    enums: Vec<ParsedDefiner>,
    flags: Vec<ParsedDefiner>,
    structs: Vec<ParsedContainer>,
    messages: Vec<ParsedContainer>,
    tests: Vec<ParsedTestCase>,
) -> Objects {
    let enums = parsed_definer_to_definer(enums, &structs, &messages);
    let flags = parsed_definer_to_definer(flags, &structs, &messages);

    let containers = [structs.as_slice(), messages.as_slice()].concat();
    let definers = [enums.as_slice(), flags.as_slice()].concat();

    let mut tests = container::parsed_test_case_to_test_case(tests, &containers, &enums, &flags);

    let structs = parsed_container_to_container(structs, &mut tests, &containers, &definers);
    let messages = parsed_container_to_container(messages, &mut tests, &containers, &definers);

    let mut o = Objects {
        enums,
        flags,
        structs,
        messages,
    };

    o.check_values();
    o.sort_members();

    o
}
pub(crate) fn parsed_container_to_container(
    parsed: Vec<ParsedContainer>,
    tests: &mut Vec<TestCase>,
    containers: &[ParsedContainer],
    definers: &[Definer],
) -> Vec<Container> {
    let mut v = Vec::with_capacity(parsed.len());

    for p in parsed {
        let tests = Objects::get_tests_for_object(tests, p.name(), p.tags());

        let sizes = p.create_sizes(containers, definers);

        let only_has_io_error = p.recursive_only_has_io_errors(containers, definers);

        check_if_statement_operators(&p, definers);

        let members =
            container::parsed_members_to_members(p.members, &p.tags, containers, definers);

        v.push(Container::new(
            p.name,
            members,
            p.tags,
            p.object_type,
            p.file_info,
            tests,
            sizes,
            only_has_io_error,
        ));
    }

    v
}

pub(crate) fn parsed_definer_to_definer(
    parsed: Vec<ParsedDefiner>,
    structs: &[ParsedContainer],
    messages: &[ParsedContainer],
) -> Vec<Definer> {
    let mut v = Vec::with_capacity(parsed.len());

    for p in parsed {
        let objects_used_in = get_definer_objects_used_in(messages, structs, &p);

        v.push(Definer::new(
            p.name,
            p.definer_ty,
            p.fields,
            p.basic_type,
            p.self_value,
            p.tags,
            objects_used_in,
            p.file_info,
        ));
    }

    v
}

pub(crate) fn get_container<'a>(
    containers: &'a [ParsedContainer],
    name: &str,
    tags: &Tags,
) -> Option<&'a ParsedContainer> {
    containers
        .iter()
        .find(|a| a.name() == name && a.tags().fulfills_all(tags))
}

pub(crate) fn get_definer<'a>(
    definers: &'a [Definer],
    name: &str,
    tags: &Tags,
) -> Option<&'a Definer> {
    definers
        .iter()
        .find(|a| a.name() == name && a.tags().fulfills_all(tags))
}

pub(crate) fn all_definitions_mut(
    members: &mut [StructMember],
) -> Vec<&mut StructMemberDefinition> {
    let mut v = Vec::new();

    fn inner<'a>(m: &'a mut StructMember, v: &mut Vec<&'a mut StructMemberDefinition>) {
        match m {
            StructMember::Definition(d) => v.push(d),
            StructMember::IfStatement(statement) => {
                for m in statement.all_members_mut() {
                    inner(m, v);
                }
            }
            StructMember::OptionalStatement(optional) => {
                for m in optional.members_mut() {
                    inner(m, v);
                }
            }
        }
    }

    for m in members {
        inner(m, &mut v);
    }

    v
}

pub(crate) fn all_definitions(members: &[StructMember]) -> Vec<&StructMemberDefinition> {
    let mut v = Vec::new();

    fn inner<'a>(m: &'a StructMember, v: &mut Vec<&'a StructMemberDefinition>) {
        match m {
            StructMember::Definition(d) => v.push(d),
            StructMember::IfStatement(statement) => {
                for m in statement.all_members() {
                    inner(m, v);
                }
            }
            StructMember::OptionalStatement(optional) => {
                for m in optional.members() {
                    inner(m, v);
                }
            }
        }
    }

    for m in members {
        inner(m, &mut v);
    }

    v
}
