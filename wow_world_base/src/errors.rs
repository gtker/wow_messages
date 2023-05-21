use std::fmt::{Display, Formatter};

/// Error enum for when an integer is out of range.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
            "Enum '{}' can not have value: '{}'",
            self.name, self.value
        ))
    }
}

impl std::error::Error for EnumError {}

/// Errors that can be encountered while parsing messages.
#[derive(Debug)]
pub enum ParseErrorKind {
    Io(std::io::Error),
    Enum(EnumError),
    String(std::string::FromUtf8Error),
    InvalidSize,
    BufferSizeTooSmall(std::io::Error),
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorKind::Io(i) => i.fmt(f),
            ParseErrorKind::Enum(i) => i.fmt(f),
            ParseErrorKind::String(i) => i.fmt(f),
            ParseErrorKind::InvalidSize => f.write_fmt(format_args!("message has invalid size")),
            ParseErrorKind::BufferSizeTooSmall(io) => {
                f.write_fmt(format_args!("buffer too small with io error: '{io}'"))
            }
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
