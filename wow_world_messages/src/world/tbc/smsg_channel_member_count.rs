use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_channel_member_count.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_channel_member_count.wowm#L1):
/// ```text
/// smsg SMSG_CHANNEL_MEMBER_COUNT = 0x03D4 {
///     CString channel;
///     u8 flags;
///     u32 amount_of_members;
/// }
/// ```
pub struct SMSG_CHANNEL_MEMBER_COUNT {
    pub channel: String,
    pub flags: u8,
    pub amount_of_members: u32,
}

#[cfg(feature = "print-testcase")]
impl SMSG_CHANNEL_MEMBER_COUNT {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CHANNEL_MEMBER_COUNT {{").unwrap();
        // Members
        writeln!(s, "    channel = \"{}\";", self.channel).unwrap();
        writeln!(s, "    flags = {};", self.flags).unwrap();
        writeln!(s, "    amount_of_members = {};", self.amount_of_members).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 980_u32.to_le_bytes();
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
        writeln!(s, "    versions = \"2.4.3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_CHANNEL_MEMBER_COUNT {}
impl crate::Message for SMSG_CHANNEL_MEMBER_COUNT {
    const OPCODE: u32 = 0x03d4;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // channel: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel` must not be null-terminated.");
        w.write_all(self.channel.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // flags: u8
        w.write_all(&self.flags.to_le_bytes())?;

        // amount_of_members: u32
        w.write_all(&self.amount_of_members.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(6..=261).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03D4, size: body_size });
        }

        // channel: CString
        let channel = {
            let channel = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(channel)?
        };

        // flags: u8
        let flags = crate::util::read_u8_le(&mut r)?;

        // amount_of_members: u32
        let amount_of_members = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            channel,
            flags,
            amount_of_members,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_CHANNEL_MEMBER_COUNT {}

impl SMSG_CHANNEL_MEMBER_COUNT {
    pub(crate) fn size(&self) -> usize {
        self.channel.len() + 1 // channel: CString
        + 1 // flags: u8
        + 4 // amount_of_members: u32
    }
}

