use crate::impl_features::{get_impl_features_for_container, Feature};
use crate::ir_printer::{IrFileInfo, IrIntegerType, IrTags};
use crate::parser::types::array::{Array, ArraySize, ArrayType};
use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::if_statement::{Conditional, Equation, IfStatement};
use crate::parser::types::optional::OptionalStatement;
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::test_case::{TestCase, TestCaseMember, TestUpdateMaskValue, TestValue};
use crate::parser::types::ty::Type;
use crate::parser::types::ContainerValue;
use crate::rust_printer::UpdateMaskType;
use crate::Objects;
use serde::Serialize;

pub(crate) fn containers_to_ir(containers: &[&Container], o: &Objects) -> Vec<IrContainer> {
    containers.iter().map(|a| container_to_ir(a, o)).collect()
}

fn container_to_ir(e: &Container, o: &Objects) -> IrContainer {
    let members = e.members().iter().map(|a| a.into()).collect();

    let tests = e.tests(o).iter().map(|a| a.into()).collect();

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
pub(crate) enum IrContainerType {
    Struct,
    CLogin(u16),
    SLogin(u16),
    Msg(u16),
    CMsg(u16),
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
pub(crate) struct IrContainer {
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
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrOptionalStatement {
    name: String,
    members: Vec<IrStructMember>,
}

impl From<&OptionalStatement> for IrOptionalStatement {
    fn from(v: &OptionalStatement) -> Self {
        let members = v.members().iter().map(|a| a.into()).collect();

        Self {
            name: v.name().to_string(),
            members,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "content")]
#[serde(rename_all = "snake_case")]
pub(crate) enum IrStructMember {
    Definition(Box<IrStructMemberDefinition>),
    IfStatement(IrIfStatement),
    Optional(IrOptionalStatement),
}

impl From<&StructMember> for IrStructMember {
    fn from(v: &StructMember) -> Self {
        match v {
            StructMember::Definition(d) => Self::Definition(Box::new(d.into())),
            StructMember::IfStatement(statement) => Self::IfStatement(statement.into()),
            StructMember::OptionalStatement(optional) => Self::Optional(optional.into()),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "content")]
#[serde(rename_all = "snake_case")]
pub(crate) enum IrEquation {
    Equals { value: String },
    NotEquals { value: String },
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
pub(crate) struct IrConditional {
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
pub(crate) struct IrIfStatement {
    pub conditional: IrConditional,
    members: Vec<IrStructMember>,
    else_if_statements: Vec<IrIfStatement>,
    else_members: Vec<IrStructMember>,
    original_type: IrType,
}

impl From<&IfStatement> for IrIfStatement {
    fn from(v: &IfStatement) -> Self {
        let members = v.members().iter().map(|a| a.into()).collect();
        let else_ifs = v.else_ifs().iter().map(|a| a.into()).collect();

        let else_statement_members = v.else_members().iter().map(|a| a.into()).collect();

        Self {
            conditional: v.conditional().clone().into(),
            members,
            else_if_statements: else_ifs,
            else_members: else_statement_members,
            original_type: v.original_ty().into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrStructMemberDefinition {
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
            constant_value: v.value().as_ref().map(|a| a.into()),
            used_as_size_in: v.used_as_size_in().clone(),
            used_in_if: v.used_in_if(),
            tags: IrTags::from_member_tags(v.tags()),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "content")]
#[serde(rename_all = "snake_case")]
pub(crate) enum IrType {
    Integer(IrIntegerType),
    Bool(IrIntegerType),
    DateTime,
    PackedGuid,
    Guid,
    NamedGuid,
    FloatingPoint,
    CString,
    SizedCString,
    String,
    Array(IrArray),
    Enum {
        type_name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        upcast: Option<IrIntegerType>,
    },
    Flag {
        type_name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        upcast: Option<IrIntegerType>,
    },
    Struct {
        type_name: String,
    },
    UpdateMask,
    AuraMask,
    MonsterMoveSpline,
    AchievementDoneArray,
    AchievementInProgressArray,
    EnchantMask,
    InspectTalentGearMask,
    Gold,
    Level,
    Level16,
    Level32,
    VariableItemRandomProperty,
    AddonArray,
    IpAddress,
    Seconds,
    Milliseconds,
}

impl From<&Type> for IrType {
    fn from(v: &Type) -> Self {
        match v {
            Type::Integer(i) => Self::Integer(i.into()),
            Type::PackedGuid => Self::PackedGuid,
            Type::Guid => Self::Guid,
            Type::NamedGuid => Self::NamedGuid,
            Type::FloatingPoint => Self::FloatingPoint,
            Type::CString => Self::CString,
            Type::String => Self::String,
            Type::UpdateMask { .. } => Self::UpdateMask,
            Type::AuraMask => Self::AuraMask,
            Type::Array(array) => Self::Array(array.into()),
            Type::Enum { e, upcast } => Self::Enum {
                type_name: e.name().to_string(),
                upcast: upcast.map(|a| (&a).into()),
            },
            Type::Flag { e, upcast } => Self::Flag {
                type_name: e.name().to_string(),
                upcast: upcast.map(|a| (&a).into()),
            },
            Type::Struct { e } => Self::Struct {
                type_name: e.name().to_string(),
            },
            Type::SizedCString => Self::SizedCString,
            Type::Bool(i) => Self::Bool(i.into()),
            Type::DateTime => Self::DateTime,
            Type::AchievementDoneArray => Self::AchievementDoneArray,
            Type::AchievementInProgressArray => Self::AchievementInProgressArray,
            Type::MonsterMoveSplines => Self::MonsterMoveSpline,
            Type::EnchantMask => Self::EnchantMask,
            Type::InspectTalentGearMask => Self::InspectTalentGearMask,
            Type::Gold => Self::Gold,
            Type::Level => Self::Level,
            Type::Level16 => Self::Level16,
            Type::Level32 => Self::Level32,
            Type::VariableItemRandomProperty => Self::VariableItemRandomProperty,
            Type::AddonArray => Self::AddonArray,
            Type::IpAddress => Self::IpAddress,
            Type::Seconds => Self::Seconds,
            Type::Milliseconds => Self::Milliseconds,
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrArray {
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
#[serde(rename_all = "snake_case")]
pub(crate) enum IrArrayType {
    Integer(IrIntegerType),
    Complex(String),
    CString,
    Guid,
    PackedGuid,
}

impl From<&ArrayType> for IrArrayType {
    fn from(v: &ArrayType) -> Self {
        match v {
            ArrayType::Integer(i) => Self::Integer(i.into()),
            ArrayType::Struct(f) => Self::Complex(f.name().into()),
            ArrayType::CString => Self::CString,
            ArrayType::Guid => Self::Guid,
            ArrayType::PackedGuid => Self::PackedGuid,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "content")]
#[serde(rename_all = "snake_case")]
pub(crate) enum IrArraySize {
    Fixed(i64),
    Variable(String),
    Endless,
}

impl From<ArraySize> for IrArraySize {
    fn from(v: ArraySize) -> Self {
        match v {
            ArraySize::Fixed(s) => Self::Fixed(s),
            ArraySize::Variable(s) => Self::Variable(s.name().into()),
            ArraySize::Endless => Self::Endless,
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrIntegerEnumValue {
    value: u64,
    original_string: String,
}

impl From<&ContainerValue> for IrIntegerEnumValue {
    fn from(v: &ContainerValue) -> Self {
        Self {
            value: v.value(),
            original_string: v.original_string().to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrTestCase {
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
pub(crate) struct IrTestCaseMember {
    variable_name: String,
    value: IrTestValue,
    tags: IrTags,
}

impl From<&TestCaseMember> for IrTestCaseMember {
    fn from(v: &TestCaseMember) -> Self {
        Self {
            variable_name: v.name().to_string(),
            value: v.value().into(),
            tags: IrTags::from_member_tags(v.tags()),
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct IrTestUpdateMaskValue {
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
pub(crate) enum IrUpdateMaskType {
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
#[serde(rename_all = "snake_case")]
pub(crate) enum IrTestValue {
    Number(IrIntegerEnumValue),
    Bool(bool),
    DateTime(IrIntegerEnumValue),
    Guid(IrIntegerEnumValue),
    FloatingNumber {
        value: f64,
        original_string: String,
    },
    Array {
        values: Vec<usize>,
        size: IrArraySize,
    },
    String(String),
    Flag(Vec<String>),
    Enum(IrIntegerEnumValue),
    SubObject {
        type_name: String,
        members: Vec<IrTestCaseMember>,
    },
    ArrayOfSubObject {
        type_name: String,
        members: Vec<Vec<IrTestCaseMember>>,
    },
    UpdateMask(Vec<IrTestUpdateMaskValue>),
    IpAddress(IrIntegerEnumValue),
    Seconds(IrIntegerEnumValue),
    Milliseconds(IrIntegerEnumValue),
}

impl From<&TestValue> for IrTestValue {
    fn from(v: &TestValue) -> Self {
        match v {
            TestValue::Number(i) => Self::Number(i.into()),
            TestValue::DateTime(v) => Self::DateTime(v.into()),
            TestValue::Guid(i) => Self::Guid(i.into()),
            TestValue::Bool(b) => Self::Bool(*b),
            TestValue::IpAddress(v) => IrTestValue::IpAddress(v.into()),
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
            TestValue::SubObject { c, members } => Self::SubObject {
                type_name: c.name().to_string(),
                members: members.iter().map(|a| a.into()).collect(),
            },
            TestValue::ArrayOfSubObject(e, t) => Self::ArrayOfSubObject {
                type_name: e.name().to_string(),
                members: t
                    .iter()
                    .map(|a| a.iter().map(|a| a.into()).collect::<Vec<_>>())
                    .collect(),
            },
            TestValue::UpdateMask(v) => {
                IrTestValue::UpdateMask(v.iter().map(|a| a.into()).collect())
            }
            TestValue::Seconds(i) => Self::Seconds(i.into()),
            TestValue::Milliseconds(i) => Self::Milliseconds(i.into()),
        }
    }
}
