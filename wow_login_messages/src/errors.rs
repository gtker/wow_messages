use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParseErrorKind {
    Io(std::io::Error),
    Enum(EnumError),
    String(std::string::FromUtf8Error),
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorKind::Io(i) => i.fmt(f),
            ParseErrorKind::Enum(i) => i.fmt(f),
            ParseErrorKind::String(i) => i.fmt(f),
        }
    }
}

impl std::error::Error for ParseErrorKind {}

impl From<EnumError> for ParseErrorKind {
    fn from(e: EnumError) -> Self {
        Self::Enum(e)
    }
}

impl From<std::string::FromUtf8Error> for ParseErrorKind {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<std::io::Error> for ParseErrorKind {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

#[derive(Debug)]
pub struct EnumError {
    pub name: &'static str,
    pub value: i128,
}

impl EnumError {
    pub const fn new(name: &'static str, value: i128) -> Self {
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

#[derive(Debug)]
pub enum ExpectedOpcodeError {
    Opcode(u32),
    Parse(ParseErrorKind),
}

impl Display for ExpectedOpcodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opcode(opcode) => {
                f.write_str(&format!("unexpected opcode found: '{opcode:#06X}'"))
            }
            Self::Parse(i) => i.fmt(f),
        }
    }
}

impl std::error::Error for ExpectedOpcodeError {}

impl From<ParseErrorKind> for ExpectedOpcodeError {
    fn from(e: ParseErrorKind) -> Self {
        Self::Parse(e)
    }
}

impl From<std::io::Error> for ExpectedOpcodeError {
    fn from(e: std::io::Error) -> Self {
        Self::Parse(e.into())
    }
}
