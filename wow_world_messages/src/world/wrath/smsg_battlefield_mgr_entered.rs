use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_entered.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_entered.wowm#L1):
/// ```text
/// smsg SMSG_BATTLEFIELD_MGR_ENTERED = 0x04E0 {
///     u32 battle_id;
///     u8 unknown1;
///     u8 unknown2;
///     Bool clear_afk;
/// }
/// ```
pub struct SMSG_BATTLEFIELD_MGR_ENTERED {
    pub battle_id: u32,
    pub unknown1: u8,
    pub unknown2: u8,
    pub clear_afk: bool,
}

#[cfg(feature = "print-testcase")]
impl SMSG_BATTLEFIELD_MGR_ENTERED {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_BATTLEFIELD_MGR_ENTERED {{").unwrap();
        // Members
        writeln!(s, "    battle_id = {};", self.battle_id).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    unknown2 = {};", self.unknown2).unwrap();
        writeln!(s, "    clear_afk = {};", if self.clear_afk { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 11_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1248_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "battle_id");
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

impl crate::private::Sealed for SMSG_BATTLEFIELD_MGR_ENTERED {}
impl crate::Message for SMSG_BATTLEFIELD_MGR_ENTERED {
    const OPCODE: u32 = 0x04e0;

    fn size_without_header(&self) -> u32 {
        7
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // battle_id: u32
        w.write_all(&self.battle_id.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u8
        w.write_all(&self.unknown2.to_le_bytes())?;

        // clear_afk: Bool
        w.write_all(u8::from(self.clear_afk).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 7 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04E0, size: body_size });
        }

        // battle_id: u32
        let battle_id = crate::util::read_u32_le(&mut r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(&mut r)?;

        // unknown2: u8
        let unknown2 = crate::util::read_u8_le(&mut r)?;

        // clear_afk: Bool
        let clear_afk = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            battle_id,
            unknown1,
            unknown2,
            clear_afk,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BATTLEFIELD_MGR_ENTERED {}

