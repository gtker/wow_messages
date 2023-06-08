use std::io::{Read, Write};

use crate::wrath::{
    BattlefieldListLocation, BattlegroundType,
};

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
    pub can_gain_exp: bool,
}

impl crate::private::Sealed for CMSG_BATTLEFIELD_LIST {}
impl crate::Message for CMSG_BATTLEFIELD_LIST {
    const OPCODE: u32 = 0x023c;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_BATTLEFIELD_LIST {{").unwrap();
        // Members
        writeln!(s, "    battleground_type = {};", self.battleground_type.as_test_case_value()).unwrap();
        writeln!(s, "    location = {};", self.location.as_test_case_value()).unwrap();
        writeln!(s, "    can_gain_exp = {};", if self.can_gain_exp { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 10_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 572_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "battleground_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "location", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "can_gain_exp", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        6
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // battleground_type: BattlegroundType
        w.write_all(&(self.battleground_type.as_int().to_le_bytes()))?;

        // location: BattlefieldListLocation
        w.write_all(&(self.location.as_int().to_le_bytes()))?;

        // can_gain_exp: Bool
        w.write_all(u8::from(self.can_gain_exp).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 6 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x023C, size: body_size });
        }

        // battleground_type: BattlegroundType
        let battleground_type = crate::util::read_u32_le(&mut r)?.try_into()?;

        // location: BattlefieldListLocation
        let location = crate::util::read_u8_le(&mut r)?.try_into()?;

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

