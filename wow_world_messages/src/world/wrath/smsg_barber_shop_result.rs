use crate::wrath::BarberShopResult;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_barber_shop_result.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_barber_shop_result.wowm#L10):
/// ```text
/// smsg SMSG_BARBER_SHOP_RESULT = 0x0428 {
///     (u32)BarberShopResult result;
/// }
/// ```
pub struct SMSG_BARBER_SHOP_RESULT {
    pub result: BarberShopResult,
}

impl crate::Message for SMSG_BARBER_SHOP_RESULT {
    const OPCODE: u32 = 0x0428;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // result: BarberShopResult
        w.write_all(&u32::from(self.result.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0428, size: body_size as u32 });
        }

        // result: BarberShopResult
        let result: BarberShopResult = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            result,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BARBER_SHOP_RESULT {}

