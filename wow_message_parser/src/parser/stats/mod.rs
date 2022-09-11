pub(crate) mod tbc_messages;
pub(crate) mod vanilla_messages;
pub(crate) mod wrath_messages;

use crate::container::ContainerType;
use crate::parser::types::objects::Objects;
use crate::parser::types::tags::{Tags, WorldVersion};
use crate::Version;

pub struct Data {
    name: &'static str,
    opcode: usize,
    definition: bool,
    tests: usize,
    reason: &'static str,
}

impl Data {
    pub const fn new(name: &'static str, opcode: usize) -> Self {
        Self {
            name,
            opcode,
            definition: false,
            tests: 0,
            reason: "",
        }
    }

    pub const fn with_reason(name: &'static str, opcode: usize, reason: &'static str) -> Self {
        Self {
            name,
            opcode,
            definition: false,
            tests: 0,
            reason,
        }
    }
}

pub fn stats_for_1_12(o: &Objects) {
    let mut data = vanilla_messages::DATA;

    let tags = Tags::new_with_version(Version::World(WorldVersion::Minor(1, 12)));
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

    println!("Messages without definition:");
    for d in &data {
        if !d.definition {
            println!("\t{}: {}", d.name, d.reason);
        }
    }

    println!();

    println!(
        "Messages with definition: {} / {} ({}%) ({} left)",
        definition_sum,
        &data.len(),
        (definition_sum as f32 / data.len() as f32) * 100.0_f32,
        data.len() - definition_sum
    );
    println!(
        "Total messages with tests: {} / {} ({}%)",
        test_sum,
        &data.len(),
        (test_sum as f32 / data.len() as f32) * 100.0_f32
    );
}

fn get_real_name(s: &str) -> String {
    s.replace("_Client", "").replace("_Server", "")
}
