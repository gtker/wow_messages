use std::io::{Read, Write};

use wow_world_base::shared::battlefield_port_action_vanilla_tbc_wrath::BattlefieldPortAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_battlefield_port.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_battlefield_port.wowm#L21):
/// ```text
/// cmsg CMSG_BATTLEFIELD_PORT = 0x02D5 {
///     u8 arena_type;
///     u8 unknown1;
///     u32 bg_type_id;
///     u16 unknown2;
///     BattlefieldPortAction action;
/// }
/// ```
pub struct CMSG_BATTLEFIELD_PORT {
    /// mangosone/mangos-tbc/azerothcore: arenatype if arena
    ///
    pub arena_type: u8,
    /// mangosone/mangos-tbc/azerothcore: unk, can be 0x0 (may be if was invited?) and 0x1
    ///
    pub unknown1: u8,
    /// mangosone/mangos-tbc/azerothcore: type id from dbc
    ///
    pub bg_type_id: u32,
    /// mangosone/mangos-tbc/azerothcore: 0x1F90 constant?
    ///
    pub unknown2: u16,
    pub action: BattlefieldPortAction,
}

#[cfg(feature = "print-testcase")]
impl CMSG_BATTLEFIELD_PORT {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_BATTLEFIELD_PORT {{").unwrap();
        // Members
        writeln!(s, "    arena_type = {};", self.arena_type).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    bg_type_id = {};", self.bg_type_id).unwrap();
        writeln!(s, "    unknown2 = {};", self.unknown2).unwrap();
        writeln!(s, "    action = {};", self.action.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 15_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 725_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "arena_type");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_BATTLEFIELD_PORT {}
impl crate::Message for CMSG_BATTLEFIELD_PORT {
    const OPCODE: u32 = 0x02d5;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // arena_type: u8
        w.write_all(&self.arena_type.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // bg_type_id: u32
        w.write_all(&self.bg_type_id.to_le_bytes())?;

        // unknown2: u16
        w.write_all(&self.unknown2.to_le_bytes())?;

        // action: BattlefieldPortAction
        w.write_all(&(self.action.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02D5, size: body_size });
        }

        // arena_type: u8
        let arena_type = crate::util::read_u8_le(&mut r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(&mut r)?;

        // bg_type_id: u32
        let bg_type_id = crate::util::read_u32_le(&mut r)?;

        // unknown2: u16
        let unknown2 = crate::util::read_u16_le(&mut r)?;

        // action: BattlefieldPortAction
        let action = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            arena_type,
            unknown1,
            bg_type_id,
            unknown2,
            action,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_BATTLEFIELD_PORT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_BATTLEFIELD_PORT {}

