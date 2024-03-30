use std::io::{Read, Write};

use crate::Guid;
use crate::shared::level_vanilla_tbc_wrath::Level;
use crate::tbc::{
    Area, Class, FriendStatus, Relation, RelationType,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_contact_list.wowm:38`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_contact_list.wowm#L38):
/// ```text
/// smsg SMSG_CONTACT_LIST = 0x0067 {
///     RelationType list_mask;
///     u32 amount_of_relations;
///     Relation[amount_of_relations] relations;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_CONTACT_LIST {
    /// Indicates which kinds of relations are being sent in this list
    pub list_mask: RelationType,
    pub relations: Vec<Relation>,
}

impl crate::private::Sealed for SMSG_CONTACT_LIST {}
impl SMSG_CONTACT_LIST {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(8..=65535).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // list_mask: RelationType
        let list_mask = RelationType::new(crate::util::read_u32_le(&mut r)?);

        // amount_of_relations: u32
        let amount_of_relations = crate::util::read_u32_le(&mut r)?;

        // relations: Relation[amount_of_relations]
        let relations = {
            let mut relations = Vec::with_capacity(amount_of_relations as usize);

            let allocation_size = u64::from(amount_of_relations) * 13;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_relations {
                relations.push(Relation::read(&mut r)?);
            }
            relations
        };

        Ok(Self {
            list_mask,
            relations,
        })
    }

}

impl crate::Message for SMSG_CONTACT_LIST {
    const OPCODE: u32 = 0x0067;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_CONTACT_LIST"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CONTACT_LIST {{").unwrap();
        // Members
        writeln!(s, "    list_mask = {};", self.list_mask.as_test_case_value()).unwrap();
        writeln!(s, "    amount_of_relations = {};", self.relations.len()).unwrap();
        writeln!(s, "    relations = [").unwrap();
        for v in self.relations.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            guid = {};", v.guid.guid()).unwrap();
            writeln!(s, "            relation_mask = {};", RelationType::new(v.relation_mask.as_int()).as_test_case_value()).unwrap();
            writeln!(s, "            note = \"{}\";", v.note).unwrap();
            if let Some(if_statement) = &v.relation_mask.get_friend() {
                writeln!(s, "            status = {};", FriendStatus::try_from(if_statement.status.as_int()).unwrap().as_test_case_value()).unwrap();
                match &if_statement.status {
                    crate::tbc::Relation_FriendStatus::Online {
                        area,
                        class,
                        level,
                    } => {
                        writeln!(s, "            area = {};", area.as_test_case_value()).unwrap();
                        writeln!(s, "            level = {};", level.as_int()).unwrap();
                        writeln!(s, "            class = {};", class.as_test_case_value()).unwrap();
                    }
                    _ => {}
                }

            }


            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 103_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "list_mask", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_relations", "    ");
        if !self.relations.is_empty() {
            writeln!(s, "    /* relations: Relation[amount_of_relations] start */").unwrap();
            for (i, v) in self.relations.iter().enumerate() {
                writeln!(s, "    /* relations: Relation[amount_of_relations] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "relation_mask", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.note.len() + 1, "note", "        ");
                if let Some(if_statement) = &v.relation_mask.get_friend() {
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "status", "        ");
                    match &if_statement.status {
                        crate::tbc::Relation_FriendStatus::Online {
                            area,
                            class,
                            level,
                        } => {
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "area", "        ");
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "level", "        ");
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "class", "        ");
                        }
                        _ => {}
                    }

                }

                writeln!(s, "    /* relations: Relation[amount_of_relations] {i} end */").unwrap();
            }
            writeln!(s, "    /* relations: Relation[amount_of_relations] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // list_mask: RelationType
        w.write_all(&(self.list_mask.as_int().to_le_bytes()))?;

        // amount_of_relations: u32
        w.write_all(&(self.relations.len() as u32).to_le_bytes())?;

        // relations: Relation[amount_of_relations]
        for i in self.relations.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(103, "SMSG_CONTACT_LIST", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_CONTACT_LIST {}

impl SMSG_CONTACT_LIST {
    pub(crate) fn size(&self) -> usize {
        4 // list_mask: RelationType
        + 4 // amount_of_relations: u32
        + self.relations.iter().fold(0, |acc, x| acc + x.size()) // relations: Relation[amount_of_relations]
    }
}

