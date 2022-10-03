use crate::impl_features::{get_impl_features_for_container, Feature};
use crate::ir_printer::{IrEndianness, IrFileInfo, IrIntegerType, IrTags};
use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::if_statement::{Conditional, Equation, IfStatement};
use crate::parser::types::optional::OptionalStatement;
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::test_case::{TestCase, TestCaseMember, TestUpdateMaskValue, TestValue};
use crate::parser::types::ty::Type;
use crate::parser::types::{
    Array, ArraySize, ArrayType, FloatingPointType, VerifiedContainerValue,
};
use crate::rust_printer::UpdateMaskType;
use serde::Serialize;

pub fn containers_to_ir(containers: &[&Container]) -> Vec<IrContainer> {
    containers.iter().map(|a| container_to_ir(a)).collect()
}

fn container_to_ir(e: &Container) -> IrContainer {
    let members = e.fields().iter().map(|a| a.into()).collect();

    let tests = e.tests().iter().map(|a| a.into()).collect();

    IrContainer {
        name: e.name().to_string(),
        object_type: e.container_type().into(),
        constant: e.is_constant_sized(),
        members,
        tags: IrTags::from_tags(e.tags()),
        tests,
        file_info: IrFileInfo {
            file_name: e.file_info().name().to_string(),
            start_position: e.file_info().start_line(),
        },
        only_has_io_error: e.only_has_io_errors(),
        features: get_impl_features_for_container(e).to_array(),
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "opcode")]
pub enum IrContainerType {
    #[serde(rename = "struct")]
    Struct,
    #[serde(rename = "clogin")]
    CLogin(u16),
    #[serde(rename = "slogin")]
    SLogin(u16),
    #[serde(rename = "msg")]
    Msg(u16),
    #[serde(rename = "cmsg")]
    CMsg(u16),
    #[serde(rename = "smsg")]
    SMsg(u16),
}

