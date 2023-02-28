use crate::wrath::BattlefieldListLocation;
use crate::wrath::BattlegroundType;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_battlefield_list.wowm:18`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_battlefield_list.wowm#L18):
/// ```text
/// cmsg CMSG_BATTLEFIELD_LIST = 0x023C {
///     BattlegroundType battleground_type;
///     BattlefieldListLocation location;
///     Bool can_gain_exp;
/// }
/// ```
pub struct CMSG_BATTLEFIELD_LIST {
    pub battleground_type: BattlegroundType,
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

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // battleground_type: BattlegroundType
        w.write_all(&u32::from(self.battleground_type.as_int()).to_le_bytes())?;

        // location: BattlefieldListLocation
        w.write_all(&u8::from(self.location.as_int()).to_le_bytes())?;

        // can_gain_exp: Bool
        w.write_all(u8::from(self.can_gain_exp).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 6 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x023C, size: body_size as u32 });
        }

        // battleground_type: BattlegroundType
        let battleground_type: BattlegroundType = crate::util::read_u32_le(&mut r)?.try_into()?;

        // location: BattlefieldListLocation
        let location: BattlefieldListLocation = crate::util::read_u8_le(&mut r)?.try_into()?;

        // can_gain_exp: Bool
        let can_gain_exp = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            battleground_type,
            location,
            can_gain_exp,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_BATTLEFIELD_LIST {}

