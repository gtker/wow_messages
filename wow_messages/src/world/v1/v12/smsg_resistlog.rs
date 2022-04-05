use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new5.wowm:251`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new5.wowm#L251):
/// ```text
/// smsg SMSG_RESISTLOG = 0x1D6 {
///     u64 guid1;
///     u64 guid2;
///     u32 unknown1;
///     f32 unknown2;
///     f32 unknown3;
///     u32 unknown4;
///     u32 unknown5;
/// }
/// ```
pub struct SMSG_RESISTLOG {
    pub guid1: u64,
    pub guid2: u64,
    pub unknown1: u32,
    pub unknown2: f32,
    pub unknown3: f32,
    pub unknown4: u32,
    pub unknown5: u32,
}

impl WorldServerMessageWrite for SMSG_RESISTLOG {
    const OPCODE: u16 = 0x1d6;

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
impl WorldMessageBody for SMSG_RESISTLOG {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid1: u64
        let guid1 = crate::util::read_u64_le(r)?;

        // guid2: u64
        let guid2 = crate::util::read_u64_le(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // unknown2: f32
        let unknown2 = crate::util::read_f32_le(r)?;
        // unknown3: f32
        let unknown3 = crate::util::read_f32_le(r)?;
        // unknown4: u32
        let unknown4 = crate::util::read_u32_le(r)?;

        // unknown5: u32
        let unknown5 = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid1,
            guid2,
            unknown1,
            unknown2,
            unknown3,
            unknown4,
            unknown5,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid1: u64
        w.write_all(&self.guid1.to_le_bytes())?;

        // guid2: u64
        w.write_all(&self.guid2.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: f32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: f32
        w.write_all(&self.unknown3.to_le_bytes())?;

        // unknown4: u32
        w.write_all(&self.unknown4.to_le_bytes())?;

        // unknown5: u32
        w.write_all(&self.unknown5.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_RESISTLOG {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_RESISTLOG {
    fn maximum_possible_size() -> usize {
        8 // guid1: u64
        + 8 // guid2: u64
        + 4 // unknown1: u32
        + 4 // unknown2: f32
        + 4 // unknown3: f32
        + 4 // unknown4: u32
        + 4 // unknown5: u32
    }
}

