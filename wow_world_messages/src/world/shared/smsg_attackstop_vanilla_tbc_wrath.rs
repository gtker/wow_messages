use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackstop.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackstop.wowm#L11):
/// ```text
/// smsg SMSG_ATTACKSTOP = 0x0144 {
///     PackedGuid player;
///     PackedGuid enemy;
///     u32 unknown1;
/// }
/// ```
pub struct SMSG_ATTACKSTOP {
    pub player: Guid,
    pub enemy: Guid,
    /// cmangos/vmangos/mangoszero/arcemu/azerothcore/mangostwo: set to 0 with comment: unk, can be 1 also
    pub unknown1: u32,
}

impl crate::private::Sealed for SMSG_ATTACKSTOP {}
impl SMSG_ATTACKSTOP {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(8..=22).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // player: PackedGuid
        let player = crate::util::read_packed_guid(&mut r)?;

        // enemy: PackedGuid
        let enemy = crate::util::read_packed_guid(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            player,
            enemy,
            unknown1,
        })
    }

}

impl crate::Message for SMSG_ATTACKSTOP {
    const OPCODE: u32 = 0x0144;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ATTACKSTOP {{").unwrap();
        // Members
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        writeln!(s, "    enemy = {};", self.enemy.guid()).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 324_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.player), "player", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.enemy), "enemy", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.8 1.9 1.10 1.11 1.12 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        crate::util::write_packed_guid(&self.player, &mut w)?;

        // enemy: PackedGuid
        crate::util::write_packed_guid(&self.enemy, &mut w)?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(324, "SMSG_ATTACKSTOP", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ATTACKSTOP {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ATTACKSTOP {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ATTACKSTOP {}

impl SMSG_ATTACKSTOP {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.player) // player: PackedGuid
        + crate::util::packed_guid_size(&self.enemy) // enemy: PackedGuid
        + 4 // unknown1: u32
    }
}

