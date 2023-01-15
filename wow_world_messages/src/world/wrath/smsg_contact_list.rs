use std::convert::{TryFrom, TryInto};
use crate::world::wrath::Relation;
use crate::world::wrath::RelationType;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_contact_list.wowm:38`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_contact_list.wowm#L38):
/// ```text
/// smsg SMSG_CONTACT_LIST = 0x0067 {
///     RelationType list_mask;
///     u32 amount_of_relations;
///     Relation[amount_of_relations] relations;
/// }
/// ```
pub struct SMSG_CONTACT_LIST {
    /// Indicates which kinds of relations are being sent in this list
    ///
    pub list_mask: RelationType,
    pub relations: Vec<Relation>,
}

impl crate::Message for SMSG_CONTACT_LIST {
    const OPCODE: u32 = 0x0067;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // list_mask: RelationType
        w.write_all(&(self.list_mask.as_int() as u32).to_le_bytes())?;

        // amount_of_relations: u32
        w.write_all(&(self.relations.len() as u32).to_le_bytes())?;

        // relations: Relation[amount_of_relations]
        for i in self.relations.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(8..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0067, size: body_size as u32 });
        }

        // list_mask: RelationType
        let list_mask = RelationType::new(crate::util::read_u32_le(r)?);

        // amount_of_relations: u32
        let amount_of_relations = crate::util::read_u32_le(r)?;

        // relations: Relation[amount_of_relations]
        let mut relations = Vec::with_capacity(amount_of_relations as usize);
        for i in 0..amount_of_relations {
            relations.push(Relation::read(r)?);
        }

        Ok(Self {
            list_mask,
            relations,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_CONTACT_LIST {}

impl SMSG_CONTACT_LIST {
    pub(crate) fn size(&self) -> usize {
        4 // list_mask: RelationType
        + 4 // amount_of_relations: u32
        + self.relations.iter().fold(0, |acc, x| acc + x.size()) // relations: Relation[amount_of_relations]
    }
}

