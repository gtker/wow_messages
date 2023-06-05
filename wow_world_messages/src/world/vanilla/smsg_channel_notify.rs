use std::io::{Read, Write};

use crate::vanilla::ChatNotify;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_channel_notify.wowm:100`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_channel_notify.wowm#L100):
/// ```text
/// smsg SMSG_CHANNEL_NOTIFY = 0x0099 {
///     ChatNotify notify_type;
///     CString channel_name;
/// }
/// ```
pub struct SMSG_CHANNEL_NOTIFY {
    pub notify_type: ChatNotify,
    pub channel_name: String,
}

#[cfg(feature = "print-testcase")]
impl SMSG_CHANNEL_NOTIFY {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CHANNEL_NOTIFY {{").unwrap();
        // Members
        writeln!(s, "    notify_type = {};", self.notify_type.as_test_case_value()).unwrap();
        writeln!(s, "    channel_name = \"{}\";", self.channel_name).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 153_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "notify_type");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1.12\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_CHANNEL_NOTIFY {}
impl crate::Message for SMSG_CHANNEL_NOTIFY {
    const OPCODE: u32 = 0x0099;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // notify_type: ChatNotify
        w.write_all(&(self.notify_type.as_int().to_le_bytes()))?;

        // channel_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel_name` must not be null-terminated.");
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(2..=257).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0099, size: body_size });
        }

        // notify_type: ChatNotify
        let notify_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        // channel_name: CString
        let channel_name = {
            let channel_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(channel_name)?
        };

        Ok(Self {
            notify_type,
            channel_name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_CHANNEL_NOTIFY {}

impl SMSG_CHANNEL_NOTIFY {
    pub(crate) fn size(&self) -> usize {
        1 // notify_type: ChatNotify
        + self.channel_name.len() + 1 // channel_name: CString
    }
}

