use crate:: {
    Guid,
};
use crate::wrath::Class;
use crate::wrath::DeclinedNames;
use crate::wrath::Gender;
use crate::wrath::Race;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Response to [`CMSG_NAME_QUERY`](crate::vanilla::CMSG_NAME_QUERY).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm:40`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm#L40):
/// ```text
/// smsg SMSG_NAME_QUERY_RESPONSE = 0x0051 {
///     PackedGuid guid;
///     u8 early_terminate = 0;
///     CString character_name;
///     CString realm_name;
///     (u8)Race race;
///     (u8)Gender gender;
///     (u8)Class class;
///     DeclinedNames has_declined_names;
///     if (has_declined_names == YES) {
///         CString[5] declined_names;
///     }
/// }
/// ```
pub struct SMSG_NAME_QUERY_RESPONSE {
    pub guid: Guid,
    pub character_name: String,
    /// Used for showing cross realm realm names. If this is an empty string it is shown like a regular player on the same realm.
    ///
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

impl crate::Message for SMSG_NAME_QUERY_RESPONSE {
    const OPCODE: u32 = 0x0051;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        // early_terminate: u8
        w.write_all(&Self::EARLY_TERMINATE_VALUE.to_le_bytes())?;

        // character_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.character_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `character_name` must not be null-terminated.");
        w.write_all(self.character_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // realm_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.realm_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `realm_name` must not be null-terminated.");
        w.write_all(self.realm_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // race: Race
        w.write_all(&u8::from(self.race.as_int()).to_le_bytes())?;

        // gender: Gender
        w.write_all(&u8::from(self.gender.as_int()).to_le_bytes())?;

        // class: Class
        w.write_all(&u8::from(self.class.as_int()).to_le_bytes())?;

        // has_declined_names: DeclinedNames
        w.write_all(&u8::from(self.has_declined_names.as_int()).to_le_bytes())?;

        match &self.has_declined_names {
            SMSG_NAME_QUERY_RESPONSE_DeclinedNames::No => {}
            SMSG_NAME_QUERY_RESPONSE_DeclinedNames::Yes {
                declined_names,
            } => {
                // declined_names: CString[5]
                for i in declined_names.iter() {
                    w.write_all(i.as_bytes())?;
                    w.write_all(&[0])?;
                }

            }
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(9..=1806).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0051, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

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
        let race: Race = (crate::util::read_u8_le(&mut r)? as u8).try_into()?;

        // gender: Gender
        let gender: Gender = (crate::util::read_u8_le(&mut r)? as u8).try_into()?;

        // class: Class
        let class: Class = (crate::util::read_u8_le(&mut r)? as u8).try_into()?;

        // has_declined_names: DeclinedNames
        let has_declined_names: DeclinedNames = crate::util::read_u8_le(&mut r)?.try_into()?;

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
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_NAME_QUERY_RESPONSE {}

impl SMSG_NAME_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
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

impl SMSG_NAME_QUERY_RESPONSE_DeclinedNames {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::No => {
                1
            }
            Self::Yes {
                declined_names,
            } => {
                1
                + declined_names.iter().fold(0, |acc, x| acc + x.len() + 1) // declined_names: CString[5]
            }
        }
    }
}

