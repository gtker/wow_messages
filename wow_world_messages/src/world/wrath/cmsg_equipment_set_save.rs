use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_equipment_set_save.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_equipment_set_save.wowm#L1):
/// ```text
/// cmsg CMSG_EQUIPMENT_SET_SAVE = 0x04BD {
///     PackedGuid guid;
///     u32 index;
///     CString name;
///     CString icon_name;
///     Guid[19] equipment;
/// }
/// ```
pub struct CMSG_EQUIPMENT_SET_SAVE {
    pub guid: Guid,
    pub index: u32,
    pub name: String,
    pub icon_name: String,
    pub equipment: [Guid; 19],
}

impl crate::private::Sealed for CMSG_EQUIPMENT_SET_SAVE {}
impl CMSG_EQUIPMENT_SET_SAVE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(159..=677).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // index: u32
        let index = crate::util::read_u32_le(&mut r)?;

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // icon_name: CString
        let icon_name = {
            let icon_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(icon_name)?
        };

        // equipment: Guid[19]
        let equipment = {
            let mut equipment = [Guid::default(); 19];
            for i in equipment.iter_mut() {
                *i = crate::util::read_guid(&mut r)?;
            }
            equipment
        };

        Ok(Self {
            guid,
            index,
            name,
            icon_name,
            equipment,
        })
    }

}

impl crate::Message for CMSG_EQUIPMENT_SET_SAVE {
    const OPCODE: u32 = 0x04bd;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_EQUIPMENT_SET_SAVE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_EQUIPMENT_SET_SAVE {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    index = {};", self.index).unwrap();
        writeln!(s, "    name = \"{}\";", self.name).unwrap();
        writeln!(s, "    icon_name = \"{}\";", self.icon_name).unwrap();
        write!(s, "    equipment = [").unwrap();
        for v in self.equipment.as_slice() {
            write!(s, "{v:#08X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1213_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.guid), "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "index", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.name.len() + 1, "name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.icon_name.len() + 1, "icon_name", "    ");
        writeln!(s, "    /* equipment: Guid[19] start */").unwrap();
        for (i, v) in self.equipment.iter().enumerate() {
            crate::util::write_bytes(&mut s, &mut bytes, 8, &format!("equipment {i}"), "    ");
        }
        writeln!(s, "    /* equipment: Guid[19] end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // index: u32
        w.write_all(&self.index.to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().next_back(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // icon_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.icon_name.as_bytes().iter().next_back(), Some(&0_u8), "String `icon_name` must not be null-terminated.");
        w.write_all(self.icon_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // equipment: Guid[19]
        for i in self.equipment.iter() {
            w.write_all(&i.guid().to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1213, "CMSG_EQUIPMENT_SET_SAVE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_EQUIPMENT_SET_SAVE {}

impl CMSG_EQUIPMENT_SET_SAVE {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 4 // index: u32
        + self.name.len() + 1 // name: CString
        + self.icon_name.len() + 1 // icon_name: CString
        + self.equipment.len() *  8 // equipment: Guid[19]
    }
}

