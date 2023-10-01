use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_crossed_inebriation_threshold.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_crossed_inebriation_threshold.wowm#L1):
/// ```text
/// smsg SMSG_CROSSED_INEBRIATION_THRESHOLD = 0x03C0 {
///     Guid player;
///     u32 state;
///     Item item;
/// }
/// ```
pub struct SMSG_CROSSED_INEBRIATION_THRESHOLD {
    pub player: Guid,
    pub state: u32,
    pub item: u32,
}

impl crate::private::Sealed for SMSG_CROSSED_INEBRIATION_THRESHOLD {}
impl SMSG_CROSSED_INEBRIATION_THRESHOLD {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 16 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // player: Guid
        let player = crate::util::read_guid(&mut r)?;

        // state: u32
        let state = crate::util::read_u32_le(&mut r)?;

        // item: Item
        let item = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            player,
            state,
            item,
        })
    }

}

impl crate::Message for SMSG_CROSSED_INEBRIATION_THRESHOLD {
    const OPCODE: u32 = 0x03c0;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_CROSSED_INEBRIATION_THRESHOLD"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CROSSED_INEBRIATION_THRESHOLD {{").unwrap();
        // Members
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        writeln!(s, "    state = {};", self.state).unwrap();
        writeln!(s, "    item = {};", self.item).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 18_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 960_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "player", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "state", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // state: u32
        w.write_all(&self.state.to_le_bytes())?;

        // item: Item
        w.write_all(&self.item.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(960, "SMSG_CROSSED_INEBRIATION_THRESHOLD", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_CROSSED_INEBRIATION_THRESHOLD {}

