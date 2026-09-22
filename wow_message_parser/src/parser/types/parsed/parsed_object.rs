use crate::parser::types::if_statement::DefinerUsage;
use crate::parser::types::parsed::parsed_container::ParsedContainer;
use crate::parser::types::parsed::parsed_definer::ParsedDefiner;
use crate::parser::types::parsed::parsed_test_case::ParsedTestCase;
use crate::parser::types::parsed::parsed_update_mask::ParsedUpdateMaskField;
use crate::Objects;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub(crate) struct ParsedObjects {
    enums: Vec<ParsedDefiner>,
    flags: Vec<ParsedDefiner>,
    structs: Vec<ParsedContainer>,
    messages: Vec<ParsedContainer>,
    tests: Vec<ParsedTestCase>,
    update_mask_fields: Vec<ParsedUpdateMaskField>,
}

impl ParsedObjects {
    pub(crate) fn empty() -> Self {
        Self {
            enums: vec![],
            flags: vec![],
            structs: vec![],
            messages: vec![],
            tests: vec![],
            update_mask_fields: vec![],
        }
    }

    pub(crate) fn new(
        enums: Vec<ParsedDefiner>,
        flags: Vec<ParsedDefiner>,
        structs: Vec<ParsedContainer>,
        messages: Vec<ParsedContainer>,
        tests: Vec<ParsedTestCase>,
        update_mask_fields: Vec<ParsedUpdateMaskField>,
    ) -> Self {
        Self {
            enums,
            flags,
            structs,
            messages,
            tests,
            update_mask_fields,
        }
    }

    pub(crate) fn add_vecs(&mut self, mut c: Self) {
        self.enums.append(&mut c.enums);
        self.flags.append(&mut c.flags);
        self.structs.append(&mut c.structs);
        self.messages.append(&mut c.messages);
        self.tests.append(&mut c.tests);
        self.update_mask_fields.append(&mut c.update_mask_fields);
    }

    pub(crate) fn into_objects(self) -> Objects {
        Objects::new(
            self.enums,
            self.flags,
            self.structs,
            self.messages,
            self.tests,
            self.update_mask_fields,
        )
    }
}

pub(crate) fn get_objects_used_in(
    containers: &[ParsedContainer],
    e: &ParsedContainer,
) -> BTreeSet<String> {
    let mut v = BTreeSet::new();

    for c in containers {
        if !e.tags().has_version_intersections(c.tags()) {
            continue;
        }

        if c.contains_container(e.name()) {
            v.insert(c.name().to_string());
        }
    }

    v
}

pub(crate) fn get_definer_objects_used_in(
    containers: &[ParsedContainer],
    e: &ParsedDefiner,
) -> Vec<(String, DefinerUsage)> {
    let mut v = Vec::new();

    for c in containers {
        if !e.tags().has_version_intersections(c.tags()) {
            continue;
        }

        let ty = match c.contains_definer(e.name()) {
            None => continue,
            Some(DefinerUsage::NotInIf) => DefinerUsage::NotInIf,
            Some(DefinerUsage::InIf) => DefinerUsage::InIf,
        };

        v.push((c.name().to_string(), ty));
    }

    v.sort_by(|a, b| a.0.cmp(&b.0));

    v
}
