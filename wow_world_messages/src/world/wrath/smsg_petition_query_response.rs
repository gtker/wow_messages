use crate:: {
    Guid,
};
use crate::wrath::CharterType;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_petition_query_response.wowm:90`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_petition_query_response.wowm#L90):
/// ```text
/// smsg SMSG_PETITION_QUERY_RESPONSE = 0x01C7 {
///     u32 petition_id;
///     Guid charter_owner;
///     CString guild_name;
///     CString body_text;
///     u32 minimum_signatures;
///     u32 maximum_signatures;
///     u32 unknown1;
///     u32 unknown2;
///     u32 unknown3;
///     u32 unknown4;
///     u32 unknown5;
///     u16 unknown6;
///     u32 unknown7;
///     u32 unknown8;
///     u32 unknown9;
///     u8[10] unknown10;
///     u32 unknown11;
///     (u32)CharterType charter_type;
/// }
/// ```
pub struct SMSG_PETITION_QUERY_RESPONSE {
    pub petition_id: u32,
    pub charter_owner: Guid,
    pub guild_name: String,
    /// cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos
    ///
    pub body_text: String,
    /// cmangos/vmangos/mangoszero: Set to 9, only info is comment from vmangos
    ///
    pub minimum_signatures: u32,
    /// cmangos/vmangos/mangoszero: Set to 9, only info is comment from vmangos
    ///
    pub maximum_signatures: u32,
    /// mangosone: bypass client - side limitation, a different value is needed here for each petition
    ///
    pub unknown1: u32,
    pub unknown2: u32,
    pub unknown3: u32,
    pub unknown4: u32,
    pub unknown5: u32,
    pub unknown6: u16,
    pub unknown7: u32,
    pub unknown8: u32,
    pub unknown9: u32,
    pub unknown10: [u8; 10],
    pub unknown11: u32,
    pub charter_type: CharterType,
}

impl crate::Message for SMSG_PETITION_QUERY_RESPONSE {
    const OPCODE: u32 = 0x01c7;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // petition_id: u32
        w.write_all(&self.petition_id.to_le_bytes())?;

        // charter_owner: Guid
        w.write_all(&self.charter_owner.guid().to_le_bytes())?;

        // guild_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.guild_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `guild_name` must not be null-terminated.");
        w.write_all(self.guild_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // body_text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.body_text.as_bytes().iter().rev().next(), Some(&0_u8), "String `body_text` must not be null-terminated.");
        w.write_all(self.body_text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // minimum_signatures: u32
        w.write_all(&self.minimum_signatures.to_le_bytes())?;

        // maximum_signatures: u32
        w.write_all(&self.maximum_signatures.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: u32
        w.write_all(&self.unknown3.to_le_bytes())?;

        // unknown4: u32
        w.write_all(&self.unknown4.to_le_bytes())?;

        // unknown5: u32
        w.write_all(&self.unknown5.to_le_bytes())?;

        // unknown6: u16
        w.write_all(&self.unknown6.to_le_bytes())?;

        // unknown7: u32
        w.write_all(&self.unknown7.to_le_bytes())?;

        // unknown8: u32
        w.write_all(&self.unknown8.to_le_bytes())?;

        // unknown9: u32
        w.write_all(&self.unknown9.to_le_bytes())?;

        // unknown10: u8[10]
        for i in self.unknown10.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // unknown11: u32
        w.write_all(&self.unknown11.to_le_bytes())?;

        // charter_type: CharterType
        w.write_all(&u32::from(self.charter_type.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(74..=584).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01C7, size: body_size as u32 });
        }

        // petition_id: u32
        let petition_id = crate::util::read_u32_le(&mut r)?;

        // charter_owner: Guid
        let charter_owner = Guid::read(&mut r)?;

        // guild_name: CString
        let guild_name = {
            let guild_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(guild_name)?
        };

        // body_text: CString
        let body_text = {
            let body_text = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(body_text)?
        };

        // minimum_signatures: u32
        let minimum_signatures = crate::util::read_u32_le(&mut r)?;

        // maximum_signatures: u32
        let maximum_signatures = crate::util::read_u32_le(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(&mut r)?;

        // unknown3: u32
        let unknown3 = crate::util::read_u32_le(&mut r)?;

        // unknown4: u32
        let unknown4 = crate::util::read_u32_le(&mut r)?;

        // unknown5: u32
        let unknown5 = crate::util::read_u32_le(&mut r)?;

        // unknown6: u16
        let unknown6 = crate::util::read_u16_le(&mut r)?;

        // unknown7: u32
        let unknown7 = crate::util::read_u32_le(&mut r)?;

        // unknown8: u32
        let unknown8 = crate::util::read_u32_le(&mut r)?;

        // unknown9: u32
        let unknown9 = crate::util::read_u32_le(&mut r)?;

        // unknown10: u8[10]
        let unknown10 = {
            let mut unknown10 = [0_u8; 10];
            r.read_exact(&mut unknown10)?;
            unknown10
        };

        // unknown11: u32
        let unknown11 = crate::util::read_u32_le(&mut r)?;

        // charter_type: CharterType
        let charter_type: CharterType = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            petition_id,
            charter_owner,
            guild_name,
            body_text,
            minimum_signatures,
            maximum_signatures,
            unknown1,
            unknown2,
            unknown3,
            unknown4,
            unknown5,
            unknown6,
            unknown7,
            unknown8,
            unknown9,
            unknown10,
            unknown11,
            charter_type,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PETITION_QUERY_RESPONSE {}

impl SMSG_PETITION_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // petition_id: u32
        + 8 // charter_owner: Guid
        + self.guild_name.len() + 1 // guild_name: CString
        + self.body_text.len() + 1 // body_text: CString
        + 4 // minimum_signatures: u32
        + 4 // maximum_signatures: u32
        + 4 // unknown1: u32
        + 4 // unknown2: u32
        + 4 // unknown3: u32
        + 4 // unknown4: u32
        + 4 // unknown5: u32
        + 2 // unknown6: u16
        + 4 // unknown7: u32
        + 4 // unknown8: u32
        + 4 // unknown9: u32
        + 10 * core::mem::size_of::<u8>() // unknown10: u8[10]
        + 4 // unknown11: u32
        + 4 // charter_type: CharterType
    }
}

