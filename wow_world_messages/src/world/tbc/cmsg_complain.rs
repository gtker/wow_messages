use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::SpamType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_complain.wowm:26`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_complain.wowm#L26):
/// ```text
/// cmsg CMSG_COMPLAIN = 0x03C6 {
///     SpamType complaint_type;
///     Guid offender;
///     if (complaint_type == MAIL) {
///         u32 unknown1;
///         u32 mail_id;
///         u32 unknown2;
///     }
///     else if (complaint_type == CHAT) {
///         u32 language;
///         u32 message_type;
///         u32 channel_id;
///         u32 time;
///         CString description;
///     }
/// }
/// ```
pub struct CMSG_COMPLAIN {
    pub complaint_type: CMSG_COMPLAIN_SpamType,
    pub offender: Guid,
}

impl crate::private::Sealed for CMSG_COMPLAIN {}
impl crate::Message for CMSG_COMPLAIN {
    const OPCODE: u32 = 0x03c6;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_COMPLAIN {{").unwrap();
        // Members
        writeln!(s, "    complaint_type = {};", crate::tbc::SpamType::try_from(self.complaint_type.as_int()).unwrap().as_test_case_value()).unwrap();
        writeln!(s, "    offender = {};", self.offender.guid()).unwrap();
        match &self.complaint_type {
            crate::tbc::CMSG_COMPLAIN_SpamType::Mail {
                mail_id,
                unknown1,
                unknown2,
            } => {
                writeln!(s, "    unknown1 = {};", unknown1).unwrap();
                writeln!(s, "    mail_id = {};", mail_id).unwrap();
                writeln!(s, "    unknown2 = {};", unknown2).unwrap();
            }
            crate::tbc::CMSG_COMPLAIN_SpamType::Chat {
                channel_id,
                description,
                language,
                message_type,
                time,
            } => {
                writeln!(s, "    language = {};", language).unwrap();
                writeln!(s, "    message_type = {};", message_type).unwrap();
                writeln!(s, "    channel_id = {};", channel_id).unwrap();
                writeln!(s, "    time = {};", time).unwrap();
                writeln!(s, "    description = \"{}\";", description).unwrap();
            }
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 966_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "complaint_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "offender", "    ");
        match &self.complaint_type {
            crate::tbc::CMSG_COMPLAIN_SpamType::Mail {
                mail_id,
                unknown1,
                unknown2,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "mail_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown2", "    ");
            }
            crate::tbc::CMSG_COMPLAIN_SpamType::Chat {
                channel_id,
                description,
                language,
                message_type,
                time,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "language", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "message_type", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "channel_id", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "time", "    ");
                crate::util::write_bytes(&mut s, &mut bytes, description.len() + 1, "description", "    ");
            }
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // complaint_type: SpamType
        w.write_all(&(self.complaint_type.as_int().to_le_bytes()))?;

        // offender: Guid
        w.write_all(&self.offender.guid().to_le_bytes())?;

        match &self.complaint_type {
            CMSG_COMPLAIN_SpamType::Mail {
                mail_id,
                unknown1,
                unknown2,
            } => {
                // unknown1: u32
                w.write_all(&unknown1.to_le_bytes())?;

                // mail_id: u32
                w.write_all(&mail_id.to_le_bytes())?;

                // unknown2: u32
                w.write_all(&unknown2.to_le_bytes())?;

            }
            CMSG_COMPLAIN_SpamType::Chat {
                channel_id,
                description,
                language,
                message_type,
                time,
            } => {
                // language: u32
                w.write_all(&language.to_le_bytes())?;

                // message_type: u32
                w.write_all(&message_type.to_le_bytes())?;

                // channel_id: u32
                w.write_all(&channel_id.to_le_bytes())?;

                // time: u32
                w.write_all(&time.to_le_bytes())?;

                // description: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(description.as_bytes().iter().rev().next(), Some(&0_u8), "String `description` must not be null-terminated.");
                w.write_all(description.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(9..=281).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03C6, size: body_size });
        }

        // complaint_type: SpamType
        let complaint_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        // offender: Guid
        let offender = crate::util::read_guid(&mut r)?;

        let complaint_type_if = match complaint_type {
            SpamType::Mail => {
                // unknown1: u32
                let unknown1 = crate::util::read_u32_le(&mut r)?;

                // mail_id: u32
                let mail_id = crate::util::read_u32_le(&mut r)?;

                // unknown2: u32
                let unknown2 = crate::util::read_u32_le(&mut r)?;

                CMSG_COMPLAIN_SpamType::Mail {
                    mail_id,
                    unknown1,
                    unknown2,
                }
            }
            SpamType::Chat => {
                // language: u32
                let language = crate::util::read_u32_le(&mut r)?;

                // message_type: u32
                let message_type = crate::util::read_u32_le(&mut r)?;

                // channel_id: u32
                let channel_id = crate::util::read_u32_le(&mut r)?;

                // time: u32
                let time = crate::util::read_u32_le(&mut r)?;

                // description: CString
                let description = {
                    let description = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(description)?
                };

                CMSG_COMPLAIN_SpamType::Chat {
                    channel_id,
                    description,
                    language,
                    message_type,
                    time,
                }
            }
        };

        Ok(Self {
            complaint_type: complaint_type_if,
            offender,
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_COMPLAIN {}

impl CMSG_COMPLAIN {
    pub(crate) fn size(&self) -> usize {
        self.complaint_type.size() // complaint_type: CMSG_COMPLAIN_SpamType
        + 8 // offender: Guid
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CMSG_COMPLAIN_SpamType {
    Mail {
        mail_id: u32,
        unknown1: u32,
        unknown2: u32,
    },
    Chat {
        channel_id: u32,
        description: String,
        language: u32,
        message_type: u32,
        time: u32,
    },
}

impl Default for CMSG_COMPLAIN_SpamType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Mail {
            mail_id: Default::default(),
            unknown1: Default::default(),
            unknown2: Default::default(),
        }
    }
}

impl CMSG_COMPLAIN_SpamType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Mail { .. } => 0,
            Self::Chat { .. } => 1,
        }
    }

}

impl std::fmt::Display for CMSG_COMPLAIN_SpamType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mail{ .. } => f.write_str("Mail"),
            Self::Chat{ .. } => f.write_str("Chat"),
        }
    }
}

impl CMSG_COMPLAIN_SpamType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Mail {
                ..
            } => {
                1
                + 4 // mail_id: u32
                + 4 // unknown1: u32
                + 4 // unknown2: u32
            }
            Self::Chat {
                description,
                ..
            } => {
                1
                + 4 // channel_id: u32
                + description.len() + 1 // description: CString
                + 4 // language: u32
                + 4 // message_type: u32
                + 4 // time: u32
            }
        }
    }
}

