use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_gameobject_query_response.wowm:42`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_gameobject_query_response.wowm#L42):
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
///         CString icon_name;
///         CString cast_bar_caption;
///         CString unknown;
///         u32[6] raw_data;
///         f32 gameobject_size;
///         u32[6] gameobject_quest_items;
///     }
/// }
/// ```
pub struct SMSG_GAMEOBJECT_QUERY_RESPONSE {
    /// When the `found` optional is not present all emulators bitwise OR the entry with `0x80000000`.``
    ///
    pub entry_id: u32,
    pub found: Option<SMSG_GAMEOBJECT_QUERY_RESPONSE_found>,
}

impl crate::Message for SMSG_GAMEOBJECT_QUERY_RESPONSE {
    const OPCODE: u32 = 0x005f;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // entry_id: u32
        w.write_all(&self.entry_id.to_le_bytes())?;

        // optional found
        if let Some(v) = &self.found {
            // info_type: u32
            w.write_all(&v.info_type.to_le_bytes())?;

            // display_id: u32
            w.write_all(&v.display_id.to_le_bytes())?;

            // name1: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name1.as_bytes().iter().rev().next(), Some(&0_u8), "String `name1` must not be null-terminated.");
            w.write_all(v.name1.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name2: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name2.as_bytes().iter().rev().next(), Some(&0_u8), "String `name2` must not be null-terminated.");
            w.write_all(v.name2.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name3: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name3.as_bytes().iter().rev().next(), Some(&0_u8), "String `name3` must not be null-terminated.");
            w.write_all(v.name3.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name4: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name4.as_bytes().iter().rev().next(), Some(&0_u8), "String `name4` must not be null-terminated.");
            w.write_all(v.name4.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // icon_name: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.icon_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `icon_name` must not be null-terminated.");
            w.write_all(v.icon_name.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // cast_bar_caption: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.cast_bar_caption.as_bytes().iter().rev().next(), Some(&0_u8), "String `cast_bar_caption` must not be null-terminated.");
            w.write_all(v.cast_bar_caption.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // unknown: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.unknown.as_bytes().iter().rev().next(), Some(&0_u8), "String `unknown` must not be null-terminated.");
            w.write_all(v.unknown.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // raw_data: u32[6]
            for i in v.raw_data.iter() {
                w.write_all(&i.to_le_bytes())?;
            }

            // gameobject_size: f32
            w.write_all(&v.gameobject_size.to_le_bytes())?;

            // gameobject_quest_items: u32[6]
            for i in v.gameobject_quest_items.iter() {
                w.write_all(&i.to_le_bytes())?;
            }

        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=1856).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x005F, size: body_size as u32 });
        }

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
            let name1 = {
                let name1 = crate::util::read_c_string_to_vec(r)?;
                String::from_utf8(name1)?
            };

            // name2: CString
            let name2 = {
                let name2 = crate::util::read_c_string_to_vec(r)?;
                String::from_utf8(name2)?
            };

            // name3: CString
            let name3 = {
                let name3 = crate::util::read_c_string_to_vec(r)?;
                String::from_utf8(name3)?
            };

            // name4: CString
            let name4 = {
                let name4 = crate::util::read_c_string_to_vec(r)?;
                String::from_utf8(name4)?
            };

            // icon_name: CString
            let icon_name = {
                let icon_name = crate::util::read_c_string_to_vec(r)?;
                String::from_utf8(icon_name)?
            };

            // cast_bar_caption: CString
            let cast_bar_caption = {
                let cast_bar_caption = crate::util::read_c_string_to_vec(r)?;
                String::from_utf8(cast_bar_caption)?
            };

            // unknown: CString
            let unknown = {
                let unknown = crate::util::read_c_string_to_vec(r)?;
                String::from_utf8(unknown)?
            };

            // raw_data: u32[6]
            let raw_data = {
                let mut raw_data = [u32::default(); 6];
                for i in raw_data.iter_mut() {
                    *i = crate::util::read_u32_le(r)?;
                }
                raw_data
            };

            // gameobject_size: f32
            let gameobject_size = crate::util::read_f32_le(r)?;
            // gameobject_quest_items: u32[6]
            let gameobject_quest_items = {
                let mut gameobject_quest_items = [u32::default(); 6];
                for i in gameobject_quest_items.iter_mut() {
                    *i = crate::util::read_u32_le(r)?;
                }
                gameobject_quest_items
            };

            Some(SMSG_GAMEOBJECT_QUERY_RESPONSE_found {
                info_type,
                display_id,
                name1,
                name2,
                name3,
                name4,
                icon_name,
                cast_bar_caption,
                unknown,
                raw_data,
                gameobject_size,
                gameobject_quest_items,
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
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GAMEOBJECT_QUERY_RESPONSE {}

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
            + found.icon_name.len() + 1 // icon_name: CString
            + found.cast_bar_caption.len() + 1 // cast_bar_caption: CString
            + found.unknown.len() + 1 // unknown: CString
            + 6 * core::mem::size_of::<u32>() // raw_data: u32[6]
            + 4 // gameobject_size: f32
            + 6 * core::mem::size_of::<u32>() // gameobject_quest_items: u32[6]
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct SMSG_GAMEOBJECT_QUERY_RESPONSE_found {
    pub info_type: u32,
    pub display_id: u32,
    pub name1: String,
    pub name2: String,
    pub name3: String,
    pub name4: String,
    pub icon_name: String,
    pub cast_bar_caption: String,
    pub unknown: String,
    pub raw_data: [u32; 6],
    pub gameobject_size: f32,
    pub gameobject_quest_items: [u32; 6],
}

impl SMSG_GAMEOBJECT_QUERY_RESPONSE_found {
    pub(crate) fn size(&self) -> usize {
        4 // info_type: u32
        + 4 // display_id: u32
        + self.name1.len() + 1 // name1: CString
        + self.name2.len() + 1 // name2: CString
        + self.name3.len() + 1 // name3: CString
        + self.name4.len() + 1 // name4: CString
        + self.icon_name.len() + 1 // icon_name: CString
        + self.cast_bar_caption.len() + 1 // cast_bar_caption: CString
        + self.unknown.len() + 1 // unknown: CString
        + 6 * core::mem::size_of::<u32>() // raw_data: u32[6]
        + 4 // gameobject_size: f32
        + 6 * core::mem::size_of::<u32>() // gameobject_quest_items: u32[6]
    }

}

