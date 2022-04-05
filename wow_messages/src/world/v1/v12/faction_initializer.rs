use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{FactionFlag};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/add_messages.wowm:1468`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/add_messages.wowm#L1468):
/// ```text
/// struct FactionInitializer {
///     FactionFlag flag;
///     u32 standing;
/// }
/// ```
pub struct FactionInitializer {
    pub flag: FactionFlag,
    pub standing: u32,
}

impl ReadableAndWritable for FactionInitializer {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // flag: FactionFlag
        let flag = FactionFlag::read(r)?;

        // standing: u32
        let standing = crate::util::read_u32_le(r)?;

        Ok(Self {
            flag,
            standing,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // flag: FactionFlag
        self.flag.write(w)?;

        // standing: u32
        w.write_all(&self.standing.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for FactionInitializer {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for FactionInitializer {
    fn maximum_possible_size() -> usize {
        FactionFlag::size() // flag: FactionFlag
        + 4 // standing: u32
    }
}

