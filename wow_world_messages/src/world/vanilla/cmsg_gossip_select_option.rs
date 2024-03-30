use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gossip/cmsg_gossip_select_option.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gossip/cmsg_gossip_select_option.wowm#L1):
/// ```text
/// cmsg CMSG_GOSSIP_SELECT_OPTION = 0x017C {
///     Guid guid;
///     u32 gossip_list_id;
///     optional unknown {
///         CString code;
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_GOSSIP_SELECT_OPTION {
    pub guid: Guid,
    pub gossip_list_id: u32,
    pub unknown: Option<CMSG_GOSSIP_SELECT_OPTION_unknown>,
}

impl crate::private::Sealed for CMSG_GOSSIP_SELECT_OPTION {}
impl CMSG_GOSSIP_SELECT_OPTION {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(12..=268).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // gossip_list_id: u32
        let gossip_list_id = crate::util::read_u32_le(&mut r)?;

        // optional unknown
        let current_size = {
            8 // guid: Guid
            + 4 // gossip_list_id: u32
        };
        let unknown = if current_size < body_size as usize {
            // code: CString
            let code = {
                let code = crate::util::read_c_string_to_vec(&mut r)?;
                String::from_utf8(code)?
            };

            Some(CMSG_GOSSIP_SELECT_OPTION_unknown {
                code,
            })
        } else {
            None
        };

        Ok(Self {
            guid,
            gossip_list_id,
            unknown,
        })
    }

}

impl crate::Message for CMSG_GOSSIP_SELECT_OPTION {
    const OPCODE: u32 = 0x017c;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_GOSSIP_SELECT_OPTION"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GOSSIP_SELECT_OPTION {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    gossip_list_id = {};", self.gossip_list_id).unwrap();
        if let Some(unknown) = &self.unknown {
            writeln!(s, "    code = \"{}\";", unknown.code).unwrap();
        }

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 380_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "gossip_list_id", "    ");
        if let Some(unknown) = &self.unknown {
            crate::util::write_bytes(&mut s, &mut bytes, unknown.code.len() + 1, "code", "    ");
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // gossip_list_id: u32
        w.write_all(&self.gossip_list_id.to_le_bytes())?;

        // optional unknown
        if let Some(v) = &self.unknown {
            // code: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(v.code.as_bytes().iter().next_back(), Some(&0_u8), "String `code` must not be null-terminated.");
            w.write_all(v.code.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(380, "CMSG_GOSSIP_SELECT_OPTION", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GOSSIP_SELECT_OPTION {}

impl CMSG_GOSSIP_SELECT_OPTION {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // gossip_list_id: u32
        + if let Some(unknown) = &self.unknown {
            unknown.code.len() + 1 // code: CString
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_GOSSIP_SELECT_OPTION_unknown {
    pub code: String,
}

