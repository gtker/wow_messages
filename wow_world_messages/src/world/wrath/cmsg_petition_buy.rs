use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// cmangos/vmangos/mangoszero: All fields with 'skip' are completely unused
///
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

impl crate::Message for CMSG_PETITION_BUY {
    const OPCODE: u32 = 0x01bd;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
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
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(82..=3142).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01BD, size: body_size as u32 });
        }

        // npc: Guid
        let npc = Guid::read(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // unknown2: Guid
        let unknown2 = Guid::read(r)?;

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(name)?
        };

        // unknown3: CString
        let unknown3 = {
            let unknown3 = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(unknown3)?
        };

        // unknown4: u32
        let unknown4 = crate::util::read_u32_le(r)?;

        // unknown5: u32
        let unknown5 = crate::util::read_u32_le(r)?;

        // unknown6: u32
        let unknown6 = crate::util::read_u32_le(r)?;

        // unknown7: u32
        let unknown7 = crate::util::read_u32_le(r)?;

        // unknown8: u32
        let unknown8 = crate::util::read_u32_le(r)?;

        // unknown9: u32
        let unknown9 = crate::util::read_u32_le(r)?;

        // unknown10: u32
        let unknown10 = crate::util::read_u32_le(r)?;

        // unknown11: u16
        let unknown11 = crate::util::read_u16_le(r)?;

        // unknown12: u32
        let unknown12 = crate::util::read_u32_le(r)?;

        // unknown13: u32
        let unknown13 = crate::util::read_u32_le(r)?;

        // unknown14: u32
        let unknown14 = crate::util::read_u32_le(r)?;

        // unknown15: CString[10]
        let unknown15 = {
            let mut unknown15 = [(); 10].map(|_| String::default());
            for i in unknown15.iter_mut() {
                let s = crate::util::read_c_string_to_vec(r)?;
                *i = String::from_utf8(s)?;
            }
            unknown15
        };

        // index: u32
        let index = crate::util::read_u32_le(r)?;

        // unknown16: u32
        let unknown16 = crate::util::read_u32_le(r)?;

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

