use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_motd.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_motd.wowm#L1):
/// ```text
/// smsg SMSG_MOTD = 0x033D {
///     SizedCString motd;
/// }
/// ```
pub struct SMSG_MOTD {
    pub motd: String,
}

impl crate::private::Sealed for SMSG_MOTD {}
impl SMSG_MOTD {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(5..=8004).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x033D, size: body_size });
        }

        // motd: SizedCString
        let motd = {
            let motd = crate::util::read_u32_le(&mut r)?;
            let motd = crate::util::read_sized_c_string_to_vec(&mut r, motd)?;
            String::from_utf8(motd)?
        };

        Ok(Self {
            motd,
        })
    }

}

impl crate::Message for SMSG_MOTD {
    const OPCODE: u32 = 0x033d;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MOTD {{").unwrap();
        // Members
        writeln!(s, "    motd = \"{}\";", self.motd).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 829_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, self.motd.len() + 5, "motd", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // motd: SizedCString
        w.write_all(&((self.motd.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.motd.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_MOTD {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MOTD {}

impl SMSG_MOTD {
    pub(crate) fn size(&self) -> usize {
        self.motd.len() + 5 // motd: SizedCString
    }
}

