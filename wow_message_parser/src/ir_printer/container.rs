use crate::ir_printer::definer::IrDefinerType;
use crate::ir_printer::{IrFileInfo, IrIntegerType, IrTags};
use crate::parser::types::array::{Array, ArraySize, ArrayType};
use crate::parser::types::container::{Container, ContainerType, UpdateMaskMember};
use crate::parser::types::definer::Definer;
use crate::parser::types::if_statement::{Equation, IfStatement};
use crate::parser::types::optional::OptionalStatement;
use crate::parser::types::sizes::Sizes;
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::test_case::{
    TestCase, TestCaseMember, TestUpdateMaskValue, TestValue, TestVector3d,
};
use crate::parser::types::ty::Type;
use crate::parser::types::ContainerValue;
use crate::rust_printer::rust_view::rust_member::RustMember;
use crate::rust_printer::rust_view::rust_optional::RustOptional;
use crate::rust_printer::rust_view::rust_type::RustType;
use crate::rust_printer::UpdateMaskObjectType;
use crate::Objects;
use serde::Serialize;
use std::collections::BTreeMap;

pub(crate) fn containers_to_ir(containers: &[Container], o: &Objects) -> Vec<IrContainer> {
    containers.iter().map(|a| container_to_ir(a, o)).collect()
}

fn container_to_ir_no_tests(e: &Container) -> IrContainer {
    let members = e
        .members()
        .iter()
        .filter_map(IrStructMember::from_struct_member)
        .collect();

    let has_manual_size_field = e
        .all_definitions()
        .iter()
        .any(|a| a.size_of_fields_before_size().is_some());

    IrContainer {
        name: e.name().to_string(),
        object_type: IrContainerType::from_container_type(e.container_type()),
        sizes: IrSizes::from_sizes(e.sizes()),
        members,
        tags: IrTags::from_tags(e.tags()),
        tests: Vec::new(),
        file_info: IrFileInfo::from_file_info(e.file_info()),
        prepared_objects: rust_object_to_prepared_objects(e.rust_object().members()),
        only_has_io_error: e.only_has_io_errors(),
        has_manual_size_field,
        manual_size_subtraction: e.manual_size_field_subtraction(),
        optional: get_optional(e.members(), e.rust_object().optional()),
    }
}

fn get_optional(
    members: &[StructMember],
    rust_optional: Option<&RustOptional>,
) -> Option<IrOptionalStatement> {
    for member in members {
        match member {
            StructMember::Definition(_) => {}
            StructMember::IfStatement(_) => {}
            StructMember::OptionalStatement(optional) => {
                return Some(IrOptionalStatement::from_optional(
                    optional,
                    rust_optional.unwrap(),
                ));
            }
        }
    }

    None
}

pub(crate) fn container_to_update_mask_ir(e: &Container) -> IrUpdateMaskStruct {
    let members = e.create_update_mask_members();

    let members = members
        .iter()
        .map(|a| a.iter().map(IrUpdateMaskMember::from_member).collect())
        .collect();

    IrUpdateMaskStruct {
        name: e.name().to_string(),
        sizes: IrSizes::from_sizes(e.sizes()),
        members,
        tags: IrTags::from_tags(e.tags()),
        file_info: IrFileInfo::from_file_info(e.file_info()),
    }
}

pub(crate) fn container_to_ir(e: &Container, o: &Objects) -> IrContainer {
    let mut c = container_to_ir_no_tests(e);

    let tests = e.tests(o).iter().map(IrTestCase::from_test_case).collect();

    c.tests = tests;

    c
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(tag = "container_type_tag", content = "opcode")]
pub(crate) enum IrContainerType {
    Struct,
    CLogin(u16),
    SLogin(u16),
    Msg(u16),
    CMsg(u16),
    SMsg(u16),
}

