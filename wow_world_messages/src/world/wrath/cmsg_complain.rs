use crate::Guid;
use crate::wrath::SpamType;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_complain.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_complain.wowm#L8):
/// ```text
/// cmsg CMSG_COMPLAIN = 0x03C7 {
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

impl crate::Message for CMSG_COMPLAIN {
    const OPCODE: u32 = 0x03c7;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // complaint_type: SpamType
        w.write_all(&(self.complaint_type.as_int() as u8).to_le_bytes())?;

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

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(9..=281).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03C7, size: body_size as u32 });
        }

        // complaint_type: SpamType
        let complaint_type: SpamType = crate::util::read_u8_le(r)?.try_into()?;

        // offender: Guid
        let offender = Guid::read(r)?;

        let complaint_type_if = match complaint_type {
            SpamType::Mail => {
                // unknown1: u32
                let unknown1 = crate::util::read_u32_le(r)?;

                // mail_id: u32
                let mail_id = crate::util::read_u32_le(r)?;

                // unknown2: u32
                let unknown2 = crate::util::read_u32_le(r)?;

                CMSG_COMPLAIN_SpamType::Mail {
                    mail_id,
                    unknown1,
                    unknown2,
                }
            }
            SpamType::Chat => {
                // language: u32
                let language = crate::util::read_u32_le(r)?;

                // message_type: u32
                let message_type = crate::util::read_u32_le(r)?;

                // channel_id: u32
                let channel_id = crate::util::read_u32_le(r)?;

                // time: u32
                let time = crate::util::read_u32_le(r)?;

                // description: CString
                let description = {
                    let description = crate::util::read_c_string_to_vec(r)?;
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
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_COMPLAIN {}

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

impl CMSG_COMPLAIN_SpamType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Mail {
                mail_id,
                unknown1,
                unknown2,
            } => {
                1
                + 4 // mail_id: u32
                + 4 // unknown1: u32
                + 4 // unknown2: u32
            }
            Self::Chat {
                channel_id,
                description,
                language,
                message_type,
                time,
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

