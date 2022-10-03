use crate::file_info::FileInfo;
use crate::file_utils::get_import_path;
use crate::parser::types::if_statement::{DefinerUsage, Equation, IfStatement};
use crate::parser::types::objects::{Object, Objects};
use crate::parser::types::sizes::{Sizes, BOOL_SIZE, DATETIME_SIZE};
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::tags::{LoginVersion, Tags, WorldVersion};
use crate::parser::types::test_case::TestCase;
use crate::parser::types::ty::Type;
use crate::parser::types::{ArraySize, ArrayType, ObjectType};
use crate::rust_printer::rust_view::RustObject;
use crate::rust_printer::{
    DefinerType, Version, LOGIN_CLIENT_MESSAGE_ENUM_NAME, LOGIN_SERVER_MESSAGE_ENUM_NAME,
    WORLD_CLIENT_MESSAGE_ENUM_NAME, WORLD_SERVER_MESSAGE_ENUM_NAME,
};
use crate::CONTAINER_SELF_SIZE_FIELD;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum ContainerType {
    Struct,
    CLogin(u16),
    SLogin(u16),
    Msg(u16),
    CMsg(u16),
    SMsg(u16),
}

impl ContainerType {
    pub fn str(&self) -> &str {
        match self {
            ContainerType::Struct => "struct",
            ContainerType::CLogin(_) => "clogin",
            ContainerType::SLogin(_) => "slogin",
            ContainerType::Msg(_) => "msg",
            ContainerType::CMsg(_) => "cmsg",
            ContainerType::SMsg(_) => "smsg",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Container {
    name: String,
    object_type: ContainerType,
    sizes: Sizes,
    members: Vec<StructMember>,
    tags: Tags,
    tests: Vec<TestCase>,
    file_info: FileInfo,
    only_has_io_error: bool,
    rust_object_view: Option<RustObject>,
}

impl PartialEq<Self> for Container {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.tags().first_and_main_versions() == other.tags().first_and_main_versions()
    }
}

impl Eq for Container {}

impl PartialOrd for Container {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Container {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_first = self.tags().first_version();
        let other_first = other.tags().first_version();

        self.name
            .cmp(&other.name)
            .then_with(|| self_first.cmp(&other_first))
    }
}

impl Container {
    pub fn new(
        name: String,
        members: Vec<StructMember>,
        tags: Tags,
        object_type: ContainerType,
        file_info: FileInfo,
        tests: Vec<TestCase>,
        sizes: Sizes,
        only_has_io_error: bool,
    ) -> Self {
        let s = Self {
            name,
            object_type,
            sizes,
            members,
            tags,
            tests,
            file_info,
            only_has_io_error,
            rust_object_view: None,
        };

        s.self_check();

        s
    }

    pub fn get_variable_name_of_definer_ty(&self, ty_name: &str) -> Option<String> {
        for d in self.all_definitions() {
            if let Type::Identifier { s, .. } = d.ty() {
                if s == ty_name {
                    return Some(d.name().to_string());
                }
            }
        }

        None
    }

    pub fn empty_body(&self) -> bool {
        self.members.is_empty()
    }

    pub fn is_pure_movement_info(&self) -> bool {
        let only_one_field = self.fields().len() == 1;
        if only_one_field {
            let field = self.fields().first().unwrap();
            match field {
                StructMember::Definition(d) => {
                    d.name() == "info"
                        && match d.ty() {
                            Type::Identifier { s, .. } => s == "MovementInfo",
                            _ => false,
                        }
                }
                _ => false,
            }
        } else {
            false
        }
    }

    pub fn contains_definer(&self, ty_name: &str) -> DefinerUsage {
        fn inner(m: &StructMember, ty_name: &str, variable_name: &str) -> DefinerUsage {
            match m {
                StructMember::Definition(d) => {
                    if let Type::Identifier { s, .. } = d.ty() {
                        if s == ty_name {
                            return DefinerUsage::NotInIf;
                        }
                    }
                }
                StructMember::IfStatement(statement) => {
                    if statement.name() == variable_name {
                        return DefinerUsage::InIf;
                    }

                    let mut not_in_if = false;
                    for m in statement.all_members() {
                        match inner(m, ty_name, variable_name) {
                            DefinerUsage::Unused => {}
                            DefinerUsage::NotInIf => not_in_if = true,
                            DefinerUsage::InIf => return DefinerUsage::InIf,
                        }
                    }

                    if not_in_if {
                        return DefinerUsage::NotInIf;
                    }
                }
                StructMember::OptionalStatement(optional) => {
                    let mut not_in_if = false;

                    for m in optional.members() {
                        match inner(m, ty_name, variable_name) {
                            DefinerUsage::Unused => {}
                            DefinerUsage::NotInIf => not_in_if = true,
                            DefinerUsage::InIf => return DefinerUsage::InIf,
                        }
                    }

                    if not_in_if {
                        return DefinerUsage::NotInIf;
                    }
                }
            }

            DefinerUsage::Unused
        }

        let variable_name = self.get_variable_name_of_definer_ty(ty_name);

        if let Some(variable_name) = variable_name {
            let mut not_in_if = false;

            for m in self.fields() {
                match inner(m, ty_name, &variable_name) {
                    DefinerUsage::Unused => {}
                    DefinerUsage::NotInIf => not_in_if = true,
                    DefinerUsage::InIf => return DefinerUsage::InIf,
                }
            }

            if not_in_if {
                return DefinerUsage::NotInIf;
            }
        }

        DefinerUsage::Unused
    }

    pub fn set_rust_object(&mut self, object: RustObject) {
        self.rust_object_view = Some(object);
    }

    pub fn type_definition_in_same_scope(&self, variable_name: &str) -> bool {
        fn inner(
            m: &StructMember,
            ty_scope: &mut u32,
            variable_name: &str,
            variable_scope: &mut u32,
            ty: &str,
            current_scope: &mut u32,
        ) {
            match m {
                StructMember::Definition(d) => {
                    if d.ty().rust_str() == ty {
                        *ty_scope = *current_scope;
                    }
                }
                StructMember::IfStatement(statement) => {
                    if statement.name() == variable_name {
                        *variable_scope = *current_scope;
                    }

                    *current_scope += 1;
                    for m in statement.all_members() {
                        inner(
                            m,
                            ty_scope,
                            variable_name,
                            variable_scope,
                            ty,
                            current_scope,
                        );
                    }
                    *current_scope -= 1;
                }
                StructMember::OptionalStatement(optional) => {
                    *current_scope += 1;
                    for m in optional.members() {
                        inner(
                            m,
                            ty_scope,
                            variable_name,
                            variable_scope,
                            ty,
                            current_scope,
                        );
                    }
                    *current_scope -= 1;
                }
            }
        }

        let ty = self.get_type_of_variable(variable_name);

        let mut ty_scope = 0;
        let mut variable_scope = 0;
        let mut current_scope = 1;

        for m in &self.members {
            inner(
                m,
                &mut ty_scope,
                variable_name,
                &mut variable_scope,
                &ty.rust_str(),
                &mut current_scope,
            );
        }

        ty_scope == variable_scope
    }

    pub fn is_constant_sized(&self) -> bool {
        self.sizes.is_constant().is_some()
    }

    pub fn only_has_io_errors(&self) -> bool {
        self.only_has_io_error
    }

    pub fn get_opcode_import_path(&self, version: Version) -> String {
        // `all` doesn't have an opcodes.rs
        let import_path = match version {
            Version::Login(f) => match f {
                LoginVersion::Specific(_) => get_import_path(version),
                LoginVersion::All => get_import_path(Version::Login(LoginVersion::Specific(3))),
            },
            Version::World(_) => get_import_path(version),
        };

        match self.container_type() {
            ContainerType::CLogin(_) => {
                format!(
                    "{}::opcodes::{}",
                    import_path, LOGIN_CLIENT_MESSAGE_ENUM_NAME
                )
            }
            ContainerType::SLogin(_) => {
                format!(
                    "{}::opcodes::{}",
                    import_path, LOGIN_SERVER_MESSAGE_ENUM_NAME
                )
            }
            ContainerType::SMsg(_) => {
                format!(
                    "{}::opcodes::{}",
                    import_path, WORLD_SERVER_MESSAGE_ENUM_NAME
                )
            }
            ContainerType::CMsg(_) => {
                format!(
                    "{}::opcodes::{}",
                    import_path, WORLD_CLIENT_MESSAGE_ENUM_NAME
                )
            }
            _ => unimplemented!(),
        }
    }

    pub fn get_field_ty(&self, field_name: &str) -> &Type {
        for d in self.all_definitions() {
            if d.name() == field_name {
                return d.ty();
            }
        }

        panic!("unable to find field: '{}'", field_name)
    }

    pub fn has_overlapping_version(&self, tags: &Tags) -> bool {
        for o in self.tags().logon_versions() {
            for i in tags.logon_versions() {
                if i == o || *i == LoginVersion::All || *o == LoginVersion::All {
                    return true;
                }
            }
        }

        for o in self.tags().versions() {
            for i in tags.versions() {
                if i == o || *i == WorldVersion::All || *o == WorldVersion::All {
                    return true;
                }
            }
        }

        false
    }

    pub fn any_fields_have_constant_value(&self) -> bool {
        for d in self.all_definitions() {
            if d.verified_value().is_some() {
                return true;
            }
        }

        false
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags().contains(tag)
    }

    pub fn container_type(&self) -> ContainerType {
        self.object_type
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn file_info(&self) -> &FileInfo {
        &self.file_info
    }

    pub fn rust_object(&self) -> &RustObject {
        self.rust_object_view.as_ref().unwrap()
    }

    pub fn size_of_fields_before_size(&self, o: &Objects) -> u64 {
        let mut sum = match self.object_type {
            ContainerType::Struct => 0,
            ContainerType::CLogin(_) | ContainerType::SLogin(_) => 0,
            _ => panic!(),
        };
        for field in self.fields() {
            match field {
                StructMember::Definition(d) => {
                    match d.ty() {
                        Type::Integer(int_type) => {
                            sum += int_type.size() as u64;
                        }
                        Type::FloatingPoint(float_ty) => {
                            sum += float_ty.size() as u64;
                        }
                        Type::CString => panic!("string before size"),
                        Type::String { .. } => panic!("string before size"),
                        Type::Array(_) => panic!("array before size"),
                        Type::Identifier { s: identifier, .. } => {
                            sum += o.get_size_of_complex(identifier);
                        }
                        Type::PackedGuid => panic!("packed guid before size"),
                        Type::Guid => sum += 8_u64,
                        Type::UpdateMask => panic!(),
                        Type::AuraMask => panic!(),
                        Type::SizedCString => panic!(),
                        Type::Bool => sum += BOOL_SIZE as u64,
                        Type::DateTime => sum += DATETIME_SIZE as u64,
                    }
                    if let Some(v) = d.verified_value() {
                        if v.original_string() == CONTAINER_SELF_SIZE_FIELD {
                            return sum;
                        }
                    }
                }
                StructMember::IfStatement(_) => panic!("if statement before size"),
                StructMember::OptionalStatement(_) => panic!("optional statement before size"),
            }
        }

        sum
    }

    pub fn panic_on_duplicate_field_names(&self) {
        let mut v = Vec::new();

        for d in self.all_definitions() {
            v.push(d.name());
        }
        v.sort_unstable();

        let mut previous_name = "";
        for e in v {
            if e == previous_name {
                panic!(
                    "struct '{struct_name}' contains duplicate fields '{field_name}'",
                    struct_name = self.name(),
                    field_name = e
                );
            }
            previous_name = e;
        }
    }

    pub fn check_if_statement_operators(&self, o: &Objects) {
        fn inner(m: &StructMember, e: &Container, o: &Objects) {
            match m {
                StructMember::IfStatement(statement) => {
                    let ty = match e.get_field_ty(statement.name()) {
                        Type::Identifier { s, .. } => s,
                        _ => unreachable!(),
                    };

                    let definer = o.get_definer(ty, e.tags());
                    match definer.definer_ty() {
                        DefinerType::Enum => {
                            for c in statement.get_conditional().equations() {
                                match c {
                                    Equation::Equals { .. } | Equation::NotEquals { .. } => {}
                                    Equation::BitwiseAnd { .. } => {
                                        panic!("enum has bitwise and")
                                    }
                                }
                            }
                        }
                        DefinerType::Flag => {
                            for c in statement.get_conditional().equations() {
                                match c {
                                    Equation::Equals { .. } | Equation::NotEquals { .. } => {
                                        panic!("flag has equals or not equals")
                                    }
                                    Equation::BitwiseAnd { .. } => {}
                                }
                            }
                        }
                    }

                    for m in statement.all_members() {
                        inner(m, e, o);
                    }
                }
                StructMember::OptionalStatement(optional) => {
                    for m in optional.members() {
                        inner(m, e, o);
                    }
                }
                StructMember::Definition(_) => {}
            }
        }

        for m in self.fields() {
            inner(m, self, o);
        }
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }

    pub fn get_complex_sizes(statement: &IfStatement, e: &Container, o: &Objects) -> Sizes {
        let mut if_sizes = Sizes::new();

        for m in statement.members() {
            Container::add_sizes_values(e, m, o, &mut if_sizes);
        }

        let mut smallest_sizes = if_sizes;
        let mut largest_sizes = if_sizes;

        let mut else_if_sizes;

        for elseif in statement.else_ifs() {
            else_if_sizes = Sizes::new();

            for m in elseif.members() {
                Container::add_sizes_values(e, m, o, &mut else_if_sizes);
            }

            if else_if_sizes.minimum() < smallest_sizes.minimum() {
                smallest_sizes = else_if_sizes;
            }
            if else_if_sizes.maximum() > largest_sizes.maximum() {
                largest_sizes = else_if_sizes;
            }
        }

        else_if_sizes = Sizes::new();
        for m in statement.else_members() {
            Container::add_sizes_values(e, m, o, &mut else_if_sizes);
        }

        if else_if_sizes.minimum() < smallest_sizes.minimum() {
            smallest_sizes = else_if_sizes;
        }
        if else_if_sizes.maximum() > largest_sizes.maximum() {
            largest_sizes = else_if_sizes;
        }

        let mut sizes = Sizes::new();
        sizes.set_minimum(smallest_sizes.minimum());
        sizes.set_maximum(largest_sizes.maximum());
        sizes
    }

    fn add_sizes_values(e: &Container, m: &StructMember, o: &Objects, sizes: &mut Sizes) {
        match m {
            StructMember::Definition(d) => *sizes += d.ty().sizes(e, o),
            StructMember::OptionalStatement(optional) => {
                let minimum = sizes.minimum();

                for m in optional.members() {
                    Container::add_sizes_values(e, m, o, sizes);
                }

                // The optional statement doesn't have be be here, so the minimum doesn't get incremented
                sizes.set_minimum(minimum);
            }
            StructMember::IfStatement(statement) => {
                let statement_sizes = Container::get_complex_sizes(statement, e, o);

                *sizes += statement_sizes;
            }
        }
    }

    pub fn sizes(&self) -> Sizes {
        self.sizes
    }

    pub fn all_definitions_transitively(&self, o: &Objects) -> Vec<StructMemberDefinition> {
        fn inner(m: &StructMember, v: &mut Vec<StructMemberDefinition>, o: &Objects, tags: &Tags) {
            match m {
                StructMember::Definition(d) => {
                    v.push(d.clone());
                    match d.ty() {
                        Type::Identifier { s, .. } => {
                            if let Object::Container(e) = o.get_object(s, tags) {
                                for m in e.fields() {
                                    inner(m, v, o, tags);
                                }
                            }
                        }
                        Type::Array(array) => {
                            if let ArrayType::Complex(s) = array.ty() {
                                if let Object::Container(e) = o.get_object(s, tags) {
                                    for m in e.fields() {
                                        inner(m, v, o, tags);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                StructMember::IfStatement(statement) => {
                    for m in statement.all_members() {
                        inner(m, v, o, tags);
                    }
                }
                StructMember::OptionalStatement(optional) => {
                    for m in optional.members() {
                        inner(m, v, o, tags);
                    }
                }
            }
        }

        let mut v = Vec::new();

        for m in self.fields() {
            inner(m, &mut v, o, self.tags());
        }

        v
    }

    pub fn all_definitions_mut(&mut self) -> Vec<&mut StructMemberDefinition> {
        fn inner<'a>(m: &'a mut StructMember, v: &mut Vec<&'a mut StructMemberDefinition>) {
            match m {
                StructMember::Definition(d) => v.push(d),
                StructMember::IfStatement(statement) => {
                    for m in statement.all_members_mut() {
                        inner(m, v);
                    }
                }
                StructMember::OptionalStatement(optional) => {
                    for m in optional.members_mut() {
                        inner(m, v);
                    }
                }
            }
        }

        let mut v = Vec::new();

        for m in self.fields_mut() {
            inner(m, &mut v);
        }

        v
    }

    pub fn all_definitions(&self) -> Vec<&StructMemberDefinition> {
        fn inner<'a>(m: &'a StructMember, v: &mut Vec<&'a StructMemberDefinition>) {
            match m {
                StructMember::Definition(d) => v.push(d),
                StructMember::IfStatement(statement) => {
                    for m in statement.all_members() {
                        inner(m, v);
                    }
                }
                StructMember::OptionalStatement(optional) => {
                    for m in optional.members() {
                        inner(m, v);
                    }
                }
            }
        }

        let mut v = Vec::new();

        for m in self.fields() {
            inner(m, &mut v);
        }

        v
    }

    pub fn contains_enum(&self, o: &Objects, tags: &Tags) -> bool {
        for d in self.all_definitions() {
            if let Type::Identifier { s, .. } = d.ty() {
                if o.get_object_type_of(s, tags) == ObjectType::Enum {
                    return true;
                }
            }
        }

        false
    }

    pub fn contains_update_mask(&self) -> bool {
        for d in self.all_definitions() {
            if d.ty() == &Type::UpdateMask {
                return true;
            }
        }

        false
    }

    pub fn contains_update_mask_transitively(&self, o: &Objects) -> bool {
        for d in self.all_definitions_transitively(o) {
            if d.ty() == &Type::UpdateMask {
                return true;
            }
        }

        false
    }

    pub fn contains_aura_mask(&self) -> bool {
        for d in self.all_definitions() {
            if d.ty() == &Type::AuraMask {
                return true;
            }
        }

        false
    }

    pub fn contains_aura_mask_transitively(&self, o: &Objects) -> bool {
        for d in self.all_definitions_transitively(o) {
            if d.ty() == &Type::AuraMask {
                return true;
            }
        }

        false
    }

    pub fn contains_datetime(&self) -> bool {
        for d in self.all_definitions() {
            if d.ty() == &Type::DateTime {
                return true;
            }
        }

        false
    }

    pub fn contains_guid_or_packed_guid(&self) -> bool {
        for d in self.all_definitions() {
            match d.ty() {
                Type::PackedGuid | Type::Guid => return true,
                Type::Array(array) => match array.ty() {
                    ArrayType::Guid | ArrayType::PackedGuid => return true,
                    _ => {}
                },
                _ => {}
            }
        }

        false
    }

    pub fn contains_guid_or_packed_guid_transitively(&self, o: &Objects) -> bool {
        for d in self.all_definitions_transitively(o) {
            match d.ty() {
                Type::PackedGuid | Type::Guid => return true,
                Type::Array(array) => match array.ty() {
                    ArrayType::Guid | ArrayType::PackedGuid => return true,
                    _ => {}
                },
                _ => {}
            }
        }

        false
    }

    pub fn get_types_needing_import_recursively<'a>(&'a self, o: &'a Objects) -> Vec<String> {
        let mut v = self.get_complex_types();

        let mut v2 = Vec::new();
        for t in &v {
            match o.get_object_type_of(t, self.tags()) {
                ObjectType::Struct | ObjectType::CLogin | ObjectType::SLogin => {
                    let mut types = o
                        .get_container(t, self.tags())
                        .get_types_needing_import_recursively(o);
                    v2.append(&mut types);
                }
                ObjectType::Enum | ObjectType::Flag => {}
            }
        }
        v.append(&mut v2);

        v.sort_unstable();
        v.dedup();

        v
    }

    pub fn get_types_needing_import(&self) -> Vec<String> {
        self.get_complex_types()
    }

    fn get_complex_types(&self) -> Vec<String> {
        let mut v = Vec::new();

        for d in self.all_definitions() {
            match d.struct_type() {
                Type::Array(a) => {
                    if let ArrayType::Complex(i) = a.ty() {
                        v.push(i.clone());
                    }
                }
                Type::Identifier { s, .. } => {
                    v.push(s.clone());
                }
                _ => {}
            }
        }

        v.sort_unstable();
        v.dedup();
        v
    }

    pub fn get_type_of_variable(&self, variable_name: &str) -> Type {
        for d in self.all_definitions() {
            if d.name() == variable_name {
                return d.ty().clone();
            }
        }

        panic!("unable to find type {}", variable_name)
    }

    pub fn tests(&self) -> &[TestCase] {
        &self.tests
    }

    pub fn fields(&self) -> &[StructMember] {
        self.members.as_slice()
    }

    fn set_all_values_to_verified(&mut self, o: &Objects) {
        fn set_values_to_verified(m: &mut StructMember, o: &Objects) {
            match m {
                StructMember::Definition(d) => d.set_verified_value(o),
                StructMember::IfStatement(statement) => {
                    for m in statement.all_members_mut() {
                        set_values_to_verified(m, o);
                    }
                }
                StructMember::OptionalStatement(optional) => {
                    for m in optional.members_mut() {
                        set_values_to_verified(m, o);
                    }
                }
            }
        }

        for m in self.fields_mut() {
            set_values_to_verified(m, o);
        }
    }

    fn find_definition_internal<'a>(
        m: &'a mut StructMember,
        name: &str,
    ) -> Option<&'a mut StructMemberDefinition> {
        match m {
            StructMember::Definition(d) => {
                if d.name() == name {
                    return Some(d);
                }
            }
            StructMember::IfStatement(statement) => {
                for m in statement.all_members_mut() {
                    if let Some(d) = Self::find_definition_internal(m, name) {
                        return Some(d);
                    }
                }
            }
            StructMember::OptionalStatement(optional) => {
                for m in optional.members_mut() {
                    if let Some(d) = Self::find_definition_internal(m, name) {
                        return Some(d);
                    }
                }
            }
        }

        None
    }
    pub fn find_definition_with_name_mut(
        &mut self,
        name: &str,
    ) -> Option<&mut StructMemberDefinition> {
        for m in self.fields_mut() {
            if let Some(d) = Self::find_definition_internal(m, name) {
                return Some(d);
            }
        }

        None
    }

    fn set_value_used_as_sizes(&mut self, m: &StructMember) {
        match m {
            StructMember::Definition(d) => match d.ty() {
                Type::Integer(_) | Type::FloatingPoint(_) | Type::DateTime | Type::CString => {}
                Type::String { length } => {
                    if length.parse::<u8>().is_ok() {
                        return;
                    }
                    if let Some(n) = self.find_definition_with_name_mut(length) {
                        n.used_as_size_in = Some(d.name().to_string());
                    } else {
                        panic!(
                            "name used as array length that doesn't exist: '{}', in member: '{}'",
                            length,
                            d.name(),
                        )
                    }
                }
                Type::Array(array) => match array.size() {
                    ArraySize::Fixed(_) => {}
                    ArraySize::Variable(variable) => {
                        if let Some(n) = self.find_definition_with_name_mut(&variable) {
                            n.used_as_size_in = Some(d.name().to_string());
                        } else {
                            panic!(
                                "name used as array length that doesn't exist: '{}', in member: '{}'",
                                variable,
                                d.name(),
                            )
                        }
                    }
                    ArraySize::Endless => {}
                },
                Type::Identifier { .. } => {}
                Type::PackedGuid => {}
                Type::Guid => {}
                Type::UpdateMask => {}
                Type::AuraMask => {}
                Type::SizedCString => {}
                Type::Bool => {}
            },
            StructMember::IfStatement(statement) => {
                for m in statement.all_members() {
                    self.set_value_used_as_sizes(m);
                }
            }
            StructMember::OptionalStatement(optional) => {
                for m in optional.members() {
                    self.set_value_used_as_sizes(m);
                }
            }
        }
    }

    fn set_if_statements(&mut self, o: &Objects) {
        fn inner(m: &mut StructMember, c: &Container, o: &Objects) {
            match m {
                StructMember::Definition(_) => {}
                StructMember::IfStatement(statement) => {
                    statement.set_original_ty(c.get_type_of_variable(statement.name()));

                    for else_if in statement.else_ifs_mut() {
                        else_if.set_original_ty(c.get_type_of_variable(else_if.name()));
                    }

                    for m in statement.all_members_mut() {
                        inner(m, c, o);
                    }
                }
                StructMember::OptionalStatement(optional) => {
                    for m in optional.members_mut() {
                        inner(m, c, o);
                    }
                }
            }
        }

        let c = self.clone();
        for m in &mut self.members {
            inner(m, &c, o);
        }
    }

    pub fn set_used_in_if(&mut self) {
        let mut variables_used_in_if = Vec::new();

        fn find_used_in_if(m: &StructMember, variables_used_in_if: &mut Vec<String>) {
            match m {
                StructMember::Definition(_) => {}
                StructMember::IfStatement(statement) => {
                    variables_used_in_if.push(statement.name().to_string());

                    for m in statement.all_members() {
                        find_used_in_if(m, variables_used_in_if);
                    }
                }
                StructMember::OptionalStatement(optional) => {
                    for m in optional.members() {
                        find_used_in_if(m, variables_used_in_if);
                    }
                }
            }
        }

        for m in self.fields() {
            find_used_in_if(m, &mut variables_used_in_if);
        }

        for d in self.all_definitions_mut() {
            d.set_used_in_if(variables_used_in_if.contains(&d.name().to_string()));
        }
    }

    fn check_values(&self, o: &Objects) {
        for d in self.all_definitions() {
            match &d.ty() {
                Type::Array(a) => {
                    if let ArrayType::Complex(c) = &a.ty() {
                        o.contains_complex_type(c, self.tags(), d.name())
                    }
                }
                Type::Identifier { s: i, .. } => {
                    o.contains_complex_type(i, self.tags(), d.name());

                    match d.value() {
                        None => {}
                        Some(v) => match v.identifier().parse::<usize>() {
                            Ok(_) => {}
                            Err(_) => {
                                o.contains_value_in_type(i, v.identifier());
                            }
                        },
                    }
                }
                _ => {}
            }
        }
    }

    pub fn self_check(&self) {
        self.panic_on_duplicate_field_names();
    }

    pub fn set_internals(&mut self, o: &Objects) {
        self.check_values(o);

        self.set_used_in_if();

        self.set_if_statements(o);

        self.set_all_values_to_verified(o);

        for m in &self.members.clone() {
            self.set_value_used_as_sizes(m);
        }
    }

    pub fn fields_mut(&mut self) -> &mut [StructMember] {
        &mut self.members
    }
}
