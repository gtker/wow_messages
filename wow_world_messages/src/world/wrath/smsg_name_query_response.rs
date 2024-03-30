use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    Class, DeclinedNames, Gender, Race,
};

/// Response to [`CMSG_NAME_QUERY`](crate::vanilla::CMSG_NAME_QUERY).
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm:39`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm#L39):
/// ```text
/// smsg SMSG_NAME_QUERY_RESPONSE = 0x0051 {
///     PackedGuid guid;
///     u8 early_terminate = 0;
///     CString character_name;
///     CString realm_name;
///     Race race;
///     Gender gender;
///     Class class;
///     DeclinedNames has_declined_names;
///     if (has_declined_names == YES) {
///         CString[5] declined_names;
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_NAME_QUERY_RESPONSE {
    pub guid: Guid,
    pub character_name: String,
    /// Used for showing cross realm realm names. If this is an empty string it is shown like a regular player on the same realm.
    pub realm_name: String,
    pub race: Race,
    pub gender: Gender,
    pub class: Class,
    pub has_declined_names: SMSG_NAME_QUERY_RESPONSE_DeclinedNames,
}

impl SMSG_NAME_QUERY_RESPONSE {
    /// The field `early_terminate` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `0` |
    /// | Hex | `0x00` |
    /// | Original | `0` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const EARLY_TERMINATE_VALUE: u8 = 0x00;

}

impl crate::private::Sealed for SMSG_NAME_QUERY_RESPONSE {}
impl SMSG_NAME_QUERY_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(8..=1806).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // early_terminate: u8
        let _early_terminate = crate::util::read_u8_le(&mut r)?;
        // early_terminate is expected to always be 0 (0)

        // character_name: CString
        let character_name = {
            let character_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(character_name)?
        };

        // realm_name: CString
        let realm_name = {
            let realm_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(realm_name)?
        };

        // race: Race
        let race = crate::util::read_u8_le(&mut r)?.try_into()?;

        // gender: Gender
        let gender = crate::util::read_u8_le(&mut r)?.try_into()?;

        // class: Class
        let class = crate::util::read_u8_le(&mut r)?.try_into()?;

        // has_declined_names: DeclinedNames
        let has_declined_names = crate::util::read_u8_le(&mut r)?.try_into()?;

        let has_declined_names_if = match has_declined_names {
            DeclinedNames::No => SMSG_NAME_QUERY_RESPONSE_DeclinedNames::No,
            DeclinedNames::Yes => {
                // declined_names: CString[5]
                let declined_names = {
                    let mut declined_names = [(); 5].map(|_| String::default());
                    for i in declined_names.iter_mut() {
                        let s = crate::util::read_c_string_to_vec(&mut r)?;
                        *i = String::from_utf8(s)?;
                    }
                    declined_names
                };

                SMSG_NAME_QUERY_RESPONSE_DeclinedNames::Yes {
                    declined_names,
                }
            }
        };

        Ok(Self {
            guid,
            character_name,
            realm_name,
            race,
            gender,
            class,
            has_declined_names: has_declined_names_if,
        })
    }

}

impl crate::Message for SMSG_NAME_QUERY_RESPONSE {
    const OPCODE: u32 = 0x0051;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_NAME_QUERY_RESPONSE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_NAME_QUERY_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    character_name = \"{}\";", self.character_name).unwrap();
        writeln!(s, "    realm_name = \"{}\";", self.realm_name).unwrap();
        writeln!(s, "    race = {};", self.race.as_test_case_value()).unwrap();
        writeln!(s, "    gender = {};", self.gender.as_test_case_value()).unwrap();
        writeln!(s, "    class = {};", self.class.as_test_case_value()).unwrap();
        writeln!(s, "    has_declined_names = {};", DeclinedNames::try_from(self.has_declined_names.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.has_declined_names {
            crate::wrath::SMSG_NAME_QUERY_RESPONSE_DeclinedNames::Yes {
                declined_names,
            } => {
                writeln!(s, "    declined_names = [").unwrap();
                for v in declined_names.as_slice() {
                    write!(s, "\"{v}\", ").unwrap();
                }
                writeln!(s, "    ];").unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 81_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.guid), "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "early_terminate", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.character_name.len() + 1, "character_name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.realm_name.len() + 1, "realm_name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "race", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "gender", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "class", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "has_declined_names", "    ");
        match &self.has_declined_names {
            crate::wrath::SMSG_NAME_QUERY_RESPONSE_DeclinedNames::Yes {
                declined_names,
            } => {
                writeln!(s, "    /* declined_names: CString[5] start */").unwrap();
                for (i, v) in declined_names.iter().enumerate() {
                    crate::util::write_bytes(&mut s, &mut bytes, v.len() + 1, &format!("declined_names {i}"), "    ");
                }
                writeln!(s, "    /* declined_names: CString[5] end */").unwrap();
            }
            _ => {}
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
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // early_terminate: u8
        w.write_all(&Self::EARLY_TERMINATE_VALUE.to_le_bytes())?;

        // character_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.character_name.as_bytes().iter().next_back(), Some(&0_u8), "String `character_name` must not be null-terminated.");
        w.write_all(self.character_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // realm_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.realm_name.as_bytes().iter().next_back(), Some(&0_u8), "String `realm_name` must not be null-terminated.");
        w.write_all(self.realm_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // race: Race
        w.write_all(&(self.race.as_int().to_le_bytes()))?;

        // gender: Gender
        w.write_all(&(self.gender.as_int().to_le_bytes()))?;

        // class: Class
        w.write_all(&(self.class.as_int().to_le_bytes()))?;

        // has_declined_names: DeclinedNames
        w.write_all(&(self.has_declined_names.as_int().to_le_bytes()))?;

        match &self.has_declined_names {
            SMSG_NAME_QUERY_RESPONSE_DeclinedNames::Yes {
                declined_names,
            } => {
                // declined_names: CString[5]
                for i in declined_names.iter() {
                    w.write_all(i.as_bytes())?;
                    w.write_all(&[0])?;
                }

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(81, "SMSG_NAME_QUERY_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_NAME_QUERY_RESPONSE {}

impl SMSG_NAME_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 1 // early_terminate: u8
        + self.character_name.len() + 1 // character_name: CString
        + self.realm_name.len() + 1 // realm_name: CString
        + 1 // race: Race
        + 1 // gender: Gender
        + 1 // class: Class
        + self.has_declined_names.size() // has_declined_names: SMSG_NAME_QUERY_RESPONSE_DeclinedNames
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_NAME_QUERY_RESPONSE_DeclinedNames {
    No,
    Yes {
        declined_names: [String; 5],
    },
}

impl Default for SMSG_NAME_QUERY_RESPONSE_DeclinedNames {
    fn default() -> Self {
        // First enumerator without any fields
        Self::No
    }
}

impl SMSG_NAME_QUERY_RESPONSE_DeclinedNames {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::No => 0,
            Self::Yes { .. } => 1,
        }
    }

}

impl std::fmt::Display for SMSG_NAME_QUERY_RESPONSE_DeclinedNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::No => f.write_str("No"),
            Self::Yes{ .. } => f.write_str("Yes"),
        }
    }
}

impl SMSG_NAME_QUERY_RESPONSE_DeclinedNames {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Yes {
                declined_names,
            } => {
                1
                + declined_names.iter().fold(0, |acc, x| acc + x.len() + 1) // declined_names: CString[5]
            }
            _ => 1,
        }
    }
}

