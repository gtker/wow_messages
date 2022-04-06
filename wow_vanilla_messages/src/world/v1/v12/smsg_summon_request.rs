use std::convert::{TryFrom, TryInto};
use crate::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_summon_request.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_summon_request.wowm#L3):
/// ```text
/// smsg SMSG_SUMMON_REQUEST = 0x2AB {
///     u64 summoner_guid;
///     u32 zone_id;
///     u32 auto_decline_time_in_msecs;
/// }
/// ```
pub struct SMSG_SUMMON_REQUEST {
    pub summoner_guid: u64,
    pub zone_id: u32,
    pub auto_decline_time_in_msecs: u32,
}

impl WorldServerMessageWrite for SMSG_SUMMON_REQUEST {
    const OPCODE: u16 = 0x2ab;

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
impl WorldMessageBody for SMSG_SUMMON_REQUEST {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // summoner_guid: u64
        let summoner_guid = crate::util::read_u64_le(r)?;

        // zone_id: u32
        let zone_id = crate::util::read_u32_le(r)?;

        // auto_decline_time_in_msecs: u32
        let auto_decline_time_in_msecs = crate::util::read_u32_le(r)?;

        Ok(Self {
            summoner_guid,
            zone_id,
            auto_decline_time_in_msecs,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // summoner_guid: u64
        w.write_all(&self.summoner_guid.to_le_bytes())?;

        // zone_id: u32
        w.write_all(&self.zone_id.to_le_bytes())?;

        // auto_decline_time_in_msecs: u32
        w.write_all(&self.auto_decline_time_in_msecs.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_SUMMON_REQUEST {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_SUMMON_REQUEST {
    fn maximum_possible_size() -> usize {
        8 // summoner_guid: u64
        + 4 // zone_id: u32
        + 4 // auto_decline_time_in_msecs: u32
    }
}

