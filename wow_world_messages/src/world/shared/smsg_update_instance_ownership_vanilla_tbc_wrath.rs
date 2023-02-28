use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_update_instance_ownership.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_update_instance_ownership.wowm#L3):
/// ```text
/// smsg SMSG_UPDATE_INSTANCE_OWNERSHIP = 0x032B {
///     Bool32 player_is_saved_to_a_raid;
/// }
/// ```
pub struct SMSG_UPDATE_INSTANCE_OWNERSHIP {
    pub player_is_saved_to_a_raid: bool,
}

impl crate::Message for SMSG_UPDATE_INSTANCE_OWNERSHIP {
    const OPCODE: u32 = 0x032b;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player_is_saved_to_a_raid: Bool32
        w.write_all(u32::from(self.player_is_saved_to_a_raid).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x032B, size: body_size as u32 });
        }

        // player_is_saved_to_a_raid: Bool32
        let player_is_saved_to_a_raid = crate::util::read_u32_le(r)? != 0;

        Ok(Self {
            player_is_saved_to_a_raid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_UPDATE_INSTANCE_OWNERSHIP {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_UPDATE_INSTANCE_OWNERSHIP {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_UPDATE_INSTANCE_OWNERSHIP {}

