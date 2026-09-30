use std::fmt::{Display, Formatter};

use crate::parser::types::array::{Array, ArraySize, ArrayType};
use crate::parser::types::if_statement::{Equation, IfStatement};
use crate::parser::types::sizes::Sizes;
use crate::parser::types::struct_member::{StructMember, StructMemberDefinition};
use crate::parser::types::ty::Type;
use crate::parser::types::IntegerType;
use crate::rust_printer::rust_view::rust_enumerator::RustEnumerator;
use crate::rust_printer::rust_view::rust_object::RustObject;

#[cfg(test)]
mod tests;

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
        is_single_rust_definer: bool,
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
    MonsterMoveSpline(MonsterMoveSplineEncoding),
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

/// Typed wire layout for the Wrath full-versus-linear spline schema choice.
///
/// The generated Rust API exposes both branches as one `Vec<Vector3d>` field, so codegen retains
/// branch types and discriminator while building that shared field.
pub(crate) struct MonsterMoveSplineLayout<'a> {
    full: &'a StructMemberDefinition,
    linear: &'a StructMemberDefinition,
    variable_name: &'a str,
    flags: &'a [String],
}

impl<'a> MonsterMoveSplineLayout<'a> {
    pub(crate) fn from_if_statement(
        statement: &'a IfStatement,
    ) -> Result<Option<Self>, UnsupportedMonsterMoveSplineLayout> {
        let Equation::BitwiseAnd { values } = statement.equation() else {
            return Ok(None);
        };
        let has_spline_member = statement.all_definitions().iter().any(|definition| {
            matches!(
                definition.ty(),
                Type::FullMonsterMoveSpline | Type::MonsterMoveSplines
            )
        });
        let is_spline_choice = has_spline_member
            && values.iter().any(|value| value == "FLYING")
            && values.iter().any(|value| value == "CATMULLROM");
        if !is_spline_choice {
            return Ok(None);
        }

        if !is_spline_flag_type(statement.original_ty())
            || values.len() != 2
            || !statement.else_ifs().is_empty()
        {
            return Err(UnsupportedMonsterMoveSplineLayout);
        }

        fn definition(
            members: &[StructMember],
            ty: fn(&Type) -> bool,
        ) -> Option<&StructMemberDefinition> {
            let [StructMember::Definition(definition)] = members else {
                return None;
            };
            ty(definition.ty()).then_some(definition)
        }

        let full = definition(statement.members(), |ty| {
            matches!(ty, Type::FullMonsterMoveSpline)
        })
        .ok_or(UnsupportedMonsterMoveSplineLayout)?;
        let linear = definition(statement.else_members(), |ty| {
            matches!(ty, Type::MonsterMoveSplines)
        })
        .ok_or(UnsupportedMonsterMoveSplineLayout)?;
        Ok(Some(Self {
            full,
            linear,
            variable_name: statement.variable_name(),
            flags: values,
        }))
    }

    pub(crate) fn full(&self) -> &StructMemberDefinition {
        self.full
    }

    pub(crate) fn linear(&self) -> &StructMemberDefinition {
        self.linear
    }

    pub(crate) fn variable_name(&self) -> &str {
        self.variable_name
    }

