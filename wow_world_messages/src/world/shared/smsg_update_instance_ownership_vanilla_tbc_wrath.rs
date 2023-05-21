use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_update_instance_ownership.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_update_instance_ownership.wowm#L3):
/// ```text
/// smsg SMSG_UPDATE_INSTANCE_OWNERSHIP = 0x032B {
///     Bool32 player_is_saved_to_a_raid;
/// }
/// ```
pub struct SMSG_UPDATE_INSTANCE_OWNERSHIP {
    pub player_is_saved_to_a_raid: bool,
}

impl crate::private::Sealed for SMSG_UPDATE_INSTANCE_OWNERSHIP {}
impl SMSG_UPDATE_INSTANCE_OWNERSHIP {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // player_is_saved_to_a_raid: Bool32
        let player_is_saved_to_a_raid = crate::util::read_u32_le(&mut r)? != 0;

        Ok(Self {
            player_is_saved_to_a_raid,
        })
    }

}

impl crate::Message for SMSG_UPDATE_INSTANCE_OWNERSHIP {
    const OPCODE: u32 = 0x032b;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_UPDATE_INSTANCE_OWNERSHIP {{").unwrap();
        // Members
        writeln!(s, "    player_is_saved_to_a_raid = {};", if self.player_is_saved_to_a_raid { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 811_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "player_is_saved_to_a_raid", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player_is_saved_to_a_raid: Bool32
        w.write_all(u32::from(self.player_is_saved_to_a_raid).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(811, "SMSG_UPDATE_INSTANCE_OWNERSHIP", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_UPDATE_INSTANCE_OWNERSHIP {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_UPDATE_INSTANCE_OWNERSHIP {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_UPDATE_INSTANCE_OWNERSHIP {}

