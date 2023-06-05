use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_money_notify.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_money_notify.wowm#L7):
/// ```text
/// smsg SMSG_LOOT_MONEY_NOTIFY = 0x0163 {
///     u32 amount;
///     Bool alone;
/// }
/// ```
pub struct SMSG_LOOT_MONEY_NOTIFY {
    pub amount: u32,
    /// Controls the text displayed in chat. False is 'Your share is...' and true is 'You loot...'
    ///
    pub alone: bool,
}

#[cfg(feature = "print-testcase")]
impl SMSG_LOOT_MONEY_NOTIFY {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LOOT_MONEY_NOTIFY {{").unwrap();
        // Members
        writeln!(s, "    amount = {};", self.amount).unwrap();
        writeln!(s, "    alone = {};", if self.alone { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 7_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 355_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "alone", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_LOOT_MONEY_NOTIFY {}
impl crate::Message for SMSG_LOOT_MONEY_NOTIFY {
    const OPCODE: u32 = 0x0163;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_LOOT_MONEY_NOTIFY::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount: u32
        w.write_all(&self.amount.to_le_bytes())?;

        // alone: Bool
        w.write_all(u8::from(self.alone).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0163, size: body_size });
        }

        // amount: u32
        let amount = crate::util::read_u32_le(&mut r)?;

        // alone: Bool
        let alone = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            amount,
            alone,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LOOT_MONEY_NOTIFY {}

