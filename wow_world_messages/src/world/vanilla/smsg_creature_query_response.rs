use std::io::{Read, Write};

use crate::vanilla::CreatureFamily;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_creature_query_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_creature_query_response.wowm#L1):
/// ```text
/// smsg SMSG_CREATURE_QUERY_RESPONSE = 0x0061 {
///     u32 creature_entry;
///     optional found {
///         CString name1;
///         CString name2;
///         CString name3;
///         CString name4;
///         CString sub_name;
///         u32 type_flags;
///         u32 creature_type;
///         (u32)CreatureFamily creature_family;
///         u32 creature_rank;
///         u32 unknown0;
///         u32 spell_data_id;
///         u32 display_id;
///         u8 civilian;
///         u8 racial_leader;
///     }
/// }
/// ```
pub struct SMSG_CREATURE_QUERY_RESPONSE {
    /// When the `found` optional is not present all emulators bitwise OR the entry with `0x80000000`.``
    pub creature_entry: u32,
    pub found: Option<SMSG_CREATURE_QUERY_RESPONSE_found>,
}

impl crate::private::Sealed for SMSG_CREATURE_QUERY_RESPONSE {}
impl SMSG_CREATURE_QUERY_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=1314).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // creature_entry: u32
        let creature_entry = crate::util::read_u32_le(&mut r)?;

        // optional found
        let current_size = {
            4 // creature_entry: u32
        };
        let found = if current_size < body_size as usize {
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

            // sub_name: CString
            let sub_name = {
                let sub_name = crate::util::read_c_string_to_vec(&mut r)?;
                String::from_utf8(sub_name)?
            };

            // type_flags: u32
            let type_flags = crate::util::read_u32_le(&mut r)?;

            // creature_type: u32
            let creature_type = crate::util::read_u32_le(&mut r)?;

            // creature_family: CreatureFamily
            let creature_family = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

            // creature_rank: u32
            let creature_rank = crate::util::read_u32_le(&mut r)?;

            // unknown0: u32
            let unknown0 = crate::util::read_u32_le(&mut r)?;

            // spell_data_id: u32
            let spell_data_id = crate::util::read_u32_le(&mut r)?;

            // display_id: u32
            let display_id = crate::util::read_u32_le(&mut r)?;

            // civilian: u8
            let civilian = crate::util::read_u8_le(&mut r)?;

            // racial_leader: u8
            let racial_leader = crate::util::read_u8_le(&mut r)?;

            Some(SMSG_CREATURE_QUERY_RESPONSE_found {
                name1,
                name2,
                name3,
                name4,
                sub_name,
                type_flags,
                creature_type,
                creature_family,
                creature_rank,
                unknown0,
                spell_data_id,
                display_id,
                civilian,
                racial_leader,
            })
        } else {
            None
        };

        Ok(Self {
            creature_entry,
            found,
        })
    }

}

impl crate::Message for SMSG_CREATURE_QUERY_RESPONSE {
    const OPCODE: u32 = 0x0061;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_CREATURE_QUERY_RESPONSE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CREATURE_QUERY_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    creature_entry = {};", self.creature_entry).unwrap();
        if let Some(found) = &self.found {
            writeln!(s, "    name1 = \"{}\";", found.name1).unwrap();
            writeln!(s, "    name2 = \"{}\";", found.name2).unwrap();
            writeln!(s, "    name3 = \"{}\";", found.name3).unwrap();
            writeln!(s, "    name4 = \"{}\";", found.name4).unwrap();
            writeln!(s, "    sub_name = \"{}\";", found.sub_name).unwrap();
            writeln!(s, "    type_flags = {};", found.type_flags).unwrap();
            writeln!(s, "    creature_type = {};", found.creature_type).unwrap();
            writeln!(s, "    creature_family = {};", found.creature_family.as_test_case_value()).unwrap();
            writeln!(s, "    creature_rank = {};", found.creature_rank).unwrap();
            writeln!(s, "    unknown0 = {};", found.unknown0).unwrap();
            writeln!(s, "    spell_data_id = {};", found.spell_data_id).unwrap();
            writeln!(s, "    display_id = {};", found.display_id).unwrap();
            writeln!(s, "    civilian = {};", found.civilian).unwrap();
            writeln!(s, "    racial_leader = {};", found.racial_leader).unwrap();
        }

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 97_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "creature_entry", "    ");
        if let Some(found) = &self.found {
            crate::util::write_bytes(&mut s, &mut bytes, found.name1.len() + 1, "name1", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, found.name2.len() + 1, "name2", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, found.name3.len() + 1, "name3", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, found.name4.len() + 1, "name4", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, found.sub_name.len() + 1, "sub_name", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "type_flags", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "creature_type", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "creature_family", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "creature_rank", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown0", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "spell_data_id", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "display_id", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 1, "civilian", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 1, "racial_leader", "    ");
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
        // creature_entry: u32
        w.write_all(&self.creature_entry.to_le_bytes())?;

        // optional found
        if let Some(v) = &self.found {
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

            // sub_name: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.sub_name.as_bytes().iter().next_back(), Some(&0_u8), "String `sub_name` must not be null-terminated.");
            w.write_all(v.sub_name.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

            // type_flags: u32
            w.write_all(&v.type_flags.to_le_bytes())?;

            // creature_type: u32
            w.write_all(&v.creature_type.to_le_bytes())?;

            // creature_family: CreatureFamily
            w.write_all(&u32::from(v.creature_family.as_int()).to_le_bytes())?;

            // creature_rank: u32
            w.write_all(&v.creature_rank.to_le_bytes())?;

            // unknown0: u32
            w.write_all(&v.unknown0.to_le_bytes())?;

            // spell_data_id: u32
            w.write_all(&v.spell_data_id.to_le_bytes())?;

            // display_id: u32
            w.write_all(&v.display_id.to_le_bytes())?;

            // civilian: u8
            w.write_all(&v.civilian.to_le_bytes())?;

            // racial_leader: u8
            w.write_all(&v.racial_leader.to_le_bytes())?;

        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(97, "SMSG_CREATURE_QUERY_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_CREATURE_QUERY_RESPONSE {}

impl SMSG_CREATURE_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // creature_entry: u32
        + if let Some(found) = &self.found {
            found.name1.len() + 1 // name1: CString
            + found.name2.len() + 1 // name2: CString
            + found.name3.len() + 1 // name3: CString
            + found.name4.len() + 1 // name4: CString
            + found.sub_name.len() + 1 // sub_name: CString
            + 4 // type_flags: u32
            + 4 // creature_type: u32
            + 4 // creature_family: CreatureFamily
            + 4 // creature_rank: u32
            + 4 // unknown0: u32
            + 4 // spell_data_id: u32
            + 4 // display_id: u32
            + 1 // civilian: u8
            + 1 // racial_leader: u8
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_CREATURE_QUERY_RESPONSE_found {
    pub name1: String,
    pub name2: String,
    pub name3: String,
    pub name4: String,
    pub sub_name: String,
    pub type_flags: u32,
    pub creature_type: u32,
    pub creature_family: CreatureFamily,
    pub creature_rank: u32,
    pub unknown0: u32,
    pub spell_data_id: u32,
    pub display_id: u32,
    pub civilian: u8,
    pub racial_leader: u8,
}

