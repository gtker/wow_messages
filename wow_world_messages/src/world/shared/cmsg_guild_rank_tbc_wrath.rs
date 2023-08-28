use std::io::{Read, Write};

use crate::shared::guild_bank_rights_tbc_wrath::GuildBankRights;
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_rank.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_rank.wowm#L16):
/// ```text
/// cmsg CMSG_GUILD_RANK = 0x0231 {
///     u32 rank_id;
///     u32 rights;
///     CString rank_name;
///     Gold money_per_day;
///     GuildBankRights[6] bank_tab_rights;
/// }
/// ```
pub struct CMSG_GUILD_RANK {
    pub rank_id: u32,
    pub rights: u32,
    pub rank_name: String,
    pub money_per_day: Gold,
    pub bank_tab_rights: [GuildBankRights; 6],
}

impl crate::private::Sealed for CMSG_GUILD_RANK {}
impl CMSG_GUILD_RANK {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(61..=316).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // rank_id: u32
        let rank_id = crate::util::read_u32_le(&mut r)?;

        // rights: u32
        let rights = crate::util::read_u32_le(&mut r)?;

        // rank_name: CString
        let rank_name = {
            let rank_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(rank_name)?
        };

        // money_per_day: Gold
        let money_per_day = Gold::new(crate::util::read_u32_le(&mut r)?);

        // bank_tab_rights: GuildBankRights[6]
        let bank_tab_rights = {
            let mut bank_tab_rights = [GuildBankRights::default(); 6];
            for i in bank_tab_rights.iter_mut() {
                *i = GuildBankRights::read(&mut r)?;
            }
            bank_tab_rights
        };

        Ok(Self {
            rank_id,
            rights,
            rank_name,
            money_per_day,
            bank_tab_rights,
        })
    }

}

impl crate::Message for CMSG_GUILD_RANK {
    const OPCODE: u32 = 0x0231;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GUILD_RANK {{").unwrap();
        // Members
        writeln!(s, "    rank_id = {};", self.rank_id).unwrap();
        writeln!(s, "    rights = {};", self.rights).unwrap();
        writeln!(s, "    rank_name = \"{}\";", self.rank_name).unwrap();
        writeln!(s, "    money_per_day = {};", self.money_per_day.as_int()).unwrap();
        write!(s, "    bank_tab_rights = [").unwrap();
        for v in self.bank_tab_rights.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        rights = {};", v.rights).unwrap();
            writeln!(s, "        slots_per_day = {};", v.slots_per_day).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 561_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "rank_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "rights", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.rank_name.len() + 1, "rank_name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "money_per_day", "    ");
        writeln!(s, "    /* bank_tab_rights: GuildBankRights[6] start */").unwrap();
        for (i, v) in self.bank_tab_rights.iter().enumerate() {
            writeln!(s, "    /* bank_tab_rights: GuildBankRights[6] {i} start */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 4, "rights", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "slots_per_day", "        ");
            writeln!(s, "    /* bank_tab_rights: GuildBankRights[6] {i} end */").unwrap();
        }
        writeln!(s, "    /* bank_tab_rights: GuildBankRights[6] end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // rank_id: u32
        w.write_all(&self.rank_id.to_le_bytes())?;

        // rights: u32
        w.write_all(&self.rights.to_le_bytes())?;

        // rank_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.rank_name.as_bytes().iter().next_back(), Some(&0_u8), "String `rank_name` must not be null-terminated.");
        w.write_all(self.rank_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // money_per_day: Gold
        w.write_all((self.money_per_day.as_int()).to_le_bytes().as_slice())?;

        // bank_tab_rights: GuildBankRights[6]
        for i in self.bank_tab_rights.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(561, "CMSG_GUILD_RANK", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GUILD_RANK {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GUILD_RANK {}

impl CMSG_GUILD_RANK {
    pub(crate) fn size(&self) -> usize {
        4 // rank_id: u32
        + 4 // rights: u32
        + self.rank_name.len() + 1 // rank_name: CString
        + 4 // money_per_day: Gold
        + 6 * 8 // bank_tab_rights: GuildBankRights[6]
    }
}

