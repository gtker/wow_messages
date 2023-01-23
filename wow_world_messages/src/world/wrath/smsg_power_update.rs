use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::Power;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_power_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_power_update.wowm#L1):
/// ```text
/// smsg SMSG_POWER_UPDATE = 0x0480 {
///     PackedGuid unit;
///     Power power;
///     u32 amount;
/// }
/// ```
pub struct SMSG_POWER_UPDATE {
    pub unit: Guid,
    pub power: Power,
    pub amount: u32,
}

impl crate::Message for SMSG_POWER_UPDATE {
    const OPCODE: u32 = 0x0480;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // unit: PackedGuid
        self.unit.write_packed_guid_into_vec(w);

        // power: Power
        w.write_all(&(self.power.as_int() as u8).to_le_bytes())?;

        // amount: u32
        w.write_all(&self.amount.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(7..=14).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0480, size: body_size as u32 });
        }

        // unit: PackedGuid
        let unit = Guid::read_packed(r)?;

        // power: Power
        let power: Power = crate::util::read_u8_le(r)?.try_into()?;

        // amount: u32
        let amount = crate::util::read_u32_le(r)?;

        Ok(Self {
            unit,
            power,
            amount,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_POWER_UPDATE {}

impl SMSG_POWER_UPDATE {
    pub(crate) fn size(&self) -> usize {
        self.unit.size() // unit: Guid
        + 1 // power: Power
        + 4 // amount: u32
    }
}

