use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm#L1):
/// ```text
/// struct Addon {
///     u8 addon_type = 2;
///     u8 uses_crc = 1;
///     u8 uses_diffent_public_key;
///     u32 unknown1;
///     u8 unknown2;
/// }
/// ```
pub struct Addon {
    pub uses_diffent_public_key: u8,
    /// Other emus hardcode this to 0
    ///
    pub unknown1: u32,
    /// Other emus hardcode this to 0
    ///
    pub unknown2: u8,
}

impl Addon {
    /// The field `addon_type` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `2` |
    /// | Hex | `0x02` |
    /// | Original | `2` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const ADDON_TYPE_VALUE: u8 = 0x02;

    /// The field `uses_crc` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `1` |
    /// | Hex | `0x01` |
    /// | Original | `1` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const USES_CRC_VALUE: u8 = 0x01;

}

impl Addon {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // addon_type: u8
        w.write_all(&Self::ADDON_TYPE_VALUE.to_le_bytes())?;

        // uses_crc: u8
        w.write_all(&Self::USES_CRC_VALUE.to_le_bytes())?;

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
        let _addon_type = crate::util::read_u8_le(r)?;
        // addon_type is expected to always be 2 (2)

        // uses_crc: u8
        let _uses_crc = crate::util::read_u8_le(r)?;
        // uses_crc is expected to always be 1 (1)

        // uses_diffent_public_key: u8
        let uses_diffent_public_key = crate::util::read_u8_le(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // unknown2: u8
        let unknown2 = crate::util::read_u8_le(r)?;

        Ok(Self {
            uses_diffent_public_key,
            unknown1,
            unknown2,
        })
    }

}

