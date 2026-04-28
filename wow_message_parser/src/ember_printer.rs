use crate::file_utils::{
    create_and_overwrite_if_not_same_contents, overwrite_if_not_same_contents,
};
use crate::parser::types::array::ArraySize;
use crate::parser::types::container::{Container, ContainerType};
use crate::parser::types::definer::Definer;
use crate::parser::types::if_statement::Equation;
use crate::parser::types::objects::{Object, Objects};
use crate::parser::types::struct_member::StructMember;
use crate::parser::types::tags::ObjectTags;
use crate::parser::types::ty::Type;
use crate::parser::types::version::MajorWorldVersion;
use crate::path_utils::{ember_directory, ember_types};
use crate::rust_printer::DefinerType;
use heck::ToPascalCase;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
#[serde(tag = "kind")]
#[serde(rename_all = "lowercase")]
enum Ty {
    Enum {
        underlying: String,
        values: BTreeMap<String, i128>,
    },
    Flags {
        underlying: String,
        values: BTreeMap<String, i128>,
    },
    Struct {
        fields: Vec<StructField>,
    },
    External {
        include: String,
    },
    String {
        #[serde(skip_serializing_if = "Option::is_none")]
        encoding: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        length_type: Option<String>,
    },
}

impl Ty {
    fn from_definer(definer: &Definer) -> Self {
        let mut values = BTreeMap::new();

        for f in definer.fields() {
            values.insert(f.name().to_string(), f.value().int());
        }

        match definer.definer_ty() {
            DefinerType::Enum => Ty::Enum {
                underlying: definer.ty().ember_str().to_string(),
                values,
            },
            DefinerType::Flag => Ty::Flags {
                underlying: definer.ty().ember_str().to_string(),
                values,
            },
        }
    }

    fn from_container(c: &Container) -> Option<Self> {
        let mut fields = Vec::with_capacity(c.members().len());

        for m in c.members() {
            Self::from_struct_member(m, &mut fields);
        }

        Some(Ty::Struct { fields })
    }

