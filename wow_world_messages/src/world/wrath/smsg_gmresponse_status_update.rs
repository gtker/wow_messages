use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gmresponse_status_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gmresponse_status_update.wowm#L1):
/// ```text
/// smsg SMSG_GMRESPONSE_STATUS_UPDATE = 0x04F1 {
///     Bool show_survey;
/// }
/// ```
pub struct SMSG_GMRESPONSE_STATUS_UPDATE {
    pub show_survey: bool,
}

impl crate::private::Sealed for SMSG_GMRESPONSE_STATUS_UPDATE {}
impl crate::Message for SMSG_GMRESPONSE_STATUS_UPDATE {
    const OPCODE: u32 = 0x04f1;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // show_survey: Bool
        w.write_all(u8::from(self.show_survey).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04F1, size: body_size });
        }

        // show_survey: Bool
        let show_survey = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            show_survey,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GMRESPONSE_STATUS_UPDATE {}

