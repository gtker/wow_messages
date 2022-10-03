use crate::file_info::FileInfo;
use crate::parser::types::container::{Container, DefinerUsage, Sizes};
use crate::parser::types::definer::Definer;
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
    tests: Vec<TestCase>,
}

impl Objects {
    pub fn new(
        enums: Vec<Definer>,
        flags: Vec<Definer>,
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

    pub fn object_has_only_io_errors(&self, variable_name: &str, finder_tags: &Tags) -> bool {
        match self.get_object_type_of(variable_name, finder_tags) {
            ObjectType::Struct | ObjectType::CLogin | ObjectType::SLogin => {
                let c = self.get_container(variable_name, finder_tags);
                c.recursive_only_has_io_errors(self)
            }
            ObjectType::Enum => self
                .get_definer(variable_name, finder_tags)
                .self_value()
                .is_some(),
            ObjectType::Flag => true,
        }
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

    pub fn empty() -> Self {
        Self {
            enums: vec![],
            flags: vec![],
            structs: vec![],
            messages: vec![],
            tests: vec![],
        }
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

    pub fn add_rust_views(&mut self) {
        let temp = self.clone();
        for e in self.all_containers_mut() {
            e.set_rust_object(create_rust_object(e, &temp));
        }
    }

    fn get_definer_objects_used_in(
        containers: &[Container],
        e: &Definer,
    ) -> Vec<(String, DefinerUsage)> {
        let mut v = Vec::new();

        for c in containers {
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

        v
    }

    pub fn check_values(&mut self) {
        let c = self.clone();
        for s in &mut self.tests {
            s.verify(&c);
        }

        for e in self.all_definers() {
            e.self_check();
        }

        let mut tests = self.tests.clone();

        for s in self.all_containers_mut() {
            s.set_internals(&c);

            let t = Self::get_tests_for_object(&mut tests, s.name(), s.tags());
            s.append_tests(t);
        }

        let containers = self.all_containers().cloned().collect::<Vec<_>>();
        for e in self.all_definers_mut() {
            let objects_used_in = Self::get_definer_objects_used_in(&containers, e);
            e.set_objects_used_in(objects_used_in);
        }

        for e in self.all_containers() {
            e.check_if_statement_operators(self);
        }

        Self::check_versions(self.all_containers(), self.all_definers());

        self.add_rust_views();

        self.tests = tests;
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
            return s.has_constant_size(self);
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

    pub fn add_vecs(&mut self, mut c: Objects) {
        self.enums.append(&mut c.enums);
        self.flags.append(&mut c.flags);
        self.structs.append(&mut c.structs);
        self.messages.append(&mut c.messages);
        self.tests.append(&mut c.tests);
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
