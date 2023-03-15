use crate::parser::types::array::ArrayType;
use crate::parser::types::definer::Definer;
use crate::parser::types::ty::Type;
use crate::parser::types::Endianness::Little;
use crate::parser::types::{Endianness, FloatingPointType, IntegerType};
use crate::{wireshark_printer, Objects};

pub(crate) fn get_wireshark_object(o: &Objects) -> WiresharkObject {
    let mut objects = WiresharkObject::new();

    for e in o.wireshark_containers() {
        for d in e.all_definitions() {
            let name = wireshark_printer::name_to_hf(d.name(), d.ty());

            if let Some(m) = objects.get_mut(&name) {
                if let Some(new_ty) = &WiresharkType::from_type(d.ty()) {
                    match m.ty_mut() {
                        WiresharkType::Integer(i) => {
                            let v = match new_ty {
                                WiresharkType::Integer(v) => v,
                                _ => panic!("variable: '{name}' is int and {new_ty:#?}"),
                            };

                            if v > i {
                                *i = *v;
                            }
                        }
                        WiresharkType::Float(i) => {
                            let v = match new_ty {
                                WiresharkType::Float(v) => v,
                                _ => panic!("variable: '{name}' is float and {new_ty:#?}"),
                            };

                            if *v < *i {
                                *i = *v;
                            }
                        }
                        WiresharkType::Enum(e, i) => match new_ty {
                            WiresharkType::Enum(v, new) => {
                                if &*i < new {
                                    *i = *new;
                                }
                                assert_eq!(e, v);
                            }
                            _ => panic!(
                                "name '{}' m.ty() is '{:?}' and new_ty is '{:?}'",
                                name,
                                m.ty(),
                                new_ty
                            ),
                        },
                        WiresharkType::Flag(e) => match new_ty {
                            WiresharkType::Flag(v) => {
                                assert_eq!(e, v);
                            }
                            _ => panic!(
                                "name '{}' m.ty() is '{:?}' and new_ty is '{:?}'",
                                name,
                                m.ty(),
                                new_ty
                            ),
                        },
                        WiresharkType::Bytes | WiresharkType::String => assert_eq!(m.ty(), new_ty),
                    }
                }
            } else if let Some(ty) = WiresharkType::from_type(d.ty()) {
                match &ty {
                    WiresharkType::Enum(e, _) => objects.add_enum(e.clone()),
                    WiresharkType::Flag(e) => objects.add_flag(e.clone()),
                    _ => {}
                }
                objects.push(WiresharkMember::new(name, ty));
            }
        }
    }

    objects.members.sort_by(|a, b| a.name().cmp(b.name()));

    objects
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct WiresharkObject {
    members: Vec<WiresharkMember>,
    enums: Vec<Definer>,
    flags: Vec<Definer>,
}

impl WiresharkObject {
    pub(crate) fn new() -> Self {
        Self {
            members: vec![],
            enums: vec![],
            flags: vec![],
        }
    }

    pub(crate) fn get(&self, name: &str) -> Option<&WiresharkMember> {
        self.members.iter().find(|a| a.name == name)
    }

    pub(crate) fn get_mut(&mut self, name: &str) -> Option<&mut WiresharkMember> {
        self.members.iter_mut().find(|a| a.name == name)
    }

    pub(crate) fn push(&mut self, m: WiresharkMember) {
        self.members.push(m);
    }

    pub(crate) fn add_enum(&mut self, e: Definer) {
        self.enums.push(e);
    }

    pub(crate) fn add_flag(&mut self, e: Definer) {
        self.flags.push(e);
    }

    pub(crate) fn members(&self) -> &[WiresharkMember] {
        &self.members
    }

    pub(crate) fn enums(&self) -> &[Definer] {
        &self.enums
    }

    pub(crate) fn flags(&self) -> &[Definer] {
        &self.flags
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct WiresharkMember {
    name: String,
    ty: WiresharkType,
}

impl WiresharkMember {
    pub(crate) fn new(name: String, ty: WiresharkType) -> Self {
        Self { name, ty }
    }
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn name_without_hf_woww(&self) -> &str {
        self.name.strip_prefix("hf_woww_").unwrap()
    }
    pub(crate) fn ty(&self) -> &WiresharkType {
        &self.ty
    }
    pub(crate) fn ty_mut(&mut self) -> &mut WiresharkType {
        &mut self.ty
    }

    pub(crate) fn has_enum_strings(&self) -> Option<&Definer> {
        match self.ty() {
            WiresharkType::Enum(e, _) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum WiresharkType {
    Integer(IntegerType),
    String,
    Float(FloatingPointType),
    Bytes,
    Enum(Definer, IntegerType),
    Flag(Definer),
}

impl WiresharkType {
    pub(crate) fn from_type(ty: &Type) -> Option<Self> {
        Some(match ty {
            Type::Enum { e, upcast } => Self::Enum(e.clone(), upcast.unwrap_or(*e.ty())),
            Type::Flag { e, .. } => Self::Flag(e.clone()),
            Type::Struct { .. } => return None,
            Type::Integer(v) => Self::Integer(*v),
            Type::Bool(i) => Self::Integer(*i),
            Type::Guid | Type::PackedGuid => Self::Integer(IntegerType::U64(Endianness::Little)),
            Type::Level => Self::Integer(IntegerType::U8),
            Type::Level16 => Self::Integer(IntegerType::U16(Endianness::Little)),
            Type::Level32 => Self::Integer(IntegerType::U32(Endianness::Little)),
            Type::Gold => Self::Integer(IntegerType::U32(Endianness::Little)),
            Type::DateTime => Self::Integer(IntegerType::U32(Endianness::Little)),
            Type::FloatingPoint(v) => Self::Float(*v),
            Type::SizedCString | Type::String { .. } | Type::CString => Self::String,
            Type::Array(array) => match array.ty() {
                ArrayType::Struct(_) => {
                    return None;
                }
                ArrayType::Integer(v) => {
                    if v == &IntegerType::U8 {
                        Self::Bytes
                    } else {
                        Self::Integer(*v)
                    }
                }
                ArrayType::CString => Self::String,
                ArrayType::PackedGuid | ArrayType::Guid => Self::Integer(IntegerType::U64(Little)),
            },
            Type::MonsterMoveSplines | Type::AuraMask | Type::UpdateMask => return None,
            Type::AchievementDoneArray | Type::AchievementInProgressArray => {
                unreachable!("achievement arrays are only in 3.3.5")
            }
            Type::EnchantMask => {
                unreachable!("enchant mask is only in 3.3.5")
            }
            Type::InspectTalentGearMask => {
                unreachable!("inspect talent gear mask is only in 3.3.5")
            }
            Type::NamedGuid => {
                unreachable!("named guid only in 2.4.3 and 3.3.5")
            }
            Type::VariableItemRandomProperty => {
                unreachable!("variable item random property id only in 2.4.3 and 3.3.5")
            }
            Type::AddonArray => {
                unreachable!("addon array only in 2.4.3/3.3.5")
            }
        })
    }

    pub(crate) fn wireshark_str(&self) -> String {
        match self {
            WiresharkType::Integer(i) => int_type_to_wireshark_string(i),
            WiresharkType::Enum(_, upcast) => {
                let i = upcast;
                int_type_to_wireshark_string(i)
            }
            WiresharkType::Flag(e) => {
                let i = e.ty();
                int_type_to_wireshark_string(i)
            }
            WiresharkType::String => "FT_STRINGZ".to_string(),
            WiresharkType::Float(_) => "FT_FLOAT".to_string(),
            WiresharkType::Bytes => "FT_BYTES".to_string(),
        }
    }

    pub(crate) fn wireshark_base(&self) -> String {
        match self {
            WiresharkType::Float(_) | WiresharkType::Bytes | WiresharkType::String => {
                "BASE_NONE".to_string()
            }
            WiresharkType::Integer(i) => match i {
                IntegerType::I32(_) => "BASE_DEC",
                _ => "BASE_HEX_DEC",
            }
            .to_string(),
            WiresharkType::Enum(_, IntegerType::U64(_) | IntegerType::I64(_)) => {
                "BASE_HEX_DEC | BASE_VAL64_STRING".to_string()
            }
            WiresharkType::Enum(_, _) | WiresharkType::Flag(_) => "BASE_HEX_DEC".to_string(),
        }
    }
}

fn int_type_to_wireshark_string(i: &IntegerType) -> String {
    match i {
        IntegerType::U8 => "FT_UINT8",
        IntegerType::U16(_) => "FT_UINT16",
        IntegerType::U32(_) => "FT_UINT32",
        IntegerType::U48 | IntegerType::U64(_) => "FT_UINT64",
        IntegerType::I32(_) => "FT_INT32",
        IntegerType::I8 => "FT_INT8",
        IntegerType::I16(_) => "FT_INT16",
        IntegerType::I64(_) => "FT_INT64",
    }
    .to_string()
}
