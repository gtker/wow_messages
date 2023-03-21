use crate::parser::types::array::{ArraySize, ArrayType};
use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::definer::Definer;
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::tags::ObjectTags;
use crate::parser::types::ty::Type;
use crate::rust_printer::DefinerType;
use crate::CONTAINER_SELF_SIZE_FIELD;
use serde::Serialize;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
pub(crate) enum Feature {
    Struct,
    CLogin,
    SLogin,
    SMsg,
    CMsg,
    Msg,

    #[allow(unused)]
    NestedStructTypes,

    ConstantValue,
    ManualSizeField,

    SimpleBuiltInTypes,
    ComplexBuiltInTypes,
    PackedGuidTypes,
    NamedGuidTypes,
    UpdateMaskTypes,
    AuraMaskTypes,
    EnchantMaskTypes,
    InspectTalentGearMaskTypes,
    VariableItemRandomPropertyTypes,
    AddonArrayType,

    AchievementDoneArrayType,
    AchievementInProgressType,

    MonsterMoveSpline,

    EmptyContainer,

    SimpleArrays,
    ComplexArrays,

    FixedArrays,
    VariableArrays,
    EndlessArrays,

    SubObjects,

    IfEnum,
    IfNotEnum,
    IfElseEnum,
    NotElseEnum,
    ElseEnum,

    #[allow(unused)]
    IfInDifferentScope,

    IfFlag,
    IfElseFlag,
    ElseFlag,

    IfMultiCondition,

    Optional,

    Enum,
    EnumSelf,
    EnumUpcast,
    Flag,
    #[allow(unused)]
    SimpleDefinerTypes,

    WorldVersion,
    LoginVersion,
}

pub(crate) struct ImplFeatures {
    inner: Vec<Feature>,
}

impl ImplFeatures {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn add(&mut self, f: Feature) {
        if !self.inner.contains(&f) {
            self.inner.push(f);
        }
    }

    pub(crate) fn to_array(&self) -> Vec<Feature> {
        self.inner.clone()
    }
}

impl Default for ImplFeatures {
    fn default() -> Self {
        Self {
            inner: Vec::with_capacity(16),
        }
    }
}

pub(crate) fn get_impl_features_for_definer(e: &Definer) -> ImplFeatures {
    let mut f = ImplFeatures::new();
    match e.definer_ty() {
        DefinerType::Enum => f.add(Feature::Enum),
        DefinerType::Flag => f.add(Feature::Flag),
    };

    if e.self_value().is_some() {
        f.add(Feature::EnumSelf);
    }

    add_version(e.tags(), &mut f);

    f
}

pub(crate) fn get_impl_features_for_container(e: &Container) -> ImplFeatures {
    let mut f = ImplFeatures::new();

    match e.container_type() {
        ContainerType::Struct => f.add(Feature::Struct),
        ContainerType::CLogin(_) => f.add(Feature::CLogin),
        ContainerType::SLogin(_) => f.add(Feature::SLogin),
        ContainerType::Msg(_) => f.add(Feature::Msg),
        ContainerType::CMsg(_) => f.add(Feature::CMsg),
        ContainerType::SMsg(_) => f.add(Feature::SMsg),
    };

    if e.members().is_empty() {
        f.add(Feature::EmptyContainer);
    }

    for m in e.members() {
        features_for_struct_member(&mut f, m);
    }

    add_version(e.tags(), &mut f);

    f
}

