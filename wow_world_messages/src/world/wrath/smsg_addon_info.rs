use std::convert::{TryFrom, TryInto};
use crate::world::wrath::Addon;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Banned addons are not properly implemented in the wowm. Sending any number other than 0 means that the packet is incomplete and thus invalid
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm:86`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm#L86):
/// ```text
/// smsg SMSG_ADDON_INFO = 0x02EF {
///     u32 number_of_addons;
///     Addon[number_of_addons] addons;
///     u32 number_of_banned_addons = 0;
/// }
/// ```
pub struct SMSG_ADDON_INFO {
    pub addons: Vec<Addon>,
}

impl SMSG_ADDON_INFO {
    /// The field `number_of_banned_addons` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `0` |
    /// | Hex | `0x00` |
    /// | Original | `0` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const NUMBER_OF_BANNED_ADDONS_VALUE: u32 = 0x00;

}

impl crate::Message for SMSG_ADDON_INFO {
    const OPCODE: u32 = 0x02ef;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // number_of_addons: u32
        w.write_all(&(self.addons.len() as u32).to_le_bytes())?;

        // addons: Addon[number_of_addons]
        for i in self.addons.iter() {
            i.write_into_vec(w)?;
        }

        // number_of_banned_addons: u32
        w.write_all(&Self::NUMBER_OF_BANNED_ADDONS_VALUE.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // number_of_addons: u32
        let number_of_addons = crate::util::read_u32_le(r)?;

        // addons: Addon[number_of_addons]
        let mut addons = Vec::with_capacity(number_of_addons as usize);
        for i in 0..number_of_addons {
            let o = Addon::read(r)?;
            addons.push(o);
        }

        // number_of_banned_addons: u32
        let _number_of_banned_addons = crate::util::read_u32_le(r)?;
        // number_of_banned_addons is expected to always be 0 (0)

        Ok(Self {
            addons,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_ADDON_INFO {}

impl SMSG_ADDON_INFO {
    pub(crate) fn size(&self) -> usize {
        4 // number_of_addons: u32
        + self.addons.len() * 8 // addons: Addon[number_of_addons]
        + 4 // number_of_banned_addons: u32
    }
}

