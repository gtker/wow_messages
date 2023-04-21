use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    AllowedClass, AllowedRace,
};
use wow_world_base::shared::level_vanilla_tbc_wrath::Level;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_petition_query_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_petition_query_response.wowm#L1):
/// ```text
/// smsg SMSG_PETITION_QUERY_RESPONSE = 0x01C7 {
///     u32 petition_id;
///     Guid charter_owner;
///     CString guild_name;
///     CString body_text;
///     u32 unknown_flags;
///     u32 minimum_signatures;
///     u32 maximum_signatures;
///     u32 deadline;
///     u32 issue_date;
///     u32 allowed_guild_id;
///     AllowedClass allowed_class;
///     AllowedRace allowed_race;
///     u16 allowed_genders;
///     Level32 allowed_minimum_level;
///     Level32 allowed_maximum_level;
///     u32 todo_amount_of_signers;
///     u32 number_of_choices;
/// }
/// ```
pub struct SMSG_PETITION_QUERY_RESPONSE {
    pub petition_id: u32,
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
    pub allowed_class: AllowedClass,
    /// cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos
    ///
    pub allowed_race: AllowedRace,
    /// cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos
    ///
    pub allowed_genders: u16,
    /// cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos
    ///
    pub allowed_minimum_level: Level,
    /// cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos
    ///
    pub allowed_maximum_level: Level,
    /// cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos
    /// vmangos: char m_choicetext`10``64`
    ///
    pub todo_amount_of_signers: u32,
    /// cmangos/vmangos/mangoszero: Set to 0, only info is comment from vmangos
    ///
    pub number_of_choices: u32,
}

impl crate::private::Sealed for SMSG_PETITION_QUERY_RESPONSE {}
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

        // allowed_class: AllowedClass
        w.write_all(&u32::from(self.allowed_class.as_int()).to_le_bytes())?;

        // allowed_race: AllowedRace
        w.write_all(&u32::from(self.allowed_race.as_int()).to_le_bytes())?;

        // allowed_genders: u16
        w.write_all(&self.allowed_genders.to_le_bytes())?;

        // allowed_minimum_level: Level32
        w.write_all(&u32::from(self.allowed_minimum_level.as_int()).to_le_bytes())?;

        // allowed_maximum_level: Level32
        w.write_all(&u32::from(self.allowed_maximum_level.as_int()).to_le_bytes())?;

        // todo_amount_of_signers: u32
        w.write_all(&self.todo_amount_of_signers.to_le_bytes())?;

        // number_of_choices: u32
        w.write_all(&self.number_of_choices.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(64..=574).contains(&body_size) {
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

        // unknown_flags: u32
        let unknown_flags = crate::util::read_u32_le(&mut r)?;

        // minimum_signatures: u32
        let minimum_signatures = crate::util::read_u32_le(&mut r)?;

        // maximum_signatures: u32
        let maximum_signatures = crate::util::read_u32_le(&mut r)?;

        // deadline: u32
        let deadline = crate::util::read_u32_le(&mut r)?;

        // issue_date: u32
        let issue_date = crate::util::read_u32_le(&mut r)?;

        // allowed_guild_id: u32
        let allowed_guild_id = crate::util::read_u32_le(&mut r)?;

        // allowed_class: AllowedClass
        let allowed_class = AllowedClass::new(crate::util::read_u32_le(&mut r)?);

        // allowed_race: AllowedRace
        let allowed_race = AllowedRace::new(crate::util::read_u32_le(&mut r)?);

        // allowed_genders: u16
        let allowed_genders = crate::util::read_u16_le(&mut r)?;

        // allowed_minimum_level: Level32
        let allowed_minimum_level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

        // allowed_maximum_level: Level32
        let allowed_maximum_level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

        // todo_amount_of_signers: u32
        let todo_amount_of_signers = crate::util::read_u32_le(&mut r)?;

        // number_of_choices: u32
        let number_of_choices = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            petition_id,
            charter_owner,
            guild_name,
            body_text,
            unknown_flags,
            minimum_signatures,
            maximum_signatures,
            deadline,
            issue_date,
            allowed_guild_id,
            allowed_class,
            allowed_race,
            allowed_genders,
            allowed_minimum_level,
            allowed_maximum_level,
            todo_amount_of_signers,
            number_of_choices,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PETITION_QUERY_RESPONSE {}

impl SMSG_PETITION_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // petition_id: u32
        + 8 // charter_owner: Guid
        + self.guild_name.len() + 1 // guild_name: CString
        + self.body_text.len() + 1 // body_text: CString
        + 4 // unknown_flags: u32
        + 4 // minimum_signatures: u32
        + 4 // maximum_signatures: u32
        + 4 // deadline: u32
        + 4 // issue_date: u32
        + 4 // allowed_guild_id: u32
        + 4 // allowed_class: AllowedClass
        + 4 // allowed_race: AllowedRace
        + 2 // allowed_genders: u16
        + 4 // allowed_minimum_level: Level32
        + 4 // allowed_maximum_level: Level32
        + 4 // todo_amount_of_signers: u32
        + 4 // number_of_choices: u32
    }
}

