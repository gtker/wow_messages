use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_auth_challenge.wowm#L21):
/// ```text
/// smsg SMSG_AUTH_CHALLENGE = 0x01EC {
///     u32 unknown1;
///     u32 server_seed;
///     u8[32] seed;
/// }
/// ```
pub struct SMSG_AUTH_CHALLENGE {
    /// TrinityCore/ArcEmu/mangostwo always set to 1.
    /// TrinityCore/mangostwo: 1...31
    ///
    pub unknown1: u32,
    pub server_seed: u32,
    /// Randomized values. Is not used at all by TrinityCore/mangostwo/ArcEmu.
    ///
    pub seed: [u8; 32],
}

#[cfg(feature = "print-testcase")]
impl SMSG_AUTH_CHALLENGE {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_AUTH_CHALLENGE {{").unwrap();
        // Members
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    server_seed = {};", self.server_seed).unwrap();
        write!(s, "    seed = [").unwrap();
        for v in self.seed.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 44_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 492_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_AUTH_CHALLENGE {}
impl crate::Message for SMSG_AUTH_CHALLENGE {
    const OPCODE: u32 = 0x01ec;

    fn size_without_header(&self) -> u32 {
        40
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // server_seed: u32
        w.write_all(&self.server_seed.to_le_bytes())?;

        // seed: u8[32]
        for i in self.seed.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 40 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01EC, size: body_size });
        }

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // server_seed: u32
        let server_seed = crate::util::read_u32_le(&mut r)?;

        // seed: u8[32]
        let seed = {
            let mut seed = [0_u8; 32];
            r.read_exact(&mut seed)?;
            seed
        };

        Ok(Self {
            unknown1,
            server_seed,
            seed,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_AUTH_CHALLENGE {}

