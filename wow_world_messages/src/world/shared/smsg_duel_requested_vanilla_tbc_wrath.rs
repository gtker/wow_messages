use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/duel/smsg_duel_requested.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/duel/smsg_duel_requested.wowm#L3):
/// ```text
/// smsg SMSG_DUEL_REQUESTED = 0x0167 {
///     Guid initiator;
///     Guid target;
/// }
/// ```
pub struct SMSG_DUEL_REQUESTED {
    pub initiator: Guid,
    pub target: Guid,
}

#[cfg(feature = "print-testcase")]
impl SMSG_DUEL_REQUESTED {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_DUEL_REQUESTED {{").unwrap();
        // Members
        writeln!(s, "    initiator = {};", self.initiator.guid()).unwrap();
        writeln!(s, "    target = {};", self.target.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 20_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 359_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "initiator");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_DUEL_REQUESTED {}
impl crate::Message for SMSG_DUEL_REQUESTED {
    const OPCODE: u32 = 0x0167;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // initiator: Guid
        w.write_all(&self.initiator.guid().to_le_bytes())?;

        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0167, size: body_size });
        }

        // initiator: Guid
        let initiator = crate::util::read_guid(&mut r)?;

        // target: Guid
        let target = crate::util::read_guid(&mut r)?;

        Ok(Self {
            initiator,
            target,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_DUEL_REQUESTED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_DUEL_REQUESTED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_DUEL_REQUESTED {}

