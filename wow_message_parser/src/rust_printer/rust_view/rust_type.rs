use std::fmt::{Display, Formatter};
use crate::parser::types::array::{Array, ArraySize};
use crate::parser::types::IntegerType;
use crate::parser::types::sizes::Sizes;
use crate::parser::types::ty::Type;
use crate::rust_printer::rust_view::rust_object::RustObject;
use crate::rust_printer::rust_view::rust_enumerator::RustEnumerator;

#[derive(Debug, Clone)]
pub(crate) enum RustType {
    Integer(IntegerType),
    Bool(IntegerType),
    DateTime,
    Floating,
    UpdateMask {
        max_size: usize,
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
        inner_object: Option<RustObject>,
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

            RustType::Array { .. } |
            RustType::Enum { .. } | RustType::Flag { .. } | RustType::Struct { .. } => {
                panic!("invalid conversion")
            }
        }
    }

    pub(crate) fn size_is_const_fn(&self) -> bool {
        match self {
            RustType::Array {
                array,
                inner_sizes,
                inner_object,
            } => {
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
