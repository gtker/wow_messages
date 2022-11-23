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

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum MessageType {
    Cmsg,
    Smsg,
    Msg,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum Reason {
    None,
    Compressed,
    RequiresNestedIf,
    ElseIfForDifferentVariable,
    NotImplementedInAnyEmulator,
    SelfSizeStruct,
    RequiresAuraMask,
    RequiresNotEqualGuid,
    ComplexImplementation,
    Custom(&'static str),
}

impl Reason {
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub const fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub const fn str(&self) -> Option<&'static str> {
        match self {
            Reason::Custom(s) => Some(s),
            _ => None,
        }
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

    #[allow(unused)]
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
            reason: Reason::Custom(reason),
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
            Reason::Custom(reason) => reason,
            Reason::None => return None,
            Reason::Compressed => "Requires compression",
            Reason::NotImplementedInAnyEmulator => "Not implemented in any emulator yet",
            Reason::RequiresNestedIf => "Requires nested if",
            Reason::ElseIfForDifferentVariable => "Requires else if for different variable",
            Reason::SelfSizeStruct => "Requires self.size for struct",
            Reason::RequiresAuraMask => "Requires AuraMask",
            Reason::RequiresNotEqualGuid => "Requires != 0 for Guid type",
            Reason::ComplexImplementation => "Complex implementation",
        })
    }

    pub(crate) const fn compressed(name: &'static str, opcode: usize) -> Self {
        Self::direct(name, opcode, Reason::Compressed)
    }

    pub(crate) const fn nested_if(name: &'static str, opcode: usize) -> Self {
        Self::direct(name, opcode, Reason::RequiresNestedIf)
    }

    pub(crate) const fn nyi(name: &'static str, opcode: usize) -> Self {
        Self::direct(name, opcode, Reason::NotImplementedInAnyEmulator)
    }

    pub(crate) const fn aura_mask(name: &'static str, opcode: usize) -> Self {
        Self::direct(name, opcode, Reason::RequiresAuraMask)
    }

    pub(crate) const fn complex(name: &'static str, opcode: usize) -> Self {
        Self::direct(name, opcode, Reason::ComplexImplementation)
    }

    pub(crate) const fn else_if_different_variable(name: &'static str, opcode: usize) -> Self {
        Self::direct(name, opcode, Reason::ElseIfForDifferentVariable)
    }

    pub(crate) const fn self_size_struct(name: &'static str, opcode: usize) -> Self {
        Self::direct(name, opcode, Reason::SelfSizeStruct)
    }

    pub(crate) fn message_ty(&self) -> MessageType {
        let i = self.name.find('_').unwrap();
        let prefix = &self.name[..i];

        match prefix {
            "SMSG" => MessageType::Smsg,
            "CMSG" => MessageType::Cmsg,
            "MSG" => MessageType::Msg,
            _ => unreachable!("{} in {}", prefix, self.name),
        }
    }
}

fn get_messages_to_print(wrath: &[Data], tbc: &[Data]) -> (Vec<Data>, &'static str) {
    type ComparisonFunction = dyn Fn(&Data, &Data) -> bool;
    struct Option {
        f: Box<ComparisonFunction>,
        s: &'static str,
    }

    fn cmsg_that_are_also_in_tbc(a: &Data, v: &Data) -> bool {
        a.name == v.name && a.opcode == v.opcode && v.name.starts_with("CMSG")
    }
    let cmsg_that_are_also_in_tbc = Option {
        f: Box::new(cmsg_that_are_also_in_tbc),
        s: "CMSG that are also in TBC",
    };

    fn msg_that_are_also_in_tbc(a: &Data, v: &Data) -> bool {
        a.name == v.name && a.opcode == v.opcode && v.name.starts_with("MSG")
    }
    let msg_that_are_also_in_tbc = Option {
        f: Box::new(msg_that_are_also_in_tbc),
        s: "MSG that are also in TBC",
    };

    fn messages_that_are_also_in_tbc(a: &Data, v: &Data) -> bool {
        a.name == v.name && a.opcode == v.opcode
    }
    let messages_that_are_also_in_tbc = Option {
        f: Box::new(messages_that_are_also_in_tbc),
        s: "Messages that are also in TBC",
    };

    fn cmsg_for_wrath(a: &Data, _v: &Data) -> bool {
        a.message_ty() == MessageType::Cmsg
    }
    let cmsg_for_wrath = Option {
        f: Box::new(cmsg_for_wrath),
        s: "Client messages",
    };

    fn msg_for_wrath(a: &Data, _v: &Data) -> bool {
        a.message_ty() == MessageType::Msg
    }
    let msg_for_wrath = Option {
        f: Box::new(msg_for_wrath),
        s: "Client messages",
    };

    for condition in [
        cmsg_that_are_also_in_tbc,
        msg_that_are_also_in_tbc,
        messages_that_are_also_in_tbc,
        cmsg_for_wrath,
        msg_for_wrath,
    ] {
        let data = wrath
            .iter()
            .filter(|a| tbc.iter().any(|v| (condition.f)(a, v)))
            .cloned()
            .collect::<Vec<_>>();

        if data.iter().any(|a| !a.definition && a.reason.is_none()) {
            return (data, condition.s);
        }
    }

    (
        if !wrath.is_empty() {
            wrath.to_vec()
        } else {
            tbc.to_vec()
        },
        "Messages",
    )
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

    let (data, description) = get_messages_to_print(&wrath.0, &tbc.0);

    if std::env::var("WOWM_ONLY_PRINT_NAME_OF_SINGLE_MESSAGE").is_ok() {
        print!(
            "{}",
            data.iter()
                .find(|a| !a.definition && a.reason.is_none())
                .unwrap()
                .name
        );
    } else {
        print_missing_definitions(&data, wrath.1, description);

        stats_for(wrath.1, &data, description);

        let messages_description = "Messages";

        for (data, version) in [&vanilla, &tbc, &wrath] {
            stats_for(*version, data, messages_description);
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
                    let ty = d.message_ty().wowm_str();
                    println!(
                        "{ty} {name} = {opcode:#06X} {{ {unimplemented} }} {{ versions = \"{version}\"; }}",
                        name = &d.name,
                        opcode = d.opcode,
                        unimplemented = UNIMPLEMENTED,
                        version = version.as_version_string(),
                    );
                }
            } else {
                println!("    {}: {}", d.name, d.reason().unwrap());
            }
        }
    }

    println!();
}

fn stats_for(version: MajorWorldVersion, data: &[Data], description: &str) {
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
        "{} {} with definition: {} / {} ({}%) ({} left, {} without implementation excluded with reason)",
        version.as_version_string(),
        description,
        definition_sum,
        data.len(),
        (definition_sum as f32 / data.len() as f32) * 100.0_f32,
        data.len() - definition_sum,
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
