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

impl crate::private::Sealed for SMSG_CHANNEL_MEMBER_COUNT {}
impl SMSG_CHANNEL_MEMBER_COUNT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(6..=261).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
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

impl crate::Message for SMSG_CHANNEL_MEMBER_COUNT {
    const OPCODE: u32 = 0x03d4;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_CHANNEL_MEMBER_COUNT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CHANNEL_MEMBER_COUNT {{").unwrap();
        // Members
        writeln!(s, "    channel = \"{}\";", self.channel).unwrap();
        writeln!(s, "    flags = {};", self.flags).unwrap();
        writeln!(s, "    amount_of_members = {};", self.amount_of_members).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 980_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, self.channel.len() + 1, "channel", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "flags", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_members", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // channel: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel.as_bytes().iter().next_back(), Some(&0_u8), "String `channel` must not be null-terminated.");
        w.write_all(self.channel.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // flags: u8
        w.write_all(&self.flags.to_le_bytes())?;

        // amount_of_members: u32
        w.write_all(&self.amount_of_members.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(980, "SMSG_CHANNEL_MEMBER_COUNT", body_size, a))
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

