use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::EquipmentSetListItem;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_equipment_set_list.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_equipment_set_list.wowm#L10):
/// ```text
/// smsg SMSG_EQUIPMENT_SET_LIST = 0x04BC {
///     u32 amount_of_equipment_sets;
///     EquipmentSetListItem[amount_of_equipment_sets] equipment_sets;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_EQUIPMENT_SET_LIST {
    pub equipment_sets: Vec<EquipmentSetListItem>,
}

impl crate::private::Sealed for SMSG_EQUIPMENT_SET_LIST {}
impl SMSG_EQUIPMENT_SET_LIST {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // amount_of_equipment_sets: u32
        let amount_of_equipment_sets = crate::util::read_u32_le(&mut r)?;

        // equipment_sets: EquipmentSetListItem[amount_of_equipment_sets]
        let equipment_sets = {
            let mut equipment_sets = Vec::with_capacity(amount_of_equipment_sets as usize);

            let allocation_size = u64::from(amount_of_equipment_sets) * 162;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_equipment_sets {
                equipment_sets.push(EquipmentSetListItem::read(&mut r)?);
            }
            equipment_sets
        };

        Ok(Self {
            equipment_sets,
        })
    }

}

impl crate::Message for SMSG_EQUIPMENT_SET_LIST {
    const OPCODE: u32 = 0x04bc;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_EQUIPMENT_SET_LIST"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_EQUIPMENT_SET_LIST {{").unwrap();
        // Members
        writeln!(s, "    amount_of_equipment_sets = {};", self.equipment_sets.len()).unwrap();
        writeln!(s, "    equipment_sets = [").unwrap();
        for v in self.equipment_sets.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            guid = {};", v.guid.guid()).unwrap();
            writeln!(s, "            name = \"{}\";", v.name).unwrap();
            writeln!(s, "            icon_name = \"{}\";", v.icon_name).unwrap();
            writeln!(s, "            equipment = [").unwrap();
            for v in v.equipment.as_slice() {
                write!(s, "{v:#08X}, ").unwrap();
            }
            writeln!(s, "            ];").unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1212_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_equipment_sets", "    ");
        if !self.equipment_sets.is_empty() {
            writeln!(s, "    /* equipment_sets: EquipmentSetListItem[amount_of_equipment_sets] start */").unwrap();
            for (i, v) in self.equipment_sets.iter().enumerate() {
                writeln!(s, "    /* equipment_sets: EquipmentSetListItem[amount_of_equipment_sets] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.name.len() + 1, "name", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.icon_name.len() + 1, "icon_name", "        ");
                writeln!(s, "    /* equipment: Guid[19] start */").unwrap();
                for (i, v) in v.equipment.iter().enumerate() {
                    crate::util::write_bytes(&mut s, &mut bytes, 8, &format!("equipment {i}"), "        ");
                }
                writeln!(s, "    /* equipment: Guid[19] end */").unwrap();
                writeln!(s, "    /* equipment_sets: EquipmentSetListItem[amount_of_equipment_sets] {i} end */").unwrap();
            }
            writeln!(s, "    /* equipment_sets: EquipmentSetListItem[amount_of_equipment_sets] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_equipment_sets: u32
        w.write_all(&(self.equipment_sets.len() as u32).to_le_bytes())?;

        // equipment_sets: EquipmentSetListItem[amount_of_equipment_sets]
        for i in self.equipment_sets.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1212, "SMSG_EQUIPMENT_SET_LIST", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_EQUIPMENT_SET_LIST {}

impl SMSG_EQUIPMENT_SET_LIST {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_equipment_sets: u32
        + self.equipment_sets.iter().fold(0, |acc, x| acc + x.size()) // equipment_sets: EquipmentSetListItem[amount_of_equipment_sets]
    }
}

