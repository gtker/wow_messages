use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_gameobject_query_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_gameobject_query_response.wowm#L1):
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_GAMEOBJECT_QUERY_RESPONSE {
    /// When the `found` optional is not present all emulators bitwise OR the entry with `0x80000000`.``
    pub entry_id: u32,
    pub found: Option<SMSG_GAMEOBJECT_QUERY_RESPONSE_found>,
}

impl crate::private::Sealed for SMSG_GAMEOBJECT_QUERY_RESPONSE {}
impl SMSG_GAMEOBJECT_QUERY_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=1316).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // entry_id: u32
        let entry_id = crate::util::read_u32_le(&mut r)?;

        // optional found
        let current_size = {
            4 // entry_id: u32
        };
        let found = if current_size < body_size as usize {
            // info_type: u32
            let info_type = crate::util::read_u32_le(&mut r)?;

            // display_id: u32
            let display_id = crate::util::read_u32_le(&mut r)?;

            // name1: CString
            let name1 = {
                let name1 = crate::util::read_c_string_to_vec(&mut r)?;
                String::from_utf8(name1)?
            };

            // name2: CString
            let name2 = {
                let name2 = crate::util::read_c_string_to_vec(&mut r)?;
                String::from_utf8(name2)?
            };

            // name3: CString
            let name3 = {
                let name3 = crate::util::read_c_string_to_vec(&mut r)?;
                String::from_utf8(name3)?
            };

            // name4: CString
            let name4 = {
                let name4 = crate::util::read_c_string_to_vec(&mut r)?;
                String::from_utf8(name4)?
            };

            // name5: CString
            let name5 = {
                let name5 = crate::util::read_c_string_to_vec(&mut r)?;
                String::from_utf8(name5)?
            };

            // raw_data: u32[6]
            let raw_data = {
                let mut raw_data = [u32::default(); 6];
                for i in raw_data.iter_mut() {
                    *i = crate::util::read_u32_le(&mut r)?;
                }
                raw_data
            };

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

impl crate::Message for SMSG_GAMEOBJECT_QUERY_RESPONSE {
    const OPCODE: u32 = 0x005f;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_GAMEOBJECT_QUERY_RESPONSE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GAMEOBJECT_QUERY_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    entry_id = {};", self.entry_id).unwrap();
        if let Some(found) = &self.found {
            writeln!(s, "    info_type = {};", found.info_type).unwrap();
            writeln!(s, "    display_id = {};", found.display_id).unwrap();
            writeln!(s, "    name1 = \"{}\";", found.name1).unwrap();
            writeln!(s, "    name2 = \"{}\";", found.name2).unwrap();
            writeln!(s, "    name3 = \"{}\";", found.name3).unwrap();
            writeln!(s, "    name4 = \"{}\";", found.name4).unwrap();
            writeln!(s, "    name5 = \"{}\";", found.name5).unwrap();
            writeln!(s, "    raw_data = [").unwrap();
            for v in found.raw_data.as_slice() {
                write!(s, "{v:#04X}, ").unwrap();
            }
            writeln!(s, "    ];").unwrap();
        }

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 95_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "entry_id", "    ");
        if let Some(found) = &self.found {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "info_type", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "display_id", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, found.name1.len() + 1, "name1", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, found.name2.len() + 1, "name2", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, found.name3.len() + 1, "name3", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, found.name4.len() + 1, "name4", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, found.name5.len() + 1, "name5", "    ");
            writeln!(s, "    /* raw_data: u32[6] start */").unwrap();
            for (i, v) in found.raw_data.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("raw_data {i}"), "    ");
            }
            writeln!(s, "    /* raw_data: u32[6] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
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
            assert_ne!(v.name1.as_bytes().iter().next_back(), Some(&0_u8), "String `name1` must not be null-terminated.");
            w.write_all(v.name1.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name2: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name2.as_bytes().iter().next_back(), Some(&0_u8), "String `name2` must not be null-terminated.");
            w.write_all(v.name2.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name3: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name3.as_bytes().iter().next_back(), Some(&0_u8), "String `name3` must not be null-terminated.");
            w.write_all(v.name3.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name4: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name4.as_bytes().iter().next_back(), Some(&0_u8), "String `name4` must not be null-terminated.");
            w.write_all(v.name4.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // name5: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.name5.as_bytes().iter().next_back(), Some(&0_u8), "String `name5` must not be null-terminated.");
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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(95, "SMSG_GAMEOBJECT_QUERY_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_GAMEOBJECT_QUERY_RESPONSE {}

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
            + 24 // raw_data: u32[6]
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

