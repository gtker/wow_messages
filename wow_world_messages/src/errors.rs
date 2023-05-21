use std::fmt::{Display, Formatter};

pub use wow_world_base::EnumError;
pub use wow_world_base::ParseErrorKind;

#[derive(Debug)]
pub enum ExpectedOpcodeError {
    Opcode {
        opcode: u32,
        name: Option<&'static str>,
        size: u32,
    },
    Parse(ParseErrorKind),
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
