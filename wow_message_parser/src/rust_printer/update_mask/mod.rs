use crate::file_utils::overwrite_if_not_same_contents;
use crate::parser::types::version::MajorWorldVersion;
use crate::path_utils::{update_mask_doc_file, update_mask_index_location, update_mask_location};
use crate::rust_printer::Writer;
use std::fmt::Write;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;

pub mod tbc_fields;
pub mod vanilla_fields;
pub mod wrath_fields;

fn print_specific_update_mask_doc(fields: &[MemberType], s: &mut String) {
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

    for (name, u) in update_types {
        writeln!(s, "Fields that all {name}s have:\n").unwrap();

        s.push_str("| Name | Offset | Size | Type |\n");
        s.push_str("|------|--------|------|------|\n");
        for field in fields {
            if field.object_ty == u {
                let ty = match field.ty {
                    UfType::Guid => "GUID",
                    UfType::Int => "INT",
                    UfType::Float => "FLOAT",
                    UfType::BytesWith(_, _, _, _) | UfType::Bytes => "BYTES",
                    UfType::TwoShort => "TWO_SHORT",
                    UfType::GuidEnumLookupArray { .. } | UfType::ArrayOfStruct { .. } => "CUSTOM",
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
}

pub(crate) fn print_update_mask_docs() {
    const LOOKUP_TABLE: &str = "## Lookup Table";
    let contents = read_to_string(update_mask_doc_file()).unwrap();

    let (s, _) = contents.split_once(LOOKUP_TABLE).unwrap();
    let mut s = s.to_string();
    s.push_str(LOOKUP_TABLE);
    s.push('\n');
    s.push_str("### Version 1.12\n\n");
    s.push_str("Taken from [vmangos](https://github.com/vmangos/core/blob/4b2a5173b0ca4917dfe91aa7b87d84232fd7203c/src/game/Objects/UpdateFields_1_12_1.cpp#L5) with some modifications.\n\n");

    print_specific_update_mask_doc(vanilla_fields::FIELDS, &mut s);

    s.push_str("### Version 2.4.3\n\n");
    s.push_str("Taken from [mangosone](https://github.com/mangosone/server/blob/f441fc27a6430e79753bceb545f62fef90a79832/src/game/Object/UpdateFields.h#L30) with some modifications.\n\n");

    print_specific_update_mask_doc(tbc_fields::FIELDS, &mut s);

    s.push_str("### Version 3.3.5\n\n");
    s.push_str("Taken from [ArcEmu](https://github.com/arcemu/arcemu/blob/1cb2b5248d050cb6fe413d7c42dd1817994b6366/src/world/Game/Entities/Update/UpdateFields.h#L26) with some modifications.\n\n");

    print_specific_update_mask_doc(wrath_fields::FIELDS, &mut s);

    overwrite_if_not_same_contents(&s, &update_mask_doc_file());
}

fn print_specific_update_mask(fields: &[MemberType], version: MajorWorldVersion) -> Writer {
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

    let mut s = Writer::new();
    s.wln("use crate::Guid;");
    s.wln("use std::convert::TryInto;");
    s.wln("use super::indices::*;");
    for m in fields {
        if let UfType::BytesWith(a, b, c, d) = m.ty() {
            for ty in [a, b, c, d] {
                match ty.ty {
                    ByteInnerTy::Byte => {}
                    ByteInnerTy::Ty(ty) => {
                        s.wln(format!("use crate::{}::{{{}}};", version.module_name(), ty));
                    }
                }
            }
        }
    }

    s.wln(format!("use crate::{}::{{UpdateContainer, UpdateContainerBuilder, UpdateCorpse, UpdateCorpseBuilder, UpdateDynamicObject, UpdateDynamicObjectBuilder, UpdateGameObject, UpdateGameObjectBuilder, UpdateItem, UpdateItemBuilder, UpdatePlayer, UpdatePlayerBuilder, UpdateUnit, UpdateUnitBuilder}};", version.module_name()));
    s.newline();

    for (ty, types) in &update_types {
        s.bodyn(format!("impl {ty}Builder"), |s| {
            for m in fields.iter().filter(|a| types.contains(&a.object_ty)) {
                print_builder_setter(s, m);
            }
        });
    }

    for (ty, types) in &update_types {
        s.bodyn(format!("impl {ty}"), |s| {
            for m in fields.iter().filter(|a| types.contains(&a.object_ty)) {
                print_setter(s, m);
                print_getter(s, m);
            }
        });
    }

    s
}

fn print_specific_update_mask_indices(fields: &[MemberType]) -> Writer {
    let mut s = Writer::new();

    s.wln("use std::convert::TryFrom;");

    for field in fields {
        if let UfType::ArrayOfStruct { name, size, .. } = field.ty {
            assert_eq!(field.size % size, 0);
            let amount_of_fields = field.size / size;

            let ty_name = format!("{name}Index");

            s.new_enum("pub", &ty_name, |s| {
                for i in 0..amount_of_fields {
                    s.wln(format!("Index{i},"));
                }
            });

            s.bodyn(format!("impl {ty_name}"), |s| {
                s.funcn_const("offset(&self)", "u16", |s| {
                    s.wln(format!("{offset} + self.index()", offset = field.offset));
                });

                s.funcn_const("index(&self)", "u16", |s| {
                    s.open_curly("match self");

                    for i in 0..amount_of_fields {
                        s.wln(format!("Self::Index{i} => {index},", index = i * size));
                    }

                    s.closing_curly();
                });

                s.funcn_const("first(&self)", "u16", |s| {
                    s.wln(format!("{offset} + self.index()", offset = field.offset));
                });

                s.funcn_const("last(&self)", "u16", |s| {
                    s.wln(format!("self.first() + {size}"));
                });
            });

            s.impl_for("Default", &ty_name, |s| {
                s.open_curly("fn default() -> Self");
                s.wln("Self::Index0");
                s.closing_curly();
            });

            s.funcn_const(
                "try_from_inner(value: u16)",
                format!("Option<{ty_name}>"),
                |s| {
                    s.open_curly("Some(match value");

                    for i in 0..amount_of_fields {
                        s.wln(format!("{i} => {ty_name}::Index{i},"));
                    }

                    s.wln("_ => return None,");

                    s.closing_curly_with(")"); // match
                },
            );

            for from_ty in ["u16", "u32", "u64", "i16", "i32", "i64"] {
                s.impl_for(format!("TryFrom<{from_ty}>"), &ty_name, |s| {
                    s.wln(format!("type Error = {from_ty};"));
                    s.newline();

                    s.open_curly(format!(
                        "fn try_from(value: {from_ty}) -> Result<Self, Self::Error>"
                    ));
                    let cast = if from_ty == "u16" { "" } else { " as u16" };

                    s.wln(format!("try_from_inner(value{cast}).ok_or(value)"));

                    s.closing_curly(); // fn try_from
                })
            }
        }
    }

    s
}

pub(crate) fn print_update_mask() {
    print_update_mask_docs();

    for version in MajorWorldVersion::versions() {
        let fields = match version {
            MajorWorldVersion::Vanilla => vanilla_fields::FIELDS,
            MajorWorldVersion::BurningCrusade => tbc_fields::FIELDS,
            MajorWorldVersion::Wrath => wrath_fields::FIELDS,
        };

        let s = print_specific_update_mask(fields, *version);
        overwrite_if_not_same_contents(s.inner(), &update_mask_location(*version));

        let s = print_specific_update_mask_indices(fields);
        overwrite_if_not_same_contents(s.inner(), &update_mask_index_location(*version));
    }
}

fn print_getter(s: &mut Writer, m: &MemberType) {
    let offset = m.offset;

    match m.ty() {
        UfType::ArrayOfStruct { name, .. } => {
            s.open_curly(format!(
                "pub fn {}_{}(&self, index: {name}Index) -> Option<{}>",
                m.object_ty.to_string().to_lowercase(),
                m.name.to_lowercase(),
                m.ty.ty_str(),
            ));
        }
        UfType::GuidEnumLookupArray {
            name,
            variable_name,
            import_location,
        } => {
            s.open_curly(format!(
                "pub fn {}_{}(&self, {variable_name}: {import_location}::{name}) -> Option<Guid>",
                m.object_ty.to_string().to_lowercase(),
                m.name.to_lowercase(),
            ));
        }
        _ => {
            s.open_curly(format!(
                "pub fn {}_{}(&self) -> Option<{}>",
                m.object_ty.to_string().to_lowercase(),
                m.name.to_lowercase(),
                m.ty.ty_str(),
            ));
        }
    }

    match m.ty() {
        UfType::Guid => {
            s.wln(format!("self.get_guid({offset})"));
        }
        UfType::Int => {
            s.wln(format!("self.values.get(&{offset}).map(|v| *v as i32)"));
        }
        UfType::Float => {
            s.wln(format!("self.values.get(&{offset}).map(|v| f32::from_le_bytes(v.to_le_bytes()))"));
        }
        UfType::Bytes => {
            s.body_else(
                format!("if let Some(v) = self.values.get(&{offset})"),
                |s| {
                    s.wln("let v = v.to_le_bytes();");
                    s.wln("let (a, b, c, d) = (v[0], v[1], v[2], v[3]);");
                    s.wln("Some((a, b, c, d))");
                },
                |s| {
                    s.wln("None");
                },
            );
        }
        UfType::BytesWith(a, b, c, d) => {
            s.body_else(
                format!("if let Some(v) = self.values.get(&{offset})"),
                |s| {
                    s.wln("let v = v.to_le_bytes();");
                    s.wln(format!(
                        "let ({}, {}, {}, {}) = (v[0], v[1], v[2], v[3]);",
                        a.name, b.name, c.name, d.name
                    ));

                    let f = |byte: &ByteType| {
                        match byte.ty {
                            ByteInnerTy::Byte => byte.name.to_string(),
                            ByteInnerTy::Ty(_) => format!("{}.try_into().unwrap()", byte.name),
                        }
                    };

                    let a = f(&a);
                    let b = f(&b);
                    let c = f(&c);
                    let d = f(&d);

                    s.wln(format!("Some(({a}, {b}, {c}, {d}))"));
                },
                |s| {
                    s.wln("None");
                },
            );
        }
        UfType::TwoShort => s.body_else(
            format!("if let Some(v) = self.values.get(&{offset})"),
            |s| {
                s.wln("let v = v.to_le_bytes();");
                s.wln("let (a, b) = (u16::from_le_bytes([v[0], v[1]]), u16::from_le_bytes([v[2], v[3]]));");
                s.wln("Some((a, b))");
            },
            |s| {
                s.wln("None");
            },
        ),
        UfType::ArrayOfStruct { name,  import_location, .. } => {
            s.wln(format!("{import_location}::{name}::from_range(self.values.range(index.first()..=index.last()))"));
        }
        UfType::GuidEnumLookupArray {  variable_name, .. } => {
            s.wln(format!("let offset = {offset} + {variable_name}.as_int() as u16 * 2;"));
            s.wln(format!("let lower = self.values.get(&offset);"));
            s.wln(format!("let upper = self.values.get(&(offset + 1));"));
            s.newline();

            s.wln("lower.map(|lower| Guid::new((*upper.unwrap() as u64) << 32 | *lower as u64))");
        }
    }

    s.closing_curly_newline(); // pub(crate) fn get_
}

fn print_setter(s: &mut Writer, m: &MemberType) {
    s.open_curly(format!(
        "pub fn set_{}_{}(&mut self, {})",
        m.object_ty.to_string().to_lowercase(),
        m.name.to_lowercase(),
        m.ty.parameter_str(),
    ));

    print_setter_internals(s, m);

    s.closing_curly_newline(); // pub(crate) fn set_
}

fn print_setter_internals(s: &mut Writer, m: &MemberType) {
    let offset = m.offset;
    match m.ty {
        UfType::Guid => {
            s.wln(format!("self.set_guid({offset}, v);"));
        }
        UfType::ArrayOfStruct { variable_name, .. } => {
            s.open_curly(format!(
                "for (index, value) in {variable_name}.mask_values(index)"
            ));
            s.wln("self.header_set(index, value);");
            s.closing_curly();
        }
        UfType::GuidEnumLookupArray { variable_name, .. } => {
            s.wln(format!(
                "let offset = {offset} + {variable_name}.as_int() as u16 * 2;",
            ));
            s.wln(format!("self.set_guid(offset, item);"));
        }
        _ => {
            let value = match &m.ty {
                UfType::Int | UfType::Float => "u32::from_le_bytes(v.to_le_bytes())".to_string(),
                UfType::Bytes => "u32::from_le_bytes([a, b, c, d])".to_string(),
                UfType::TwoShort => "crate::util::u16s_to_u32(a, b)".to_string(),
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

                    format!("u32::from_le_bytes([{a}, {b}, {c}, {d}])")
                }
                UfType::ArrayOfStruct { .. } => {
                    unreachable!("Guid has already been checked for in outer match")
                }
                UfType::Guid => unreachable!("Guid has already been checked for in outer match"),
                UfType::GuidEnumLookupArray { .. } => unreachable!(),
            };

            s.wln(format!("self.header_set({}, {});", m.offset, value));
        }
    }
}

fn print_builder_setter(s: &mut Writer, m: &MemberType) {
    s.open_curly(format!(
        "pub fn set_{}_{}(mut self, {}) -> Self",
        m.object_ty.to_string().to_lowercase(),
        m.name.to_lowercase(),
        m.ty.parameter_str(),
    ));

    print_setter_internals(s, m);

    s.wln("self");
    s.closing_curly_newline(); // pub(crate) fn set_
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) enum UpdateMaskType {
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
pub(crate) enum ByteInnerTy {
    Byte,
    Ty(&'static str),
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct ByteType {
    pub name: &'static str,
    pub ty: ByteInnerTy,
}

impl ByteType {
    pub(crate) const fn new(name: &'static str, ty: &'static str) -> Self {
        Self {
            name,
            ty: ByteInnerTy::Ty(ty),
        }
    }

    pub(crate) const fn byte(name: &'static str) -> Self {
        Self {
            name,
            ty: ByteInnerTy::Byte,
        }
    }

    pub(crate) const fn ty_str(&self) -> &'static str {
        match self.ty {
            ByteInnerTy::Byte => "u8",
            ByteInnerTy::Ty(s) => s,
        }
    }

    #[allow(unused)]
    pub(crate) const fn a() -> Self {
        Self {
            name: "a",
            ty: ByteInnerTy::Byte,
        }
    }
    #[allow(unused)]
    pub(crate) const fn b() -> Self {
        Self {
            name: "b",
            ty: ByteInnerTy::Byte,
        }
    }
    #[allow(unused)]
    pub(crate) const fn c() -> Self {
        Self {
            name: "c",
            ty: ByteInnerTy::Byte,
        }
    }
    #[allow(unused)]
    pub(crate) const fn d() -> Self {
        Self {
            name: "d",
            ty: ByteInnerTy::Byte,
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) enum UfType {
    Guid,
    Int,
    Float,
    Bytes,
    BytesWith(ByteType, ByteType, ByteType, ByteType),
    TwoShort,
    ArrayOfStruct {
        name: &'static str,
        variable_name: &'static str,
        import_location: &'static str,
        size: i32,
    },
    GuidEnumLookupArray {
        name: &'static str,
        variable_name: &'static str,
        import_location: &'static str,
    },
}

const GUID_TYPE: &str = "Guid";
const INT_TYPE: &str = "i32";
const FLOAT_TYPE: &str = "f32";
const TWO_SHORT_TYPE: &str = "u16";

impl UfType {
    pub(crate) fn ty_str(&self) -> String {
        match self {
            UfType::Guid => GUID_TYPE.to_string(),
            UfType::Int => INT_TYPE.to_string(),
            UfType::Float => FLOAT_TYPE.to_string(),
            UfType::TwoShort => format!("({TWO_SHORT_TYPE}, {TWO_SHORT_TYPE})"),
            UfType::Bytes => "(u8, u8, u8, u8)".to_string(),
            UfType::BytesWith(a, b, c, d) => format!(
                "({}, {}, {}, {})",
                a.ty_str(),
                b.ty_str(),
                c.ty_str(),
                d.ty_str(),
            ),
            UfType::GuidEnumLookupArray {
                name,
                import_location,
                ..
            }
            | UfType::ArrayOfStruct {
                name,
                import_location,
                ..
            } => format!("{import_location}::{name}"),
        }
    }

    pub(crate) fn parameter_str(&self) -> String {
        format!(
            "v: {}",
            match self {
                UfType::Guid => GUID_TYPE,
                UfType::Int => INT_TYPE,
                UfType::Float => FLOAT_TYPE,
                UfType::TwoShort => return format!("a: {TWO_SHORT_TYPE}, b: {TWO_SHORT_TYPE}"),
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
                UfType::ArrayOfStruct {
                    name,
                    variable_name,
                    import_location,
                    ..
                } =>
                    return format!(
                        "{variable_name}: {import_location}::{name}, index: {name}Index"
                    ),
                UfType::GuidEnumLookupArray {
                    name,
                    variable_name,
                    import_location,
                } => {
                    return format!("{variable_name}: {import_location}::{name}, item: Guid");
                }
            }
        )
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct MemberType {
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
    pub(crate) const fn object_ty(&self) -> UpdateMaskType {
        self.object_ty
    }
    pub(crate) const fn name(&self) -> &'static str {
        self.name
    }
    pub(crate) fn ty(&self) -> UfType {
        self.ty.clone()
    }
    pub(crate) const fn size(&self) -> i32 {
        self.size
    }
    pub(crate) const fn offset(&self) -> i32 {
        self.offset
    }
}
