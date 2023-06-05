use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_set_active_voice_channel.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_set_active_voice_channel.wowm#L1):
/// ```text
/// cmsg CMSG_SET_ACTIVE_VOICE_CHANNEL = 0x03D2 {
///     u32 unknown1;
///     CString unknown2;
/// }
/// ```
pub struct CMSG_SET_ACTIVE_VOICE_CHANNEL {
    pub unknown1: u32,
    pub unknown2: String,
}

#[cfg(feature = "print-testcase")]
impl CMSG_SET_ACTIVE_VOICE_CHANNEL {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SET_ACTIVE_VOICE_CHANNEL {{").unwrap();
        // Members
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    unknown2 = \"{}\";", self.unknown2).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 6).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 978_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_SET_ACTIVE_VOICE_CHANNEL {}
impl crate::Message for CMSG_SET_ACTIVE_VOICE_CHANNEL {
    const OPCODE: u32 = 0x03d2;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.unknown2.as_bytes().iter().rev().next(), Some(&0_u8), "String `unknown2` must not be null-terminated.");
        w.write_all(self.unknown2.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(5..=260).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03D2, size: body_size });
        }

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // unknown2: CString
        let unknown2 = {
            let unknown2 = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(unknown2)?
        };

        Ok(Self {
            unknown1,
            unknown2,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_ACTIVE_VOICE_CHANNEL {}

impl CMSG_SET_ACTIVE_VOICE_CHANNEL {
    pub(crate) fn size(&self) -> usize {
        4 // unknown1: u32
        + self.unknown2.len() + 1 // unknown2: CString
    }
}

