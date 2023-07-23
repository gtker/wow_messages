use crate::error_printer::{
    invalid_self_size_position, object_has_no_versions, overlapping_versions,
};
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
use crate::{Container, ObjectTags, Objects, CONTAINER_SELF_SIZE_FIELD};

mod container;

pub(crate) fn object_new(
    enums: Vec<ParsedDefiner>,
    flags: Vec<ParsedDefiner>,
    structs: Vec<ParsedContainer>,
    messages: Vec<ParsedContainer>,
    tests: Vec<ParsedTestCase>,
) -> Objects {
    let containers = [structs.as_slice(), messages.as_slice()].concat();
    let enums = parsed_definer_to_definer(enums, &containers);
    let flags = parsed_definer_to_definer(flags, &containers);

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

    let size_of_fields_before_size =
        size_of_fields_before(&p.name, &p, containers, definers, &p.members, &p.file_info);

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
        size_of_fields_before_size,
        rust_object_view,
    )
}

fn size_of_fields_before(
    name: &str,
    e: &ParsedContainer,
    containers: &[ParsedContainer],
    definers: &[Definer],
    members: &[ParsedStructMember],
    file_info: &FileInfo,
) -> Option<i128> {
    if !members.iter().any(|a| a.is_manual_size_field()) {
        return None;
    }

    let mut sum = 0;
    for field in members {
        match field {
            ParsedStructMember::Definition(d) => {
                if let Some(size) = d.ty().sizes_parsed(e, containers, definers).is_constant() {
                    sum += size;
                } else {
                    invalid_self_size_position(
                        name,
                        file_info,
                        format!(
                            "'{}' can not come after variable '{}' of type '{}'",
                            CONTAINER_SELF_SIZE_FIELD,
                            d.name(),
                            d.ty().str(),
                        ),
                    )
                }

                if d.is_manual_size_field() {
                    return Some(sum);
                }
            }
            ParsedStructMember::IfStatement(_) => invalid_self_size_position(
                name,
                file_info,
                format!("'{CONTAINER_SELF_SIZE_FIELD}' can not come after an if statement"),
            ),
            ParsedStructMember::OptionalStatement(_) => invalid_self_size_position(
                name,
                file_info,
                format!("'{CONTAINER_SELF_SIZE_FIELD}' can not come after an optional statement"),
            ),
        }
    }

    Some(sum)
}

pub(crate) fn parsed_tags_to_tags(
    tags: &ObjectTags,
    ty_name: &str,
    file_info: &FileInfo,
) -> ObjectTags {
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
    containers: &[ParsedContainer],
) -> Vec<Definer> {
    let mut v = Vec::with_capacity(parsed.len());

    for p in parsed {
        let objects_used_in = get_definer_objects_used_in(containers, &p);

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

pub(crate) fn get_related<'a>(
    containers: &'a [ParsedContainer],
    definers: &'a [Definer],
    name: &str,
) -> Vec<(&'a FileInfo, &'a ObjectTags)> {
    let definers = definers.iter().filter_map(|a| {
        if a.name() == name {
            Some((a.file_info(), a.tags()))
        } else {
            None
        }
    });

    let mut v = Vec::new();
    for d in definers {
        v.push(d);
    }

    let containers = containers.iter().filter_map(|a| {
        if a.name() == name {
            Some((&a.file_info, a.tags()))
        } else {
            None
        }
    });

    for c in containers {
        v.push(c);
    }

    v
}

pub(crate) fn get_container<'a>(
    containers: &'a [ParsedContainer],
    name: &str,
    tags: &ObjectTags,
) -> Option<&'a ParsedContainer> {
    containers
        .iter()
        .find(|a| a.name() == name && a.tags().fulfills_all(tags))
}

pub(crate) fn get_definer<'a>(
    definers: &'a [Definer],
    name: &str,
    tags: &ObjectTags,
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
        tags: &'a ObjectTags,
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
                overlapping_versions(
                    inner.name,
                    outer.tags,
                    outer.file_info,
                    inner.tags,
                    inner.file_info,
                );
            }
        }
    }
}
