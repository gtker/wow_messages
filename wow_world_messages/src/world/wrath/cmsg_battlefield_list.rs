use std::convert::{TryFrom, TryInto};
use crate::world::wrath::BattlefieldListLocation;
use crate::world::wrath::Map;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_battlefield_list.wowm:18`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_battlefield_list.wowm#L18):
/// ```text
/// cmsg CMSG_BATTLEFIELD_LIST = 0x023C {
///     Map map;
///     BattlefieldListLocation location;
///     Bool can_gain_exp;
/// }
/// ```
pub struct CMSG_BATTLEFIELD_LIST {
    pub map: Map,
    pub location: BattlefieldListLocation,
    /// azerothcore: players with locked xp have their own bg queue on retail
    ///
    pub can_gain_exp: bool,
}

impl crate::Message for CMSG_BATTLEFIELD_LIST {
    const OPCODE: u32 = 0x023c;

    fn size_without_header(&self) -> u32 {
        6
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // location: BattlefieldListLocation
        w.write_all(&(self.location.as_int() as u8).to_le_bytes())?;

        // can_gain_exp: Bool
        w.write_all(if self.can_gain_exp { &[1] } else { &[0] })?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 6 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // location: BattlefieldListLocation
        let location: BattlefieldListLocation = crate::util::read_u8_le(r)?.try_into()?;

        // can_gain_exp: Bool
        let can_gain_exp = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            map,
            location,
            can_gain_exp,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_BATTLEFIELD_LIST {}

