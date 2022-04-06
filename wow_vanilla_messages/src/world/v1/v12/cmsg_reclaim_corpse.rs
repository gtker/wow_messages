use std::convert::{TryFrom, TryInto};
use crate::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/resurrect/cmsg_reclaim_corpse.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/resurrect/cmsg_reclaim_corpse.wowm#L3):
/// ```text
/// cmsg CMSG_RECLAIM_CORPSE = 0x1D2 {
///     u64 guid;
/// }
/// ```
pub struct CMSG_RECLAIM_CORPSE {
    pub guid: u64,
}

impl WorldClientMessageWrite for CMSG_RECLAIM_CORPSE {
    const OPCODE: u32 = 0x1d2;

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
impl WorldMessageBody for CMSG_RECLAIM_CORPSE {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        Ok(Self {
            guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_RECLAIM_CORPSE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_RECLAIM_CORPSE {
    fn maximum_possible_size() -> usize {
        8 // guid: u64
    }
}

