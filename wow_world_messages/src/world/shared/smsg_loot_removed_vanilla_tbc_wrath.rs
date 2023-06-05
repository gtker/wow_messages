use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Notify a looting player that an item has been taken.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_removed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_removed.wowm#L3):
/// ```text
/// smsg SMSG_LOOT_REMOVED = 0x0162 {
///     u8 slot;
/// }
/// ```
pub struct SMSG_LOOT_REMOVED {
    pub slot: u8,
}

#[cfg(feature = "print-testcase")]
impl SMSG_LOOT_REMOVED {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LOOT_REMOVED {{").unwrap();
        // Members
        writeln!(s, "    slot = {};", self.slot).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 5_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 354_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "slot");
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

impl crate::private::Sealed for SMSG_LOOT_REMOVED {}
impl crate::Message for SMSG_LOOT_REMOVED {
    const OPCODE: u32 = 0x0162;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0162, size: body_size });
        }

        // slot: u8
        let slot = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            slot,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_LOOT_REMOVED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LOOT_REMOVED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LOOT_REMOVED {}

