pub(crate) mod tbc_messages;
pub(crate) mod vanilla_messages;
pub(crate) mod wrath_messages;

use crate::error_printer::{
    incorrect_opcode_for_message, message_not_in_index, opcode_has_incorrect_name,
};
use crate::parser::types::objects::Objects;
use crate::parser::types::tags::ObjectTags;
use crate::parser::types::version::MajorWorldVersion;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum MessageType {
    Cmsg,
    Smsg,
    Msg,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum Reason {
    None,
    NotImplementedInAnyEmulator,
}

impl Reason {
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub const fn is_some(&self) -> bool {
        !self.is_none()
    }
}

impl MessageType {
    pub const fn wowm_str(&self) -> &'static str {
        match self {
            MessageType::Cmsg => "cmsg",
            MessageType::Smsg => "smsg",
            MessageType::Msg => "msg",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Data {
    pub name: &'static str,
    pub opcode: usize,
    pub definition: bool,
    pub tests: usize,
    pub reason: Reason,
}

impl Data {
    pub(crate) const fn new(name: &'static str, opcode: usize) -> Self {
        Self {
            name,
            opcode,
            definition: false,
            tests: 0,
            reason: Reason::None,
        }
    }

    const fn direct(name: &'static str, opcode: usize, reason: Reason) -> Self {
        Self {
            name,
            opcode,
            definition: false,
            tests: 0,
            reason,
        }
    }

    const fn reason(&self) -> Option<&'static str> {
        Some(match self.reason {
            Reason::None => return None,
            Reason::NotImplementedInAnyEmulator => "Not implemented in any emulator yet",
        })
    }

    pub(crate) const fn nyi(name: &'static str, opcode: usize) -> Self {
        Self::direct(name, opcode, Reason::NotImplementedInAnyEmulator)
    }

    pub(crate) fn needs_work(&self) -> bool {
        !self.definition
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

    if std::env::var("WOWM_ONLY_PRINT_NAME_OF_SINGLE_MESSAGE").is_ok() {
        let message = if let Some(m) = vanilla.0.iter().find(|a| a.needs_work()) {
            (m, MajorWorldVersion::Vanilla)
        } else if let Some(m) = tbc.0.iter().find(|a| a.needs_work()) {
            (m, MajorWorldVersion::BurningCrusade)
        } else if let Some(m) = wrath.0.iter().find(|a| a.needs_work()) {
            (m, MajorWorldVersion::Wrath)
        } else {
            panic!("No messages that need work found")
        };
        print!(
            "{} {:#06X} {}",
            message.0.name,
            message.0.opcode,
            message.1.as_version_string()
        );
    } else {
        for (data, version) in [&vanilla, &tbc, &wrath] {
            print_missing_definitions(data, *version);
        }

        for (data, version) in [&vanilla, &tbc, &wrath] {
            stats_for(*version, data);
        }
    }
}

fn get_data_for(version: MajorWorldVersion, data: &[Data], o: &Objects) -> Vec<Data> {
    let tags = ObjectTags::new_with_version(version.into());

    let mut data = data.to_vec();
    for s in o.messages() {
        if !s.tags().fulfills_all(&tags) {
            continue;
        }

        if let Some(container) = data.iter_mut().find(|a| a.name == get_real_name(s.name())) {
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

fn print_missing_definitions(data: &[Data], version: MajorWorldVersion) {
    println!(
        "{} Messages without definition:",
        version.as_version_string(),
    );

    for d in data {
        if d.needs_work() {
            println!("    {}: {}", d.name, d.reason().unwrap());
        }
    }

    println!();
}

fn stats_for(version: MajorWorldVersion, data: &[Data]) {
    let mut definition_sum = 0;
    let mut reason_sum = 0;
    let mut test_sum = 0;
    for d in data {
        if d.definition {
            definition_sum += 1;
        } else if d.reason.is_some() {
            reason_sum += 1;
        }
        if d.tests > 0 {
            test_sum += 1;
        }
    }

    println!(
        "{} Messages with definition: {} / {} ({}%) ({} left, {} without implementation excluded with reason)",
        version.as_version_string(),
        definition_sum,
        data.len(),
        (definition_sum as f32 / data.len() as f32) * 100.0_f32,
        data.len() - (definition_sum + reason_sum),
        reason_sum,
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