impl From<ContainerType> for IrContainerType {
    fn from(v: ContainerType) -> Self {
        match v {
            ContainerType::Struct => Self::Struct,
            ContainerType::CLogin(o) => Self::CLogin(o),
            ContainerType::SLogin(o) => Self::SLogin(o),
            ContainerType::Msg(o) => Self::Msg(o),
            ContainerType::CMsg(o) => Self::CMsg(o),
            ContainerType::SMsg(o) => Self::SMsg(o),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IrContainer {
    name: String,
    object_type: IrContainerType,
    constant: bool,
    members: Vec<IrStructMember>,
    tags: IrTags,
    tests: Vec<IrTestCase>,
    file_info: IrFileInfo,
    only_has_io_error: bool,
    features: Vec<Feature>,
}

impl IrContainer {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Serialize)]
pub struct IrOptionalStatement {
    name: String,
    members: Vec<IrStructMember>,
    tags: IrTags,
}

impl From<&OptionalStatement> for IrOptionalStatement {
    fn from(v: &OptionalStatement) -> Self {
        let members = v.members().iter().map(|a| a.into()).collect();

        Self {
            name: v.name().to_string(),
            members,
            tags: IrTags::from_tags(v.tags()),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum IrStructMember {
    #[serde(rename = "definition")]
    Definition(IrStructMemberDefinition),
    #[serde(rename = "if_statement")]
    IfStatement(IrIfStatement),
    #[serde(rename = "optional")]
    Optional(IrOptionalStatement),
}

impl From<&StructMember> for IrStructMember {
    fn from(v: &StructMember) -> Self {
        match v {
            StructMember::Definition(d) => Self::Definition(d.into()),
            StructMember::IfStatement(statement) => Self::IfStatement(statement.into()),
            StructMember::OptionalStatement(optional) => Self::Optional(optional.into()),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum IrEquation {
    #[serde(rename = "equals")]
    Equals { value: String },
    #[serde(rename = "not_equals")]
    NotEquals { value: String },
    #[serde(rename = "bitwise_and")]
    BitwiseAnd { value: String },
}

impl From<&Equation> for IrEquation {
    fn from(v: &Equation) -> Self {
        match v {
            Equation::Equals { value } => IrEquation::Equals {
                value: value.to_string(),
            },
            Equation::NotEquals { value } => IrEquation::NotEquals {
                value: value.to_string(),
            },
            Equation::BitwiseAnd { value } => IrEquation::BitwiseAnd {
                value: value.to_string(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IrConditional {
    variable_name: String,
    equations: Vec<IrEquation>,
}

impl From<Conditional> for IrConditional {
    fn from(v: Conditional) -> Self {
        let equations = v.equations().iter().map(|a| a.into()).collect();

        Self {
            variable_name: v.variable_name().to_string(),
            equations,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IrIfStatement {
    pub conditional: IrConditional,
    members: Vec<IrStructMember>,
    #[serde(rename = "else_if_statements")]
    else_ifs: Vec<IrIfStatement>,
    #[serde(rename = "else_members")]
    else_statement_members: Vec<IrStructMember>,
    #[serde(rename = "original_type")]
    original_ty: IrType,
}

impl From<&IfStatement> for IrIfStatement {
    fn from(v: &IfStatement) -> Self {
        let members = v.members().iter().map(|a| a.into()).collect();
        let else_ifs = v.else_ifs().iter().map(|a| a.into()).collect();

        let else_statement_members = v.else_members().iter().map(|a| a.into()).collect();

        Self {
            conditional: v.conditional.clone().into(),
            members,
            else_ifs,
            else_statement_members,
            original_ty: v.original_ty().into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IrStructMemberDefinition {
    name: String,
    member_type: IrType,
    constant_value: Option<IrIntegerEnumValue>,
    used_as_size_in: Option<String>,
    used_in_if: bool,
    tags: IrTags,
}

impl From<&StructMemberDefinition> for IrStructMemberDefinition {
    fn from(v: &StructMemberDefinition) -> Self {
        Self {
            name: v.name().to_string(),
            member_type: v.ty().into(),
            constant_value: v.verified_value().as_ref().map(|a| a.into()),
            used_as_size_in: v.used_as_size_in().clone(),
            used_in_if: v.used_in_if(),
            tags: IrTags::from_tags(v.tags()),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum IrType {
    #[serde(rename = "integer")]
    Integer(IrIntegerType),
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "datetime")]
    DateTime,
    #[serde(rename = "packed_guid")]
    PackedGuid,
    #[serde(rename = "guid")]
    Guid,
    #[serde(rename = "floating_point")]
    FloatingPoint(IrFloatingPointType),
    #[serde(rename = "cstring")]
    CString,
    #[serde(rename = "sized_cstring")]
    SizedCString,
    #[serde(rename = "string")]
    String { length: String },
    #[serde(rename = "array")]
    Array(IrArray),
    #[serde(rename = "identifier")]
    Identifier {
        type_name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        upcast: Option<IrIntegerType>,
    },
    #[serde(rename = "update_mask")]
    UpdateMask,
    #[serde(rename = "aura_mask")]
    AuraMask,
}

impl From<&Type> for IrType {
    fn from(v: &Type) -> Self {
        match v {
            Type::Integer(i) => Self::Integer(i.into()),
            Type::PackedGuid => Self::PackedGuid,
            Type::Guid => Self::Guid,
            Type::FloatingPoint(f) => Self::FloatingPoint(f.into()),
            Type::CString => Self::CString,
            Type::String { length } => Self::String {
                length: length.to_string(),
            },
            Type::UpdateMask => Self::UpdateMask,
            Type::AuraMask => Self::AuraMask,
            Type::Array(array) => Self::Array(array.into()),
            Type::Identifier { s, upcast } => Self::Identifier {
                type_name: s.to_string(),
                upcast: upcast.map(|a| (&a).into()),
            },
            Type::SizedCString => Self::SizedCString,
            Type::Bool => Self::Bool,
            Type::DateTime => Self::DateTime,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IrArray {
    inner: IrArrayType,
    size: IrArraySize,
}

impl From<&Array> for IrArray {
    fn from(v: &Array) -> Self {
        Self {
            inner: v.ty().into(),
            size: v.size().into(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum IrArrayType {
    #[serde(rename = "integer")]
    Integer(IrIntegerType),
    #[serde(rename = "complex")]
    Complex(String),
    #[serde(rename = "cstring")]
    CString,
    #[serde(rename = "guid")]
    Guid,
    #[serde(rename = "packed_guid")]
    PackedGuid,
}

impl From<&ArrayType> for IrArrayType {
    fn from(v: &ArrayType) -> Self {
        match v {
            ArrayType::Integer(i) => Self::Integer(i.into()),
            ArrayType::Complex(f) => Self::Complex(f.into()),
            ArrayType::CString => Self::CString,
            ArrayType::Guid => Self::Guid,
            ArrayType::PackedGuid => Self::PackedGuid,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum IrArraySize {
    #[serde(rename = "fixed")]
    Fixed(i64),
    #[serde(rename = "variable")]
    Variable(String),
    #[serde(rename = "endless")]
    Endless,
}

impl From<ArraySize> for IrArraySize {
    fn from(v: ArraySize) -> Self {
        match v {
            ArraySize::Fixed(s) => Self::Fixed(s),
            ArraySize::Variable(s) => Self::Variable(s),
            ArraySize::Endless => Self::Endless,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "endianness")]
pub enum IrFloatingPointType {
    #[serde(rename = "f32")]
    F32(IrEndianness),
    #[serde(rename = "f64")]
    F64(IrEndianness),
}

impl From<&FloatingPointType> for IrFloatingPointType {
    fn from(v: &FloatingPointType) -> Self {
        match v {
            FloatingPointType::F32(f) => Self::F32(f.into()),
            FloatingPointType::F64(f) => Self::F64(f.into()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IrIntegerEnumValue {
    value: u64,
    original_string: String,
}

impl From<&VerifiedContainerValue> for IrIntegerEnumValue {
    fn from(v: &VerifiedContainerValue) -> Self {
        Self {
            value: v.value(),
            original_string: v.original_string().to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IrTestCase {
    subject: String,
    members: Vec<IrTestCaseMember>,
    raw_bytes: Vec<u8>,
    tags: IrTags,
    file_info: IrFileInfo,
}

impl From<&TestCase> for IrTestCase {
    fn from(v: &TestCase) -> Self {
        let members = v.members().iter().map(|a| a.into()).collect();

        Self {
            subject: v.subject().to_string(),
            members,
            raw_bytes: v.raw_bytes().to_vec(),
            tags: IrTags::from_tags(v.tags()),
            file_info: IrFileInfo {
                file_name: v.file_info().name().to_string(),
                start_position: v.file_info().start_line(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IrTestCaseMember {
    variable_name: String,
    value: IrTestValue,
    tags: IrTags,
}

impl From<&TestCaseMember> for IrTestCaseMember {
    fn from(v: &TestCaseMember) -> Self {
        Self {
            variable_name: v.name().to_string(),
            value: v.value().into(),
            tags: IrTags::from_tags(v.tags()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IrTestUpdateMaskValue {
    #[serde(rename = "type")]
    ty: IrUpdateMaskType,
    name: String,
    value: String,
}

impl From<&TestUpdateMaskValue> for IrTestUpdateMaskValue {
    fn from(e: &TestUpdateMaskValue) -> Self {
        Self {
            ty: e.ty().into(),
            name: e.name().to_string(),
            value: e.value().to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum IrUpdateMaskType {
    #[serde(rename = "OBJECT")]
    Object,
    #[serde(rename = "ITEM")]
    Item,
    #[serde(rename = "UNIT")]
    Unit,
    #[serde(rename = "PLAYER")]
    Player,
    #[serde(rename = "CONTAINER")]
    Container,
    #[serde(rename = "GAMEOBJECT")]
    GameObject,
    #[serde(rename = "DYNAMICOBJECT")]
    DynamicObject,
    #[serde(rename = "CORPSE")]
    Corpse,
}

impl From<UpdateMaskType> for IrUpdateMaskType {
    fn from(e: UpdateMaskType) -> Self {
        match e {
            UpdateMaskType::Object => Self::Object,
            UpdateMaskType::Item => Self::Item,
            UpdateMaskType::Unit => Self::Unit,
            UpdateMaskType::Player => Self::Player,
            UpdateMaskType::Container => Self::Container,
            UpdateMaskType::GameObject => Self::GameObject,
            UpdateMaskType::DynamicObject => Self::DynamicObject,
            UpdateMaskType::Corpse => Self::Corpse,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum IrTestValue {
    #[serde(rename = "number")]
    Number(IrIntegerEnumValue),
    #[serde(rename = "bool")]
    Bool(bool),
    #[serde(rename = "datetime")]
    DateTime(IrIntegerEnumValue),
    #[serde(rename = "guid")]
    Guid(IrIntegerEnumValue),
    #[serde(rename = "floating_point")]
    FloatingNumber { value: f64, original_string: String },
    #[serde(rename = "array")]
    Array {
        values: Vec<usize>,
        size: IrArraySize,
    },
    #[serde(rename = "string")]
    String(String),
    #[serde(rename = "flag")]
    Flag(Vec<String>),
    #[serde(rename = "enum")]
    Enum(IrIntegerEnumValue),
    #[serde(rename = "sub_object")]
    SubObject {
        #[serde(rename = "type_name")]
        ty_name: String,
        members: Vec<IrTestCaseMember>,
    },
    #[serde(rename = "array_of_sub_object")]
    ArrayOfSubObject {
        type_name: String,
        members: Vec<Vec<IrTestCaseMember>>,
    },
    #[serde(rename = "update_mask")]
    UpdateMask(Vec<IrTestUpdateMaskValue>),
}

impl From<&TestValue> for IrTestValue {
    fn from(v: &TestValue) -> Self {
        match v {
            TestValue::Number(i) => Self::Number(i.into()),
            TestValue::DateTime(v) => Self::DateTime(v.into()),
            TestValue::Guid(i) => Self::Guid(i.into()),
            TestValue::Bool(b) => Self::Bool(*b),
            TestValue::FloatingNumber {
                value,
                original_string,
            } => Self::FloatingNumber {
                value: *value,
                original_string: original_string.to_string(),
            },
            TestValue::Array { values, size } => Self::Array {
                values: values.to_vec(),
                size: size.clone().into(),
            },
            TestValue::String(s) => Self::String(s.to_string()),
            TestValue::Flag(f) => Self::Flag(f.to_vec()),
            TestValue::Enum(e) => Self::Enum(e.into()),
            TestValue::SubObject { ty_name, members } => Self::SubObject {
                ty_name: ty_name.to_string(),
                members: members.iter().map(|a| a.into()).collect(),
            },
            TestValue::ArrayOfSubObject(s, t) => Self::ArrayOfSubObject {
                type_name: s.to_string(),
                members: t
                    .iter()
                    .map(|a| a.iter().map(|a| a.into()).collect::<Vec<_>>())
                    .collect(),
            },
            TestValue::UpdateMask(v) => {
                IrTestValue::UpdateMask(v.iter().map(|a| a.into()).collect())
            }
        }
    }
}
