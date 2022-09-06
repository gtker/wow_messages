use crate::file_utils::{overwrite_if_not_same_contents, UPDATE_MASK_LOCATION};
use crate::rust_printer::Writer;
use std::fmt::Write;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::path::Path;
use vanilla_fields::FIELDS;

pub mod vanilla_fields;

pub fn print_update_mask_docs() {
    const UPDATE_MASK_FILE: &str = "wowm_language/src/spec/update-mask.md";
    const LOOKUP_TABLE: &str = "## Lookup Table\n";
    let contents = read_to_string(UPDATE_MASK_FILE).unwrap();

    let update_types = [
        ("object", UpdateMaskType::Object),
        ("item", UpdateMaskType::Item),
        ("container", UpdateMaskType::Container),
        ("unit", UpdateMaskType::Unit),
        ("player", UpdateMaskType::Player),
        ("gameobject", UpdateMaskType::GameObject),
        ("dynamicobject", UpdateMaskType::DynamicObject),
        ("corpse", UpdateMaskType::Corpse),
    ];

    let (s, _) = contents.split_once(LOOKUP_TABLE).unwrap();
    let mut s = s.to_string();
    s.push_str(LOOKUP_TABLE);
    s.push_str("Taken from [vmangos](https://github.com/vmangos/core/blob/4b2a5173b0ca4917dfe91aa7b87d84232fd7203c/src/game/Objects/UpdateFields_1_12_1.cpp#L5) with some modifications.\n\n");

    for (name, u) in update_types {
        writeln!(s, "Fields that all {}s have:\n", name).unwrap();

        s.push_str("| Name | Offset | Size | Type |\n");
        s.push_str("|------|--------|------|------|\n");
        for field in FIELDS {
            if field.object_ty == u {
                let ty = match field.ty {
                    UfType::Guid => "GUID",
                    UfType::Int => "INT",
                    UfType::Float => "FLOAT",
                    UfType::BytesWith(_, _, _, _) | UfType::Bytes => "BYTES",
                    UfType::TwoShort => "TWO_SHORT",
                };

                writeln!(
                    s,
                    "|`{}_{}`| 0x{:04x?} | {} | {} |",
                    name.to_uppercase(),
                    field.name(),
                    field.offset,
                    field.size,
                    ty
                )
                .unwrap();

                if field.offset == 0 && field.ty == UfType::Guid {
                    writeln!(
                        s,
                        "|`{}_TYPE`| 0x{:04x?} | {} | INT |",
                        name.to_uppercase(),
                        2,
                        1,
                    )
                    .unwrap();
                }
            }
        }

        s.push_str("\n\n");
    }

    overwrite_if_not_same_contents(&s, Path::new(UPDATE_MASK_FILE));
}

pub fn print_update_mask() {
    print_update_mask_docs();

    let update_types = [
        (
            "UpdateItem",
            vec![UpdateMaskType::Object, UpdateMaskType::Item],
        ),
        (
            "UpdateContainer",
            vec![
                UpdateMaskType::Object,
                UpdateMaskType::Item,
                UpdateMaskType::Container,
            ],
        ),
        (
            "UpdateUnit",
            vec![UpdateMaskType::Object, UpdateMaskType::Unit],
        ),
        (
            "UpdatePlayer",
            vec![
                UpdateMaskType::Object,
                UpdateMaskType::Unit,
                UpdateMaskType::Player,
            ],
        ),
        (
            "UpdateGameObject",
            vec![UpdateMaskType::Object, UpdateMaskType::GameObject],
        ),
        (
            "UpdateDynamicObject",
            vec![UpdateMaskType::Object, UpdateMaskType::DynamicObject],
        ),
        (
            "UpdateCorpse",
            vec![UpdateMaskType::Object, UpdateMaskType::Corpse],
        ),
    ];

    let mut s = Writer::new("");
    s.wln("use crate::Guid;");
    for m in FIELDS {
        if let UfType::BytesWith(a, b, c, d) = m.ty() {
            for ty in [a, b, c, d] {
                match ty.ty {
                    ByteInnerTy::Byte => {}
                    ByteInnerTy::Ty(ty) => {
                        s.wln(format!("use crate::vanilla::{{{}}};", ty));
                    }
                }
            }
        }
    }

    s.wln("use crate::vanilla::{UpdateContainer, UpdateCorpse, UpdateDynamicObject, UpdateGameObject, UpdateItem, UpdateMask, UpdatePlayer, UpdateUnit};");
    s.newline();

    for (ty, types) in update_types {
        s.bodyn(format!("impl {}", ty), |s| {
            for m in FIELDS.iter().filter(|a| types.contains(&a.object_ty)) {
                print_functions(s, m);
            }
        });
    }

    overwrite_if_not_same_contents(s.inner(), Path::new(UPDATE_MASK_LOCATION));
}

