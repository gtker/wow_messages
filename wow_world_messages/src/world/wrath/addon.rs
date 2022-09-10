use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm#L1):
/// ```text
/// struct Addon {
///     u8 addon_type;
///     u8 uses_crc;
///     u8 uses_diffent_public_key;
///     u32 unknown1;
///     u8 unknown2;
/// }
/// ```
pub struct Addon {
    /// Other emus hardcode this to 2. More research is required
    ///
    pub addon_type: u8,
    /// Other emus hardcode this to 1.
    ///
    pub uses_crc: u8,
    pub uses_diffent_public_key: u8,
    /// Other emus hardcode this to 0
    ///
    pub unknown1: u32,
    /// Other emus hardcode this to 0
    ///
    pub unknown2: u8,
}

impl Addon {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // addon_type: u8
        w.write_all(&self.addon_type.to_le_bytes())?;

        // uses_crc: u8
        w.write_all(&self.uses_crc.to_le_bytes())?;

        // uses_diffent_public_key: u8
        w.write_all(&self.uses_diffent_public_key.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u8
        w.write_all(&self.unknown2.to_le_bytes())?;

        Ok(())
    }
}

impl Addon {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // addon_type: u8
        let addon_type = crate::util::read_u8_le(r)?;

        // uses_crc: u8
        let uses_crc = crate::util::read_u8_le(r)?;

        // uses_diffent_public_key: u8
        let uses_diffent_public_key = crate::util::read_u8_le(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // unknown2: u8
        let unknown2 = crate::util::read_u8_le(r)?;

        Ok(Self {
            addon_type,
            uses_crc,
            uses_diffent_public_key,
            unknown1,
            unknown2,
        })
    }

}

