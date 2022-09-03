use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::ServerMessage;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_petition_query_response.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_petition_query_response.wowm#L3):
/// ```text
/// smsg SMSG_PETITION_QUERY_RESPONSE = 0x01C7 {
///     Guid petition_guid;
///     Guid charter_owner;
///     CString guild_name;
///     CString body_text;
///     u32 unknown_flags;
///     u32 minimum_signatures;
///     u32 maximum_signatures;
///     u32 deadline;
///     u32 issue_date;
///     u32 allowed_guild_id;
///     u32 allowed_classes;
///     u32 allowed_races;
///     u16 allowed_genders;
///     u32 allowed_minimum_level;
///     u32 allowed_maximum_level;
///     u32 todo_amount_of_signers;
///     u32 number_of_choices;
/// }
/// ```
pub struct SMSG_PETITION_QUERY_RESPONSE {
    pub petition_guid: Guid,
    pub charter_owner: Guid,
    pub guild_name: String,
    /// cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos
    ///
    pub body_text: String,
    /// cmangos/vmangos/mangoszero: Set to 1, only info is comment from vmangos
    ///
    pub unknown_flags: u32,
    /// cmangos/vmangos/mangoszero: Set to 9, only info is comment from vmangos
    ///
    pub minimum_signatures: u32,
    /// cmangos/vmangos/mangoszero: Set to 9, only info is comment from vmangos
    ///
    pub maximum_signatures: u32,
    /// cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos
    ///
    pub deadline: u32,
    /// cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos
    ///
    pub issue_date: u32,
    /// cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos
    ///
    pub allowed_guild_id: u32,
    /// cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos
    ///
    pub allowed_classes: u32,
    /// cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos
    ///
    pub allowed_races: u32,
    /// cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos
    ///
    pub allowed_genders: u16,
    /// cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos
    ///
    pub allowed_minimum_level: u32,
    /// cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos
    ///
    pub allowed_maximum_level: u32,
    /// cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos
    ///
    pub todo_amount_of_signers: u32,
    /// cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos
    ///
    pub number_of_choices: u32,
}

impl crate::Message for SMSG_PETITION_QUERY_RESPONSE {
    const OPCODE: u32 = 0x01c7;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // petition_guid: Guid
        w.write_all(&self.petition_guid.guid().to_le_bytes())?;

        // charter_owner: Guid
        w.write_all(&self.charter_owner.guid().to_le_bytes())?;

        // guild_name: CString
        w.write_all(self.guild_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // body_text: CString
        w.write_all(self.body_text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // unknown_flags: u32
        w.write_all(&self.unknown_flags.to_le_bytes())?;

        // minimum_signatures: u32
        w.write_all(&self.minimum_signatures.to_le_bytes())?;

        // maximum_signatures: u32
        w.write_all(&self.maximum_signatures.to_le_bytes())?;

        // deadline: u32
        w.write_all(&self.deadline.to_le_bytes())?;

        // issue_date: u32
        w.write_all(&self.issue_date.to_le_bytes())?;

        // allowed_guild_id: u32
        w.write_all(&self.allowed_guild_id.to_le_bytes())?;

        // allowed_classes: u32
        w.write_all(&self.allowed_classes.to_le_bytes())?;

        // allowed_races: u32
        w.write_all(&self.allowed_races.to_le_bytes())?;

        // allowed_genders: u16
        w.write_all(&self.allowed_genders.to_le_bytes())?;

        // allowed_minimum_level: u32
        w.write_all(&self.allowed_minimum_level.to_le_bytes())?;

        // allowed_maximum_level: u32
        w.write_all(&self.allowed_maximum_level.to_le_bytes())?;

        // todo_amount_of_signers: u32
        w.write_all(&self.todo_amount_of_signers.to_le_bytes())?;

        // number_of_choices: u32
        w.write_all(&self.number_of_choices.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        // charter_owner: Guid
        let charter_owner = Guid::read(r)?;

        // guild_name: CString
        let guild_name = crate::util::read_c_string_to_vec(r)?;
        let guild_name = String::from_utf8(guild_name)?;

        // body_text: CString
        let body_text = crate::util::read_c_string_to_vec(r)?;
        let body_text = String::from_utf8(body_text)?;

        // unknown_flags: u32
        let unknown_flags = crate::util::read_u32_le(r)?;

        // minimum_signatures: u32
        let minimum_signatures = crate::util::read_u32_le(r)?;

        // maximum_signatures: u32
        let maximum_signatures = crate::util::read_u32_le(r)?;

        // deadline: u32
        let deadline = crate::util::read_u32_le(r)?;

        // issue_date: u32
        let issue_date = crate::util::read_u32_le(r)?;

        // allowed_guild_id: u32
        let allowed_guild_id = crate::util::read_u32_le(r)?;

        // allowed_classes: u32
        let allowed_classes = crate::util::read_u32_le(r)?;

        // allowed_races: u32
        let allowed_races = crate::util::read_u32_le(r)?;

        // allowed_genders: u16
        let allowed_genders = crate::util::read_u16_le(r)?;

        // allowed_minimum_level: u32
        let allowed_minimum_level = crate::util::read_u32_le(r)?;

        // allowed_maximum_level: u32
        let allowed_maximum_level = crate::util::read_u32_le(r)?;

        // todo_amount_of_signers: u32
        let todo_amount_of_signers = crate::util::read_u32_le(r)?;

        // number_of_choices: u32
        let number_of_choices = crate::util::read_u32_le(r)?;

        Ok(Self {
            petition_guid,
            charter_owner,
            guild_name,
            body_text,
            unknown_flags,
            minimum_signatures,
            maximum_signatures,
            deadline,
            issue_date,
            allowed_guild_id,
            allowed_classes,
            allowed_races,
            allowed_genders,
            allowed_minimum_level,
            allowed_maximum_level,
            todo_amount_of_signers,
            number_of_choices,
        })
    }

}
impl ServerMessage for SMSG_PETITION_QUERY_RESPONSE {}

impl SMSG_PETITION_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        8 // petition_guid: Guid
        + 8 // charter_owner: Guid
        + self.guild_name.len() + 1 // guild_name: CString
        + self.body_text.len() + 1 // body_text: CString
        + 4 // unknown_flags: u32
        + 4 // minimum_signatures: u32
        + 4 // maximum_signatures: u32
        + 4 // deadline: u32
        + 4 // issue_date: u32
        + 4 // allowed_guild_id: u32
        + 4 // allowed_classes: u32
        + 4 // allowed_races: u32
        + 2 // allowed_genders: u16
        + 4 // allowed_minimum_level: u32
        + 4 // allowed_maximum_level: u32
        + 4 // todo_amount_of_signers: u32
        + 4 // number_of_choices: u32
    }
}

