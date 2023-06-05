use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_time_sync_req.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_time_sync_req.wowm#L3):
/// ```text
/// smsg SMSG_TIME_SYNC_REQ = 0x0390 {
///     u32 time_sync;
/// }
/// ```
pub struct SMSG_TIME_SYNC_REQ {
    pub time_sync: u32,
}

#[cfg(feature = "print-testcase")]
impl SMSG_TIME_SYNC_REQ {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_TIME_SYNC_REQ {{").unwrap();
        // Members
        writeln!(s, "    time_sync = {};", self.time_sync).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 912_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_sync");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_TIME_SYNC_REQ {}
impl crate::Message for SMSG_TIME_SYNC_REQ {
    const OPCODE: u32 = 0x0390;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // time_sync: u32
        w.write_all(&self.time_sync.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0390, size: body_size });
        }

        // time_sync: u32
        let time_sync = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            time_sync,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_TIME_SYNC_REQ {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_TIME_SYNC_REQ {}

