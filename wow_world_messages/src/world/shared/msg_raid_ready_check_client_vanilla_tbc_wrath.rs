use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/msg_raid_ready_check.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/msg_raid_ready_check.wowm#L3):
/// ```text
/// cmsg MSG_RAID_READY_CHECK_Client = 0x0322 {
///     optional answer {
///         u8 state;
///     }
/// }
/// ```
pub struct MSG_RAID_READY_CHECK_Client {
    pub answer: Option<MSG_RAID_READY_CHECK_Client_answer>,
}

#[cfg(feature = "print-testcase")]
impl MSG_RAID_READY_CHECK_Client {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_RAID_READY_CHECK_Client {{").unwrap();
        // Members
        if let Some(answer) = &self.answer {
            writeln!(s, "    state = {};", answer.state).unwrap();
        }

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 802_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        if let Some(answer) = &self.answer {
            crate::util::write_bytes(&mut s, &mut bytes, 1, "state", "    ");
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for MSG_RAID_READY_CHECK_Client {}
impl crate::Message for MSG_RAID_READY_CHECK_Client {
    const OPCODE: u32 = 0x0322;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        MSG_RAID_READY_CHECK_Client::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // optional answer
        if let Some(v) = &self.answer {
            // state: u8
            w.write_all(&v.state.to_le_bytes())?;

        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size > 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0322, size: body_size });
        }

        // optional answer
        let current_size = {
            0
        };
        let answer = if current_size < body_size as usize {
            // state: u8
            let state = crate::util::read_u8_le(&mut r)?;

            Some(MSG_RAID_READY_CHECK_Client_answer {
                state,
            })
        } else {
            None
        };

        Ok(Self {
            answer,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_RAID_READY_CHECK_Client {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_RAID_READY_CHECK_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_RAID_READY_CHECK_Client {}

impl MSG_RAID_READY_CHECK_Client {
    pub(crate) const fn size(&self) -> usize {
        if let Some(answer) = &self.answer {
            1 // state: u8
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MSG_RAID_READY_CHECK_Client_answer {
    pub state: u8,
}

