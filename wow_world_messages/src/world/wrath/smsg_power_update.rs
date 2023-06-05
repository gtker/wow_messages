use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::Power;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

impl crate::private::Sealed for SMSG_POWER_UPDATE {}
impl crate::Message for SMSG_POWER_UPDATE {
    const OPCODE: u32 = 0x0480;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        crate::util::write_packed_guid(&self.unit, &mut w)?;

        // power: Power
        w.write_all(&(self.power.as_int().to_le_bytes()))?;

        // amount: u32
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(7..=14).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0480, size: body_size });
        }

        // unit: PackedGuid
        let unit = crate::util::read_packed_guid(&mut r)?;

        // power: Power
        let power: Power = crate::util::read_u8_le(&mut r)?.try_into()?;

        // amount: u32
        let amount = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            unit,
            power,
            amount,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_POWER_UPDATE {}

impl SMSG_POWER_UPDATE {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.unit) // unit: PackedGuid
        + 1 // power: Power
        + 4 // amount: u32
    }
}

