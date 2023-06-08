use std::io::{Read, Write};

use crate::tbc::Addon;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Banned addons are not properly implemented in the wowm. Sending any number other than 0 means that the packet is incomplete and thus invalid
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm:86`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm#L86):
/// ```text
/// smsg SMSG_ADDON_INFO = 0x02EF {
///     AddonArray addons;
///     u32 number_of_banned_addons = 0;
/// }
/// ```
pub struct SMSG_ADDON_INFO {
    pub addons: Vec<Addon>,
}

impl SMSG_ADDON_INFO {
    /// The field `number_of_banned_addons` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `0` |
    /// | Hex | `0x00` |
    /// | Original | `0` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const NUMBER_OF_BANNED_ADDONS_VALUE: u32 = 0x00;

}

impl crate::private::Sealed for SMSG_ADDON_INFO {}
impl crate::Message for SMSG_ADDON_INFO {
    const OPCODE: u32 = 0x02ef;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ADDON_INFO {{").unwrap();
        // Members
        return None;

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 751_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        panic!("unsupported type Vec<Addon> for variable 'addons'");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "number_of_banned_addons", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // addons: AddonArray
        crate::util::write_addon_array(self.addons.as_slice(), &mut w)?;

        // number_of_banned_addons: u32
        w.write_all(&Self::NUMBER_OF_BANNED_ADDONS_VALUE.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(4..=65535).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02EF, size: body_size });
        }

        panic!("SKIP_SERIALIZE_READ_PANIC This message has an `AddonArray` tag which makes it impossible to generate a correct read implementation for it.")
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ADDON_INFO {}

impl SMSG_ADDON_INFO {
    pub(crate) fn size(&self) -> usize {
        self.addons.len() * 8 // addons: AddonArray
        + 4 // number_of_banned_addons: u32
    }
}

