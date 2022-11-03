use crate::file_utils::overwrite_if_not_same_contents;
use crate::parser::stats::{tbc_messages, vanilla_messages, wrath_messages, Data};
use crate::parser::types::version::MajorWorldVersion;
use crate::path_utils::opcode_to_name_location;
use crate::rust_printer::Writer;

fn print_specific_opcode_to_name(messages: &[Data]) -> Writer {
    let mut s = Writer::new("");

    s.funcn_pub_const("opcode_to_name(opcode: u32)", "Option<&'static str>", |s| {
        s.body_closing_with(
            "Some(match opcode",
            |s| {
                for message in messages {
                    s.wln(format!("{:#06x} => \"{}\",", message.opcode, message.name));
                }

                s.wln("_ => return None,")
            },
            ")",
        );
    });

    s
}

pub fn print_opcode_to_name() {
    let s = print_specific_opcode_to_name(&vanilla_messages::DATA);
    overwrite_if_not_same_contents(
        s.inner(),
        &opcode_to_name_location(MajorWorldVersion::Vanilla),
    );

    let s = print_specific_opcode_to_name(&tbc_messages::DATA);
    overwrite_if_not_same_contents(
        s.inner(),
        &opcode_to_name_location(MajorWorldVersion::BurningCrusade),
    );

    let s = print_specific_opcode_to_name(&wrath_messages::DATA);
    overwrite_if_not_same_contents(
        s.inner(),
        &opcode_to_name_location(MajorWorldVersion::Wrath),
    );
}
