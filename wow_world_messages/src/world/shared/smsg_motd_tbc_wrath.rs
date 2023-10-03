use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_motd.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_motd.wowm#L1):
/// ```text
/// smsg SMSG_MOTD = 0x033D {
///     u32 amount_of_motds;
///     CString[amount_of_motds] motds;
/// }
/// ```
pub struct SMSG_MOTD {
    pub motds: Vec<String>,
}

impl crate::private::Sealed for SMSG_MOTD {}
impl SMSG_MOTD {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // amount_of_motds: u32
        let amount_of_motds = crate::util::read_u32_le(&mut r)?;

        // motds: CString[amount_of_motds]
        let motds = {
            let mut motds = Vec::with_capacity(amount_of_motds as usize);
            for _ in 0..amount_of_motds {
                let s = crate::util::read_c_string_to_vec(&mut r)?;
                motds.push(String::from_utf8(s)?);
            }
            motds
        };

        Ok(Self {
            motds,
        })
    }

}

impl crate::Message for SMSG_MOTD {
    const OPCODE: u32 = 0x033d;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_MOTD"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MOTD {{").unwrap();
        // Members
        writeln!(s, "    amount_of_motds = {};", self.motds.len()).unwrap();
        writeln!(s, "    motds = [").unwrap();
        for v in self.motds.as_slice() {
            write!(s, "\"{v}\", ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 829_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_motds", "    ");
        if !self.motds.is_empty() {
            writeln!(s, "    /* motds: CString[amount_of_motds] start */").unwrap();
            for (i, v) in self.motds.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, v.len() + 1, &format!("motds {i}"), "    ");
            }
            writeln!(s, "    /* motds: CString[amount_of_motds] end */").unwrap();
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
        // amount_of_motds: u32
        w.write_all(&(self.motds.len() as u32).to_le_bytes())?;

        // motds: CString[amount_of_motds]
        for i in self.motds.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(829, "SMSG_MOTD", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_MOTD {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MOTD {}

impl SMSG_MOTD {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_motds: u32
        + self.motds.iter().fold(0, |acc, x| acc + x.len() + 1) // motds: CString[amount_of_motds]
    }
}

