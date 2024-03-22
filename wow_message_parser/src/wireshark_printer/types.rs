use crate::parser::types::array::ArrayType;
use crate::parser::types::container::Container;
use crate::parser::types::definer::Definer;
use crate::parser::types::ty::Type;
use crate::parser::types::IntegerType;
use crate::wireshark_printer;

pub(crate) fn get_wireshark_object(containers: Vec<&Container>) -> WiresharkObject {
    let mut objects = WiresharkObject::new();

    let decompressed_size_ty = Type::Integer(IntegerType::U32);
    let decompressed_size_name = "decompressed_size";
    objects.push(WiresharkMember::new(
        wireshark_printer::name_to_hf(
            decompressed_size_name,
            &decompressed_size_ty,
            containers[0].tags().has_login_version(),
        ),
        WiresharkType::from_type(&decompressed_size_ty).unwrap(),
    ));

    for e in containers {
        if e.tags().used_in_update_mask() {
            continue;
        }

        for d in e.all_definitions() {
            let name =
                wireshark_printer::name_to_hf(d.name(), d.ty(), e.tags().has_login_version());

            if let Some(m) = objects.get_mut(&name) {
                if let Some(new_ty) = WiresharkType::from_type(d.ty()) {
                    match m.ty_mut() {
                        WiresharkType::Integer(i) => {
                            let v = match new_ty {
                                WiresharkType::Integer(v) => v,
                                _ => panic!("variable: '{name}' is int and {new_ty:#?}"),
                            };

                            if v > *i {
                                *i = v;
                            }
                        }
                        WiresharkType::Enum(e, i) => match new_ty {
                            WiresharkType::Enum(v, new) => {
                                if *i < new {
                                    *i = new;
                                }
                                if !e.tags().has_login_version() {
                                    assert_eq!(*e, v);
                                }
                            }
                            _ => panic!(
                                "name '{}' m.ty() is '{:#?}' and new_ty is '{:#?}'",
                                name,
                                m.ty(),
                                new_ty
                            ),
                        },
                        WiresharkType::Flag(e) => match new_ty {
                            WiresharkType::Flag(v) => {
                                if e.tags().has_login_version()
                                    && e.tags().all_versions() < v.tags().all_versions()
                                {
                                    *e = v;
                                } else if !e.tags().has_login_version() {
                                    assert_eq!(*e, v);
                                }
                            }
                            _ => panic!(
                                "name '{}' m.ty() is '{:#?}' and new_ty is '{:#?}'",
                                name,
                                m.ty(),
                                new_ty
                            ),
                        },
                        WiresharkType::Float | WiresharkType::Bytes | WiresharkType::String => {
                            assert_eq!(m.ty(), &new_ty)
                        }
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
        if let Some(p) = self.name.strip_prefix("hf_woww_") {
            p
        } else {
            self.name.strip_prefix("hf_wow_").unwrap()
        }
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
    Float,
    Bytes,
    Enum(Definer, IntegerType),
    Flag(Definer),
}

impl WiresharkType {
    pub(crate) fn from_type(ty: &Type) -> Option<Self> {
        Some(match ty {
            Type::Enum { e, upcast } => {
                // There is a flag with the same enumerators
                if e.name() == "SecurityFlag" {
                    return None;
                } else {
                    Self::Enum(e.clone(), upcast.unwrap_or(*e.ty()))
                }
            }
            Type::Flag { e, .. } => Self::Flag(e.clone()),
            Type::Struct { .. } => return None,
            Type::Integer(v) => Self::Integer(*v),
            Type::Bool(i) => Self::Integer(*i),
            Type::Guid | Type::PackedGuid => Self::Integer(IntegerType::U64),
            Type::Level => Self::Integer(IntegerType::U8),
            Type::Spell16 | Type::Level16 => Self::Integer(IntegerType::U16),
            Type::Spell | Type::Item | Type::Level32 => Self::Integer(IntegerType::U32),
            Type::Seconds | Type::Milliseconds | Type::Gold => Self::Integer(IntegerType::U32),
            Type::DateTime => Self::Integer(IntegerType::U32),
            Type::FloatingPoint => Self::Float,
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
                ArrayType::Spell => Self::Integer(IntegerType::U32),
                ArrayType::CString => Self::String,
                ArrayType::PackedGuid | ArrayType::Guid => Self::Integer(IntegerType::U64),
            },
            Type::IpAddress => Self::Integer(IntegerType::U32),
            Type::Population => Self::Float,
            Type::MonsterMoveSplines | Type::AuraMask | Type::UpdateMask { .. } => return None,
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
            Type::CacheMask => {
                unreachable!("cache mask only in wrath")
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
            WiresharkType::Float => "FT_FLOAT".to_string(),
            WiresharkType::Bytes => "FT_BYTES".to_string(),
        }
    }

    pub(crate) fn wireshark_base(&self) -> String {
        match self {
            WiresharkType::Float | WiresharkType::Bytes | WiresharkType::String => {
                "BASE_NONE".to_string()
            }
            WiresharkType::Integer(i) => match i {
                IntegerType::I32 => "BASE_DEC",
                _ => "BASE_HEX_DEC",
            }
            .to_string(),
            WiresharkType::Enum(_, IntegerType::U64 | IntegerType::I64) => {
                "BASE_HEX_DEC | BASE_VAL64_STRING".to_string()
            }
            WiresharkType::Enum(_, _) | WiresharkType::Flag(_) => "BASE_HEX_DEC".to_string(),
        }
    }
}

fn int_type_to_wireshark_string(i: &IntegerType) -> String {
    match i {
        IntegerType::U8 => "FT_UINT8",
        IntegerType::U16 => "FT_UINT16",
        IntegerType::U32 => "FT_UINT32",
        IntegerType::U48 | IntegerType::U64 => "FT_UINT64",
        IntegerType::I32 => "FT_INT32",
        IntegerType::I8 => "FT_INT8",
        IntegerType::I16 => "FT_INT16",
        IntegerType::I64 => "FT_INT64",
    }
    .to_string()
}
