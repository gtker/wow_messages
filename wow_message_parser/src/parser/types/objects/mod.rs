pub mod conversion;

use crate::file_info::FileInfo;
use crate::parser::types::container::Container;
use crate::parser::types::definer::Definer;
use crate::parser::types::parsed_container::ParsedContainer;
use crate::parser::types::parsed_definer::ParsedDefiner;
use crate::parser::types::parsed_test_case::ParsedTestCase;
use crate::parser::types::sizes::Sizes;
use crate::parser::types::tags::{LoginVersion, Tags, WorldVersion};
use crate::parser::types::test_case::TestCase;
use crate::parser::types::ty::Type;
use crate::parser::types::{ArraySize, ArrayType, ObjectType};
use crate::rust_printer::rust_view::create_rust_object;
use crate::{DefinerType, Version};

#[derive(Debug, Clone)]
pub struct Objects {
    enums: Vec<Definer>,
    flags: Vec<Definer>,
    structs: Vec<Container>,
    messages: Vec<Container>,
}

impl Objects {
    pub fn new(
        enums: Vec<ParsedDefiner>,
        flags: Vec<ParsedDefiner>,
        structs: Vec<ParsedContainer>,
        messages: Vec<ParsedContainer>,
        tests: Vec<ParsedTestCase>,
    ) -> Self {
        let enums = conversion::parsed_definer_to_definer(enums, &structs, &messages);
        let flags = conversion::parsed_definer_to_definer(flags, &structs, &messages);

        let containers = [structs.as_slice(), messages.as_slice()].concat();
        let definers = [enums.as_slice(), flags.as_slice()].concat();

        let mut tests =
            conversion::parsed_test_case_to_test_case(tests, &containers, &enums, &flags);

        let structs =
            conversion::parsed_container_to_container(structs, &mut tests, &containers, &definers);
        let messages =
            conversion::parsed_container_to_container(messages, &mut tests, &containers, &definers);

        let mut o = Self {
            enums,
            flags,
            structs,
            messages,
        };

        o.check_values();
        o.sort_members();

        o
    }

