use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{NpcTextUpdate, NpcTextUpdateError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:4833`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L4833):
/// ```text
/// smsg SMSG_NPC_TEXT_UPDATE = 0x180 {
///     u32 text_id;
///     f32 probability;
///     NpcTextUpdate[8] texts;
/// }
/// ```
pub struct SMSG_NPC_TEXT_UPDATE {
    pub text_id: u32,
    pub probability: f32,
    pub texts: [NpcTextUpdate; 8],
}

impl WorldServerMessageWrite for SMSG_NPC_TEXT_UPDATE {
    const OPCODE: u16 = 0x180;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_NPC_TEXT_UPDATE {
    type Error = SMSG_NPC_TEXT_UPDATEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // text_id: u32
        let text_id = crate::util::read_u32_le(r)?;

        // probability: f32
        let probability = crate::util::read_f32_le(r)?;
        // texts: NpcTextUpdate[8]
        let mut texts = Vec::with_capacity(8 as usize);
        for i in 0..8 {
            texts.push(NpcTextUpdate::read(r)?);
        }
        let texts = texts.try_into().unwrap();

        Ok(Self {
            text_id,
            probability,
            texts,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // text_id: u32
        w.write_all(&self.text_id.to_le_bytes())?;

        // probability: f32
        w.write_all(&self.probability.to_le_bytes())?;

        // texts: NpcTextUpdate[8]
        for i in self.texts.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_NPC_TEXT_UPDATE {
    fn size(&self) -> usize {
        4 // text_id: u32
        + 4 // probability: f32
        + self.texts.iter().fold(0, |acc, x| acc + x.size()) // texts: NpcTextUpdate[8]
    }
}

impl MaximumPossibleSized for SMSG_NPC_TEXT_UPDATE {
    fn maximum_possible_size() -> usize {
        4 // text_id: u32
        + 4 // probability: f32
        + 8 * NpcTextUpdate::maximum_possible_size() // texts: NpcTextUpdate[8]
    }
}

#[derive(Debug)]
pub enum SMSG_NPC_TEXT_UPDATEError {
    Io(std::io::Error),
    NpcTextUpdate(NpcTextUpdateError),
}

impl std::error::Error for SMSG_NPC_TEXT_UPDATEError {}
impl std::fmt::Display for SMSG_NPC_TEXT_UPDATEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::NpcTextUpdate(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_NPC_TEXT_UPDATEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<NpcTextUpdateError> for SMSG_NPC_TEXT_UPDATEError {
    fn from(e: NpcTextUpdateError) -> Self {
        Self::NpcTextUpdate(e)
    }
}

