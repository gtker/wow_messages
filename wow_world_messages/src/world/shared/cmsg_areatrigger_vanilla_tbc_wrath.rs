use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/cmsg_areatrigger.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/cmsg_areatrigger.wowm#L3):
/// ```text
/// cmsg CMSG_AREATRIGGER = 0x00B4 {
///     u32 trigger_id;
/// }
/// ```
pub struct CMSG_AREATRIGGER {
    pub trigger_id: u32,
}

#[cfg(feature = "print-testcase")]
impl CMSG_AREATRIGGER {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_AREATRIGGER {{").unwrap();
        // Members
        writeln!(s, "    trigger_id = {};", self.trigger_id).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 180_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "trigger_id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1.12 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_AREATRIGGER {}
impl crate::Message for CMSG_AREATRIGGER {
    const OPCODE: u32 = 0x00b4;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_AREATRIGGER::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // trigger_id: u32
        w.write_all(&self.trigger_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00B4, size: body_size });
        }

        // trigger_id: u32
        let trigger_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            trigger_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AREATRIGGER {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AREATRIGGER {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AREATRIGGER {}

