use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::LootMethod;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_response.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_response.wowm#L22):
/// ```text
/// smsg SMSG_LOOT_RESPONSE = 0x0160 {
///     Guid guid;
///     LootMethod loot_method;
/// }
/// ```
pub struct SMSG_LOOT_RESPONSE {
    pub guid: Guid,
    pub loot_method: LootMethod,
}

impl crate::Message for SMSG_LOOT_RESPONSE {
    const OPCODE: u32 = 0x0160;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // loot_method: LootMethod
        w.write_all(&(self.loot_method.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // loot_method: LootMethod
        let loot_method: LootMethod = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            guid,
            loot_method,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_LOOT_RESPONSE {}

