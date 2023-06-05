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

#[cfg(feature = "print-testcase")]
impl SMSG_INSPECT_TALENT {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_INSPECT_TALENT {{").unwrap();
        // Members
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        write!(s, "    talent_data = [").unwrap();
        for v in self.talent_data.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1011_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_INSPECT_TALENT {}
impl crate::Message for SMSG_INSPECT_TALENT {
    const OPCODE: u32 = 0x03f3;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(2..=65544).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03F3, size: body_size });
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
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_INSPECT_TALENT {}

impl SMSG_INSPECT_TALENT {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.player) // player: PackedGuid
        + self.talent_data.len() * core::mem::size_of::<u8>() // talent_data: u8[-]
    }
}

