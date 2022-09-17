use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// cmangos/vmangos/mangoszero: All fields with 'skip' are completely unused
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_petition_buy.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_petition_buy.wowm#L5):
/// ```text
/// cmsg CMSG_PETITION_BUY = 0x01BD {
///     Guid npc;
///     u32 skip1;
///     Guid skip2;
///     CString name;
///     u32 skip3;
///     u32 skip4;
///     u32 skip5;
///     u32 skip6;
///     u32 skip7;
///     u32 skip8;
///     u32 skip9;
///     u32 skip10;
///     u32 skip11;
///     u32 skip12;
///     u16 skip13;
///     u8 skip14;
///     u32 index;
///     u32 skip15;
/// }
/// ```
pub struct CMSG_PETITION_BUY {
    pub npc: Guid,
    pub skip1: u32,
    pub skip2: Guid,
    pub name: String,
    pub skip3: u32,
    pub skip4: u32,
    pub skip5: u32,
    pub skip6: u32,
    pub skip7: u32,
    pub skip8: u32,
    pub skip9: u32,
    pub skip10: u32,
    pub skip11: u32,
    pub skip12: u32,
    pub skip13: u16,
    pub skip14: u8,
    /// cmangos/vmangos/mangoszero: Named but never used
    ///
    pub index: u32,
    pub skip15: u32,
}

impl crate::Message for CMSG_PETITION_BUY {
    const OPCODE: u32 = 0x01bd;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // skip1: u32
        w.write_all(&self.skip1.to_le_bytes())?;

        // skip2: Guid
        w.write_all(&self.skip2.guid().to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // skip3: u32
        w.write_all(&self.skip3.to_le_bytes())?;

        // skip4: u32
        w.write_all(&self.skip4.to_le_bytes())?;

        // skip5: u32
        w.write_all(&self.skip5.to_le_bytes())?;

        // skip6: u32
        w.write_all(&self.skip6.to_le_bytes())?;

        // skip7: u32
        w.write_all(&self.skip7.to_le_bytes())?;

        // skip8: u32
        w.write_all(&self.skip8.to_le_bytes())?;

        // skip9: u32
        w.write_all(&self.skip9.to_le_bytes())?;

        // skip10: u32
        w.write_all(&self.skip10.to_le_bytes())?;

        // skip11: u32
        w.write_all(&self.skip11.to_le_bytes())?;

        // skip12: u32
        w.write_all(&self.skip12.to_le_bytes())?;

        // skip13: u16
        w.write_all(&self.skip13.to_le_bytes())?;

        // skip14: u8
        w.write_all(&self.skip14.to_le_bytes())?;

        // index: u32
        w.write_all(&self.index.to_le_bytes())?;

        // skip15: u32
        w.write_all(&self.skip15.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // npc: Guid
        let npc = Guid::read(r)?;

        // skip1: u32
        let skip1 = crate::util::read_u32_le(r)?;

        // skip2: Guid
        let skip2 = Guid::read(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // skip3: u32
        let skip3 = crate::util::read_u32_le(r)?;

        // skip4: u32
        let skip4 = crate::util::read_u32_le(r)?;

        // skip5: u32
        let skip5 = crate::util::read_u32_le(r)?;

        // skip6: u32
        let skip6 = crate::util::read_u32_le(r)?;

        // skip7: u32
        let skip7 = crate::util::read_u32_le(r)?;

        // skip8: u32
        let skip8 = crate::util::read_u32_le(r)?;

        // skip9: u32
        let skip9 = crate::util::read_u32_le(r)?;

        // skip10: u32
        let skip10 = crate::util::read_u32_le(r)?;

        // skip11: u32
        let skip11 = crate::util::read_u32_le(r)?;

        // skip12: u32
        let skip12 = crate::util::read_u32_le(r)?;

        // skip13: u16
        let skip13 = crate::util::read_u16_le(r)?;

        // skip14: u8
        let skip14 = crate::util::read_u8_le(r)?;

        // index: u32
        let index = crate::util::read_u32_le(r)?;

        // skip15: u32
        let skip15 = crate::util::read_u32_le(r)?;

        Ok(Self {
            npc,
            skip1,
            skip2,
            name,
            skip3,
            skip4,
            skip5,
            skip6,
            skip7,
            skip8,
            skip9,
            skip10,
            skip11,
            skip12,
            skip13,
            skip14,
            index,
            skip15,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_PETITION_BUY {}

impl CMSG_PETITION_BUY {
    pub(crate) fn size(&self) -> usize {
        8 // npc: Guid
        + 4 // skip1: u32
        + 8 // skip2: Guid
        + self.name.len() + 1 // name: CString
        + 4 // skip3: u32
        + 4 // skip4: u32
        + 4 // skip5: u32
        + 4 // skip6: u32
        + 4 // skip7: u32
        + 4 // skip8: u32
        + 4 // skip9: u32
        + 4 // skip10: u32
        + 4 // skip11: u32
        + 4 // skip12: u32
        + 2 // skip13: u16
        + 1 // skip14: u8
        + 4 // index: u32
        + 4 // skip15: u32
    }
}

