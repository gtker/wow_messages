use crate::file_info::FileInfo;
use crate::parser::types::container::{DefinerUsage, StructMember, StructMemberDefinition};
use crate::parser::types::tags::Tags;
use crate::parser::types::ty::Type;
use crate::ContainerType;

#[derive(Debug, Clone)]
pub struct ParsedContainer {
    pub name: String,
    pub object_type: ContainerType,
    pub members: Vec<StructMember>,
    pub tags: Tags,
    pub file_info: FileInfo,
}

impl ParsedContainer {
    pub fn contains_definer(&self, ty_name: &str) -> DefinerUsage {
        fn inner(m: &StructMember, ty_name: &str, variable_name: &str) -> DefinerUsage {
            match m {
                StructMember::Definition(d) => {
                    if let Type::Identifier { s, .. } = d.ty() {
                        if s == ty_name {
                            return DefinerUsage::NotInIf;
                        }
                    }
                }
                StructMember::IfStatement(statement) => {
                    if statement.name() == variable_name {
                        return DefinerUsage::InIf;
                    }

                    let mut not_in_if = false;
                    for m in statement.all_members() {
                        match inner(m, ty_name, variable_name) {
                            DefinerUsage::Unused => {}
                            DefinerUsage::NotInIf => not_in_if = true,
                            DefinerUsage::InIf => return DefinerUsage::InIf,
                        }
                    }

                    if not_in_if {
                        return DefinerUsage::NotInIf;
                    }
                }
                StructMember::OptionalStatement(optional) => {
                    let mut not_in_if = false;

                    for m in optional.members() {
                        match inner(m, ty_name, variable_name) {
                            DefinerUsage::Unused => {}
                            DefinerUsage::NotInIf => not_in_if = true,
                            DefinerUsage::InIf => return DefinerUsage::InIf,
                        }
                    }

                    if not_in_if {
                        return DefinerUsage::NotInIf;
                    }
                }
            }

            DefinerUsage::Unused
        }

        let variable_name = self.get_variable_name_of_definer_ty(ty_name);

        if let Some(variable_name) = variable_name {
            let mut not_in_if = false;

            for m in self.fields() {
                match inner(m, ty_name, &variable_name) {
                    DefinerUsage::Unused => {}
                    DefinerUsage::NotInIf => not_in_if = true,
                    DefinerUsage::InIf => return DefinerUsage::InIf,
                }
            }

            if not_in_if {
                return DefinerUsage::NotInIf;
            }
        }

        DefinerUsage::Unused
    }

    pub fn get_variable_name_of_definer_ty(&self, ty_name: &str) -> Option<String> {
        for d in self.all_definitions() {
            if let Type::Identifier { s, .. } = d.ty() {
                if s == ty_name {
                    return Some(d.name().to_string());
                }
            }
        }

        None
    }

    pub fn get_field_ty(&self, field_name: &str) -> &Type {
        for d in self.all_definitions() {
            if d.name() == field_name {
                return d.ty();
            }
        }

        panic!("unable to find field: '{}'", field_name)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }

    pub fn all_definitions(&self) -> Vec<&StructMemberDefinition> {
        fn inner<'a>(m: &'a StructMember, v: &mut Vec<&'a StructMemberDefinition>) {
            match m {
                StructMember::Definition(d) => v.push(d),
                StructMember::IfStatement(statement) => {
                    for m in statement.all_members() {
                        inner(m, v);
                    }
                }
                StructMember::OptionalStatement(optional) => {
                    for m in optional.members() {
                        inner(m, v);
                    }
                }
            }
        }

        let mut v = Vec::new();

        for m in self.fields() {
            inner(m, &mut v);
        }

        v
    }

    pub fn fields(&self) -> &[StructMember] {
        self.members.as_slice()
    }

    pub fn new(
        name: &str,
        members: Vec<StructMember>,
        tags: Tags,
        object_type: ContainerType,
        file_info: FileInfo,
    ) -> Self {
        Self {
            name: name.to_string(),
            object_type,
            members,
            tags,
            file_info,
        }
    }
}
