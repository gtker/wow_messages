use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_inspect_talent.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_inspect_talent.wowm#L1):
/// ```text
/// smsg SMSG_INSPECT_TALENT = 0x03F3 {
///     PackedGuid player;
///     u8[-] talent_data;
/// }
/// ```
pub struct SMSG_INSPECT_TALENT {
    pub player: Guid,
    pub talent_data: Vec<u8>,
}

impl crate::private::Sealed for SMSG_INSPECT_TALENT {}
impl SMSG_INSPECT_TALENT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(1..=65544).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // player: PackedGuid
        let player = crate::util::read_packed_guid(&mut r)?;

        // talent_data: u8[-]
        let talent_data = {
            let mut current_size = {
                crate::util::packed_guid_size(&player) // player: PackedGuid
            };
            let mut talent_data = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                talent_data.push(crate::util::read_u8_le(&mut r)?);
                current_size += 1;
            }
            talent_data
        };

        Ok(Self {
            player,
            talent_data,
        })
    }

}

impl crate::Message for SMSG_INSPECT_TALENT {
    const OPCODE: u32 = 0x03f3;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_INSPECT_TALENT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_INSPECT_TALENT {{").unwrap();
        // Members
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        writeln!(s, "    talent_data = [").unwrap();
        for v in self.talent_data.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1011_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.player), "player", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.talent_data.len(), "talent_data", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        crate::util::write_packed_guid(&self.player, &mut w)?;

        // talent_data: u8[-]
        for i in self.talent_data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1011, "SMSG_INSPECT_TALENT", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_INSPECT_TALENT {}

impl SMSG_INSPECT_TALENT {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.player) // player: PackedGuid
        + self.talent_data.len() * core::mem::size_of::<u8>() // talent_data: u8[-]
    }
}

