use crate::container::DefinerUsage;
use crate::file_info::FileInfo;
use crate::parser::types::tags::Tags;
use crate::parser::types::IntegerType;
use crate::parser::utility;
use crate::rust_printer::DefinerType;
use crate::ENUM_SELF_VALUE_FIELD;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DefinerField {
    name: String,
    value: DefinerValue,
    tags: Tags,
}

impl DefinerField {
    pub fn new(name: &str, value: DefinerValue) -> Self {
        Self::key_value(name, value, Tags::new())
    }

    pub fn key_value(name: &str, value: DefinerValue, tags: Tags) -> Self {
        Self {
            name: name.to_string(),
            value,
            tags,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &DefinerValue {
        &self.value
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DefinerValue {
    int: u64,
    original: String,
}

impl DefinerValue {
    pub fn int(&self) -> u64 {
        self.int
    }

    pub fn original(&self) -> &str {
        &self.original
    }
}

impl From<&str> for DefinerValue {
    fn from(s: &str) -> Self {
        let v = utility::parse_value(s);
        if let Ok(v) = v {
            return Self {
                int: v,
                original: s.to_string(),
            };
        }
        if s == ENUM_SELF_VALUE_FIELD {
            panic!("self.value passed to DefinerValue From<&str>");
        }

        panic!("complex passed to definer {:#?}", s);
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SelfValueDefinerField {
    name: String,
    tags: Tags,
}

impl SelfValueDefinerField {
    pub fn new(name: &str, tags: Tags) -> Self {
        Self {
            name: name.to_string(),
            tags,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Definer {
    name: String,
    definer_ty: DefinerType,
    fields: Vec<DefinerField>,
    self_value: Option<SelfValueDefinerField>,
    basic_type: IntegerType,
    tags: Tags,
    objects_used_in: Option<Vec<(String, DefinerUsage)>>,
    file_info: FileInfo,
}

impl Definer {
    pub fn new(
        name: &str,
        definer_ty: DefinerType,
        fields: Vec<DefinerField>,
        basic_type: IntegerType,
        self_value: Option<SelfValueDefinerField>,
        tags: Tags,
        file_info: FileInfo,
    ) -> Self {
        Self {
            name: name.to_string(),
            definer_ty,
            fields,
            self_value,
            basic_type,
            tags,
            objects_used_in: None,
            file_info,
        }
    }

    pub fn only_used_in_if(&self) -> bool {
        for v in self.objects_used_in() {
            match v.1 {
                DefinerUsage::NotInIf => return false,
                DefinerUsage::InIf => {}
                _ => unreachable!(),
            }
        }

        true
    }

    pub fn objects_used_in(&self) -> &[(String, DefinerUsage)] {
        self.objects_used_in.as_ref().unwrap()
    }

    pub fn set_objects_used_in(&mut self, mut objects_used_in: Vec<(String, DefinerUsage)>) {
        objects_used_in.sort_by(|a, b| a.0.cmp(&b.0));
        self.objects_used_in = Some(objects_used_in);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn definer_ty(&self) -> DefinerType {
        self.definer_ty
    }

    pub fn ty(&self) -> &IntegerType {
        &self.basic_type
    }

    pub fn self_value(&self) -> &Option<SelfValueDefinerField> {
        &self.self_value
    }

    pub fn fields(&self) -> &[DefinerField] {
        &self.fields
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }

    pub fn file_info(&self) -> &FileInfo {
        &self.file_info
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags().contains(tag)
    }

    pub fn get_field_with_value(&self, value: usize) -> Option<&DefinerField> {
        let value = value as u64;
        self.fields.iter().find(|a| a.value.int == value)
    }

    pub fn get_set_flag_fields(&self, value: usize) -> Vec<&DefinerField> {
        let mut v = Vec::new();
        let value = value as u64;

        for f in self.fields() {
            let val = f.value().int();
            if val == 0 && value == 0 {
                v.push(f);
            } else if (value & f.value().int()) != 0 {
                v.push(f);
            }
        }

        v
    }
}
