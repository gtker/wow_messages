use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// cmangos/vmangos/mangoszero: All fields with 'skip' are completely unused
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_petition_buy.wowm:27`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_petition_buy.wowm#L27):
/// ```text
/// cmsg CMSG_PETITION_BUY = 0x01BD {
///     Guid npc;
///     u32 unknown1;
///     Guid unknown2;
///     CString name;
///     CString unknown3;
///     u32 unknown4;
///     u32 unknown5;
///     u32 unknown6;
///     u32 unknown7;
///     u32 unknown8;
///     u32 unknown9;
///     u32 unknown10;
///     u16 unknown11;
///     u32 unknown12;
///     u32 unknown13;
///     u32 unknown14;
///     CString[10] unknown15;
///     u32 index;
///     u32 unknown16;
/// }
/// ```
pub struct CMSG_PETITION_BUY {
    pub npc: Guid,
    pub unknown1: u32,
    pub unknown2: Guid,
    pub name: String,
    pub unknown3: String,
    pub unknown4: u32,
    pub unknown5: u32,
    pub unknown6: u32,
    pub unknown7: u32,
    pub unknown8: u32,
    pub unknown9: u32,
    pub unknown10: u32,
    pub unknown11: u16,
    pub unknown12: u32,
    pub unknown13: u32,
    pub unknown14: u32,
    pub unknown15: [String; 10],
    pub index: u32,
    pub unknown16: u32,
}

impl crate::private::Sealed for CMSG_PETITION_BUY {}
impl CMSG_PETITION_BUY {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(82..=3142).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01BD, size: body_size });
        }

        // npc: Guid
        let npc = crate::util::read_guid(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // unknown2: Guid
        let unknown2 = crate::util::read_guid(&mut r)?;

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // unknown3: CString
        let unknown3 = {
            let unknown3 = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(unknown3)?
        };

        // unknown4: u32
        let unknown4 = crate::util::read_u32_le(&mut r)?;

        // unknown5: u32
        let unknown5 = crate::util::read_u32_le(&mut r)?;

        // unknown6: u32
        let unknown6 = crate::util::read_u32_le(&mut r)?;

        // unknown7: u32
        let unknown7 = crate::util::read_u32_le(&mut r)?;

        // unknown8: u32
        let unknown8 = crate::util::read_u32_le(&mut r)?;

        // unknown9: u32
        let unknown9 = crate::util::read_u32_le(&mut r)?;

        // unknown10: u32
        let unknown10 = crate::util::read_u32_le(&mut r)?;

        // unknown11: u16
        let unknown11 = crate::util::read_u16_le(&mut r)?;

        // unknown12: u32
        let unknown12 = crate::util::read_u32_le(&mut r)?;

        // unknown13: u32
        let unknown13 = crate::util::read_u32_le(&mut r)?;

        // unknown14: u32
        let unknown14 = crate::util::read_u32_le(&mut r)?;

        // unknown15: CString[10]
        let unknown15 = {
            let mut unknown15 = [(); 10].map(|_| String::default());
            for i in unknown15.iter_mut() {
                let s = crate::util::read_c_string_to_vec(&mut r)?;
                *i = String::from_utf8(s)?;
            }
            unknown15
        };

        // index: u32
        let index = crate::util::read_u32_le(&mut r)?;

        // unknown16: u32
        let unknown16 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            npc,
            unknown1,
            unknown2,
            name,
            unknown3,
            unknown4,
            unknown5,
            unknown6,
            unknown7,
            unknown8,
            unknown9,
            unknown10,
            unknown11,
            unknown12,
            unknown13,
            unknown14,
            unknown15,
            index,
            unknown16,
        })
    }

}

