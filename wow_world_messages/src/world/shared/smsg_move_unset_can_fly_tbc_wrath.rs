use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_move_unset_can_fly.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_move_unset_can_fly.wowm#L1):
/// ```text
/// smsg SMSG_MOVE_UNSET_CAN_FLY = 0x0344 {
///     PackedGuid player;
///     u32 counter;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_MOVE_UNSET_CAN_FLY {
    pub player: Guid,
    pub counter: u32,
}

impl crate::private::Sealed for SMSG_MOVE_UNSET_CAN_FLY {}
impl SMSG_MOVE_UNSET_CAN_FLY {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=13).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // player: PackedGuid
        let player = crate::util::read_packed_guid(&mut r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            player,
            counter,
        })
    }

}

impl crate::Message for SMSG_MOVE_UNSET_CAN_FLY {
    const OPCODE: u32 = 0x0344;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_MOVE_UNSET_CAN_FLY"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MOVE_UNSET_CAN_FLY {{").unwrap();
        // Members
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        writeln!(s, "    counter = {};", self.counter).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 836_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.player), "player", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "counter", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        crate::util::write_packed_guid(&self.player, &mut w)?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(836, "SMSG_MOVE_UNSET_CAN_FLY", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_MOVE_UNSET_CAN_FLY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MOVE_UNSET_CAN_FLY {}

impl SMSG_MOVE_UNSET_CAN_FLY {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.player) // player: PackedGuid
        + 4 // counter: u32
    }
}

