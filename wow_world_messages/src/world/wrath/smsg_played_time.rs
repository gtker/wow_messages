use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_played_time.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_played_time.wowm#L9):
/// ```text
/// smsg SMSG_PLAYED_TIME = 0x01CD {
///     u32 total_played_time;
///     u32 level_played_time;
///     Bool show_on_ui;
/// }
/// ```
pub struct SMSG_PLAYED_TIME {
    /// Time played in total (seconds)
    ///
    pub total_played_time: u32,
    /// Time played on this level (seconds)
    ///
    pub level_played_time: u32,
    /// Whether this is a silent query or the client should show it on the UI (chat box).
    /// Send back the value received in [`CMSG_PLAYED_TIME`](crate::wrath::CMSG_PLAYED_TIME)
    ///
    pub show_on_ui: bool,
}

impl crate::Message for SMSG_PLAYED_TIME {
    const OPCODE: u32 = 0x01cd;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // total_played_time: u32
        w.write_all(&self.total_played_time.to_le_bytes())?;

        // level_played_time: u32
        w.write_all(&self.level_played_time.to_le_bytes())?;

        // show_on_ui: Bool
        w.write_all(u8::from(self.show_on_ui).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01CD, size: body_size as u32 });
        }

        // total_played_time: u32
        let total_played_time = crate::util::read_u32_le(r)?;

        // level_played_time: u32
        let level_played_time = crate::util::read_u32_le(r)?;

        // show_on_ui: Bool
        let show_on_ui = crate::util::read_u8_le(r)? != 0;

        Ok(Self {
            total_played_time,
            level_played_time,
            show_on_ui,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PLAYED_TIME {}

