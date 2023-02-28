use wow_world_base::shared::activate_taxi_reply_vanilla_tbc_wrath::ActivateTaxiReply;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_activatetaxireply.wowm:19`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_activatetaxireply.wowm#L19):
/// ```text
/// smsg SMSG_ACTIVATETAXIREPLY = 0x01AE {
///     ActivateTaxiReply reply;
/// }
/// ```
pub struct SMSG_ACTIVATETAXIREPLY {
    pub reply: ActivateTaxiReply,
}

impl crate::Message for SMSG_ACTIVATETAXIREPLY {
    const OPCODE: u32 = 0x01ae;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // reply: ActivateTaxiReply
        w.write_all(&u32::from(self.reply.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01AE, size: body_size as u32 });
        }

        // reply: ActivateTaxiReply
        let reply: ActivateTaxiReply = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            reply,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ACTIVATETAXIREPLY {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ACTIVATETAXIREPLY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ACTIVATETAXIREPLY {}

