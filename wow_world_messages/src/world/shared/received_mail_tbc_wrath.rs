use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::auction_house_vanilla_tbc_wrath::AuctionHouse;
use wow_world_base::shared::mail_message_type_vanilla_tbc_wrath::MailMessageType;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_query_next_mail_time_server.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_query_next_mail_time_server.wowm#L29):
/// ```text
/// struct ReceivedMail {
///     Guid sender;
///     AuctionHouse auction_house;
///     MailMessageType message_type;
///     u32 stationery;
///     f32 time;
/// }
/// ```
pub struct ReceivedMail {
    pub sender: Guid,
    pub auction_house: AuctionHouse,
    pub message_type: MailMessageType,
    pub stationery: u32,
    /// mangosone sets to `0xC6000000`
    /// mangosone: float unk, time or something
    pub time: f32,
}

impl ReceivedMail {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // sender: Guid
        w.write_all(&self.sender.guid().to_le_bytes())?;

        // auction_house: AuctionHouse
        w.write_all(&(self.auction_house.as_int().to_le_bytes()))?;

        // message_type: MailMessageType
        w.write_all(&(self.message_type.as_int().to_le_bytes()))?;

        // stationery: u32
        w.write_all(&self.stationery.to_le_bytes())?;

        // time: f32
        w.write_all(&self.time.to_le_bytes())?;

        Ok(())
    }
}

impl ReceivedMail {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // sender: Guid
        let sender = crate::util::read_guid(&mut r)?;

        // auction_house: AuctionHouse
        let auction_house = crate::util::read_u32_le(&mut r)?.try_into()?;

        // message_type: MailMessageType
        let message_type = crate::util::read_u32_le(&mut r)?.try_into()?;

        // stationery: u32
        let stationery = crate::util::read_u32_le(&mut r)?;

        // time: f32
        let time = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            sender,
            auction_house,
            message_type,
            stationery,
            time,
        })
    }

}

