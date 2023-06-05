use std::io::{Read, Write};

use crate::wrath::EquipmentSetListItem;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_equipment_set_list.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_equipment_set_list.wowm#L10):
/// ```text
/// smsg SMSG_EQUIPMENT_SET_LIST = 0x04BC {
///     u32 amount_of_equipment_sets;
///     EquipmentSetListItem[amount_of_equipment_sets] equipment_sets;
/// }
/// ```
pub struct SMSG_EQUIPMENT_SET_LIST {
    pub equipment_sets: Vec<EquipmentSetListItem>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_EQUIPMENT_SET_LIST {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_EQUIPMENT_SET_LIST {{").unwrap();
        // Members
        writeln!(s, "    amount_of_equipment_sets = {};", self.equipment_sets.len()).unwrap();
        write!(s, "    equipment_sets = [").unwrap();
        for v in self.equipment_sets.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "    guid = {};", v.guid.guid()).unwrap();
            writeln!(s, "    name = \"{}\";", v.name).unwrap();
            writeln!(s, "    icon_name = \"{}\";", v.icon_name).unwrap();
            write!(s, "    equipment = [").unwrap();
            for v in v.equipment.as_slice() {
                write!(s, "{v:#08X}, ").unwrap();
            }
            writeln!(s, "];").unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1212_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_equipment_sets");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_EQUIPMENT_SET_LIST {}
impl crate::Message for SMSG_EQUIPMENT_SET_LIST {
    const OPCODE: u32 = 0x04bc;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(4..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04BC, size: body_size });
        }

        // amount_of_equipment_sets: u32
        let amount_of_equipment_sets = crate::util::read_u32_le(&mut r)?;

        // equipment_sets: EquipmentSetListItem[amount_of_equipment_sets]
        let equipment_sets = {
            let mut equipment_sets = Vec::with_capacity(amount_of_equipment_sets as usize);
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
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_EQUIPMENT_SET_LIST {}

impl SMSG_EQUIPMENT_SET_LIST {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_equipment_sets: u32
        + self.equipment_sets.iter().fold(0, |acc, x| acc + x.size()) // equipment_sets: EquipmentSetListItem[amount_of_equipment_sets]
    }
}

