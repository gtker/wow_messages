use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_update_instance_ownership.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_update_instance_ownership.wowm#L3):
/// ```text
/// smsg SMSG_UPDATE_INSTANCE_OWNERSHIP = 0x032B {
///     u32 has_been_saved;
/// }
/// ```
pub struct SMSG_UPDATE_INSTANCE_OWNERSHIP {
    /// cmangos/vmangos/mangoszero: true or false means, whether you have current raid instances
    ///
    pub has_been_saved: u32,
}

impl crate::Message for SMSG_UPDATE_INSTANCE_OWNERSHIP {
    const OPCODE: u32 = 0x032b;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // has_been_saved: u32
        w.write_all(&self.has_been_saved.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // has_been_saved: u32
        let has_been_saved = crate::util::read_u32_le(r)?;

        Ok(Self {
            has_been_saved,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_UPDATE_INSTANCE_OWNERSHIP {}

