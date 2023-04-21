use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_set_flat_spell_modifier.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_set_flat_spell_modifier.wowm#L1):
/// ```text
/// smsg SMSG_SET_FLAT_SPELL_MODIFIER = 0x0266 {
///     u8 eff;
///     u8 op;
///     u32 value;
/// }
/// ```
pub struct SMSG_SET_FLAT_SPELL_MODIFIER {
    pub eff: u8,
    pub op: u8,
    pub value: u32,
}

impl crate::private::Sealed for SMSG_SET_FLAT_SPELL_MODIFIER {}
impl crate::Message for SMSG_SET_FLAT_SPELL_MODIFIER {
    const OPCODE: u32 = 0x0266;

    fn size_without_header(&self) -> u32 {
        6
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // eff: u8
        w.write_all(&self.eff.to_le_bytes())?;

        // op: u8
        w.write_all(&self.op.to_le_bytes())?;

        // value: u32
        w.write_all(&self.value.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 6 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0266, size: body_size as u32 });
        }

        // eff: u8
        let eff = crate::util::read_u8_le(&mut r)?;

        // op: u8
        let op = crate::util::read_u8_le(&mut r)?;

        // value: u32
        let value = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            eff,
            op,
            value,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SET_FLAT_SPELL_MODIFIER {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SET_FLAT_SPELL_MODIFIER {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SET_FLAT_SPELL_MODIFIER {}

