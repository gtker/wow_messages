use std::collections::HashMap;

use crate::container::{Container, Equation, StructMember};
use crate::parser::enumerator::{Definer, DefinerValue};
use crate::parser::types::tags::Tags;
use crate::parser::types::ty::Type;
use crate::parser::types::ty::Type::Identifier;
use crate::parser::types::{ContainerValue, IntegerType};

#[derive(Debug, Clone)]
pub struct RustStructComplexTree {
    tree: OrderedMap,
    new_enums: Vec<ComplexEnum>,
}

#[derive(Debug, Copy, Clone)]
pub enum DefinerType {
    Enum,
    Flag,
}

#[derive(Debug, Clone)]
pub struct ComplexEnum {
    name: String,
    definer_ty: DefinerType,
    variable_name: String,
    original_struct_name: String,
    original_ty_name: String,
    ty: IntegerType,
    fields: Vec<Enumerator>,
}

impl ComplexEnum {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn variable_name(&self) -> &str {
        &self.variable_name
    }

    pub fn ty(&self) -> &IntegerType {
        &self.ty
    }

    pub fn definer_ty(&self) -> DefinerType {
        self.definer_ty
    }

    pub fn original_ty_name(&self) -> &str {
        &self.original_ty_name
    }

    pub fn original_struct_name(&self) -> &str {
        &self.original_struct_name
    }

    pub fn fields(&self) -> &[Enumerator] {
        &self.fields
    }

