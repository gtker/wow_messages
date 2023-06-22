use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// cmangos/vmangos/mangoszero: deliver undelivered mail
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/smsg_received_mail.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/smsg_received_mail.wowm#L3):
/// ```text
/// smsg SMSG_RECEIVED_MAIL = 0x0285 {
///     u32 unknown1;
/// }
/// ```
pub struct SMSG_RECEIVED_MAIL {
    /// cmangos/vmangos sends 0 as u32, mangoszero sends 0 as f32
    pub unknown1: u32,
}

impl crate::private::Sealed for SMSG_RECEIVED_MAIL {}
impl crate::Message for SMSG_RECEIVED_MAIL {
    const OPCODE: u32 = 0x0285;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_RECEIVED_MAIL {{").unwrap();
        // Members
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 645_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0285, size: body_size });
        }

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            unknown1,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_RECEIVED_MAIL {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_RECEIVED_MAIL {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_RECEIVED_MAIL {}

