use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_player_skinned.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_player_skinned.wowm#L3):
/// ```text
/// smsg SMSG_PLAYER_SKINNED = 0x02BC {
///     Bool spirit_released;
/// }
/// ```
pub struct SMSG_PLAYER_SKINNED {
    pub spirit_released: bool,
}

impl crate::Message for SMSG_PLAYER_SKINNED {
    const OPCODE: u32 = 0x02bc;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // spirit_released: Bool
        w.write_all(u8::from(self.spirit_released).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02BC, size: body_size as u32 });
        }

        // spirit_released: Bool
        let spirit_released = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            spirit_released,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PLAYER_SKINNED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PLAYER_SKINNED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PLAYER_SKINNED {}

