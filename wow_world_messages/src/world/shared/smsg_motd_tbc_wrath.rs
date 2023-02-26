use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_motd.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_motd.wowm#L1):
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

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // motd: SizedCString
        w.write_all(&((self.motd.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.motd.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(5..=8004).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x033D, size: body_size as u32 });
        }

        // motd: SizedCString
        let motd = {
            let motd = crate::util::read_u32_le(r)?;
            let motd = crate::util::read_sized_c_string_to_vec(r, motd)?;
            String::from_utf8(motd)?
        };

        Ok(Self {
            motd,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_MOTD {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MOTD {}

impl SMSG_MOTD {
    pub(crate) fn size(&self) -> usize {
        self.motd.len() + 5 // motd: SizedCString
    }
}

