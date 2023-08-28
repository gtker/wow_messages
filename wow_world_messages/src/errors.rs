use std::error::Error;
use std::fmt::{Display, Formatter};

pub use wow_world_base::EnumError;
pub use wow_world_base::ParseErrorKind;
use wow_world_base::ParseErrorKind::BufferSizeTooSmall;

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
            self.kind = BufferSizeTooSmall(e);
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
            ParseErrorKind::InvalidSize { .. } => None,
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
