use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm:66`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm#L66):
/// ```text
/// struct Addon {
///     u8 addon_type;
///     u8 uses_crc;
///     Bool uses_diffent_public_key;
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
    pub uses_diffent_public_key: bool,
    /// Other emus hardcode this to 0
    ///
    pub unknown1: u32,
    /// Other emus hardcode this to 0
    ///
    pub unknown2: u8,
}

impl Addon {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // addon_type: u8
        w.write_all(&self.addon_type.to_le_bytes())?;

        // uses_crc: u8
        w.write_all(&self.uses_crc.to_le_bytes())?;

        // uses_diffent_public_key: Bool
        w.write_all(u8::from(self.uses_diffent_public_key).to_le_bytes().as_slice())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u8
        w.write_all(&self.unknown2.to_le_bytes())?;

        Ok(())
    }
}

impl Addon {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // addon_type: u8
        let addon_type = crate::util::read_u8_le(&mut r)?;

        // uses_crc: u8
        let uses_crc = crate::util::read_u8_le(&mut r)?;

        // uses_diffent_public_key: Bool
        let uses_diffent_public_key = crate::util::read_u8_le(&mut r)? != 0;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // unknown2: u8
        let unknown2 = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            addon_type,
            uses_crc,
            uses_diffent_public_key,
            unknown1,
            unknown2,
        })
    }

}

