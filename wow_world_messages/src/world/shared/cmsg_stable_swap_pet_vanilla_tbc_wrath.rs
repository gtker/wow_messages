use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_stable_swap_pet.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_stable_swap_pet.wowm#L3):
/// ```text
/// cmsg CMSG_STABLE_SWAP_PET = 0x0275 {
///     Guid npc;
///     u32 pet_slot;
/// }
/// ```
pub struct CMSG_STABLE_SWAP_PET {
    pub npc: Guid,
    pub pet_slot: u32,
}

impl crate::private::Sealed for CMSG_STABLE_SWAP_PET {}
impl CMSG_STABLE_SWAP_PET {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0275, size: body_size });
        }

        // npc: Guid
        let npc = crate::util::read_guid(&mut r)?;

        // pet_slot: u32
        let pet_slot = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            npc,
            pet_slot,
        })
    }

}

impl crate::Message for CMSG_STABLE_SWAP_PET {
    const OPCODE: u32 = 0x0275;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_STABLE_SWAP_PET {{").unwrap();
        // Members
        writeln!(s, "    npc = {};", self.npc.guid()).unwrap();
        writeln!(s, "    pet_slot = {};", self.pet_slot).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 16_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 629_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "npc", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "pet_slot", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // pet_slot: u32
        w.write_all(&self.pet_slot.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_STABLE_SWAP_PET {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_STABLE_SWAP_PET {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_STABLE_SWAP_PET {}

