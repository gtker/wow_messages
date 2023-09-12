use std::io::{Read, Write};

use wow_world_base::shared::pet_query_disabled_names_tbc_wrath::PetQueryDisabledNames;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_pet_name_query_response.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_pet_name_query_response.wowm#L30):
/// ```text
/// smsg SMSG_PET_NAME_QUERY_RESPONSE = 0x0053 {
///     u32 pet_number;
///     CString name;
///     u32 pet_name_timestamp;
///     PetQueryDisabledNames names;
///     if (names == PRESENT) {
///         CString[5] declined_names;
///     }
/// }
/// ```
pub struct SMSG_PET_NAME_QUERY_RESPONSE {
    pub pet_number: u32,
    pub name: String,
    pub pet_name_timestamp: u32,
    pub names: SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames,
}

impl crate::private::Sealed for SMSG_PET_NAME_QUERY_RESPONSE {}
impl SMSG_PET_NAME_QUERY_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(10..=1545).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // pet_number: u32
        let pet_number = crate::util::read_u32_le(&mut r)?;

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // pet_name_timestamp: u32
        let pet_name_timestamp = crate::util::read_u32_le(&mut r)?;

        // names: PetQueryDisabledNames
        let names = crate::util::read_u8_le(&mut r)?.try_into()?;

        let names_if = match names {
            PetQueryDisabledNames::Present => {
                // declined_names: CString[5]
                let declined_names = {
                    let mut declined_names = [(); 5].map(|_| String::default());
                    for i in declined_names.iter_mut() {
                        let s = crate::util::read_c_string_to_vec(&mut r)?;
                        *i = String::from_utf8(s)?;
                    }
                    declined_names
                };

                SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames::Present {
                    declined_names,
                }
            }
            PetQueryDisabledNames::NotPresent => SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames::NotPresent,
        };

        Ok(Self {
            pet_number,
            name,
            pet_name_timestamp,
            names: names_if,
        })
    }

}

impl crate::Message for SMSG_PET_NAME_QUERY_RESPONSE {
    const OPCODE: u32 = 0x0053;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_PET_NAME_QUERY_RESPONSE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PET_NAME_QUERY_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    pet_number = {};", self.pet_number).unwrap();
        writeln!(s, "    name = \"{}\";", self.name).unwrap();
        writeln!(s, "    pet_name_timestamp = {};", self.pet_name_timestamp).unwrap();
        writeln!(s, "    names = {};", PetQueryDisabledNames::try_from(self.names.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.names {
            crate::shared::smsg_pet_name_query_response_tbc_wrath::SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames::Present {
                declined_names,
            } => {
                write!(s, "    declined_names = [").unwrap();
                for v in declined_names.as_slice() {
                    write!(s, "\"{v}\", ").unwrap();
                }
                writeln!(s, "];").unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 83_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "pet_number", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.name.len() + 1, "name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "pet_name_timestamp", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "names", "    ");
        match &self.names {
            crate::shared::smsg_pet_name_query_response_tbc_wrath::SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames::Present {
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
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().next_back(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // pet_name_timestamp: u32
        w.write_all(&self.pet_name_timestamp.to_le_bytes())?;

        // names: PetQueryDisabledNames
        w.write_all(&(self.names.as_int().to_le_bytes()))?;

        match &self.names {
            SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames::Present {
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
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(83, "SMSG_PET_NAME_QUERY_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PET_NAME_QUERY_RESPONSE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PET_NAME_QUERY_RESPONSE {}

impl SMSG_PET_NAME_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // pet_number: u32
        + self.name.len() + 1 // name: CString
        + 4 // pet_name_timestamp: u32
        + self.names.size() // names: SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames {
    Present {
        declined_names: [String; 5],
    },
    NotPresent,
}

impl Default for SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NotPresent
    }
}

impl SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Present { .. } => 1,
            Self::NotPresent => 0,
        }
    }

}

impl std::fmt::Display for SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Present{ .. } => f.write_str("Present"),
            Self::NotPresent => f.write_str("NotPresent"),
        }
    }
}

impl SMSG_PET_NAME_QUERY_RESPONSE_PetQueryDisabledNames {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Present {
                declined_names,
            } => {
                1
                + declined_names.iter().fold(0, |acc, x| acc + x.len() + 1) // declined_names: CString[5]
            }
            _ => 1,
        }
    }
}

