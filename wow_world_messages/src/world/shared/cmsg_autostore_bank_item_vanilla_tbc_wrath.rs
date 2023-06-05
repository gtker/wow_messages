use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_autostore_bank_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_autostore_bank_item.wowm#L3):
/// ```text
/// cmsg CMSG_AUTOSTORE_BANK_ITEM = 0x0282 {
///     u8 bag_index;
///     u8 slot_index;
/// }
/// ```
pub struct CMSG_AUTOSTORE_BANK_ITEM {
    pub bag_index: u8,
    pub slot_index: u8,
}

#[cfg(feature = "print-testcase")]
impl CMSG_AUTOSTORE_BANK_ITEM {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_AUTOSTORE_BANK_ITEM {{").unwrap();
        // Members
        writeln!(s, "    bag_index = {};", self.bag_index).unwrap();
        writeln!(s, "    slot_index = {};", self.slot_index).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 642_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "bag_index");
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

impl crate::private::Sealed for CMSG_AUTOSTORE_BANK_ITEM {}
impl crate::Message for CMSG_AUTOSTORE_BANK_ITEM {
    const OPCODE: u32 = 0x0282;

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // bag_index: u8
        w.write_all(&self.bag_index.to_le_bytes())?;

        // slot_index: u8
        w.write_all(&self.slot_index.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0282, size: body_size });
        }

        // bag_index: u8
        let bag_index = crate::util::read_u8_le(&mut r)?;

        // slot_index: u8
        let slot_index = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            bag_index,
            slot_index,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AUTOSTORE_BANK_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AUTOSTORE_BANK_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUTOSTORE_BANK_ITEM {}

