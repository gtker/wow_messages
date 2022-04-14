use crate::container::{Container, Equation, IfStatement, StructMember};
use crate::file_info::FileInfo;
use crate::parser::enumerator::DefinerValue;
use crate::parser::types::objects::Objects;
use crate::parser::types::tags::Tags;
use crate::parser::types::ty::Type;
use crate::parser::types::{
    Array, ArraySize, ArrayType, FloatingPointType, IntegerType, ObjectType,
};
use crate::test_case::TestCase;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct RustMember {
    name: String,
    ty: RustType,

    constant_sized: bool,

    tags: Tags,
}

impl RustMember {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn ty(&self) -> &RustType {
        &self.ty
    }
    pub fn tags(&self) -> &Tags {
        &self.tags
    }
    pub fn constant_sized(&self) -> bool {
        self.constant_sized
    }
}

#[derive(Debug, Clone)]
pub struct RustEnumerator {
    name: String,
    value: DefinerValue,
    members: Vec<RustMember>,
}

impl RustEnumerator {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn value(&self) -> &DefinerValue {
        &self.value
    }
    pub fn members(&self) -> &[RustMember] {
        &self.members
    }
}

#[derive(Debug, Clone)]
pub enum RustType {
    Integer(IntegerType),
    Floating(FloatingPointType),
    BuiltIn(String),
    String,
    Array(Array),
    Enum {
        ty_name: String,
        enumerators: Vec<RustEnumerator>,
        upcast: Option<IntegerType>,
        is_simple: bool,
    },
    Flag {
        ty_name: String,
        enumerators: Vec<RustEnumerator>,
        is_simple: bool,
    },
    Struct(String),
}

impl Display for RustType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RustType::Integer(i) => f.write_str(i.rust_str()),
            RustType::Floating(i) => f.write_str(i.rust_str()),
            RustType::BuiltIn(builtin) => f.write_str(builtin),
            RustType::String => f.write_str("String"),
            RustType::Array(array) => f.write_str(&array.rust_str()),
            RustType::Enum { ty_name, .. } | RustType::Flag { ty_name, .. } => f.write_str(ty_name),
            RustType::Struct(name) => f.write_str(name),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RustOptional {
    name: String,
    ty: String,
    members: Vec<RustMember>,
    constant_sized: bool,
}

impl RustOptional {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn ty(&self) -> &str {
        &self.ty
    }
    pub fn members(&self) -> &Vec<RustMember> {
        &self.members
    }
    pub fn constant_sized(&self) -> bool {
        self.constant_sized
    }
}

#[derive(Debug, Clone)]
pub struct RustObject {
    name: String,
    members: Vec<RustMember>,
    optional: Option<RustOptional>,
    constant_sized: bool,

    tests: Vec<TestCase>,

    tags: Tags,
    file_info: FileInfo,
}

impl RustObject {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn members(&self) -> &[RustMember] {
        &self.members
    }
    pub fn optional(&self) -> Option<&RustOptional> {
        self.optional.as_ref()
    }
    pub fn tests(&self) -> &[TestCase] {
        &self.tests
    }
    pub fn tags(&self) -> &Tags {
        &self.tags
    }
    pub fn file_info(&self) -> &FileInfo {
        &self.file_info
    }
    pub fn constant_sized(&self) -> bool {
        self.constant_sized
    }
}

