use crate::parser::types::definer::Definer;
use crate::parser::types::parsed_container::ParsedContainer;
use crate::parser::types::parsed_definer::ParsedDefiner;
use crate::parser::types::parsed_object::get_definer_objects_used_in;
use crate::Container;

pub fn parsed_container_to_container(parsed: Vec<ParsedContainer>) -> Vec<Container> {
    let mut v = Vec::with_capacity(parsed.len());

    for p in parsed {
        v.push(Container::new(
            p.name,
            p.members,
            p.tags,
            p.object_type,
            p.file_info,
        ));
    }

    v
}

pub fn parsed_definer_to_definer(
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
