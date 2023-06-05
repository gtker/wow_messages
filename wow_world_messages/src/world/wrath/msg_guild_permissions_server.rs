use std::io::{Read, Write};

use crate::wrath::BankTab;
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_guild_permissions.wowm:26`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_guild_permissions.wowm#L26):
/// ```text
/// smsg MSG_GUILD_PERMISSIONS_Server = 0x03FD {
///     u32 id;
///     u32 rights;
///     Gold gold_limit_per_day;
///     u8 purchased_bank_tabs;
///     BankTab[6] bank_tabs;
/// }
/// ```
pub struct MSG_GUILD_PERMISSIONS_Server {
    pub id: u32,
    pub rights: u32,
    pub gold_limit_per_day: Gold,
    pub purchased_bank_tabs: u8,
    pub bank_tabs: [BankTab; 6],
}

#[cfg(feature = "print-testcase")]
impl MSG_GUILD_PERMISSIONS_Server {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_GUILD_PERMISSIONS_Server {{").unwrap();
        // Members
        writeln!(s, "    id = {};", self.id).unwrap();
        writeln!(s, "    rights = {};", self.rights).unwrap();
        writeln!(s, "    gold_limit_per_day = {};", self.gold_limit_per_day.as_int()).unwrap();
        writeln!(s, "    purchased_bank_tabs = {};", self.purchased_bank_tabs).unwrap();
        write!(s, "    bank_tabs = [").unwrap();
        for v in self.bank_tabs.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        flags = {};", v.flags).unwrap();
            writeln!(s, "        stacks_per_day = {};", v.stacks_per_day).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 63_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1021_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "rights", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "gold_limit_per_day", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "purchased_bank_tabs", "    ");
        writeln!(s, "    /* bank_tabs: BankTab[6] start */").unwrap();
        for (i, v) in self.bank_tabs.iter().enumerate() {
            writeln!(s, "    /* bank_tabs: BankTab[6] {i} start */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "stacks_per_day", "        ");
            writeln!(s, "    /* bank_tabs: BankTab[6] {i} end */").unwrap();
        }
        writeln!(s, "    /* bank_tabs: BankTab[6] end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for MSG_GUILD_PERMISSIONS_Server {}
impl crate::Message for MSG_GUILD_PERMISSIONS_Server {
    const OPCODE: u32 = 0x03fd;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        MSG_GUILD_PERMISSIONS_Server::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        61
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // rights: u32
        w.write_all(&self.rights.to_le_bytes())?;

        // gold_limit_per_day: Gold
        w.write_all((self.gold_limit_per_day.as_int()).to_le_bytes().as_slice())?;

        // purchased_bank_tabs: u8
        w.write_all(&self.purchased_bank_tabs.to_le_bytes())?;

        // bank_tabs: BankTab[6]
        for i in self.bank_tabs.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 61 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03FD, size: body_size });
        }

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // rights: u32
        let rights = crate::util::read_u32_le(&mut r)?;

        // gold_limit_per_day: Gold
        let gold_limit_per_day = Gold::new(crate::util::read_u32_le(&mut r)?);

        // purchased_bank_tabs: u8
        let purchased_bank_tabs = crate::util::read_u8_le(&mut r)?;

        // bank_tabs: BankTab[6]
        let bank_tabs = {
            let mut bank_tabs = [BankTab::default(); 6];
            for i in bank_tabs.iter_mut() {
                *i = BankTab::read(&mut r)?;
            }
            bank_tabs
        };

        Ok(Self {
            id,
            rights,
            gold_limit_per_day,
            purchased_bank_tabs,
            bank_tabs,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_GUILD_PERMISSIONS_Server {}