    pub fn no_enumerator_without_fields(&self) -> bool {
        for f in self.fields() {
            if f.is_simple_or_subfields_const() {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone)]
pub struct Enumerator {
    name: String,
    value: DefinerValue,
    fields: Vec<EnumField>,
    not_main_if: bool,
}

impl Enumerator {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &DefinerValue {
        &self.value
    }

    pub fn subfields(&self) -> &[EnumField] {
        &self.fields
    }

    pub fn should_not_be_in_type(&self) -> bool {
        self.is_simple() || self.not_main_if
    }

    pub fn is_simple(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn is_simple_or_subfields_const(&self) -> bool {
        self.is_simple() || self.subfields().iter().all(|a| a.constant_value.is_some())
    }
}

#[derive(Debug, Clone)]
pub struct EnumField {
    name: String,
    ty: Type,
    used_as_size_in: Option<String>,
    constant_value: Option<String>,
}

impl EnumField {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn used_as_size_in(&self) -> &Option<String> {
        &self.used_as_size_in
    }

    pub fn constant_value(&self) -> &Option<String> {
        &self.constant_value
    }
}

fn get_enumerators(e: &Definer) -> Vec<Enumerator> {
    let mut v = Vec::new();

    for f in e.fields() {
        v.push(Enumerator {
            name: f.name().to_string(),
            value: f.value().clone(),
            fields: vec![],
            not_main_if: false,
        });
    }

    v
}

fn handle_enum(
    struct_name: &str,
    struct_tags: &Tags,
    variable_name: &str,
    d: &Declaration,
    e: &Definer,
    v: &mut Vec<ComplexEnum>,
    enums: &[Definer],
    flags: &[Definer],
    ty: DefinerType,
) {
    let mut enumerators = get_enumerators(e);

    for enumerator in enumerators.iter_mut() {
        for inner in &d.values {
            if enumerator.name == *inner.0 {
                for a in &inner.1.inner {
                    if !handle_decl(struct_name, struct_tags, a.name(), a, enums, flags, v) {
                        // simple type
                        enumerator.fields.push(EnumField {
                            name: a.name.to_string(),
                            ty: a.ty.clone(),
                            used_as_size_in: a.used_as_size_in.clone(),
                            constant_value: a.constant_value.clone(),
                        });
                    } else {
                        // enum type
                        enumerator.fields.push(EnumField {
                            name: a.name().to_string(),
                            ty: Type::from_str(
                                format!("{}{}", struct_name, a.ty().rust_str()).as_str(),
                            ),
                            used_as_size_in: a.used_as_size_in.clone(),
                            constant_value: a.constant_value.clone(),
                        });
                    }
                }
            }
        }
        for inner in &d.else_values {
            if enumerator.name != *inner.0 && !d.else_values.contains_key(enumerator.name.as_str())
            {
                for a in &inner.1.inner {
                    if !handle_decl(struct_name, struct_tags, a.name(), a, enums, flags, v) {
                        // simple type
                        enumerator.fields.push(EnumField {
                            name: a.name.to_string(),
                            ty: a.ty.clone(),
                            used_as_size_in: a.used_as_size_in.clone(),
                            constant_value: a.constant_value.clone(),
                        });
                    } else {
                        // enum type
                        enumerator.fields.push(EnumField {
                            name: a.name().to_string(),
                            ty: Type::from_str(
                                format!("{}{}", struct_name, a.ty().rust_str()).as_str(),
                            ),
                            used_as_size_in: a.used_as_size_in.clone(),
                            constant_value: a.constant_value.clone(),
                        });
                    }
                }
            }
        }
        for inner in &d.else_if_values {
            if enumerator.name == *inner.0 {
                enumerator.not_main_if = true;
                for a in &inner.1.inner {
                    if !handle_decl(struct_name, struct_tags, a.name(), a, enums, flags, v) {
                        // simple type
                        enumerator.fields.push(EnumField {
                            name: a.name.to_string(),
                            ty: a.ty.clone(),
                            used_as_size_in: a.used_as_size_in.clone(),
                            constant_value: a.constant_value.clone(),
                        });
                    } else {
                        // enum type
                        enumerator.fields.push(EnumField {
                            name: a.name().to_string(),
                            ty: Type::from_str(
                                format!("{}{}", struct_name, a.ty().rust_str()).as_str(),
                            ),
                            used_as_size_in: a.used_as_size_in.clone(),
                            constant_value: a.constant_value.clone(),
                        });
                    }
                }
            }
        }
    }

    for e in &mut enumerators {
        e.fields.dedup_by(|a, b| a.name == b.name);
    }

    v.push(ComplexEnum {
        name: format!("{}{}", struct_name, d.ty.rust_str()),
        definer_ty: ty,
        variable_name: variable_name.to_string(),
        original_struct_name: struct_name.to_string(),
        original_ty_name: d.ty.rust_str(),
        ty: *e.ty(),
        fields: enumerators,
    });
}

fn handle_decl(
    struct_name: &str,
    struct_tags: &Tags,
    variable_name: &str,
    d: &Declaration,
    enums: &[Definer],
    flags: &[Definer],
    v: &mut Vec<ComplexEnum>,
) -> bool {
    if d.does_not_have_subvariables() {
        return false;
    }

    let (e, ty) =
        if let Some(e) = enums.iter().find(|a| {
            a.name() == d.ty.rust_str() && a.tags().has_version_intersections(struct_tags)
        }) {
            (e, DefinerType::Enum)
        } else if let Some(e) = flags.iter().find(|a| {
            a.name() == d.ty.rust_str() && a.tags().has_version_intersections(struct_tags)
        }) {
            (e, DefinerType::Flag)
        } else {
            // not an enum, no need to go deeper since only ifs will nest us
            panic!("non enum has subtypes/enum used was not found: {:#?}", d);
        };

    handle_enum(
        struct_name,
        struct_tags,
        variable_name,
        d,
        e,
        v,
        enums,
        flags,
        ty,
    );

    true
}

fn add_new_enums(
    struct_name: &str,
    struct_tags: &Tags,
    declarations: &[Declaration],
    enums: &[Definer],
    flags: &[Definer],
) -> Vec<ComplexEnum> {
    let mut v = Vec::new();

    for t in declarations {
        handle_decl(struct_name, struct_tags, t.name(), t, enums, flags, &mut v);
    }

    v.dedup_by(|a, b| a.name() == b.name());

    v
}

impl RustStructComplexTree {
    pub fn new_enums(&self) -> &[ComplexEnum] {
        &self.new_enums
    }

    pub fn new(s: &Container, enums: &[Definer], flags: &[Definer]) -> Self {
        let mut tree = OrderedMap::new();

        for i in s.fields() {
            add_struct_member(i, &mut tree, &mut OrderedMap::new());
        }

        let new_enums = add_new_enums(s.name(), s.tags(), &tree.inner, enums, flags);

        Self { tree, new_enums }
    }

    pub fn contains_any_complex_enums(&self) -> bool {
        for e in &self.tree.inner {
            if e.does_not_have_subvariables() {
                return true;
            }
        }
        false
    }

    pub fn declarations(&self) -> &[Declaration] {
        &self.tree.inner
    }
}

#[derive(Debug, Clone)]
pub struct OrderedMap {
    inner: Vec<Declaration>,
}

impl OrderedMap {
    pub fn inner(&self) -> &[Declaration] {
        &self.inner
    }

    fn new() -> Self {
        Self { inner: vec![] }
    }

    fn insert(&mut self, v: Declaration) {
        self.inner.push(v);
    }

    fn get_mut(&mut self, k: &str) -> Option<&mut Declaration> {
        for i in &mut self.inner {
            if k == i.name {
                return Some(i);
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct Declaration {
    name: String,
    ty: Type,
    constant_value: Option<String>,
    used_as_size_in: Option<String>,
    values: HashMap<String, OrderedMap>,
    else_if_values: HashMap<String, OrderedMap>,
    else_values: HashMap<String, OrderedMap>,
}

impl Declaration {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn transfer(&mut self, other: Self) {
        for k in other.values {
            self.values.insert(k.0, k.1);
        }

        for k in other.else_if_values {
            self.else_if_values.insert(k.0, k.1);
        }

        for k in other.else_values {
            self.else_values.insert(k.0, k.1);
        }
    }

    pub fn for_transfer() -> Self {
        Self {
            name: "".to_string(),
            ty: Type::PackedGuid,
            constant_value: None,
            used_as_size_in: None,
            values: Default::default(),
            else_if_values: Default::default(),
            else_values: Default::default(),
        }
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn type_name(&self) -> &str {
        match &self.ty {
            Identifier { s, .. } => s,
            _ => panic!("invalid operation"),
        }
    }

    pub fn used_as_size_in(&self) -> Option<String> {
        self.used_as_size_in.clone()
    }

    pub fn values(&self) -> &HashMap<String, OrderedMap> {
        &self.values
    }

    pub fn get_subfields_for_enumerator(&self, enumerator: &str) -> Vec<&Declaration> {
        let mut v = Vec::new();
        if let Some((_, o)) = &self.values.iter().find(|a| a.0 == enumerator) {
            let decs = o.inner();
            for d in decs {
                v.push(d);
            }
        }

        for ev in self.else_values() {
            if ev.0 == enumerator {
                continue;
            }

            for d in ev.1.inner() {
                v.push(d);
            }
        }

        v
    }

    pub fn else_values(&self) -> &HashMap<String, OrderedMap> {
        &self.else_values
    }

    pub fn constant_value(&self) -> &Option<String> {
        &self.constant_value
    }

    pub fn does_not_have_subvariables(&self) -> bool {
        self.values.is_empty() && self.else_values.is_empty()
    }

    pub fn get_declarations_for_enumerator(&self, enumerator: &str) -> &OrderedMap {
        self.values.iter().find(|&a| a.0 == enumerator).unwrap().1
    }

    pub fn new(
        name: &str,
        ty: &Type,
        constant_value: &Option<ContainerValue>,
        used_as_size_in: Option<String>,
    ) -> Self {
        let constant_value = constant_value.as_ref().map(|s| s.identifier().to_string());

        Self {
            name: name.to_string(),
            ty: ty.clone(),
            constant_value,
            used_as_size_in,
            values: Default::default(),
            else_if_values: Default::default(),
            else_values: Default::default(),
        }
    }
}

fn add_struct_member(m: &StructMember, tree: &mut OrderedMap, parent_tree: &mut OrderedMap) {
    match m {
        StructMember::Definition(d) => {
            tree.insert(Declaration::new(
                d.name(),
                d.ty(),
                d.value(),
                d.used_as_size_in().clone(),
            ));
        }
        StructMember::IfStatement(statement) => {
            let mut v = Declaration::for_transfer();

            let mut k = OrderedMap::new();
            for f in &statement.members {
                add_struct_member(f, &mut k, tree);
            }
            for e in statement.get_conditional().equations() {
                match e {
                    Equation::Equals { value } | Equation::BitwiseAnd { value } => {
                        v.values.insert(value.clone(), k.clone());
                    }
                    Equation::NotEquals { value } => {
                        v.else_values.insert(value.clone(), k.clone());
                    }
                }
            }

            for s in statement.else_ifs() {
                let mut k = OrderedMap::new();
                for else_if in &s.members {
                    add_struct_member(else_if, &mut k, tree);
                }
                for e in s.get_conditional().equations() {
                    match e {
                        Equation::Equals { value } | Equation::BitwiseAnd { value } => {
                            v.else_if_values.insert(value.clone(), k.clone());
                        }
                        Equation::NotEquals { .. } => panic!(),
                    }
                }
            }

            let mut k = OrderedMap::new();
            for f in &statement.else_statement_members {
                add_struct_member(f, &mut k, tree);
            }

            for e in statement.get_conditional().equations() {
                match e {
                    Equation::Equals { value } | Equation::BitwiseAnd { value } => {
                        v.else_values.insert(value.clone(), k.clone());
                    }
                    Equation::NotEquals { value } => {
                        v.values.insert(value.clone(), k.clone());
                    }
                }
            }
            for s in statement.else_ifs() {
                for e in s.get_conditional().equations() {
                    match e {
                        Equation::Equals { value } | Equation::BitwiseAnd { value } => {
                            v.else_values.insert(value.clone(), k.clone());
                        }
                        Equation::NotEquals { .. } => panic!(),
                    }
                }
            }

            if let Some(decl) = tree.get_mut(statement.name()) {
                decl.transfer(v);
            } else if let Some(decl) = parent_tree.get_mut(statement.name()) {
                decl.transfer(v);
            } else {
                panic!()
            }
        }
        StructMember::OptionalStatement(_) => {
            // TODO OPTIONAL
        }
    }
}
