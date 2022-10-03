use crate::parser::types::definer::Definer;
use crate::parser::types::objects::conversion::get_definer;
use crate::parser::types::parsed::parsed_if_statement::ParsedIfStatement;
use crate::parser::types::parsed::parsed_optional::ParsedOptionalStatement;
use crate::parser::types::parsed::parsed_ty::ParsedType;
use crate::parser::types::{ContainerValue, ParsedContainerValue};
use crate::{Tags, CONTAINER_SELF_SIZE_FIELD};

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum ParsedStructMember {
    Definition(ParsedStructMemberDefinition),
    IfStatement(ParsedIfStatement),
    OptionalStatement(ParsedOptionalStatement),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct ParsedStructMemberDefinition {
    pub name: String,
    pub(crate) struct_type: ParsedType,
    pub value: Option<ParsedContainerValue>,
    pub verified_value: Option<ContainerValue>,
    pub used_as_size_in: Option<String>,
    pub used_in_if: Option<bool>,
    pub tags: Tags,
}

impl ParsedStructMemberDefinition {
    pub(crate) fn struct_type(&self) -> ParsedType {
        self.struct_type.clone()
    }

    pub(crate) fn set_used_as_size_in(&mut self, var: String) {
        self.used_as_size_in = Some(var);
    }

    pub(crate) fn set_used_in_if(&mut self, used: bool) {
        self.used_in_if = Some(used);
    }

    pub(crate) fn new(
        name: &str,
        struct_type: ParsedType,
        value: Option<ParsedContainerValue>,
        tags: Tags,
    ) -> Self {
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

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn ty(&self) -> &ParsedType {
        &self.struct_type
    }

    pub(crate) fn value(&self) -> &Option<ParsedContainerValue> {
        &self.value
    }

    pub(crate) fn set_verified_value(&mut self, definers: &[Definer]) {
        match &self.value() {
            None => {}
            Some(v) => {
                let parsed_val = crate::parser::utility::parse_value(v.identifier());
                if let Some(int_val) = parsed_val {
                    self.verified_value =
                        Some(ContainerValue::new(int_val, v.identifier().to_string()))
                } else {
                    let value = if v.identifier() != CONTAINER_SELF_SIZE_FIELD {
                        get_definer(definers, &self.ty().str(), self.tags())
                            .unwrap()
                            .get_field_with_name(v.identifier())
                            .unwrap()
                            .value()
                            .int()
                    } else {
                        0
                    };
                    self.verified_value =
                        Some(ContainerValue::new(value, v.identifier().to_string()));
                }
            }
        }
    }

    pub(crate) fn tags(&self) -> &Tags {
        &self.tags
    }
}
