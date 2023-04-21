use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_save_guild_emblem_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_save_guild_emblem_client.wowm#L3):
/// ```text
/// cmsg MSG_SAVE_GUILD_EMBLEM_Client = 0x01F1 {
///     Guid vendor;
///     u32 emblem_style;
///     u32 emblem_color;
///     u32 border_style;
///     u32 border_color;
///     u32 background_color;
/// }
/// ```
pub struct MSG_SAVE_GUILD_EMBLEM_Client {
    pub vendor: Guid,
    pub emblem_style: u32,
    pub emblem_color: u32,
    pub border_style: u32,
    pub border_color: u32,
    pub background_color: u32,
}

impl crate::private::Sealed for MSG_SAVE_GUILD_EMBLEM_Client {}
impl crate::Message for MSG_SAVE_GUILD_EMBLEM_Client {
    const OPCODE: u32 = 0x01f1;

    fn size_without_header(&self) -> u32 {
        28
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // vendor: Guid
        w.write_all(&self.vendor.guid().to_le_bytes())?;

        // emblem_style: u32
        w.write_all(&self.emblem_style.to_le_bytes())?;

        // emblem_color: u32
        w.write_all(&self.emblem_color.to_le_bytes())?;

        // border_style: u32
        w.write_all(&self.border_style.to_le_bytes())?;

        // border_color: u32
        w.write_all(&self.border_color.to_le_bytes())?;

        // background_color: u32
        w.write_all(&self.background_color.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 28 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01F1, size: body_size as u32 });
        }

        // vendor: Guid
        let vendor = Guid::read(&mut r)?;

        // emblem_style: u32
        let emblem_style = crate::util::read_u32_le(&mut r)?;

        // emblem_color: u32
        let emblem_color = crate::util::read_u32_le(&mut r)?;

        // border_style: u32
        let border_style = crate::util::read_u32_le(&mut r)?;

        // border_color: u32
        let border_color = crate::util::read_u32_le(&mut r)?;

        // background_color: u32
        let background_color = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            vendor,
            emblem_style,
            emblem_color,
            border_style,
            border_color,
            background_color,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_SAVE_GUILD_EMBLEM_Client {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_SAVE_GUILD_EMBLEM_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_SAVE_GUILD_EMBLEM_Client {}

