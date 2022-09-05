use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::Map;
use crate::world::vanilla::RaidInstanceMessage;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_raid_instance_message.wowm:18`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_raid_instance_message.wowm#L18):
/// ```text
/// smsg SMSG_RAID_INSTANCE_MESSAGE = 0x02FA {
///     RaidInstanceMessage message_type;
///     Map map;
///     u32 time_left;
/// }
/// ```
pub struct SMSG_RAID_INSTANCE_MESSAGE {
    pub message_type: RaidInstanceMessage,
    pub map: Map,
    pub time_left: u32,
}

impl crate::Message for SMSG_RAID_INSTANCE_MESSAGE {
    const OPCODE: u32 = 0x02fa;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // message_type: RaidInstanceMessage
        w.write_all(&(self.message_type.as_int() as u32).to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // time_left: u32
        w.write_all(&self.time_left.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // message_type: RaidInstanceMessage
        let message_type: RaidInstanceMessage = crate::util::read_u32_le(r)?.try_into()?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // time_left: u32
        let time_left = crate::util::read_u32_le(r)?;

        Ok(Self {
            message_type,
            map,
            time_left,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_RAID_INSTANCE_MESSAGE {}

