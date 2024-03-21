pub mod conversion;

use crate::parser::types::container::Container;
use crate::parser::types::definer::Definer;
use crate::parser::types::objects::conversion::object_new;
use crate::parser::types::parsed::parsed_container::ParsedContainer;
use crate::parser::types::parsed::parsed_definer::ParsedDefiner;
use crate::parser::types::parsed::parsed_test_case::ParsedTestCase;
use crate::parser::types::tags::ObjectTags;
use crate::parser::types::test_case::TestCase;
use crate::parser::types::version::{AllRustVersions, LoginVersion, MajorWorldVersion};
use crate::ContainerType;
use std::collections::BTreeSet;

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

    pub(crate) fn get_rust_world_versions_with_objects(&self) -> BTreeSet<MajorWorldVersion> {
        let mut b = BTreeSet::new();

        for e in self.all_containers() {
            if let Some(AllRustVersions::World(w)) = e.tags().try_all_rust_versions() {
                b.append(&mut w.clone());
            }
        }

        b
    }

    pub(crate) fn get_world_messages_with_versions_and_all(
        &self,
        version_number: &MajorWorldVersion,
    ) -> Vec<&Container> {
        let mut v = Vec::new();

        for e in self.all_containers() {
            let all = if let Some(all) = e.tags().try_all_rust_versions() {
                all
            } else {
                continue;
            };

            if all.has_world(version_number) {
                v.push(e);
            }
        }

        v
    }

    pub(crate) fn get_login_versions_with_objects(&self) -> BTreeSet<LoginVersion> {
        let mut b = BTreeSet::new();

        for e in self.all_containers() {
            if let Some(AllRustVersions::Login(l)) = e.tags().try_all_rust_versions() {
                b.append(&mut l.clone());
            }
        }

        let _ = b.remove(&LoginVersion::All);

        b
    }

    pub(crate) fn get_login_messages_with_versions_and_all(
        &self,
        version_number: &LoginVersion,
    ) -> Vec<&Container> {
        let mut v = Vec::new();

        for s in self.all_containers() {
            let all = if let Some(all) = s.tags().try_all_rust_versions() {
                all
            } else {
                continue;
            };
            if all.has_login(version_number) {
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
            .map(Object::Enum)
            .chain(self.flags.iter().map(Object::Flag))
            .chain(self.all_containers().map(Object::Container))
    }

    pub(crate) fn all_containers(&self) -> impl Iterator<Item = &Container> {
        self.structs.iter().chain(&self.messages)
    }

    pub(crate) fn all_definers(&self) -> impl Iterator<Item = &Definer> {
        self.enums.iter().chain(&self.flags)
    }

    pub(crate) fn messages(&self) -> &[Container] {
        &self.messages
    }

    pub(crate) fn structs(&self) -> &[Container] {
        &self.structs
    }

    pub(crate) fn world_wireshark_messages(&self) -> Vec<&Container> {
        let mut v = self
            .world_wireshark_containers()
            .into_iter()
            .filter(|a| {
                matches!(
                    a.container_type(),
                    ContainerType::Msg(_) | ContainerType::CMsg(_) | ContainerType::SMsg(_)
                )
            })
            .collect::<Vec<&Container>>();

        v.sort_by(|a, b| a.name().cmp(b.name()));

        v
    }

    pub(crate) fn all_login_wireshark_messages(&self) -> Vec<&Container> {
        let mut v = self
            .messages()
            .into_iter()
            .filter(|a| {
                matches!(
                    a.container_type(),
                    ContainerType::CLogin(_) | ContainerType::SLogin(_)
                ) && a.tags().has_login_version()
            })
            .collect::<Vec<&Container>>();

        v.sort_by(|a, b| a.name().cmp(b.name()));

        v
    }

    pub(crate) fn login_wireshark_messages(&self) -> Vec<&Container> {
        let mut v = self
            .login_wireshark_containers()
            .into_iter()
            .filter(|a| {
                matches!(
                    a.container_type(),
                    ContainerType::CLogin(_) | ContainerType::SLogin(_)
                )
            })
            .collect::<Vec<&Container>>();

        v.sort_by(|a, b| a.name().cmp(b.name()));

        v
    }

    pub(crate) fn login_wireshark_containers(&self) -> Vec<&Container> {
        self.all_containers()
            .filter(|e| e.tags().has_login_version_8_or_all() || e.name() == "CMD_SURVEY_RESULT")
            .collect()
    }

    pub(crate) fn world_wireshark_containers(&self) -> Vec<&Container> {
        self.all_containers()
            .filter(|e| {
                e.tags().fulfills_all(&ObjectTags::new_with_version(
                    MajorWorldVersion::Vanilla.into(),
                ))
            })
            .collect()
    }

    pub(crate) fn get_world_enum(&self, name: &str, version: MajorWorldVersion) -> &Definer {
        let tags = ObjectTags::new_with_world_versions(&[version]);

        self.enums
            .iter()
            .find(|a| a.name() == name && a.tags().fulfills_all(&tags))
            .unwrap()
    }

    pub(crate) fn get_world_struct(&self, name: &str, version: MajorWorldVersion) -> &Container {
        let tags = ObjectTags::new_with_world_versions(&[version]);

        self.structs
            .iter()
            .find(|a| a.name() == name && a.tags().fulfills_all(&tags))
            .unwrap()
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
pub(crate) enum Object<'a> {
    Container(&'a Container),
    Enum(&'a Definer),
    Flag(&'a Definer),
}

impl Object<'_> {
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
