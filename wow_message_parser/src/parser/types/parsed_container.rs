use crate::file_info::FileInfo;
use crate::parser::types::container::{StructMember, StructMemberDefinition};
use crate::parser::types::definer::Definer;
use crate::parser::types::if_statement::{DefinerUsage, IfStatement};
use crate::parser::types::objects::conversion::{get_container, get_definer};
use crate::parser::types::sizes::Sizes;
use crate::parser::types::tags::Tags;
use crate::parser::types::ty::Type;
use crate::parser::types::ArrayType;
use crate::{ContainerType, DefinerType};

#[derive(Debug, Clone)]
pub struct ParsedContainer {
    pub name: String,
    pub object_type: ContainerType,
    pub members: Vec<StructMember>,
    pub tags: Tags,
    pub file_info: FileInfo,
}

impl ParsedContainer {
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

    fn add_sizes_values(
        e: &Self,
        m: &StructMember,
        containers: &[Self],
        definers: &[Definer],
        sizes: &mut Sizes,
    ) {
        match m {
            StructMember::Definition(d) => *sizes += d.ty().sizes_parsed(e, containers, definers),
            StructMember::OptionalStatement(optional) => {
                let minimum = sizes.minimum();

                for m in optional.members() {
                    Self::add_sizes_values(e, m, containers, definers, sizes);
                }

                // The optional statement doesn't have be be here, so the minimum doesn't get incremented
                sizes.set_minimum(minimum);
            }
            StructMember::IfStatement(statement) => {
                let statement_sizes = Self::get_complex_sizes(statement, e, containers, definers);

                *sizes += statement_sizes;
            }
        }
    }

    pub fn create_sizes(&self, containers: &[Self], definers: &[Definer]) -> Sizes {
        let mut sizes = Sizes::new();
        for m in self.fields() {
            Self::add_sizes_values(self, m, containers, definers, &mut sizes);
        }

        sizes
    }

    pub fn get_complex_sizes(
        statement: &IfStatement,
        e: &Self,
        containers: &[Self],
        definers: &[Definer],
    ) -> Sizes {
        let mut if_sizes = Sizes::new();

        for m in statement.members() {
            Self::add_sizes_values(e, m, containers, definers, &mut if_sizes);
        }

        let mut smallest_sizes = if_sizes;
        let mut largest_sizes = if_sizes;

        let mut else_if_sizes;

        for elseif in statement.else_ifs() {
            else_if_sizes = Sizes::new();

            for m in elseif.members() {
                Self::add_sizes_values(e, m, containers, definers, &mut else_if_sizes);
            }

            if else_if_sizes.minimum() < smallest_sizes.minimum() {
                smallest_sizes = else_if_sizes;
            }
            if else_if_sizes.maximum() > largest_sizes.maximum() {
                largest_sizes = else_if_sizes;
            }
        }

        else_if_sizes = Sizes::new();
        for m in statement.else_members() {
            Self::add_sizes_values(e, m, containers, definers, &mut else_if_sizes);
        }

        if else_if_sizes.minimum() < smallest_sizes.minimum() {
            smallest_sizes = else_if_sizes;
        }
        if else_if_sizes.maximum() > largest_sizes.maximum() {
            largest_sizes = else_if_sizes;
        }

        let mut sizes = Sizes::new();
        sizes.set_minimum(smallest_sizes.minimum());
        sizes.set_maximum(largest_sizes.maximum());
        sizes
    }

    pub fn get_type_of_variable(&self, variable_name: &str) -> Type {
        for d in self.all_definitions() {
            if d.name() == variable_name {
                return d.ty().clone();
            }
        }

        panic!("unable to find type {}", variable_name)
    }

    pub fn fields(&self) -> &[StructMember] {
        self.members.as_slice()
    }

    pub fn recursive_only_has_io_errors(&self, containers: &[Self], definers: &[Definer]) -> bool {
        if self.contains_string_or_cstring() {
            return false;
        }

        for t in self.get_types_needing_import() {
            if let Some(d) = get_definer(definers, t.as_str(), self.tags()) {
                if d.definer_ty() == DefinerType::Enum {
                    return false;
                }
            } else if let Some(c) = get_container(containers, t.as_str(), self.tags()) {
                if !c.recursive_only_has_io_errors(containers, definers) {
                    return false;
                }
            } else {
                unreachable!()
            }
        }

        true
    }

    pub fn contains_string_or_cstring(&self) -> bool {
        for d in self.all_definitions() {
            match d.ty() {
                Type::CString | Type::String { .. } | Type::SizedCString => return true,
                Type::Array(array) => {
                    if matches!(array.ty(), ArrayType::CString) {
                        return true;
                    }
                }
                _ => {}
            }
        }

        false
    }

    pub fn get_types_needing_import(&self) -> Vec<String> {
        self.get_complex_types()
    }

    fn get_complex_types(&self) -> Vec<String> {
        let mut v = Vec::new();

        for d in self.all_definitions() {
            match &d.struct_type() {
                Type::Array(a) => {
                    if let ArrayType::Complex(i) = a.ty() {
                        v.push(i.clone());
                    }
                }
                Type::Identifier { s, .. } => {
                    v.push(s.clone());
                }
                _ => {}
            }
        }

        v.sort_unstable();
        v.dedup();

        v
    }
}
