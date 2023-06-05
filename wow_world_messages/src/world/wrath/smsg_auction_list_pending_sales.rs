use std::io::{Read, Write};

use crate::wrath::PendingAuctionSale;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_list_pending_sales.wowm:19`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_list_pending_sales.wowm#L19):
/// ```text
/// smsg SMSG_AUCTION_LIST_PENDING_SALES = 0x0490 {
///     u32 amount_of_pending_sales;
///     PendingAuctionSale[amount_of_pending_sales] pending_sales;
/// }
/// ```
pub struct SMSG_AUCTION_LIST_PENDING_SALES {
    pub pending_sales: Vec<PendingAuctionSale>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_AUCTION_LIST_PENDING_SALES {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_AUCTION_LIST_PENDING_SALES {{").unwrap();
        // Members
        writeln!(s, "    amount_of_pending_sales = {};", self.pending_sales.len()).unwrap();
        write!(s, "    pending_sales = [").unwrap();
        for v in self.pending_sales.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        string1 = \"{}\";", v.string1).unwrap();
            writeln!(s, "        string2 = \"{}\";", v.string2).unwrap();
            writeln!(s, "        unknown1 = {};", v.unknown1).unwrap();
            writeln!(s, "        unknown2 = {};", v.unknown2).unwrap();
            writeln!(s, "    {}", if v.time_left.to_string().contains(".") { v.time_left.to_string() } else { format!("{}.0", v.time_left) }).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1168_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_pending_sales", "    ");
        if !self.pending_sales.is_empty() {
            writeln!(s, "    /* pending_sales: PendingAuctionSale[amount_of_pending_sales] start */").unwrap();
            for (i, v) in self.pending_sales.iter().enumerate() {
                writeln!(s, "    /* pending_sales: PendingAuctionSale[amount_of_pending_sales] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, v.string1.len() + 1, "string1", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.string2.len() + 1, "string2", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown2", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "time_left", "        ");
                writeln!(s, "    /* pending_sales: PendingAuctionSale[amount_of_pending_sales] {i} end */").unwrap();
            }
            writeln!(s, "    /* pending_sales: PendingAuctionSale[amount_of_pending_sales] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_AUCTION_LIST_PENDING_SALES {}
impl crate::Message for SMSG_AUCTION_LIST_PENDING_SALES {
    const OPCODE: u32 = 0x0490;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_AUCTION_LIST_PENDING_SALES::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_pending_sales: u32
        w.write_all(&(self.pending_sales.len() as u32).to_le_bytes())?;

        // pending_sales: PendingAuctionSale[amount_of_pending_sales]
        for i in self.pending_sales.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(4..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0490, size: body_size });
        }

        // amount_of_pending_sales: u32
        let amount_of_pending_sales = crate::util::read_u32_le(&mut r)?;

        // pending_sales: PendingAuctionSale[amount_of_pending_sales]
        let pending_sales = {
            let mut pending_sales = Vec::with_capacity(amount_of_pending_sales as usize);
            for _ in 0..amount_of_pending_sales {
                pending_sales.push(PendingAuctionSale::read(&mut r)?);
            }
            pending_sales
        };

        Ok(Self {
            pending_sales,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_AUCTION_LIST_PENDING_SALES {}

impl SMSG_AUCTION_LIST_PENDING_SALES {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_pending_sales: u32
        + self.pending_sales.iter().fold(0, |acc, x| acc + x.size()) // pending_sales: PendingAuctionSale[amount_of_pending_sales]
    }
}

