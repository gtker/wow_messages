use std::io::{Read, Write};

use wow_world_base::shared::activate_taxi_reply_vanilla_tbc_wrath::ActivateTaxiReply;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_activatetaxireply.wowm:19`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_activatetaxireply.wowm#L19):
/// ```text
/// smsg SMSG_ACTIVATETAXIREPLY = 0x01AE {
///     ActivateTaxiReply reply;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_ACTIVATETAXIREPLY {
    pub reply: ActivateTaxiReply,
}

impl crate::private::Sealed for SMSG_ACTIVATETAXIREPLY {}
impl SMSG_ACTIVATETAXIREPLY {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // reply: ActivateTaxiReply
        let reply = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            reply,
        })
    }

}

impl crate::Message for SMSG_ACTIVATETAXIREPLY {
    const OPCODE: u32 = 0x01ae;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ACTIVATETAXIREPLY"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ACTIVATETAXIREPLY {{").unwrap();
        // Members
        writeln!(s, "    reply = {};", self.reply.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 430_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "reply", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // reply: ActivateTaxiReply
        w.write_all(&(self.reply.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(430, "SMSG_ACTIVATETAXIREPLY", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ACTIVATETAXIREPLY {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ACTIVATETAXIREPLY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ACTIVATETAXIREPLY {}