    fn from_struct_member(m: &StructMember, fields: &mut Vec<StructField>) {
        match m {
            StructMember::Definition(d) => {
                let array = if let Type::Array(array) = d.ty() {
                    Some(match array.size() {
                        ArraySize::Fixed(size) => ArrayType::Fixed { size },
                        ArraySize::Variable(size) => ArrayType::Variable {
                            count_field: size.name().to_string(),
                        },
                        ArraySize::Endless => ArrayType::Endless {
                            until_end: true,
                        },
                    })
                } else {
                    None
                };

                let upcast = match d.ty() {
                    Type::Enum { upcast, .. } |
                    Type::Flag { upcast, .. } => {
                        upcast.map(|c| c.ember_str().to_string())
                    }
                    _ => None,
                };

                fields.push(StructField::Field {
                    name: d.name().to_string(),
                    ty: d.ty().ember_str().to_string(),
                    upcast,
                    array,
                });
            }
            StructMember::IfStatement(statement) => {
                if statement.else_ifs().is_empty()
                    || statement.else_members().is_empty()
                    || statement.is_elseif_flag()
                {
                    return;
                }

                let when = match statement.equation() {
                    Equation::Equals { values } => {
                        if values.len() != 1 {
                            return;
                        }

                        Conditional::Eq {
                            field: statement.variable_name().to_string(),
                            value: values[0].to_string(),
                        }
                    }
                    Equation::NotEquals { value } => Conditional::Neq {
                        field: statement.variable_name().to_string(),
                        value: value.clone(),
                    },
                    Equation::BitwiseAnd { values } => {
                        if values.len() != 1 {
                            return;
                        }
                        Conditional::HasFlag {
                            field: statement.variable_name().to_string(),
                            value: values[0].to_string(),
                        }
                    }
                };

                let mut f = Vec::with_capacity(statement.members().len());

                for m in statement.members() {
                    Self::from_struct_member(m, &mut f);
                }

                fields.push(StructField::Group {
                    ty: "group".to_string(),
                    when,
                    fields: f,
                });
            }
            StructMember::OptionalStatement(_) => {}
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum ArrayType {
    Fixed { size: i128 },
    Variable { count_field: String },
    Endless { until_end: bool }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum StructField {
    Field {
        name: String,
        #[serde(rename = "type")]
        ty: String,
        #[serde(rename = "as")]
        #[serde(skip_serializing_if = "Option::is_none")]
        upcast: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        array: Option<ArrayType>,
    },
    Group {
        #[serde(rename = "type")]
        ty: String,
        when: Conditional,
        fields: Vec<StructField>,
    },
}

#[derive(Debug, Serialize)]
#[serde(tag = "op")]
#[serde(rename_all = "snake_case")]
enum Conditional {
    HasFlag { field: String, value: String },
    Eq { field: String, value: String },
    Neq { field: String, value: String },
}

#[derive(Debug, Serialize)]
struct Types {
    types: BTreeMap<String, Ty>,
}

impl Types {
    fn from_regular_objects(o: &Objects) -> Self {
        let mut types = BTreeMap::new();

        types.insert(
            "PackedGuid".to_string(),
            Ty::External {
                include: "protocol/types/PackedGuid.h".to_string(),
            },
        );

        types.insert(
            "CString".to_string(),
            Ty::String {
                encoding: Some("null_terminated".to_string()),
                length_type: None,
            },
        );

        types.insert(
            "PrefixedString32".to_string(),
            Ty::String {
                encoding: Some("prefixed_null_terminated".to_string()),
                length_type: Some("uint32".to_string()),
            },
        );

        for e in o.all_objects().filter(|a| {
            a.tags().has_world_version()
                && a.tags()
                    .has_version_intersections(&ObjectTags::new_with_world_versions(&[
                        MajorWorldVersion::Vanilla,
                    ]))
        }) {
            match e {
                Object::Container(c) => {
                    if c.container_type() == ContainerType::Struct {
                        if let Some(t) = Ty::from_container(c) {
                            types.insert(c.name().to_string(), t);
                        }
                    }
                }
                Object::Enum(definer) | Object::Flag(definer) => {
                    types.insert(definer.name().to_string(), Ty::from_definer(definer));
                }
            }
        }

        Types { types }
    }
}

#[derive(Debug, Serialize)]
struct Message {
    name: String,
    opcode: String,
    direction: String,
    fields: Vec<StructField>,
}

impl Message {
    fn from_regular_objects(c: &Container) -> Self {
        let mut fields = Vec::with_capacity(c.members().len());

        for m in c.members() {
            Ty::from_struct_member(m, &mut fields);
        }

        Message {
            name: c
                .name()
                .replace("CMSG_", "")
                .replace("SMSG_", "")
                .replace("MSG", "")
                .to_pascal_case(),
            opcode: c.name().to_ascii_lowercase().to_string(),
            direction: if c.name().starts_with("CMSG") {
                "client"
            } else {
                "server"
            }
            .to_string(),
            fields,
        }
    }
}

pub(crate) fn write_ember_schema(o: &Objects) {
    let types = Types::from_regular_objects(o);
    let json = serde_json::to_string_pretty(&types).unwrap();
    overwrite_if_not_same_contents(&json, &ember_types());

    for c in o.all_containers().filter(|a| {
        a.tags().has_world_version()
            && a.tags()
                .has_version_intersections(&ObjectTags::new_with_world_versions(&[
                    MajorWorldVersion::Vanilla,
                ]))
            && a.container_type() != ContainerType::Struct
    }) {
        let m = Message::from_regular_objects(c);
        let json = serde_json::to_string_pretty(&m).unwrap();

        create_and_overwrite_if_not_same_contents(
            &json,
            &ember_directory().join(format!("{}.json", c.name().to_lowercase())),
        );
    }
}
