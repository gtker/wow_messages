use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_played_time.wowm:6`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_played_time.wowm#L6):
/// ```text
/// cmsg CMSG_PLAYED_TIME = 0x01CC {
///     Bool show_on_ui;
/// }
/// ```
pub struct CMSG_PLAYED_TIME {
    /// Whether the clients wants it shown on the UI. Just ping it back in [`SMSG_PLAYED_TIME`](crate::wrath::SMSG_PLAYED_TIME)
    ///
    pub show_on_ui: bool,
}

impl crate::Message for CMSG_PLAYED_TIME {
    const OPCODE: u32 = 0x01cc;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // show_on_ui: Bool
        w.write_all(u8::from(self.show_on_ui).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01CC, size: body_size as u32 });
        }

        // show_on_ui: Bool
        let show_on_ui = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            show_on_ui,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PLAYED_TIME {}

