use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_destroyitem.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_destroyitem.wowm#L3):
/// ```text
/// cmsg CMSG_DESTROYITEM = 0x0111 {
///     u8 bag;
///     u8 slot;
///     u8 amount;
///     u8 unknown1;
///     u8 unknown2;
///     u8 unknown3;
/// }
/// ```
pub struct CMSG_DESTROYITEM {
    pub bag: u8,
    pub slot: u8,
    pub amount: u8,
    pub unknown1: u8,
    pub unknown2: u8,
    pub unknown3: u8,
}

impl crate::private::Sealed for CMSG_DESTROYITEM {}
impl CMSG_DESTROYITEM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 6 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // bag: u8
        let bag = crate::util::read_u8_le(&mut r)?;

        // slot: u8
        let slot = crate::util::read_u8_le(&mut r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(&mut r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(&mut r)?;

        // unknown2: u8
        let unknown2 = crate::util::read_u8_le(&mut r)?;

        // unknown3: u8
        let unknown3 = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            bag,
            slot,
            amount,
            unknown1,
            unknown2,
            unknown3,
        })
    }

}

impl crate::Message for CMSG_DESTROYITEM {
    const OPCODE: u32 = 0x0111;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_DESTROYITEM {{").unwrap();
        // Members
        writeln!(s, "    bag = {};", self.bag).unwrap();
        writeln!(s, "    slot = {};", self.slot).unwrap();
        writeln!(s, "    amount = {};", self.amount).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    unknown2 = {};", self.unknown2).unwrap();
        writeln!(s, "    unknown3 = {};", self.unknown3).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 10_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 273_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "bag", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown2", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown3", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        6
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // bag: u8
        w.write_all(&self.bag.to_le_bytes())?;

        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u8
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: u8
        w.write_all(&self.unknown3.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(273, "CMSG_DESTROYITEM", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_DESTROYITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_DESTROYITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_DESTROYITEM {}

