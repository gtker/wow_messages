use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/resurrect/smsg_resurrect_request.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/resurrect/smsg_resurrect_request.wowm#L1):
/// ```text
/// smsg SMSG_RESURRECT_REQUEST = 0x015B {
///     Guid guid;
///     SizedCString name;
///     Bool player;
/// }
/// ```
pub struct SMSG_RESURRECT_REQUEST {
    pub guid: Guid,
    pub name: String,
    pub player: bool,
}

impl crate::Message for SMSG_RESURRECT_REQUEST {
    const OPCODE: u32 = 0x015b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // name: SizedCString
        w.write_all(&((self.name.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // player: Bool
        w.write_all(u8::from(self.player).to_le_bytes().as_slice())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(14..=8013).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x015B, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // name: SizedCString
        let name = crate::util::read_u32_le(r)?;
        let name = crate::util::read_sized_c_string_to_vec(r, name)?;
        let name = String::from_utf8(name)?;;
        // player: Bool
        let player = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            guid,
            name,
            player,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_RESURRECT_REQUEST {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_RESURRECT_REQUEST {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_RESURRECT_REQUEST {}

impl SMSG_RESURRECT_REQUEST {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + self.name.len() + 5 // name: SizedCString
        + 1 // player: Bool
    }
}

