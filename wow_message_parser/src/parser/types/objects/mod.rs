pub mod conversion;

use crate::parser::types::container::Container;
use crate::parser::types::definer::Definer;
use crate::parser::types::objects::conversion::object_new;
use crate::parser::types::parsed::parsed_container::ParsedContainer;
use crate::parser::types::parsed::parsed_definer::ParsedDefiner;
use crate::parser::types::parsed::parsed_test_case::ParsedTestCase;
use crate::parser::types::tags::ObjectTags;
use crate::parser::types::test_case::TestCase;
use crate::parser::types::version::{LoginVersion, MajorWorldVersion, Version};
use crate::ContainerType;

#[derive(Debug, Clone)]
pub(crate) struct Objects {
    enums: Vec<Definer>,
    flags: Vec<Definer>,
    structs: Vec<Container>,
    messages: Vec<Container>,
    tests: Vec<TestCase>,
}

impl Objects {
    pub(crate) fn new(
        enums: Vec<ParsedDefiner>,
        flags: Vec<ParsedDefiner>,
        structs: Vec<ParsedContainer>,
        messages: Vec<ParsedContainer>,
        tests: Vec<ParsedTestCase>,
    ) -> Self {
        object_new(enums, flags, structs, messages, tests)
    }

    pub(crate) fn get_container(&self, name: &str, finder_tags: &ObjectTags) -> &Container {
        if let Some(e) = self
            .all_containers()
            .find(|a| a.name() == name && a.tags().has_version_intersections(finder_tags))
        {
            return e;
        }

        panic!(
            "container not found: {} with tags: {:#?}",
            name, finder_tags
        );
    }

    pub(crate) fn get_tags_of_object_fallible(
        &self,
        type_name: &str,
        finder_tags: &ObjectTags,
    ) -> Option<&ObjectTags> {
        if let Some(e) = self
            .enums
            .iter()
            .find(|a| a.name() == type_name && a.tags().has_version_intersections(finder_tags))
        {
            return Some(e.tags());
        }

        if let Some(e) = self
            .flags
            .iter()
            .find(|a| a.name() == type_name && a.tags().has_version_intersections(finder_tags))
        {
            return Some(e.tags());
        }

        if let Some(e) = self
            .all_containers()
            .find(|a| a.name() == type_name && a.tags().has_version_intersections(finder_tags))
        {
            return Some(e.tags());
        }

        None
    }

    pub(crate) fn get_main_world_versions_with_objects(&self) -> Vec<MajorWorldVersion> {
        let mut v = Vec::new();

        for s in self.all_containers() {
            for l in s.tags().versions() {
                if l.is_main_version() {
                    v.push(l.as_major_world());
                }
            }
        }

        v.sort();
        v.dedup();

        v
    }

    pub(crate) fn get_world_messages_with_versions_and_all(
        &self,
        version_number: &MajorWorldVersion,
    ) -> Vec<&Container> {
        let mut v = Vec::new();

        for e in self.all_containers() {
            let tags = ObjectTags::new_with_version(Version::World(*version_number));
            if e.tags().fulfills_all(&tags) {
                v.push(e);
            }
        }

        v
    }

    pub(crate) fn get_login_versions_with_objects(&self) -> Vec<LoginVersion> {
        let mut v = Vec::new();

        for s in self.all_containers() {
            for l in s.tags().logon_versions() {
                v.push(l);
            }
        }

        v.sort();
        v.dedup();

        let index = v.iter().position(|a| a.eq(&LoginVersion::All));
        if let Some(index) = index {
            v.remove(index);
        }

        v
    }

    pub(crate) fn get_login_messages_with_versions_and_all(
        &self,
        version_number: &LoginVersion,
    ) -> Vec<&Container> {
        let mut v = Vec::new();

        for s in self.all_containers() {
            let mut logon = s.tags().logon_versions();
            if logon.any(|a| a == *version_number || a == LoginVersion::All) {
                v.push(s);
            }
        }

        v
    }

    pub(crate) fn enums(&self) -> &[Definer] {
        &self.enums
    }

    pub(crate) fn flags(&self) -> &[Definer] {
        &self.flags
    }

    pub(crate) fn all_objects(&self) -> impl Iterator<Item = Object> + '_ {
        self.enums
            .iter()
            .map(|a| Object::Enum(a.clone()))
            .chain(self.flags.iter().map(|a| Object::Flag(a.clone())))
            .chain(self.all_containers().map(|a| Object::Container(a.clone())))
    }

    pub(crate) fn all_containers(&self) -> impl Iterator<Item = &Container> {
        self.structs.iter().chain(&self.messages)
    }

    pub(crate) fn messages(&self) -> &[Container] {
        &self.messages
    }

    pub(crate) fn wireshark_messages(&self) -> Vec<&Container> {
        let mut v = self
            .wireshark_containers()
            .into_iter()
            .filter(|a| match a.container_type() {
                ContainerType::Msg(_) | ContainerType::CMsg(_) | ContainerType::SMsg(_) => true,
                _ => false,
            })
            .collect::<Vec<&Container>>();

        v.sort_by(|a, b| a.name().cmp(b.name()));

        v
    }

    pub(crate) fn wireshark_containers(&self) -> Vec<&Container> {
        self.all_containers()
            .filter(|e| {
                e.tags().fulfills_all(&ObjectTags::new_with_version(
                    MajorWorldVersion::Vanilla.into(),
                ))
            })
            .collect()
    }

    pub(crate) fn sort_members(&mut self) {
        self.structs.sort();
        self.messages.sort();
        self.enums.sort();
        self.flags.sort();
    }

    pub fn tests(&self) -> &[TestCase] {
        &self.tests
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum Object {
    Container(Container),
    Enum(Definer),
    Flag(Definer),
}

impl Object {
    pub(crate) fn tags(&self) -> &ObjectTags {
        match self {
            Object::Container(e) => e.tags(),
            Object::Enum(e) => e.tags(),
            Object::Flag(e) => e.tags(),
        }
    }

    pub(crate) fn name(&self) -> &str {
        match self {
            Object::Container(e) => e.name(),
            Object::Enum(e) => e.name(),
            Object::Flag(e) => e.name(),
        }
    }
}
