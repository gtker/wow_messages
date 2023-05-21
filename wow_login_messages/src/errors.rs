use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ParseError {
    opcode: u32,
    message: &'static str,
    kind: ParseErrorKind,
}

impl ParseError {
    pub const fn new(opcode: u32, message: &'static str, kind: ParseErrorKind) -> Self {
        Self {
            opcode,
            message,
            kind,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "'{}' ({:#06X}) had error '{}'",
            self.message, self.opcode, self.kind
        )
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            ParseErrorKind::Io(e) => Some(e),
            ParseErrorKind::Enum(e) => Some(e),
            ParseErrorKind::String(e) => Some(e),
        }
    }
}

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

impl Error for ParseErrorKind {}

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

impl Error for EnumError {}

#[derive(Debug)]
pub enum ExpectedOpcodeError {
    Opcode(u32),
    Parse(ParseError),
    Io(std::io::Error),
}

impl Display for ExpectedOpcodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opcode(opcode) => {
                f.write_str(&format!("unexpected opcode found: '{opcode:#06X}'"))
            }
            Self::Parse(i) => i.fmt(f),
            Self::Io(e) => e.fmt(f),
        }
    }
}

impl Error for ExpectedOpcodeError {}

impl From<ParseError> for ExpectedOpcodeError {
    fn from(e: ParseError) -> Self {
        Self::Parse(e)
    }
}

impl From<std::io::Error> for ExpectedOpcodeError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
