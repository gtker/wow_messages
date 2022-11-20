use std::convert::{TryFrom, TryInto};
use crate::world::wrath::Map;
use crate::world::wrath::RaidDifficulty;
use crate::world::wrath::RaidInstanceMessage;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_raid_instance_message.wowm:45`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_raid_instance_message.wowm#L45):
/// ```text
/// smsg SMSG_RAID_INSTANCE_MESSAGE = 0x02FA {
///     RaidInstanceMessage message_type;
///     Map map;
///     (u32)RaidDifficulty difficulty;
///     u32 time_left;
/// }
/// ```
pub struct SMSG_RAID_INSTANCE_MESSAGE {
    pub message_type: RaidInstanceMessage,
    pub map: Map,
    pub difficulty: RaidDifficulty,
    pub time_left: u32,
}

impl crate::Message for SMSG_RAID_INSTANCE_MESSAGE {
    const OPCODE: u32 = 0x02fa;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // message_type: RaidInstanceMessage
        w.write_all(&(self.message_type.as_int() as u32).to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // difficulty: RaidDifficulty
        w.write_all(&(self.difficulty.as_int() as u32).to_le_bytes())?;

        // time_left: u32
        w.write_all(&self.time_left.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02FA, size: body_size as u32 });
        }

        // message_type: RaidInstanceMessage
        let message_type: RaidInstanceMessage = crate::util::read_u32_le(r)?.try_into()?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // difficulty: RaidDifficulty
        let difficulty: RaidDifficulty = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // time_left: u32
        let time_left = crate::util::read_u32_le(r)?;

        Ok(Self {
            message_type,
            map,
            difficulty,
            time_left,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_RAID_INSTANCE_MESSAGE {}

