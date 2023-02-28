use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_equipment_set_use_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_equipment_set_use_result.wowm#L1):
/// ```text
/// smsg SMSG_EQUIPMENT_SET_USE_RESULT = 0x04D6 {
///     u8 result;
/// }
/// ```
pub struct SMSG_EQUIPMENT_SET_USE_RESULT {
    pub result: u8,
}

impl crate::Message for SMSG_EQUIPMENT_SET_USE_RESULT {
    const OPCODE: u32 = 0x04d6;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // result: u8
        w.write_all(&self.result.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04D6, size: body_size as u32 });
        }

        // result: u8
        let result = crate::util::read_u8_le(r)?;

        Ok(Self {
            result,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_EQUIPMENT_SET_USE_RESULT {}

