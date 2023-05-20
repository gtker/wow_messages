use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_superceded_spell.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_superceded_spell.wowm#L8):
/// ```text
/// smsg SMSG_SUPERCEDED_SPELL = 0x012C {
///     u32 new;
///     u32 old;
/// }
/// ```
pub struct SMSG_SUPERCEDED_SPELL {
    pub new: u32,
    pub old: u32,
}

impl crate::private::Sealed for SMSG_SUPERCEDED_SPELL {}
impl crate::Message for SMSG_SUPERCEDED_SPELL {
    const OPCODE: u32 = 0x012c;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // new: u32
        w.write_all(&self.new.to_le_bytes())?;

        // old: u32
        w.write_all(&self.old.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x012C, size: body_size });
        }

        // new: u32
        let new = crate::util::read_u32_le(&mut r)?;

        // old: u32
        let old = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            new,
            old,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SUPERCEDED_SPELL {}

