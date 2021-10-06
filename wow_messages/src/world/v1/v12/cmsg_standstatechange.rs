use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{UnitStandState, UnitStandStateError};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/add_messages.wowm:839`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/add_messages.wowm):
/// ```text
/// cmsg CMSG_STANDSTATECHANGE = 0x101 {
///     UnitStandState animation_state;
/// }
/// ```
pub struct CMSG_STANDSTATECHANGE {
    pub animation_state: UnitStandState,
}

impl WorldClientMessageWrite for CMSG_STANDSTATECHANGE {
    const OPCODE: u32 = 0x101;

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
impl WorldMessageBody for CMSG_STANDSTATECHANGE {
    type Error = CMSG_STANDSTATECHANGEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // animation_state: UnitStandState
        let animation_state = UnitStandState::read_u32_le(r)?;

        Ok(Self {
            animation_state,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // animation_state: UnitStandState
        self.animation_state.write_u32_le(w)?;

        Ok(())
    }
}

impl ConstantSized for CMSG_STANDSTATECHANGE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_STANDSTATECHANGE {
    fn maximum_possible_size() -> usize {
        4 // animation_state: UnitStandState upcasted to u32
    }
}

#[derive(Debug)]
pub enum CMSG_STANDSTATECHANGEError {
    Io(std::io::Error),
    UnitStandState(UnitStandStateError),
}

impl std::error::Error for CMSG_STANDSTATECHANGEError {}
impl std::fmt::Display for CMSG_STANDSTATECHANGEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::UnitStandState(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_STANDSTATECHANGEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<UnitStandStateError> for CMSG_STANDSTATECHANGEError {
    fn from(e: UnitStandStateError) -> Self {
        Self::UnitStandState(e)
    }
}

