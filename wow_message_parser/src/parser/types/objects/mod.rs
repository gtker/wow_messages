pub mod conversion;

use crate::parser::types::container::Container;
use crate::parser::types::definer::Definer;
use crate::parser::types::objects::conversion::object_new;
use crate::parser::types::parsed::parsed_container::ParsedContainer;
use crate::parser::types::parsed::parsed_definer::ParsedDefiner;
use crate::parser::types::parsed::parsed_test_case::ParsedTestCase;
use crate::parser::types::sizes::Sizes;
use crate::parser::types::tags::{LoginVersion, Tags, WorldVersion};
use crate::parser::types::test_case::TestCase;
use crate::parser::types::ObjectType;
use crate::{DefinerType, Version};

#[derive(Debug, Clone)]
pub struct Objects {
    enums: Vec<Definer>,
    flags: Vec<Definer>,
    structs: Vec<Container>,
    messages: Vec<Container>,
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

    pub(crate) fn try_get_definer(&self, ty_name: &str, tags: &Tags) -> Option<&Definer> {
        if let Some(d) = self
            .enums
            .iter()
            .find(|a| a.name() == ty_name && a.tags().has_version_intersections(tags))
        {
            return Some(d);
        }

        if let Some(d) = self
            .flags
            .iter()
            .find(|a| a.name() == ty_name && a.tags().has_version_intersections(tags))
        {
            return Some(d);
        }

        None
    }

    pub(crate) fn get_definer(&self, ty_name: &str, tags: &Tags) -> &Definer {
        self.try_get_definer(ty_name, tags)
            .unwrap_or_else(|| panic!("unable to find definer: '{}'", ty_name))
    }

    pub(crate) fn get_object_type_of(&self, type_name: &str, finder_tags: &Tags) -> ObjectType {
        if self
            .enums
            .iter()
            .any(|a| a.name() == type_name && a.tags().has_version_intersections(finder_tags))
        {
            return ObjectType::Enum;
        }

        if self
            .flags
            .iter()
            .any(|a| a.name() == type_name && a.tags().has_version_intersections(finder_tags))
        {
            return ObjectType::Flag;
        }

        if self
            .structs
            .iter()
            .any(|a| a.name() == type_name && a.tags().has_version_intersections(finder_tags))
        {
            return ObjectType::Struct;
        }

        panic!(
            "unable to find variable name: '{}' with tags: '{:#?}'",
            type_name, finder_tags
        );
    }

    pub(crate) fn get_object(&self, name: &str, finder_tags: &Tags) -> Object {
        if let Some(e) = self
            .all_containers()
            .find(|a| a.name() == name && a.tags().fulfills_all(finder_tags))
        {
            return Object::Container(e.clone());
        }

        if let Some(e) = self
            .all_definers()
            .find(|a| a.name() == name && a.tags().fulfills_all(finder_tags))
        {
            match e.definer_ty() {
                DefinerType::Enum => return Object::Enum(e.clone()),
                DefinerType::Flag => return Object::Flag(e.clone()),
            }
        }

        panic!(
            "unable to find variable name: '{}' with tags: '{:#?}'",
            name, finder_tags
        );
    }

    pub(crate) fn get_container(&self, name: &str, finder_tags: &Tags) -> &Container {
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
        finder_tags: &Tags,
    ) -> Option<&Tags> {
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

    pub(crate) fn get_tags_of_object(&self, type_name: &str, finder_tags: &Tags) -> &Tags {
        if let Some(tags) = self.get_tags_of_object_fallible(type_name, finder_tags) {
            return tags;
        }

        panic!(
            "unable to find type: '{}' with tags '{:?}'",
            type_name, finder_tags
        );
    }

    pub(crate) fn get_main_world_versions_with_objects(&self) -> Vec<WorldVersion> {
        let mut v = Vec::new();

        for s in self.all_containers() {
            for l in s.tags().versions() {
                if l.is_main_version() {
                    v.push(*l);
                }
            }
        }

        v.sort();
        v.dedup();

        let index = v.iter().position(|a| a.eq(&WorldVersion::All));
        if let Some(index) = index {
            v.remove(index);
        }

        v
    }

    pub(crate) fn get_world_messages_with_versions_and_all(
        &self,
        version_number: &WorldVersion,
    ) -> Vec<&Container> {
        let mut v = Vec::new();

        for e in self.all_containers() {
            let tags = Tags::new_with_version(Version::World(*version_number));
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
                v.push(*l);
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
            let logon = s.tags().logon_versions();
            if logon.contains(version_number) || logon.contains(&LoginVersion::All) {
                v.push(s);
            }
        }

        v
    }

    pub(crate) fn get_size_of_complex(&self, type_name: &str) -> u64 {
        if let Some(e) = self.all_definers().find(|a| a.name() == type_name) {
            return e.ty().size() as u64;
        }

        panic!(
            "complex types that are not enum/flag before size: '{type_name}'",
            type_name = type_name
        )
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

    pub(crate) fn all_definers(&self) -> impl Iterator<Item = &Definer> {
        self.enums.iter().chain(&self.flags)
    }

    pub(crate) fn all_containers(&self) -> impl Iterator<Item = &Container> {
        self.structs.iter().chain(&self.messages)
    }

    pub(crate) fn messages(&self) -> &[Container] {
        &self.messages
    }

    pub(crate) fn wireshark_messages(&self) -> Vec<&Container> {
        let mut v: Vec<&Container> = self
            .messages()
            .iter()
            .filter(|e| {
                e.tags()
                    .fulfills_all(&Tags::new_with_version(WorldVersion::Minor(1, 12).into()))
            })
            .collect();

        v.sort_by(|a, b| a.name().cmp(b.name()));

        v
    }

    pub(crate) fn wireshark_containers(&self) -> Vec<&Container> {
        self.all_containers()
            .filter(|e| {
                e.tags()
                    .fulfills_all(&Tags::new_with_version(WorldVersion::Minor(1, 12).into()))
            })
            .collect()
    }

    pub(crate) fn sort_members(&mut self) {
        self.structs.sort();
        self.messages.sort();
        self.enums.sort();
        self.flags.sort();
    }

    pub(crate) fn get_tests_for_object(
        tests: &mut Vec<TestCase>,
        name: &str,
        tags: &Tags,
    ) -> Vec<TestCase> {
        let mut v = Vec::new();
        let mut indices = Vec::new();

        for (i, t) in tests.iter().enumerate() {
            if t.subject() == name && t.tags().has_version_intersections(tags) {
                indices.push(i);
                v.push(t.clone());
            }
        }
        indices.reverse();

        for i in indices {
            tests.remove(i);
        }

        v
    }
}

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Object {
    Container(Container),
    Enum(Definer),
    Flag(Definer),
}

impl Object {
    pub(crate) fn tags(&self) -> &Tags {
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

    pub(crate) fn sizes(&self) -> Sizes {
        match self {
            Object::Container(e) => e.sizes(),
            Object::Enum(e) | Object::Flag(e) => e.sizes(),
        }
    }
}