fn features_for_struct_member(f: &mut ImplFeatures, m: &StructMember) {
    match m {
        StructMember::Definition(d) => features_for_definition(f, d),
        StructMember::IfStatement(statement) => {
            let ty = statement.definer_type();
            f.add(match ty {
                DefinerType::Enum => {
                    if statement.is_not_enum() {
                        Feature::IfNotEnum
                    } else {
                        Feature::IfEnum
                    }
                }
                DefinerType::Flag => Feature::IfFlag,
            });

            if statement.conditional().equations().len() > 1 {
                f.add(Feature::IfMultiCondition);
            }

            for m in statement.members() {
                features_for_struct_member(f, m);
            }

            for elseif in statement.else_ifs() {
                f.add(match ty {
                    DefinerType::Enum => Feature::IfElseEnum,
                    DefinerType::Flag => Feature::IfElseFlag,
                });
                for m in elseif.members() {
                    features_for_struct_member(f, m);
                }
            }

            for m in statement.else_members() {
                f.add(match ty {
                    DefinerType::Enum => {
                        if statement.is_not_enum() {
                            Feature::NotElseEnum
                        } else {
                            Feature::ElseEnum
                        }
                    }
                    DefinerType::Flag => Feature::ElseFlag,
                });

                features_for_struct_member(f, m);
            }
        }
        StructMember::OptionalStatement(optional) => {
            f.add(Feature::Optional);

            for m in optional.members() {
                features_for_struct_member(f, m);
            }
        }
    }
}

fn features_for_definition(f: &mut ImplFeatures, d: &StructMemberDefinition) {
    match d.ty() {
        Type::Seconds
        | Type::Milliseconds
        | Type::IpAddress
        | Type::Level16
        | Type::Level32
        | Type::Level
        | Type::Gold
        | Type::DateTime
        | Type::Bool(_)
        | Type::Integer(_)
        | Type::FloatingPoint => {
            f.add(Feature::SimpleBuiltInTypes);

            if let Some(v) = d.value() {
                if v.original_string() == CONTAINER_SELF_SIZE_FIELD {
                    f.add(Feature::ManualSizeField);
                }
                f.add(Feature::ConstantValue);
            }
        }
        Type::SizedCString | Type::Guid | Type::CString | Type::String { .. } => {
            f.add(Feature::ComplexBuiltInTypes)
        }
        Type::UpdateMask { .. } => f.add(Feature::UpdateMaskTypes),
        Type::AuraMask => f.add(Feature::AuraMaskTypes),
        Type::PackedGuid => f.add(Feature::PackedGuidTypes),
        Type::NamedGuid => f.add(Feature::NamedGuidTypes),
        Type::Array(array) => {
            match array.size() {
                ArraySize::Fixed(_) => f.add(Feature::FixedArrays),
                ArraySize::Variable(_) => f.add(Feature::VariableArrays),
                ArraySize::Endless => f.add(Feature::EndlessArrays),
            }
            match array.ty() {
                ArrayType::Integer(_) => f.add(Feature::SimpleArrays),
                ArrayType::Struct(_)
                | ArrayType::CString
                | ArrayType::Guid
                | ArrayType::PackedGuid => f.add(Feature::ComplexArrays),
            }
        }
        Type::Enum { upcast, .. } | Type::Flag { upcast, .. } => {
            if upcast.is_some() {
                f.add(Feature::EnumUpcast);
            }

            f.add(Feature::SubObjects);
        }
        Type::Struct { .. } => {
            f.add(Feature::SubObjects);
        }
        Type::AchievementDoneArray => f.add(Feature::AchievementDoneArrayType),
        Type::AchievementInProgressArray => f.add(Feature::AchievementInProgressType),
        Type::MonsterMoveSplines => f.add(Feature::MonsterMoveSpline),
        Type::EnchantMask => f.add(Feature::EnchantMaskTypes),
        Type::InspectTalentGearMask => f.add(Feature::InspectTalentGearMaskTypes),
        Type::VariableItemRandomProperty => f.add(Feature::VariableItemRandomPropertyTypes),
        Type::AddonArray => f.add(Feature::AddonArrayType),
    }
}

fn add_version(tags: &ObjectTags, f: &mut ImplFeatures) {
    if tags.has_world_version() {
        f.add(Feature::WorldVersion);
    } else {
        f.add(Feature::LoginVersion);
    }
}
