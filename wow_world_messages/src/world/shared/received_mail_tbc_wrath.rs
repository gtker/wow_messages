use crate::Guid;
use wow_world_base::shared::mail_message_type_vanilla_tbc_wrath::MailMessageType;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_query_next_mail_time_server.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_query_next_mail_time_server.wowm#L29):
/// ```text
/// struct ReceivedMail {
///     Guid sender;
///     u32 auction_house_id;
///     MailMessageType message_type;
///     u32 stationery;
///     f32 time;
/// }
/// ```
pub struct ReceivedMail {
    pub sender: Guid,
    pub auction_house_id: u32,
    pub message_type: MailMessageType,
    pub stationery: u32,
    /// mangosone sets to `0xC6000000`
    /// mangosone: float unk, time or something
    ///
    pub time: f32,
}

impl ReceivedMail {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // sender: Guid
        w.write_all(&self.sender.guid().to_le_bytes())?;

        // auction_house_id: u32
        w.write_all(&self.auction_house_id.to_le_bytes())?;

        // message_type: MailMessageType
        w.write_all(&(self.message_type.as_int() as u32).to_le_bytes())?;

        // stationery: u32
        w.write_all(&self.stationery.to_le_bytes())?;

        // time: f32
        w.write_all(&self.time.to_le_bytes())?;

        Ok(())
    }
}

impl ReceivedMail {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // sender: Guid
        let sender = Guid::read(r)?;

        // auction_house_id: u32
        let auction_house_id = crate::util::read_u32_le(r)?;

        // message_type: MailMessageType
        let message_type: MailMessageType = crate::util::read_u32_le(r)?.try_into()?;

        // stationery: u32
        let stationery = crate::util::read_u32_le(r)?;

        // time: f32
        let time = crate::util::read_f32_le(r)?;
        Ok(Self {
            sender,
            auction_house_id,
            message_type,
            stationery,
            time,
        })
    }

}

