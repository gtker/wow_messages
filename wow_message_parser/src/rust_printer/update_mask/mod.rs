use crate::file_utils::overwrite_if_not_same_contents;
use crate::parser::types::version::MajorWorldVersion;
use crate::parser::types::IntegerType;
use crate::path_utils::{update_mask_index_location, update_mask_location};
use crate::rust_printer::writer::Writer;
use std::fmt::{Display, Formatter};

pub mod tbc_fields;
pub mod vanilla_fields;
pub mod wrath_fields;

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

    print_includes(&mut s, fields, version, &update_types);

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

fn print_includes(
    s: &mut Writer,
    fields: &[MemberType],
    version: MajorWorldVersion,
    update_types: &[(&str, Vec<UpdateMaskType>); 7],
) {
    s.wln("use crate::Guid;");
    s.wln("use std::convert::TryInto;");
    s.wln("use super::indices::*;");
    s.newline();

    let mut imports = Vec::new();

    for m in fields {
        if let UfType::BytesWith(a, b, c, d) = m.ty() {
            for ty in [a, b, c, d] {
                match ty.ty {
                    ByteInnerTy::Byte => {}
                    ByteInnerTy::Ty(ty) => {
                        imports.push(ty.to_string());
                    }
                }
            }
        }
    }

    for (ty, _) in update_types {
        imports.push(ty.to_string());
        imports.push(format!("{ty}Builder"));
    }

    s.wln(format!("use crate::{}::{{", version.module_name()));
    s.inc_indent();
    for (i, ty) in imports.iter().enumerate() {
        let extra = if i != imports.len() - 1 { ", " } else { "" };
        s.w_break_at(format!("{ty}{extra}"));
    }
    s.newline();
    s.closing_curly_with(";");

    s.newline();
}

fn print_specific_update_mask_indices(fields: &[MemberType]) -> Writer {
    let mut s = Writer::new();

    s.wln("use std::convert::TryFrom;");

    for field in fields {
        if let UfType::ArrayOfStruct { name, size, .. } = field.ty {
            assert_eq!(field.size % size, 0);
            let amount_of_fields = field.size / size;
            let lower_name = name.to_lowercase();

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
                format!("{lower_name}_try_from_inner(value: u16)"),
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

            for (_, _, from_ty) in IntegerType::try_from_types() {
                let from_ty = *from_ty;

                s.impl_for(format!("TryFrom<{from_ty}>"), &ty_name, |s| {
                    s.wln(format!("type Error = {from_ty};"));
                    s.newline();

                    s.open_curly(format!(
                        "fn try_from(value: {from_ty}) -> Result<Self, Self::Error>"
                    ));
                    let cast = if from_ty == "u16" {
                        ""
                    } else if from_ty == "u8" {
                        ".into()"
                    } else {
                        ".try_into().ok().ok_or(value)?"
                    };

                    s.wln(format!(
                        "{lower_name}_try_from_inner(value{cast}).ok_or(value)"
                    ));

                    s.closing_curly(); // fn try_from
                })
            }
        }
    }

    s
}

pub(crate) fn print_update_mask() {
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
            s.wln(format!("self.get_int({offset})"));
        }
        UfType::Float => {
            s.wln(format!("self.get_float({offset})"));
        }
        UfType::Bytes => {
            s.wln(format!("self.get_bytes({offset})"));
        }
        UfType::TwoShort => {
            s.wln(format!("self.get_shorts({offset})"));
        }
        UfType::BytesWith(a, b, c, d) => {
            let bytes = [&a, &b, &c, &d];
            if bytes.iter().all(|a| matches!(a.ty, ByteInnerTy::Byte)) {
                s.wln(format!("self.get_bytes({offset})"));
            } else {
                s.body_closing_with(
                    format!(
                        "self.get_bytes({offset}).map(|({}, {}, {}, {})|",
                        a.name, b.name, c.name, d.name
                    ),
                    |s| {
                        let f = |byte: &ByteType| match byte.ty {
                            ByteInnerTy::Byte => byte.name.to_string(),
                            ByteInnerTy::Ty(_) => format!("{}.try_into().unwrap()", byte.name),
                        };

                        let a = f(&a);
                        let b = f(&b);
                        let c = f(&c);
                        let d = f(&d);

                        s.wln(format!("({a}, {b}, {c}, {d})"));
                    },
                    ")",
                );
            }
        }
        UfType::ArrayOfStruct {
            name,
            import_location,
            ..
        } => {
            s.wln(format!("{import_location}::{name}::from_range(self.values.range(index.first()..=index.last()))"));
        }
        UfType::GuidEnumLookupArray { variable_name, .. } => {
            s.wln(format!(
                "let offset = {offset} + {variable_name}.as_int() as u16 * 2;"
            ));
            s.wln("self.get_guid(offset)");
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
    match &m.ty {
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
            s.wln("self.set_guid(offset, item);");
        }
        UfType::Int => {
            s.wln(format!("self.set_int({offset}, v);"));
        }
        UfType::Float => {
            s.wln(format!("self.set_float({offset}, v);"));
        }
        UfType::Bytes => {
            s.wln(format!("self.set_bytes({offset}, a, b, c, d);"));
        }
        UfType::TwoShort => {
            s.wln(format!("self.set_shorts({offset}, a, b);"));
        }
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

            s.wln(format!("self.set_bytes({offset}, {a}, {b}, {c}, {d});"));
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

impl UpdateMaskType {
    pub(crate) const fn all() -> &'static [Self] {
        &[
            Self::Object,
            Self::Item,
            Self::Container,
            Self::Unit,
            Self::Player,
            Self::GameObject,
            Self::DynamicObject,
            Self::Corpse,
        ]
    }

    pub(crate) const fn as_str(&self) -> &'static str {
        match self {
            UpdateMaskType::Object => "object",
            UpdateMaskType::Item => "item",
            UpdateMaskType::Unit => "unit",
            UpdateMaskType::Player => "player",
            UpdateMaskType::Container => "container",
            UpdateMaskType::GameObject => "gameobject",
            UpdateMaskType::DynamicObject => "dynamicobject",
            UpdateMaskType::Corpse => "corpse",
        }
    }
}

impl Display for UpdateMaskType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
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
