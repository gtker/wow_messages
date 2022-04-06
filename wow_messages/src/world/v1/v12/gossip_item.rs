use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gossip/smsg_gossip_message.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gossip/smsg_gossip_message.wowm#L3):
/// ```text
/// struct GossipItem {
///     u32 id;
///     u8 item_icon;
///     u8 coded;
/// }
/// ```
pub struct GossipItem {
    pub id: u32,
    pub item_icon: u8,
    pub coded: u8,
}

impl ReadableAndWritable for GossipItem {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // item_icon: u8
        let item_icon = crate::util::read_u8_le(r)?;

        // coded: u8
        let coded = crate::util::read_u8_le(r)?;

        Ok(Self {
            id,
            item_icon,
            coded,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // item_icon: u8
        w.write_all(&self.item_icon.to_le_bytes())?;

        // coded: u8
        w.write_all(&self.coded.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for GossipItem {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for GossipItem {
    fn maximum_possible_size() -> usize {
        4 // id: u32
        + 1 // item_icon: u8
        + 1 // coded: u8
    }
}

