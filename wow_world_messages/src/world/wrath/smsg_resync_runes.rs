use std::io::{Read, Write};

use crate::wrath::ResyncRune;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_resync_runes.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_resync_runes.wowm#L8):
/// ```text
/// smsg SMSG_RESYNC_RUNES = 0x0487 {
///     u32 amount_of_runes;
///     ResyncRune[amount_of_runes] runes;
/// }
/// ```
pub struct SMSG_RESYNC_RUNES {
    pub runes: Vec<ResyncRune>,
}

impl crate::private::Sealed for SMSG_RESYNC_RUNES {}
impl crate::Message for SMSG_RESYNC_RUNES {
    const OPCODE: u32 = 0x0487;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_runes: u32
        w.write_all(&(self.runes.len() as u32).to_le_bytes())?;

        // runes: ResyncRune[amount_of_runes]
        for i in self.runes.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(4..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0487, size: body_size });
        }

        // amount_of_runes: u32
        let amount_of_runes = crate::util::read_u32_le(&mut r)?;

        // runes: ResyncRune[amount_of_runes]
        let runes = {
            let mut runes = Vec::with_capacity(amount_of_runes as usize);
            for _ in 0..amount_of_runes {
                runes.push(ResyncRune::read(&mut r)?);
            }
            runes
        };

        Ok(Self {
            runes,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_RESYNC_RUNES {}

impl SMSG_RESYNC_RUNES {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_runes: u32
        + self.runes.len() * 2 // runes: ResyncRune[amount_of_runes]
    }
}

