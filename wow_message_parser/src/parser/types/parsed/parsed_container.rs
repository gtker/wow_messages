use crate::error_printer::{complex_not_found, duplicate_field_names};
use crate::file_info::FileInfo;
use crate::parser::types::definer::Definer;
use crate::parser::types::if_statement::DefinerUsage;
use crate::parser::types::objects::conversion::{get_container, get_definer, get_related};
use crate::parser::types::parsed::parsed_array::{ParsedArraySize, ParsedArrayType};
use crate::parser::types::parsed::parsed_if_statement::ParsedIfStatement;
use crate::parser::types::parsed::parsed_struct_member::{
    ParsedStructMember, ParsedStructMemberDefinition,
};
use crate::parser::types::parsed::parsed_ty::ParsedType;
use crate::parser::types::sizes::Sizes;
use crate::parser::types::tags::ObjectTags;
use crate::{ContainerType, DefinerType, MAX_ALLOCATION_SIZE, MAX_ALLOCATION_SIZE_WRATH};

#[derive(Debug, Clone)]
pub(crate) struct ParsedContainer {
    pub name: String,
    pub object_type: ContainerType,
    pub members: Vec<ParsedStructMember>,
    pub tags: ObjectTags,
    pub file_info: FileInfo,
}

impl ParsedContainer {
    pub(crate) fn new(
        name: &str,
        members: Vec<ParsedStructMember>,
        tags: ObjectTags,
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
    pub(crate) fn contains_container(&self, ty_name: &str) -> bool {
        for d in self.all_definitions() {
            match d.ty() {
                ParsedType::Array(array) => {
                    if let ParsedArrayType::Complex(s) = array.ty() {
                        if s == ty_name {
                            return true;
                        }
                    }
                }
                ParsedType::Identifier { s, .. } => {
                    if s == ty_name {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    pub(crate) fn contains_definer(&self, ty_name: &str) -> Option<DefinerUsage> {
        fn inner(
            m: &ParsedStructMember,
            ty_name: &str,
            variable_name: &str,
        ) -> Option<DefinerUsage> {
            match m {
                ParsedStructMember::Definition(d) => {
                    if let ParsedType::Identifier { s, .. } = d.ty() {
                        if s == ty_name {
                            return Some(DefinerUsage::NotInIf);
                        }
                    }
                }
                ParsedStructMember::IfStatement(statement) => {
                    if statement.name() == variable_name {
                        return Some(DefinerUsage::InIf);
                    }

                    let mut not_in_if = false;
                    for m in statement.all_members() {
                        match inner(m, ty_name, variable_name) {
                            None => {}
                            Some(DefinerUsage::NotInIf) => not_in_if = true,
                            Some(DefinerUsage::InIf) => return Some(DefinerUsage::InIf),
                        }
                    }

                    if not_in_if {
                        return Some(DefinerUsage::NotInIf);
                    }
                }
                ParsedStructMember::OptionalStatement(optional) => {
                    let mut not_in_if = false;

                    for m in optional.members() {
                        match inner(m, ty_name, variable_name) {
                            None => {}
                            Some(DefinerUsage::NotInIf) => not_in_if = true,
                            Some(DefinerUsage::InIf) => return Some(DefinerUsage::InIf),
                        }
                    }

                    if not_in_if {
                        return Some(DefinerUsage::NotInIf);
                    }
                }
            }

            None
        }

        let variable_name = self.get_variable_name_of_definer_ty(ty_name);

        if let Some(variable_name) = variable_name {
            let mut not_in_if = false;

            for m in self.fields() {
                match inner(m, ty_name, &variable_name) {
                    None => {}
                    Some(DefinerUsage::NotInIf) => not_in_if = true,
                    Some(DefinerUsage::InIf) => return Some(DefinerUsage::InIf),
                }
            }

            if not_in_if {
                return Some(DefinerUsage::NotInIf);
            }
        }

        None
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

    pub(crate) fn get_field(&self, field_name: &str) -> ParsedStructMemberDefinition {
        for d in self.all_definitions() {
            if d.name() == field_name {
                return d.clone();
            }
        }

        panic!("unable to find field: '{field_name}'")
    }

    pub(crate) fn get_field_ty(&self, field_name: &str) -> &ParsedType {
        for d in self.all_definitions() {
            if d.name() == field_name {
                return d.ty();
            }
        }

        panic!("unable to find field: '{field_name}'")
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn tags(&self) -> &ObjectTags {
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
        if self.contains_type_that_can_have_allocation_too_large_error(containers, definers) {
            return false;
        }

        if self.contains_string_or_cstring() {
            return false;
        }

        if self.contains_datetime() {
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
                let related = get_related(containers, definers, &t);
                complex_not_found(self.name(), self.tags(), &self.file_info, &t, &related);
            }
        }

        true
    }

    pub(crate) fn contains_datetime(&self) -> bool {
        for d in self.all_definitions() {
            if d.ty() == &ParsedType::DateTime {
                return true;
            }
        }

        false
    }

    pub(crate) fn contains_type_that_can_have_allocation_too_large_error(
        &self,
        containers: &[Self],
        definers: &[Definer],
    ) -> bool {
        for d in self.all_definitions() {
            match d.ty() {
                ParsedType::SizedCString => return true,
                ParsedType::Array(array) => {
                    if let ParsedArraySize::Variable(m) = array.size() {
                        let field = self.get_field(&m);
                        let size_field_max_value = match field.ty() {
                            ParsedType::Integer(i) => i.largest_value(),
                            _ => panic!(),
                        };

                        let object_min_size = array
                            .inner_sizes(containers, definers, self.tags())
                            .minimum();

                        let max_size = if self.tags.contains_wrath() {
                            MAX_ALLOCATION_SIZE_WRATH
                        } else {
                            MAX_ALLOCATION_SIZE
                        };

                        if self.tags().has_world_version()
                            && size_field_max_value * object_min_size > max_size
                        {
                            return true;
                        }
                    }
                }
                _ => {}
            }
        }

        false
    }

    pub(crate) fn contains_string_or_cstring(&self) -> bool {
        for d in self.all_definitions() {
            match d.ty() {
                ParsedType::CString | ParsedType::String { .. } | ParsedType::SizedCString => {
                    return true
                }
                ParsedType::Array(array) => {
                    if matches!(array.ty(), ParsedArrayType::CString) {
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
                    if let ParsedArrayType::Complex(i) = a.ty() {
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
                    statement.set_original_ty(c.get_field_ty(statement.name()).clone());

                    for else_if in statement.else_ifs_mut() {
                        else_if.set_original_ty(c.get_field_ty(else_if.name()).clone());
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
                duplicate_field_names(self.name(), e, &self.file_info);
            }
            previous_name = e;
        }
    }

    pub(crate) fn enum_type_used_in_separate_if_statements(&self, ty_name: &str) -> bool {
        let variable_name = self.get_variable_name_of_definer_ty(ty_name).unwrap();
        self.enum_variable_used_in_separate_if_statements(&variable_name)
    }

    pub(crate) fn enum_variable_used_in_separate_if_statements(&self, variable_name: &str) -> bool {
        fn inner(b: &ParsedStructMember, a: &mut i32, variable_name: &str) {
            match b {
                ParsedStructMember::Definition(_) => {}
                ParsedStructMember::IfStatement(statement) => {
                    if statement.name() == variable_name {
                        *a += 1;
                    }

                    for m in statement.all_members() {
                        inner(m, a, variable_name);
                    }
                }
                ParsedStructMember::OptionalStatement(optional) => {
                    for m in optional.members() {
                        inner(m, a, variable_name);
                    }
                }
            }
        }

        let mut a = 0;
        for member in &self.members {
            inner(member, &mut a, variable_name);
        }

        a > 1
    }
}
