use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_feign_death_resisted.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_feign_death_resisted.wowm#L3):
/// ```text
/// smsg SMSG_FEIGN_DEATH_RESISTED = 0x02B4 {
/// }
/// ```
pub struct SMSG_FEIGN_DEATH_RESISTED {
}

#[cfg(feature = "print-testcase")]
impl SMSG_FEIGN_DEATH_RESISTED {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_FEIGN_DEATH_RESISTED {{").unwrap();
        // Members

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 4_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 692_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_FEIGN_DEATH_RESISTED {}
impl crate::Message for SMSG_FEIGN_DEATH_RESISTED {
    const OPCODE: u32 = 0x02b4;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02B4, size: body_size });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_FEIGN_DEATH_RESISTED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_FEIGN_DEATH_RESISTED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_FEIGN_DEATH_RESISTED {}

