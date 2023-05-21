use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::{
    GroupLootSetting, ItemQuality,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_loot_method.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_loot_method.wowm#L9):
/// ```text
/// cmsg CMSG_LOOT_METHOD = 0x007A {
///     (u32)GroupLootSetting loot_setting;
///     Guid loot_master;
///     (u32)ItemQuality loot_threshold;
/// }
/// ```
pub struct CMSG_LOOT_METHOD {
    pub loot_setting: GroupLootSetting,
    pub loot_master: Guid,
    pub loot_threshold: ItemQuality,
}

impl crate::private::Sealed for CMSG_LOOT_METHOD {}
impl CMSG_LOOT_METHOD {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 16 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x007A, size: body_size });
        }

        // loot_setting: GroupLootSetting
        let loot_setting = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // loot_master: Guid
        let loot_master = crate::util::read_guid(&mut r)?;

        // loot_threshold: ItemQuality
        let loot_threshold = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            loot_setting,
            loot_master,
            loot_threshold,
        })
    }

}

impl crate::Message for CMSG_LOOT_METHOD {
    const OPCODE: u32 = 0x007a;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_LOOT_METHOD {{").unwrap();
        // Members
        writeln!(s, "    loot_setting = {};", self.loot_setting.as_test_case_value()).unwrap();
        writeln!(s, "    loot_master = {};", self.loot_master.guid()).unwrap();
        writeln!(s, "    loot_threshold = {};", self.loot_threshold.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 20_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 122_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "loot_setting", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "loot_master", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "loot_threshold", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // loot_setting: GroupLootSetting
        w.write_all(&u32::from(self.loot_setting.as_int()).to_le_bytes())?;

        // loot_master: Guid
        w.write_all(&self.loot_master.guid().to_le_bytes())?;

        // loot_threshold: ItemQuality
        w.write_all(&u32::from(self.loot_threshold.as_int()).to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_LOOT_METHOD {}

