pub(crate) mod tbc_messages;
pub(crate) mod vanilla_messages;
pub(crate) mod wrath_messages;

use crate::parser::types::container::ContainerType;
use crate::parser::types::objects::Objects;
use crate::parser::types::tags::{Tags, WorldVersion};
use crate::Version;

#[derive(Debug, Clone)]
pub(crate) struct Data {
    name: &'static str,
    opcode: usize,
    definition: bool,
    tests: usize,
    reason: &'static str,
}

impl Data {
    pub(crate) const fn new(name: &'static str, opcode: usize) -> Self {
        Self {
            name,
            opcode,
            definition: false,
            tests: 0,
            reason: "",
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
            reason,
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
    let tags = Tags::new_with_version(version);
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
                    assert_eq!(i as usize, container.opcode);
                }
                _ => panic!("invalid for counting"),
            }

            container.definition = !s.tags().unimplemented();
            container.tests = s.tests().len();
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

    let amount_of_missing_definitions =
        data.iter()
            .fold(0, |acc, o| if o.definition { acc } else { acc + 1 });

    if amount_of_missing_definitions < 10 {
        println!("{} Messages without definition:", version.as_world());
        for d in &data {
            if !d.definition {
                println!("\t{}: {}", d.name, d.reason);
            }
        }

        println!();
    }

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
