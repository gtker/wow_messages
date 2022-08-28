use crate::file_info::FileInfo;
use crate::file_utils::get_import_path;
use crate::parser::types::objects::Objects;
use crate::parser::types::tags::{LoginVersion, Tag, Tags, WorldVersion};
use crate::parser::types::ty::Type;
use crate::parser::types::{
    ArraySize, ArrayType, ContainerValue, ObjectType, VerifiedContainerValue,
};
use crate::rust_printer::rust_view::RustObject;
use crate::rust_printer::{
    field_name_to_rust_name, DefinerType, LOGIN_CLIENT_MESSAGE_ENUM_NAME,
    LOGIN_SERVER_MESSAGE_ENUM_NAME, WORLD_CLIENT_MESSAGE_ENUM_NAME, WORLD_SERVER_MESSAGE_ENUM_NAME,
};
use crate::test_case::TestCase;
use crate::{CONTAINER_SELF_SIZE_FIELD, LOGIN_VERSIONS};
use std::ops::AddAssign;

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
    sizes: Option<Sizes>,
    members: Vec<StructMember>,
    tags: Tags,
    tests: Vec<TestCase>,
    file_info: FileInfo,
    only_has_io_error: Option<bool>,
    rust_object_view: Option<RustObject>,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum DefinerUsage {
    Unused,
    NotInIf,
    InIf,
}

impl Container {
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

