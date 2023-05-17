use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_client_control_update.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_client_control_update.wowm#L3):
/// ```text
/// smsg SMSG_CLIENT_CONTROL_UPDATE = 0x0159 {
///     PackedGuid guid;
///     Bool allow_movement;
/// }
/// ```
pub struct SMSG_CLIENT_CONTROL_UPDATE {
    pub guid: Guid,
    pub allow_movement: bool,
}

impl crate::private::Sealed for SMSG_CLIENT_CONTROL_UPDATE {}
impl crate::Message for SMSG_CLIENT_CONTROL_UPDATE {
    const OPCODE: u32 = 0x0159;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        // allow_movement: Bool
        w.write_all(u8::from(self.allow_movement).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(3..=10).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0159, size: body_size });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        // allow_movement: Bool
        let allow_movement = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            guid,
            allow_movement,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_CLIENT_CONTROL_UPDATE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_CLIENT_CONTROL_UPDATE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CLIENT_CONTROL_UPDATE {}

impl SMSG_CLIENT_CONTROL_UPDATE {
    pub(crate) const fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
        + 1 // allow_movement: Bool
    }
}

