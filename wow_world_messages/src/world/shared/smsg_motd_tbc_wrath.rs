use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_motd.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_motd.wowm#L1):
/// ```text
/// smsg SMSG_MOTD = 0x033D {
///     SizedCString motd;
/// }
/// ```
pub struct SMSG_MOTD {
    pub motd: String,
}

impl crate::Message for SMSG_MOTD {
    const OPCODE: u32 = 0x033d;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // motd: SizedCString
        w.write_all(&((self.motd.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.motd.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(5..=8004).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x033D, size: body_size as u32 });
        }

        // motd: SizedCString
        let motd = crate::util::read_u32_le(r)?;
        let motd = crate::util::read_sized_c_string_to_vec(r, motd)?;
        let motd = String::from_utf8(motd)?;;
        Ok(Self {
            motd,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_MOTD {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_MOTD {}

impl SMSG_MOTD {
    pub(crate) fn size(&self) -> usize {
        self.motd.len() + 5 // motd: SizedCString
    }
}

