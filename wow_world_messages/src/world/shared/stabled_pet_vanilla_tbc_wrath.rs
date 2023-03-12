use std::io::{Read, Write};

use wow_world_base::shared::level_vanilla_tbc_wrath::Level;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/msg_list_stabled_pets_server.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/msg_list_stabled_pets_server.wowm#L3):
/// ```text
/// struct StabledPet {
///     u32 pet_number;
///     u32 entry;
///     Level32 level;
///     CString name;
///     u32 loyalty;
///     u8 slot;
/// }
/// ```
pub struct StabledPet {
    pub pet_number: u32,
    pub entry: u32,
    pub level: Level,
    pub name: String,
    pub loyalty: u32,
    /// vmangos/mangoszero/cmangos: client slot 1 == current pet (0)
    ///
    pub slot: u8,
}

impl StabledPet {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes())?;

        // entry: u32
        w.write_all(&self.entry.to_le_bytes())?;

        // level: Level32
        w.write_all(&u32::from(self.level.as_int()).to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // loyalty: u32
        w.write_all(&self.loyalty.to_le_bytes())?;

        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        Ok(())
    }
}

impl StabledPet {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // pet_number: u32
        let pet_number = crate::util::read_u32_le(&mut r)?;

        // entry: u32
        let entry = crate::util::read_u32_le(&mut r)?;

        // level: Level32
        let level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // loyalty: u32
        let loyalty = crate::util::read_u32_le(&mut r)?;

        // slot: u8
        let slot = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            pet_number,
            entry,
            level,
            name,
            loyalty,
            slot,
        })
    }

}

impl StabledPet {
    pub(crate) fn size(&self) -> usize {
        4 // pet_number: u32
        + 4 // entry: u32
        + 4 // level: Level32
        + self.name.len() + 1 // name: CString
        + 4 // loyalty: u32
        + 1 // slot: u8
    }
}

