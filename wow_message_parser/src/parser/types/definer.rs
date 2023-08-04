use crate::error_printer::{
    definer_with_invalid_value, duplicate_definer_value, invalid_definer_value,
};
use crate::file_info::FileInfo;
use crate::parser::types::if_statement::DefinerUsage;
use crate::parser::types::tags::{MemberTags, ObjectTags};
use crate::parser::types::{compare_name_and_tags, IntegerType};
use crate::parser::utility;
use crate::rust_printer::{field_name_to_rust_name, DefinerType};
use hashbrown::HashMap;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct DefinerField {
    name: String,
    rust_name: String,
    value: DefinerValue,
    tags: MemberTags,
}

impl DefinerField {
    pub(crate) fn new(name: &str, value: DefinerValue, tags: MemberTags) -> Self {
        Self {
            name: name.to_string(),
            rust_name: field_name_to_rust_name(name),
            value,
            tags,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn function_name(&self) -> String {
        self.name.to_lowercase()
    }

    pub(crate) fn rust_name(&self) -> &str {
        &self.rust_name
    }

    pub(crate) fn value(&self) -> &DefinerValue {
        &self.value
    }

    pub(crate) fn tags(&self) -> &MemberTags {
        &self.tags
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct DefinerValue {
    int: i128,
    original: String,
}

impl DefinerValue {
    pub(crate) fn int(&self) -> i128 {
        self.int
    }

    pub(crate) fn original(&self) -> &str {
        &self.original
    }

    pub(crate) fn from_str(
        s: &str,
        ty_name: &str,
        enumerator_name: &str,
        file_info: &FileInfo,
    ) -> Self {
        let v = utility::parse_value(s);
        if let Some(v) = v {
            return Self {
                int: v,
                original: s.to_string(),
            };
        }

        invalid_definer_value(ty_name, enumerator_name, s, file_info);
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct Definer {
    name: String,
    definer_ty: DefinerType,
    fields: Vec<DefinerField>,
    basic_type: IntegerType,
    tags: ObjectTags,
    objects_used_in: Vec<(String, DefinerUsage)>,
    file_info: FileInfo,
}

impl Ord for Definer {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_first = self.tags().all_versions();
        let other_first = other.tags().all_versions();

        compare_name_and_tags(&self.name, self_first, &other.name, other_first)
    }
}

impl PartialOrd for Definer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Definer {
    pub(crate) fn new(
        name: String,
        definer_ty: DefinerType,
        fields: Vec<DefinerField>,
        basic_type: IntegerType,
        tags: ObjectTags,
        objects_used_in: Vec<(String, DefinerUsage)>,
        file_info: FileInfo,
    ) -> Self {
        let (min, max) = (basic_type.smallest_value(), basic_type.largest_value());
        for field in &fields {
            let value = field.value().int();
            if value > max || value < min {
                definer_with_invalid_value(&name, &file_info, basic_type, value);
            }
        }

        let s = Self {
            name,
            definer_ty,
            fields,
            basic_type,
            tags,
            objects_used_in,
            file_info,
        };

        s.self_check();

        s
    }

    pub(crate) fn only_used_in_if(&self) -> bool {
        for v in self.objects_used_in() {
            match v.1 {
                DefinerUsage::NotInIf => return false,
                DefinerUsage::InIf => {}
            }
        }

        true
    }

    pub(crate) fn objects_used_in(&self) -> &[(String, DefinerUsage)] {
        &self.objects_used_in
    }

    pub(crate) fn used_in_if_in_object(&self, name: &str) -> bool {
        self.objects_used_in()
            .iter()
            .any(|a| a.0 == name && a.1 == DefinerUsage::InIf)
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn definer_ty(&self) -> DefinerType {
        self.definer_ty
    }

    pub(crate) fn ty(&self) -> &IntegerType {
        &self.basic_type
    }

    pub(crate) fn fields(&self) -> &[DefinerField] {
        &self.fields
    }

    pub(crate) fn tags(&self) -> &ObjectTags {
        &self.tags
    }

    pub(crate) fn file_info(&self) -> &FileInfo {
        &self.file_info
    }

    pub(crate) fn get_field_with_name(&self, name: &str) -> Option<&DefinerField> {
        self.fields.iter().find(|a| a.name() == name)
    }

    pub(crate) fn get_field_with_value(&self, value: i128) -> Option<&DefinerField> {
        self.fields.iter().find(|a| a.value.int == value)
    }

    pub(crate) fn hex_digit_width(&self) -> usize {
        let mut width = 0;

        for enumerator in self.fields() {
            let len = format!("{:#0X}", enumerator.value.int()).len();
            if len > width {
                width = len;
            }
        }

        width
    }

    pub(crate) fn get_set_flag_fields(&self, value: i128) -> Vec<&DefinerField> {
        let mut v = Vec::new();

        for f in self.fields() {
            let val = f.value().int();
            if (val == 0 && value == 0) || (value & f.value().int()) != 0 {
                v.push(f);
            }
        }

        v
    }

    fn self_check(&self) {
        let mut h = HashMap::new();

        for field in &self.fields {
            if let Some(other) = h.get(&field.value().int) {
                duplicate_definer_value(
                    self.name(),
                    other,
                    field.name(),
                    field.value().int,
                    self.file_info(),
                );
            }

            h.insert(field.value.int, &field.name);
        }
    }
}
