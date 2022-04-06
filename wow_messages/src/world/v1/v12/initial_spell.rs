use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/smsg_initial_spells.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/smsg_initial_spells.wowm#L13):
/// ```text
/// struct InitialSpell {
///     u16 spell_id;
///     u16 unknown1;
/// }
/// ```
pub struct InitialSpell {
    pub spell_id: u16,
    pub unknown1: u16,
}

impl ReadableAndWritable for InitialSpell {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // spell_id: u16
        let spell_id = crate::util::read_u16_le(r)?;

        // unknown1: u16
        let unknown1 = crate::util::read_u16_le(r)?;

        Ok(Self {
            spell_id,
            unknown1,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // spell_id: u16
        w.write_all(&self.spell_id.to_le_bytes())?;

        // unknown1: u16
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for InitialSpell {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for InitialSpell {
    fn maximum_possible_size() -> usize {
        2 // spell_id: u16
        + 2 // unknown1: u16
    }
}

