use std::io::{Read, Write};

use crate::wrath::Area;
use std::time::Duration;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_entry_invite.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_entry_invite.wowm#L1):
/// ```text
/// smsg SMSG_BATTLEFIELD_MGR_ENTRY_INVITE = 0x04DE {
///     u32 battle_id;
///     Area area;
///     Seconds accept_time;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_BATTLEFIELD_MGR_ENTRY_INVITE {
    pub battle_id: u32,
    pub area: Area,
    pub accept_time: Duration,
}

impl crate::private::Sealed for SMSG_BATTLEFIELD_MGR_ENTRY_INVITE {}
impl SMSG_BATTLEFIELD_MGR_ENTRY_INVITE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // battle_id: u32
        let battle_id = crate::util::read_u32_le(&mut r)?;

        // area: Area
        let area = crate::util::read_u32_le(&mut r)?.try_into()?;

        // accept_time: Seconds
        let accept_time = Duration::from_secs(crate::util::read_u32_le(&mut r)?.into());

        Ok(Self {
            battle_id,
            area,
            accept_time,
        })
    }

}

impl crate::Message for SMSG_BATTLEFIELD_MGR_ENTRY_INVITE {
    const OPCODE: u32 = 0x04de;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_BATTLEFIELD_MGR_ENTRY_INVITE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_BATTLEFIELD_MGR_ENTRY_INVITE {{").unwrap();
        // Members
        writeln!(s, "    battle_id = {};", self.battle_id).unwrap();
        writeln!(s, "    area = {};", self.area.as_test_case_value()).unwrap();
        writeln!(s, "    accept_time = {};", self.accept_time.as_secs()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1246_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "battle_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "area", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "accept_time", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // battle_id: u32
        w.write_all(&self.battle_id.to_le_bytes())?;

        // area: Area
        w.write_all(&(self.area.as_int().to_le_bytes()))?;

        // accept_time: Seconds
        w.write_all((self.accept_time.as_secs() as u32).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1246, "SMSG_BATTLEFIELD_MGR_ENTRY_INVITE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BATTLEFIELD_MGR_ENTRY_INVITE {}

