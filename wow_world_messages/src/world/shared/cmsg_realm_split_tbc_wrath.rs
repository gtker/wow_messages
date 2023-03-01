use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_realm_split.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_realm_split.wowm#L3):
/// ```text
/// cmsg CMSG_REALM_SPLIT = 0x038C {
///     u32 realm_id;
/// }
/// ```
pub struct CMSG_REALM_SPLIT {
    /// Realm ID that was sent earlier by the Auth Server
    /// ArcEmu/TriniyCore/mangosthree send back in [`SMSG_REALM_SPLIT`](crate::tbc::SMSG_REALM_SPLIT).
    ///
    pub realm_id: u32,
}

impl crate::Message for CMSG_REALM_SPLIT {
    const OPCODE: u32 = 0x038c;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // realm_id: u32
        w.write_all(&self.realm_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x038C, size: body_size as u32 });
        }

        // realm_id: u32
        let realm_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            realm_id,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_REALM_SPLIT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_REALM_SPLIT {}