    pub fn try_get_definer(&self, ty_name: &str, tags: &Tags) -> Option<&Definer> {
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

    pub fn get_definer(&self, ty_name: &str, tags: &Tags) -> &Definer {
        self.try_get_definer(ty_name, tags)
            .unwrap_or_else(|| panic!("unable to find definer: '{}'", ty_name))
    }

    pub fn get_object_type_of(&self, type_name: &str, finder_tags: &Tags) -> ObjectType {
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

    pub fn get_object(&self, name: &str, finder_tags: &Tags) -> Object {
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

    pub fn get_container(&self, name: &str, finder_tags: &Tags) -> &Container {
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

    pub fn get_tags_of_object_fallible(
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

    pub fn get_tags_of_object(&self, type_name: &str, finder_tags: &Tags) -> &Tags {
        if let Some(tags) = self.get_tags_of_object_fallible(type_name, finder_tags) {
            return tags;
        }

        panic!(
            "unable to find type: '{}' with tags '{:?}'",
            type_name, finder_tags
        );
    }

    pub fn get_main_world_versions_with_objects(&self) -> Vec<WorldVersion> {
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

    pub fn get_world_messages_with_versions_and_all(
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

    pub fn get_login_versions_with_objects(&self) -> Vec<LoginVersion> {
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

    pub fn get_login_messages_with_versions_and_all(
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

    pub fn get_size_of_complex(&self, type_name: &str) -> u64 {
        if let Some(e) = self.all_definers().find(|a| a.name() == type_name) {
            return e.ty().size() as u64;
        }

        panic!(
            "complex types that are not enum/flag before size: '{type_name}'",
            type_name = type_name
        )
    }

    pub fn enums(&self) -> &[Definer] {
        &self.enums
    }

    pub fn flags(&self) -> &[Definer] {
        &self.flags
    }

    pub fn all_objects(&self) -> impl Iterator<Item = Object> + '_ {
        self.enums
            .iter()
            .map(|a| Object::Enum(a.clone()))
            .chain(self.flags.iter().map(|a| Object::Flag(a.clone())))
            .chain(self.all_containers().map(|a| Object::Container(a.clone())))
    }

    pub fn all_definers(&self) -> impl Iterator<Item = &Definer> {
        self.enums.iter().chain(&self.flags)
    }

    pub fn all_definers_mut(&mut self) -> impl Iterator<Item = &mut Definer> {
        self.enums.iter_mut().chain(&mut self.flags)
    }

    pub fn all_containers(&self) -> impl Iterator<Item = &Container> {
        self.structs.iter().chain(&self.messages)
    }

    pub fn all_containers_mut(&mut self) -> impl Iterator<Item = &mut Container> {
        self.structs.iter_mut().chain(&mut self.messages)
    }

    pub fn structs(&self) -> &[Container] {
        &self.structs
    }

    pub fn structs_mut(&mut self) -> &mut [Container] {
        &mut self.structs
    }

    pub fn messages(&self) -> &[Container] {
        &self.messages
    }

    pub fn wireshark_messages(&self) -> Vec<&Container> {
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

    pub fn wireshark_containers(&self) -> Vec<&Container> {
        self.all_containers()
            .filter(|e| {
                e.tags()
                    .fulfills_all(&Tags::new_with_version(WorldVersion::Minor(1, 12).into()))
            })
            .collect()
    }

    pub fn messages_mut(&mut self) -> &mut [Container] {
        &mut self.messages
    }

    pub fn sort_members(&mut self) {
        self.structs.sort();
        self.messages.sort();
        self.enums.sort();
        self.flags.sort();
    }

    pub fn get_tests_for_object(
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

    pub fn check_values(&mut self) {
        let c = self.clone();

        for s in self.all_containers_mut() {
            s.set_internals(&c);
        }

        for e in self.all_containers() {
            e.check_if_statement_operators(self);
        }

        Self::check_versions(self.all_containers(), self.all_definers());

        for e in self.all_containers_mut() {
            e.set_rust_object(create_rust_object(e, &c));
        }
    }

    fn check_versions<'a>(
        containers: impl Iterator<Item = &'a Container>,
        definers: impl Iterator<Item = &'a Definer>,
    ) {
        struct Obj<'a> {
            name: &'a str,
            tags: &'a Tags,
            file_info: &'a FileInfo,
        }

        let mut v: Vec<Obj> = Vec::new();
        for e in containers {
            v.push(Obj {
                name: e.name(),
                tags: e.tags(),
                file_info: e.file_info(),
            });
        }
        for e in definers {
            v.push(Obj {
                name: e.name(),
                tags: e.tags(),
                file_info: e.file_info(),
            });
        }

        for outer in &v {
            for inner in &v {
                if outer.name == inner.name
                    && outer.tags.has_version_intersections(inner.tags)
                    && outer.name as *const _ != inner.name as *const _
                {
                    panic!(
                        "Objects with same name and overlapping versions: {}
version 1: {:#?} in {} line {},
version 2: {:#?} in {} line {}",
                        inner.name,
                        inner.tags,
                        inner.file_info.name(),
                        inner.file_info.start_line(),
                        outer.tags,
                        outer.file_info.name(),
                        outer.file_info.start_line(),
                    );
                }
            }
        }
    }

    pub fn type_has_constant_size(&self, ty: &Type) -> bool {
        let type_name = match ty {
            Type::DateTime
            | Type::Bool
            | Type::Integer(_)
            | Type::FloatingPoint(_)
            | Type::Guid => return true,
            Type::PackedGuid
            | Type::UpdateMask
            | Type::AuraMask
            | Type::SizedCString
            | Type::CString
            | Type::String { .. } => return false,
            Type::Array(array) => match array.size() {
                ArraySize::Fixed(_) => match array.ty() {
                    ArrayType::Guid | ArrayType::Integer(_) => return true,
                    ArrayType::Complex(ident) => ident,
                    ArrayType::CString | ArrayType::PackedGuid => return false,
                },
                ArraySize::Variable(_) | ArraySize::Endless => return false,
            },
            Type::Identifier { s, .. } => s,
        };

        if self.all_definers().any(|a| a.name() == type_name) {
            return true;
        }

        if let Some(s) = self.all_containers().find(|a| a.name() == type_name) {
            return s.is_constant_sized();
        }

        panic!(
            "Type name: '{type_name}' was not found.",
            type_name = type_name
        );
    }

    pub fn contains_value_in_type(&self, variable_name: &str, value_name: &str) {
        let enums = self.all_definers().find(|a| a.name() == variable_name);
        match enums {
            None => {}
            Some(v) => {
                for a in v.fields() {
                    if a.name() == value_name {
                        return;
                    }
                }
            }
        }

        panic!(
            "value: '{}' not found in variable: '{}'",
            value_name, variable_name
        );
    }

    pub fn contains_complex_type(&self, variable_name: &str, tags: &Tags, struct_name: &str) {
        for e in self.all_definers() {
            if e.name() == variable_name && e.tags().fulfills_all(tags) {
                return;
            }
        }

        for e in self.all_containers() {
            if e.name() == variable_name && e.tags().fulfills_all(tags) {
                return;
            }
        }

        panic!(
            "Complex type not found: '{}' for object: '{}' for versions logon: '{:?}', versions: '{:?}'",
            variable_name,
            struct_name,
            tags.logon_versions(),
            tags.versions()
        );
    }

    pub fn get_definer_field_value(
        &self,
        definer_name: &str,
        field_name: &str,
        tags: &Tags,
    ) -> u64 {
        if let Some(e) = self
            .all_definers()
            .find(|a| a.name() == definer_name && a.tags().has_version_intersections(tags))
        {
            for field in e.fields() {
                if field.name() == field_name {
                    return field.value().int();
                }
            }
        }

        panic!(
            "field not found: '{field_name}' in definer: '{definer_name}'",
            field_name = field_name,
            definer_name = definer_name
        )
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
    pub fn tags(&self) -> &Tags {
        match self {
            Object::Container(e) => e.tags(),
            Object::Enum(e) => e.tags(),
            Object::Flag(e) => e.tags(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Object::Container(e) => e.name(),
            Object::Enum(e) => e.name(),
            Object::Flag(e) => e.name(),
        }
    }

    pub fn sizes(&self) -> Sizes {
        match self {
            Object::Container(e) => e.sizes(),
            Object::Enum(e) | Object::Flag(e) => e.sizes(),
        }
    }
}
