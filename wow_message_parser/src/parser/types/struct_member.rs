use crate::parser::types::definer::Definer;
use crate::parser::types::if_statement::IfStatement;
use crate::parser::types::objects::conversion::get_definer;
use crate::parser::types::optional::OptionalStatement;
use crate::parser::types::ty::Type;
use crate::parser::types::{ContainerValue, VerifiedContainerValue};
use crate::{Tags, CONTAINER_SELF_SIZE_FIELD};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StructMember {
    Definition(StructMemberDefinition),
    IfStatement(IfStatement),
    OptionalStatement(OptionalStatement),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StructMemberDefinition {
    name: String,
    struct_type: Type,
    value: Option<ContainerValue>,
    verified_value: Option<VerifiedContainerValue>,
    pub used_as_size_in: Option<String>,
    used_in_if: Option<bool>,
    tags: Tags,
}

impl StructMemberDefinition {
    pub fn struct_type(&self) -> Type {
        self.struct_type.clone()
    }

    pub fn used_as_size_in(&self) -> &Option<String> {
        &self.used_as_size_in
    }

    pub fn used_in_if(&self) -> bool {
        self.used_in_if.unwrap()
    }

    pub fn set_used_in_if(&mut self, used: bool) {
        self.used_in_if = Some(used);
    }

    pub fn new(name: &str, struct_type: Type, value: Option<ContainerValue>, tags: Tags) -> Self {
        Self {
            name: name.to_string(),
            struct_type,
            value,
            verified_value: None,
            used_as_size_in: None,
            used_in_if: None,
            tags,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> &Type {
        &self.struct_type
    }

    pub fn value(&self) -> &Option<ContainerValue> {
        &self.value
    }

    pub fn verified_value(&self) -> &Option<VerifiedContainerValue> {
        &self.verified_value
    }

    pub fn set_verified_value(&mut self, definers: &[Definer]) {
        match &self.value() {
            None => {}
            Some(v) => {
                let parsed_val = crate::parser::utility::parse_value(v.identifier());
                if let Some(int_val) = parsed_val {
                    self.verified_value = Some(VerifiedContainerValue::new(
                        int_val,
                        v.identifier().to_string(),
                    ))
                } else {
                    let value = if v.identifier() != CONTAINER_SELF_SIZE_FIELD {
                        get_definer(definers, &self.ty().rust_str(), &self.tags())
                            .unwrap()
                            .get_field_with_name(&v.identifier())
                            .unwrap()
                            .value()
                            .int()
                    } else {
                        0
                    };
                    self.verified_value = Some(VerifiedContainerValue::new(
                        value,
                        v.identifier().to_string(),
                    ));
                }
            }
        }
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }
}
