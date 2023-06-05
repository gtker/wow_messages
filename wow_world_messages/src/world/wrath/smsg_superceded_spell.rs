use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_superceded_spell.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_superceded_spell.wowm#L8):
/// ```text
/// smsg SMSG_SUPERCEDED_SPELL = 0x012C {
///     u32 new;
///     u32 old;
/// }
/// ```
pub struct SMSG_SUPERCEDED_SPELL {
    pub new: u32,
    pub old: u32,
}

#[cfg(feature = "print-testcase")]
impl SMSG_SUPERCEDED_SPELL {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SUPERCEDED_SPELL {{").unwrap();
        // Members
        writeln!(s, "    new = {};", self.new).unwrap();
        writeln!(s, "    old = {};", self.old).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 12_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 300_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "new");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_SUPERCEDED_SPELL {}
impl crate::Message for SMSG_SUPERCEDED_SPELL {
    const OPCODE: u32 = 0x012c;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // new: u32
        w.write_all(&self.new.to_le_bytes())?;

        // old: u32
        w.write_all(&self.old.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x012C, size: body_size });
        }

        // new: u32
        let new = crate::util::read_u32_le(&mut r)?;

        // old: u32
        let old = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            new,
            old,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SUPERCEDED_SPELL {}

