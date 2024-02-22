use std::error::Error;
use std::fmt::{Display, Formatter};
use wow_world_base::DateTimeError;

pub use wow_world_base::EnumError;

pub(crate) const MAX_ALLOCATION_SIZE: u32 = 0x7F_FF_FF_FF;

#[derive(Debug)]
pub struct ParseError {
    opcode: u32,
    message: &'static str,
    size: u32,
    kind: ParseErrorKind,
}

impl ParseError {
    pub const fn new(opcode: u32, message: &'static str, size: u32, kind: ParseErrorKind) -> Self {
        Self {
            opcode,
            message,
            size,
            kind,
        }
    }

    pub(crate) fn opcode_convert(mut self) -> Self {
        if let ParseErrorKind::Io(e) = self.kind {
            self.kind = ParseErrorKind::BufferSizeTooSmall(e);
        }

        self
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "'{}' ({:#06X}, size {}) had error '{}'",
            self.message, self.opcode, self.size, self.kind
        )
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            ParseErrorKind::Io(e) => Some(e),
            ParseErrorKind::Enum(e) => Some(e),
            ParseErrorKind::String(e) => Some(e),
            ParseErrorKind::BufferSizeTooSmall(io) => Some(io),
            ParseErrorKind::DateTime(e) => Some(e),
            ParseErrorKind::InvalidSize { .. } => None,
            ParseErrorKind::AllocationTooLargeError(_) => None,
        }
    }
}

#[derive(Debug)]
pub enum ExpectedOpcodeError {
    Opcode {
        opcode: u32,
        name: Option<&'static str>,
        size: u32,
    },
    Parse(ParseError),
    Io(std::io::Error),
}

impl Display for ExpectedOpcodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opcode { opcode, name, size } => {
                if let Some(name) = name {
                    write!(
                        f,
                        "unexpected message '{name}' found: opcode '{opcode:#06X}' and size '{size}'"
                    )
                } else {
                    write!(
                        f,
                        "unexpected opcode without name found: '{opcode:#06X}' and size '{size}'"
                    )
                }
            }
            Self::Parse(i) => i.fmt(f),
            ExpectedOpcodeError::Io(i) => i.fmt(f),
        }
    }
}

impl std::error::Error for ExpectedOpcodeError {}

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

/// Errors that can be encountered while parsing messages.
#[derive(Debug)]
pub enum ParseErrorKind {
    Io(std::io::Error),
    Enum(EnumError),
    DateTime(DateTimeError),
    String(std::string::FromUtf8Error),
    InvalidSize,
    BufferSizeTooSmall(std::io::Error),
    AllocationTooLargeError(u64),
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
            ParseErrorKind::DateTime(i) => i.fmt(f),
            ParseErrorKind::AllocationTooLargeError(i) => {
                write!(f, "message attempts to allocate buffer of size {i}")
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

impl From<DateTimeError> for ParseErrorKind {
    fn from(value: DateTimeError) -> Self {
        Self::DateTime(value)
    }
}
