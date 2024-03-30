use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/msg_raid_ready_check.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/msg_raid_ready_check.wowm#L9):
/// ```text
/// smsg MSG_RAID_READY_CHECK_Server = 0x0322 {
///     optional state_check {
///         Guid guid;
///         u8 state;
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MSG_RAID_READY_CHECK_Server {
    pub state_check: Option<MSG_RAID_READY_CHECK_Server_state_check>,
}

impl crate::private::Sealed for MSG_RAID_READY_CHECK_Server {}
impl MSG_RAID_READY_CHECK_Server {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size > 9 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // optional state_check
        let current_size = {
            0
        };
        let state_check = if current_size < body_size as usize {
            // guid: Guid
            let guid = crate::util::read_guid(&mut r)?;

            // state: u8
            let state = crate::util::read_u8_le(&mut r)?;

            Some(MSG_RAID_READY_CHECK_Server_state_check {
                guid,
                state,
            })
        } else {
            None
        };

        Ok(Self {
            state_check,
        })
    }

}

impl crate::Message for MSG_RAID_READY_CHECK_Server {
    const OPCODE: u32 = 0x0322;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "MSG_RAID_READY_CHECK_Server"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_RAID_READY_CHECK_Server {{").unwrap();
        // Members
        if let Some(state_check) = &self.state_check {
            writeln!(s, "    guid = {};", state_check.guid.guid()).unwrap();
            writeln!(s, "    state = {};", state_check.state).unwrap();
        }

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 802_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        if let Some(state_check) = &self.state_check {
            crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 1, "state", "    ");
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // optional state_check
        if let Some(v) = &self.state_check {
            // guid: Guid
            w.write_all(&v.guid.guid().to_le_bytes())?;

            // state: u8
            w.write_all(&v.state.to_le_bytes())?;

        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(802, "MSG_RAID_READY_CHECK_Server", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_RAID_READY_CHECK_Server {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_RAID_READY_CHECK_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_RAID_READY_CHECK_Server {}

impl MSG_RAID_READY_CHECK_Server {
    pub(crate) const fn size(&self) -> usize {
        if let Some(state_check) = &self.state_check {
            8 // guid: Guid
            + 1 // state: u8
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MSG_RAID_READY_CHECK_Server_state_check {
    pub guid: Guid,
    pub state: u8,
}

