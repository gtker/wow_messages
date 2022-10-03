use crate::file_info::FileInfo;
use crate::parser::types::definer::Definer;
use crate::parser::types::if_statement::DefinerUsage;
use crate::parser::types::objects::conversion::{get_container, get_definer};
use crate::parser::types::parsed::parsed_if_statement::ParsedIfStatement;
use crate::parser::types::parsed::parsed_struct_member::{
    ParsedStructMember, ParsedStructMemberDefinition,
};
use crate::parser::types::parsed::parsed_ty::ParsedType;
use crate::parser::types::sizes::Sizes;
use crate::parser::types::tags::Tags;
use crate::parser::types::ArrayType;
use crate::{ContainerType, DefinerType};

#[derive(Debug, Clone)]
pub(crate) struct ParsedContainer {
    pub name: String,
    pub object_type: ContainerType,
    pub members: Vec<ParsedStructMember>,
    pub tags: Tags,
    pub file_info: FileInfo,
}

impl ParsedContainer {
    pub(crate) fn new(
        name: &str,
        members: Vec<ParsedStructMember>,
        tags: Tags,
        object_type: ContainerType,
        file_info: FileInfo,
    ) -> Self {
        let mut s = Self {
            name: name.to_string(),
            object_type,
            members,
            tags,
            file_info,
        };

        s.self_check();
        s.set_used_in_if();
        s.set_if_statements();

        s
    }