    pub(crate) fn flags(&self) -> &[String] {
        self.flags
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct UnsupportedMonsterMoveSplineLayout;

fn is_spline_flag_type(ty: &Type) -> bool {
    matches!(
        ty,
        Type::Flag { e, upcast: None }
            if e.name() == "SplineFlag" && e.ty() == &IntegerType::U32
    )
}

#[derive(Debug, Clone)]
pub(crate) enum MonsterMoveSplineEncoding {
    Legacy,
    Linear,
    Full,
    FullWhenFlagsSet {
        variable_name: String,
        flags: Vec<String>,
        full_field_name: String,
    },
}

impl MonsterMoveSplineEncoding {
    pub(crate) fn from_layout(layout: &MonsterMoveSplineLayout<'_>) -> Self {
        Self::FullWhenFlagsSet {
            variable_name: layout.variable_name().to_string(),
            flags: layout.flags().to_vec(),
            full_field_name: layout.full().name().to_string(),
        }
    }

    pub(crate) fn test_case_alias(&self) -> Option<&str> {
        match self {
            Self::FullWhenFlagsSet {
                full_field_name, ..
            } => Some(full_field_name),
            Self::Legacy | Self::Linear | Self::Full => None,
        }
    }

    pub(crate) fn condition_expression(&self, prefix: &str, accessor: &str) -> Option<String> {
        let Self::FullWhenFlagsSet {
            variable_name,
            flags,
            ..
        } = self
        else {
            return None;
        };

        Some(Self::flags_expression(
            variable_name,
            flags,
            prefix,
            accessor,
        ))
    }

    fn flags_expression(
        variable_name: &str,
        flags: &[String],
        prefix: &str,
        accessor: &str,
    ) -> String {
        flags
            .iter()
            .map(|flag| {
                format!(
                    "{prefix}{variable_name}.{accessor}{}()",
                    flag.to_lowercase()
                )
            })
            .collect::<Vec<_>>()
            .join(" || ")
    }

    pub(crate) fn size_expression(&self, prefix: &str, name: &str) -> String {
        match self {
            Self::Legacy => {
                format!("crate::util::monster_move_spline_size({prefix}{name}.as_slice())")
            }
            Self::Linear => format!(
                "crate::util::wrath_monster_move_spline_size({prefix}{name}.as_slice(), false)"
            ),
            Self::Full => format!(
                "crate::util::wrath_monster_move_spline_size({prefix}{name}.as_slice(), true)"
            ),
            Self::FullWhenFlagsSet {
                variable_name,
                flags,
                ..
            } => format!(
                "crate::util::wrath_monster_move_spline_size({prefix}{name}.as_slice(), {})",
                Self::flags_expression(variable_name, flags, prefix, "get_")
            ),
        }
    }

    fn is_const_size(&self) -> bool {
        !matches!(self, Self::Legacy)
    }

    pub(crate) fn is_wrath(&self) -> bool {
        !matches!(self, Self::Legacy)
    }
}

impl RustType {
    pub(crate) fn is_constant(&self) -> Option<i128> {
        match self {
            RustType::Enum {
                int_ty,
                enumerators,
                ..
            }
            | RustType::Flag {
                int_ty,
                enumerators,
                ..
            } => {
                let mut size = 0;

                for enumerator in enumerators {
                    if let Some(i) = enumerator.is_constant() {
                        if i > size {
                            size = i;
                        }
                    } else {
                        return None;
                    }
                }

                Some(size + int_ty.sizes().is_constant().unwrap())
            }
            RustType::Struct { sizes, .. } => sizes.is_constant(),
            _ => self.to_type().sizes().is_constant(),
        }
    }

    pub(crate) fn str(&self) -> String {
        match self {
            RustType::Array { array, .. } => array.str(),
            RustType::Flag { ty_name, .. } | RustType::Enum { ty_name, .. } => ty_name.clone(),
            RustType::Struct { ty_name, .. } => ty_name.clone(),
            RustType::MonsterMoveSpline(MonsterMoveSplineEncoding::Full) => {
                Type::FullMonsterMoveSpline.str()
            }
            RustType::MonsterMoveSpline(_) => Type::MonsterMoveSplines.str(),
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
            RustType::MonsterMoveSpline(MonsterMoveSplineEncoding::Full) => {
                Type::FullMonsterMoveSpline
            }
            RustType::MonsterMoveSpline(_) => Type::MonsterMoveSplines,
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
            RustType::Array { array, .. } => Type::Array(array.clone()),

            RustType::Enum { .. } | RustType::Flag { .. } | RustType::Struct { .. } => {
                panic!("invalid conversion")
            }
            RustType::Population => Type::Population,
            RustType::Spell => Type::Spell,
            RustType::Spell16 => Type::Spell16,
            RustType::Item => Type::Item,
            RustType::CacheMask => Type::CacheMask,
        }
    }

    pub(crate) fn test_case_alias(&self) -> Option<&str> {
        match self {
            RustType::MonsterMoveSpline(encoding) => encoding.test_case_alias(),
            _ => None,
        }
    }

    pub(crate) fn size_requires_variable(&self) -> bool {
        match self {
            RustType::Spell
            | RustType::Spell16
            | RustType::Item
            | RustType::Population
            | RustType::Integer(_)
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

            RustType::CacheMask
            | RustType::UpdateMask { .. }
            | RustType::AuraMask
            | RustType::NamedGuid
            | RustType::PackedGuid
            | RustType::String
            | RustType::CString
            | RustType::SizedCString
            | RustType::MonsterMoveSpline(_)
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
                    ArrayType::Spell
                    | ArrayType::Integer(_)
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
            | RustType::SizedCString => false,

            RustType::MonsterMoveSpline(encoding) => encoding.is_const_size(),

            _ => true,
        }
    }
}

impl Display for RustType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.rust_str())
    }
}
