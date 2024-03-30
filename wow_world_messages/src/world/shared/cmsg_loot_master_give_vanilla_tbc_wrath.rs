use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/cmsg_loot_master_give.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/cmsg_loot_master_give.wowm#L3):
/// ```text
/// cmsg CMSG_LOOT_MASTER_GIVE = 0x02A3 {
///     Guid loot;
///     u8 slot_id;
///     Guid player;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_LOOT_MASTER_GIVE {
    pub loot: Guid,
    pub slot_id: u8,
    pub player: Guid,
}

impl crate::private::Sealed for CMSG_LOOT_MASTER_GIVE {}
impl CMSG_LOOT_MASTER_GIVE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 17 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // loot: Guid
        let loot = crate::util::read_guid(&mut r)?;

        // slot_id: u8
        let slot_id = crate::util::read_u8_le(&mut r)?;

        // player: Guid
        let player = crate::util::read_guid(&mut r)?;

        Ok(Self {
            loot,
            slot_id,
            player,
        })
    }

}

impl crate::Message for CMSG_LOOT_MASTER_GIVE {
    const OPCODE: u32 = 0x02a3;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_LOOT_MASTER_GIVE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_LOOT_MASTER_GIVE {{").unwrap();
        // Members
        writeln!(s, "    loot = {};", self.loot.guid()).unwrap();
        writeln!(s, "    slot_id = {};", self.slot_id).unwrap();
        writeln!(s, "    player = {};", self.player.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 21_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 675_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "loot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "slot_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "player", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        17
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // loot: Guid
        w.write_all(&self.loot.guid().to_le_bytes())?;

        // slot_id: u8
        w.write_all(&self.slot_id.to_le_bytes())?;

        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(675, "CMSG_LOOT_MASTER_GIVE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_LOOT_MASTER_GIVE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_LOOT_MASTER_GIVE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LOOT_MASTER_GIVE {}

