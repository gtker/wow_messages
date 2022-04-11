use crate::file_info::FileInfo;
use crate::parser::types::tags::Tags;
use crate::parser::types::IntegerType;
use crate::parser::utility;
use crate::ENUM_SELF_VALUE_FIELD;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DefinerField {
    name: String,
    value: DefinerValue,
    kv: Tags,
}

impl DefinerField {
    pub fn new(name: &str, value: DefinerValue) -> Self {
        Self::key_value(name, value, Tags::new())
    }

    pub fn key_value(name: &str, value: DefinerValue, kv: Tags) -> Self {
        Self {
            name: name.to_string(),
            value,
            kv,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &DefinerValue {
        &self.value
    }

    pub fn tags(&self) -> &Tags {
        &self.kv
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
    kv: Tags,
}

impl SelfValueDefinerField {
    pub fn new(name: &str, kv: Tags) -> Self {
        Self {
            name: name.to_string(),
            kv,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kvs(&self) -> &Tags {
        &self.kv
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Definer {
    name: String,
    fields: Vec<DefinerField>,
    self_value: Option<SelfValueDefinerField>,
    basic_type: IntegerType,
    extra_key_value: Tags,
    file_info: FileInfo,
}

impl Definer {
    pub fn new(
        name: &str,
        fields: Vec<DefinerField>,
        basic_type: IntegerType,
        self_value: Option<SelfValueDefinerField>,
        extras: Tags,
        file_info: FileInfo,
    ) -> Self {
        Self {
            name: name.to_string(),
            fields,
            self_value,
            basic_type,
            extra_key_value: extras,
            file_info,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
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
        &self.extra_key_value
    }

    pub fn file_info(&self) -> &FileInfo {
        &self.file_info
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags().contains(tag)
    }
}
