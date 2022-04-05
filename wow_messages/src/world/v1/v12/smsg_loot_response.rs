use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{LootMethod, LootMethodError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/smsg_loot_response.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/smsg_loot_response.wowm#L3):
/// ```text
/// smsg SMSG_LOOT_RESPONSE = 0x160 {
///     u64 guid;
///     LootMethod loot_method;
/// }
/// ```
pub struct SMSG_LOOT_RESPONSE {
    pub guid: u64,
    pub loot_method: LootMethod,
}

impl WorldServerMessageWrite for SMSG_LOOT_RESPONSE {
    const OPCODE: u16 = 0x160;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_LOOT_RESPONSE {
    type Error = SMSG_LOOT_RESPONSEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        // loot_method: LootMethod
        let loot_method = LootMethod::read(r)?;

        Ok(Self {
            guid,
            loot_method,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        // loot_method: LootMethod
        self.loot_method.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_LOOT_RESPONSE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_LOOT_RESPONSE {
    fn maximum_possible_size() -> usize {
        8 // guid: u64
        + LootMethod::size() // loot_method: LootMethod
    }
}

#[derive(Debug)]
pub enum SMSG_LOOT_RESPONSEError {
    Io(std::io::Error),
    LootMethod(LootMethodError),
}

impl std::error::Error for SMSG_LOOT_RESPONSEError {}
impl std::fmt::Display for SMSG_LOOT_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::LootMethod(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_LOOT_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<LootMethodError> for SMSG_LOOT_RESPONSEError {
    fn from(e: LootMethodError) -> Self {
        Self::LootMethod(e)
    }
}