impl crate::Message for CMSG_PETITION_BUY {
    const OPCODE: u32 = 0x01bd;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_PETITION_BUY {{").unwrap();
        // Members
        writeln!(s, "    npc = {};", self.npc.guid()).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    unknown2 = {};", self.unknown2.guid()).unwrap();
        writeln!(s, "    name = \"{}\";", self.name).unwrap();
        writeln!(s, "    unknown3 = \"{}\";", self.unknown3).unwrap();
        writeln!(s, "    unknown4 = {};", self.unknown4).unwrap();
        writeln!(s, "    unknown5 = {};", self.unknown5).unwrap();
        writeln!(s, "    unknown6 = {};", self.unknown6).unwrap();
        writeln!(s, "    unknown7 = {};", self.unknown7).unwrap();
        writeln!(s, "    unknown8 = {};", self.unknown8).unwrap();
        writeln!(s, "    unknown9 = {};", self.unknown9).unwrap();
        writeln!(s, "    unknown10 = {};", self.unknown10).unwrap();
        writeln!(s, "    unknown11 = {};", self.unknown11).unwrap();
        writeln!(s, "    unknown12 = {};", self.unknown12).unwrap();
        writeln!(s, "    unknown13 = {};", self.unknown13).unwrap();
        writeln!(s, "    unknown14 = {};", self.unknown14).unwrap();
        write!(s, "    unknown15 = [").unwrap();
        for v in self.unknown15.as_slice() {
            write!(s, "\"{v}\", ").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    index = {};", self.index).unwrap();
        writeln!(s, "    unknown16 = {};", self.unknown16).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 445_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "npc", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "unknown2", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.name.len() + 1, "name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.unknown3.len() + 1, "unknown3", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown4", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown5", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown6", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown7", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown8", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown9", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown10", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 2, "unknown11", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown12", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown13", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown14", "    ");
        writeln!(s, "    /* unknown15: CString[10] start */").unwrap();
        for (i, v) in self.unknown15.iter().enumerate() {
            crate::util::write_bytes(&mut s, &mut bytes, v.len() + 1, &format!("unknown15 {i}"), "    ");
        }
        writeln!(s, "    /* unknown15: CString[10] end */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "index", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown16", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: Guid
        w.write_all(&self.unknown2.guid().to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // unknown3: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.unknown3.as_bytes().iter().rev().next(), Some(&0_u8), "String `unknown3` must not be null-terminated.");
        w.write_all(self.unknown3.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // unknown4: u32
        w.write_all(&self.unknown4.to_le_bytes())?;

        // unknown5: u32
        w.write_all(&self.unknown5.to_le_bytes())?;

        // unknown6: u32
        w.write_all(&self.unknown6.to_le_bytes())?;

        // unknown7: u32
        w.write_all(&self.unknown7.to_le_bytes())?;

        // unknown8: u32
        w.write_all(&self.unknown8.to_le_bytes())?;

        // unknown9: u32
        w.write_all(&self.unknown9.to_le_bytes())?;

        // unknown10: u32
        w.write_all(&self.unknown10.to_le_bytes())?;

        // unknown11: u16
        w.write_all(&self.unknown11.to_le_bytes())?;

        // unknown12: u32
        w.write_all(&self.unknown12.to_le_bytes())?;

        // unknown13: u32
        w.write_all(&self.unknown13.to_le_bytes())?;

        // unknown14: u32
        w.write_all(&self.unknown14.to_le_bytes())?;

        // unknown15: CString[10]
        for i in self.unknown15.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        // index: u32
        w.write_all(&self.index.to_le_bytes())?;

        // unknown16: u32
        w.write_all(&self.unknown16.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PETITION_BUY {}

impl CMSG_PETITION_BUY {
    pub(crate) fn size(&self) -> usize {
        8 // npc: Guid
        + 4 // unknown1: u32
        + 8 // unknown2: Guid
        + self.name.len() + 1 // name: CString
        + self.unknown3.len() + 1 // unknown3: CString
        + 4 // unknown4: u32
        + 4 // unknown5: u32
        + 4 // unknown6: u32
        + 4 // unknown7: u32
        + 4 // unknown8: u32
        + 4 // unknown9: u32
        + 4 // unknown10: u32
        + 2 // unknown11: u16
        + 4 // unknown12: u32
        + 4 // unknown13: u32
        + 4 // unknown14: u32
        + self.unknown15.iter().fold(0, |acc, x| acc + x.len() + 1) // unknown15: CString[10]
        + 4 // index: u32
        + 4 // unknown16: u32
    }
}

