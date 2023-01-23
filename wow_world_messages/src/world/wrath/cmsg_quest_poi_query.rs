use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_quest_poi_query.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_quest_poi_query.wowm#L1):
/// ```text
/// cmsg CMSG_QUEST_POI_QUERY = 0x01E3 {
///     u32 amount_of_pois;
///     u32[amount_of_pois] points_of_interests;
/// }
/// ```
pub struct CMSG_QUEST_POI_QUERY {
    pub points_of_interests: Vec<u32>,
}

impl crate::Message for CMSG_QUEST_POI_QUERY {
    const OPCODE: u32 = 0x01e3;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // amount_of_pois: u32
        w.write_all(&(self.points_of_interests.len() as u32).to_le_bytes())?;

        // points_of_interests: u32[amount_of_pois]
        for i in self.points_of_interests.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01E3, size: body_size as u32 });
        }

        // amount_of_pois: u32
        let amount_of_pois = crate::util::read_u32_le(r)?;

        // points_of_interests: u32[amount_of_pois]
        let mut points_of_interests = Vec::with_capacity(amount_of_pois as usize);
        for i in 0..amount_of_pois {
            points_of_interests.push(crate::util::read_u32_le(r)?);
        }

        Ok(Self {
            points_of_interests,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_QUEST_POI_QUERY {}

impl CMSG_QUEST_POI_QUERY {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_pois: u32
        + self.points_of_interests.len() * core::mem::size_of::<u32>() // points_of_interests: u32[amount_of_pois]
    }
}

