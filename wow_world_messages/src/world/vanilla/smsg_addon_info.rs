use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::Addon;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm:60`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm#L60):
/// ```text
/// smsg SMSG_ADDON_INFO = 0x02EF {
///     Addon[-] addons;
/// }
/// ```
pub struct SMSG_ADDON_INFO {
    pub addons: Vec<Addon>,
}

impl crate::Message for SMSG_ADDON_INFO {
    const OPCODE: u32 = 0x02ef;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // addons: Addon[-]
        for i in self.addons.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // addons: Addon[-]
        let mut current_size = {
            0
        };
        let mut addons = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            addons.push(Addon::read(r)?);
            current_size += 1;
        }

        Ok(Self {
            addons,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_ADDON_INFO {}

impl SMSG_ADDON_INFO {
    pub(crate) fn size(&self) -> usize {
        self.addons.iter().fold(0, |acc, x| acc + x.size()) // addons: Addon[-]
    }
}

