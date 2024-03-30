use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_area_trigger_message.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_area_trigger_message.wowm#L3):
/// ```text
/// smsg SMSG_AREA_TRIGGER_MESSAGE = 0x02B8 {
///     SizedCString message;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_AREA_TRIGGER_MESSAGE {
    pub message: String,
}

impl crate::private::Sealed for SMSG_AREA_TRIGGER_MESSAGE {}
impl SMSG_AREA_TRIGGER_MESSAGE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=8004).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // message: SizedCString
        let message = {
            let message = crate::util::read_u32_le(&mut r)?;
            let message = crate::util::read_sized_c_string_to_vec(&mut r, message)?;
            String::from_utf8(message)?
        };

        Ok(Self {
            message,
        })
    }

}

impl crate::Message for SMSG_AREA_TRIGGER_MESSAGE {
    const OPCODE: u32 = 0x02b8;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_AREA_TRIGGER_MESSAGE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_AREA_TRIGGER_MESSAGE {{").unwrap();
        // Members
        writeln!(s, "    message = \"{}\";", self.message).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 696_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, self.message.len() + 5, "message", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // message: SizedCString
        w.write_all(&((self.message.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(696, "SMSG_AREA_TRIGGER_MESSAGE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_AREA_TRIGGER_MESSAGE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_AREA_TRIGGER_MESSAGE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_AREA_TRIGGER_MESSAGE {}

impl SMSG_AREA_TRIGGER_MESSAGE {
    pub(crate) fn size(&self) -> usize {
        self.message.len() + 5 // message: SizedCString
    }
}

