pub mod parse;

use crate::container::StructMemberDefinition;
use crate::rust_printer::complex_print::ComplexEnum;

#[derive(Debug, Clone)]
pub enum NewEnumStructMember {
    Definition(StructMemberDefinition),
    IfStatement(NewIfStatement),
}

#[derive(Debug, Clone)]
pub struct NewEnumerator {
    name: String,
    fields: Vec<NewEnumStructMember>,
}

impl NewEnumerator {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            fields: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fields(&self) -> &[NewEnumStructMember] {
        &self.fields
    }

    pub fn add_field(&mut self, mut fields: Vec<NewEnumStructMember>) {
        self.fields.append(&mut fields);
    }

    pub fn get_variable_names_for_members(&self) -> Vec<&StructMemberDefinition> {
        let mut v = Vec::new();

        for m in self.fields() {
            match m {
                NewEnumStructMember::Definition(d) => {
                    v.push(d);
                }
                NewEnumStructMember::IfStatement(_) => {}
            }
        }

        v
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum IfStatementType {
    Enum,
    Flag,
}

#[derive(Debug, Clone)]
pub struct NewIfStatement {
    variable_name: String,
    enum_or_flag: IfStatementType,
    original_ty_name: String,
    new_ty_name: String,
    enumerators: Vec<NewEnumerator>,
}

impl NewIfStatement {
    pub fn new(
        variable_name: &str,
        enum_or_flag: IfStatementType,
        original_ty_name: &str,
        new_ty_name: &str,
        enumerators: Vec<NewEnumerator>,
    ) -> Self {
        Self {
            variable_name: variable_name.to_string(),
            enum_or_flag,
            original_ty_name: original_ty_name.to_string(),
            new_ty_name: new_ty_name.to_string(),
            enumerators,
        }
    }

    pub fn variable_name(&self) -> &str {
        &self.variable_name
    }

    pub fn enum_or_flag(&self) -> IfStatementType {
        self.enum_or_flag
    }

    pub fn single_enumerator_with_fields(&self) -> &NewEnumerator {
        let e: Vec<&NewEnumerator> = self
            .enumerators()
            .iter()
            .filter(|a| !a.fields().is_empty())
            .collect();
        assert!(e.len() == 1);
        e[0]
    }

    pub fn original_ty_name(&self) -> &str {
        &self.original_ty_name
    }

    pub fn new_ty_name(&self) -> &str {
        &self.new_ty_name
    }

    pub fn enumerators(&self) -> &[NewEnumerator] {
        &self.enumerators
    }
}
