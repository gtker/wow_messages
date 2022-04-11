use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{FarSightOperation, FarSightOperationError};
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_FAR_SIGHT {
    pub operation: FarSightOperation,
}

impl WorldClientMessageWrite for CMSG_FAR_SIGHT {
    const OPCODE: u32 = 0x27a;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (Self::size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (Self::size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_FAR_SIGHT {
    type Error = CMSG_FAR_SIGHTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // operation: FarSightOperation
        let operation = FarSightOperation::read(r)?;

        Ok(Self {
            operation,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // operation: FarSightOperation
        self.operation.write(w)?;

        Ok(())
    }
}

impl ConstantSized for CMSG_FAR_SIGHT {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_FAR_SIGHT {
    fn maximum_possible_size() -> usize {
        FarSightOperation::size() // operation: FarSightOperation
    }
}

#[derive(Debug)]
pub enum CMSG_FAR_SIGHTError {
    Io(std::io::Error),
    FarSightOperation(FarSightOperationError),
}

impl std::error::Error for CMSG_FAR_SIGHTError {}
impl std::fmt::Display for CMSG_FAR_SIGHTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::FarSightOperation(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_FAR_SIGHTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<FarSightOperationError> for CMSG_FAR_SIGHTError {
    fn from(e: FarSightOperationError) -> Self {
        Self::FarSightOperation(e)
    }
}