pub fn create_if_statement(
    statement: &IfStatement,
    struct_ty_name: &str,
    tags: &Tags,
    o: &Objects,
    current_scope: &mut Vec<RustMember>,
    parent_scope: &mut Vec<RustMember>,
) {
    let mut if_enumerators = Vec::new();
    let mut not_enumerators = Vec::new();

    for i in statement.get_conditional().equations() {
        match i {
            Equation::BitwiseAnd { value } | Equation::Equals { value } => {
                if_enumerators.push(value)
            }
            Equation::NotEquals { value } => not_enumerators.push(value),
        }
    }

    let mut if_enumerator_members = Vec::new();
    for m in statement.members() {
        create_struct_member(
            m,
            struct_ty_name,
            tags,
            o,
            &mut if_enumerator_members,
            current_scope,
            &mut None,
            &mut false,
        );
    }

    let mut else_enumerator_members = Vec::new();
    for m in &statement.else_statement_members {
        create_struct_member(
            m,
            struct_ty_name,
            tags,
            o,
            &mut else_enumerator_members,
            current_scope,
            &mut None,
            &mut false,
        );
    }

    let subject = current_scope
        .iter_mut()
        .find(|a| statement.name() == a.name);
    let subject = match subject {
        None => parent_scope
            .iter_mut()
            .find(|a| statement.name() == a.name)
            .unwrap(),
        Some(s) => s,
    };

    if !if_enumerator_members.is_empty() {
        match &mut subject.ty {
            RustType::Enum { is_simple, .. } | RustType::Flag { is_simple, .. } => {
                *is_simple = false;
                subject.constant_sized = false;
            }
            _ => panic!("{} is not a definer", subject.name),
        }
    }

    let enums = match &mut subject.ty {
        RustType::Enum { enumerators, .. } | RustType::Flag { enumerators, .. } => enumerators,
        _ => panic!(),
    };

    for i in &if_enumerators {
        enums
            .iter_mut()
            .find(|a| &&a.name == i)
            .unwrap()
            .members
            .append(&mut if_enumerator_members.clone());
    }

    for i in &not_enumerators {
        enums
            .iter_mut()
            .find(|a| &&a.name != i)
            .unwrap()
            .members
            .append(&mut if_enumerator_members.clone());
    }

    for i in &if_enumerators {
        enums
            .iter_mut()
            .find(|a| &&a.name != i)
            .unwrap()
            .members
            .append(&mut else_enumerator_members.clone());
    }

    for i in &not_enumerators {
        enums
            .iter_mut()
            .find(|a| &&a.name == i)
            .unwrap()
            .members
            .append(&mut if_enumerator_members.clone());
    }

    for else_if in statement.else_ifs() {
        create_if_statement(
            else_if,
            struct_ty_name,
            tags,
            o,
            current_scope,
            parent_scope,
        );
    }
}