impl IrContainerType {
    fn from_container_type(v: ContainerType) -> Self {
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

#[derive(Clone, Debug, Serialize)]
pub(crate) struct IrSizes {
    constant_sized: bool,
    minimum_size: u32,
    maximum_size: u32,
}

fn i128_to_u32(v: i128) -> u32 {
    if v < 0 {
        0
    } else if v >= (u32::MAX as i128) {
        u32::MAX
    } else {
        v as u32
    }
}

impl IrSizes {
    fn from_sizes(v: Sizes) -> Self {
        Self {
            constant_sized: v.is_constant().is_some(),
            minimum_size: i128_to_u32(v.minimum()),
            maximum_size: i128_to_u32(v.maximum()),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct IrPreparedObject {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    enumerators: Option<BTreeMap<String, Vec<IrPreparedObject>>>,
    is_elseif_flag: bool,
    enum_part_of_separate_statements: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    definer_type: Option<IrDefinerType>,
}

pub(crate) fn rust_object_to_prepared_objects(members: &[RustMember]) -> Vec<IrPreparedObject> {
    fn inner(m: &RustMember, v: &mut Vec<IrPreparedObject>) {
        let enumerators = match m.ty() {
            RustType::Flag { enumerators, .. } | RustType::Enum { enumerators, .. } => {
                let mut map = BTreeMap::new();

                for enumerator in enumerators {
                    let mut fields = Vec::new();

                    for field in enumerator.members() {
                        inner(field, &mut fields);
                    }

                    if !fields.is_empty() {
                        map.insert(enumerator.name().to_string(), fields);
                    }
                }

                if !map.is_empty() {
                    Some(map)
                } else {
                    None
                }
            }
            _ => None,
        };

        let (enum_part_of_separate_statements, is_elseif_flag) = match m.ty() {
            RustType::Flag { is_elseif, .. } => (false, *is_elseif),
            RustType::Enum {
                separate_if_statements,
                ..
            } => (*separate_if_statements, false),
            _ => (false, false),
        };

        v.push(IrPreparedObject {
            name: m.name().to_string(),
            enumerators,
            is_elseif_flag,
            enum_part_of_separate_statements,
            definer_type: if let Some(dt) = m.definer_type() {
                Some(IrDefinerType::from_definer_type(dt))
            } else {
                None
            },
        });
    }

    let mut v = Vec::new();

    for m in members {
        inner(m, &mut v);
    }

    v
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct IrContainer {
    name: String,
    object_type: IrContainerType,
    sizes: IrSizes,
    members: Vec<IrStructMember>,
    tags: IrTags,
    tests: Vec<IrTestCase>,
    file_info: IrFileInfo,
    only_has_io_error: bool,
    has_manual_size_field: bool,
    manual_size_subtraction: Option<u16>,
    prepared_objects: Vec<IrPreparedObject>,
    optional: Option<IrOptionalStatement>,
}

#[derive(Debug, Serialize)]
pub(crate) struct IrUpdateMaskStruct {
    name: String,
    sizes: IrSizes,
    members: Vec<Vec<IrUpdateMaskMember>>,
    tags: IrTags,
    file_info: IrFileInfo,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct IrUpdateMaskMember {
    member: IrStructMemberDefinition,
    offset: i32,
    size: i32,
}

impl IrUpdateMaskMember {
    pub(crate) fn from_member(a: &UpdateMaskMember) -> Self {
        Self {
            member: IrStructMemberDefinition::from_definition(&a.member),
            offset: a.offset,
            size: a.size,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct IrOptionalStatement {
    name: String,
    members: Vec<IrStructMember>,
    prepared_objects: Vec<IrPreparedObject>,
}

impl IrOptionalStatement {
    fn from_optional(v: &OptionalStatement, rust_optional: &RustOptional) -> Self {
        let members = v
            .members()
            .iter()
            .filter_map(IrStructMember::from_struct_member)
            .collect();

        Self {
            name: v.name().to_string(),
            members,
            prepared_objects: rust_object_to_prepared_objects(rust_optional.members()),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "struct_member_tag", content = "struct_member_content")]
pub(crate) enum IrStructMember {
    Definition(Box<IrStructMemberDefinition>),
    IfStatement(IrIfStatement),
}

impl IrStructMember {
    fn from_struct_member(v: &StructMember) -> Option<Self> {
        match v {
            StructMember::Definition(d) => Some(Self::Definition(Box::new(
                IrStructMemberDefinition::from_definition(d),
            ))),
            StructMember::IfStatement(statement) => {
                Some(Self::IfStatement(IrIfStatement::from_statement(statement)))
            }
            StructMember::OptionalStatement(_) => None,
        }
    }
}

fn values_from_equation(v: &Equation, definer: &Definer) -> Vec<String> {
    match v {
        Equation::Equals { values: value } => value.clone(),
        Equation::NotEquals { value } => definer
            .fields()
            .iter()
            .filter_map(|d| {
                if d.name() == value {
                    None
                } else {
                    Some(d.name().to_string())
                }
            })
            .collect(),
        Equation::BitwiseAnd { values: value } => value.clone(),
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct IrIfStatement {
    variable_name: String,
    values: Vec<String>,
    members: Vec<IrStructMember>,
    else_if_statements: Vec<IrIfStatement>,
    original_type: IrType,
    definer_type: IrDefinerType,
    part_of_separate_if_statement: bool,
    is_else_if_flag: bool,
}

impl IrIfStatement {
    fn from_statement(v: &IfStatement) -> Self {
        let members = v
            .members()
            .iter()
            .filter_map(IrStructMember::from_struct_member)
            .collect();
        let mut else_ifs: Vec<_> = v
            .else_ifs()
            .iter()
            .map(IrIfStatement::from_statement)
            .collect();

        let else_statement_members: Vec<_> = v
            .else_members()
            .iter()
            .filter_map(IrStructMember::from_struct_member)
            .collect();

        let mut else_conditions = Vec::new();
        for enumerator in v.original_ty().definer().fields() {
            if !v.equation().contains_enumerator(enumerator.name())
                && !v
                    .else_ifs()
                    .iter()
                    .any(|a| a.equation().contains_enumerator(enumerator.name()))
            {
                else_conditions.push(enumerator.name().to_string());
            }
        }

        if !else_statement_members.is_empty() {
            else_ifs.push(IrIfStatement {
                variable_name: v.variable_name().to_string(),
                values: else_conditions,
                members: else_statement_members,
                else_if_statements: vec![],
                original_type: IrType::from_type(v.original_ty()),
                definer_type: IrDefinerType::from_definer_type(
                    v.original_ty().definer().definer_ty(),
                ),
                part_of_separate_if_statement: v.part_of_separate_if_statement(),
                is_else_if_flag: v.is_elseif_flag(),
            });
        }

        Self {
            variable_name: v.variable_name().to_string(),
            values: values_from_equation(v.equation(), v.original_ty().definer()),
            members,
            else_if_statements: else_ifs,
            original_type: IrType::from_type(v.original_ty()),
            definer_type: IrDefinerType::from_definer_type(v.original_ty().definer().definer_ty()),
            part_of_separate_if_statement: v.part_of_separate_if_statement(),
            is_else_if_flag: v.is_elseif_flag(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct IrStructMemberDefinition {
    name: String,
    data_type: IrType,
    constant_value: Option<IrIntegerEnumValue>,
    used_as_size_in: Option<String>,
    size_of_fields_before_size: Option<u8>,
    used_in_if: bool,
    tags: IrTags,
}

impl IrStructMemberDefinition {
    fn from_definition(v: &StructMemberDefinition) -> Self {
        Self {
            name: v.name().to_string(),
            data_type: IrType::from_type(v.ty()),
            constant_value: v
                .value()
                .as_ref()
                .map(IrIntegerEnumValue::from_container_value),
            used_as_size_in: v.used_as_size_in().clone(),
            size_of_fields_before_size: v.size_of_fields_before_size().map(|a| a as u8),
            used_in_if: v.used_in_if(),
            tags: IrTags::from_member_tags(v.tags()),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "data_type_tag")]
pub(crate) enum IrType {
    Integer {
        integer_type: IrIntegerType,
    },
    Bool {
        integer_type: IrIntegerType,
    },
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
        integer_type: IrIntegerType,
        upcast: bool,
    },
    Flag {
        type_name: String,
        integer_type: IrIntegerType,
        upcast: bool,
    },
    Struct {
        struct_data: IrContainer,
    },
    UpdateMask,
    AuraMask,
    MonsterMoveSpline,
    AchievementDoneArray,
    AchievementInProgressArray,
    EnchantMask,
    InspectTalentGearMask,
    Gold,
    Population,
    Level,
    Level16,
    Level32,
    VariableItemRandomProperty,
    AddonArray,
    IpAddress,
    Seconds,
    Milliseconds,
    Spell,
    Spell16,
    Item,
    CacheMask,
}

impl IrType {
    fn from_type(v: &Type) -> Self {
        match v {
            Type::Integer(i) => Self::Integer {
                integer_type: IrIntegerType::from_integer_type(i),
            },
            Type::PackedGuid => Self::PackedGuid,
            Type::Guid => Self::Guid,
            Type::NamedGuid => Self::NamedGuid,
            Type::FloatingPoint => Self::FloatingPoint,
            Type::CString => Self::CString,
            Type::String => Self::String,
            Type::UpdateMask { .. } => Self::UpdateMask,
            Type::AuraMask => Self::AuraMask,
            Type::Array(array) => Self::Array(IrArray::from_array(array)),
            Type::Enum { e, upcast } => {
                let (upcast, integer_type) = if let Some(c) = upcast {
                    (true, IrIntegerType::from_integer_type(c))
                } else {
                    (false, IrIntegerType::from_integer_type(e.ty()))
                };

                Self::Enum {
                    type_name: e.name().to_string(),
                    integer_type,
                    upcast,
                }
            }
            Type::Flag { e, upcast } => {
                let (upcast, integer_type) = if let Some(c) = upcast {
                    (true, IrIntegerType::from_integer_type(c))
                } else {
                    (false, IrIntegerType::from_integer_type(e.ty()))
                };

                Self::Flag {
                    type_name: e.name().to_string(),
                    integer_type,
                    upcast,
                }
            }
            Type::Struct { e } => Self::Struct {
                struct_data: container_to_ir_no_tests(e),
            },
            Type::SizedCString => Self::SizedCString,
            Type::Bool(i) => Self::Bool {
                integer_type: IrIntegerType::from_integer_type(i),
            },
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
            Type::Population => Self::Population,
            Type::Spell => Self::Spell,
            Type::Spell16 => Self::Spell16,
            Type::Item => Self::Item,
            Type::CacheMask => Self::CacheMask,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct IrArray {
    inner_type: IrArrayType,
    size: IrArraySize,
    compressed: bool,
}

impl IrArray {
    fn from_array(v: &Array) -> Self {
        Self {
            inner_type: IrArrayType::from_array_type(v.ty()),
            size: IrArraySize::from_array_size(v.size()),
            compressed: v.compressed(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "array_type_tag")]
pub(crate) enum IrArrayType {
    Integer { integer_type: IrIntegerType },
    Struct { struct_data: IrContainer },
    CString,
    Guid,
    PackedGuid,
    Spell,
}

impl IrArrayType {
    fn from_array_type(v: &ArrayType) -> Self {
        match v {
            ArrayType::Integer(i) => Self::Integer {
                integer_type: IrIntegerType::from_integer_type(i),
            },
            ArrayType::Struct(f) => Self::Struct {
                struct_data: container_to_ir_no_tests(f),
            },
            ArrayType::CString => Self::CString,
            ArrayType::Guid => Self::Guid,
            ArrayType::PackedGuid => Self::PackedGuid,
            ArrayType::Spell => Self::Spell,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "array_size_tag", content = "size")]
pub(crate) enum IrArraySize {
    Fixed(String),
    Variable(String),
    Endless,
}

impl IrArraySize {
    fn from_array_size(v: ArraySize) -> Self {
        match v {
            ArraySize::Fixed(s) => Self::Fixed(s.to_string()),
            ArraySize::Variable(s) => Self::Variable(s.name().into()),
            ArraySize::Endless => Self::Endless,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct IrIntegerEnumValue {
    pub value: String,
    pub original_string: String,
}

impl IrIntegerEnumValue {
    fn from_container_value(v: &ContainerValue) -> Self {
        Self {
            value: v.value().to_string(),
            original_string: v.original_string().to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct IrTestCase {
    subject: String,
    members: Vec<IrTestCaseMember>,
    raw_bytes: Vec<u8>,
    tags: IrTags,
    file_info: IrFileInfo,
}

impl IrTestCase {
    fn from_test_case(v: &&TestCase) -> Self {
        let members = v
            .members()
            .iter()
            .map(IrTestCaseMember::from_test_case_member)
            .collect();

        Self {
            subject: v.subject().to_string(),
            members,
            raw_bytes: v.raw_bytes().to_vec(),
            tags: IrTags::from_tags(v.tags()),
            file_info: IrFileInfo {
                file_name: v.file_info().name().to_string(),
                start_position: v.file_info().start_line() as u32,
                end_position: v.file_info().end_line() as u32,
            },
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct IrTestCaseMember {
    variable_name: String,
    value: IrTestValue,
    tags: IrTags,
}

impl IrTestCaseMember {
    fn from_test_case_member(v: &TestCaseMember) -> Self {
        Self {
            variable_name: v.name().to_string(),
            value: IrTestValue::from_test_value(v.value()),
            tags: IrTags::from_member_tags(v.tags()),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct IrTestUpdateMaskValue {
    update_mask_type: IrUpdateMaskType,
    update_mask_name: String,
    update_mask_value: String,
}

impl IrTestUpdateMaskValue {
    fn from_mask_value(e: &TestUpdateMaskValue) -> Self {
        Self {
            update_mask_type: IrUpdateMaskType::from_mask_type(e.ty()),
            update_mask_name: e.name().to_string(),
            update_mask_value: e.value().to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
pub(crate) enum IrUpdateMaskType {
    Object,
    Item,
    Unit,
    Player,
    Container,
    GameObject,
    DynamicObject,
    Corpse,
}

impl IrUpdateMaskType {
    fn from_mask_type(e: UpdateMaskObjectType) -> Self {
        match e {
            UpdateMaskObjectType::Object => Self::Object,
            UpdateMaskObjectType::Item => Self::Item,
            UpdateMaskObjectType::Unit => Self::Unit,
            UpdateMaskObjectType::Player => Self::Player,
            UpdateMaskObjectType::Container => Self::Container,
            UpdateMaskObjectType::GameObject => Self::GameObject,
            UpdateMaskObjectType::DynamicObject => Self::DynamicObject,
            UpdateMaskObjectType::Corpse => Self::Corpse,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "test_value_tag", content = "content")]
pub(crate) enum IrTestValue {
    Integer(IrIntegerEnumValue),
    Bool(bool),
    DateTime(IrIntegerEnumValue),
    Guid(IrIntegerEnumValue),
    FloatingPoint {
        value: f64,
        original_string: String,
    },
    Array {
        values: Vec<String>,
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
        size: IrArraySize,
    },
    MonsterMoveSpline(Vec<TestVector3d>),
    UpdateMask(Vec<IrTestUpdateMaskValue>),
    IpAddress(IrIntegerEnumValue),
    Seconds(IrIntegerEnumValue),
    Milliseconds(IrIntegerEnumValue),
    Population(f32),
    Gold(IrIntegerEnumValue),
    Level(IrIntegerEnumValue),
}

impl IrTestValue {
    fn from_test_value(v: &TestValue) -> Self {
        match v {
            TestValue::Number(i) => Self::Integer(IrIntegerEnumValue::from_container_value(i)),
            TestValue::DateTime(v) => Self::DateTime(IrIntegerEnumValue::from_container_value(v)),
            TestValue::Guid(i) => Self::Guid(IrIntegerEnumValue::from_container_value(i)),
            TestValue::Bool(b) => Self::Bool(*b),
            TestValue::IpAddress(v) => {
                IrTestValue::IpAddress(IrIntegerEnumValue::from_container_value(v))
            }
            TestValue::FloatingNumber {
                value,
                original_string,
            } => Self::FloatingPoint {
                value: *value,
                original_string: original_string.to_string(),
            },
            TestValue::IntegerArray { values, size } => Self::Array {
                values: values.iter().map(|a| a.to_string()).collect(),
                size: IrArraySize::from_array_size(size.clone()),
            },
            TestValue::String(s) => Self::String(s.to_string()),
            TestValue::Flag(f) => Self::Flag(f.to_vec()),
            TestValue::Enum(e) => Self::Enum(IrIntegerEnumValue::from_container_value(e)),
            TestValue::SubObject { c, members } => Self::SubObject {
                type_name: c.name().to_string(),
                members: members
                    .iter()
                    .map(IrTestCaseMember::from_test_case_member)
                    .collect(),
            },
            TestValue::ArrayOfSubObject {
                c: e,
                members: t,
                size,
            } => Self::ArrayOfSubObject {
                type_name: e.name().to_string(),
                members: t
                    .iter()
                    .map(|a| {
                        a.iter()
                            .map(IrTestCaseMember::from_test_case_member)
                            .collect::<Vec<_>>()
                    })
                    .collect(),
                size: IrArraySize::from_array_size(size.clone()),
            },
            TestValue::UpdateMask(v) => IrTestValue::UpdateMask(
                v.iter()
                    .map(IrTestUpdateMaskValue::from_mask_value)
                    .collect(),
            ),
            TestValue::Seconds(i) => Self::Seconds(IrIntegerEnumValue::from_container_value(i)),
            TestValue::Milliseconds(i) => {
                Self::Milliseconds(IrIntegerEnumValue::from_container_value(i))
            }
            TestValue::Population { value, .. } => Self::Population(*value),
            TestValue::Gold(i) => Self::Gold(IrIntegerEnumValue::from_container_value(i)),
            TestValue::Level(i) => Self::Level(IrIntegerEnumValue::from_container_value(i)),
            TestValue::MonsterMoveSpline(v) => Self::MonsterMoveSpline(v.clone()),
            TestValue::StringArray { values, size } => Self::Array {
                values: values.clone(),
                size: IrArraySize::from_array_size(size.clone()),
            },
        }
    }
}
