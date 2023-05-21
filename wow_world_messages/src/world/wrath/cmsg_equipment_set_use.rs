use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::EquipmentSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_equipment_set_use.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_equipment_set_use.wowm#L9):
/// ```text
/// cmsg CMSG_EQUIPMENT_SET_USE = 0x04D5 {
///     EquipmentSet[19] sets;
/// }
/// ```
pub struct CMSG_EQUIPMENT_SET_USE {
    pub sets: [EquipmentSet; 19],
}

impl crate::private::Sealed for CMSG_EQUIPMENT_SET_USE {}
impl CMSG_EQUIPMENT_SET_USE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 190 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04D5, size: body_size });
        }

        // sets: EquipmentSet[19]
        let sets = {
            let mut sets = [EquipmentSet::default(); 19];
            for i in sets.iter_mut() {
                *i = EquipmentSet::read(&mut r)?;
            }
            sets
        };

        Ok(Self {
            sets,
        })
    }

}

impl crate::Message for CMSG_EQUIPMENT_SET_USE {
    const OPCODE: u32 = 0x04d5;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_EQUIPMENT_SET_USE {{").unwrap();
        // Members
        write!(s, "    sets = [").unwrap();
        for v in self.sets.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        item = {};", v.item.guid()).unwrap();
            writeln!(s, "        source_bag = {};", v.source_bag).unwrap();
            writeln!(s, "        source_slot = {};", v.source_slot).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 194_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1237_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        writeln!(s, "    /* sets: EquipmentSet[19] start */").unwrap();
        for (i, v) in self.sets.iter().enumerate() {
            writeln!(s, "    /* sets: EquipmentSet[19] {i} start */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 8, "item", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 1, "source_bag", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 1, "source_slot", "        ");
            writeln!(s, "    /* sets: EquipmentSet[19] {i} end */").unwrap();
        }
        writeln!(s, "    /* sets: EquipmentSet[19] end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        190
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // sets: EquipmentSet[19]
        for i in self.sets.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_EQUIPMENT_SET_USE {}

