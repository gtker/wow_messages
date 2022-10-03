use crate::parser::types::container::Container;
use crate::parser::types::parsed_definer::ParsedDefiner;
use crate::parser::types::test_case::TestCase;
use crate::Objects;

#[derive(Debug, Clone)]
pub struct ParsedObjects {
    enums: Vec<ParsedDefiner>,
    flags: Vec<ParsedDefiner>,
    structs: Vec<Container>,
    messages: Vec<Container>,
    tests: Vec<TestCase>,
}

impl ParsedObjects {
    pub fn empty() -> Self {
        Self {
            enums: vec![],
            flags: vec![],
            structs: vec![],
            messages: vec![],
            tests: vec![],
        }
    }

    pub fn new(
        enums: Vec<ParsedDefiner>,
        flags: Vec<ParsedDefiner>,
        structs: Vec<Container>,
        messages: Vec<Container>,
        tests: Vec<TestCase>,
    ) -> Self {
        Self {
            enums,
            flags,
            structs,
            messages,
            tests,
        }
    }

    pub fn add_vecs(&mut self, mut c: Self) {
        self.enums.append(&mut c.enums);
        self.flags.append(&mut c.flags);
        self.structs.append(&mut c.structs);
        self.messages.append(&mut c.messages);
        self.tests.append(&mut c.tests);
    }

    pub fn to_objects(self) -> Objects {
        let mut o = Objects::new(
            self.enums,
            self.flags,
            self.structs,
            self.messages,
            self.tests,
        );
        o.check_values();
        o.sort_members();

        o
    }
}
