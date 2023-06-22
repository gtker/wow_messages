use crate::parser::types::array::{Array, ArraySize, ArrayType};
use crate::parser::types::sizes::Sizes;
use crate::parser::types::ty::Type;
use crate::parser::types::IntegerType;
use crate::rust_printer::rust_view::rust_enumerator::RustEnumerator;
use crate::rust_printer::rust_view::rust_object::RustObject;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub(crate) enum RustType {
    Integer(IntegerType),
    Bool(IntegerType),
    DateTime,
    Floating,
    UpdateMask {
        max_size: i128,
    },
    AuraMask,
    Guid,
    NamedGuid,
    PackedGuid,
    String,
    CString,
    SizedCString,
    Array {
        array: Array,
        inner_sizes: Sizes,
    },
    Enum {
        ty_name: String,
        original_ty_name: String,
        enumerators: Vec<RustEnumerator>,
        int_ty: IntegerType,
        is_simple: bool,
        is_elseif: bool,
        separate_if_statements: bool,
    },
    Flag {
        ty_name: String,
        original_ty_name: String,
        int_ty: IntegerType,
        enumerators: Vec<RustEnumerator>,
        is_simple: bool,
        is_elseif: bool,
    },
    Struct {
        ty_name: String,
        sizes: Sizes,
        object: RustObject,
    },
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

impl RustType {
    pub(crate) fn str(&self) -> String {
        match self {
            RustType::Array { array, .. } => array.str(),
            RustType::Flag { ty_name, .. } | RustType::Enum { ty_name, .. } => ty_name.clone(),
            RustType::Struct { ty_name, .. } => ty_name.clone(),
            _ => self.to_type().str(),
        }
    }

    pub(crate) fn rust_str(&self) -> String {
        match self {
            RustType::Array { array, .. } => array.rust_str(),
            RustType::Enum { .. } | RustType::Flag { .. } | RustType::Struct { .. } => self.str(),

            _ => self.to_type().rust_str(),
        }
    }

    pub(crate) fn to_type(&self) -> Type {
        match self {
            RustType::Integer(i) => Type::Integer(*i),
            RustType::Bool(i) => Type::Bool(*i),
            RustType::DateTime => Type::DateTime,
            RustType::Floating => Type::FloatingPoint,
            RustType::UpdateMask { max_size } => Type::UpdateMask {
                max_size: *max_size,
            },
            RustType::AuraMask => Type::AuraMask,
            RustType::Guid => Type::Guid,
            RustType::NamedGuid => Type::NamedGuid,
            RustType::PackedGuid => Type::PackedGuid,
            RustType::String => Type::String,
            RustType::CString => Type::CString,
            RustType::SizedCString => Type::SizedCString,
            RustType::MonsterMoveSpline => Type::MonsterMoveSplines,
            RustType::AchievementDoneArray => Type::AchievementDoneArray,
            RustType::AchievementInProgressArray => Type::AchievementInProgressArray,
            RustType::EnchantMask => Type::EnchantMask,
            RustType::InspectTalentGearMask => Type::InspectTalentGearMask,
            RustType::Gold => Type::Gold,
            RustType::Level => Type::Level,
            RustType::Level16 => Type::Level16,
            RustType::Level32 => Type::Level32,
            RustType::VariableItemRandomProperty => Type::VariableItemRandomProperty,
            RustType::AddonArray => Type::AddonArray,
            RustType::IpAddress => Type::IpAddress,
            RustType::Seconds => Type::Seconds,
            RustType::Milliseconds => Type::Milliseconds,

            RustType::Array { .. }
            | RustType::Enum { .. }
            | RustType::Flag { .. }
            | RustType::Struct { .. } => {
                panic!("invalid conversion")
            }
        }
    }

    pub(crate) fn size_requires_variable(&self) -> bool {
        match self {
            RustType::Integer(_)
            | RustType::Bool(_)
            | RustType::DateTime
            | RustType::Floating
            | RustType::Guid
            | RustType::IpAddress
            | RustType::Seconds
            | RustType::Milliseconds
            | RustType::Gold
            | RustType::Level
            | RustType::Level16
            | RustType::Level32 => false,

            RustType::UpdateMask { .. }
            | RustType::AuraMask
            | RustType::NamedGuid
            | RustType::PackedGuid
            | RustType::String
            | RustType::CString
            | RustType::SizedCString
            | RustType::MonsterMoveSpline
            | RustType::AchievementDoneArray
            | RustType::AchievementInProgressArray
            | RustType::EnchantMask
            | RustType::InspectTalentGearMask
            | RustType::VariableItemRandomProperty
            | RustType::AddonArray => true,

            RustType::Array { array, .. } => !array.is_constant(),

            RustType::Enum { is_simple, .. } | RustType::Flag { is_simple, .. } => !*is_simple,

            RustType::Struct { sizes, .. } => sizes.is_constant().is_none(),
        }
    }

    pub(crate) fn size_is_const_fn(&self) -> bool {
        match self {
            RustType::Array {
                array, inner_sizes, ..
            } => {
                let inner_object = match array.ty() {
                    ArrayType::Struct(c) => Some(c.rust_object()),
                    ArrayType::Integer(_)
                    | ArrayType::CString
                    | ArrayType::Guid
                    | ArrayType::PackedGuid => None,
                };

                let inner = if let Some(object) = inner_object {
                    object
                        .members_in_struct()
                        .all(|a| a.ty().size_is_const_fn())
                } else {
                    inner_sizes.is_constant().is_some()
                };

                match array.size() {
                    ArraySize::Fixed(_) => inner,
                    ArraySize::Variable(_) | ArraySize::Endless => false,
                }
            }

            RustType::Struct { object, .. } => object
                .members_in_struct()
                .all(|a| a.ty().size_is_const_fn()),

            RustType::Enum {
                is_simple,
                enumerators,
                ..
            }
            | RustType::Flag {
                is_simple,
                enumerators,
                ..
            } => {
                if *is_simple {
                    true
                } else {
                    enumerators
                        .iter()
                        .flat_map(|a| a.members_in_struct())
                        .all(|a| a.ty().size_is_const_fn())
                }
            }

            RustType::UpdateMask { .. }
            | RustType::NamedGuid
            | RustType::AchievementDoneArray
            | RustType::AchievementInProgressArray
            | RustType::VariableItemRandomProperty
            | RustType::AddonArray
            | RustType::String
            | RustType::CString
            | RustType::MonsterMoveSpline
            | RustType::SizedCString => false,

            _ => true,
        }
    }
}

impl Display for RustType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.rust_str())
    }
}
