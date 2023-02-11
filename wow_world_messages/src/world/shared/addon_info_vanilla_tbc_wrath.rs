use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm#L1):
/// ```text
/// struct AddonInfo {
///     CString addon_name;
///     u8 addon_has_signature;
///     u32 addon_crc;
///     u32 addon_extra_crc;
/// }
/// ```
pub struct AddonInfo {
    pub addon_name: String,
    pub addon_has_signature: u8,
    pub addon_crc: u32,
    pub addon_extra_crc: u32,
}

impl AddonInfo {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // addon_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.addon_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `addon_name` must not be null-terminated.");
        w.write_all(self.addon_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // addon_has_signature: u8
        w.write_all(&self.addon_has_signature.to_le_bytes())?;

        // addon_crc: u32
        w.write_all(&self.addon_crc.to_le_bytes())?;

        // addon_extra_crc: u32
        w.write_all(&self.addon_extra_crc.to_le_bytes())?;

        Ok(())
    }
}

impl AddonInfo {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // addon_name: CString
        let addon_name = crate::util::read_c_string_to_vec(r)?;
        let addon_name = String::from_utf8(addon_name)?;

        // addon_has_signature: u8
        let addon_has_signature = crate::util::read_u8_le(r)?;

        // addon_crc: u32
        let addon_crc = crate::util::read_u32_le(r)?;

        // addon_extra_crc: u32
        let addon_extra_crc = crate::util::read_u32_le(r)?;

        Ok(Self {
            addon_name,
            addon_has_signature,
            addon_crc,
            addon_extra_crc,
        })
    }

}

impl AddonInfo {
    pub(crate) fn size(&self) -> usize {
        self.addon_name.len() + 1 // addon_name: CString
        + 1 // addon_has_signature: u8
        + 4 // addon_crc: u32
        + 4 // addon_extra_crc: u32
    }
}

