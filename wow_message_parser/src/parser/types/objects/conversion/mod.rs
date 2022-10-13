use crate::error_printer::object_has_no_versions;
use crate::file_info::FileInfo;
use crate::parser::types::definer::Definer;
use crate::parser::types::objects::conversion::container::{
    check_if_statement_operators, verify_and_set_members,
};
use crate::parser::types::parsed::parsed_container::ParsedContainer;
use crate::parser::types::parsed::parsed_definer::ParsedDefiner;
use crate::parser::types::parsed::parsed_object::get_definer_objects_used_in;
use crate::parser::types::parsed::parsed_struct_member::{
    ParsedStructMember, ParsedStructMemberDefinition,
};
use crate::parser::types::parsed::parsed_test_case::ParsedTestCase;
use crate::rust_printer::rust_view::create_rust_object;
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

    check_versions(&containers, &definers);

    let structs = parsed_containers_to_container(structs, &containers, &definers);
    let messages = parsed_containers_to_container(messages, &containers, &definers);

    let tests = container::parsed_test_case_to_test_case(tests, &containers, &enums, &flags);

    let mut o = Objects {
        enums,
        flags,
        structs,
        messages,
        tests,
    };

    o.sort_members();

    o
}

pub(crate) fn parsed_container_to_container(
    mut p: ParsedContainer,
    containers: &[ParsedContainer],
    definers: &[Definer],
) -> Container {
    let sizes = p.create_sizes(containers, definers);

    let tags = parsed_tags_to_tags(&p.tags, p.name(), &p.file_info);

    let only_has_io_error = p.recursive_only_has_io_errors(containers, definers);

    check_if_statement_operators(&p, definers);

    verify_and_set_members(&mut p.members, &p.tags, containers, definers, &p.file_info);

    let members =
        container::parsed_members_to_members(&p, p.members.clone(), containers, definers, p.tags());

    let rust_object_view = create_rust_object(&p, &members, containers, definers);

    Container::new(
        p.name,
        members,
        tags,
        p.object_type,
        p.file_info,
        sizes,
        only_has_io_error,
        rust_object_view,
    )
}

pub(crate) fn parsed_tags_to_tags(tags: &Tags, ty_name: &str, file_info: &FileInfo) -> Tags {
    if !tags.has_login_version() && !tags.has_world_version() {
        object_has_no_versions(ty_name, file_info)
    }

    tags.clone()
}

pub(crate) fn parsed_containers_to_container(
    parsed: Vec<ParsedContainer>,
    containers: &[ParsedContainer],
    definers: &[Definer],
) -> Vec<Container> {
    let mut v = Vec::with_capacity(parsed.len());

    for p in parsed {
        v.push(parsed_container_to_container(p, containers, definers));
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
    members: &mut [ParsedStructMember],
) -> Vec<&mut ParsedStructMemberDefinition> {
    let mut v = Vec::new();

    fn inner<'a>(m: &'a mut ParsedStructMember, v: &mut Vec<&'a mut ParsedStructMemberDefinition>) {
        match m {
            ParsedStructMember::Definition(d) => v.push(d),
            ParsedStructMember::IfStatement(statement) => {
                for m in statement.all_members_mut() {
                    inner(m, v);
                }
            }
            ParsedStructMember::OptionalStatement(optional) => {
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

pub(crate) fn all_definitions(
    members: &[ParsedStructMember],
) -> Vec<&ParsedStructMemberDefinition> {
    let mut v = Vec::new();

    fn inner<'a>(m: &'a ParsedStructMember, v: &mut Vec<&'a ParsedStructMemberDefinition>) {
        match m {
            ParsedStructMember::Definition(d) => v.push(d),
            ParsedStructMember::IfStatement(statement) => {
                for m in statement.all_members() {
                    inner(m, v);
                }
            }
            ParsedStructMember::OptionalStatement(optional) => {
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

fn check_versions(containers: &[ParsedContainer], definers: &[Definer]) {
    struct Obj<'a> {
        name: &'a str,
        tags: &'a Tags,
        file_info: &'a FileInfo,
    }

    let mut v: Vec<Obj> = Vec::new();
    for e in containers {
        v.push(Obj {
            name: e.name(),
            tags: e.tags(),
            file_info: &e.file_info,
        });
    }
    for e in definers {
        v.push(Obj {
            name: e.name(),
            tags: e.tags(),
            file_info: e.file_info(),
        });
    }

    for outer in &v {
        for inner in &v {
            if outer.name == inner.name
                && outer.tags.has_version_intersections(inner.tags)
                && outer.name as *const _ != inner.name as *const _
            {
                panic!(
                    "Objects with same name and overlapping versions: {}
version 1: {:#?} in {} line {},
version 2: {:#?} in {} line {}",
                    inner.name,
                    inner.tags,
                    inner.file_info.name(),
                    inner.file_info.start_line(),
                    outer.tags,
                    outer.file_info.name(),
                    outer.file_info.start_line(),
                );
            }
        }
    }
}
