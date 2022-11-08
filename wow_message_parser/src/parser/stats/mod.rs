pub(crate) mod tbc_messages;
pub(crate) mod vanilla_messages;
pub(crate) mod wrath_messages;

use crate::error_printer::{
    incorrect_opcode_for_message, message_not_in_index, opcode_has_incorrect_name,
};
use crate::parser::types::objects::Objects;
use crate::parser::types::tags::ObjectTags;
use crate::parser::types::version::MajorWorldVersion;
use crate::UNIMPLEMENTED;

#[derive(Debug, Clone)]
pub(crate) struct Data {
    pub name: &'static str,
    pub opcode: usize,
    pub definition: bool,
    pub tests: usize,
    pub reason: Option<&'static str>,
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
    let vanilla = {
        let version = MajorWorldVersion::Vanilla;
        (get_data_for(version, vanilla_messages::DATA, o), version)
    };
    let tbc = {
        let version = MajorWorldVersion::BurningCrusade;
        (get_data_for(version, tbc_messages::DATA, o), version)
    };
    let wrath = {
        let version = MajorWorldVersion::Wrath;
        (get_data_for(version, wrath_messages::DATA, o), version)
    };

    let (data, description) = {
        let description = "Messsages that are also in Vanilla";
        let data = wrath
            .0
            .iter()
            .filter(|a| {
                vanilla
                    .0
                    .iter()
                    .find(|v| a.name == v.name && a.opcode == v.opcode)
                    .is_some()
            })
            .cloned()
            .collect::<Vec<_>>();
        (data, description)
    };

    print_missing_definitions(&data, wrath.1, description);

    stats_for(wrath.1, &data, description);

    let messages_description = "Messages";

    for (data, version) in [&vanilla, &tbc, &wrath] {
        stats_for(*version, &data, messages_description);
    }
}

fn get_data_for(version: MajorWorldVersion, data: &[Data], o: &Objects) -> Vec<Data> {
    let tags = ObjectTags::new_with_version(version.into());

    let mut data = data.to_vec();
    for s in o.messages() {
        if !s.tags().fulfills_all(&tags) {
            continue;
        }

        if let Some(mut container) = data.iter_mut().find(|a| a.name == get_real_name(s.name())) {
            let opcode = s.opcode();
            if opcode as usize != container.opcode {
                incorrect_opcode_for_message(
                    container.name,
                    s.file_info(),
                    container.opcode,
                    opcode,
                );
            }
            assert_eq!(opcode as usize, container.opcode);

            container.definition = !s.tags().unimplemented();
            container.tests = s.tests(o).len();
        } else if let Some(message) = data.iter_mut().find(|a| a.opcode == s.opcode() as usize) {
            opcode_has_incorrect_name(
                &get_real_name(s.name()),
                message.name,
                s.file_info(),
                s.opcode() as usize,
                version,
            );
        } else {
            let opcode = s.opcode();
            message_not_in_index(
                &get_real_name(s.name()),
                s.file_info(),
                opcode as usize,
                version,
            );
        }
    }

    data
}

fn print_missing_definitions(data: &[Data], version: MajorWorldVersion, description: &str) {
    println!(
        "{} {} without definition:",
        version.as_version_string(),
        description,
    );

    let print_missing_as_wowm = version != MajorWorldVersion::Vanilla;

    for d in data {
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
                        version = version.as_version_string(),
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
}

fn stats_for(version: MajorWorldVersion, data: &[Data], description: &str) {
    let mut definition_sum = 0;
    let mut test_sum = 0;
    for d in data {
        if d.definition {
            definition_sum += 1;
        }
        if d.tests > 0 {
            test_sum += 1;
        }
    }

    println!(
        "{} {} with definition: {} / {} ({}%) ({} left)",
        version.as_version_string(),
        description,
        definition_sum,
        data.len(),
        (definition_sum as f32 / data.len() as f32) * 100.0_f32,
        data.len() - definition_sum
    );
    println!(
        "    with tests: {} / {} ({}%)",
        test_sum,
        data.len(),
        (test_sum as f32 / data.len() as f32) * 100.0_f32
    );
}

fn get_real_name(s: &str) -> String {
    s.replace("_Client", "").replace("_Server", "")
}
