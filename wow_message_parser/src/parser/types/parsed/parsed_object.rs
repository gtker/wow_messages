use crate::parser::types::if_statement::DefinerUsage;
use crate::parser::types::parsed::parsed_container::ParsedContainer;
use crate::parser::types::parsed::parsed_definer::ParsedDefiner;
use crate::parser::types::parsed::parsed_test_case::ParsedTestCase;
use crate::Objects;

#[derive(Debug, Clone)]
pub struct ParsedObjects {
    enums: Vec<ParsedDefiner>,
    flags: Vec<ParsedDefiner>,
    structs: Vec<ParsedContainer>,
    messages: Vec<ParsedContainer>,
    tests: Vec<ParsedTestCase>,
}

impl ParsedObjects {
    pub(crate) fn empty() -> Self {
        Self {
            enums: vec![],
            flags: vec![],
            structs: vec![],
            messages: vec![],
            tests: vec![],
        }
    }

    pub(crate) fn new(
        enums: Vec<ParsedDefiner>,
        flags: Vec<ParsedDefiner>,
        structs: Vec<ParsedContainer>,
        messages: Vec<ParsedContainer>,
        tests: Vec<ParsedTestCase>,
    ) -> Self {
        Self {
            enums,
            flags,
            structs,
            messages,
            tests,
        }
    }

    pub(crate) fn add_vecs(&mut self, mut c: Self) {
        self.enums.append(&mut c.enums);
        self.flags.append(&mut c.flags);
        self.structs.append(&mut c.structs);
        self.messages.append(&mut c.messages);
        self.tests.append(&mut c.tests);
    }

    pub(crate) fn to_objects(self) -> Objects {
        Objects::new(
            self.enums,
            self.flags,
            self.structs,
            self.messages,
            self.tests,
        )
    }
}

pub(crate) fn get_definer_objects_used_in(
    messages: &[ParsedContainer],
    structs: &[ParsedContainer],
    e: &ParsedDefiner,
) -> Vec<(String, DefinerUsage)> {
    let mut v = Vec::new();

    for c in [messages, structs].concat() {
        if !e.tags().has_version_intersections(c.tags()) {
            continue;
        }

        let ty = match c.contains_definer(e.name()) {
            DefinerUsage::Unused => continue,
            DefinerUsage::NotInIf => DefinerUsage::NotInIf,
            DefinerUsage::InIf => DefinerUsage::InIf,
        };

        v.push((c.name().to_string(), ty));
    }

    v.sort_by(|a, b| a.0.cmp(&b.0));

    v
}