pub fn create_struct_member(
    m: &StructMember,
    struct_ty_name: &str,
    tags: &Tags,
    o: &Objects,
    current_scope: &mut Vec<RustMember>,
    parent_scope: &mut Vec<RustMember>,
    optional: &mut Option<RustOptional>,
    constant_sized: &mut bool,
) {
    match m {
        StructMember::Definition(d) => {
            let mut definition_constantly_sized = true;
            let ty = match d.ty() {
                Type::Integer(i) => {
                    if let Some(_) = d.used_as_size_in() {
                        return;
                    }
                    if let Some(_) = d.verified_value() {
                        return;
                    }
                    RustType::Integer(i.clone())
                }
                Type::Guid => RustType::BuiltIn("Guid".to_string()),
                Type::PackedGuid => {
                    *constant_sized = false;
                    definition_constantly_sized = false;
                    RustType::BuiltIn("Guid".to_string())
                }
                Type::FloatingPoint(f) => RustType::Floating(f.clone()),
                Type::CString | Type::String { .. } => {
                    *constant_sized = false;
                    definition_constantly_sized = false;
                    RustType::String
                }
                Type::Array(array) => {
                    match array.size() {
                        ArraySize::Fixed(_) => {}
                        ArraySize::Variable(_) | ArraySize::Endless => {
                            *constant_sized = false;
                            definition_constantly_sized = false;
                        }
                    }

                    match array.ty() {
                        ArrayType::Integer(_) | ArrayType::Guid => {}
                        ArrayType::Complex(complex) => {
                            let c = o.get_container(complex, tags);
                            if !c.has_constant_size(o) {
                                *constant_sized = false;
                                definition_constantly_sized = false;
                            }
                        }
                        ArrayType::PackedGuid | ArrayType::CString => {
                            *constant_sized = false;
                            definition_constantly_sized = false;
                        }
                    }

                    RustType::Array(array.clone())
                }
                Type::Identifier { s, upcast } => {
                    let add_types = || -> Vec<RustEnumerator> {
                        let mut enumerators = Vec::new();
                        let definer = o.get_definer(s, tags);

                        for field in definer.fields() {
                            enumerators.push(RustEnumerator {
                                name: field.name().to_string(),
                                value: field.value().clone(),
                                members: vec![],
                            });
                        }
                        enumerators
                    };

                    match o.get_object_type_of(s, tags) {
                        ObjectType::Enum => {
                            let enumerators = add_types();

                            RustType::Enum {
                                ty_name: s.clone(),
                                enumerators,
                                upcast: upcast.clone(),
                                is_simple: true,
                            }
                        }
                        ObjectType::Flag => {
                            let enumerators = add_types();

                            RustType::Flag {
                                ty_name: s.clone(),
                                enumerators,
                                is_simple: true,
                            }
                        }
                        ObjectType::Struct => {
                            let c = o.get_container(s, tags);
                            if !c.has_constant_size(o) {
                                *constant_sized = false;
                                definition_constantly_sized = false;
                            }

                            RustType::Struct(s.clone())
                        }
                        ObjectType::CLogin | ObjectType::SLogin => {
                            panic!("object contains message type")
                        }
                    }
                }
                Type::UpdateMask => {
                    *constant_sized = false;
                    definition_constantly_sized = false;
                    RustType::BuiltIn("UpdateMask".to_string())
                }
                Type::AuraMask => {
                    *constant_sized = false;
                    definition_constantly_sized = false;
                    RustType::BuiltIn("AuraMask".to_string())
                }
            };

            let name = d.name().to_string();

            current_scope.push(RustMember {
                name,
                ty,
                constant_sized: definition_constantly_sized,
                tags: tags.clone(),
            });
        }
        StructMember::IfStatement(statement) => {
            *constant_sized = false;
            create_if_statement(
                statement,
                struct_ty_name,
                tags,
                o,
                current_scope,
                parent_scope,
            );
        }
        StructMember::OptionalStatement(option) => {
            *constant_sized = false;

            let mut members = Vec::new();

            let mut constant_sized = true;
            for i in option.members() {
                create_struct_member(
                    i,
                    struct_ty_name,
                    tags,
                    o,
                    &mut members,
                    current_scope,
                    &mut None,
                    &mut constant_sized,
                );
            }

            *optional = Some(RustOptional {
                name: option.name().to_string(),
                ty: format!(
                    "{original_ty}_{ty}",
                    original_ty = struct_ty_name,
                    ty = option.name()
                ),
                members,
                constant_sized,
            });
        }
    }
}

pub fn create_rust_object(e: &Container, o: &Objects) -> RustObject {
    let mut v = Vec::new();
    let mut optional = None;

    let mut constant_sized = true;

    for m in e.fields() {
        create_struct_member(
            m,
            e.name(),
            e.tags(),
            o,
            &mut v,
            &mut vec![],
            &mut optional,
            &mut constant_sized,
        );
    }

    for m in &mut v {
        set_simple_objects_name(m, e.name());
    }

    RustObject {
        name: e.name().to_string(),
        members: v,
        optional,
        constant_sized,
        tests: e.tests().to_vec(),
        tags: e.tags().clone(),
        file_info: e.file_info().clone(),
    }
}

fn set_simple_objects_name(m: &mut RustMember, struct_ty_name: &str) {
    match &mut m.ty {
        RustType::Enum {
            ty_name,
            is_simple,
            enumerators,
            ..
        }
        | RustType::Flag {
            ty_name,
            is_simple,
            enumerators,
            ..
        } => {
            if !(*is_simple) {
                let definer_ty = ty_name.clone();
                ty_name.clear();
                ty_name.push_str(&format!(
                    "{struct_ty_name}{definer_ty}",
                    struct_ty_name = struct_ty_name,
                    definer_ty = definer_ty
                ));

                for e in enumerators {
                    for f in &mut e.members {
                        set_simple_objects_name(f, struct_ty_name);
                    }
                }
            }
        }
        _ => {}
    }
}
