use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_realm_split.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_realm_split.wowm#L3):
/// ```text
/// cmsg CMSG_REALM_SPLIT = 0x038C {
///     u32 realm_id;
/// }
/// ```
pub struct CMSG_REALM_SPLIT {
    /// Realm ID that was sent earlier by the Auth Server
    /// ArcEmu/TriniyCore/mangosthree send back in [`SMSG_REALM_SPLIT`](crate::tbc::SMSG_REALM_SPLIT).
    ///
    pub realm_id: u32,
}

#[cfg(feature = "print-testcase")]
impl CMSG_REALM_SPLIT {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_REALM_SPLIT {{").unwrap();
        // Members
        writeln!(s, "    realm_id = {};", self.realm_id).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 908_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "realm_id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_REALM_SPLIT {}
impl crate::Message for CMSG_REALM_SPLIT {
    const OPCODE: u32 = 0x038c;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_REALM_SPLIT::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // realm_id: u32
        w.write_all(&self.realm_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x038C, size: body_size });
        }

        // realm_id: u32
        let realm_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            realm_id,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_REALM_SPLIT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_REALM_SPLIT {}

