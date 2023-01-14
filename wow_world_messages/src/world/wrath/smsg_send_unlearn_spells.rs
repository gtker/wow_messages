use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_send_unlearn_spells.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_send_unlearn_spells.wowm#L8):
/// ```text
/// smsg SMSG_SEND_UNLEARN_SPELLS = 0x041E {
///     u32 amount_of_spells;
///     u32[amount_of_spells] spells;
/// }
/// ```
pub struct SMSG_SEND_UNLEARN_SPELLS {
    pub spells: Vec<u32>,
}

impl crate::Message for SMSG_SEND_UNLEARN_SPELLS {
    const OPCODE: u32 = 0x041e;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // amount_of_spells: u32
        w.write_all(&(self.spells.len() as u32).to_le_bytes())?;

        // spells: u32[amount_of_spells]
        for i in self.spells.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x041E, size: body_size as u32 });
        }

        // amount_of_spells: u32
        let amount_of_spells = crate::util::read_u32_le(r)?;

        // spells: u32[amount_of_spells]
        let mut spells = Vec::with_capacity(amount_of_spells as usize);
        for i in 0..amount_of_spells {
            spells.push(crate::util::read_u32_le(r)?);
        }

        Ok(Self {
            spells,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_SEND_UNLEARN_SPELLS {}

impl SMSG_SEND_UNLEARN_SPELLS {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_spells: u32
        + self.spells.len() * core::mem::size_of::<u32>() // spells: u32[amount_of_spells]
    }
}

