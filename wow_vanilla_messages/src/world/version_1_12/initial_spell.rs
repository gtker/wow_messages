use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_initial_spells.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_initial_spells.wowm#L13):
/// ```text
/// struct InitialSpell {
///     u16 spell_id;
///     u16 unknown1;
/// }
/// ```
pub struct InitialSpell {
    /// # Comment
    ///
    /// cmangos/mangoszero: only send 'first' part of spell
    ///
    pub spell_id: u16,
    /// # Comment
    ///
    /// cmangos/mangoszero: sets to 0
    /// cmangos/mangoszero: it's not slot id
    ///
    pub unknown1: u16,
}

impl InitialSpell {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // spell_id: u16
        w.write_all(&self.spell_id.to_le_bytes())?;

        // unknown1: u16
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
}

impl InitialSpell {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // spell_id: u16
        let spell_id = crate::util::read_u16_le(r)?;

        // unknown1: u16
        let unknown1 = crate::util::read_u16_le(r)?;

        Ok(Self {
            spell_id,
            unknown1,
        })
    }

}