fn print_functions(s: &mut Writer, m: &MemberType) {
    s.open_curly(format!(
        "pub fn set_{}_{}(mut self, {}) -> Self",
        m.object_ty,
        m.name,
        m.ty.parameter_str(),
    ));

    s.wln(format!("self.header_set({});", m.offset));
    match m.ty {
        UfType::Guid => {
            s.wln(format!("self.header_set({});", m.offset + 1));

            s.wln(format!(
                "self.values.insert({}, v.guid() as u32);",
                m.offset
            ));
            s.wln(format!(
                "self.values.insert({}, (v.guid() >> 32) as u32);",
                m.offset + 1
            ));
        }
        _ => {
            let value = match &m.ty {
                UfType::Int => "v as u32".to_string(),
                UfType::Float => "u32::from_le_bytes(v.to_le_bytes())".to_string(),
                UfType::Bytes => "u32::from_le_bytes([a, b, c, d])".to_string(),
                UfType::TwoShort => "v".to_string(),
                UfType::BytesWith(a, b, c, d) => {
                    let get_name = |byte_type: &ByteType| -> String {
                        match byte_type.ty {
                            ByteInnerTy::Byte => byte_type.name.to_string(),
                            ByteInnerTy::Ty(_) => {
                                format!("{name}.as_int()", name = byte_type.name)
                            }
                        }
                    };

                    let a = get_name(a);
                    let b = get_name(b);
                    let c = get_name(c);
                    let d = get_name(d);

                    format!(
                        "u32::from_le_bytes([{a}, {b}, {c}, {d}])",
                        a = a,
                        b = b,
                        c = c,
                        d = d
                    )
                }
                _ => unreachable!(),
            };

            s.wln(format!("self.values.insert({}, {});", m.offset, value));
        }
    }

    s.wln("self");
    s.closing_curly_newline(); // pub fn set_
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum UpdateMaskType {
    Object,
    Item,
    Unit,
    Player,
    Container,
    GameObject,
    DynamicObject,
    Corpse,
}

impl Display for UpdateMaskType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            UpdateMaskType::Object => "object",
            UpdateMaskType::Item => "item",
            UpdateMaskType::Unit => "unit",
            UpdateMaskType::Player => "player",
            UpdateMaskType::Container => "container",
            UpdateMaskType::GameObject => "gameobject",
            UpdateMaskType::DynamicObject => "dynamicobject",
            UpdateMaskType::Corpse => "corpse",
        })
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ByteInnerTy {
    Byte,
    Ty(&'static str),
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ByteType {
    pub name: &'static str,
    pub ty: ByteInnerTy,
}

impl ByteType {
    pub const fn new(name: &'static str, ty: &'static str) -> Self {
        Self {
            name,
            ty: ByteInnerTy::Ty(ty),
        }
    }

    pub const fn byte(name: &'static str) -> Self {
        Self {
            name,
            ty: ByteInnerTy::Byte,
        }
    }

    pub const fn ty_str(&self) -> &'static str {
        match self.ty {
            ByteInnerTy::Byte => "u8",
            ByteInnerTy::Ty(s) => s,
        }
    }

    #[allow(unused)]
    pub const fn a() -> Self {
        Self {
            name: "a",
            ty: ByteInnerTy::Byte,
        }
    }
    #[allow(unused)]
    pub const fn b() -> Self {
        Self {
            name: "b",
            ty: ByteInnerTy::Byte,
        }
    }
    #[allow(unused)]
    pub const fn c() -> Self {
        Self {
            name: "c",
            ty: ByteInnerTy::Byte,
        }
    }
    #[allow(unused)]
    pub const fn d() -> Self {
        Self {
            name: "d",
            ty: ByteInnerTy::Byte,
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum UfType {
    Guid,
    Int,
    Float,
    Bytes,
    BytesWith(ByteType, ByteType, ByteType, ByteType),
    TwoShort,
}

const GUID_TYPE: &str = "Guid";
const INT_TYPE: &str = "i32";
const FLOAT_TYPE: &str = "f32";
const TWO_SHORT_TYPE: &str = "u32";

impl UfType {
    pub fn parameter_str(&self) -> String {
        format!(
            "v: {}",
            match self {
                UfType::Guid => GUID_TYPE,
                UfType::Int => INT_TYPE,
                UfType::Float => FLOAT_TYPE,
                UfType::TwoShort => TWO_SHORT_TYPE,
                UfType::Bytes => {
                    return "a: u8, b: u8, c: u8, d: u8".to_string();
                }
                UfType::BytesWith(a, b, c, d) => {
                    return format!(
                        "{}: {}, {}: {}, {}: {}, {}: {}",
                        a.name,
                        a.ty_str(),
                        b.name,
                        b.ty_str(),
                        c.name,
                        c.ty_str(),
                        d.name,
                        d.ty_str(),
                    );
                }
            }
        )
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct MemberType {
    object_ty: UpdateMaskType,
    name: &'static str,
    offset: i32,
    size: i32,
    ty: UfType,
}

impl MemberType {
    const fn new(ty: UpdateMaskType, s: &'static str, offset: i32, size: i32, uf: UfType) -> Self {
        Self {
            object_ty: ty,
            name: s,
            offset,
            size,
            ty: uf,
        }
    }
    pub fn object_ty(&self) -> UpdateMaskType {
        self.object_ty
    }
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn ty(&self) -> UfType {
        self.ty.clone()
    }
}