    pub fn append_tests(&mut self, mut t: Vec<TestCase>) {
        self.tests.append(&mut t);
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
                    for m in &optional.members {
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
        self.sizes.unwrap().is_constant()
    }

    pub fn only_has_io_errors(&self) -> bool {
        self.only_has_io_error.unwrap()
    }

    pub fn recursive_only_has_io_errors(&self, o: &Objects) -> bool {
        if self.contains_string_or_cstring() {
            return false;
        }

        for t in &self.get_types_needing_import() {
            match o.get_object_type_of(t, self.tags()) {
                ObjectType::Flag => {}
                ObjectType::Struct => {
                    if !o.object_has_only_io_errors(t, self.tags()) {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            }
        }
        true
    }

    fn set_io_errors(&mut self, o: &Objects) {
        self.only_has_io_error = Some(self.recursive_only_has_io_errors(o));
    }

    pub fn get_opcode_import_path(&self) -> String {
        let import_path = if self.tags().logon_versions().contains(&LoginVersion::All) {
            let mut tags = Tags::new();
            tags.push(Tag::new(LOGIN_VERSIONS, "3"));
            get_import_path(&tags, None)
        } else {
            get_import_path(self.tags(), None)
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
                    }
                    if let Some(v) = &d.verified_value {
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
        for m in &statement.else_statement_members {
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
                let minimum = sizes.minimum;

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

    pub fn create_sizes(&self, o: &Objects) -> Sizes {
        let mut sizes = Sizes::new();
        for m in self.fields() {
            Container::add_sizes_values(self, m, o, &mut sizes);
        }

        sizes
    }

    pub fn sizes(&self) -> Sizes {
        self.sizes.unwrap()
    }

    pub fn has_constant_size(&self, o: &Objects) -> bool {
        for m in self.fields() {
            match m {
                StructMember::Definition(d) => match d.ty() {
                    Type::Integer(_) => {}
                    Type::FloatingPoint(_) => {}
                    Type::CString => return false,
                    Type::String { length } => match length.parse::<u64>() {
                        Ok(_) => {}
                        Err(_) => return false,
                    },
                    Type::Array(array) => match array.size() {
                        ArraySize::Fixed(_) => match array.ty() {
                            ArrayType::Integer(_) => {}
                            ArrayType::Complex(identifier) => {
                                match o.type_has_constant_size(&Type::Identifier {
                                    s: identifier.clone(),
                                    upcast: None,
                                }) {
                                    true => {}
                                    false => return false,
                                }
                            }
                            ArrayType::CString => return false,
                            ArrayType::Guid => {}
                            ArrayType::PackedGuid => return false,
                        },
                        ArraySize::Variable(_) => return false,
                        ArraySize::Endless => return false,
                    },
                    Type::Identifier { s: identifier, .. } => {
                        match o.type_has_constant_size(&Type::Identifier {
                            s: identifier.clone(),
                            upcast: None,
                        }) {
                            true => {}
                            false => return false,
                        }
                    }
                    Type::PackedGuid => return false,
                    Type::Guid => {}
                    Type::UpdateMask | Type::AuraMask => return false,
                    Type::SizedCString => return false,
                },
                StructMember::IfStatement(_) => {
                    return false;
                }
                StructMember::OptionalStatement(_) => {
                    return false;
                }
            }
        }

        true
    }

    pub fn has_if_statements(&self) -> bool {
        for m in &self.members {
            match m {
                StructMember::Definition(_) => {}
                StructMember::IfStatement(_) => {
                    return true;
                }
                StructMember::OptionalStatement(_) => {}
            }
        }

        false
    }

    pub fn get_types_needing_errors(&self, o: &Objects, tags: &Tags) -> Vec<&str> {
        self.get_types_needing_import()
            .into_iter()
            .filter(|t| {
                !o.object_has_only_io_errors(t, tags)
                    && !matches!(
                        o.get_object_type_of(t, tags),
                        ObjectType::Flag | ObjectType::Enum
                    )
            })
            .collect()
    }

    pub fn contains_string_or_cstring(&self) -> bool {
        for d in self.all_definitions() {
            match d.ty() {
                Type::CString | Type::String { .. } => return true,
                Type::Array(array) => {
                    if matches!(array.ty(), ArrayType::CString) {
                        return true;
                    }
                }
                _ => {}
            }
        }

        false
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

    pub fn needs_enum_error(&self, o: &Objects, tags: &Tags) -> bool {
        for d in self.all_definitions() {
            if let Type::Identifier { s, .. } = d.ty() {
                if o.get_object_type_of(s, tags) == ObjectType::Enum {
                    match o.get_definer(s, tags).self_value().is_none() {
                        true => return true,
                        false => {}
                    }
                }
            }
        }

        false
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

    pub fn contains_aura_mask(&self) -> bool {
        for d in self.all_definitions() {
            if d.ty() == &Type::AuraMask {
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

    pub fn get_types_needing_import_recursively<'a>(&'a self, o: &'a Objects) -> Vec<&'a str> {
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

    pub fn get_types_needing_import(&self) -> Vec<&str> {
        self.get_complex_types()
    }

    fn get_complex_types(&self) -> Vec<&str> {
        let mut v = Vec::new();

        for d in self.all_definitions() {
            match &d.struct_type {
                Type::Array(a) => {
                    if let ArrayType::Complex(i) = a.ty() {
                        v.push(i.as_str());
                    }
                }
                Type::Identifier { s, .. } => {
                    v.push(s.as_str());
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

    pub fn new(
        name: &str,
        members: Vec<StructMember>,
        tags: Tags,
        object_type: ContainerType,
        file_info: FileInfo,
    ) -> Self {
        Self {
            name: name.to_string(),
            object_type,
            sizes: None,
            members,
            tags,
            tests: vec![],
            file_info,
            only_has_io_error: None,
            rust_object_view: None,
        }
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
                    for m in optional.members.iter_mut() {
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
                for m in &mut optional.members {
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
                Type::Integer(_) => {}
                Type::FloatingPoint(_) => {}
                Type::CString => {}
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
            },
            StructMember::IfStatement(statement) => {
                for m in &statement.members {
                    self.set_value_used_as_sizes(m);
                }
                for m in &statement.else_statement_members {
                    self.set_value_used_as_sizes(m);
                }
            }
            StructMember::OptionalStatement(optional) => {
                for m in &optional.members {
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

                    for else_if in &mut statement.else_ifs {
                        else_if.set_original_ty(c.get_type_of_variable(else_if.name()));
                    }

                    for m in statement.all_members_mut() {
                        inner(m, c, o);
                    }
                }
                StructMember::OptionalStatement(optional) => {
                    for m in &mut optional.members {
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
        fn inner(m: &mut StructMember) -> Option<&str> {
            match m {
                StructMember::Definition(_) => None,
                StructMember::IfStatement(statement) => {
                    let mut used_in_if = Vec::new();
                    for m in &mut statement.members {
                        if let Some(name) = inner(m) {
                            used_in_if.push(name.to_string());
                        }
                    }
                    for m in &mut statement.members {
                        set(m, &used_in_if);
                    }

                    let mut used_in_if = Vec::new();
                    for ifs in &mut statement.else_ifs {
                        for m in &mut ifs.members {
                            if let Some(name) = inner(m) {
                                used_in_if.push(name.to_string());
                            }
                        }
                        for m in &mut ifs.members {
                            set(m, &used_in_if);
                        }
                    }

                    let mut used_in_if = Vec::new();
                    for m in &mut statement.else_statement_members {
                        if let Some(name) = inner(m) {
                            used_in_if.push(name.to_string());
                        }
                    }
                    for m in &mut statement.else_statement_members {
                        set(m, &used_in_if);
                    }

                    Some(statement.name())
                }
                StructMember::OptionalStatement(optional) => {
                    let mut used_in_if = Vec::new();
                    for m in &mut optional.members {
                        if let Some(name) = inner(m) {
                            used_in_if.push(name.to_string());
                        }
                    }
                    for m in &mut optional.members {
                        set(m, &used_in_if);
                    }
                    None
                }
            }
        }
        let mut used_in_if = Vec::new();

        for m in self.fields_mut() {
            if let Some(name) = inner(m) {
                used_in_if.push(name.to_string());
            }
        }
        fn set(m: &mut StructMember, used_in_if: &[String]) {
            match m {
                StructMember::Definition(d) => {
                    d.set_used_in_if(used_in_if.contains(&d.name().to_string()));
                }
                StructMember::IfStatement(_) => {}
                StructMember::OptionalStatement(_) => {}
            }
        }
        for m in self.fields_mut() {
            set(m, &used_in_if);
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

    pub fn set_internals(&mut self, o: &Objects) {
        self.panic_on_duplicate_field_names();

        self.check_values(o);

        self.set_used_in_if();

        self.set_if_statements(o);

        self.set_io_errors(o);

        self.set_all_values_to_verified(o);

        for m in &self.members.clone() {
            self.set_value_used_as_sizes(m);
        }

        self.sizes = Some(self.create_sizes(o));
    }

    pub fn fields_mut(&mut self) -> &mut [StructMember] {
        &mut self.members
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Sizes {
    minimum: usize,
    maximum: usize,
}

pub const AURA_MASK_MAX_SIZE: u8 = 4 + 32 * 4;
pub const AURA_MASK_MIN_SIZE: u8 = 4;
const fn update_mask_max() -> u16 {
    let amount_of_bytes_for_data = 0x501; // PLAYER_END
    let amount_of_mask_blocks_size = core::mem::size_of::<u32>() as i32;

    let mut max_mask_blocks = amount_of_bytes_for_data / 8;
    if (amount_of_bytes_for_data % 8) > 0 {
        max_mask_blocks += 1;
    }

    (amount_of_mask_blocks_size + max_mask_blocks + amount_of_bytes_for_data) as u16
}
pub const UPDATE_MASK_MAX_SIZE: u16 = update_mask_max();
pub const UPDATE_MASK_MIN_SIZE: u8 = 1;
pub const PACKED_GUID_MAX_SIZE: u8 = 9;
pub const PACKED_GUID_MIN_SIZE: u8 = 2;
pub const GUID_SIZE: u8 = core::mem::size_of::<u64>() as u8;

impl Sizes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inc(&mut self, minimum: usize, maximum: usize) {
        self.minimum = self.minimum.saturating_add(minimum);
        self.maximum = self.maximum.saturating_add(maximum);
    }

    pub fn inc_both(&mut self, v: usize) {
        self.inc(v, v);
    }

    pub fn minimum(&self) -> usize {
        self.minimum
    }

    pub fn maximum(&self) -> usize {
        self.maximum
    }

    pub fn set_maximum(&mut self, maximum: usize) {
        self.maximum = maximum;
    }

    pub fn set_minimum(&mut self, minimum: usize) {
        self.minimum = minimum;
    }

    pub fn is_constant(&self) -> bool {
        self.minimum == self.maximum
    }
}

impl AddAssign for Sizes {
    fn add_assign(&mut self, rhs: Self) {
        self.inc(rhs.minimum, rhs.maximum);
    }
}

#[derive(Debug, Clone, Eq)]
pub struct OptionalStatement {
    name: String,
    members: Vec<StructMember>,
    tags: Tags,
}

impl PartialEq for OptionalStatement {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl OptionalStatement {
    pub fn new(name: &str, members: Vec<StructMember>) -> Self {
        Self {
            name: name.to_string(),
            members,
            tags: Tags::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn members(&self) -> &[StructMember] {
        &self.members
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StructMember {
    Definition(StructMemberDefinition),
    IfStatement(IfStatement),
    OptionalStatement(OptionalStatement),
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub conditional: Conditional,
    members: Vec<StructMember>,
    else_ifs: Vec<IfStatement>,
    else_statement_members: Vec<StructMember>,
    original_ty: Option<Type>,
}

impl Eq for IfStatement {}

impl PartialEq for IfStatement {
    fn eq(&self, other: &Self) -> bool {
        self.members.first().unwrap() == other.members.first().unwrap()
    }
}

impl IfStatement {
    pub fn new(
        conditional: Conditional,
        members: Vec<StructMember>,
        else_ifs: Vec<IfStatement>,
        else_statement_members: Vec<StructMember>,
    ) -> Self {
        Self {
            conditional,
            members,
            else_ifs,
            else_statement_members,
            original_ty: None,
        }
    }

    pub fn is_not_enum(&self) -> bool {
        matches!(self.conditional.equations[0], Equation::NotEquals { .. })
    }

    pub fn flag_get_enumerator(&self) -> String {
        assert_eq!(self.get_conditional().equations.len(), 1);

        match &self.get_conditional().equations[0] {
            Equation::BitwiseAnd { value } => value.to_string(),
            _ => unreachable!(),
        }
    }

    pub fn flag_get_enumerator_rust_name(&self) -> String {
        field_name_to_rust_name(&self.flag_get_enumerator())
    }

    pub fn is_elseif_flag(&self) -> bool {
        match self.conditional.equations[0] {
            Equation::BitwiseAnd { .. } => !self.else_ifs().is_empty(),
            _ => false,
        }
    }

    pub fn members(&self) -> &[StructMember] {
        &self.members
    }

    pub fn members_mut(&mut self) -> &mut [StructMember] {
        &mut self.members
    }

    pub fn member_enumerators(&self) -> Vec<&str> {
        let mut v = Vec::new();

        for eq in &self.conditional.equations {
            match eq {
                Equation::BitwiseAnd { value }
                | Equation::Equals { value }
                | Equation::NotEquals { value } => {
                    v.push(value.as_str());
                }
            }
        }

        v
    }

    pub fn else_members(&self) -> &[StructMember] {
        &self.else_statement_members
    }

    pub fn else_members_mut(&mut self) -> &mut [StructMember] {
        &mut self.else_statement_members
    }

    pub fn original_ty(&self) -> &Type {
        self.original_ty.as_ref().unwrap()
    }

    pub fn name(&self) -> &str {
        &self.conditional.variable_name
    }

    pub fn set_original_ty(&mut self, original_ty: Type) {
        self.original_ty = Some(original_ty)
    }

    pub fn definer_type(&self) -> DefinerType {
        match self.conditional.equations[0] {
            Equation::Equals { .. } => DefinerType::Enum,
            Equation::BitwiseAnd { .. } => DefinerType::Flag,
            Equation::NotEquals { .. } => DefinerType::Enum,
        }
    }

    pub fn else_ifs_mut(&mut self) -> &mut [IfStatement] {
        &mut self.else_ifs
    }

    pub fn else_ifs(&self) -> &[IfStatement] {
        &self.else_ifs
    }

    pub fn all_members_mut(&mut self) -> impl Iterator<Item = &mut StructMember> + '_ {
        self.members
            .iter_mut()
            .chain(self.else_ifs.iter_mut().flat_map(|a| a.members.iter_mut()))
            .chain(self.else_statement_members.iter_mut())
    }

    pub fn all_members(&self) -> impl Iterator<Item = &StructMember> {
        let else_ifs = self.else_ifs.iter().flat_map(|a| a.members());
        self.members()
            .iter()
            .chain(else_ifs)
            .chain(&self.else_statement_members)
    }

    pub fn get_conditional(&self) -> &Conditional {
        &self.conditional
    }

    pub fn get_variable_names_for_members(&self) -> Vec<&StructMemberDefinition> {
        let mut v = Vec::new();

        for m in &self.members {
            match m {
                StructMember::Definition(d) => {
                    v.push(d);
                }
                StructMember::IfStatement(_) => {}
                StructMember::OptionalStatement(_) => {}
            }
        }

        v
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StructMemberDefinition {
    name: String,
    struct_type: Type,
    value: Option<ContainerValue>,
    verified_value: Option<VerifiedContainerValue>,
    used_as_size_in: Option<String>,
    used_in_if: Option<bool>,
    tags: Tags,
}

impl StructMemberDefinition {
    pub fn used_as_size_in(&self) -> &Option<String> {
        &self.used_as_size_in
    }

    pub fn used_in_if(&self) -> bool {
        self.used_in_if.unwrap()
    }

    pub fn set_used_in_if(&mut self, used: bool) {
        self.used_in_if = Some(used);
    }

    pub fn new(name: &str, struct_type: Type, value: Option<ContainerValue>, tags: Tags) -> Self {
        Self {
            name: name.to_string(),
            struct_type,
            value,
            verified_value: None,
            used_as_size_in: None,
            used_in_if: None,
            tags,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> &Type {
        &self.struct_type
    }

    pub fn value(&self) -> &Option<ContainerValue> {
        &self.value
    }

    pub fn verified_value(&self) -> &Option<VerifiedContainerValue> {
        &self.verified_value
    }

    pub fn set_verified_value(&mut self, o: &Objects) {
        match &self.value() {
            None => {}
            Some(v) => {
                let parsed_val = crate::parser::utility::parse_value(v.identifier());
                if let Some(int_val) = parsed_val {
                    self.verified_value = Some(VerifiedContainerValue::new(
                        int_val,
                        v.identifier().to_string(),
                    ))
                } else {
                    let value = if v.identifier() != CONTAINER_SELF_SIZE_FIELD {
                        o.get_definer_field_value(&self.ty().rust_str(), v.identifier(), &self.tags)
                    } else {
                        0
                    };
                    self.verified_value = Some(VerifiedContainerValue::new(
                        value,
                        v.identifier().to_string(),
                    ));
                }
            }
        }
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }
}

#[derive(Debug, Clone)]
pub enum Operator {
    Equals,
    NotEquals,
    BitwiseAnd,
}

impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        match s {
            "&" => Operator::BitwiseAnd,
            "==" => Operator::Equals,
            "!=" => Operator::NotEquals,
            _ => panic!("invalid operator {}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Conditional {
    variable_name: String,
    equations: Vec<Equation>,
}

impl Conditional {
    pub fn variable_name(&self) -> &str {
        &self.variable_name
    }

    pub fn equations(&self) -> &[Equation] {
        &self.equations
    }

    pub fn new(conditions: &[Condition]) -> Self {
        let variable_name = conditions[0].value.to_string();

        let mut equations = Vec::new();
        for c in conditions {
            if c.value != variable_name {
                panic!(
                    "matching variable in if statement '||' is not the same, '{}' and '{}'",
                    variable_name, c.value
                );
            }

            let v = match c.operator {
                Operator::Equals => Equation::Equals {
                    value: c.equals_value.to_owned(),
                },
                Operator::BitwiseAnd => Equation::BitwiseAnd {
                    value: c.equals_value.to_owned(),
                },
                Operator::NotEquals => Equation::NotEquals {
                    value: c.equals_value.to_owned(),
                },
            };
            equations.push(v);
        }

        Self {
            variable_name,
            equations,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Equation {
    Equals { value: String },
    NotEquals { value: String },
    BitwiseAnd { value: String },
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub value: String,
    pub operator: Operator,
    pub equals_value: String,
}

impl Condition {
    pub fn new(value: &str, equals_value: &str, operator: Operator) -> Self {
        Self {
            value: value.to_string(),
            operator,
            equals_value: equals_value.to_string(),
        }
    }
}
