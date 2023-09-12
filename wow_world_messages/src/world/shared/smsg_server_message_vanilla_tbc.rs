use std::io::{Read, Write};

use wow_world_base::shared::server_message_type_vanilla_tbc::ServerMessageType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_server_message.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_server_message.wowm#L11):
/// ```text
/// smsg SMSG_SERVER_MESSAGE = 0x0291 {
///     ServerMessageType message_type;
///     CString message;
/// }
/// ```
pub struct SMSG_SERVER_MESSAGE {
    pub message_type: ServerMessageType,
    pub message: String,
}

impl crate::private::Sealed for SMSG_SERVER_MESSAGE {}
impl SMSG_SERVER_MESSAGE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=260).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // message_type: ServerMessageType
        let message_type = crate::util::read_u32_le(&mut r)?.try_into()?;

        // message: CString
        let message = {
            let message = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(message)?
        };

        Ok(Self {
            message_type,
            message,
        })
    }

}

impl crate::Message for SMSG_SERVER_MESSAGE {
    const OPCODE: u32 = 0x0291;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SERVER_MESSAGE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SERVER_MESSAGE {{").unwrap();
        // Members
        writeln!(s, "    message_type = {};", self.message_type.as_test_case_value()).unwrap();
        writeln!(s, "    message = \"{}\";", self.message).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 657_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "message_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.message.len() + 1, "message", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // message_type: ServerMessageType
        w.write_all(&(self.message_type.as_int().to_le_bytes()))?;

        // message: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.message.as_bytes().iter().next_back(), Some(&0_u8), "String `message` must not be null-terminated.");
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(657, "SMSG_SERVER_MESSAGE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SERVER_MESSAGE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SERVER_MESSAGE {}

impl SMSG_SERVER_MESSAGE {
    pub(crate) fn size(&self) -> usize {
        4 // message_type: ServerMessageType
        + self.message.len() + 1 // message: CString
    }
}

