use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_guild_bank_list.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_guild_bank_list.wowm#L15):
/// ```text
/// struct GuildBankTab {
///     CString tab_name;
///     CString tab_icon;
/// }
/// ```
pub struct GuildBankTab {
    pub tab_name: String,
    pub tab_icon: String,
}

impl GuildBankTab {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // tab_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.tab_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `tab_name` must not be null-terminated.");
        w.write_all(self.tab_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // tab_icon: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.tab_icon.as_bytes().iter().rev().next(), Some(&0_u8), "String `tab_icon` must not be null-terminated.");
        w.write_all(self.tab_icon.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl GuildBankTab {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // tab_name: CString
        let tab_name = crate::util::read_c_string_to_vec(r)?;
        let tab_name = String::from_utf8(tab_name)?;

        // tab_icon: CString
        let tab_icon = crate::util::read_c_string_to_vec(r)?;
        let tab_icon = String::from_utf8(tab_icon)?;

        Ok(Self {
            tab_name,
            tab_icon,
        })
    }

}

impl GuildBankTab {
    pub(crate) fn size(&self) -> usize {
        self.tab_name.len() + 1 // tab_name: CString
        + self.tab_icon.len() + 1 // tab_icon: CString
    }
}

