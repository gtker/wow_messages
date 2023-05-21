use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/msg_raid_ready_check_confirm.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/msg_raid_ready_check_confirm.wowm#L1):
/// ```text
/// cmsg MSG_RAID_READY_CHECK_CONFIRM_Client = 0x03AE {
///     optional set {
///         u8 state;
///     }
/// }
/// ```
pub struct MSG_RAID_READY_CHECK_CONFIRM_Client {
    pub set: Option<MSG_RAID_READY_CHECK_CONFIRM_Client_set>,
}

impl crate::private::Sealed for MSG_RAID_READY_CHECK_CONFIRM_Client {}
impl MSG_RAID_READY_CHECK_CONFIRM_Client {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size > 1 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x03AE, size: body_size });
        }

        // optional set
        let current_size = {
            0
        };
        let set = if current_size < body_size as usize {
            // state: u8
            let state = crate::util::read_u8_le(&mut r)?;

            Some(MSG_RAID_READY_CHECK_CONFIRM_Client_set {
                state,
            })
        } else {
            None
        };

        Ok(Self {
            set,
        })
    }

}

impl crate::Message for MSG_RAID_READY_CHECK_CONFIRM_Client {
    const OPCODE: u32 = 0x03ae;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_RAID_READY_CHECK_CONFIRM_Client {{").unwrap();
        // Members
        if let Some(set) = &self.set {
            writeln!(s, "    state = {};", set.state).unwrap();
        }

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 942_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        if let Some(set) = &self.set {
            crate::util::write_bytes(&mut s, &mut bytes, 1, "state", "    ");
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
        // optional set
        if let Some(v) = &self.set {
            // state: u8
            w.write_all(&v.state.to_le_bytes())?;

        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_RAID_READY_CHECK_CONFIRM_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_RAID_READY_CHECK_CONFIRM_Client {}

impl MSG_RAID_READY_CHECK_CONFIRM_Client {
    pub(crate) const fn size(&self) -> usize {
        if let Some(set) = &self.set {
            1 // state: u8
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MSG_RAID_READY_CHECK_CONFIRM_Client_set {
    pub state: u8,
}

