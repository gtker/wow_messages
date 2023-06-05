use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/msg_raid_ready_check.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/msg_raid_ready_check.wowm#L9):
/// ```text
/// smsg MSG_RAID_READY_CHECK_Server = 0x0322 {
///     optional state_check {
///         Guid guid;
///         u8 state;
///     }
/// }
/// ```
pub struct MSG_RAID_READY_CHECK_Server {
    pub state_check: Option<MSG_RAID_READY_CHECK_Server_state_check>,
}

#[cfg(feature = "print-testcase")]
impl MSG_RAID_READY_CHECK_Server {
    pub fn to_test_case_string(&self) -> String {
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

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 802_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for MSG_RAID_READY_CHECK_Server {}
impl crate::Message for MSG_RAID_READY_CHECK_Server {
    const OPCODE: u32 = 0x0322;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size > 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0322, size: body_size });
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

