use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new_all.wowm:2061`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new_all.wowm#L2061):
/// ```text
/// smsg SMSG_LOOT_MONEY_NOTIFY = 0x163 {
///     u32 amount;
/// }
/// ```
pub struct SMSG_LOOT_MONEY_NOTIFY {
    pub amount: u32,
}

impl WorldServerMessageWrite for SMSG_LOOT_MONEY_NOTIFY {
    const OPCODE: u16 = 0x163;

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
impl WorldMessageBody for SMSG_LOOT_MONEY_NOTIFY {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount: u32
        let amount = crate::util::read_u32_le(r)?;

        Ok(Self {
            amount,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount: u32
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_LOOT_MONEY_NOTIFY {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_LOOT_MONEY_NOTIFY {
    fn maximum_possible_size() -> usize {
        4 // amount: u32
    }
}

