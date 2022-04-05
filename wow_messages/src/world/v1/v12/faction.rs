use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/add_messages.wowm:1494`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/add_messages.wowm#L1494):
/// ```text
/// struct Faction {
///     u32 reputation_list_id;
///     u32 standing;
/// }
/// ```
pub struct Faction {
    pub reputation_list_id: u32,
    pub standing: u32,
}

impl ReadableAndWritable for Faction {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // reputation_list_id: u32
        let reputation_list_id = crate::util::read_u32_le(r)?;

        // standing: u32
        let standing = crate::util::read_u32_le(r)?;

        Ok(Self {
            reputation_list_id,
            standing,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes())?;

        // standing: u32
        w.write_all(&self.standing.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for Faction {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for Faction {
    fn maximum_possible_size() -> usize {
        4 // reputation_list_id: u32
        + 4 // standing: u32
    }
}