    pub(crate) fn contains_definer(&self, ty_name: &str) -> DefinerUsage {
        fn inner(m: &ParsedStructMember, ty_name: &str, variable_name: &str) -> DefinerUsage {
            match m {
                ParsedStructMember::Definition(d) => {
                    if let ParsedType::Identifier { s, .. } = d.ty() {
                        if s == ty_name {
                            return DefinerUsage::NotInIf;
                        }
                    }
                }
                ParsedStructMember::IfStatement(statement) => {
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
                ParsedStructMember::OptionalStatement(optional) => {
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

    pub(crate) fn get_variable_name_of_definer_ty(&self, ty_name: &str) -> Option<String> {
        for d in self.all_definitions() {
            if let ParsedType::Identifier { s, .. } = d.ty() {
                if s == ty_name {
                    return Some(d.name().to_string());
                }
            }
        }

        None
    }

    pub(crate) fn get_field_ty(&self, field_name: &str) -> &ParsedType {
        for d in self.all_definitions() {
            if d.name() == field_name {
                return d.ty();
            }
        }

        panic!("unable to find field: '{}'", field_name)
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn tags(&self) -> &Tags {
        &self.tags
    }

    pub(crate) fn all_definitions(&self) -> Vec<&ParsedStructMemberDefinition> {
        fn inner<'a>(m: &'a ParsedStructMember, v: &mut Vec<&'a ParsedStructMemberDefinition>) {
            match m {
                ParsedStructMember::Definition(d) => v.push(d),
                ParsedStructMember::IfStatement(statement) => {
                    for m in statement.all_members() {
                        inner(m, v);
                    }
                }
                ParsedStructMember::OptionalStatement(optional) => {
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
        m: &ParsedStructMember,
        containers: &[Self],
        definers: &[Definer],
        sizes: &mut Sizes,
    ) {
        match m {
            ParsedStructMember::Definition(d) => {
                *sizes += d.ty().sizes_parsed(e, containers, definers)
            }
            ParsedStructMember::OptionalStatement(optional) => {
                let minimum = sizes.minimum();

                for m in optional.members() {
                    Self::add_sizes_values(e, m, containers, definers, sizes);
                }

                // The optional statement doesn't have be be here, so the minimum doesn't get incremented
                sizes.set_minimum(minimum);
            }
            ParsedStructMember::IfStatement(statement) => {
                let statement_sizes = Self::get_complex_sizes(statement, e, containers, definers);

                *sizes += statement_sizes;
            }
        }
    }

    pub(crate) fn create_sizes(&self, containers: &[Self], definers: &[Definer]) -> Sizes {
        let mut sizes = Sizes::new();
        for m in self.fields() {
            Self::add_sizes_values(self, m, containers, definers, &mut sizes);
        }

        sizes
    }

    pub(crate) fn get_complex_sizes(
        statement: &ParsedIfStatement,
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

    pub(crate) fn get_type_of_variable(&self, variable_name: &str) -> ParsedType {
        for d in self.all_definitions() {
            if d.name() == variable_name {
                return d.ty().clone();
            }
        }

        panic!("unable to find type {}", variable_name)
    }

    pub(crate) fn fields(&self) -> &[ParsedStructMember] {
        self.members.as_slice()
    }

    pub(crate) fn fields_mut(&mut self) -> &mut [ParsedStructMember] {
        self.members.as_mut_slice()
    }

    pub(crate) fn recursive_only_has_io_errors(
        &self,
        containers: &[Self],
        definers: &[Definer],
    ) -> bool {
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

    pub(crate) fn contains_string_or_cstring(&self) -> bool {
        for d in self.all_definitions() {
            match d.ty() {
                ParsedType::CString | ParsedType::String { .. } | ParsedType::SizedCString => {
                    return true
                }
                ParsedType::Array(array) => {
                    if matches!(array.ty(), ArrayType::CString) {
                        return true;
                    }
                }
                _ => {}
            }
        }

        false
    }

    pub(crate) fn get_types_needing_import(&self) -> Vec<String> {
        self.get_complex_types()
    }

    fn get_complex_types(&self) -> Vec<String> {
        let mut v = Vec::new();

        for d in self.all_definitions() {
            match &d.struct_type() {
                ParsedType::Array(a) => {
                    if let ArrayType::Complex(i) = a.ty() {
                        v.push(i.clone());
                    }
                }
                ParsedType::Identifier { s, .. } => {
                    v.push(s.clone());
                }
                _ => {}
            }
        }

        v.sort_unstable();
        v.dedup();

        v
    }

    pub(crate) fn set_used_in_if(&mut self) {
        let mut variables_used_in_if = Vec::new();

        fn find_used_in_if(m: &ParsedStructMember, variables_used_in_if: &mut Vec<String>) {
            match m {
                ParsedStructMember::Definition(_) => {}
                ParsedStructMember::IfStatement(statement) => {
                    variables_used_in_if.push(statement.name().to_string());

                    for m in statement.all_members() {
                        find_used_in_if(m, variables_used_in_if);
                    }
                }
                ParsedStructMember::OptionalStatement(optional) => {
                    for m in optional.members() {
                        find_used_in_if(m, variables_used_in_if);
                    }
                }
            }
        }

        for m in self.fields() {
            find_used_in_if(m, &mut variables_used_in_if);
        }

        for d in self.all_definitions_mut() {
            d.set_used_in_if(variables_used_in_if.contains(&d.name().to_string()));
        }
    }

    pub(crate) fn all_definitions_mut(&mut self) -> Vec<&mut ParsedStructMemberDefinition> {
        fn inner<'a>(
            m: &'a mut ParsedStructMember,
            v: &mut Vec<&'a mut ParsedStructMemberDefinition>,
        ) {
            match m {
                ParsedStructMember::Definition(d) => v.push(d),
                ParsedStructMember::IfStatement(statement) => {
                    for m in statement.all_members_mut() {
                        inner(m, v);
                    }
                }
                ParsedStructMember::OptionalStatement(optional) => {
                    for m in optional.members_mut() {
                        inner(m, v);
                    }
                }
            }
        }

        let mut v = Vec::new();

        for m in self.fields_mut() {
            inner(m, &mut v);
        }

        v
    }

    fn set_if_statements(&mut self) {
        fn inner(m: &mut ParsedStructMember, c: &ParsedContainer) {
            match m {
                ParsedStructMember::Definition(_) => {}
                ParsedStructMember::IfStatement(statement) => {
                    statement.set_original_ty(c.get_type_of_variable(statement.name()));

                    for else_if in statement.else_ifs_mut() {
                        else_if.set_original_ty(c.get_type_of_variable(else_if.name()));
                    }

                    for m in statement.all_members_mut() {
                        inner(m, c);
                    }
                }
                ParsedStructMember::OptionalStatement(optional) => {
                    for m in optional.members_mut() {
                        inner(m, c);
                    }
                }
            }
        }

        let c = self.clone();
        for m in &mut self.members {
            inner(m, &c);
        }
    }

    pub(crate) fn self_check(&self) {
        self.panic_on_duplicate_field_names();
    }

    pub(crate) fn panic_on_duplicate_field_names(&self) {
        let mut v = Vec::new();

        for d in self.all_definitions() {
            v.push(d.name());
        }
        v.sort_unstable();

        let mut previous_name = "";
        for e in v {
            if e == previous_name {
                panic!(
                    "struct '{struct_name}' contains duplicate fields '{field_name}'",
                    struct_name = self.name(),
                    field_name = e
                );
            }
            previous_name = e;
        }
    }
}
