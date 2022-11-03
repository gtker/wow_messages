use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
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
    ///
    pub unknown1: u32,
}

impl crate::Message for SMSG_ATTACKSTOP {
    const OPCODE: u32 = 0x0144;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(w);

        // enemy: PackedGuid
        self.enemy.write_packed_guid_into_vec(w);

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size < 8 || body_size > 22 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0144, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(r)?;

        // enemy: PackedGuid
        let enemy = Guid::read_packed(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        Ok(Self {
            player,
            enemy,
            unknown1,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_ATTACKSTOP {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_ATTACKSTOP {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_ATTACKSTOP {}

impl SMSG_ATTACKSTOP {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: Guid
        + self.enemy.size() // enemy: Guid
        + 4 // unknown1: u32
    }
}

