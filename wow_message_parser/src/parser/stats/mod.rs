pub(crate) mod tbc_messages;
pub(crate) mod vanilla_messages;
pub(crate) mod wrath_messages;

use crate::error_printer::incorrect_opcode_for_message;
use crate::parser::types::container::ContainerType;
use crate::parser::types::objects::Objects;
use crate::parser::types::tags::ObjectTags;
use crate::parser::types::version::WorldVersion;
use crate::parser::types::version::{MajorWorldVersion, Version};
use crate::UNIMPLEMENTED;

#[derive(Debug, Clone)]
pub(crate) struct Data {
    name: &'static str,
    opcode: usize,
    definition: bool,
    tests: usize,
    reason: Option<&'static str>,
}

impl Data {
    pub(crate) const fn new(name: &'static str, opcode: usize) -> Self {
        Self {
            name,
            opcode,
            definition: false,
            tests: 0,
            reason: None,
        }
    }

    pub(crate) const fn with_reason(
        name: &'static str,
        opcode: usize,
        reason: &'static str,
    ) -> Self {
        Self {
            name,
            opcode,
            definition: false,
            tests: 0,
            reason: Some(reason),
        }
    }
}

pub(crate) fn print_message_stats(o: &Objects) {
    stats_for(
        WorldVersion::Minor(1, 12).into(),
        vanilla_messages::DATA.to_vec(),
        o,
    );
    println!();

    stats_for(
        WorldVersion::Patch(2, 4, 3).into(),
        tbc_messages::DATA.to_vec(),
        o,
    );
    println!();

    stats_for(
        WorldVersion::Patch(3, 3, 5).into(),
        wrath_messages::DATA.to_vec(),
        o,
    );
}

fn stats_for(version: Version, mut data: Vec<Data>, o: &Objects) {
    let tags = ObjectTags::new_with_version(version);
    for s in o.messages() {
        if !s.tags().fulfills_all(&tags) {
            continue;
        }

        if let Some(mut container) = data.iter_mut().find(|a| a.name == get_real_name(s.name())) {
            match s.container_type() {
                ContainerType::CLogin(i)
                | ContainerType::SLogin(i)
                | ContainerType::Msg(i)
                | ContainerType::CMsg(i)
                | ContainerType::SMsg(i) => {
                    if i as usize != container.opcode {
                        incorrect_opcode_for_message(
                            container.name,
                            s.file_info(),
                            container.opcode,
                            i,
                        );
                    }
                    assert_eq!(i as usize, container.opcode);
                }
                _ => unreachable!("not a message"),
            }

            container.definition = !s.tags().unimplemented();
            container.tests = s.tests(o).len();
        }
    }

    let mut definition_sum = 0;
    let mut test_sum = 0;
    for d in &data {
        if d.definition {
            definition_sum += 1;
        }
        if d.tests > 0 {
            test_sum += 1;
        }
    }

    let print_missing_as_wowm = version.as_major_world() != MajorWorldVersion::Vanilla;

    println!("{} Messages without definition:", version.as_world());
    for d in &data {
        if !d.definition {
            if print_missing_as_wowm {
                if d.reason.is_none() {
                    let i = d.name.find('_').unwrap();
                    let prefix = &d.name[..i];
                    let ty = match prefix {
                        "SMSG" => "smsg",
                        "CMSG" => "cmsg",
                        "MSG" => "msg",
                        _ => unreachable!("{} in {}", prefix, d.name),
                    };
                    println!(
                        "{ty} {name} = {opcode:#06X} {{ {unimplemented} }} {{ versions = \"{version}\"; }}",
                        name = &d.name,
                        opcode = d.opcode,
                        unimplemented = UNIMPLEMENTED,
                        version = version.as_world(),
                    );
                }
            } else if let Some(reason) = d.reason {
                println!("    {}: {}", d.name, reason);
            } else {
                println!("    {}", d.name);
            }
        }
    }

    println!();

    println!(
        "{} Messages with definition: {} / {} ({}%) ({} left)",
        version.as_world(),
        definition_sum,
        &data.len(),
        (definition_sum as f32 / data.len() as f32) * 100.0_f32,
        data.len() - definition_sum
    );
    println!(
        "{} Total messages with tests: {} / {} ({}%)",
        version.as_world(),
        test_sum,
        &data.len(),
        (test_sum as f32 / data.len() as f32) * 100.0_f32
    );
}

fn get_real_name(s: &str) -> String {
    s.replace("_Client", "").replace("_Server", "")
}
