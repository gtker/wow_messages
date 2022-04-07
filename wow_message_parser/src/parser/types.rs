use std::fmt::{Display, Formatter};

use crate::container::{Container, StructMember};
use crate::file_info::FileInfo;
use crate::parser::enumerator::Definer;
use crate::parser::stats::stats_for_1_12;
use crate::test_case::TestCase;
use crate::{LOGIN_LOGON_VERSIONS, TEST_STR, VERSIONS};

#[derive(Debug, Clone)]
pub struct Objects {
    enums: Vec<Definer>,
    flags: Vec<Definer>,
    structs: Vec<Container>,
    messages: Vec<Container>,
    tests: Vec<TestCase>,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ObjectType {
    Struct,
    CLogin,
    SLogin,
    Enum,
    Flag,
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

    pub fn get_definer(&self, ty_name: &str, tags: &Tags) -> &Definer {
        if let Some(d) = self
            .enums
            .iter()
            .find(|a| a.name() == ty_name && a.tags().has_version_intersections(tags))
        {
            return d;
        }

        if let Some(d) = self
            .flags
            .iter()
            .find(|a| a.name() == ty_name && a.tags().has_version_intersections(tags))
        {
            return d;
        }

        panic!("unable to find definer");
    }

    pub fn object_has_only_io_errors(&self, variable_name: &str, finder_tags: &Tags) -> bool {
        match self.get_object_type_of(variable_name, finder_tags) {
            ObjectType::Struct | ObjectType::CLogin | ObjectType::SLogin => {
                let c = self.get_container(variable_name, finder_tags);
                c.recursive_only_has_io_errors(self)
            }
            ObjectType::Enum => match self.get_definer(variable_name, finder_tags).self_value() {
                None => false,
                Some(_) => true,
            },
            ObjectType::Flag => true,
        }
    }

    pub fn get_object_type_of(&self, variable_name: &str, finder_tags: &Tags) -> ObjectType {
        if self
            .enums
            .iter()
            .find(|a| a.name() == variable_name && a.tags().has_version_intersections(finder_tags))
            .is_some()
        {
            return ObjectType::Enum;
        }

        if self
            .flags
            .iter()
            .find(|a| a.name() == variable_name && a.tags().has_version_intersections(finder_tags))
            .is_some()
        {
            return ObjectType::Flag;
        }

        if let Some(_) = self
            .structs
            .iter()
            .find(|a| a.name() == variable_name && a.tags().has_version_intersections(finder_tags))
        {
            return ObjectType::Struct;
        }

        panic!(
            "unable to find variable name: '{}' with tags: '{:#?}'",
            variable_name, finder_tags
        );
    }

    pub fn get_container(&self, name: &str, finder_tags: &Tags) -> &Container {
        if let Some(e) = self
            .messages
            .iter()
            .find(|a| a.name() == name && a.tags().has_version_intersections(finder_tags))
        {
            return e;
        }

        if let Some(e) = self
            .structs
            .iter()
            .find(|a| a.name() == name && a.tags().has_version_intersections(finder_tags))
        {
            return e;
        }

        panic!(
            "container not found: {} with tags: {:#?}",
            name, finder_tags
        );
    }

    pub fn get_tags_of_object(&self, type_name: &str, finder_tags: &Tags) -> &Tags {
        if let Some(e) = self
            .enums
            .iter()
            .find(|a| a.name() == type_name && a.tags().has_version_intersections(finder_tags))
        {
            return e.tags();
        }

        if let Some(e) = self
            .flags
            .iter()
            .find(|a| a.name() == type_name && a.tags().has_version_intersections(finder_tags))
        {
            return e.tags();
        }

        if let Some(e) = self
            .structs
            .iter()
            .find(|a| a.name() == type_name && a.tags().has_version_intersections(finder_tags))
        {
            return e.tags();
        }

        if let Some(e) = self
            .messages
            .iter()
            .find(|a| a.name() == type_name && a.tags().has_version_intersections(finder_tags))
        {
            return e.tags();
        }

        panic!("unable to find type");
    }

    pub fn get_world_versions_with_objects(&self) -> Vec<WorldVersion> {
        let mut v = Vec::new();

        for s in self.messages() {
            for l in s.tags().versions() {
                v.push(*l);
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

        for s in self.messages() {
            let logon = s.tags().versions();
            if logon.contains(version_number) || logon.contains(&WorldVersion::All) {
                v.push(s);
            }
        }

        v
    }

    pub fn get_login_versions_with_objects(&self) -> Vec<LoginVersion> {
        let mut v = Vec::new();

        for s in self.messages() {
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

        for s in self.messages() {
            let logon = s.tags().logon_versions();
            if logon.contains(version_number) || logon.contains(&LoginVersion::All) {
                v.push(s);
            }
        }

        v
    }

    pub fn get_size_of_complex(&self, type_name: &str) -> u64 {
        if let Some(e) = self.enums.iter().find(|a| a.name() == type_name) {
            return e.ty().size() as u64;
        }
        if let Some(e) = self.flags.iter().find(|a| a.name() == type_name) {
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

    pub fn structs(&self) -> &[Container] {
        &self.structs
    }

    pub fn structs_mut(&mut self) -> &mut [Container] {
        &mut self.structs
    }

    pub fn messages(&self) -> &[Container] {
        &self.messages
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
        for s in &mut self.tests {
            s.verify(&c);
        }

        let mut tests = self.tests.clone();

        for s in self.structs_mut() {
            s.set_internals(&c);

            let t = Self::get_tests_for_object(&mut tests, s.name(), s.tags());
            s.append_tests(t);
        }

        Self::check_versions(self.structs(), self.messages(), self.enums(), self.flags());

        for s in self.messages_mut() {
            s.set_internals(&c);

            let t = Self::get_tests_for_object(&mut tests, s.name(), s.tags());
            s.append_tests(t);
        }

        self.tests = tests;
    }

    fn check_versions(
        structs: &[Container],
        messages: &[Container],
        enums: &[Definer],
        flags: &[Definer],
    ) {
        struct Obj<'a> {
            name: &'a str,
            tags: &'a Tags,
            file_info: &'a FileInfo,
        }

        let mut v: Vec<Obj> = Vec::new();
        for e in structs {
            v.push(Obj {
                name: e.name(),
                tags: e.tags(),
                file_info: e.file_info(),
            });
        }
        for e in messages {
            v.push(Obj {
                name: e.name(),
                tags: e.tags(),
                file_info: e.file_info(),
            });
        }
        for e in enums {
            v.push(Obj {
                name: e.name(),
                tags: e.tags(),
                file_info: e.file_info(),
            });
        }
        for e in flags {
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
            Type::Integer(_) => return true,
            Type::FloatingPoint(_) => return true,
            Type::CString | Type::String { .. } => return false,
            Type::Array(array) => match array.size() {
                ArraySize::Fixed(_) => match array.ty() {
                    ArrayType::Integer(_) => return true,
                    ArrayType::Complex(ident) => ident,
                    ArrayType::CString => return false,
                    ArrayType::Guid => return true,
                    ArrayType::PackedGuid => return false,
                },
                ArraySize::Variable(_) => return false,
                ArraySize::Endless => return false,
            },
            Type::Identifier { s, .. } => s,
            Type::PackedGuid => return false,
            Type::Guid => return true,
            Type::UpdateMask => return false,
            Type::AuraMask => return false,
        };

        if self.enums.iter().find(|a| a.name() == type_name).is_some() {
            return true;
        }

        if self.flags.iter().find(|a| a.name() == type_name).is_some() {
            return true;
        }

        if let Some(s) = self.structs.iter().find(|a| a.name() == type_name) {
            return s.has_constant_size(self);
        }

        if let Some(s) = self.messages.iter().find(|a| a.name() == type_name) {
            return s.has_constant_size(self);
        }

        for s in self.structs() {
            if let Some(ce) = s
                .nested_types()
                .new_enums()
                .iter()
                .find(|&a| a.name() == type_name)
            {
                for f in ce.fields() {
                    for sf in f.subfields() {
                        match self.type_has_constant_size(sf.ty()) {
                            true => {}
                            false => return false,
                        }
                    }
                }

                return true;
            }
        }

        for s in self.messages() {
            if let Some(ce) = s
                .nested_types()
                .new_enums()
                .iter()
                .find(|&a| a.name() == type_name)
            {
                for f in ce.fields() {
                    for sf in f.subfields() {
                        match self.type_has_constant_size(sf.ty()) {
                            true => {}
                            false => return false,
                        }
                    }
                }

                return true;
            }
        }

        panic!(
            "Type name: '{type_name}' was not found.",
            type_name = type_name
        );
    }

    pub fn check_value(&self, i: &StructMember, tags: &Tags) {
        match i {
            StructMember::Definition(d) => match &d.ty() {
                Type::Integer(_) => {}
                Type::FloatingPoint(_) => {}
                Type::CString => {}
                Type::String { .. } => {}
                Type::Array(a) => match &a.inner {
                    ArrayType::Integer(_) => {}
                    ArrayType::Complex(c) => self.contains_complex_type(c, tags, d.name()),
                    ArrayType::CString => {}
                    ArrayType::Guid => {}
                    ArrayType::PackedGuid => {}
                },
                Type::Identifier { s: i, .. } => {
                    self.contains_complex_type(i, tags, d.name());
                    match d.value() {
                        None => {}
                        Some(v) => match v.identifier().parse::<usize>() {
                            Ok(_) => {}
                            Err(_) => {
                                self.contains_value_in_type(i, v.identifier());
                            }
                        },
                    }
                }
                Type::PackedGuid => {}
                Type::Guid => {}
                Type::UpdateMask => {}
                Type::AuraMask => {}
            },
            StructMember::IfStatement(statement) => {
                for member in &statement.members {
                    self.check_value(member, tags);
                }
                for member in &statement.else_statement_members {
                    self.check_value(member, tags);
                }
            }
            StructMember::OptionalStatement(optional) => {
                for m in optional.members() {
                    self.check_value(m, tags);
                }
            }
        }
    }

    fn contains_value_in_type(&self, variable_name: &str, value_name: &str) {
        let enums = self.enums.iter().find(|a| a.name() == variable_name);
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

        let flags = self.flags.iter().find(|a| a.name() == variable_name);
        match flags {
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

    fn contains_complex_type(&self, variable_name: &str, tags: &Tags, struct_name: &str) {
        for e in self.enums() {
            if e.name() == variable_name && e.tags().has_all_versions(tags) {
                return;
            }
        }

        for e in self.flags() {
            if e.name() == variable_name && e.tags().has_all_versions(tags) {
                return;
            }
        }

        for e in self.structs() {
            if e.name() == variable_name && e.tags().has_all_versions(tags) {
                return;
            }
        }

        for e in self.messages() {
            if e.name() == variable_name && e.tags().has_all_versions(tags) {
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
            .enums
            .iter()
            .find(|a| a.name() == definer_name && a.tags().has_version_intersections(tags))
        {
            for field in e.fields() {
                if field.name() == field_name {
                    return field.value().int();
                }
            }
        }

        if let Some(e) = self
            .flags
            .iter()
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

    pub fn print_stats_for_1_12(&self) {
        stats_for_1_12(self);
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Endianness {
    Little,
    Big,
}

impl Endianness {
    pub fn rust_str(&self) -> &str {
        match self {
            Endianness::Little => "le",
            Endianness::Big => "be",
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FloatingPointType {
    F32(Endianness),
    F64(Endianness),
}
impl FloatingPointType {
    pub fn size(&self) -> u8 {
        match self {
            FloatingPointType::F32(_) => 4,
            FloatingPointType::F64(_) => 8,
        }
    }

    pub fn str(&self) -> &str {
        match self {
            FloatingPointType::F32(e) => match e {
                Endianness::Little => "f32",
                Endianness::Big => "f32_be",
            },
            FloatingPointType::F64(e) => match e {
                Endianness::Little => "f64",
                Endianness::Big => "f64_be",
            },
        }
    }

    pub fn rust_str(&self) -> &str {
        match self {
            FloatingPointType::F32(_) => "f32",
            FloatingPointType::F64(_) => "f64",
        }
    }

    pub fn rust_endian_str(&self) -> &str {
        match self {
            FloatingPointType::F32(e) | FloatingPointType::F64(e) => e.rust_str(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum IntegerType {
    U8,
    U16(Endianness),
    U32(Endianness),
    U64(Endianness),
}

impl IntegerType {
    pub fn size(&self) -> u8 {
        match self {
            IntegerType::U8 => 1,
            IntegerType::U16(_) => 2,
            IntegerType::U32(_) => 4,
            IntegerType::U64(_) => 8,
        }
    }

    pub fn str(&self) -> &str {
        match self {
            IntegerType::U8 => "u8",
            IntegerType::U16(e) => match e {
                Endianness::Little => "u16",
                Endianness::Big => "u16_be",
            },
            IntegerType::U32(e) => match e {
                Endianness::Little => "u32",
                Endianness::Big => "u32_be",
            },
            IntegerType::U64(e) => match e {
                Endianness::Little => "u64",
                Endianness::Big => "u64_be",
            },
        }
    }

    pub fn rust_str(&self) -> &str {
        match self {
            IntegerType::U8 => "u8",
            IntegerType::U16(_) => "u16",
            IntegerType::U32(_) => "u32",
            IntegerType::U64(_) => "u64",
        }
    }

    pub fn rust_endian_str(&self) -> &str {
        match self {
            IntegerType::U8 => "le",
            IntegerType::U16(i) | IntegerType::U32(i) | IntegerType::U64(i) => i.rust_str(),
        }
    }
}

impl From<&str> for IntegerType {
    fn from(s: &str) -> Self {
        match s {
            "u8" => IntegerType::U8,
            "Bool" => IntegerType::U8,
            "u16" => IntegerType::U16(Endianness::Little),
            "u16_be" => IntegerType::U16(Endianness::Big),
            "u32" => IntegerType::U32(Endianness::Little),
            "Spell" => IntegerType::U32(Endianness::Little),
            "Item" => IntegerType::U32(Endianness::Little),
            "u32_be" => IntegerType::U32(Endianness::Big),
            "u64" => IntegerType::U64(Endianness::Little),
            "u64_be" => IntegerType::U64(Endianness::Big),
            _ => panic!("invalid basic type attempted to be created as IntegerType"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ArrayType {
    Integer(IntegerType),
    Complex(String),
    CString,
    Guid,
    PackedGuid,
}

impl ArrayType {
    pub fn rust_str(&self) -> String {
        match &self {
            ArrayType::Integer(i) => i.rust_str().to_string(),
            ArrayType::Complex(i) => i.clone(),
            ArrayType::CString => "String".to_string(),
            ArrayType::Guid => "Guid".to_string(),
            ArrayType::PackedGuid => "Guid".to_string(),
        }
    }

    pub fn str(&self) -> String {
        match self {
            ArrayType::Integer(i) => i.str().to_string(),
            ArrayType::Complex(i) => i.clone(),
            ArrayType::CString => "CString".to_string(),
            ArrayType::Guid => "Guid".to_string(),
            ArrayType::PackedGuid => "PackedGuid".to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ArraySize {
    Fixed(i64),
    Variable(String),
    Endless,
}

impl ArraySize {
    pub fn str(&self) -> String {
        match self {
            ArraySize::Fixed(i) => i.to_string(),
            ArraySize::Variable(i) => i.clone(),
            ArraySize::Endless => "-".to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Array {
    inner: ArrayType,
    size: ArraySize,
}

impl Array {
    pub fn ty(&self) -> &ArrayType {
        &self.inner
    }

    pub fn size(&self) -> ArraySize {
        self.size.clone()
    }

    pub fn str(&self) -> String {
        format!("{}[{}]", self.inner.str(), self.size.str())
    }

    pub fn rust_str(&self) -> String {
        match &self.size {
            ArraySize::Fixed(i) => format!("[{}; {}]", self.inner.rust_str(), i),
            ArraySize::Variable(_) | ArraySize::Endless => {
                format!("Vec<{}>", self.inner.rust_str())
            }
        }
    }

    pub fn rust_size_str(&self) -> String {
        match &self.size {
            ArraySize::Fixed(i) => i.to_string(),
            ArraySize::Variable(s) => s.clone(),
            ArraySize::Endless => panic!(),
        }
    }

    pub fn is_constant_sized_u8_array(&self) -> bool {
        match &self.size() {
            ArraySize::Fixed(_) => match &self.ty() {
                ArrayType::Integer(i) => match i {
                    IntegerType::U8 => true,
                    _ => false,
                },
                _ => false,
            },
            ArraySize::Variable(_) => false,
            ArraySize::Endless => false,
        }
    }

    pub fn rust_kind_str(&self) -> String {
        match &self.inner {
            ArrayType::Integer(i) => i.rust_str().to_string(),
            ArrayType::Complex(i) => i.clone(),
            ArrayType::CString => "String".to_string(),
            ArrayType::Guid | ArrayType::PackedGuid => "Guid".to_string(),
        }
    }

    pub fn rust_endian_str(&self) -> String {
        match self.inner {
            ArrayType::Integer(i) => i.rust_endian_str().to_string(),
            _ => panic!("endianness only supported for integer types"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Type {
    Integer(IntegerType),
    PackedGuid,
    Guid,
    FloatingPoint(FloatingPointType),
    CString,
    String {
        length: String,
    },
    Array(Array),
    Identifier {
        s: String,
        upcast: Option<IntegerType>,
    },
    UpdateMask,
    AuraMask,
}

impl Type {
    pub fn str(&self) -> String {
        match self {
            Type::Integer(i) => i.str().to_string(),
            Type::CString => "CString".to_string(),
            Type::String { length } => format!("String[{}]", length),
            Type::Array(a) => a.str(),
            Type::Identifier { s, .. } => s.clone(),
            Type::FloatingPoint(i) => i.str().to_string(),
            Type::PackedGuid => "PackedGuid".to_string(),
            Type::Guid => "Guid".to_string(),
            Type::UpdateMask => "UpdateMask".to_string(),
            Type::AuraMask => "AuraMask".to_string(),
        }
    }

    pub fn rust_str(&self) -> String {
        let s = match self {
            Type::Integer(i) => i.rust_str().to_string(),
            Type::FloatingPoint(i) => i.rust_str().to_string(),
            Type::Identifier { s, .. } => s.clone(),
            Type::CString => "String".to_string(),
            Type::Array(a) => a.rust_str(),
            Type::String { .. } => "String".to_string(),
            Type::PackedGuid | Type::Guid => "Guid".to_string(),
            Type::UpdateMask => "UpdateMask".to_string(),
            Type::AuraMask => "AuraMask".to_string(),
        };

        s
    }

    pub fn rust_size_of(&self) -> u8 {
        match self {
            Type::Integer(i) => i.size(),
            _ => panic!("attempting to get size of complex type: '{}'", self.str()),
        }
    }

    pub fn rust_endian_str(&self) -> &str {
        match self {
            Type::Integer(i) => i.rust_endian_str(),
            _ => panic!("endianness attempted for complex type"),
        }
    }

    pub fn with_upcast(s: &str, upcasted: &str) -> Self {
        let t = Self::from_str(s);
        match t {
            Type::Identifier { .. } => {}
            _ => panic!("upcast for type that does not support it"),
        }

        let int = match upcasted {
            "u16" => IntegerType::U16(Endianness::Little),
            "u32" => IntegerType::U32(Endianness::Little),
            "u64" => IntegerType::U64(Endianness::Little),
            "u16_be" => IntegerType::U16(Endianness::Big),
            "u32_be" => IntegerType::U32(Endianness::Big),
            "u64_be" => IntegerType::U64(Endianness::Big),
            _ => panic!("unsupported upcast: {}", upcasted),
        };

        Self::Identifier {
            s: s.to_string(),
            upcast: Some(int),
        }
    }

    pub fn from_str(s: &str) -> Self {
        let s = match s {
            "u8" => Self::Integer(IntegerType::U8),
            "Bool" => Self::Integer(IntegerType::U8),
            "u16" => Self::Integer(IntegerType::U16(Endianness::Little)),
            "u32" => Self::Integer(IntegerType::U32(Endianness::Little)),
            "Spell" => Self::Integer(IntegerType::U32(Endianness::Little)),
            "Item" => Self::Integer(IntegerType::U32(Endianness::Little)),
            "u64" => Self::Integer(IntegerType::U64(Endianness::Little)),
            "Guid" => Self::Guid,
            "PackedGuid" => Self::PackedGuid,
            "AuraMask" => Self::AuraMask,
            "UpdateMask" => Self::UpdateMask,
            "u16_be" => Self::Integer(IntegerType::U16(Endianness::Big)),
            "u32_be" => Self::Integer(IntegerType::U32(Endianness::Big)),
            "u64_be" => Self::Integer(IntegerType::U64(Endianness::Big)),
            "f32" => Self::FloatingPoint(FloatingPointType::F32(Endianness::Little)),
            "f32_be" => Self::FloatingPoint(FloatingPointType::F32(Endianness::Big)),
            "f64" => Self::FloatingPoint(FloatingPointType::F64(Endianness::Little)),
            "f64_be" => Self::FloatingPoint(FloatingPointType::F64(Endianness::Big)),
            "CString" => Self::CString,
            _ => Self::Identifier {
                s: s.to_string(),
                upcast: None,
            },
        };
        match s {
            Type::Identifier { s: i, .. } => {
                if i.contains('[') {
                    let mut i = i.split('[');
                    let array_type = i.next().unwrap();
                    let array_type: Type = Type::from_str(array_type);

                    let amount = i.next().unwrap().strip_suffix(']').unwrap();
                    let parsed = str::parse::<i64>(amount);

                    let size = if parsed.is_ok() {
                        ArraySize::Fixed(parsed.unwrap())
                    } else if amount == "-" {
                        ArraySize::Endless
                    } else {
                        ArraySize::Variable(amount.to_string())
                    };

                    match array_type {
                        Type::Integer(i) => Self::Array(Array {
                            inner: ArrayType::Integer(i),
                            size,
                        }),
                        Type::Identifier { s: i, .. } => {
                            if i == "String" {
                                return Self::String {
                                    length: amount.to_string(),
                                };
                            }

                            Self::Array(Array {
                                inner: ArrayType::Complex(i),
                                size,
                            })
                        }
                        Type::CString => Self::Array(Array {
                            inner: ArrayType::CString,
                            size,
                        }),
                        Type::String { .. } => panic!("unsupported"),
                        Type::Array(_) => panic!("unsupported"),
                        Type::FloatingPoint(_) => panic!("unsupported"),
                        Type::UpdateMask => panic!("unsupported"),
                        Type::AuraMask => panic!("unsupported"),
                        Type::PackedGuid => Self::Array(Array {
                            inner: ArrayType::PackedGuid,
                            size,
                        }),
                        Type::Guid => Self::Array(Array {
                            inner: ArrayType::Guid,
                            size,
                        }),
                    }
                } else {
                    Self::Identifier { s: i, upcast: None }
                }
            }
            s => s,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum WorldVersion {
    Major(u8),
    Minor(u8, u8),
    Patch(u8, u8, u8),
    Exact(u8, u8, u8, u8),
    All,
}

impl Display for WorldVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldVersion::Major(m) => f.write_fmt(format_args!("{}", m)),
            WorldVersion::Minor(m, i) => f.write_fmt(format_args!("{}.{}", m, i)),
            WorldVersion::Patch(m, i, p) => f.write_fmt(format_args!("{}.{}.{}", m, i, p)),
            WorldVersion::Exact(m, i, p, b) => f.write_fmt(format_args!("{}.{}.{}.{}", m, i, p, b)),
            WorldVersion::All => f.write_str("*"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum LoginVersion {
    Specific(u8),
    All,
}

impl Display for LoginVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginVersion::Specific(v) => f.write_fmt(format_args!("{}", v)),
            LoginVersion::All => f.write_str("*"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tags {
    inner: Vec<Tag>,
    login_logon_versions: Vec<LoginVersion>,
    world_versions: Vec<WorldVersion>,
}

impl Tags {
    pub fn new() -> Self {
        Self {
            inner: vec![],
            login_logon_versions: vec![],
            world_versions: vec![],
        }
    }

    pub fn push(&mut self, t: Tag) {
        self.append_or_insert(t.key(), t.value());
    }

    pub fn append(&mut self, t: &Tags) {
        for tag in &t.inner {
            self.append_or_insert(tag.key(), tag.value());
        }
    }

    pub fn append_or_insert(&mut self, key: &str, value: &str) {
        if key == LOGIN_LOGON_VERSIONS {
            for w in value.split_whitespace() {
                if let Ok(v) = w.parse::<u8>() {
                    self.login_logon_versions.push(LoginVersion::Specific(v));
                } else if w == "*" {
                    self.login_logon_versions.push(LoginVersion::All);
                } else {
                    panic!("invalid value passed as login_logon_versions: '{}'", w);
                }
            }
            self.login_logon_versions.sort();
            self.login_logon_versions.dedup();
            if self.login_logon_versions.contains(&LoginVersion::All) {
                self.login_logon_versions = vec![LoginVersion::All];
            }

            //return;
        } else if key == VERSIONS {
            for w in value.split_whitespace() {
                if let Ok(v) = w.parse::<u8>() {
                    self.world_versions.push(WorldVersion::Major(v));
                    continue;
                } else if w == "*" {
                    self.world_versions.push(WorldVersion::All);
                    continue;
                }

                let d: Vec<u8> = w.split('.').map(|a| a.parse::<u8>().unwrap()).collect();
                self.world_versions.push(match d.len() {
                    2 => WorldVersion::Minor(d[0], d[1]),
                    3 => WorldVersion::Patch(d[0], d[1], d[2]),
                    4 => WorldVersion::Exact(d[0], d[1], d[2], d[3]),
                    _ => panic!("incorrect world version string"),
                });
            }
            self.world_versions.sort();
            self.world_versions.dedup();
            if self.world_versions.contains(&WorldVersion::All) {
                self.world_versions = vec![WorldVersion::All];
            }
        }
        for v in self.inner.iter_mut() {
            if v.key == key {
                v.value += " ";
                v.value += value;
                return;
            }
        }

        self.inner.push(Tag::new(key, value));
    }

    pub fn contains(&self, name: &str) -> bool {
        self.inner.iter().find(|a| a.key == name).is_some()
    }

    pub fn get_ref(&self, name: &str) -> Option<&str> {
        match self.inner.iter().find(|a| a.key == name) {
            None => None,
            Some(v) => Some(v.value()),
        }
    }

    pub fn has_all_versions(&self, tags: &Tags) -> bool {
        // if self has all versions of tags
        if tags.contains(TEST_STR) && self.contains(TEST_STR) {
            return true;
        }

        for t in tags.logon_versions() {
            match self.logon_versions().contains(t)
                || self.logon_versions().contains(&LoginVersion::All)
            {
                true => {}
                false => {
                    return false;
                }
            }
        }

        for t in tags.versions() {
            match self.versions().contains(t) || self.versions().contains(&WorldVersion::All) {
                true => {}
                false => {
                    return false;
                }
            }
        }

        true
    }

    pub fn has_version_intersections(&self, tags: &Tags) -> bool {
        if tags.contains(TEST_STR) && self.contains(TEST_STR) {
            return true;
        }

        for t in &self.login_logon_versions {
            match tags.login_logon_versions.contains(t)
                || self.login_logon_versions.contains(&LoginVersion::All)
            {
                true => return true,
                false => {}
            }
        }

        for t in self.versions() {
            match tags.versions().contains(t) || self.versions().contains(&WorldVersion::All) {
                true => return true,
                false => {}
            }
        }

        false
    }

    pub fn has_version_all(&self) -> bool {
        self.login_logon_versions.contains(&LoginVersion::All)
            || self.world_versions.contains(&WorldVersion::All)
    }

    pub fn has_wildcard_logon_version(&self) -> bool {
        self.logon_versions().contains(&LoginVersion::All)
    }

    pub fn logon_versions(&self) -> &[LoginVersion] {
        &self.login_logon_versions
    }

    pub fn versions(&self) -> &[WorldVersion] {
        &self.world_versions
    }

    pub fn has_login_version(&self) -> bool {
        if !self.login_logon_versions.is_empty() {
            assert!(self.world_versions.is_empty());
            return true;
        } else if !self.world_versions.is_empty() {
            assert!(self.login_logon_versions.is_empty());
            return false;
        }

        false
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tag {
    key: String,
    value: String,
}

impl Tag {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_owned(),
            value: value.to_owned(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VerifiedContainerValue {
    value: u64,
    original_string: String,
}

impl Display for VerifiedContainerValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{original_string} ({value})",
            original_string = self.original_string,
            value = self.value
        )
    }
}

impl VerifiedContainerValue {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn original_string(&self) -> &str {
        &self.original_string
    }

    pub fn new(value: u64, original_string: String) -> Self {
        Self {
            value,
            original_string,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ContainerValue {
    identifier: String,
}

impl ContainerValue {
    pub fn identifier(&self) -> &str {
        &self.identifier
    }
}

impl Display for ContainerValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.identifier)
    }
}

impl From<&str> for ContainerValue {
    fn from(s: &str) -> Self {
        Self {
            identifier: s.to_string(),
        }
    }
}
