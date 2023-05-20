use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/duel/smsg_duel_complete.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/duel/smsg_duel_complete.wowm#L3):
/// ```text
/// smsg SMSG_DUEL_COMPLETE = 0x016A {
///     Bool ended_without_interruption;
/// }
/// ```
pub struct SMSG_DUEL_COMPLETE {
    pub ended_without_interruption: bool,
}

impl crate::private::Sealed for SMSG_DUEL_COMPLETE {}
impl crate::Message for SMSG_DUEL_COMPLETE {
    const OPCODE: u32 = 0x016a;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // ended_without_interruption: Bool
        w.write_all(u8::from(self.ended_without_interruption).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x016A, size: body_size });
        }

        // ended_without_interruption: Bool
        let ended_without_interruption = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            ended_without_interruption,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_DUEL_COMPLETE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_DUEL_COMPLETE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_DUEL_COMPLETE {}

