use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_gameobject_query_response.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_gameobject_query_response.wowm#L3):
/// ```text
/// smsg SMSG_GAMEOBJECT_QUERY_RESPONSE = 0x005F {
///     u32 entry_id;
///     optional found {
///         u32 info_type;
///         u32 display_id;
///         CString name1;
///         CString name2;
///         CString name3;
///         CString name4;
///         CString name5;
///         u32[6] raw_data;
///     }
/// }
/// ```
pub struct SMSG_GAMEOBJECT_QUERY_RESPONSE {
    pub entry_id: u32,
    pub found: Option<SMSG_GAMEOBJECT_QUERY_RESPONSE_found>,
}

impl crate::Message for SMSG_GAMEOBJECT_QUERY_RESPONSE {
    const OPCODE: u32 = 0x005f;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // entry_id: u32
        w.write_all(&self.entry_id.to_le_bytes())?;

        // optional found
        if let Some(v) = &self.found {
            // info_type: u32
            w.write_all(&v.info_type.to_le_bytes())?;

            // display_id: u32
            w.write_all(&v.display_id.to_le_bytes())?;

            // name1: CString
            // Guard against strings that are already null-terminated
            assert_ne!(v.name1.as_bytes().iter().rev().next(), Some(&0u8), "String name1 must not be null-terminated.");
            w.write_all(v.name1.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name2: CString
            // Guard against strings that are already null-terminated
            assert_ne!(v.name2.as_bytes().iter().rev().next(), Some(&0u8), "String name2 must not be null-terminated.");
            w.write_all(v.name2.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name3: CString
            // Guard against strings that are already null-terminated
            assert_ne!(v.name3.as_bytes().iter().rev().next(), Some(&0u8), "String name3 must not be null-terminated.");
            w.write_all(v.name3.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name4: CString
            // Guard against strings that are already null-terminated
            assert_ne!(v.name4.as_bytes().iter().rev().next(), Some(&0u8), "String name4 must not be null-terminated.");
            w.write_all(v.name4.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name5: CString
            // Guard against strings that are already null-terminated
            assert_ne!(v.name5.as_bytes().iter().rev().next(), Some(&0u8), "String name5 must not be null-terminated.");
            w.write_all(v.name5.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // raw_data: u32[6]
            for i in v.raw_data.iter() {
                w.write_all(&i.to_le_bytes())?;
            }

        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // entry_id: u32
        let entry_id = crate::util::read_u32_le(r)?;

        // optional found
        let current_size = {
            4 // entry_id: u32
        };
        let found = if current_size < body_size as usize {
            // info_type: u32
            let info_type = crate::util::read_u32_le(r)?;

            // display_id: u32
            let display_id = crate::util::read_u32_le(r)?;

            // name1: CString
            let name1 = crate::util::read_c_string_to_vec(r)?;
            let name1 = String::from_utf8(name1)?;

            // name2: CString
            let name2 = crate::util::read_c_string_to_vec(r)?;
            let name2 = String::from_utf8(name2)?;

            // name3: CString
            let name3 = crate::util::read_c_string_to_vec(r)?;
            let name3 = String::from_utf8(name3)?;

            // name4: CString
            let name4 = crate::util::read_c_string_to_vec(r)?;
            let name4 = String::from_utf8(name4)?;

            // name5: CString
            let name5 = crate::util::read_c_string_to_vec(r)?;
            let name5 = String::from_utf8(name5)?;

            // raw_data: u32[6]
            let mut raw_data = [u32::default(); 6];
            for i in raw_data.iter_mut() {
                *i = crate::util::read_u32_le(r)?;
            }

            Some(SMSG_GAMEOBJECT_QUERY_RESPONSE_found {
                info_type,
                display_id,
                name1,
                name2,
                name3,
                name4,
                name5,
                raw_data,
            })
        } else {
            None
        };

        Ok(Self {
            entry_id,
            found,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_GAMEOBJECT_QUERY_RESPONSE {}

impl SMSG_GAMEOBJECT_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // entry_id: u32
        + if let Some(found) = &self.found {
            4 // info_type: u32
            + 4 // display_id: u32
            + found.name1.len() + 1 // name1: CString
            + found.name2.len() + 1 // name2: CString
            + found.name3.len() + 1 // name3: CString
            + found.name4.len() + 1 // name4: CString
            + found.name5.len() + 1 // name5: CString
            + 6 * core::mem::size_of::<u32>() // raw_data: u32[6]
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SMSG_GAMEOBJECT_QUERY_RESPONSE_found {
    pub info_type: u32,
    pub display_id: u32,
    pub name1: String,
    pub name2: String,
    pub name3: String,
    pub name4: String,
    pub name5: String,
    pub raw_data: [u32; 6],
}

impl SMSG_GAMEOBJECT_QUERY_RESPONSE_found {
    pub(crate) fn size(&self) -> usize {
        4 // info_type: u32
        + 4 // display_id: u32
        + self.name1.len() + 1 // name1: CString
        + self.name2.len() + 1 // name2: CString
        + self.name3.len() + 1 // name3: CString
        + self.name4.len() + 1 // name4: CString
        + self.name5.len() + 1 // name5: CString
        + 6 * core::mem::size_of::<u32>() // raw_data: u32[6]
    }

}

