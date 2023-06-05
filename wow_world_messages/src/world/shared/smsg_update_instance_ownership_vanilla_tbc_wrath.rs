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

#[cfg(feature = "print-testcase")]
impl SMSG_UPDATE_INSTANCE_OWNERSHIP {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_UPDATE_INSTANCE_OWNERSHIP {{").unwrap();
        // Members
        writeln!(s, "    player_is_saved_to_a_raid = {};", if self.player_is_saved_to_a_raid { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 811_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "player_is_saved_to_a_raid");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_UPDATE_INSTANCE_OWNERSHIP {}
impl crate::Message for SMSG_UPDATE_INSTANCE_OWNERSHIP {
    const OPCODE: u32 = 0x032b;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player_is_saved_to_a_raid: Bool32
        w.write_all(u32::from(self.player_is_saved_to_a_raid).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x032B, size: body_size });
        }

        // player_is_saved_to_a_raid: Bool32
        let player_is_saved_to_a_raid = crate::util::read_u32_le(&mut r)? != 0;

        Ok(Self {
            player_is_saved_to_a_raid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_UPDATE_INSTANCE_OWNERSHIP {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_UPDATE_INSTANCE_OWNERSHIP {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_UPDATE_INSTANCE_OWNERSHIP {}

