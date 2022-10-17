use std::fmt::{Display, Formatter};
pub use wow_world_base::EnumError;

#[derive(Debug)]
pub enum ParseError {
    Io(std::io::Error),
    Enum(EnumError),
    String(std::string::FromUtf8Error),
    InvalidSize(u32),
    BufferSizeTooSmall {
        opcode: u32,
        size: u32,
        io: std::io::Error,
    },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Io(i) => i.fmt(f),
            ParseError::Enum(i) => i.fmt(f),
            ParseError::String(i) => i.fmt(f),
            ParseError::InvalidSize(i) => {
                f.write_fmt(format_args!("message has invalid size: '{}'", i))
            }
            ParseError::BufferSizeTooSmall { opcode, size, io } => f.write_fmt(format_args!(
                "opcode '{}' has received too small size '{}' with io error: '{}'",
                opcode, size, io
            )),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<EnumError> for ParseError {
    fn from(e: EnumError) -> Self {
        Self::Enum(e)
    }
}

impl From<std::string::FromUtf8Error> for ParseError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<std::io::Error> for ParseError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

#[derive(Debug)]
pub enum ExpectedOpcodeError {
    Opcode { opcode: u32, size: u32 },
    Parse(ParseError),
}

impl Display for ExpectedOpcodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opcode { opcode, size } => write!(
                f,
                "unexpected opcode found: '{:#06X}' and size '{}'",
                opcode, size
            ),
            Self::Parse(i) => i.fmt(f),
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
        Self::Parse(e.into())
    }
}
