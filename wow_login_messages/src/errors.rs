use std::fmt::{format, Debug, Display, Formatter};

#[derive(Debug)]
pub struct EnumError {
    pub name: &'static str,
    pub value: u32,
}

impl EnumError {
    pub fn new(name: &'static str, value: u32) -> Self {
        Self { name, value }
    }
}

impl Display for EnumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Enum {} can not have value: '{}'",
            self.name, self.value
        ))
    }
}

impl std::error::Error for EnumError {}
